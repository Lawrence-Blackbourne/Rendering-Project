use super::ParserError;
use super::tokeniser::{
    Token::{self, *},
    TokenisedXml,
};
use std::io::{self, Read};

pub(super) fn parse_xml<T: Read>(xml: TokenisedXml<T>) -> Result<ParsedXml<T>, ParserError> {
    ParsedXml::try_from(xml)
}

#[derive(Debug)]
pub(super) struct ParsedXml<T: Read> {
    pub tokens: TokenisedXml<T>,
    names: Vec<String>,
    root_found: bool,
    next: Option<Item>,
    done: bool,
}

impl<'a, T: Read> TryFrom<TokenisedXml<T>> for ParsedXml<T> {
    type Error = ParserError;

    fn try_from(mut tokens: TokenisedXml<T>) -> Result<Self, Self::Error> {
        Self::handle_xml_intro(&mut tokens)?;

        Ok(ParsedXml {
            tokens,
            names: Vec::new(),
            root_found: false,
            next: None,
            done: false,
        })
    }
}

impl<T: Read> ParsedXml<T> {
    pub(super) fn next(&mut self) -> Option<Result<Item, ParserError>> {
        if self.done {
            return None;
        }
        match self.get_next_element() {
            Ok(Item::EndFile) => {
                self.done = true;
                Some(Ok(Item::EndFile))
            }
            Ok(v) => Some(Ok(v)),
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }

    pub(super) fn skip_current_element(&mut self) -> Option<Result<(), ParserError>> {
        let mut current_depth = 1;
        loop {
            match self.next()? {
                Ok(Item::Element(_)) => current_depth += 1,
                Ok(Item::EndCurrentElement) => current_depth -= 1,
                Ok(_) => (),
                Err(e) => return Some(Err(e)),
            }
            if current_depth == 0 {
                break;
            }
        }
        Some(Ok(()))
    }

    fn get_next_element(&mut self) -> Result<Item, ParserError> {
        if self.next.is_some() {
            let value = self.next.clone().unwrap();
            self.next = None;
            return Ok(value);
        }
        self.remove_leading_whitespace()?;
        match self.tokens.next().transpose()? {
            Some(StartTag) => {
                let (name, start) = match self.tokens.next().transpose()? {
                    Some(Word(name)) => (name, true),
                    Some(Slash) => match self.tokens.next().transpose()? {
                        Some(Word(name)) => (name, false),
                        Some(token) => return Err(ParserError::InvalidToken(token)),
                        None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                    },
                    Some(token) => return Err(ParserError::InvalidToken(token)),
                    None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                };
                if self.root_found && self.names.len() == 0 {
                    return Err(ParserError::MultipleRootElements);
                } else if !self.root_found {
                    self.root_found = true;
                }
                if start {
                    let attributes = self.get_attributes()?;
                    match self.tokens.next().transpose()? {
                        Some(EndTag) => {
                            self.names.push(name.clone());
                            Ok(Item::Element(Element { name, attributes }))
                        }
                        Some(Slash) => {
                            self.expect_token(EndTag, ParserError::FileCutShortAbruptlyDuringTag)?;
                            self.next = Some(Item::EndCurrentElement);
                            Ok(Item::Element(Element { name, attributes }))
                        }
                        Some(token) => Err(ParserError::InvalidToken(token)),
                        None => Err(ParserError::FileCutShortAbruptlyDuringTag),
                    }
                } else {
                    match self.names.pop() {
                        Some(correct) if correct == name => {
                            self.expect_token(EndTag, ParserError::FileCutShortAbruptlyDuringTag)?;
                            Ok(Item::EndCurrentElement)
                        },
                        Some(correct) =>
                            Err(ParserError::ElementsClosedOutOfOrder{
                                found: name,
                                correct,
                            }),
                        None =>
                            Err(ParserError::ElementClosedAfterRootElementClosed(name)),
                    }
                }
            }
            Some(Word(str)) => self.get_text(str.as_str()),
            Some(QuotationMark) => self.get_text("\""),
            Some(Slash) => self.get_text("\\"),
            Some(QuestionMark) => self.get_text("?"),
            Some(Equals) => self.get_text("="),
            Some(token) => Err(ParserError::InvalidToken(token)),
            None => self.handle_ending(),
        }
    }

    fn get_attributes(&mut self) -> Result<Vec<(String, String)>, ParserError> {
        let mut result = Vec::new();
        loop {
            self.remove_leading_whitespace()?;
            match self.tokens.peek() {
                Some(Ok(Word(_))) => {
                    let Word(key) = self.tokens.next().unwrap()? else { unreachable!() };
                    self.expect_token(Equals, ParserError::FileCutShortAbruptlyDuringTag)?;
                    self.expect_token(QuotationMark, ParserError::FileCutShortAbruptlyDuringTag)?;
                    let mut value = String::new();
                    loop {
                        match self.tokens.next().transpose()? {
                            Some(Word(word)) => value += &word,
                            Some(Whitespace(char)) => value.push(char),
                            Some(Slash) => value.push('/'),
                            Some(QuestionMark) => value.push('?'),
                            Some(Equals) => value.push('='),
                            Some(QuotationMark) => break,
                            Some(token) => return Err(ParserError::InvalidToken(token)),
                            None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                        }
                    }
                    result.push((key, value));
                }
                Some(Ok(_)) => return Ok(result),
                Some(Err(_)) => return Err(self.tokens.next().unwrap().unwrap_err()),
                None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
            }
        }
    }

    fn get_text(&mut self, current: &str) -> Result<Item, ParserError> {
        let mut result = String::from(current);
        loop {
            match self.tokens.peek() {
                Some(Ok(Whitespace(char))) => result.push(*char),
                Some(Ok(Word(word))) => result += &word,
                Some(Ok(QuotationMark)) => result.push('"'),
                Some(Ok(Slash)) => result.push('/'),
                Some(Ok(QuestionMark)) => result.push('?'),
                Some(Ok(Equals)) => result.push('='),
                Some(Ok(_)) => break,
                Some(Err(_)) => return Err(self.tokens.next().unwrap().unwrap_err()),
                None => return self.handle_ending(),
            }
            self.tokens.next();
        }
        Ok(Item::Text(result))
    }

    fn handle_xml_intro(tokens: &mut TokenisedXml<T>) -> Result<(), ParserError> {
        Self::remove_leading_whitespace_from_tokens(tokens)?;
        match tokens.peek() {
            Some(Ok(StartTag)) => (),
            Some(Ok(token)) => return Err(ParserError::InvalidToken(token.clone())),
            Some(Err(e)) => return Err(tokens.next().unwrap().unwrap_err()),
            None => return Err(ParserError::NoRootElement),
        }
        match tokens.peek_n(1) {
            Some(Ok(QuestionMark)) => {
                tokens.next();
                tokens.next();
                loop {
                    match tokens.next().transpose()? {
                        Some(QuestionMark) => break,
                        Some(StartTag) => return Err(ParserError::InvalidToken(StartTag)),
                        Some(Slash) => return Err(ParserError::InvalidToken(Slash)),
                        Some(EndTag) => return Err(ParserError::InvalidToken(EndTag)),
                        Some(_) => (),
                        None => (),
                    }
                }
                match tokens.next().transpose()? {
                    Some(EndTag) => Ok(()),
                    Some(token) => Err(ParserError::InvalidToken(token)),
                    None => Err(ParserError::FileCutShortAbruptlyDuringXMLDeclaration),
                }
            }
            Some(Ok(Word(_))) => Ok(()),
            Some(Ok(token)) => Err(ParserError::InvalidToken(token.clone())),
            Some(Err(_)) => {
                let _ = tokens.next();
                Err(tokens.next().unwrap().unwrap_err())
            },
            None => Err(ParserError::FileCutShortAbruptlyDuringTag),
        }
    }

    fn remove_leading_whitespace(&mut self) -> Result<(), ParserError> {
        Self::remove_leading_whitespace_from_tokens(&mut self.tokens)
    }

    fn remove_leading_whitespace_from_tokens(
        tokens: &mut TokenisedXml<T>,
    ) -> Result<(), ParserError> {
        loop {
            match tokens.peek() {
                Some(Ok(Whitespace(_))) => (),
                Some(Ok(_)) => break,
                Some(Err(_)) => return Err(tokens.next().unwrap().unwrap_err()),
                None => break,
            }
            tokens.next();
        }
        Ok(())
    }

    fn handle_ending(&mut self) -> Result<Item, ParserError> {
        if !self.root_found {
            Err(ParserError::NoRootElement)
        } else if self.names.len() != 0 {
            Err(ParserError::ElementNotClosed)
        } else {
            Ok(Item::EndFile)
        }
    }

    fn expect_token(
        &mut self,
        token: Token,
        no_token_error: ParserError,
    ) -> Result<(), ParserError> {
        match self.tokens.next().transpose()? {
            Some(t) if t == token => Ok(()),
            Some(t) => Err(ParserError::InvalidToken(t)),
            None => Err(no_token_error),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Item {
    Element(Element),
    Text(String),
    EndCurrentElement,
    EndFile,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Element {
    name: String,
    attributes: Vec<(String, String)>,
}

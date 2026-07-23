use super::tokenizer::{
    Token::{self, *},
    TokenisedXml,
};
use std::io;

pub(super) fn parse_xml(xml: TokenisedXml) -> Result<ParsedXml, ParserError> {
    ParsedXml::try_from(xml)
}

#[derive(Debug)]
pub(super) struct ParsedXml {
    pub tokens: TokenisedXml,
    names: Vec<String>,
    root_found: bool,
    next: Option<Item>,
    done: bool,
}

impl<'a> TryFrom<TokenisedXml> for ParsedXml {
    type Error = ParserError;

    fn try_from(mut tokens: TokenisedXml) -> Result<Self, Self::Error> {
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

impl ParsedXml {
    pub(super) fn next(&mut self) -> Option<Result<Item, ParserError>> {
        if self.done {
            return None
        }
        match self.get_next_element() {
            Ok(Item::EndFile) => {
                self.done = true;
                Some(Ok(Item::EndFile))
            },
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
                break
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
                    Some(Slash) => {
                        match self.tokens.next().transpose()? {
                            Some(Word(name)) => (name, false),
                            Some(token) => return Err(ParserError::InvalidToken(token)),
                            None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                        }
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
                } else if Some(name) == self.names.pop() {
                    self.expect_token(EndTag, ParserError::FileCutShortAbruptlyDuringTag)?;
                    Ok(Item::EndCurrentElement)
                } else {
                    Err(ParserError::ElementClosedIncorrectly)
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
            match self.tokens.peek().transpose()? {
                Some(Word(key)) => {
                    self.tokens.next();
                    self.expect_token(Equals, ParserError::FileCutShortAbruptlyDuringTag)?;
                    self.expect_token(QuotationMark, ParserError::FileCutShortAbruptlyDuringTag)?;
                    let mut value = String::new();
                    loop {
                        match self.tokens.next().transpose()? {
                            Some(Word(word)) => value += &word,
                            Some(Whitespace(char)) => value.push(char),
                            Some(Slash) => value.push('\\'),
                            Some(QuestionMark) => value.push('?'),
                            Some(Equals) => value.push('='),
                            Some(QuotationMark) => break,
                            Some(token) => return Err(ParserError::InvalidToken(token)),
                            None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                        }
                    }
                    result.push((key, value));
                }
                Some(_) => return Ok(result),
                None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
            }
        }
    }

    fn get_text(&mut self, current: &str) -> Result<Item, ParserError> {
        let mut result = String::from(current);
        loop {
            match self.tokens.peek().transpose()? {
                Some(Whitespace(char)) => result.push(char),
                Some(Word(word)) => result += &word,
                Some(QuotationMark) => result.push('"'),
                Some(Slash) => result.push('/'),
                Some(QuestionMark) => result.push('?'),
                Some(Equals) => result.push('='),
                Some(_) => break,
                None => return self.handle_ending(),
            }
            self.tokens.next();
        }
        Ok(Item::Text(result))
    }

    fn handle_xml_intro(tokens: &mut TokenisedXml) -> Result<(), ParserError> {
        Self::remove_leading_whitespace_from_tokens(tokens)?;
        match tokens.peek().transpose()? {
            Some(StartTag) => (),
            Some(token) => return Err(ParserError::InvalidToken(token)),
            None => return Err(ParserError::NoRootElement),
        }
        match tokens.peek_i(1).transpose()? {
            Some(QuestionMark) => {
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
            Some(Word(_)) => Ok(()),
            Some(token) => Err(ParserError::InvalidToken(token)),
            None => Err(ParserError::FileCutShortAbruptlyDuringTag),
        }
    }

    fn remove_leading_whitespace(&mut self) -> Result<(), ParserError> {
        Self::remove_leading_whitespace_from_tokens(&mut self.tokens)
    }

    fn remove_leading_whitespace_from_tokens(tokens: &mut TokenisedXml) -> Result<(), ParserError> {
        loop {
            match tokens.peek().transpose()? {
                Some(Whitespace(_)) => (),
                Some(_) => break,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ParserError {
    FileCutShortAbruptlyDuringXMLDeclaration,
    FileCutShortAbruptlyDuringTag,
    NoRootElement,
    MultipleRootElements,
    ElementNotClosed,
    ElementClosedIncorrectly,
    InvalidToken(Token),
    FileReadError(io::ErrorKind),
}

impl From<io::Error> for ParserError {
    fn from(value: io::Error) -> Self {
        ParserError::FileReadError(value.kind())
    }
}

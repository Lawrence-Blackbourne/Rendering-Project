use super::tokenizer::{TokenisedXml,
                       Token::{self, *}};
use std::iter::Peekable;

pub(super) fn parse_xml(xml: TokenisedXml) -> Result<ParsedXml, ParserError> {
    ParsedXml::try_from(xml)
}

#[derive(Clone, Debug)]
pub(super) struct ParsedXml<'a>{
    pub tokens: TokenisedXml<'a>,
    names: Vec<String>,
    root_found: bool,
    next: Option<Item>,
    error: Option<ParserError>
}

impl<'a> TryFrom<TokenisedXml<'a>> for ParsedXml<'a> {
    type Error = ParserError;

    fn try_from(mut tokens: TokenisedXml<'a>) -> Result<Self, Self::Error> {
        Self::handle_xml_intro(&mut tokens)?;

        Ok(ParsedXml{
            tokens,
            names: Vec::new(),
            root_found: false,
            next: None,
            error: None,
        })
    }
}

impl ParsedXml<'_> {
    pub(super) fn next(&mut self) -> Result<Item, ParserError> {
        if let Some(error) = &self.error {
            Err(error.clone())
        } else {
            let result = self.get_next_element();
            if let Err(e) = result {
                self.error = Some(e.clone());
                Err(e)
            } else {
                result
            }
        }
    }

    fn get_next_element(&mut self) -> Result<Item, ParserError> {
        if self.next.is_some() {
            let value = self.next.clone().unwrap();
            self.next = None;
            return Ok(value)
        }
        self.remove_leading_whitespace();
        match self.tokens.next() {
            Some(StartTag) => {
                let (name, start) = match self.tokens.next() {
                    Some(Word(name)) => (name, true),
                    Some(Slash) => {
                        match self.tokens.next() {
                            Some(Word(name)) => (name, false),
                            Some(token) => return Err(ParserError::InvalidToken(token)),
                            None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                        }
                    }
                    Some(token) => return Err(ParserError::InvalidToken(token)),
                    None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                };
                if self.root_found && self.names.len() == 0 {
                    return Err(ParserError::MultipleRootElements)
                } else if !self.root_found {
                    self.root_found = true;
                }
                if start {
                    let attributes = self.get_attributes()?;
                    match self.tokens.next() {
                        Some(EndTag) => {
                            self.names.push(name.clone());
                            Ok(Item::Element(Element{
                                name,
                                attributes,
                            }))
                        },
                        Some(Slash) => {
                            match self.tokens.next() {
                                Some(EndTag) => {
                                    self.next = Some(Item::EndCurrentElement);
                                    Ok(Item::Element(Element{
                                        name,
                                        attributes,
                                    }))
                                }
                                Some(token) => Err(ParserError::InvalidToken(token)),
                                None => Err(ParserError::FileCutShortAbruptlyDuringTag),
                            }
                        }
                        Some(token) => Err(ParserError::InvalidToken(token)),
                        None => Err(ParserError::FileCutShortAbruptlyDuringTag),
                    }
                } else if Some(name) == self.names.pop() {
                    match self.tokens.next() {
                        Some(EndTag) => Ok(Item::EndCurrentElement),
                        Some(token) => Err(ParserError::InvalidToken(token)),
                        None => Err(ParserError::FileCutShortAbruptlyDuringTag),
                    }
                } else {
                    Err(ParserError::ElementClosedIncorrectly)
                }
            }
            Some(Word(str)) => self.get_text(&str),
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
            self.remove_leading_whitespace();
            match self.tokens.peek() {
                Some(Word(key)) => {
                    self.tokens.next();
                    match self.tokens.next() {
                        Some(Equals) => (),
                        Some(token) => return Err(ParserError::InvalidToken(token)),
                        None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                    }
                    match self.tokens.next() {
                        Some(QuotationMark) => (),
                        Some(token) => return Err(ParserError::InvalidToken(token)),
                        None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                    }
                    let mut value = String::new();
                    loop {
                        match self.tokens.next() {
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
            match self.tokens.peek() {
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
        Self::remove_leading_whitespace_from_tokens(tokens);
        match tokens.peek() {
            Some(StartTag) => (),
            Some(token) => return Err(ParserError::InvalidToken(token)),
            None => return Err(ParserError::NoRootElement),
        }
        match tokens.peek_n(2) {
            Some(QuestionMark) => {
                tokens.next();
                tokens.next();
                loop {
                    match tokens.next() {
                        Some(QuestionMark) => break,
                        Some(StartTag) => return Err(ParserError::InvalidToken(StartTag)),
                        Some(Slash) => return Err(ParserError::InvalidToken(Slash)),
                        Some(EndTag) => return Err(ParserError::InvalidToken(EndTag)),
                        Some(_) => (),
                        None => (),
                    }
                }
                match tokens.next() {
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

    fn remove_leading_whitespace(&mut self) {
        Self::remove_leading_whitespace_from_tokens(&mut self.tokens);
    }

    fn remove_leading_whitespace_from_tokens(tokens: &mut TokenisedXml) {
        loop {
            match tokens.peek() {
                Some(Whitespace(_)) => (),
                Some(_) => return,
                None => return,
            }
            tokens.next();
        }
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
}
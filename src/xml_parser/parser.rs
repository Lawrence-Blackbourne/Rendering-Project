use super::tokenizer::{TokenisedXml,
                       Token::{self, *}};
use std::iter::Peekable;

pub(super) fn parse_xml(xml: TokenisedXml) -> Result<ParsedXml, ParserError> {
    let mut result = ParsedXml::try_from(xml).unwrap();
    for _ in 0..100 {
        println!("{:?}", result.tokens.next())
    }
    panic!()
}

#[derive(Clone, Debug)]
pub(super) struct ParsedXml<'a>{
    root_name: String,
    tokens: Peekable<TokenisedXml<'a>>,
}

impl<'a> TryFrom<TokenisedXml<'a>> for ParsedXml<'a> {
    type Error = ParserError;

    fn try_from(tokens: TokenisedXml<'a>) -> Result<Self, Self::Error> {
        let mut tokens = tokens.peekable();
        let name = Self::handle_xml_intro(&mut tokens)?;

        Ok(ParsedXml{
            root_name: name,
            tokens,
        })
    }
}

impl Iterator for ParsedXml<'_> {
    type Item = Result<Self, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl ParsedXml<'_> {
    fn handle_xml_intro(tokens: &mut Peekable<TokenisedXml>) -> Result<String, ParserError> {
        match tokens.next() {
            Some(StartTag) => (),
            Some(token) => return Err(ParserError::InvalidToken(token)),
            None => return Err(ParserError::XMLCutShort),
        }
        match tokens.next() {
            Some(QuestionMark) => {
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
                    Some(EndTag) => (),
                    Some(token) => return Err(ParserError::InvalidToken(token)),
                    None => return Err(ParserError::XMLCutShort),
                }
                match tokens.next() {
                    Some(StartTag) => (),
                    Some(token) => return Err(ParserError::InvalidToken(token)),
                    None => return Err(ParserError::XMLCutShort),
                }
                Self::get_xml_name(tokens)
            }
            Some(Text(txt)) => {
                Self::get_xml_name(tokens)
            }
            Some(token) => Err(ParserError::InvalidToken(token)),
            None => Err(ParserError::XMLCutShort),
        }
    }

    fn get_xml_name(tokens: &mut Peekable<TokenisedXml>) -> Result<String, ParserError> {
        let name = match tokens.next() {
            Some(Text(name)) => name,
            Some(token) => return Err(ParserError::InvalidToken(token)),
            None => return Err(ParserError::XMLCutShort),
        };
        match tokens.next() {
            Some(EndTag) => (),
            Some(token) => return Err(ParserError::InvalidToken(token)),
            None => return Err(ParserError::XMLCutShort),
        };
        Ok(name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedXmlElement {
    name: String,
    data: (String, String),
    parts: Vec<ParsedXmlElement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ParsedXmlElementParts {
    Text(String),
    Element(ParsedXmlElement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ParserError {
    XMLCutShort,
    InvalidToken(Token),
}
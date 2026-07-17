use std::str::Chars;

pub(super) fn tokenize_xml(xml: &str) -> TokenisedXml {
    TokenisedXml::from(xml)
}

#[derive(Clone, Debug)]
pub(super) struct TokenisedXml<'a> {
    internal: std::iter::Peekable<Chars<'a>>,
}

impl<'a> From<&'a str> for TokenisedXml<'a> {
    fn from(value: &'a str) ->  Self {
        TokenisedXml {
            internal: value.chars().peekable(),
        }
    }
}

impl Iterator for TokenisedXml<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.internal.next() {
            Some('<') => Some(Token::StartTag),
            Some('>') => Some(Token::EndTag),
            Some('=') => Some(Token::Equals),
            Some('"') => Some(Token::QuotationMark),
            Some('?') => Some(Token::QuestionMark),
            Some('/') => Some(Token::Slash),
            Some(' ') => self.next(),
            Some('\n') => self.next(),
            Some('\t') => self.next(),
            Some(char) => {
                let mut txt = String::from(char);
                loop {
                    match self.internal.peek() {
                        Some('<') => return Some(Token::Text(txt)),
                        Some('>') => return Some(Token::Text(txt)),
                        Some('=') => return Some(Token::Text(txt)),
                        Some('"') => return Some(Token::Text(txt)),
                        Some('?') => return Some(Token::Text(txt)),
                        Some(char) => txt.push(*char),
                        None => return Some(Token::Text(txt)),
                    }
                    self.internal.next();
                }
            },
            None => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Token {
    StartTag,
    EndTag,
    QuestionMark,
    Slash,
    Equals,
    QuotationMark,
    Text(String),
}
use std::str::Chars;
use std::collections::VecDeque;

pub(super) fn tokenize_xml(xml: &str) -> TokenisedXml {
    TokenisedXml::from(xml)
}

#[derive(Clone, Debug)]
pub(super) struct TokenisedXml<'a> {
    internal: std::iter::Peekable<Chars<'a>>,
    next: VecDeque<Token>,
}

impl<'a> From<&'a str> for TokenisedXml<'a> {
    fn from(value: &'a str) ->  Self {
        TokenisedXml {
            internal: value.chars().peekable(),
            next: VecDeque::new(),
        }
    }
}

impl Iterator for TokenisedXml<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.next.is_empty() {
            self.next.pop_front()
        } else {
            self.get_next_token()
        }
    }
}

impl TokenisedXml<'_> {
    pub(super) fn peek(&mut self) -> Option<Token> {
        if self.next.is_empty() {
            let next = self.get_next_token()?;
            self.next.push_back(next);
        }
        Some(self.next.get(0)?.clone())
    }

    pub(super) fn peek_n(&mut self, n: usize) -> Option<Token> {
        if n == 0 {
            return None
        }
        if self.next.len() < n {
            let mut remaining = n - self.next.len();
            while remaining > 0 {
                let next = self.get_next_token()?;
                self.next.push_back(next);
                remaining -= 1;
            }
        }
        Some(self.next.get(n - 1)?.clone())
    }

    fn get_next_token(&mut self) -> Option<Token> {
        match self.internal.next() {
            Some('<') => Some(Token::StartTag),
            Some('>') => Some(Token::EndTag),
            Some('=') => Some(Token::Equals),
            Some('"') => Some(Token::QuotationMark),
            Some('?') => Some(Token::QuestionMark),
            Some('/') => Some(Token::Slash),
            Some(char) if char.is_whitespace() => Some(Token::Whitespace(char)),
            Some(char) => {
                let mut txt = String::from(char);
                loop {
                    match self.internal.peek() {
                        Some('<') => return Some(Token::Word(txt)),
                        Some('>') => return Some(Token::Word(txt)),
                        Some('=') => return Some(Token::Word(txt)),
                        Some('"') => return Some(Token::Word(txt)),
                        Some('?') => return Some(Token::Word(txt)),
                        Some(char) if char.is_whitespace() => return Some(Token::Word(txt)),
                        Some(char) => txt.push(*char),
                        None => return Some(Token::Word(txt)),
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
    Word(String),
    Whitespace(char),
}
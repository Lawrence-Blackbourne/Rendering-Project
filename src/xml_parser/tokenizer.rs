use super::parser::ParserError;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub(super) fn tokenize_xml(file_path: &Path) -> Result<TokenisedXml, ParserError> {
    Ok(TokenisedXml::try_from(file_path)?)
}

#[derive(Debug)]
pub(super) struct TokenisedXml {
    file: BufReader<File>,
    current_line: VecDeque<char>,
    next: VecDeque<Option<Result<Token, ParserError>>>,
    finished: bool,
}

impl TryFrom<&Path> for TokenisedXml {
    type Error = io::Error;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let file = BufReader::new(File::open(file_path)?);
        Ok(Self {
            file,
            current_line: VecDeque::new(),
            next: VecDeque::new(),
            finished: false,
        })
    }
}

impl Iterator for TokenisedXml {
    type Item = Result<Token, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_empty() {
            self.add_token_to_queue();
        }
        self.next.pop_front().unwrap().clone()
    }
}

impl TokenisedXml {
    pub(super) fn peek(&mut self) -> Option<Result<Token, ParserError>> {
        if self.next.is_empty() {
            self.add_token_to_queue();
        }
        self.next.get(0).unwrap().clone()
    }

    /// Returns the ith token without consuming any.
    /// Indexing starts at 0, meaning that `peek_i(0)` will return the same value as `peek()`.
    pub(super) fn peek_i(&mut self, n: usize) -> Option<Result<Token, ParserError>> {
        if self.next.len() < n + 1 {
            let mut remaining = n + 1 - self.next.len();
            while remaining > 0 {
                self.add_token_to_queue();
                remaining -= 1;
            }
        }
        self.next.get(n).unwrap().clone()
    }

    /// A layer on top of `get_next_token` to ensure that it is not called again once it gives a
    /// bad response.
    fn add_token_to_queue(&mut self) -> () {
        if self.finished {
            self.next.push_back(None)
        } else {
            match self.get_next_token() {
                Some(Ok(t)) => self.next.push_back(Some(Ok(t))),
                Some(Err(e)) => {
                    self.finished = true;
                    self.next.push_back(Some(Err(e)))
                },
                None => {
                    self.finished = true;
                    self.next.push_back(None)
                }
            }
        }
    }

    /// The behaviour after an `Ok(Err(e))` or `None` result is returned is undefined.
    fn get_next_token(&mut self) -> Option<Result<Token, ParserError>> {
        match self.pop_char()? {
            Ok('<') => Some(Ok(Token::StartTag)),
            Ok('>') => Some(Ok(Token::EndTag)),
            Ok('=') => Some(Ok(Token::Equals)),
            Ok('"') => Some(Ok(Token::QuotationMark)),
            Ok('?') => Some(Ok(Token::QuestionMark)),
            Ok('/') => Some(Ok(Token::Slash)),
            Ok(char) if char.is_whitespace() => Some(Ok(Token::Whitespace(char))),
            Ok(char) => {
                let mut txt = String::from(char);
                loop {
                    match self.peek_char() {
                        Some(Ok('<')) => break,
                        Some(Ok('>')) => break,
                        Some(Ok('=')) => break,
                        Some(Ok('"')) => break,
                        Some(Ok('?')) => break,
                        Some(Ok(char)) if char.is_whitespace() => break,
                        Some(Ok(char)) => txt.push(char),
                        Some(Err(e)) => return Some(Err(e.into())),
                        None => break,
                    }
                    match self.consume_char() {
                        Ok(()) => (),
                        Err(e) => return Some(Err(e.into())),
                    }
                }
                Some(Ok(Token::Word(txt)))
            }
            Err(e) => Some(Err(e.into())),
        }
    }

    fn peek_char(&mut self) -> Option<Result<char, io::Error>> {
        match self.update_line() {
            Ok(true) => Some(Ok(*self.current_line.get(0).unwrap())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }

    fn pop_char(&mut self) -> Option<Result<char, io::Error>> {
        match self.update_line() {
            Ok(true) => Some(Ok(self.current_line.pop_front().unwrap())),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }

    // Consumes the next char in the sequence without returning it, only returning if there is an
    // error or not.
    fn consume_char(&mut self) -> Result<(), io::Error> {
        match self.pop_char() {
            Some(Ok(_)) => Ok(()),
            Some(Err(e)) => Err(e),
            None => Ok(()),
        }
    }

    /// A value of `Ok(true)` means that there is definitely at least one character to read.
    /// A value of `Ok(false)` means that we have reached the end of the file and there is no more
    /// data to read.
    fn update_line(&mut self) -> Result<bool, io::Error> {
        if self.current_line.is_empty() {
            self.get_new_line()
        } else {
            Ok(true)
        }
    }

    /// A value of `Ok(false)` means that we have reached the end of the file
    fn get_new_line(&mut self) -> Result<bool, io::Error> {
        let mut buf = String::new();
        match self.file.read_line(&mut buf) {
            Ok(0) => return Ok(false),
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        for char in buf.chars() {
            self.current_line.push_back(char)
        }
        Ok(true)
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

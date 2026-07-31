use super::ParserError;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

pub(super) fn tokenise_xml_file(file_path: &Path) -> Result<TokenisedXml<File>, ParserError> {
    Ok(TokenisedXml::from(File::open(file_path)?))
}

#[derive(Debug)]
pub(super) struct TokenisedXml<T: Read> {
    text: BufReader<T>,
    next: VecDeque<Result<Token, ParserError>>,
    finished: bool,
    // The current section of the file we are working from
    buffer: String,
    // How far into the buffer we are currently looking
    byte_offset: usize,
}

impl<T: Read> From<T> for TokenisedXml<T> {

    /// Creates a new instance of the tokeniser from the provided text.
    fn from(text: T) -> Self {
        let text = BufReader::new(text);
        Self {
            text,
            next: VecDeque::new(),
            finished: false,
            buffer: String::new(),
            byte_offset: 0,
        }
    }
}

impl<T: Read> Iterator for TokenisedXml<T> {
    type Item = Result<Token, ParserError>;

    /// Returns the next token available, returning None if we have reached the end of the stream or
    /// Some(Err(e)) if we have run into some error.
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_empty() {
            self.add_token_to_queue()
        }
        self.next.pop_front()
    }
}

impl<T: Read> TokenisedXml<T> {
    pub(super) fn peek(&mut self) -> Option<&Result<Token, ParserError>> {
        if self.next.is_empty() {
            self.add_token_to_queue();
        }
        self.next.get(0)
    }

    /// Returns the ith token without consuming any.
    /// Indexing starts at 0, meaning that `peek_n(0)` will return the same value as `peek()`.
    pub(super) fn peek_n(&mut self, n: usize) -> Option<&Result<Token, ParserError>> {
        if self.next.len() <= n {
            let mut remaining = n + 1 - self.next.len();
            while remaining > 0 && !self.finished {
                self.add_token_to_queue();
                remaining -= 1;
            }
        }
        self.next.get(n)
    }

    /// Gets the next token and pushed it to the queue
    /// Also acts as a layer on top of `get_next_token` to ensure that it is not called again once
    /// it gives a bad response.
    fn add_token_to_queue(&mut self) {
        if !self.finished {
            let result = self.get_next_token();
            self.finished = matches!(result, Some(Err(_)) | None);
            if let Some(r) = result {
                self.next.push_back(r);
            }
        }
    }

    /// Returns the next token in the sequence.
    /// The behaviour after an `Ok(Err(e))` or `None` result is returned is undefined.
    /// For this reason, this should be only used through add_token_to_queue
    fn get_next_token(&mut self) -> Option<Result<Token, ParserError>> {
        match self.update_line() {
            Ok(true) => (),
            Ok(false) => return None,
            Err(e) => return Some(Err(e.into())),
        }
        match self.pop_char().unwrap() {
            '<' => Some(Ok(Token::StartTag)),
            '>' => Some(Ok(Token::EndTag)),
            '=' => Some(Ok(Token::Equals)),
            '"' => Some(Ok(Token::QuotationMark)),
            '?' => Some(Ok(Token::QuestionMark)),
            '/' => Some(Ok(Token::Slash)),
            char if char.is_whitespace() => Some(Ok(Token::Whitespace(char))),
            char => Some(Ok(Token::Word(self.pop_word(char))))
        }
    }

    /// Pops off the next char in the buffer.
    /// Will only pop a char if the buffer has chars remaining.
    fn pop_char(&mut self) -> Option<char> {
        if self.has_data_in_buffer() {
            let result = self.buffer[self.byte_offset..].chars().next().unwrap();
            self.byte_offset += result.len_utf8();
            Some(result)
        } else {
            None
        }
    }

    /// Gets the next word from the buffer.
    /// Will exit at the end of the buffer, as no word can cross multiple lines
    fn pop_word(&mut self, first_char: char) -> String {
        let mut word_length = 0;
        loop {
            match self.buffer[self.byte_offset + word_length..].chars().next() {
                Some('<' | '>' | '=' | '"' | '?' | '/') => break,
                Some(char) if char.is_whitespace() => break,
                Some(char) => word_length += char.len_utf8(),
                None => break,
            }
        }
        let result = String::from(
            &self.buffer[self.byte_offset - first_char.len_utf8()..self.byte_offset + word_length]
        );
        self.byte_offset += word_length;
        result
    }

    /// Returns if there is data available in the buffer.
    fn has_data_in_buffer(&self) -> bool {
        self.byte_offset < self.buffer.len()
    }

    /// A value of `Ok(true)` means that there is definitely at least one character to read.
    /// A value of `Ok(false)` means that we have reached the end of the file and there is no more
    /// data to read.
    fn update_line(&mut self) -> Result<bool, io::Error> {
        if self.byte_offset >= self.buffer.len() {
            self.get_new_line()
        } else {
            Ok(true)
        }
    }

    /// A value of `Ok(false)` means that we have reached the end of the file, and there is no new
    /// line
    fn get_new_line(&mut self) -> Result<bool, io::Error> {
        self.buffer.clear();
        self.byte_offset = 0;
        match self.text.read_line(&mut self.buffer) {
            Ok(0) => Ok(false),
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

/// Describes a single token found in the XML stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Token {
    StartTag,
    EndTag,
    QuestionMark,
    Slash,
    Equals,
    QuotationMark,
    Word(String),
    Whitespace(char),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::StartTag => write!(f, "Start tag: '<'"),
            Token::EndTag => write!(f, "End tag: '>'"),
            Token::QuestionMark => write!(f, "Question mark: '?'"),
            Token::Slash => write!(f, "Slash: '/'"),
            Token::Equals => write!(f, "Equals: '='"),
            Token::QuotationMark => write!(f, "Quotation mark: '\"'"),
            Token::Word(word) => write!(f, "Word: \"{word}\""),
            Token::Whitespace(c) => write!(f, "Whitespace character: '{c}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_tokenise_xml_file() {
        let mut tokens = tokenise_xml_file(Path::new(super::super::VULKAN_XML_PATH)).unwrap();
        loop {
            let token = tokens.next();
            match token {
                None => break,
                Some(_) => (),
            }
        }
        assert!(tokens.next().is_none())
    }

    #[test]
    fn cannot_tokenise_invalid_file() {
        let _ = tokenise_xml_file(Path::new("")).unwrap_err();
    }

    #[test]
    fn test_individual_tokens() {
        let test_data = [
            ("<", Token::StartTag),
            (">", Token::EndTag),
            ("?", Token::QuestionMark),
            ("/", Token::Slash),
            ("=", Token::Equals),
            ("\"", Token::QuotationMark),
            (" ", Token::Whitespace(' ')),
            ("\t", Token::Whitespace('\t')),
            ("\n", Token::Whitespace('\n')),
            ("a", Token::Word(String::from("a"))),
            ("abc", Token::Word(String::from("abc"))),
        ];
        for test in test_data {
            let mut xml = TokenisedXml::from(TestReader::from(test.0));
            assert_eq!(xml.next().unwrap().unwrap(), test.1);
            assert_eq!(xml.next().is_none(), true);
            assert_eq!(xml.next().is_none(), true);
        }
    }

    #[test]
    fn test_failures() {
        let test_data = [
            ("", Vec::new()),
            ("<", Vec::new()),
            ("<\n", vec![Token::StartTag, Token::Whitespace('\n')]),
            (
                "<? test?as<much>as=you\"possibly\ncan\t>",
                vec![
                    Token::StartTag,
                    Token::QuestionMark,
                    Token::Whitespace(' '),
                    Token::Word(String::from("test")),
                    Token::QuestionMark,
                    Token::Word(String::from("as")),
                    Token::StartTag,
                    Token::Word(String::from("much")),
                    Token::EndTag,
                    Token::Word(String::from("as")),
                    Token::Equals,
                    Token::Word(String::from("you")),
                    Token::QuotationMark,
                    Token::Word(String::from("possibly")),
                    Token::Whitespace('\n'),
                ],
            ),
        ];
        for test in test_data {
            let mut xml = TokenisedXml::from(TestReader::from(test.0).with_fail_on_end(true));
            for token in test.1 {
                assert_eq!(xml.next().unwrap().unwrap(), token);
            }
            assert_eq!(
                xml.next().unwrap().unwrap_err().to_string(),
                PARSER_ERROR_TEST_STRING
            );
            assert_eq!(xml.next().is_none(), true);
        }
    }

    #[test]
    fn test_peeking() {
        let mut xml = TokenisedXml::from(TestReader::from("<Test 猫/>\n").with_fail_on_end(true));
        let result = vec![
            Token::StartTag,
            Token::Word(String::from("Test")),
            Token::Whitespace(' '),
            Token::Word(String::from("猫")),
            Token::Slash,
            Token::EndTag,
            Token::Whitespace('\n'),
        ];
        assert_eq!(*xml.peek().unwrap().as_ref().unwrap(), result.get(0).unwrap().clone());
        assert_eq!(*xml.peek().unwrap().as_ref().unwrap(), result.get(0).unwrap().clone());
        for i in 0..result.len() {
            assert_eq!(
                *xml.peek_n(i).unwrap().as_ref().unwrap(),
                result.get(i).unwrap().clone()
            );
        }
        assert_eq!(xml.peek_n(result.len() + 1).is_none(), true);
        assert_eq!(
            xml.peek_n(result.len()).unwrap().as_ref().unwrap_err().to_string(),
            PARSER_ERROR_TEST_STRING
        );
        assert_eq!(*xml.peek().unwrap().as_ref().unwrap(), result.get(0).unwrap().clone());
        for test in result {
            assert_eq!(xml.next().unwrap().unwrap(), test);
        }
        assert_eq!(xml.next().unwrap().unwrap_err().to_string(), PARSER_ERROR_TEST_STRING);
        assert_eq!(xml.next().is_none(), true);
    }

    #[test]
    fn test_token_debug() {
        let data = [
            (Token::StartTag, "Start tag: '<'"),
            (Token::EndTag, "End tag: '>'"),
            (Token::QuestionMark, "Question mark: '?'"),
            (Token::Slash, "Slash: '/'"),
            (Token::Equals, "Equals: '='"),
            (Token::QuotationMark, "Quotation mark: '\"'"),
            (Token::Word(String::from("")), "Word: \"\""),
            (Token::Word(String::from("Test")), "Word: \"Test\""),
            (Token::Whitespace(' '), "Whitespace character: ' '"),
            (Token::Whitespace('猫'), "Whitespace character: '猫'"),
        ];
        for test in data {
            assert_eq!(test.0.to_string(), test.1);
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestReader {
        data: io::Cursor<Vec<u8>>,
        fail: bool,
    }

    const TEST_READER_ERROR_STRING: &str = "Test";
    const PARSER_ERROR_TEST_STRING: &str = "an io error occurred: \"Test\"";

    impl From<&str> for TestReader {
        fn from(value: &str) -> Self {
            Self {
                data: io::Cursor::new(value.as_bytes().to_vec()),
                fail: false,
            }
        }
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match self.data.read(buf) {
                Ok(0) if self.fail =>
                    Err(io::Error::new(io::ErrorKind::Other, TEST_READER_ERROR_STRING)),
                other => other,
            }
        }
    }

    impl TestReader {
        fn with_fail_on_end(mut self, value: bool) -> Self {
            self.fail = value;
            self
        }
    }
}

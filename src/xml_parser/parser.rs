use super::ParserError;
use super::tokeniser::{
    Token::{self, *},
    TokenisedXml, tokenise_xml_file,
};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(super) fn parse_xml_file(file_path: &Path) -> Result<ParsedXml<File>, ParserError> {
    ParsedXml::try_from(tokenise_xml_file(file_path)?)
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

    /// Skips to the end of the element currently being looked at by the parser.
    /// If the parser is outside the root element, skips to the very end of the file.
    pub(super) fn skip_current_element(&mut self) -> Result<(), ParserError> {
        let mut current_depth = 1;
        loop {
            match self.next() {
                Some(Ok(Item::Element { .. })) => current_depth += 1,
                Some(Ok(Item::EndCurrentElement)) => current_depth -= 1,
                Some(Ok(_)) => (),
                Some(Err(e)) => return Err(e),
                None => return Ok(()),
            }
            if current_depth == 0 {
                break;
            }
        }
        Ok(())
    }

    fn get_next_element(&mut self) -> Result<Item, ParserError> {
        if self.next.is_some() {
            let value = self.next.clone().unwrap();
            self.next = None;
            return Ok(value);
        }
        let whitespace = self.remove_leading_whitespace()?;
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
                if self.root_found && self.names.len() == 0 && start {
                    return Err(ParserError::MultipleRootElements);
                } else if !self.root_found {
                    self.root_found = true;
                }
                if start {
                    let attributes = self.get_attributes()?;
                    match self.tokens.next().transpose()? {
                        Some(EndTag) => {
                            self.names.push(name.clone());
                            Ok(Item::Element { name, attributes })
                        }
                        Some(Slash) => {
                            self.expect_token(EndTag, ParserError::FileCutShortAbruptlyDuringTag)?;
                            self.next = Some(Item::EndCurrentElement);
                            Ok(Item::Element { name, attributes })
                        }
                        Some(token) => Err(ParserError::InvalidToken(token)),
                        None => Err(ParserError::FileCutShortAbruptlyDuringTag),
                    }
                } else {
                    match self.names.pop() {
                        Some(correct) if correct == name => {
                            self.expect_token(EndTag, ParserError::FileCutShortAbruptlyDuringTag)?;
                            Ok(Item::EndCurrentElement)
                        }
                        Some(correct) => Err(ParserError::ElementsClosedOutOfOrder {
                            found: name,
                            correct,
                        }),
                        None => Err(ParserError::ElementClosedAfterRootElementClosed(name)),
                    }
                }
            }
            Some(Word(str)) if !self.names.is_empty() => {
                self.get_text((whitespace + str.as_str()).as_str())
            }
            Some(QuotationMark) if !self.names.is_empty() => {
                self.get_text((whitespace + "\"").as_str())
            }
            Some(Slash) if !self.names.is_empty() => self.get_text((whitespace + "\\").as_str()),
            Some(QuestionMark) if !self.names.is_empty() => {
                self.get_text((whitespace + "?").as_str())
            }
            Some(Equals) if !self.names.is_empty() => self.get_text((whitespace + "=").as_str()),
            Some(EndTag) if !self.names.is_empty() => self.get_text((whitespace + ">").as_str()),
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
                    let Word(key) = self.tokens.next().unwrap()? else {
                        unreachable!()
                    };
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
                            Some(EndTag) => value.push('>'),
                            Some(QuotationMark) => break,
                            Some(token) => return Err(ParserError::InvalidToken(token)),
                            None => return Err(ParserError::FileCutShortAbruptlyDuringTag),
                        }
                    }
                    result.push((key, value));
                }
                Some(Ok(_)) => return Ok(result),
                Some(Err(_)) => return Err(self.tokens.next().unwrap().unwrap_err()),
                None => return Ok(result),
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
                Some(Ok(EndTag)) => result.push('>'),
                Some(Ok(_)) => break,
                Some(Err(_)) => return Err(self.tokens.next().unwrap().unwrap_err()),
                None => return self.handle_ending(),
            }
            self.tokens.next();
        }
        Ok(Item::Text(result))
    }

    fn handle_xml_intro(tokens: &mut TokenisedXml<T>) -> Result<(), ParserError> {
        loop {
            match tokens.peek() {
                Some(Ok(StartTag)) => break,
                Some(Ok(Whitespace(_))) => {
                    let _ = tokens.next();
                }
                Some(Ok(token)) => return Err(ParserError::InvalidToken(token.clone())),
                Some(Err(_)) => return Err(tokens.next().unwrap().unwrap_err()),
                None => return Err(ParserError::NoRootElement),
            }
        }
        match tokens.peek_n(1) {
            Some(Ok(QuestionMark)) => {
                tokens.next();
                tokens.next();
                loop {
                    match tokens.next().transpose()? {
                        Some(QuestionMark) => break,
                        Some(StartTag) => return Err(ParserError::InvalidToken(StartTag)),
                        Some(EndTag) => return Err(ParserError::InvalidToken(EndTag)),
                        Some(_) => (),
                        None => return Err(ParserError::FileCutShortAbruptlyDuringXMLDeclaration),
                    }
                }
                match tokens.next().transpose()? {
                    Some(EndTag) => Ok(()),
                    Some(token) => Err(ParserError::InvalidToken(token)),
                    None => Err(ParserError::FileCutShortAbruptlyDuringXMLDeclaration),
                }
            }
            // Even if the following items cannot be valid, we should still successfully create the
            // parser, because the issue here would be in the first element and not the intro.
            // Erroring here would cause unpredictable error behaviour.
            Some(Ok(_)) => Ok(()),
            Some(Err(_)) => {
                let _ = tokens.next();
                Err(tokens.next().unwrap().unwrap_err())
            }
            None => Ok(()),
        }
    }

    fn remove_leading_whitespace(&mut self) -> Result<String, ParserError> {
        let mut result = String::new();
        loop {
            match self.tokens.peek() {
                Some(Ok(Whitespace(c))) => {
                    result.push(*c);
                }
                Some(Ok(_)) => break,
                Some(Err(_)) => return Err(self.tokens.next().unwrap().unwrap_err()),
                None => break,
            }
            self.tokens.next();
        }
        Ok(result)
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
    Element {
        name: String,
        attributes: Vec<(String, String)>,
    },
    Text(String),
    EndCurrentElement,
    EndFile,
}

#[cfg(test)]
mod tests {
    use super::super::tests::{PARSER_IO_ERROR_TEST_STRING, TestReader};
    use super::*;

    #[test]
    fn can_parse_text() {
        let mut xml = parse_xml_file(&Path::new("vulkan_XML/vk.xml")).unwrap();
        loop {
            match xml.next().unwrap().unwrap() {
                Item::EndFile => break,
                _ => (),
            }
        }
        assert!(xml.next().is_none());
    }

    #[test]
    fn cannot_parse_failing_text() {
        let _ = parse_xml_file(&Path::new("")).unwrap_err();
    }

    #[test]
    fn test_failing_intros() {
        let data = [
            ("", ParserError::NoRootElement),
            (
                "Text",
                ParserError::InvalidToken(Word(String::from("Text"))),
            ),
            (">", ParserError::InvalidToken(EndTag)),
            ("?", ParserError::InvalidToken(QuestionMark)),
            ("/", ParserError::InvalidToken(Slash)),
            ("=", ParserError::InvalidToken(Equals)),
            ("\n\t \"", ParserError::InvalidToken(QuotationMark)),
            ("?", ParserError::InvalidToken(QuestionMark)),
            ("<?", ParserError::FileCutShortAbruptlyDuringXMLDeclaration),
            ("<?<", ParserError::InvalidToken(StartTag)),
            ("<?>", ParserError::InvalidToken(EndTag)),
            ("<??", ParserError::FileCutShortAbruptlyDuringXMLDeclaration),
            ("<??<", ParserError::InvalidToken(StartTag)),
            ("<??\"", ParserError::InvalidToken(QuotationMark)),
            ("<??=", ParserError::InvalidToken(Equals)),
            ("<???", ParserError::InvalidToken(QuestionMark)),
        ];
        for test in data {
            assert_eq!(
                ParsedXml::try_from(TokenisedXml::new(TestReader::new(test.0)))
                    .unwrap_err()
                    .to_string(),
                test.1.to_string()
            );
            assert_eq!(
                ParsedXml::try_from(TokenisedXml::new(
                    TestReader::new(test.0).with_fail_on_end(true)
                ))
                .unwrap_err()
                .to_string(),
                PARSER_IO_ERROR_TEST_STRING,
            );
        }
    }

    #[test]
    fn test_immediate_error() {
        assert_eq!(
            ParsedXml::try_from(TokenisedXml::new(TestReader::new("")))
                .unwrap_err()
                .to_string(),
            ParserError::NoRootElement.to_string(),
        );
        assert_eq!(
            ParsedXml::try_from(TokenisedXml::new(
                TestReader::new("<?").with_fail_on_end(true)
            ))
            .unwrap_err()
            .to_string(),
            PARSER_IO_ERROR_TEST_STRING,
        );
    }

    #[test]
    fn test_valid_intro() {
        check_xml_gives_correct_values("\n<??>", Vec::new(), Some(ParserError::NoRootElement));
        check_xml_gives_correct_values(
            "<? word test \n test = \"test / words?>",
            Vec::new(),
            Some(ParserError::NoRootElement),
        );
    }

    #[test]
    fn test_element() {
        check_xml_gives_correct_values(
            "<outer>\n<middle>\n<inner/></middle></outer>",
            vec![
                Item::Element {
                    name: String::from("outer"),
                    attributes: Vec::new(),
                },
                Item::Element {
                    name: String::from("middle"),
                    attributes: Vec::new(),
                },
                Item::Element {
                    name: String::from("inner"),
                    attributes: Vec::new(),
                },
                Item::EndCurrentElement,
                Item::EndCurrentElement,
                Item::EndCurrentElement,
            ],
            None,
        );
    }

    #[test]
    fn test_text() {
        check_xml_gives_correct_values(
            "<outer>\n text 1<inner1>?text 2>?=/\"</inner1><inner2>>text3</inner2>\"text4 word \
            猫<inner3>=text5</inner3></outer>",
            vec![
                Item::Element {
                    name: String::from("outer"),
                    attributes: Vec::new(),
                },
                Item::Text(String::from("\n text 1")),
                Item::Element {
                    name: String::from("inner1"),
                    attributes: Vec::new(),
                },
                Item::Text(String::from("?text 2>?=/\"")),
                Item::EndCurrentElement,
                Item::Element {
                    name: String::from("inner2"),
                    attributes: Vec::new(),
                },
                Item::Text(String::from(">text3")),
                Item::EndCurrentElement,
                Item::Text(String::from("\"text4 word 猫")),
                Item::Element {
                    name: String::from("inner3"),
                    attributes: Vec::new(),
                },
                Item::Text(String::from("=text5")),
                Item::EndCurrentElement,
                Item::EndCurrentElement,
            ],
            None,
        );
    }

    #[test]
    fn test_attributes() {
        check_xml_gives_correct_values(
            "<outer name1=\"value1\">\n text 1<inner1 猫=\"猫\" name2=\"value2\">\ntext 2</inner1>\
            <inner2 name3=\"value3\" name4=\"value4\"/>text3 other words 猫</outer>",
            vec![
                Item::Element {
                    name: String::from("outer"),
                    attributes: vec![(String::from("name1"), String::from("value1"))],
                },
                Item::Text(String::from("\n text 1")),
                Item::Element {
                    name: String::from("inner1"),
                    attributes: vec![
                        (String::from("猫"), String::from("猫")),
                        (String::from("name2"), String::from("value2")),
                    ],
                },
                Item::Text(String::from("\ntext 2")),
                Item::EndCurrentElement,
                Item::Element {
                    name: String::from("inner2"),
                    attributes: vec![
                        (String::from("name3"), String::from("value3")),
                        (String::from("name4"), String::from("value4")),
                    ],
                },
                Item::EndCurrentElement,
                Item::Text(String::from("text3 other words 猫")),
                Item::EndCurrentElement,
            ],
            None,
        );
    }

    #[test]
    fn test_self_closing_root_element() {
        check_xml_gives_correct_values(
            "<element name=\"value1\" 猫=\"value2\"/>",
            vec![
                Item::Element {
                    name: String::from("element"),
                    attributes: vec![
                        (String::from("name"), String::from("value1")),
                        (String::from("猫"), String::from("value2")),
                    ],
                },
                Item::EndCurrentElement,
            ],
            None,
        );
    }

    #[test]
    fn test_two_root_elements() {
        check_xml_gives_correct_values(
            "<outer1/><outer2></outer2>",
            vec![
                Item::Element {
                    name: String::from("outer1"),
                    attributes: Vec::new(),
                },
                Item::EndCurrentElement,
            ],
            Some(ParserError::MultipleRootElements),
        );
    }

    #[test]
    fn test_zero_root_elements() {
        check_xml_gives_correct_values(
            "<? word test \n test = \"test / words?>",
            Vec::new(),
            Some(ParserError::NoRootElement),
        );
    }

    #[test]
    fn test_errors_in_element() {
        #[rustfmt::skip]
        let data = [
            ("<??>\n\tElement", ParserError::InvalidToken(Word(String::from("Element")))),
            ("<??>=", ParserError::InvalidToken(Equals)),
            ("<??>>", ParserError::InvalidToken(EndTag)),
            ("<??>/", ParserError::InvalidToken(Slash)),
            ("<??>\"", ParserError::InvalidToken(QuotationMark)),
            ("<??>?", ParserError::InvalidToken(QuestionMark)),
            ("<", ParserError::FileCutShortAbruptlyDuringTag),
            ("<=", ParserError::InvalidToken(Equals)),
            ("</=", ParserError::InvalidToken(Equals)),
            ("<??><>", ParserError::InvalidToken(EndTag)),
            ("</", ParserError::FileCutShortAbruptlyDuringTag),
            ("<\"", ParserError::InvalidToken(QuotationMark)),
            ("< ", ParserError::InvalidToken(Whitespace(' '))),
            ("<??><<", ParserError::InvalidToken(StartTag)),
            ("<??><?", ParserError::InvalidToken(QuestionMark)),
            ("<element", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element=", ParserError::InvalidToken(Equals)),
            ("<element/", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element\"", ParserError::InvalidToken(QuotationMark)),
            ("<??><element<", ParserError::InvalidToken(StartTag)),
            ("<element?", ParserError::InvalidToken(QuestionMark)),
            ("<element/<", ParserError::InvalidToken(StartTag)),
            ("<element/=", ParserError::InvalidToken(Equals)),
            ("<element//", ParserError::InvalidToken(Slash)),
            ("<element/\"", ParserError::InvalidToken(QuotationMark)),
            ("<element/ ", ParserError::InvalidToken(Whitespace(' '))),
            ("<element/test", ParserError::InvalidToken(Word(String::from("test")))),
            ("<element attribute=\"value\"", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element attribute=\"value\"=", ParserError::InvalidToken(Equals)),
            ("<element attribute=\"value\"/", ParserError::FileCutShortAbruptlyDuringTag),
            ("<??><element attribute=\"value\"\"", ParserError::InvalidToken(QuotationMark)),
            ("<element attribute=\"value\"<", ParserError::InvalidToken(StartTag)),
            ("<element attribute=\"value\"?", ParserError::InvalidToken(QuestionMark)),
        ];
        for test in data {
            check_xml_gives_correct_values(test.0, Vec::new(), Some(test.1));
        }
    }

    #[test]
    fn test_errors_in_attributes() {
        #[rustfmt::skip]
        let data = [
            ("<element ", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element <", ParserError::InvalidToken(StartTag)),
            ("<element ?", ParserError::InvalidToken(QuestionMark)),
            ("<element =", ParserError::InvalidToken(Equals)),
            ("<element \"", ParserError::InvalidToken(QuotationMark)),
            ("<element name", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element name<", ParserError::InvalidToken(StartTag)),
            ("<element name>", ParserError::InvalidToken(EndTag)),
            ("<element name?", ParserError::InvalidToken(QuestionMark)),
            ("<element name/", ParserError::InvalidToken(Slash)),
            ("<element name\"", ParserError::InvalidToken(QuotationMark)),
            ("<element name ", ParserError::InvalidToken(Whitespace(' '))),
            ("<element name=", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element name=<", ParserError::InvalidToken(StartTag)),
            ("<element name=>", ParserError::InvalidToken(EndTag)),
            ("<element name=?", ParserError::InvalidToken(QuestionMark)),
            ("<element name=/", ParserError::InvalidToken(Slash)),
            ("<element name==", ParserError::InvalidToken(Equals)),
            ("<element name= ", ParserError::InvalidToken(Whitespace(' '))),
            ("<element name=\"", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element name=\"text", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element name=\">?/=Value <", ParserError::InvalidToken(StartTag)),
            ("<element name=\">?/=Value \" name2=\"value2\"<", ParserError::InvalidToken(StartTag)),
        ];
        for test in data {
            check_xml_gives_correct_values(test.0, Vec::new(), Some(test.1));
        }
    }

    #[test]
    fn test_elements_closed_out_of_order() {
        check_xml_gives_correct_values(
            "<root><inner></root>",
            vec![
                Item::Element {
                    name: String::from("root"),
                    attributes: Vec::new(),
                },
                Item::Element {
                    name: String::from("inner"),
                    attributes: Vec::new(),
                },
            ],
            Some(ParserError::ElementsClosedOutOfOrder {
                correct: String::from("inner"),
                found: String::from("root"),
            }),
        );
    }

    #[test]
    fn test_element_closure_after_root_element_closed() {
        check_xml_gives_correct_values(
            "<root/></inner>",
            vec![
                Item::Element {
                    name: String::from("root"),
                    attributes: Vec::new(),
                },
                Item::EndCurrentElement,
            ],
            Some(ParserError::ElementClosedAfterRootElementClosed(
                String::from("inner"),
            )),
        );
    }

    #[test]
    fn test_text_outside_element() {
        check_xml_gives_correct_values(
            "<??>text block <element/>",
            Vec::new(),
            Some(ParserError::InvalidToken(Word(String::from("text")))),
        );
        check_xml_gives_correct_values(
            "<element/>\ttext block",
            vec![
                Item::Element {
                    name: String::from("element"),
                    attributes: Vec::new(),
                },
                Item::EndCurrentElement,
            ],
            Some(ParserError::InvalidToken(Word(String::from("text")))),
        );
    }

    #[test]
    fn test_eof_in_test() {
        check_xml_gives_correct_values(
            "<element>text",
            vec![Item::Element {
                name: String::from("element"),
                attributes: Vec::new(),
            }],
            Some(ParserError::ElementNotClosed),
        );
        let mut xml = ParsedXml::try_from(TokenisedXml::new(
            TestReader::new("<element>text\n").with_fail_on_end(true),
        ))
        .unwrap();
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("element"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(
            xml.next().unwrap().unwrap_err().to_string(),
            PARSER_IO_ERROR_TEST_STRING,
        );
        assert!(xml.next().is_none());
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_element_not_closed_at_end_of_file() {
        check_xml_gives_correct_values(
            "<element>",
            vec![Item::Element {
                name: String::from("element"),
                attributes: Vec::new(),
            }],
            Some(ParserError::ElementNotClosed),
        );
    }

    #[test]
    fn test_skip_current_element() {
        let mut xml = ParsedXml::try_from(TokenisedXml::new(TestReader::new(
            "<root><middle1 name=\"value\"><inner1> text </inner1>\
            <inner2/></middle1><middle2/></root>",
        )))
        .unwrap();
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("middle1"),
                attributes: vec![(String::from("name"), String::from("value"))],
            },
        );
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("middle2"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert_eq!(xml.next().unwrap().unwrap(), Item::EndCurrentElement);
        assert_eq!(xml.next().unwrap().unwrap(), Item::EndFile);
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_skipping_root_element() {
        let mut xml = ParsedXml::try_from(TokenisedXml::new(TestReader::new(
            "<root><middle1 name=\"value\"><inner1> text </inner1>\
            <inner2/></middle1><middle2/></root>",
        )))
        .unwrap();
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert_eq!(xml.next().unwrap().unwrap(), Item::EndFile);
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_skipping_outside_root_element() {
        let mut xml = ParsedXml::try_from(TokenisedXml::new(TestReader::new("<root/>"))).unwrap();
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert!(xml.next().is_none());
        let mut xml = ParsedXml::try_from(TokenisedXml::new(TestReader::new("<root/>"))).unwrap();
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(xml.next().unwrap().unwrap(), Item::EndCurrentElement);
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_errors_in_skipping_current_element() {
        let mut xml =
            ParsedXml::try_from(TokenisedXml::new(TestReader::new("<root><inner>"))).unwrap();
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(
            xml.skip_current_element().unwrap_err().to_string(),
            ParserError::ElementNotClosed.to_string(),
        );
        assert!(xml.next().is_none());
    }

    fn check_xml_gives_correct_values(txt: &str, good: Vec<Item>, err: Option<ParserError>) {
        let mut xml = ParsedXml::try_from(TokenisedXml::new(TestReader::new(txt))).unwrap();
        for item in good {
            assert_eq!(xml.next().unwrap().unwrap(), item);
        }
        if let Some(val) = err {
            assert_eq!(
                xml.next().unwrap().unwrap_err().to_string(),
                val.to_string()
            );
        } else {
            assert_eq!(xml.next().unwrap().unwrap(), Item::EndFile)
        }
        assert!(xml.next().is_none());
        assert!(xml.next().is_none());
    }
}

use super::ParserError;
use super::tokeniser::{
    self,
    Token::{self, *},
    TokenisedXml, tokenise_xml_file,
};
use State::*;
use std::fs::File;
use std::io::Read;
use std::iter::{FusedIterator, Iterator};
use std::path::Path;

pub(super) fn parse_xml_file(file_path: &Path) -> Result<ParsedXml<File>, ParserError> {
    Ok(ParsedXml::new(tokenise_xml_file(file_path)?))
}

#[derive(Debug)]
pub(super) struct ParsedXml<T: Read> {
    pub tokens: TokenisedXml<T>,
    names: Vec<String>,
    state: State,
    is_done: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Item {
    Element {
        name: String,
        attributes: Vec<(String, String)>,
    },
    Text(String),
    EndCurrentElement,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    Start {
        found_declaration: bool,
        found_start_tag: bool,
    },
    Declaration {
        found_ending_question_mark: bool,
    },
    StartTagFoundAfterRoot,
    Element {
        name: String,
        attributes: Vec<(String, String)>,
        current_attribute_name: Option<String>,
        equals_found: bool,
        quotation_mark_found: bool,
        current_attribute_text: String,
    },
    Neutral,
    TextBlock(String),
    SelfClosingElement,
    ClosingElement {
        name_checked: bool,
    },
}

impl<'a, T: Read> ParsedXml<T> {
    fn new(tokens: TokenisedXml<T>) -> Self {
        ParsedXml {
            tokens,
            names: Vec::new(),
            state: Start {
                found_declaration: false,
                found_start_tag: false,
            },
            is_done: false,
        }
    }
}

impl<T: Read> Iterator for ParsedXml<T> {
    type Item = Result<Item, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.is_done {
                return None;
            }
            match self.tokens.next() {
                Some(Ok(t)) => match self.process_token(t) {
                    Some(Ok(i)) => return Some(Ok(i)),
                    Some(Err(e)) => {
                        self.is_done = true;
                        return Some(Err(e));
                    }
                    None => (),
                },
                Some(Err(e)) => {
                    self.is_done = true;
                    return Some(Err(e));
                }
                None => {
                    self.is_done = true;
                    return match self.handle_no_more_tokens() {
                        Ok(()) => None,
                        Err(e) => Some(Err(e)),
                    };
                }
            }
        }
    }
}

impl<T: Read> FusedIterator for ParsedXml<T> {}

impl<T: Read> ParsedXml<T> {
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

    fn process_token(&mut self, token: Token) -> Option<Result<Item, ParserError>> {
        let (new_state, result) = match self.state.clone() {
            Start {
                found_declaration,
                found_start_tag: false,
            } => match token {
                Whitespace(_) => (None, None),
                StartTag => (
                    Some(Start {
                        found_declaration,
                        found_start_tag: true,
                    }),
                    None,
                ),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },
            Start {
                found_declaration,
                found_start_tag: true,
            } => match token {
                QuestionMark if !found_declaration => (
                    Some(Declaration {
                        found_ending_question_mark: false,
                    }),
                    None,
                ),
                Word(name) => (
                    Some(Element {
                        name,
                        attributes: Vec::new(),
                        current_attribute_name: None,
                        equals_found: false,
                        quotation_mark_found: false,
                        current_attribute_text: String::new(),
                    }),
                    None,
                ),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },

            Declaration {
                found_ending_question_mark: false,
            } => {
                if token == QuestionMark {
                    (
                        Some(Declaration {
                            found_ending_question_mark: true,
                        }),
                        None,
                    )
                } else {
                    (None, None)
                }
            }
            Declaration {
                found_ending_question_mark: true,
            } => match token {
                EndTag => (
                    Some(Start {
                        found_declaration: true,
                        found_start_tag: false,
                    }),
                    None,
                ),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },

            StartTagFoundAfterRoot => match token {
                Word(name) => {
                    if !self.names.is_empty() {
                        (
                            Some(Element {
                                name,
                                attributes: Vec::new(),
                                current_attribute_name: None,
                                equals_found: false,
                                quotation_mark_found: false,
                                current_attribute_text: String::new(),
                            }),
                            None,
                        )
                    } else {
                        (None, Some(Err(ParserError::MultipleRootElements)))
                    }
                }
                Slash => (
                    Some(ClosingElement {
                        name_checked: false,
                    }),
                    None,
                ),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },

            Element {
                name,
                attributes,
                current_attribute_name: None,
                current_attribute_text,
                ..
            } => match token {
                Whitespace(_) => (None, None),
                EndTag => {
                    self.names.push(name.clone());
                    (Some(Neutral), Some(Ok(Item::Element { name, attributes })))
                }
                Word(attribute_name) => (
                    Some(Element {
                        name,
                        attributes,
                        current_attribute_name: Some(attribute_name),
                        equals_found: false,
                        quotation_mark_found: false,
                        current_attribute_text,
                    }),
                    None,
                ),
                Slash => (
                    Some(SelfClosingElement),
                    Some(Ok(Item::Element { name, attributes })),
                ),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },
            Element {
                name,
                attributes,
                current_attribute_name: Some(attribute_name),
                equals_found: false,
                current_attribute_text,
                ..
            } => match token {
                Equals => (
                    Some(Element {
                        name,
                        attributes,
                        current_attribute_name: Some(attribute_name),
                        equals_found: true,
                        quotation_mark_found: false,
                        current_attribute_text,
                    }),
                    None,
                ),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },
            Element {
                name,
                attributes,
                current_attribute_name: Some(attribute_name),
                equals_found: true,
                quotation_mark_found: false,
                current_attribute_text,
                ..
            } => match token {
                QuotationMark => (
                    Some(Element {
                        name,
                        attributes,
                        current_attribute_name: Some(attribute_name),
                        equals_found: true,
                        quotation_mark_found: true,
                        current_attribute_text,
                    }),
                    None,
                ),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },
            Element {
                name,
                mut attributes,
                current_attribute_name: Some(attribute_name),
                equals_found: true,
                quotation_mark_found: true,
                current_attribute_text,
            } => match token {
                QuotationMark => {
                    attributes.push((attribute_name, current_attribute_text));
                    (
                        Some(Element {
                            name,
                            attributes,
                            current_attribute_name: None,
                            equals_found: false,
                            quotation_mark_found: false,
                            current_attribute_text: String::new(),
                        }),
                        None,
                    )
                }
                StartTag => (None, Some(Err(ParserError::InvalidToken(StartTag)))),
                t => (
                    Some(Element {
                        name,
                        attributes,
                        current_attribute_name: Some(attribute_name),
                        equals_found: true,
                        quotation_mark_found: true,
                        current_attribute_text: t.to_text().add_to_string(current_attribute_text),
                    }),
                    None,
                ),
            },

            Neutral => match token {
                StartTag => (Some(StartTagFoundAfterRoot), None),
                t => {
                    let text = match t.to_text() {
                        tokeniser::Text::String(text) => text,
                        tokeniser::Text::Char(c) => String::from(c),
                    };
                    (Some(TextBlock(text)), None)
                }
            },

            TextBlock(text) => match token {
                StartTag => (Some(StartTagFoundAfterRoot), Some(Ok(Item::Text(text)))),
                t => (Some(TextBlock(t.to_text().add_to_string(text))), None),
            },

            SelfClosingElement => match token {
                EndTag => (Some(Neutral), Some(Ok(Item::EndCurrentElement))),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },

            ClosingElement {
                name_checked: false,
            } => match token {
                Word(name) => match self.names.pop() {
                    Some(expected) if expected == name => {
                        (Some(ClosingElement { name_checked: true }), None)
                    }
                    Some(expected) => (
                        None,
                        Some(Err(ParserError::ElementsClosedOutOfOrder {
                            correct: expected,
                            found: name,
                        })),
                    ),
                    None => (
                        None,
                        Some(Err(ParserError::ElementClosedAfterRootElementClosed(name))),
                    ),
                },
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },
            ClosingElement { name_checked: true } => match token {
                EndTag if self.names.is_empty() => {
                    (Some(Neutral), Some(Ok(Item::EndCurrentElement)))
                }
                EndTag => (Some(Neutral), Some(Ok(Item::EndCurrentElement))),
                Whitespace(_) => (None, None),
                t => (None, Some(Err(ParserError::InvalidToken(t)))),
            },
        };

        if let Some(new_state) = new_state {
            self.state = new_state;
        }
        result
    }

    fn handle_no_more_tokens(&mut self) -> Result<(), ParserError> {
        match self.state {
            Start {
                found_start_tag: false,
                ..
            } => Err(ParserError::NoRootElement),
            Start {
                found_start_tag: true,
                ..
            } => Err(ParserError::FileCutShortAbruptlyDuringTag),
            Declaration { .. } => Err(ParserError::FileCutShortAbruptlyDuringXMLDeclaration),
            StartTagFoundAfterRoot => Err(ParserError::FileCutShortAbruptlyDuringTag),
            Element { .. } => Err(ParserError::FileCutShortAbruptlyDuringTag),
            Neutral if self.names.is_empty() => Ok(()),
            Neutral => Err(ParserError::ElementNotClosed),
            TextBlock(_) if self.names.is_empty() => Ok(()),
            TextBlock(_) => Err(ParserError::ElementNotClosed),
            SelfClosingElement => Err(ParserError::FileCutShortAbruptlyDuringTag),
            ClosingElement { .. } => Err(ParserError::FileCutShortAbruptlyDuringTag),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        VULKAN_XML_PATH,
        tests::{PARSER_IO_ERROR_TEST_STRING, TestReader},
    };
    use super::*;

    #[test]
    fn can_parse_text() {
        let mut xml = parse_xml_file(Path::new(VULKAN_XML_PATH)).unwrap();
        loop {
            let val = xml.next();
            if val.is_none() {
                break;
            }
            let _ = val.unwrap().unwrap();
        }
        assert!(xml.next().is_none())
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
            ("<??", ParserError::FileCutShortAbruptlyDuringXMLDeclaration),
            ("<??<", ParserError::InvalidToken(StartTag)),
            ("<??\"", ParserError::InvalidToken(QuotationMark)),
            ("<??=", ParserError::InvalidToken(Equals)),
            ("<???", ParserError::InvalidToken(QuestionMark)),
        ];
        for test in data {
            assert_eq!(
                ParsedXml::new(TokenisedXml::new(TestReader::new(test.0)))
                    .next()
                    .unwrap()
                    .unwrap_err()
                    .to_string(),
                test.1.to_string()
            );
            assert_eq!(
                ParsedXml::new(TokenisedXml::new(
                    TestReader::new(test.0).with_fail_on_end(true)
                ))
                .next()
                .unwrap()
                .unwrap_err()
                .to_string(),
                PARSER_IO_ERROR_TEST_STRING,
            );
        }
    }

    #[test]
    fn test_immediate_error() {
        assert_eq!(
            ParsedXml::new(TokenisedXml::new(TestReader::new("")))
                .next()
                .unwrap()
                .unwrap_err()
                .to_string(),
            ParserError::NoRootElement.to_string(),
        );
        assert_eq!(
            ParsedXml::new(TokenisedXml::new(
                TestReader::new("<?").with_fail_on_end(true)
            ))
            .next()
            .unwrap()
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
            "<outer><middle><inner/></middle></outer>",
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
        let data1 = [
            ("<??>\n\tElement", ParserError::InvalidToken(Word(String::from("Element")))),
            ("<??>=", ParserError::InvalidToken(Equals)),
            ("<??>>", ParserError::InvalidToken(EndTag)),
            ("<??>/", ParserError::InvalidToken(Slash)),
            ("<??>\"", ParserError::InvalidToken(QuotationMark)),
            ("<??>?", ParserError::InvalidToken(QuestionMark)),
            ("<", ParserError::FileCutShortAbruptlyDuringTag),
            ("<=", ParserError::InvalidToken(Equals)),
            ("</=", ParserError::InvalidToken(Slash)),
            ("<??><>", ParserError::InvalidToken(EndTag)),
            ("</", ParserError::InvalidToken(Slash)),
            ("<\"", ParserError::InvalidToken(QuotationMark)),
            ("< ", ParserError::InvalidToken(Whitespace(' '))),
            ("<??><<", ParserError::InvalidToken(StartTag)),
            ("<??><?", ParserError::InvalidToken(QuestionMark)),
            ("<element", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element=", ParserError::InvalidToken(Equals)),
            ("<element\"", ParserError::InvalidToken(QuotationMark)),
            ("<??><element<", ParserError::InvalidToken(StartTag)),
            ("<element?", ParserError::InvalidToken(QuestionMark)),
            ("<element attribute=\"value\"", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element attribute=\"value\"=", ParserError::InvalidToken(Equals)),
            ("<??><element attribute=\"value\"\"", ParserError::InvalidToken(QuotationMark)),
            ("<element attribute=\"value\"<", ParserError::InvalidToken(StartTag)),
            ("<element attribute=\"value\"?", ParserError::InvalidToken(QuestionMark)),
        ];
        for test in data1 {
            check_xml_gives_correct_values(test.0, Vec::new(), Some(test.1));
        }
        let data2 = [
            ("<element></?element ?", ParserError::InvalidToken(QuestionMark)),
            ("<element/<", ParserError::InvalidToken(StartTag)),
            ("<element/=", ParserError::InvalidToken(Equals)),
            ("<element//", ParserError::InvalidToken(Slash)),
            ("<element/\"", ParserError::InvalidToken(QuotationMark)),
            ("<element/ ", ParserError::InvalidToken(Whitespace(' '))),
            ("<element/test", ParserError::InvalidToken(Word(String::from("test")))),
            ("<element/ attribute=\"value\"", ParserError::InvalidToken(Whitespace(' '))),
            ("<element><?", ParserError::InvalidToken(QuestionMark)),
            ("<element></?", ParserError::InvalidToken(QuestionMark)),
            ("<element></element ?", ParserError::InvalidToken(QuestionMark)),
            ("<element><", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element></", ParserError::FileCutShortAbruptlyDuringTag),
        ];
        for test in data2 {
            check_xml_gives_correct_values(
                test.0,
                vec![Item::Element {
                    name: String::from("element"),
                    attributes: Vec::new(),
                }],
                Some(test.1),
            );
        }
        let data3 = [
            ("<element attribute=\"value\"/", ParserError::FileCutShortAbruptlyDuringTag),
            ("<element attribute=\"value\"/=", ParserError::InvalidToken(Equals)),
            ("<??><element attribute=\"value\"/\"", ParserError::InvalidToken(QuotationMark)),
            ("<element attribute=\"value\"/<", ParserError::InvalidToken(StartTag)),
            ("<element attribute=\"value\"/?", ParserError::InvalidToken(QuestionMark)),
        ];
        for test in data3 {
            check_xml_gives_correct_values(
                test.0,
                vec![Item::Element {
                    name: String::from("element"),
                    attributes: vec![(String::from("attribute"), String::from("value"))],
                }],
                Some(test.1),
            );
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
            None,
        );
    }

    #[test]
    fn test_eof_in_text() {
        check_xml_gives_correct_values(
            "<element>text",
            vec![Item::Element {
                name: String::from("element"),
                attributes: Vec::new(),
            }],
            Some(ParserError::ElementNotClosed),
        );
        let mut xml = ParsedXml::new(TokenisedXml::new(
            TestReader::new("<element>text\n").with_fail_on_end(true),
        ));
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
        let mut xml = ParsedXml::new(TokenisedXml::new(TestReader::new(
            "<root><middle1 name=\"value\"><inner1> text </inner1>\
            <inner2/></middle1><middle2/></root>",
        )));
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
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_skipping_root_element() {
        let mut xml = ParsedXml::new(TokenisedXml::new(TestReader::new(
            "<root><middle1 name=\"value\"><inner1> text </inner1>\
            <inner2/></middle1><middle2/></root>",
        )));
        assert_eq!(
            xml.next().unwrap().unwrap(),
            Item::Element {
                name: String::from("root"),
                attributes: Vec::new(),
            },
        );
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert!(xml.next().is_none());
    }

    #[test]
    fn test_skipping_outside_root_element() {
        let mut xml = ParsedXml::new(TokenisedXml::new(TestReader::new("<root/>")));
        assert_eq!(xml.skip_current_element().unwrap(), ());
        assert!(xml.next().is_none());
        let mut xml = ParsedXml::new(TokenisedXml::new(TestReader::new("<root/>")));
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
        let mut xml = ParsedXml::new(TokenisedXml::new(TestReader::new("<root><inner>")));
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
        let mut xml = ParsedXml::new(TokenisedXml::new(TestReader::new(txt)));
        for item in good {
            assert_eq!(xml.next().unwrap().unwrap(), item);
        }
        if let Some(val) = err {
            assert_eq!(
                xml.next().unwrap().unwrap_err().to_string(),
                val.to_string()
            );
        }
        assert!(xml.next().is_none());
        assert!(xml.next().is_none());
    }
}

//! Here we are defining custom code to turn our tokens into parsed syntax.
//! We could easily do this using an off-the-shelf crate, such as `syn`.
//! However, this is a good learning experience, and I am trying to keep the number of crates
//! required to an absolute minimum (for fun and learning).
//! This will only focus on what syntax I am actually using in my macros, and will be expanded as
//! that grows.
//! One benefit of doing things this way is that I can also directly use any modifications to the
//! AST I made in the token tree.

use proc_macro::{token_stream, TokenStream, TokenTree, Group, Ident, Delimiter, Literal};
use std::iter::Peekable;

pub struct Items {
    tokens: Peekable<token_stream::IntoIter>,
}

impl Items {
    pub fn new(tokens: TokenStream) -> Self {
        Items {
            tokens: tokens.into_iter().peekable(),
        }
    }
}

impl Iterator for Items {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let attributes = self.get_attributes();
        self.tokenise_item(attributes)
    }
}

pub enum Item {
    Enum(Enum),
    XmlEnum(XmlEnum),
    Struct(Struct),
    DataStruct(DataStruct),
    ImplBlock(ImplBlock),
}

pub struct Enum {
    attributes: Vec<Group>,
    name: Ident,
    elements: Vec<EnumElement>,
}

pub struct EnumElement {
    attributes: Vec<Group>,
    name: Ident,
    element_type: Option<TokenStream>
}

pub struct XmlEnum {
    attributes: Vec<Group>,
    name: Ident,
    elements: Vec<XmlEnumElement>,
}

pub struct XmlEnumElement {
    attributes: Vec<Group>,
    name: Ident,
    xml_name: Option<Literal>,
}

pub struct Struct {
    attributes: Vec<Group>,
    name: Ident,
    elements: Vec<StructElement>,
}

pub struct StructElement {
    attributes: Vec<Group>,
    visibility: String, //TODO check if String is how I should be storing visibilities.
    name: Ident,
    element_type: String,
}

pub struct DataStruct {
    name: Ident,
    enum_name: String,
    identifier: String, //TODO assess if this is needed.
    elements: Vec<DataStructElement>,
}

pub struct DataStructElement {
    attributes: Vec<Group>,
    name: Ident,
    element_type: String, //TODO check if String is how I should be storing types.
}

pub struct ImplBlock {
    tokens: TokenStream,
}

type PeekableTokenStream = Peekable<token_stream::IntoIter>;

impl Items {
    fn get_attributes(&mut self) -> Vec<Group> {
        get_attributes_from_tokens(&mut self.tokens)
    }

    fn tokenise_item(&mut self, attributes: Vec<Group>) -> Option<Item> {
        match self.tokens.next()? {
            TokenTree::Ident(ident) => {
                match ident.to_string().as_str() {
                    "enum" => Some(Item::Enum(self.tokenise_enum(attributes))),
                    "xml" => Some(Item::XmlEnum(self.tokenise_xml_enum(attributes))),
                    "struct" => Some(Item::Struct(self.tokenise_struct(attributes))),
                    "data" => Some(Item::DataStruct(self.tokenise_data_struct())),
                    "impl" => Some(Item::ImplBlock(self.tokenise_impl_block(attributes))),
                    other => panic!("Unexpected root level identifier found: {}", other),
                }
            }
            other => panic!("Unexpected token tree found: {}", other.to_string())
        }
    }

    fn tokenise_enum(&mut self, attributes: Vec<Group>) -> Enum {
        let Some(TokenTree::Ident(name)) = self.tokens.next() else {
            panic!("Expected enum name");
        };
        let mut inner_tokens = expect_group(&mut self.tokens, Delimiter::Brace);
        let mut elements = Vec::new();
        while inner_tokens.peek().is_some() {
            let element_attributes = get_attributes_from_tokens(&mut inner_tokens);
            let element_name = get_ident(&mut self.tokens);
            let element_type = match inner_tokens.next() {
                None => None,
                Some(TokenTree::Punct(p)) if p == ',' => None,
                Some(TokenTree::Group(group)) => {
                    assert_eq!(
                        group.delimiter(),
                        Delimiter::Parenthesis,
                        "Invalid end of enum element, found \"{}\"", group
                    );
                    match inner_tokens.next() {
                        None => {},
                        Some(TokenTree::Punct(p)) if p == ',' => {},
                        Some(other) => panic!("Invalid end of enum element, found \"{}\"", other),
                    }
                    Some(group.stream())
                },
                Some(other) => panic!("Invalid end of enum element, found \"{}\"", other),
            };
            elements.push(EnumElement {
                attributes: element_attributes,
                name: element_name,
                element_type,
            });
        }
        Enum {
            attributes,
            name,
            elements,
        }
    }

    fn tokenise_xml_enum(&mut self, attributes: Vec<Group>) -> XmlEnum {
        expect_ident(&mut self.tokens, "enum");
        let Some(TokenTree::Ident(name)) = self.tokens.next() else {
            panic!("Expected xml enum name");
        };
        let mut inner_tokens = expect_group(&mut self.tokens, Delimiter::Brace);
        let mut elements = Vec::new();
        while inner_tokens.peek().is_some() {
            let element_attributes = get_attributes_from_tokens(&mut inner_tokens);
            let element_name = get_ident(&mut self.tokens);
            let element_xml_name = match inner_tokens.next() {
                None => None,
                Some(TokenTree::Punct(p)) if p == ',' => None,
                Some(TokenTree::Punct(p)) if p == ':' => {
                    let xml_name = get_literal(&mut inner_tokens);
                    match inner_tokens.next() {
                        None => {},
                        Some(TokenTree::Punct(p)) if p == ',' => {},
                        Some(other) => panic!(
                            "Invalid end of xml enum element, found \"{}\"",
                            other,
                        ),
                    }
                    Some(xml_name)
                },
                Some(other) => panic!("Invalid end of xml enum element, found \"{}\"", other),
            };
            elements.push(XmlEnumElement {
                attributes: element_attributes,
                name: element_name,
                xml_name: element_xml_name,
            });
        }
        XmlEnum {
            attributes,
            name,
            elements,
        }
    }
}

fn get_attributes_from_tokens(tokens: &mut PeekableTokenStream) -> Vec<Group> {
    let mut attributes = Vec::new();
    loop {
        match tokens.peek() {
            Some(TokenTree::Punct(punct)) => {
                match punct.as_char() {
                    '#' => {
                        tokens.next();
                        let Some(TokenTree::Group(group)) = tokens.next() else {
                            panic!("# character found without a following attribute");
                        };
                        attributes.push(group);
                    },
                    _ => return attributes,
                }
            },
            _ => return attributes,
        }
    }
}

fn get_ident(tokens: &mut PeekableTokenStream) -> Ident {
    let Some(result) = tokens.next() else {
        panic!("Expected ident but found end of stream");
    };
    let TokenTree::Ident(result) = result else {
        panic!("Expected ident but found \"{}\"", result.to_string());
    };
    result
}

fn get_literal(tokens: &mut PeekableTokenStream) -> Literal {
    let Some(result) = tokens.next() else {
        panic!("Expected literal but found end of stream");
    };
    let TokenTree::Literal(result) = result else {
        panic!("Expected literal but found \"{}\"", result.to_string());
    };
    result
}

fn expect_group(tokens: &mut PeekableTokenStream, delimiter: Delimiter) -> PeekableTokenStream {
    let Some(result) = tokens.next() else {
        panic!("Expected group but found end of stream");
    };
    let TokenTree::Group(result) = result else {
        panic!("Expected group but found \"{}\"", result.to_string());
    };
    assert_eq!(
        result.delimiter(),
        delimiter,
        "Expected delimiter of {:?} but found {:?}",
        delimiter,
        result.delimiter(),
    );
    result.stream().into_iter().peekable()
}

fn expect_ident(tokens: &mut PeekableTokenStream, value: &str) {
    let Some(result) = tokens.next() else {
        panic!("Expected ident but found end of stream");
    };
    let TokenTree::Ident(result) = result else {
        panic!("Expected ident but found \"{}\"", result.to_string());
    };
    assert_eq!(
        result.to_string(),
        value,
        "Expected ident \"{}\", but got \"{}\"",
        value,
        result.to_string(),
    );
}

fn expect_punct(tokens: &mut PeekableTokenStream, punct: char) {
    let Some(result) = tokens.next() else {
        panic!("Expected '{}' but found end of stream", punct);
    };
    let TokenTree::Punct(result) = result else {
        panic!("Expected '{}' but found \"{}\"", punct, result.to_string());
    };
    assert_eq!(result, punct, "Expected '{}' but found '{}'", punct, result);
}
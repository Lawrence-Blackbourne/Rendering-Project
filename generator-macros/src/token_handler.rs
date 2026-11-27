//! Here we are defining custom code to turn our tokens into parsed syntax.
//! We could easily do this using an off-the-shelf crate, such as `syn`.
//! However, this is a good learning experience, and I am trying to keep the number of crates
//! required to an absolute minimum (for fun and learning).
//! This will only focus on what syntax I am actually using in my macros, and will be expanded as
//! that grows.
//! One benefit of doing things this way is that I can also directly use any modifications to the
//! AST I made in the token tree.

use crate::tokens::{self, TokenStream, TokenTree, Group, Ident, Delimiter, Literal};
use std::iter::Peekable;

pub struct Items {
    tokens: Peekable<tokens::IntoIter>,
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
        let attributes = get_attributes(&mut self.tokens);
        self.tokenise_item(attributes)
    }
}

#[derive(Debug)]
pub enum Item {
    Enum(Enum),
    XmlEnum(XmlEnum),
    Struct(Struct),
    DataStruct(DataStruct),
    ImplBlock(ImplBlock),
}

impl Item {
    pub fn attributes(&self) -> Option<&Vec<Group>> {
        match self {
            Item::Enum(item) => Some(item.attributes()),
            Item::XmlEnum(item) => Some(item.attributes()),
            Item::Struct(item) => Some(item.attributes()),
            Item::DataStruct(_) => None,
            Item::ImplBlock(item) => Some(item.attributes()),
        }
    }
}

#[derive(Debug)]
pub struct Enum {
    attributes: Vec<Group>,
    name: Ident,
    elements: Vec<EnumElement>,
}

impl Enum {
    pub fn attributes(&self) -> &Vec<Group> {
        &self.attributes
    }
}

#[derive(Debug)]
pub struct EnumElement {
    attributes: Vec<Group>,
    name: Ident,
    element_type: Option<TokenStream>
}

#[derive(Debug)]
pub struct XmlEnum {
    attributes: Vec<Group>,
    name: Ident,
    elements: Vec<XmlEnumElement>,
}

impl XmlEnum {
    pub fn attributes(&self) -> &Vec<Group> {
        &self.attributes
    }
}


#[derive(Debug)]
pub struct XmlEnumElement {
    attributes: Vec<Group>,
    name: Ident,
    xml_name: Option<Literal>,
}

#[derive(Debug)]
pub struct Struct {
    attributes: Vec<Group>,
    name: Ident,
    elements: Vec<StructElement>,
}

impl Struct {
    pub fn attributes(&self) -> &Vec<Group> {
        &self.attributes
    }
}


#[derive(Debug)]
pub struct StructElement {
    attributes: Vec<Group>,
    visibility: TokenStream,
    name: Ident,
    element_type: TokenStream,
}

#[derive(Debug)]
pub struct DataStruct {
    name: Ident,
    enum_name: Ident,
    identifier: Ident, //TODO assess if this is needed.
    elements: Vec<DataStructElement>,
}

#[derive(Debug)]
pub struct DataStructElement {
    attributes: Vec<Group>,
    name: Ident,
    element_type: TokenStream, //TODO check if String is how I should be storing types.
}

impl TryFrom<StructElement> for DataStructElement {
    type Error = ();

    fn try_from(value: StructElement) -> Result<Self, Self::Error> {
        if value.visibility.is_empty() {
            Err(())
        } else {
            Ok(DataStructElement {
                attributes: value.attributes,
                name: value.name,
                element_type: value.element_type,
            })
        }
    }
}

#[derive(Debug)]
pub struct ImplBlock {
    attributes: Vec<Group>,
    type_implemented_on: Ident,
    tokens: TokenStream,
}

impl ImplBlock {
    pub fn attributes(&self) -> &Vec<Group> {
        &self.attributes
    }
}


type PeekableTokenStream = Peekable<tokens::IntoIter>;

impl Items {

    pub fn tokenise_item(&mut self, attributes: Vec<Group>) -> Option<Item> {
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
        let name = get_ident(&mut self.tokens);
        let mut inner_tokens = expect_group(&mut self.tokens, Delimiter::Brace);
        let mut elements = Vec::new();
        while inner_tokens.peek().is_some() {
            let element_attributes = get_attributes(&mut inner_tokens);
            let element_name = get_ident(&mut inner_tokens);
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
        let name = get_ident(&mut self.tokens);
        let mut inner_tokens = expect_group(&mut self.tokens, Delimiter::Brace);
        let mut elements = Vec::new();
        while inner_tokens.peek().is_some() {
            let element_attributes = get_attributes(&mut inner_tokens);
            let element_name = get_ident(&mut inner_tokens);
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

    fn tokenise_struct(&mut self, attributes: Vec<Group>) -> Struct {
        let name = get_ident(&mut self.tokens);
        let mut inner_tokens = expect_group(&mut self.tokens, Delimiter::Brace);
        let mut elements = Vec::new();
        while let Some(element) = get_struct_element(&mut inner_tokens) {
            elements.push(element);
        }
        Struct {
            attributes,
            name,
            elements,
        }
    }

    fn tokenise_data_struct(&mut self) -> DataStruct {
        expect_ident(&mut self.tokens, "enum");
        let name = get_ident(&mut self.tokens);
        expect_ident(&mut self.tokens, "for");
        let enum_name = get_ident(&mut self.tokens);
        expect_ident(&mut self.tokens, "as");
        let identifier = get_ident(&mut self.tokens);
        let mut inner_tokens = expect_group(&mut self.tokens, Delimiter::Brace);
        let mut elements = Vec::new();
        while let Some(element) = get_struct_element(&mut inner_tokens) {
            elements.push(DataStructElement::try_from(element)
                .expect("Data struct elements cannot have visibility"));
        }
        DataStruct {
            name,
            enum_name,
            identifier,
            elements,
        }
    }

    fn tokenise_impl_block(&mut self, attributes: Vec<Group>) -> ImplBlock {
        let type_implemented_on = get_ident(&mut self.tokens);
        let tokens = expect_group(&mut self.tokens, Delimiter::Brace).collect();
        ImplBlock {
            attributes,
            type_implemented_on,
            tokens,
        }
    }
}

fn get_struct_element(tokens: &mut PeekableTokenStream) -> Option<StructElement> {
    if tokens.peek().is_none() {
        return None
    }
    let element_attributes = get_attributes(tokens);
    let possible_visibility = get_ident(tokens);
    let (visibility, element_name) = match possible_visibility.to_string().as_str() {
        "pub" => {
            match tokens.next() {
                Some(TokenTree::Group(group)) => (
                    vec![
                        TokenTree::Ident(possible_visibility),
                        TokenTree::Group(group),
                    ].into_iter().collect(),
                    get_ident(tokens),
                ),
                Some(TokenTree::Ident(ident)) => (
                    TokenStream::from(TokenTree::Ident(possible_visibility)), ident
                ),
                Some(other) => panic!("Unexpected token found in struct: {}", other),
                None => panic!("Unexpected end of stream"),
            }
        },
        _ => (TokenStream::new(), possible_visibility),
    };
    let element_type = expect_group(tokens, Delimiter::Parenthesis).collect();
    match tokens.next() {
        None => {},
        Some(TokenTree::Punct(p)) if p == ',' => {},
        Some(other) => panic!("Invalid end of struct element, found \"{}\"", other),
    }
    Some(StructElement {
        attributes: element_attributes,
        visibility,
        name: element_name,
        element_type,
    })
}

fn get_attributes(tokens: &mut PeekableTokenStream) -> Vec<Group> {
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

#[cfg(test)]
mod tests {
    use super::*;


}
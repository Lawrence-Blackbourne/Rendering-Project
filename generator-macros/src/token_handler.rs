//! Here we are defining custom code to turn our tokens into parsed syntax.
//! We could easily do this using an off-the-shelf crate, such as `syn`.
//! However, this is a good learning experience, and I am trying to keep the number of crates
//! required to an absolute minimum (for fun and learning).
//! This will only focus on what syntax I am actually using in my macros, and will be expanded as
//! that grows.
//! One benefit of doing things this way is that I can also directly use any modifications to the
//! AST I made in the token tree.

use proc_macro::{token_stream, TokenStream, TokenTree};
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
    attributes: Vec<String>,
    name: String,
    elements: Vec<EnumElement>,
}

pub struct EnumElement {
    attributes: Vec<String>,
    name: String,
    element_type: Option<String>
}

pub struct XmlEnum {
    attributes: Vec<String>,
    name: String,
    elements: Vec<XmlEnumElement>,
}

pub struct XmlEnumElement {
    attributes: Vec<String>,
    name: String,
    xml_name: String,
}

pub struct Struct {
    attributes: Vec<String>,
    name: String,
    elements: Vec<StructElement>,
}

pub struct StructElement {
    attributes: Vec<String>,
    visibility: String, //TODO check if String is how I should be storing visibilities.
    name: String,
    element_type: String,
}

pub struct DataStruct {
    name: String,
    enum_name: String,
    identifier: String, //TODO assess if this is needed.
    elements: Vec<DataStructElement>,
}

pub struct DataStructElement {
    attributes: Vec<String>,
    name: String,
    element_type: String, //TODO check if String is how I should be storing types.
}

pub struct ImplBlock {
    tokens: TokenStream,
}

impl Items {
    fn get_attributes(&mut self) -> Vec<String> {
        let attributes = Vec::new();
        loop {
            match self.tokens.peek() {
                Some(TokenTree::Punct(punct)) => {
                    todo!()
                },
                _ => return attributes,
            }
        }
    }

    fn tokenise_item(&mut self, attributes: Vec<String>) -> Option<Item> {
        todo!()
    }
}
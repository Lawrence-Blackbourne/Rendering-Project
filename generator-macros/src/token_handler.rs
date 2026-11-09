use proc_macro::TokenStream;

//! Here we are defining custom code to turn our tokens into parsed syntax.
//! We could easily do this using an off-the-shelf crate, such as `syn`.
//! However, this is a good learning experience, and I am trying to keep the number of crates
//! required to an absolute minimum (for fun and learning).
//! This will only focus on what syntax I am actually using in my macros, and will be expanded as
//! that grows.
//! One benefit of doing things this way is that I can also directly use any modifications to the
//! AST I made in the token tree.

pub struct Items {
    tokens: TokenStream,
    state: ParsingState,
    is_done: bool,
}

impl Items {
    pub fn new(tokens: TokenStream) -> Self {
        Items {
            tokens,
            state: Start,
            is_done: false,
        }
    }
}

pub enum Item {
    Enum(Enum),
    XmlEnum(XmlEnum),
    Struct(Struct),
    DataStruct(DataStruct),
    ImplBlock(ImplBlock),
}

enum ParsingState {

}
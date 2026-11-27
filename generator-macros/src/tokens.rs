#[cfg(not(test))]
pub use proc_macro::{TokenStream,
                     TokenTree,
                     Group,
                     Ident,
                     Literal,
                     Delimiter,
                     token_stream::IntoIter};

#[cfg(test)]
mod implementation {
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Clone)]
    pub struct TokenStream {
        items: Vec<TokenTree>,
    }

    impl TokenStream {
        pub fn new() -> TokenStream {
            TokenStream {
                items: Vec::new(),
            }
        }

        pub fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
    }

    impl IntoIterator for TokenStream {
        type Item = TokenTree;
        type IntoIter = IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            self.items.into_iter()
        }
    }

    impl Display for TokenStream {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut first = true;
            for item in &self.items {
                if !first {
                    write!(f, " ")?;
                }
                first = false;
                write!(f, "{}", item)?;
            }
            Ok(())
        }
    }

    impl From<TokenTree> for TokenStream {
        fn from(value: TokenTree) -> Self {
            TokenStream {
                items: vec![value],
            }
        }
    }

    impl FromIterator<TokenTree> for TokenStream {
        fn from_iter<T: IntoIterator<Item=TokenTree>>(iter: T) -> Self {
            TokenStream {
                items: iter.into_iter().collect(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum TokenTree {
        Group(Group),
        Ident(Ident),
        Punct(Punct),
        Literal(Literal),
    }

    impl Display for TokenTree {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TokenTree::Group(group) => group.fmt(f),
                TokenTree::Ident(ident) => ident.fmt(f),
                TokenTree::Punct(punct) => punct.fmt(f),
                TokenTree::Literal(literal) => literal.fmt(f),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Group {
        delimiter: Delimiter,
        stream: TokenStream,
    }

    impl Group {
        pub fn delimiter(&self) -> Delimiter {
            self.delimiter
        }

        pub fn stream(&self) -> TokenStream {
            self.stream.clone()
        }
    }

    impl Display for Group {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self.delimiter {
                Delimiter::Parenthesis => write!(f, "({})", self.stream.to_string()),
                Delimiter::Brace => write!(f, "{{{}}}", self.stream.to_string()),
                Delimiter::Bracket => write!(f, "[{}]", self.stream.to_string()),
                Delimiter::None => write!(f, "{}", self.stream.to_string()),

            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Ident {
        item: String,
    }

    impl Display for Ident {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.item)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Punct {
        item: char,
    }

    impl Punct {
        pub fn as_char(&self) -> char {
            self.item
        }
    }

    impl Display for Punct {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.item)
        }
    }

    impl PartialEq<char> for Punct {
        fn eq(&self, other: &char) -> bool {
            self.item == *other
        }
    }

    #[derive(Debug, Clone)]
    pub struct Literal {
        item: String,
    }

    impl Display for Literal {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.item)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Delimiter {
        Parenthesis,
        Brace,
        Bracket,
        None,
    }

    pub type IntoIter = <Vec<TokenTree> as IntoIterator>::IntoIter;
}

#[cfg(test)]
pub use implementation::*;
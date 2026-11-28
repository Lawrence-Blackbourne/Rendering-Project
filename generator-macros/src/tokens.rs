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
    use std::iter::Peekable;
    use std::str::{Chars, FromStr};

    #[derive(Debug, Clone)]
    pub struct TokenStream {
        items: Vec<TokenTree>,
    }

    impl TokenStream {
        pub fn new() -> Self {
            Self {
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
            let mut prev: Option<&TokenTree> = None;
            for item in &self.items {
                if let Some(prev) = prev {
                    match prev {
                        TokenTree::Punct(punct) if punct.spacing() == Spacing::Joint
                            && matches!(item, TokenTree::Punct(_)) => {}
                        _ => {
                            write!(f, " ")?;
                        }
                    }
                }
                prev = Some(item);
                write!(f, "{}", item)?;
            }
            Ok(())
        }
    }

    impl From<TokenTree> for TokenStream {
        fn from(value: TokenTree) -> Self {
            Self {
                items: vec![value],
            }
        }
    }

    impl FromIterator<TokenTree> for TokenStream {
        fn from_iter<T: IntoIterator<Item=TokenTree>>(iter: T) -> Self {
            Self {
                items: iter.into_iter().collect(),
            }
        }
    }

    /// This implementation does not consider many edge cases related to parsing the text.
    /// This is because it is designed for the use case of `token_handler.rs`.
    /// Any edge cases not explicitly mentioned should be assumed to not be considered.
    ///
    /// We assume all identifiers consist only of a-z and A-Z, as well as _.
    /// _ cannot be in the middle of an identifier.
    /// Any unknown characters will be presented as punctuation.
    ///
    /// Delimiter::None should never be produced
    impl FromStr for TokenStream {
        type Err = LexError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::from_chars_till_delimiter(&mut s.chars().peekable(), Delimiter::None)
        }
    }

    impl TokenStream {
        pub fn from_chars_till_delimiter(
            chars: &mut Peekable<Chars>,
            delimiter: Delimiter
        ) -> Result<Self, LexError> {
            let mut token_trees = Vec::new();
            loop {
                match chars.next() {
                    Some(c) if c.is_ascii_alphabetic() => {
                        let mut result = String::from(c);
                        while let Some(c) = chars.peek() {
                            if *c == '_' || c.is_ascii_alphabetic() {
                                result.push(chars.next().unwrap())
                            } else {
                                break
                            }
                        }
                        token_trees.push(TokenTree::Ident(Ident::new(&result, Span {})))
                    }
                    Some(c) if let Some(new_delimiter) = opening_delimiter(c) => {
                        token_trees.push(TokenTree::Group(Group::new(
                            new_delimiter,
                            Self::from_chars_till_delimiter(chars, new_delimiter)?,
                        )))
                    }
                    Some(c) if let Some(new_delimiter) = closing_delimiter(c) => {
                        if new_delimiter == delimiter {
                            return Ok(token_trees.into_iter().collect());
                        }
                        return Err(LexError::new(&format!(
                            "Found closing delimiter {:?} but {:?} expected",
                            new_delimiter,
                            delimiter,
                        )))
                    }
                    Some(c) if c.is_whitespace() => {},
                    Some(c) => {
                        if let Some(next) = chars.peek() && next.is_whitespace() {
                            token_trees.push(TokenTree::Punct(Punct::new(c, Spacing::Alone)))
                        } else {
                            token_trees.push(TokenTree::Punct(Punct::new(c, Spacing::Joint)))
                        }
                    }
                    None => {
                        if delimiter == Delimiter::None {
                            return Ok(token_trees.into_iter().collect())
                        }
                        return Err(LexError::new(&format!(
                            "Found expected closing delimiter {:?} but reached end of stream",
                            delimiter,
                        )))
                    }
                }
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
                Self::Group(group) => group.fmt(f),
                Self::Ident(ident) => ident.fmt(f),
                Self::Punct(punct) => punct.fmt(f),
                Self::Literal(literal) => literal.fmt(f),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Group {
        delimiter: Delimiter,
        stream: TokenStream,
    }

    impl Group {
        pub fn new(delimiter: Delimiter, stream: TokenStream) -> Self {
            Self {
                delimiter,
                stream,
            }
        }

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

    impl Ident {
        pub fn new(string: &str, _: Span) -> Self {
            Self {
                item: String::from(string),
            }
        }
    }

    impl Display for Ident {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.item)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Punct {
        item: char,
        spacing: Spacing,
    }

    impl Punct {
        pub fn new(ch: char, spacing: Spacing) -> Self {
            Self {
                item: ch,
                spacing,
            }
        }

        pub fn as_char(&self) -> char {
            self.item
        }

        pub fn spacing(&self) -> Spacing {
            self.spacing
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

    pub struct Span {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LexError {
        error: String
    }

    impl LexError {
        fn new(error: &str) -> Self {
            Self {
                error: String::from(error)
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Spacing {
        Joint,
        Alone,
    }

    pub type IntoIter = <Vec<TokenTree> as IntoIterator>::IntoIter;

    fn is_delimiter(c: char) -> bool {
        c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']'
    }

    fn is_closing_delimiter(c: char) -> bool {
        c == ')' || c == '}' || c == ']'
    }

    fn char_is_delimiter_closing(delimiter: Delimiter, c: char) -> bool {
        match c {
            ')' => delimiter == Delimiter::Parenthesis,
            '}' => delimiter == Delimiter::Brace,
            ']' => delimiter == Delimiter::Bracket,
            _ => false,
        }
    }

    fn opening_delimiter(c: char) -> Option<Delimiter> {
        match c {
            '(' => Some(Delimiter::Parenthesis),
            '{' => Some(Delimiter::Brace),
            '[' => Some(Delimiter::Bracket),
            _ => None,
        }
    }

    fn closing_delimiter(c: char) -> Option<Delimiter> {
        match c {
            ')' => Some(Delimiter::Parenthesis),
            '}' => Some(Delimiter::Brace),
            ']' => Some(Delimiter::Bracket),
            _ => None,
        }
    }
}

#[cfg(test)]
pub use implementation::*;
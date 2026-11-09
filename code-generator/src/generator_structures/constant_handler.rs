const INDENT_STR: &str = "    ";

pub enum Constant {
    Value(String),
    List(ConstantList),
}

pub struct ConstantList {
    name: Option<String>,
    bracket: ConstantBracket,
    items: Vec<Constant>,
}

pub enum ConstantBracket {
    Parenthesis,
    Braces,
    SquareBrackets,
}

//TODO automatically choose to make a new line or not
pub trait ToConstant {
    fn to_constant(self) -> Constant;
}

impl ToConstant for u32 {
    fn to_constant(self) -> Constant {
        Constant::Value(self.to_string())
    }
}

impl ToConstant for (u32, u32, u32) {
    fn to_constant(self) -> Constant {
        Constant::List(ConstantList {
            name: None,
            bracket: ConstantBracket::Parenthesis,
            items: vec![
                Constant::Value(self.0.to_string()),
                Constant::Value(self.1.to_string()),
                Constant::Value(self.2.to_string()),
            ],
        })
    }
}

impl<T: ToConstant> ToConstant for Option<T> {
    fn to_constant(self) -> Constant {
        match self {
            Some(value) => Constant::List(ConstantList {
                name: Some(String::from("Some")),
                bracket: ConstantBracket::Parenthesis,
                items: vec![value.to_constant()]
            }),
            None => Constant::Value(String::from("None")),
        }
    }
}

impl<T: ToConstant> ToConstant for Vec<T> {
    fn to_constant(self) -> Constant {
        Constant::List(ConstantList {
            name: Some(String::from("vec!")),
            bracket: ConstantBracket::SquareBrackets,
            items: self.into_iter()
                .map(|item| item.to_constant())
                .collect::<Vec<_>>()
        })
    }
}
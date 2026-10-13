pub trait Generator {
    fn generate_text() -> String;
}




/// This macro will generate an enum which implements `GeneratedEnum`.
/// It will also implement `TryFrom<&str>`.
/// The enum will automatically have `#[non_exhaustive]` and
/// `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`, but other attributes and doc comments can be
/// added.
/// These other attributes and doc comments will be passed along to the ATTRIBUTES section, to be
/// used on the final enum in the generated code.
/// Similarly, doc comments and attributes can be added to each element, which will go to that
/// elements stored text, to be used on the final enum in the generated code.
/// The struct generated in place will have pub(super) and the one generated later will have the
/// visibility given for its elements.
/// todo FIX COMMENTS
///
/// TODO take comments on data struct and pass it to the functions on xml enum
/// Have some kind of with data option on create_generator to allow us to pass an array of
/// `(ImageFormat, ImageFormatDetails)` when generating the final code which will then add functions
/// to get that data from the ImageFormat.
/// Move this all into one create_generator! function that can then also generate one big
/// generate_text function, which would take the above array
macro_rules! create_generator {
    // Internal helper branches.
    (@replace $old:tt, $new:tt) => {$new};

    (@format_attribute doc = $text:expr) => {concat!("///", $text)};
    (@format_attribute $other:meta) => {stringify!(#[$other])};

    (@enum_to_constant_text $name:ident, $element:ident) => {
        String::from(stringify!($name::$element))
    };
    (@enum_to_constant_text $name:ident, $element:ident, $replacement:ident, $type:ty) => {
        format!("{}({})", stringify!($name::$element), $replacement.to_constant())
    };

    (@generator_implementation $name:ident, $type:ident $(,$extra:tt)* $(,)?) => {
        impl Generator for $name {
            fn generate_text() -> String {
                format!(
                    "{}\n\
                    pub {} {} {{\n\
                    {}\n\
                    }}\n\
                    {}",
                    Self::ATTRIBUTES,
                    stringify!($type),
                    Self::NAME,
                    Self::ELEMENTS
                        .into_iter()
                        .map(|element| format!(
                            "{},", Self::generate_element_text(&element)
                        ))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    concat!($(stringify!($extra)),*),
                )
            }
        }
    };

    // The XML enum branch.
    (
        $(#[$attribute:meta])*
        xml enum $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $element:ident $(: $xml_name:literal)?
            ),*
            $(,)?
        }
        $(
            data struct $data_struct_name:ident {
                $(
                    $(#[$data_struct_element_attribute:meta])*
                    $data_name:ident: $data_type:ty
                ),*
                $(,)?
            }
        )?

    ) => {
        $(#[$attribute])*
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $(#[$element_attribute])*
                $element,
            )*
        }

        impl $name {
            const ATTRIBUTES: &'static str = concat!(
                $(create_generator!(@format_attribute $attribute), "\n",)*
                "#[non_exhaustive]\n",
                "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]",
            );

            const NAME: &'static str = stringify!($name);

            const ELEMENTS: &'static [&str] = &[
                $(stringify!($element)),*
            ];

            fn generate_element_text(element: &str) -> &'static str {
                match element {
                    $(
                        stringify!($element) => concat!(
                            $('\t', create_generator!(@format_attribute $element_attribute), "\n",)*
                            '\t', stringify!($name)
                        ),
                    )*
                    _ => panic!("Unknown element"),
                }
            }
        }

        create_generator!(@generator_implementation $name, enum);

        impl TryFrom<&str> for $name {
            type Error = String;

            fn try_from<'a>(value: &'a str) -> Result<Self, Self::Error> {
                match value {
                    $($($xml_name => Ok($name::$element),)*)*
                    other => Err(
                        String::from(concat!("invalid ", stringify!($name), " value: ")) + other
                    ),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {$(
                    $name::$element => write!(f, "{}", stringify!($element)),
                )*}
            }
        }

        impl ToConstant for $name {
            fn to_constant(self) -> String {
                match self {$(
                    $name::$element => String::from(stringify!($name::$element)),
                )*}
            }
        }

        $(
            #[derive(Debug)]
            pub struct $data_struct_name {
                $(
                    $(#[$data_struct_element_attribute])*
                    pub $data_name: $data_type,
                )*
            }

            impl $name {
                pub fn generate_getters(data: Vec<($name, $data_struct_name)>) -> String {
                    //TODO check unique data entry present for all variants

                    $(
                        let mut $data_name = format!(
                            "\tfn {}(&self) => {} {{\
                            \t\tmatch self {{",
                            stringify!($data_name),
                            stringify!($data_type),
                        );
                    )*

                    for (variant, item) in data {
                        $($data_name.push_str(
                            &format!("\t\t\t{} => {},\n", variant, item.$data_name.to_constant())
                        );)*
                    }

                    $(
                        $data_name += "\t\t}\n\t}";
                    )*

                    let mut result = String::new();
                    $(result.push_str(&format!("{}\t\t}}\n\t}}", $data_name));)*
                    result
                }
            }
        )*
    };

    // The enum branch
    (
        $(#[$attribute:meta])*
        enum $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $element:ident$(($type:ty))?
            ),*
            $(,)?
        }
        $($extra:tt)*
    ) => {
        $(#[$attribute])*
        pub enum $name {
            $(
                $(#[$element_attribute])*
                $element$(($type))*,
            )*
        }

        impl $name {
            const ATTRIBUTES: &'static str = concat!(
                $(create_generator!(@format_attribute $attribute), "\n",)*
            );

            const NAME: &'static str = stringify!($name);

            const ELEMENTS: &'static [&str] = &[
                $(stringify!($element)),*
            ];

            fn generate_element_text(element: &str) -> &'static str {
                match element {
                    $(
                        stringify!($element) => concat!(
                            $('\t', create_generator!(@format_attribute $element_attribute), "\n",)*
                            '\t', stringify!($name$(($type))*)
                        ),
                    )*
                    _ => panic!("Unknown element"),
                }
            }
        }

        create_generator!(@generator_implementation $name, enum, $($extra),*);

        impl ToConstant for $name {
            fn to_constant(self) -> String {
                match self {
                    $($name::$element$((create_generator!(@replace $type, val)))* =>
                        create_generator!(@enum_to_constant_text $name, $element $(,val , $type)*),)*
                }
            }
        }
    };

    // The struct branch.
    (
        $(#[$attribute:meta])*
        struct $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $visibility:vis $element:ident: $type:ty
            ),*
            $(,)?
        }
        $($extra:tt)*
    ) => {
        $(#[$attribute])*
        pub struct $name {
            $(
                $(#[$element_attribute])*
                pub $element: $type,
            )*
        }

        impl $name {
            const ATTRIBUTES: &'static str = concat!(
                $(create_generator!(@format_attribute $attribute), "\n",)*
            );

            const NAME: &'static str = stringify!($name);

            const ELEMENTS: &'static [&str] = &[
                $(stringify!($element)),*
            ];

            fn generate_element_text(element: &str) -> &'static str {
                match element {
                    $(
                        stringify!($element) => concat!(
                            $('\t', create_generator!(@format_attribute $element_attribute), "\n",)*
                            '\t', stringify!($visibility), ' ', stringify!($element), ": ",
                            stringify!($type),
                        ),
                    )*
                    _ => panic!("Unknown element"),
                }
            }
        }

        create_generator!(@generator_implementation $name, struct, $($extra),*);

        impl ToConstant for $name {
            fn to_constant(self) -> String {
                let mut result = format!("{} {{\n", stringify!($name));
                $(result.push_str(&format!(
                    "\t{}: {},",
                    stringify!($element),
                    self.$element.to_constant(),
                ));)*
                format!("{result}\n}}")
            }
        }
    };
}

pub(crate) use create_generator;

pub trait ToConstant {
    fn to_constant(self) -> String;
}

impl ToConstant for u32 {
    fn to_constant(self) -> String {
        self.to_string()
    }
}

impl ToConstant for (u32, u32, u32) {
    fn to_constant(self) -> String {
        format!("({}, {}, {})", self.0.to_string(), self.1.to_string(), self.2.to_string())
    }
}

impl <T: ToConstant> ToConstant for Option<T> {
    fn to_constant(self) -> String {
        match self {
            Some(value) => format!("Some({})", value.to_constant()),
            None => String::from("None"),
        }
    }
}

impl <T: ToConstant> ToConstant for Vec<T> {
    fn to_constant(self) -> String {
        format!(
            "vec![{}]",
            self.into_iter()
                .map(|item| item.to_constant())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}
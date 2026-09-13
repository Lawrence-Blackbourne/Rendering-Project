pub mod format_generator_enums;

pub trait Generator {
    fn generate_text() -> String;
}

/// This macro will generate an enum which implements `GeneratedEnum`.
/// It will also implement `TryFrom<&str>`.
/// The enum will automatically have `#[non_exhaustive]` and
/// `#[derive(Debug, Copy, Clone, PartialEq, Eq)]`, but other attributes and doc comments can be
/// added.
/// These other attributes and doc comments will be passed along to the ATTRIBUTES section, to be
/// used on the final enum in the generated code.
/// Similarly, doc comments and attributes can be added to each element, which will go to that
/// elements stored text, to be used on the final enum in the generated code.
/// todo FIX COMMENTS
macro_rules! create_generator {
    // Internal helper branches.
    (@format_attribute doc = $text:expr) => {concat!("///", $text)};
    (@format_attribute $other:meta) => {stringify!(#[$other])};

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
                $element:ident: $xml_name:literal
            ),*
            $(,)?
        }
    ) => {
        $(#[$attribute])*
        #[non_exhaustive]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
                "#[derive(Debug, Copy, Clone, PartialEq, Eq)]",
            );

            const NAME: &'static str = stringify!($name);

            const ELEMENTS: &'static [&str] = &[
                $(stringify!($element)),*
            ];

            pub const ALL: &'static [Self] = &[
                $($name::$element),*
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
                    $($xml_name => Ok($name::$element),)*
                    other => Err(
                        String::from(concat!("invalid ", stringify!($name), "value: ")) + other
                    ),
                }
            }
        }
    };

    // The enum branch
    (
        $(#[$attribute:meta])*
        enum $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $element:ident($type:ty)
            ),*
            $(,)?
        }
        $($extra:block)*
    ) => {
        $(#[$attribute])*
        pub enum $name {
            $(
                $(#[$element_attribute])*
                $element($type),
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
                            '\t', stringify!($name($type))
                        ),
                    )*
                    _ => panic!("Unknown element"),
                }
            }
        }

        create_generator!(@generator_implementation $name, enum, $($extra),*);

        $($extra)*
    };
    
    // The struct branch.
    (
        $(#[$attribute:meta])*
        struct $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $visability:vis $element:ident: $type:ty
            ),*
            $(,)?
        }
        $($extra:tt)*
    ) => {
        $(#[$attribute])*
        pub struct $name {
            $(
                $(#[$element_attribute])*
                $visability $element: $type,
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
                            '\t', stringify!($visability), ' ', stringify!($element), ": ",
                            stringify!($type),
                        ),
                    )*
                    _ => panic!("Unknown element"),
                }
            }
        }

        create_generator!(@generator_implementation $name, struct, $($extra),*);

        $($extra)*
    };
}

pub(in crate::generator_structures) use create_generator;
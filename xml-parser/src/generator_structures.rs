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
/// Similarly, doc comments and attributes can be added to each variant, which will go to that
/// variants stored text, to be used on the final enum in the generated code.
/// todo FIX COMMENTS
macro_rules! create_generator {
    // Internal helper branches.
    (@format_attribute doc = $text:expr) => {concat!("///", $text)};
    (@format_attribute $other:meta) => {stringify!(#[$other])};

    // The XML enum branch.
    (
        $(#[$attribute:meta])*
        xml enum $name:ident {
            $(
                $(#[$variant_attribute:meta])*
                $variant:ident: $xml_name:literal
            ),*
            $(,)?
        }
    ) => {
        $(#[$attribute])*
        #[non_exhaustive]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $(
                $(#[$variant_attribute:meta])*
                $variant,
            )*
        }

        impl $name {
            const ATTRIBUTES: &'static str = concat!(
                $(create_generator!(@format_attribute $attribute), "\n",)*
                "#[non_exhaustive]\n",
                "#[derive(Debug, Copy, Clone, PartialEq, Eq)]",
            );

            const NAME: &'static str = stringify!($name);

            const ALL: &'static [Self] = &[
                $($name::$variant),*
            ];

            fn generate_variant_text(&self) -> &'static str {
                match *self {
                    $(
                        $name::$variant => concat!(
                            $('\t', create_generator!(@format_attribute $variant_attribute), "\n",)*
                            '\t', stringify!($name)
                        ),
                    )*
                }
            }
        }

        impl Generator for $name {
            fn generate_text() -> String {
                format!(
                    "{}\n\
                    pub enum {} {{\n\
                    {}\n\
                    }}",
                    Self::ATTRIBUTES,
                    Self::NAME,
                    Self::ALL
                        .into_iter()
                        .map(|element| format!("{},", Self::generate_variant_text(&element)))
                        .collect::<Vec<_>>()
                        .join("\n\n"),
                )
            }
        }

        impl TryFrom<&str> for $name {
            type Error = String;

            fn try_from<'a>(value: &'a str) -> Result<Self, Self::Error> {
                match value {
                    $($xml_name => Ok($name::$variant),)*
                    other => Err(
                        String::from(concat!("invalid ", stringify!($name), "value: ")) + other
                    ),
                }
            }
        }
    };

    // TODO regular enums.
    // These enums will not have a try_from, and will just be enums that will appear in the
    // generated file and this one.
    
    // The struct branch.
    (
        $(#[$attribute:meta])*
        struct $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $element:ident: $type:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$attribute])*
        struct $name {
            $(
                $(#[$element_attribute])*
                $element: $type,
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

            fn generate_element_text(element: &str) -> Option<&str> {
                match element {
                    $(
                        stringify!($element) => Some(concat!(
                            $('\t', create_generator!(@format_attribute $element_attribute), "\n",)*
                            '\t', stringify!($name), ": ", stringify!($type)
                        )),
                    )*
                    _ => None,
                }
            }
        }

        impl Generator for $name {
            fn generate_text() -> String {
                format!(
                    "{}\n\
                    pub struct {} {{\n\
                    {}\n\
                    }}",
                    Self::ATTRIBUTES,
                    Self::NAME,
                    Self::ELEMENTS
                        .into_iter()
                        .map(|element| format!(
                            "{},", Self::generate_element_text(element).unwrap()
                        ))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        }
    }
}

pub(in crate::generator_structures) use create_generator;
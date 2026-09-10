pub mod generated_format_enums;

/// This macro will generate an enum which implements `GeneratedEnum`.
/// It will also implement `TryFrom<&str>`.
/// The enum will automatically have `#[non_exhaustive]` and
/// `#[derive(Debug, Copy, Clone, PartialEq, Eq)]`, but other attributes and doc comments can be
/// added.
/// These other attributes and doc comments will be passed along to the ATTRIBUTES section, to be
/// used on the final enum in the generated code.
/// Similarly, doc comments and attributes can be added to each variant, which will go to that
/// variants stored text, to be used on the final enum in the generated code.
/// todo
macro_rules! generate_enum {
    // Internal helper branches
    (@format_attribute doc = $text:expr) => {concat!("///", $text)};
    (@format_attribute $other:meta) => {stringify!(#[$other])};

    (
        $(#[$attribute:meta])*
        $name:ident {
            $(
                $(#[$variant_attribute:meta])*
                ($variant:ident, $generated_name:ident, $xml_name:literal)
            ),*
            $(,)?
        }
    ) => {
        #[non_exhaustive]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $($variant),*
        }

        impl GeneratedEnum<'_> for $name {
            const ATTRIBUTES: &'static str = concat!(
                $(generate_enum!(@format_attribute $attribute), "\n",)*
                "#[non_exhaustive]\n",
                "#[derive(Debug, Copy, Clone, PartialEq, Eq)]",
            );

            const NAME: &'static str = stringify!($name);

            const ALL: &'static [Self] = &[
                $($name::$variant,)*
            ];

            fn generate_variant_text(&self) -> &'static str {
                match *self {
                    $(
                        $name::$variant => concat!(
                            $('\t', generate_enum!(@format_attribute $variant_attribute), "\n",)*
                            '\t', stringify!($generated_name)
                        ),
                    )*
                }
            }
        }

        impl TryFrom<&str> for $name {
            type Error = ();

            fn try_from<'a>(value: &'a str) -> Result<Self, Self::Error> {
                match value {
                    $($xml_name => Ok($name::$variant),)*
                    _ => Err(()),
                }
            }
        }
    };
}

pub(in crate::generated_enums) use generate_enum;

/// This macro will turn a `doc = r" text"` back into `/// text`, while leaving other meta tokens
/// alone.
macro_rules! format_attribute {
    (doc = $text:expr) => {concat!("///", $text)};
    ($other:meta) => {stringify!(#[$other])};
}

pub(in crate::generated_enums) use format_attribute;

pub trait GeneratedEnum<'a>: Sized + 'static {
    const ATTRIBUTES: &'static str;
    const NAME: &'static str;
    const ALL: &'static [Self];

    fn generate_variant_text(&self) -> &'static str;

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
                .join("\n\n")
        )
    }
}
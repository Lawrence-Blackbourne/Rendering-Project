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
    () => {};

    (@replace $old:tt, $new:tt) => {$new};

    (@format_attribute doc = $text:expr) => {concat!("///", $text)};
    (@format_attribute $other:meta) => {stringify!(#[$other])};

    (@enum_to_constant_text $name:ident, $element:ident) => {
        String::from(stringify!($name::$element))
    };
    (@enum_to_constant_text $name:ident, $element:ident, $replacement:ident, $type:ty) => {
        format!("{}({})", stringify!($name::$element), $replacement.to_constant())
    };

    (@generate_text_implementation $name:ident, $type:ident $(,)?) => {
        impl $name {
            pub fn generate_text() -> String {
                format!(
                    "{}\n\
                    pub {} {} {{\n\
                    {}\n\
                    }}",
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
                )
            }
        }
    };

    (@munch
        [names: $($names:ident)*],
        [getters: $($getter_type:ident, $xml_enum_type:ident, $identifiers:ident)*],
        [impls: $($types:ty, $blocks:tt)*],
    ) => {
        #[derive(Debug)]
        pub struct GenerationData {
            $(pub $identifiers: Vec<($xml_enum_type, $getter_type)>,)*
        }
        
        impl GenerationData {
            pub fn new() -> GenerationData {
                GenerationData {
                    $($identifiers: Vec::new(),)*
                }
            }
        }
        
        pub fn generate_text(data: GenerationData) -> String {
            let mut result = String::new();
            $(result.push_str(&$names::generate_text());)*
            $(result.push_str(&$getter_type::generate_text(data.$identifiers));)*
            $(result.push_str(
                &format!("impl {} {{{}}}", stringify!($types), stringify!($blocks))
            );)*
            result
        }
    };

    // The XML enum branch.
    (@munch
        [names: $($names:ident)*],
        [getters: $($getter_type:ident, $xml_enum_type:ident, $identifiers:ident)*],
        [impls: $($types:ty, $blocks:tt)*],
        $(#[$attribute:meta])*
        xml enum $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $element:ident $(: $xml_name:literal)?
            ),*
            $(,)?
        }
        $($tail:tt)*

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

        create_generator!(@generate_text_implementation $name, enum);

        impl TryFrom<&str> for $name {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
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

        create_generator!(@munch
            [names: $($names)* $name],
            [getters: $($getter_type, $xml_enum_type, $identifiers)*],
            [impls: $($types, $blocks)*],
            $($tail)*
        );
    };

    // The data strut branch
    (@munch
        [names: $($names:ident)*],
        [getters: $($getter_type:ident, $xml_enum_type:ident, $identifiers:ident)*],
        [impls: $($types:ty, $blocks:tt)*],
        data struct $name:ident for $enum_name:ident as $identifier:ident {
            $(
                $(#[$element_attribute:meta])*
                $element_name:ident: $type:ty
            ),*
            $(,)?
        }
        $($tail:tt)*
    ) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                $(#[$element_attribute])*
                pub $element_name: $type,
            )*
        }

        impl $name {
            pub fn generate_text(data: Vec<($enum_name, $name)>) -> String {
                //TODO check unique data entry present for all variants

                $(
                    let mut $element_name = format!(
                        "\tfn {}(&self) => {} {{\
                        \t\tmatch self {{",
                        stringify!($element_name),
                        stringify!($element_name),
                    );
                )*

                for (variant, item) in data {
                    $($element_name.push_str(
                        &format!("\t\t\t{} => {},\n", variant, item.$element_name.to_constant())
                    );)*
                }

                $(
                    $element_name += "\t\t}\n\t}";
                )*

                let mut result = String::new();
                $(result.push_str(&format!("{}\t\t}}\n\t}}", $element_name));)*
                result
            }
        }
        create_generator!(@munch
            [names: $($names)*],
            [getters: $($getter_type, $xml_enum_type, $identifiers)*
                $name, $enum_name, $identifier],
            [impls: $($types, $blocks)*],
            $($tail)*
        );
    };

    // The enum branch
    (@munch
        [names: $($names:ident)*],
        [getters: $($getter_type:ident, $xml_enum_type:ident, $identifiers:ident)*],
        [impls: $($types:ty, $blocks:tt)*],
        $(#[$attribute:meta])*
        enum $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $element:ident$(($type:ty))?
            ),*
            $(,)?
        }
        $($tail:tt)*
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

        create_generator!(@generate_text_implementation $name, enum);

        impl ToConstant for $name {
            fn to_constant(self) -> String {
                match self {
                    $($name::$element$((create_generator!(@replace $type, val)))* =>
                        create_generator!(@enum_to_constant_text $name, $element $(,val , $type)*),)*
                }
            }
        }

        create_generator!(@munch
            [names: $($names)* $name],
            [getters: $($getter_type, $xml_enum_type, $identifiers)*],
            [impls: $($types, $blocks)*],
            $($tail)*
        );
    };

    // The struct branch.
    (@munch
        [names: $($names:ident)*],
        [getters: $($getter_type:ident, $xml_enum_type:ident, $identifiers:ident)*],
        [impls: $($types:ty, $blocks:tt)*],
        $(#[$attribute:meta])*
        struct $name:ident {
            $(
                $(#[$element_attribute:meta])*
                $visibility:vis $element:ident: $type:ty
            ),*
            $(,)?
        }
        $($tail:tt)*
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

        create_generator!(@generate_text_implementation $name, struct);

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

        create_generator!(@munch
            [names: $($names)* $name],
            [getters: $($getter_type, $xml_enum_type, $identifiers)*],
            [impls: $($types, $blocks)*],
            $($tail)*
        );
    };
    
    // The impl block branch
    (@munch
        [names: $($names:ident)*],
        [getters: $($getter_type:ident, $xml_enum_type:ident, $identifiers:ident)*],
        [impls: $($types:ty, $blocks:tt)*],
        impl $type:ident $block:tt
        $($tail:tt)*
    ) => {
        impl $type $block
        create_generator!(@munch
            [names: $($names)*],
            [getters: $($getter_type, $xml_enum_type, $identifiers)*],
            [impls: $($types, $blocks)* $type, $block],
            $($tail)*
        );
    };

    (
        $($tail:tt)*
    ) => {
        create_generator!(@munch
            [names: ],
            [getters: ],
            [impls: ],
            $($tail)*
        );
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
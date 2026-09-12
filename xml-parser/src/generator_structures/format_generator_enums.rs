use super::{create_generator, Generator};

create_generator! {
    /// The class of the image format.
    /// Similar formats may share a class.
    xml enum ImageFormatClass {
        TestElement: "test"
    }
}

create_generator! {
    /// The compression scheme used in the format
    xml enum ImageFormatCompressionScheme {
        TestElement: "test"
    }
}

create_generator! {
    /// The compression scheme used in the format
    struct TestStruct {
        element1: &'static str,
        
        /// Test
        element2: ImageFormatCompressionScheme,
    }
}

create_generator! {
    /// The different options for what a channel can represent.
    xml enum ImageFormatChannel {
        Red: "R",
        Green: "G",
        Blue: "B",
        Alpha: "A",
        Depth: "D",
        Stencil: "S",
    }
}

/// This describes how the data gets converted when passed to the shader.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImageFormatComponentConversion {
    /// The data is stored as an integer given to the shaders as an integer directly.
    Int,

    /// The data is stored as an integer, and cast as a float, the value of which is equal to the
    /// value of the integer stored when passed to the shader.
    Scaled,

    /// The data is stored as an integer and cast as a float when passed to the shader.
    /// The value of the cast float is normalized to between 0 and 1 inclusively for an unsigned
    /// data type, and between -1 and 1 for a signed data type.
    Norm,

    /// The data is stored directly as a float type, and is passed as such to the shader.
    Float,

    /// The values are interpreted using sRGB non-linear encoding.
    /// Data with this type is always unsigned.
    SRGB,
}
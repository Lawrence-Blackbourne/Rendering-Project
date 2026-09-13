use ash::vk;

use super::{create_generator, Generator};

create_generator! {
    /// An image format.
    #[non_exhaustive]
    struct ImageFormat {
        /// The underlying ash format.
        format: vk::Format,

        /// The class of the format.
        class: ImageFormatClass,

        /// The number of bytes in a single block of texels.
        num_bytes: u8,

        /// The size in 3D space of each block of texels.
        block_extent: (u8, u8, u8),

        /// Stores if the data is packed or not.
        /// On a non packed data format, the data is stored in a byte array, with each component
        /// taking up as many of these bytes as needed.
        /// The components are stored with the first components in the least significant indexes in
        /// the array.
        /// On a packed data format, the data is stored in integer(s) with the same number of bits
        /// as the packed value, and these are stored in an array. Multiple components may be in a
        /// single packed value, and there may (rarely) be unused space at the end of each packed
        /// value.
        /// The components are stored with the first components in the least significant bits in the
        /// integer.
        /// The distinction matters due to the Endianess of CPUs.
        packed: Option<u8>,

        /// The components of the format.
        components: Vec<ImageFormatComponent>,

        /// The texel planes in the format.
        planes: Vec<ImageFormatPlane>,
    }

    impl ImageFormat {
        pub fn get_num_texels(&self) -> u32 {
            self.block_extent.0 as u32 * self.block_extent.1 as u32 * self.block_extent.2 as u32
        }

        //TODO
        // get plane from component
        // get components and planes
        // set getters for other elements
    }
}

create_generator! {
    /// The class of the image format.
    /// Similar formats may share a class.
    xml enum ImageFormatClass {
        TestElement: "test"
    }
}

create_generator! {
    /// The compression scheme used in the format.
    xml enum ImageFormatCompressionScheme {
        TestElement: "test"
    }
}

create_generator! {
    struct ImageFormatComponent {
        /// What channel is this (e.g. red, green, blue, etc.).
        pub channel_type: ImageFormatChannel,

        /// How the channel is stored
        pub data_type: ImageFormatComponentDataType,

        /// The index for the plane
        plane: Option<usize>,
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

create_generator! {
    struct ImageFormatComponentDataType {
        size: ImageFormatComponentDataTypeSize,

        format: ImageFormatComponentDataTypeFormat,
    }
}

create_generator! {
    /// How the data for the component is stored.
    enum ImageFormatComponentDataTypeSize {
        /// The data for the component is stored directly in the given number of bits.
        Bits(u8),

        /// The data is stored using the given compression scheme.
        Compressed(ImageFormatComponentCompressionScheme)
    }
}

create_generator! {
    /// The compression scheme used to store the image channel data.
    xml enum ImageFormatComponentCompressionScheme {
    }
}

create_generator! {
    /// This describes the data type that the data is stored using
    xml enum ImageFormatComponentDataTypeFormat {
        /// A single boolean integer with 0 as false and 1 as true.
        Boolean: "BOOL",

        /// A fixed point number with, 11 integer bits, and 5 decimal bits, using two's complement.
        SignedFixedPointWith5Decimal: "SFIXED5",

        /// A signed floating point number.
        SignedFloatingPoint: "SFLOAT",

        /// An unsigned floating point number.
        UnsignedFloatingPoint: "UFLOAT",

        /// A two's complement signed integer.
        SignedInteger: "SINT",

        /// An unsigned integer.
        UnsignedInteger: "INT",

        /// A normalised decimal in the range `[-1, 1]`.
        SignedNormalised: "SNORM",

        /// A normalised decimal in the range `[0, 1]`.
        UnsignedNormalised: "UNORM",

        /// The data is stored as a normalised decimal in the range `[0, 1]`.
        /// If this channel is `Red`, `Green`, or `Blue`, then the data is interpreted using sRGB
        /// nonlinear encoding.
        /// If this channel is `Alpha`, then the data is interpreted directly.
        /// This channel will not be of any other type.
        SRGB: "SRGB"
    }
}

//todo
struct ImageFormatPlane{}
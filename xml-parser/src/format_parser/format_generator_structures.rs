use crate::generator_structures::{create_generator, Generator};

create_generator! {
    /// An X in the format name indicates that there is unused bits
    xml enum ImageFormat {
        // 8 bits.
        // R.
        R8Uint: "VK_FORMAT_R8_UINT",
        R8Sint: "VK_FORMAT_R8_SINT",
        R8Unorm: "VK_FORMAT_R8_UNORM",
        R8Snorm: "VK_FORMAT_R8_SNORM",
        R8Uscaled: "VK_FORMAT_R8_USCALED",
        R8Sscaled: "VK_FORMAT_R8_SSCALED",
        R8Srgb: "VK_FORMAT_R8_SRGB",

        // A.
        A8Unorm: "VK_FORMAT_A8_UNORM",

        // R, G.
        R4G4UnormPack8: "VK_FORMAT_R4G4_UNORM_PACK8",

        // 16 bits.
        // R.
        R16Uint: "VK_FORMAT_R16_UINT",
        R16Sint: "VK_FORMAT_R16_SINT",
        R16Unorm: "VK_FORMAT_R16_UNORM",
        R16Snorm: "VK_FORMAT_R16_SNORM",
        R16Uscaled: "VK_FORMAT_R16_USCALED",
        R16Sscaled: "VK_FORMAT_R16_SSCALED",
        R16Sfloat: "VK_FORMAT_R16_SFLOAT",

        // D.
        D16Unorm: "VK_FORMAT_D16_UNORM",

        // R, G.
        R8G8Uint: "VK_FORMAT_R8G8_UINT",
        R8G8Sint: "VK_FORMAT_R8G8_SINT",
        R8G8Unorm: "VK_FORMAT_R8G8_UNORM",
        R8G8Snorm: "VK_FORMAT_R8G8_SNORM",
        R8G8Uscaled: "VK_FORMAT_R8G8_USCALED",
        R8G8Sscaled: "VK_FORMAT_R8G8_SSCALED",
        R8G8Srgb: "VK_FORMAT_R8G8_SRGB",

        // R, G, B.
        R5G6B5UnormPack16: "VK_FORMAT_R5G6B5_UNORM_PACK16",
        B5G6R5UnormPack16: "VK_FORMAT_B5G6R5_UNORM_PACK16",

        // R, G, B, A.
        R4G4B4A4UnormPack16: "VK_FORMAT_R4G4B4A4_UNORM_PACK16",
        B4G4R4A4UnormPack16: "VK_FORMAT_B4G4R4A4_UNORM_PACK16",
        R5G5B5A1UnormPack16: "VK_FORMAT_R5G5B5A1_UNORM_PACK16",
        B5G5R5A1UnormPack16: "VK_FORMAT_B5G5R5A1_UNORM_PACK16",
        A1R5G5B5UnormPack16: "VK_FORMAT_A1R5G5B5_UNORM_PACK16",
        A1B5G5R5UnormPack16: "VK_FORMAT_A1B5G5R5_UNORM_PACK16",

        // 24 bits.

        // R, G, B.
        R8G8B8Uint: "VK_FORMAT_R8G8B8_UINT",
        B8G8R8Uint: "VK_FORMAT_B8G8R8_UINT",
        R8G8B8Sint: "VK_FORMAT_R8G8B8_SINT",
        B8G8R8Sint: "VK_FORMAT_B8G8R8_SINT",
        R8G8B8Unorm: "VK_FORMAT_R8G8B8_UNORM",
        B8G8R8Unorm: "VK_FORMAT_B8G8R8_UNORM",
        R8G8B8Snorm: "VK_FORMAT_R8G8B8_SNORM",
        B8G8R8Snorm: "VK_FORMAT_B8G8R8_SNORM",
        R8G8B8Uscaled: "VK_FORMAT_R8G8B8_USCALED",
        B8G8R8Uscaled: "VK_FORMAT_B8G8R8_USCALED",
        R8G8B8Sscaled: "VK_FORMAT_R8G8B8_SSCALED",
        B8G8R8Sscaled: "VK_FORMAT_B8G8R8_SSCALED",
        R8G8B8Srgb: "VK_FORMAT_R8G8B8_SRGB",
        B8G8R8Srgb: "VK_FORMAT_B8G8R8_SRGB",

        // 32 bits.
        // R.
        R32Uint: "VK_FORMAT_R32_UINT",
        R32Sint: "VK_FORMAT_R32_SINT",
        R32Sfloat: "VK_FORMAT_R32_SFLOAT",

        // R, G.
        R16G16Uint: "VK_FORMAT_R16G16_UINT",
        R16G16Sint: "VK_FORMAT_R16G16_SINT",
        R16G16Unorm: "VK_FORMAT_R16G16_UNORM",
        R16G16Snorm: "VK_FORMAT_R16G16_SNORM",
        R16G16Uscaled: "VK_FORMAT_R16G16_USCALED",
        R16G16Sscaled: "VK_FORMAT_R16G16_SSCALED",
        R16G16Sfloat: "VK_FORMAT_R16G16_SFLOAT",

        // R, G, B.
        Shared5BitExponentB9G9R9UfloatPack32: "VK_FORMAT_E5B9G9R9_UFLOAT_PACK32",
        B10G11R11UfloatPack32: "VK_FORMAT_B10G11R11_UFLOAT_PACK32",

        // R, G, B, A.
        R8G8B8A8Uint: "VK_FORMAT_R8G8B8A8_UINT",
        B8G8R8A8Uint: "VK_FORMAT_B8G8R8A8_UINT",
        R8G8B8A8Sint: "VK_FORMAT_R8G8B8A8_SINT",
        B8G8R8A8Sint: "VK_FORMAT_B8G8R8A8_SINT",
        R8G8B8A8Unorm: "VK_FORMAT_R8G8B8A8_UNORM",
        B8G8R8A8Unorm: "VK_FORMAT_B8G8R8A8_UNORM",
        B8G8R8A8Snorm: "VK_FORMAT_B8G8R8A8_SNORM",
        R8G8B8A8Snorm: "VK_FORMAT_R8G8B8A8_SNORM",
        R8G8B8A8Uscaled: "VK_FORMAT_R8G8B8A8_USCALED",
        B8G8R8A8Uscaled: "VK_FORMAT_B8G8R8A8_USCALED",
        R8G8B8A8Sscaled: "VK_FORMAT_R8G8B8A8_SSCALED",
        B8G8R8A8Sscaled: "VK_FORMAT_B8G8R8A8_SSCALED",
        R8G8B8A8Srgb: "VK_FORMAT_R8G8B8A8_SRGB",
        B8G8R8A8Srgb: "VK_FORMAT_B8G8R8A8_SRGB",
        A8B8G8R8UintPack32: "VK_FORMAT_A8B8G8R8_UINT_PACK32",
        A8B8G8R8SintPack32: "VK_FORMAT_A8B8G8R8_SINT_PACK32",
        A8B8G8R8UnormPack32: "VK_FORMAT_A8B8G8R8_UNORM_PACK32",
        A8B8G8R8SnormPack32: "VK_FORMAT_A8B8G8R8_SNORM_PACK32",
        A8B8G8R8UscaledPack32: "VK_FORMAT_A8B8G8R8_USCALED_PACK32",
        A8B8G8R8SscaledPack32: "VK_FORMAT_A8B8G8R8_SSCALED_PACK32",
        A8B8G8R8SrgbPack32: "VK_FORMAT_A8B8G8R8_SRGB_PACK32",
        A2R10G10B10UintPack32: "VK_FORMAT_A2R10G10B10_UINT_PACK32",
        A2B10G10R10UintPack32: "VK_FORMAT_A2B10G10R10_UINT_PACK32",
        A2R10G10B10SintPack32: "VK_FORMAT_A2R10G10B10_SINT_PACK32",
        A2B10G10R10SintPack32: "VK_FORMAT_A2B10G10R10_SINT_PACK32",
        A2R10G10B10UnormPack32: "VK_FORMAT_A2R10G10B10_UNORM_PACK32",
        A2B10G10R10UnormPack32: "VK_FORMAT_A2B10G10R10_UNORM_PACK32",
        A2R10G10B10SnormPack32: "VK_FORMAT_A2R10G10B10_SNORM_PACK32",
        A2B10G10R10SnormPack32: "VK_FORMAT_A2B10G10R10_SNORM_PACK32",
        A2R10G10B10UscaledPack32: "VK_FORMAT_A2R10G10B10_USCALED_PACK32",
        A2B10G10R10UscaledPack32: "VK_FORMAT_A2B10G10R10_USCALED_PACK32",
        A2R10G10B10SscaledPack32: "VK_FORMAT_A2R10G10B10_SSCALED_PACK32",
        A2B10G10R10SscaledPack32: "VK_FORMAT_A2B10G10R10_SSCALED_PACK32",

        // 48 bits.
        // R, G, B.
        R16G16B16Uint: "VK_FORMAT_R16G16B16_UINT",
        R16G16B16Sint: "VK_FORMAT_R16G16B16_SINT",
        R16G16B16Unorm: "VK_FORMAT_R16G16B16_UNORM",
        R16G16B16Snorm: "VK_FORMAT_R16G16B16_SNORM",
        R16G16B16Uscaled: "VK_FORMAT_R16G16B16_USCALED",
        R16G16B16Sscaled: "VK_FORMAT_R16G16B16_SSCALED",
        R16G16B16Sfloat: "VK_FORMAT_R16G16B16_SFLOAT",

        // 64 bits.
        // R.
        R64Uint: "VK_FORMAT_R64_UINT",
        R64Sint: "VK_FORMAT_R64_SINT",
        R64Sfloat: "VK_FORMAT_R64_SFLOAT",

        // R, G.
        R32G32Uint: "VK_FORMAT_R32G32_UINT",
        R32G32Sint: "VK_FORMAT_R32G32_SINT",
        R32G32Sfloat: "VK_FORMAT_R32G32_SFLOAT",

        // R, G, B, A.
        R16G16B16A16Uint: "VK_FORMAT_R16G16B16A16_UINT",
        R16G16B16A16Sint: "VK_FORMAT_R16G16B16A16_SINT",
        R16G16B16A16Unorm: "VK_FORMAT_R16G16B16A16_UNORM",
        R16G16B16A16Snorm: "VK_FORMAT_R16G16B16A16_SNORM",
        R16G16B16A16Uscaled: "VK_FORMAT_R16G16B16A16_USCALED",
        R16G16B16A16Sscaled: "VK_FORMAT_R16G16B16A16_SSCALED",
        R16G16B16A16Sfloat: "VK_FORMAT_R16G16B16A16_SFLOAT",

        // 96 bits.
        // R, G, B.
        R32G32B32Uint: "VK_FORMAT_R32G32B32_UINT",
        R32G32B32Sint: "VK_FORMAT_R32G32B32_SINT",
        R32G32B32Sfloat: "VK_FORMAT_R32G32B32_SFLOAT",

        // 128 bits.
        // R, G.
        R64G64Uint: "VK_FORMAT_R64G64_UINT",
        R64G64Sint: "VK_FORMAT_R64G64_SINT",
        R64G64Sfloat: "VK_FORMAT_R64G64_SFLOAT",

        // R, G, B, A.
        R32G32B32A32Uint: "VK_FORMAT_R32G32B32A32_UINT",
        R32G32B32A32Sint: "VK_FORMAT_R32G32B32A32_SINT",
        R32G32B32A32Sfloat: "VK_FORMAT_R32G32B32A32_SFLOAT",

        // 196 bits.
        // R, G, B.
        R64G64B64Uint: "VK_FORMAT_R64G64B64_UINT",
        R64G64B64Sint: "VK_FORMAT_R64G64B64_SINT",
        R64G64B64Sfloat: "VK_FORMAT_R64G64B64_SFLOAT",

        // 256 bits.
        // R, G, B, A.
        R64G64B64A64Uint: "VK_FORMAT_R64G64B64A64_UINT",
        R64G64B64A64Sint: "VK_FORMAT_R64G64B64A64_SINT",
        R64G64B64A64Sfloat: "VK_FORMAT_R64G64B64A64_SFLOAT",
    }
}

create_generator! {
    /// The class of the image format.
    /// Similar formats may share a class.
    xml enum ImageFormatClass {
        EightBit: "8-bit",
        EightBitAlpha: "8-bit alpha",
        SixteenBit: "16-bit",
        SixteenBitDepth: "D16",
        TwentyFourBit: "24-bit",
        TwentyFourBitDepth: "D24",
        ThirtyTwoBit: "32-bit",
        FourtyEightBit: "48-bit",
        SixtyFourBit: "64-bit",
        NinetySixBit: "96-bit",
        OneHundredAndTwentyEightBit: "128-bit",
        OneHundredAndNinteyTwoBit: "192-bit",
        TwoHundredAndFiftySizBit: "256-bit",
    }
}

create_generator! {
    //todo
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct ImageFormatPlane{
        components: Vec<ImageFormatComponent>,

        scale_divisors: ImageFormatScaleDivisors,

        /// This represents a single plane format which is compatible with the format of this plane.
        compatible_single_plane_format: ImageFormat,
    }

    impl ImageFormatPlane {
        pub fn scale_divisors(&self) -> ImageFormatScaleDivisors {self.scale_divisors}

        pub fn compatible_single_plane_format(&self) -> ImageFormat {
            self.compatible_single_plane_format
        }
    }
}

create_generator! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct ImageFormatScaleDivisors{
        pub width: ImageFormatScaleDivisor,
        pub height: ImageFormatScaleDivisor,
    }
}

create_generator! {
    /// Image format scale divisors allow formats to have higher resolution for some components
    /// than others.
    /// The only divisors currently in use are `Regular` and `Half`.
    /// Regular means that each texel in the format has as a texel in the plane in that
    /// direction, and half means every two texels in the format have a texel in the plane in
    /// that direction.
    #[derive(PartialOrd, Ord, Default)]
    xml enum ImageFormatScaleDivisor {
        #[default] Regular: "1",
        Half: "2",
    }
}

create_generator! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct ImageFormatComponent {
        /// What channel is this (e.g. red, green, blue, etc.).
        channel: ImageFormatChannel,

        /// How the channel is stored
        data_type: ImageFormatComponentDataType,
    }

    impl ImageFormatComponent {
        pub fn channel_type(&self) -> ImageFormatChannel {self.channel}

        pub fn data_type(&self) -> ImageFormatComponentDataType {self.data_type}
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct ImageFormatComponentDataType {
        size: ImageFormatComponentDataTypeSize,

        format: ImageFormatComponentDataTypeFormat,
    }

    impl ImageFormatComponentDataType {
        pub fn size(&self) -> ImageFormatComponentDataTypeSize {self.size}

        pub fn format(&self) -> ImageFormatComponentDataTypeFormat {self.format}
    }
}

create_generator! {
    /// How the data for the component is stored.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        Test: "test"
    }
}

create_generator! {
    /// This describes the data type that the data is stored using
    xml enum ImageFormatComponentDataTypeFormat {
        /// A single boolean integer with 0 as false and 1 as true.
        Boolean: "BOOL",

        /// An unsigned integer.
        UnsignedInteger: "UINT",

        /// A two's complement signed integer.
        SignedInteger: "SINT",

        /// An unsigned floating point number.
        UnsignedFloatingPoint: "UFLOAT",

        /// A signed floating point number.
        SignedFloatingPoint: "SFLOAT",

        /// A normalised decimal in the range `[0, 1]`.
        UnsignedNormalised: "UNORM",

        /// A normalised decimal in the range `[-1, 1]`.
        SignedNormalised: "SNORM",

        /// The data is stored as an unsigned integer.
        /// When the data is passed to the shader, it is converted to a floating point number with
        /// the same value.
        /// The data may actually be using a shared exponent, watch out for that in the name of the
        /// format.
        UnsignedIntegerToFloatingPoint: "USCALED",

        /// The data is stored as a signed integer.
        /// When the data is passed to the shader, it is converted to a floating point number with
        /// the same value.
        SignedIntegerToFloatingPoint: "SSCALED",

        /// A fixed point number with, 11 integer bits, and 5 decimal bits, using two's complement.
        SignedFixedPointWith5Decimal: "SFIXED5",

        /// The data is stored as a normalised decimal in the range `[0, 1]`.
        /// If this channel is `Red`, `Green`, or `Blue`, then the data is interpreted using sRGB
        /// nonlinear encoding.
        /// If this channel is `Alpha`, then the data is interpreted directly.
        /// This channel will not be of any other type.
        SRGB: "SRGB",
    }
}
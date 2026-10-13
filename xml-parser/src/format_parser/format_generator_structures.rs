use crate::generator_structures::{Generator, ToConstant, create_generator};

create_generator! {
    #[allow(non_camel_case_types)]
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

        // S.
        S8Uint: "VK_FORMAT_S8_UINT",

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

        // R, X.
        R10X6UnormPack16: "VK_FORMAT_R10X6_UNORM_PACK16",
        R12X4UnormPack16: "VK_FORMAT_R12X4_UNORM_PACK16",

        // 24 bits.
        // D, S.
        D16UnormS8Uint: "VK_FORMAT_D16_UNORM_S8_UINT",

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
        G8_B8R8Chroma420Unorm: "VK_FORMAT_G8_B8R8_2PLANE_420_UNORM",
        G8_B8_R8Chroma420Unorm: "VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM",
        G8_B8R8Chroma422Unorm: "VK_FORMAT_G8_B8R8_2PLANE_422_UNORM",
        G8_B8_R8Chroma422Unorm: "VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM",
        G8_B8R8Chroma444Unorm: "VK_FORMAT_G8_B8R8_2PLANE_444_UNORM",
        G8_B8_R8Chroma444Unorm: "VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM",

        // 32 bits.
        // R.
        R32Uint: "VK_FORMAT_R32_UINT",
        R32Sint: "VK_FORMAT_R32_SINT",
        R32Sfloat: "VK_FORMAT_R32_SFLOAT",

        // D.
        D32Sfloat: "VK_FORMAT_D32_SFLOAT",

        // R, G.
        R16G16Uint: "VK_FORMAT_R16G16_UINT",
        R16G16Sint: "VK_FORMAT_R16G16_SINT",
        R16G16Unorm: "VK_FORMAT_R16G16_UNORM",
        R16G16Snorm: "VK_FORMAT_R16G16_SNORM",
        R16G16Uscaled: "VK_FORMAT_R16G16_USCALED",
        R16G16Sscaled: "VK_FORMAT_R16G16_SSCALED",
        R16G16Sfloat: "VK_FORMAT_R16G16_SFLOAT",

        // R, G, B.
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

        // R, G, G, B
        G8B8G8R8Chroma422Unorm: "VK_FORMAT_G8B8G8R8_422_UNORM",
        B8G8R8G8Chroma422Unorm: "VK_FORMAT_B8G8R8G8_422_UNORM",

        // R, G, B, E
        E5B9G9R9UfloatPack32: "VK_FORMAT_E5B9G9R9_UFLOAT_PACK32",

        // R, G, X, X.
        R10X6G10X6UnormPack32: "VK_FORMAT_R10X6G10X6_UNORM_2PACK16",
        R12X4G12X4UnormPack16: "VK_FORMAT_R12X4G12X4_UNORM_2PACK16",

        // 40 bits.
        // D, S.
        D32SfloatS8Uint: "VK_FORMAT_D32_SFLOAT_S8_UINT",

        // 48 bits.
        // R, G, B.
        R16G16B16Uint: "VK_FORMAT_R16G16B16_UINT",
        R16G16B16Sint: "VK_FORMAT_R16G16B16_SINT",
        R16G16B16Unorm: "VK_FORMAT_R16G16B16_UNORM",
        R16G16B16Snorm: "VK_FORMAT_R16G16B16_SNORM",
        R16G16B16Uscaled: "VK_FORMAT_R16G16B16_USCALED",
        R16G16B16Sscaled: "VK_FORMAT_R16G16B16_SSCALED",
        R16G16B16Sfloat: "VK_FORMAT_R16G16B16_SFLOAT",
        G16_B16R16Chroma420Unorm: "VK_FORMAT_G16_B16R16_2PLANE_420_UNORM",
        G16_B16_R16Chroma420Unorm: "VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM",
        G16_B16R16Chroma422Unorm: "VK_FORMAT_G16_B16R16_2PLANE_422_UNORM",
        G16_B16_R16Chroma422Unorm: "VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM",
        G16_B16R16Chroma444Unorm: "VK_FORMAT_G16_B16R16_2PLANE_444_UNORM",
        G16_B16_R16Chroma444Unorm: "VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM",

        // R, G, B, X, X, X.
        G10X6_B10X6R10X6Chroma420UnormPack16: "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16",
        G12X4_B12X4R12X4Chroma420UnormPack16: "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16",
        G10X6_B10X6_R10X6Chroma420UnormPack16:
            "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16",
        G12X4_B12X4_R12X4Chroma420UnormPack16:
            "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16",
        G10X6_B10X6R10X6Chroma422UnormPack16: "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16",
        G12X4_B12X4R12X4Chroma422UnormPack16: "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16",
        G10X6_B10X6_R10X6Chroma422UnormPack16:
            "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16",
        G12X4_B12X4_R12X4Chroma422UnormPack16:
            "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16",
        G10X6_B10X6R10X6Chroma444UnormPack16: "VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16",
        G12X4_B12X4R12X4Chroma444UnormPack16: "VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16",
        G10X6_B10X6_R10X6Chroma444UnormPack16:
            "VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16",
        G12X4_B12X4_R12X4Chroma444UnormPack16:
            "VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16",

        // 64 bits.
        // R.
        R64Uint: "VK_FORMAT_R64_UINT",
        R64Sint: "VK_FORMAT_R64_SINT",
        R64Sfloat: "VK_FORMAT_R64_SFLOAT",

        // R, G.
        R32G32Uint: "VK_FORMAT_R32G32_UINT",
        R32G32Sint: "VK_FORMAT_R32G32_SINT",
        R32G32Sfloat: "VK_FORMAT_R32G32_SFLOAT",

        // R, G, G, B.
        G16B16G16R16Chroma422Unorm: "VK_FORMAT_G16B16G16R16_422_UNORM",
        B16G16R16G16Chroma422Unorm: "VK_FORMAT_B16G16R16G16_422_UNORM",

        // R, G, B, A.
        R16G16B16A16Uint: "VK_FORMAT_R16G16B16A16_UINT",
        R16G16B16A16Sint: "VK_FORMAT_R16G16B16A16_SINT",
        R16G16B16A16Unorm: "VK_FORMAT_R16G16B16A16_UNORM",
        R16G16B16A16Snorm: "VK_FORMAT_R16G16B16A16_SNORM",
        R16G16B16A16Uscaled: "VK_FORMAT_R16G16B16A16_USCALED",
        R16G16B16A16Sscaled: "VK_FORMAT_R16G16B16A16_SSCALED",
        R16G16B16A16Sfloat: "VK_FORMAT_R16G16B16A16_SFLOAT",

        // R, G, G, B, X, X, X, X
        G10X6B10X6G10X6R10X6Chroma422UnormPack16:
            "VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16",
        G12X4B12X4G12X4R12X4Chroma422UnormPack16:
            "VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16",
        B10X6G10X6R10X6G10X6Chroma422UnormPack16:
            "VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16",
        B12X4G12X4R12X4G12X4Chroma422UnormPack16:
            "VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16",

        // R, G, B, A, X, X, X, X
        R10X6G10X6B10X6A10X6UnormPack16: "VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16",
        R12X4G12X4B12X4A12X4UnormPack16: "VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16",

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

        // 192 bits.
        // R, G, B.
        R64G64B64Uint: "VK_FORMAT_R64G64B64_UINT",
        R64G64B64Sint: "VK_FORMAT_R64G64B64_SINT",
        R64G64B64Sfloat: "VK_FORMAT_R64G64B64_SFLOAT",

        // 256 bits.
        // R, G, B, A.
        R64G64B64A64Uint: "VK_FORMAT_R64G64B64A64_UINT",
        R64G64B64A64Sint: "VK_FORMAT_R64G64B64A64_SINT",
        R64G64B64A64Sfloat: "VK_FORMAT_R64G64B64A64_SFLOAT",

        // BC1.
        // R, G, B.
        Bc1RGBUnorm: "VK_FORMAT_BC1_RGB_UNORM_BLOCK",
        Bc1RGBSrgb: "VK_FORMAT_BC1_RGB_SRGB_BLOCK",

        // R, G, B, A.
        Bc1RGBAUnorm: "VK_FORMAT_BC1_RGBA_UNORM_BLOCK",
        Bc1RGBASrgb: "VK_FORMAT_BC1_RGBA_SRGB_BLOCK",

        // BC2.
        // R, G, B, A.
        Bc2Unorm: "VK_FORMAT_BC2_UNORM_BLOCK",
        Bc2Srgb: "VK_FORMAT_BC2_SRGB_BLOCK",

        // BC3.
        // R, G, B, A.
        Bc3Unorm: "VK_FORMAT_BC3_UNORM_BLOCK",
        Bc3Srgb: "VK_FORMAT_BC3_SRGB_BLOCK",

        // BC4.
        // R.
        Bc4Unorm: "VK_FORMAT_BC4_UNORM_BLOCK",
        Bc4Snorm: "VK_FORMAT_BC4_SNORM_BLOCK",

        // BC5.
        // R, G.
        Bc5Unorm: "VK_FORMAT_BC5_UNORM_BLOCK",
        Bc5Snorm: "VK_FORMAT_BC5_SNORM_BLOCK",

        // BC6H.
        // R, G, B.
        Bc6hUfloat: "VK_FORMAT_BC6H_UFLOAT_BLOCK",
        Bc6hSfloat: "VK_FORMAT_BC6H_SFLOAT_BLOCK",

        // BC7
        // R, G, B, A.
        Bc7Unorm: "VK_FORMAT_BC7_UNORM_BLOCK",
        Bc7Srgb: "VK_FORMAT_BC7_SRGB_BLOCK",

        // ETC2
        // R, G, B.
        Etc2R8G8B8Unorm: "VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK",
        Etc2R8G8B8Srgb: "VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK",

        // R, G, B, A.
        Etc2R8G8B8A1Unorm: "VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK",
        Etc2R8G8B8A1Srgb: "VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK",

        // ETC2 EAC.
        // R, G, B, A.
        Etc2EacR8G8B8A8Unorm: "VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK",
        Etc2EacR8G8B8A8Srgb: "VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK",

        // EAC.
        // R.
        EacR11Unorm: "VK_FORMAT_EAC_R11_UNORM_BLOCK",
        EacR11Snorm: "VK_FORMAT_EAC_R11_SNORM_BLOCK",

        // R, G.
        EacR11G11Unorm: "VK_FORMAT_EAC_R11G11_UNORM_BLOCK",
        EacR11G11Snorm: "VK_FORMAT_EAC_R11G11_SNORM_BLOCK",

        // ASTC.
        // R, G, B, A.
        Astc4x4Sfloat: "VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK",
        Astc4x4Unorm: "VK_FORMAT_ASTC_4x4_UNORM_BLOCK",
        Astc4x4Srgb: "VK_FORMAT_ASTC_4x4_SRGB_BLOCK",
        Astc5x4Sfloat: "VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK",
        Astc5x4Unorm: "VK_FORMAT_ASTC_5x4_UNORM_BLOCK",
        Astc5x4Srgb: "VK_FORMAT_ASTC_5x4_SRGB_BLOCK",
        Astc5x5Sfloat: "VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK",
        Astc5x5Unorm: "VK_FORMAT_ASTC_5x5_UNORM_BLOCK",
        Astc5x5Srgb: "VK_FORMAT_ASTC_5x5_SRGB_BLOCK",
        Astc6x5Sfloat: "VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK",
        Astc6x5Unorm: "VK_FORMAT_ASTC_6x5_UNORM_BLOCK",
        Astc6x5Srgb: "VK_FORMAT_ASTC_6x5_SRGB_BLOCK",
        Astc6x6Sfloat: "VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK",
        Astc6x6Unorm: "VK_FORMAT_ASTC_6x6_UNORM_BLOCK",
        Astc6x6Srgb: "VK_FORMAT_ASTC_6x6_SRGB_BLOCK",
        Astc8x5Sfloat: "VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK",
        Astc8x5Unorm: "VK_FORMAT_ASTC_8x5_UNORM_BLOCK",
        Astc8x5Srgb: "VK_FORMAT_ASTC_8x5_SRGB_BLOCK",
        Astc8x6Sfloat: "VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK",
        Astc8x6Unorm: "VK_FORMAT_ASTC_8x6_UNORM_BLOCK",
        Astc8x6Srgb: "VK_FORMAT_ASTC_8x6_SRGB_BLOCK",
        Astc8x8Sfloat: "VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK",
        Astc8x8Unorm: "VK_FORMAT_ASTC_8x8_UNORM_BLOCK",
        Astc8x8Srgb: "VK_FORMAT_ASTC_8x8_SRGB_BLOCK",
        Astc10x5Sfloat: "VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK",
        Astc10x5Unorm: "VK_FORMAT_ASTC_10x5_UNORM_BLOCK",
        Astc10x5Srgb: "VK_FORMAT_ASTC_10x5_SRGB_BLOCK",
        Astc10x6Sfloat: "VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK",
        Astc10x6Unorm: "VK_FORMAT_ASTC_10x6_UNORM_BLOCK",
        Astc10x6Srgb: "VK_FORMAT_ASTC_10x6_SRGB_BLOCK",
        Astc10x8Sfloat: "VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK",
        Astc10x8Unorm: "VK_FORMAT_ASTC_10x8_UNORM_BLOCK",
        Astc10x8Srgb: "VK_FORMAT_ASTC_10x8_SRGB_BLOCK",
        Astc10x10Sfloat: "VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK",
        Astc10x10Unorm: "VK_FORMAT_ASTC_10x10_UNORM_BLOCK",
        Astc10x10Srgb: "VK_FORMAT_ASTC_10x10_SRGB_BLOCK",
        Astc12x10Sfloat: "VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK",
        Astc12x10Unorm: "VK_FORMAT_ASTC_12x10_UNORM_BLOCK",
        Astc12x10Srgb: "VK_FORMAT_ASTC_12x10_SRGB_BLOCK",
        Astc12x12Sfloat: "VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK",
        Astc12x12Unorm: "VK_FORMAT_ASTC_12x12_UNORM_BLOCK",
        Astc12x12Srgb: "VK_FORMAT_ASTC_12x12_SRGB_BLOCK",
        Astc3x3x3Sfloat: "VK_FORMAT_ASTC_3x3x3_SFLOAT_BLOCK_EXT",
        Astc3x3x3Unorm: "VK_FORMAT_ASTC_3x3x3_UNORM_BLOCK_EXT",
        Astc3x3x3Srgb: "VK_FORMAT_ASTC_3x3x3_SRGB_BLOCK_EXT",
        Astc4x3x3Sfloat: "VK_FORMAT_ASTC_4x3x3_SFLOAT_BLOCK_EXT",
        Astc4x3x3Unorm: "VK_FORMAT_ASTC_4x3x3_UNORM_BLOCK_EXT",
        Astc4x3x3Srgb: "VK_FORMAT_ASTC_4x3x3_SRGB_BLOCK_EXT",
        Astc4x4x3Sfloat: "VK_FORMAT_ASTC_4x4x3_SFLOAT_BLOCK_EXT",
        Astc4x4x3Unorm: "VK_FORMAT_ASTC_4x4x3_UNORM_BLOCK_EXT",
        Astc4x4x3Srgb: "VK_FORMAT_ASTC_4x4x3_SRGB_BLOCK_EXT",
        Astc4x4x4Sfloat: "VK_FORMAT_ASTC_4x4x4_SFLOAT_BLOCK_EXT",
        Astc4x4x4Unorm: "VK_FORMAT_ASTC_4x4x4_UNORM_BLOCK_EXT",
        Astc4x4x4Srgb: "VK_FORMAT_ASTC_4x4x4_SRGB_BLOCK_EXT",
        Astc5x4x4Sfloat: "VK_FORMAT_ASTC_5x4x4_SFLOAT_BLOCK_EXT",
        Astc5x4x4Unorm: "VK_FORMAT_ASTC_5x4x4_UNORM_BLOCK_EXT",
        Astc5x4x4Srgb: "VK_FORMAT_ASTC_5x4x4_SRGB_BLOCK_EXT",
        Astc5x5x4Sfloat: "VK_FORMAT_ASTC_5x5x4_SFLOAT_BLOCK_EXT",
        Astc5x5x4Unorm: "VK_FORMAT_ASTC_5x5x4_UNORM_BLOCK_EXT",
        Astc5x5x4Srgb: "VK_FORMAT_ASTC_5x5x4_SRGB_BLOCK_EXT",
        Astc5x5x5Sfloat: "VK_FORMAT_ASTC_5x5x5_SFLOAT_BLOCK_EXT",
        Astc5x5x5Unorm: "VK_FORMAT_ASTC_5x5x5_UNORM_BLOCK_EXT",
        Astc5x5x5Srgb: "VK_FORMAT_ASTC_5x5x5_SRGB_BLOCK_EXT",
        Astc6x5x5Sfloat: "VK_FORMAT_ASTC_6x5x5_SFLOAT_BLOCK_EXT",
        Astc6x5x5Unorm: "VK_FORMAT_ASTC_6x5x5_UNORM_BLOCK_EXT",
        Astc6x5x5Srgb: "VK_FORMAT_ASTC_6x5x5_SRGB_BLOCK_EXT",
        Astc6x6x5Sfloat: "VK_FORMAT_ASTC_6x6x5_SFLOAT_BLOCK_EXT",
        Astc6x6x5Unorm: "VK_FORMAT_ASTC_6x6x5_UNORM_BLOCK_EXT",
        Astc6x6x5Srgb: "VK_FORMAT_ASTC_6x6x5_SRGB_BLOCK_EXT",
        Astc6x6x6Sfloat: "VK_FORMAT_ASTC_6x6x6_SFLOAT_BLOCK_EXT",
        Astc6x6x6Unorm: "VK_FORMAT_ASTC_6x6x6_UNORM_BLOCK_EXT",
        Astc6x6x6Srgb: "VK_FORMAT_ASTC_6x6x6_SRGB_BLOCK_EXT",
    }

    data struct ImageFormatData {
        num_bits: u32,
        block_extent: (u32, u32, u32),
        packed: Option<u32>,
        planes: Vec<ImageFormatPlane>,
    }
}

create_generator! {
    //TODO figure out which of these are no longer needed
    /// The class of the image format.
    /// Similar formats may share a class.
    xml enum ImageFormatClass {
        // Uncompressed
        Bit8: "8-bit",
        A8: "8-bit alpha",
        S8: "S8",
        Bit16: "16-bit",
        D16: "D16",
        B24: "24-bit",
        D16S8: "D16S8",
        Bit32: "32-bit",
        D32: "D32",
        G8B8G8R8: "32-bit G8B8G8R8",
        B8G8R8G8: "32-bit B8G8R8G8",
        D32S8: "D32S8",
        Bit48: "48-bit",
        Bit64: "64-bit",
        Bit64R10G10B10A10: "64-bit R10G10B10A10",
        Bit64R12G12B12A12: "64-bit R12G12B12A12",
        Bit64G10B10G10R10: "64-bit G10B10G10R10",
        Bit64G12B12G12R12: "64-bit G12B12G12R12",
        Bit64G16B16G16R16: "64-bit G16B16G16R16",
        Bit64B10G10R10G10: "64-bit B10G10R10G10",
        Bit64B12G12R12G12: "64-bit B12G12R12G12",
        Bit64B16G16R16G16: "64-bit B16G16R16G16",
        Bit96: "96-bit",
        Bit128: "128-bit",
        Bit192: "192-bit",
        Bit256: "256-bit",

        // Planar
        B8P2_420: "8-bit 2-plane 420",
        B8P3_420: "8-bit 3-plane 420",
        B8P2_422: "8-bit 2-plane 422",
        B8P3_422: "8-bit 3-plane 422",
        B8P2_444: "8-bit 2-plane 444",
        B8P3_444: "8-bit 3-plane 444",
        B10P2_420: "10-bit 2-plane 420",
        B10P3_420: "10-bit 3-plane 420",
        B10P2_422: "10-bit 2-plane 422",
        B10P3_422: "10-bit 3-plane 422",
        B10P2_444: "10-bit 2-plane 444",
        B10P3_444: "10-bit 3-plane 444",
        B12P2_420: "12-bit 2-plane 420",
        B12P3_420: "12-bit 3-plane 420",
        B12P2_422: "12-bit 2-plane 422",
        B12P3_422: "12-bit 3-plane 422",
        B12P2_444: "12-bit 2-plane 444",
        B12P3_444: "12-bit 3-plane 444",
        B16P2_420: "16-bit 2-plane 420",
        B16P3_420: "16-bit 3-plane 420",
        B16P2_422: "16-bit 2-plane 422",
        B16P3_422: "16-bit 3-plane 422",
        B16P2_444: "16-bit 2-plane 444",
        B16P3_444: "16-bit 3-plane 444",

        // Block Compression
        Bc1Rgb: "BC1_RGB",
        Bc1Rgba: "BC1_RGBA",
        Bc2: "BC2",
        Bc3: "BC3",
        Bc4: "BC4",
        Bc5: "BC5",
        Bc6h: "BC6H",
        Bc7: "BC7",

        // ETC & EAC
        Etc2Rgb: "ETC2_RGB",
        Etc2Rgba: "ETC2_RGBA",
        Etc2EacRgba: "ETC2_EAC_RGBA",
        EacR: "EAC_R",
        EacRG: "EAC_RG",

        // ASTC
        Astc4x4: "ASTC_4x4",
        Astc5x4: "ASTC_5x4",
        Astc5x5: "ASTC_5x5",
        Astc6x5: "ASTC_6x5",
        Astc6x6: "ASTC_6x6",
        Astc8x5: "ASTC_8x5",
        Astc8x6: "ASTC_8x6",
        Astc8x8: "ASTC_8x8",
        Astc10x5: "ASTC_10x5",
        Astc10x6: "ASTC_10x6",
        Astc10x8: "ASTC_10x8",
        Astc10x10: "ASTC_10x10",
        Astc12x10: "ASTC_12x10",
        Astc12x12: "ASTC_12x12",
        Astc3x3x3: "ASTC_3x3x3",
        Astc4x3x3: "ASTC_4x3x3",
        Astc4x4x3: "ASTC_4x4x3",
        Astc4x4x4: "ASTC_4x4x4",
        Astc5x4x4: "ASTC_5x4x4",
        Astc5x5x4: "ASTC_5x5x4",
        Astc5x5x5: "ASTC_5x5x5",
        Astc6x5x5: "ASTC_6x5x5",
        Astc6x6x5: "ASTC_6x6x5",
        Astc6x6x6: "ASTC_6x6x6",
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

        /// The data in this channel will NOT be passed to any shader.
        /// These bits are entirely unused and are there only for padding purposes.
        Unused: "X",

        /// This channel represents a shared exponent that the other channels use.
        /// All other channels that use the shared exponent must have the data type format
        /// `UnsignedIntegerToFloatingPoint`.
        SharedExponent: "E",
    }
}

impl TryFrom<char> for ImageFormatChannel {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::try_from(&*value.encode_utf8(&mut [0; 4]))
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
        Bits(u32),

        /// The data is stored using the given compression scheme.
        Compressed(ImageFormatComponentCompressionScheme)
    }
}

create_generator! {
    /// The compression scheme used to store the image channel data.
    xml enum ImageFormatComponentCompressionScheme {
        BlockCompression: "BC",
        EricssonTextureCompressionTwo: "ETC2",
        EricssonAlphaCompression: "EAC",
        AdaptiveScalableTextureCompressionLowDynamicRange: "ASTC LDR",
        AdaptiveScalableTextureCompressionHighDynamicRange: "ASTC HDR",
    }
}

create_generator! {
    xml enum ChromaSubsampler {
        Chroma420: "420",
        Chroma422: "422",
        Chroma444: "444",
    }
}

create_generator! {
    /// This describes the data type that the data is stored using
    xml enum ImageFormatComponentDataTypeFormat {
        /// An unsigned integer.
        UnsignedInteger: "UINT",

        /// A two's complement signed integer.
        SignedInteger: "SINT",

        /// An unsigned floating point number.
        /// The data may actually be using a shared exponent.
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
        UnsignedIntegerToFloatingPoint: "USCALED",

        /// The data is stored as a signed integer.
        /// When the data is passed to the shader, it is converted to a floating point number with
        /// the same value.
        SignedIntegerToFloatingPoint: "SSCALED",

        /// The data is stored as a normalised decimal in the range `[0, 1]`.
        /// If this channel is `Red`, `Green`, or `Blue`, then the data is interpreted using sRGB
        /// nonlinear encoding.
        /// If this channel is `Alpha`, then the data is interpreted directly.
        /// This channel will not be of any other type.
        SRGB: "SRGB",

        /// This format is used for unused bits and shared exponents, as neither have a direct
        /// format.
        None,
    }
}

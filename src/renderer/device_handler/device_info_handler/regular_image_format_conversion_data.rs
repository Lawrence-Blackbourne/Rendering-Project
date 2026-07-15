use super::RegularImageFormatConversion::{self, Float, Int, Norm, SRGB, Scaled};
use super::RegularImageFormatOrder::{
    self, ABGR, ARGB, BGR, BGRA, D, R, RG, RGB, RGBA, RX, RXGX, RXGXBXAX,
};
use ash::vk::Format;

// rustfmt::skip is used here to avoid cargo fmt from splitting every element over many
// lines, causing this code block to become incredibly long.
// Data is (Format, red, green, blue, alpha, depth, unused, signed, packed, order, conversion)
#[rustfmt::skip]
pub(super) const REGULAR_IMAGE_FORMAT_CONVERSION_DATA: &[(
    Format,
    u8, u8, u8, u8, u8, u8,
    bool, bool,
    RegularImageFormatOrder,
    RegularImageFormatConversion
)] = &[
    // Vulkan Version 1.0.
    (Format::R4G4_UNORM_PACK8, 4, 4, 0, 0, 0, 0, false, true, RG, Norm),

    // Vulkan Version 1.0.
    (Format::R4G4B4A4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, RGBA, Norm),
    (Format::B4G4R4A4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, BGRA, Norm),
    // Vulkan Version 1.3.
    (Format::A4R4G4B4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, ARGB, Norm),
    (Format::A4B4G4R4_UNORM_PACK16, 4, 4, 4, 4, 0, 0, false, true, ABGR, Norm),

    // Vulkan Version 1.0.
    (Format::R5G6B5_UNORM_PACK16, 5, 6, 5, 0, 0, 0, false, true, RGB, Norm),
    (Format::B5G6R5_UNORM_PACK16, 5, 6, 5, 0, 0, 0, false, true, BGR, Norm),

    // Vulkan Version 1.0.
    (Format::R5G5B5A1_UNORM_PACK16, 5, 5, 5, 1, 0, 0, false, true, RGBA, Norm),
    (Format::B5G5R5A1_UNORM_PACK16, 5, 5, 5, 1, 0, 0, false, true, BGRA, Norm),
    (Format::A1R5G5B5_UNORM_PACK16, 5, 5, 5, 1, 0, 0, false, true, ARGB, Norm),

    // Vulkan Version 1.0.
    (Format::R8_UNORM, 8, 0, 0, 0, 0, 0, false, false, R, Norm),
    (Format::R8_SNORM, 8, 0, 0, 0, 0, 0, true, false, R, Norm),
    (Format::R8_USCALED, 8, 0, 0, 0, 0, 0, false, false, R, Scaled),
    (Format::R8_SSCALED, 8, 0, 0, 0, 0, 0, true, false, R, Scaled),
    (Format::R8_UINT, 8, 0, 0, 0, 0, 0, false, false, R, Int),
    (Format::R8_SINT, 8, 0, 0, 0, 0, 0, true, false, R, Int),
    (Format::R8_SRGB, 8, 0, 0, 0, 0, 0, false, false, R, SRGB),

    // Vulkan Version 1.0.
    (Format::R8G8_UNORM, 8, 8, 0, 0, 0, 0, false, false, RG, Norm),
    (Format::R8G8_SNORM, 8, 8, 0, 0, 0, 0, true, false, RG, Norm),
    (Format::R8G8_USCALED, 8, 8, 0, 0, 0, 0, false, false, RG, Scaled),
    (Format::R8G8_SSCALED, 8, 8, 0, 0, 0, 0, true, false, RG, Scaled),
    (Format::R8G8_UINT, 8, 8, 0, 0, 0, 0, false, false, RG, Int),
    (Format::R8G8_SINT, 8, 8, 0, 0, 0, 0, true, false, RG, Int),
    (Format::R8G8_SRGB, 8, 8, 0, 0, 0, 0, false, false, RG, SRGB),

    // Vulkan Version 1.0.
    (Format::R8G8B8_UNORM, 8, 8, 8, 0, 0, 0, false, false, RGB, Norm),
    (Format::R8G8B8_SNORM, 8, 8, 8, 0, 0, 0, true, false, RGB, Norm),
    (Format::R8G8B8_USCALED, 8, 8, 8, 0, 0, 0, false, false, RGB, Scaled),
    (Format::R8G8B8_SSCALED, 8, 8, 8, 0, 0, 0, true, false, RGB, Scaled),
    (Format::R8G8B8_UINT, 8, 8, 8, 0, 0, 0, false, false, RGB, Int),
    (Format::R8G8B8_SINT, 8, 8, 8, 0, 0, 0, true, false, RGB, Int),
    (Format::R8G8B8_SRGB, 8, 8, 8, 0, 0, 0, false, false, RGB, SRGB),

    // Vulkan Version 1.0.
    (Format::B8G8R8_UNORM, 8, 8, 8, 0, 0, 0, false, false, BGR, Norm),
    (Format::B8G8R8_SNORM, 8, 8, 8, 0, 0, 0, true, false, BGR, Norm),
    (Format::B8G8R8_USCALED, 8, 8, 8, 0, 0, 0, false, false, BGR, Scaled),
    (Format::B8G8R8_SSCALED, 8, 8, 8, 0, 0, 0, true, false, BGR, Scaled),
    (Format::B8G8R8_UINT, 8, 8, 8, 0, 0, 0, false, false, BGR, Int),
    (Format::B8G8R8_SINT, 8, 8, 8, 0, 0, 0, true, false, BGR, Int),
    (Format::B8G8R8_SRGB, 8, 8, 8, 0, 0, 0, false, false, BGR, SRGB),

    // Vulkan Version 1.0.
    (Format::R8G8B8A8_UNORM, 8, 8, 8, 8, 0, 0, false, false, RGBA, Norm),
    (Format::R8G8B8A8_SNORM, 8, 8, 8, 8, 0, 0, true, false, RGBA, Norm),
    (Format::R8G8B8A8_USCALED, 8, 8, 8, 8, 0, 0, false, false, RGBA, Scaled),
    (Format::R8G8B8A8_SSCALED, 8, 8, 8, 8, 0, 0, true, false, RGBA, Scaled),
    (Format::R8G8B8A8_UINT, 8, 8, 8, 8, 0, 0, false, false, RGBA, Int),
    (Format::R8G8B8A8_SINT, 8, 8, 8, 8, 0, 0, true, false, RGBA, Int),
    (Format::R8G8B8A8_SRGB, 8, 8, 8, 8, 0, 0, false, false, RGBA, SRGB),

    // Vulkan Version 1.0.
    (Format::B8G8R8A8_UNORM, 8, 8, 8, 8, 0, 0, false, false, BGRA, Norm),
    (Format::B8G8R8A8_SNORM, 8, 8, 8, 8, 0, 0, true, false, BGRA, Norm),
    (Format::B8G8R8A8_USCALED, 8, 8, 8, 8, 0, 0, false, false, BGRA, Scaled),
    (Format::B8G8R8A8_SSCALED, 8, 8, 8, 8, 0, 0, true, false, BGRA, Scaled),
    (Format::B8G8R8A8_UINT, 8, 8, 8, 8, 0, 0, false, false, BGRA, Int),
    (Format::B8G8R8A8_SINT, 8, 8, 8, 8, 0, 0, true, false, BGRA, Int),
    (Format::B8G8R8A8_SRGB, 8, 8, 8, 8, 0, 0, false, false, BGRA, SRGB),

    // Vulkan Version 1.0.
    (Format::A8B8G8R8_UNORM_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, Norm),
    (Format::A8B8G8R8_SNORM_PACK32, 8, 8, 8, 8, 0, 0, true, true, ABGR, Norm),
    (Format::A8B8G8R8_USCALED_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, Scaled),
    (Format::A8B8G8R8_SSCALED_PACK32, 8, 8, 8, 8, 0, 0, true, true, ABGR, Scaled),
    (Format::A8B8G8R8_UINT_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, Int),
    (Format::A8B8G8R8_SINT_PACK32, 8, 8, 8, 8, 0, 0, true, true, ABGR, Int),
    (Format::A8B8G8R8_SRGB_PACK32, 8, 8, 8, 8, 0, 0, false, true, ABGR, SRGB),

    // Vulkan Version 1.0.
    (Format::A2R10G10B10_UNORM_PACK32, 10, 10, 10, 2, 0, 0, false, true, ARGB, Norm),
    (Format::A2R10G10B10_SNORM_PACK32, 10, 10, 10, 2, 0, 0, true, true, ARGB, Norm),
    (Format::A2R10G10B10_USCALED_PACK32, 10, 10, 10, 2, 0, 0, false, true, ARGB, Scaled),
    (Format::A2R10G10B10_SSCALED_PACK32, 10, 10, 10, 2, 0, 0, true, true, ARGB, Scaled),
    (Format::A2R10G10B10_UINT_PACK32, 10, 10, 10, 2, 0, 0, false, true, ARGB, Int),
    (Format::A2R10G10B10_SINT_PACK32, 10, 10, 10, 2, 0, 0, true, true, ARGB, Int),

    // Vulkan Version 1.0.
    (Format::A2B10G10R10_UNORM_PACK32, 10, 10, 10, 2, 0, 0, false, true, ABGR, Norm),
    (Format::A2B10G10R10_SNORM_PACK32, 10, 10, 10, 2, 0, 0, true, true, ABGR, Norm),
    (Format::A2B10G10R10_USCALED_PACK32, 10, 10, 10, 2, 0, 0, false, true, ABGR, Scaled),
    (Format::A2B10G10R10_SSCALED_PACK32, 10, 10, 10, 2, 0, 0, true, true, ABGR, Scaled),
    (Format::A2B10G10R10_UINT_PACK32, 10, 10, 10, 2, 0, 0, false, true, ABGR, Int),
    (Format::A2B10G10R10_SINT_PACK32, 10, 10, 10, 2, 0, 0, true, true, ABGR, Int),

    // Vulkan Version 1.0.
    (Format::R16_UNORM, 16, 0, 0, 0, 0, 0, false, false, R, Norm),
    (Format::R16_SNORM, 16, 0, 0, 0, 0, 0, true, false, R, Norm),
    (Format::R16_USCALED, 16, 0, 0, 0, 0, 0, false, false, R, Scaled),
    (Format::R16_SSCALED, 16, 0, 0, 0, 0, 0,true, false, R, Scaled),
    (Format::R16_UINT, 16, 0, 0, 0, 0, 0, false, false, R, Int),
    (Format::R16_SINT, 16, 0, 0, 0, 0, 0, true, false, R, Int),
    (Format::R16_SFLOAT, 16, 0, 0, 0, 0, 0, true, false, R, Float),

    // Vulkan Version 1.0.
    (Format::R16G16_UNORM, 16, 16, 0, 0, 0, 0, false, false, RG, Norm),
    (Format::R16G16_SNORM, 16, 16, 0, 0, 0, 0, true, false, RG, Norm),
    (Format::R16G16_USCALED, 16, 16, 0, 0, 0, 0, false, false, RG, Scaled),
    (Format::R16G16_SSCALED, 16, 16, 0, 0, 0, 0, true, false, RG, Scaled),
    (Format::R16G16_UINT, 16, 16, 0, 0, 0, 0, false, false, RG, Int),
    (Format::R16G16_SINT, 16, 16, 0, 0, 0, 0, true, false, RG, Int),
    (Format::R16G16_SFLOAT, 16, 16, 0, 0, 0, 0, true, false, RG, Float),

    // Vulkan Version 1.0.
    (Format::R16G16B16_UNORM, 16, 16, 16, 0, 0, 0, false, false, RGB, Norm),
    (Format::R16G16B16_SNORM, 16, 16, 16, 0, 0, 0, true, false, RGB, Norm),
    (Format::R16G16B16_USCALED, 16, 16, 16, 0, 0, 0, false, false, RGB, Scaled),
    (Format::R16G16B16_SSCALED, 16, 16, 16, 0, 0, 0, true, false, RGB, Scaled),
    (Format::R16G16B16_UINT, 16, 16, 16, 0, 0, 0, false, false, RGB, Int),
    (Format::R16G16B16_SINT, 16, 16, 16, 0, 0, 0, true, false, RGB, Int),
    (Format::R16G16B16_SFLOAT, 16, 16, 16, 0, 0, 0, true, false, RGB, Float),

    // Vulkan Version 1.0.
    (Format::R16G16B16A16_UNORM, 16, 16, 16, 16, 0, 0, false, false, RGBA, Norm),
    (Format::R16G16B16A16_SNORM, 16, 16, 16, 16, 0, 0, true, false, RGBA, Norm),
    (Format::R16G16B16A16_USCALED, 16, 16, 16, 16, 0, 0, false, false, RGBA, Scaled),
    (Format::R16G16B16A16_SSCALED, 16, 16, 16, 16, 0, 0, true, false, RGBA, Scaled),
    (Format::R16G16B16A16_UINT, 16, 16, 16, 16, 0, 0, false, false, RGBA, Int),
    (Format::R16G16B16A16_SINT, 16, 16, 16, 16, 0, 0, true, false, RGBA, Int),
    (Format::R16G16B16A16_SFLOAT, 16, 16, 16, 16, 0, 0, true, false, RGBA, Float),

    // Vulkan Version 1.0.
    (Format::R32_UINT, 32, 0, 0, 0, 0, 0, false, false, R, Int),
    (Format::R32_SINT, 32, 0, 0, 0, 0, 0, true, false, R, Int),
    (Format::R32_SFLOAT, 32, 0, 0, 0, 0, 0, true, false, R, Float),

    // Vulkan Version 1.0.
    (Format::R32G32_UINT, 32, 32, 0, 0, 0, 0, false, false, RG, Int),
    (Format::R32G32_SINT, 32, 32, 0, 0, 0, 0, true, false, RG, Int),
    (Format::R32G32_SFLOAT, 32, 32, 0, 0, 0, 0, true, false, RG, Float),

    // Vulkan Version 1.0.
    (Format::R32G32B32_UINT, 32, 32, 32, 0, 0, 0, false, false, RGB, Int),
    (Format::R32G32B32_SINT, 32, 32, 32, 0, 0, 0, true, false, RGB, Int),
    (Format::R32G32B32_SFLOAT, 32, 32, 32, 0, 0, 0, true, false, RGB, Float),

    // Vulkan Version 1.0.
    (Format::R32G32B32A32_UINT, 32, 32, 32, 32, 0, 0, false, false, RGBA, Int),
    (Format::R32G32B32A32_SINT, 32, 32, 32, 32, 0, 0, true, false, RGBA, Int),
    (Format::R32G32B32A32_SFLOAT, 32, 32, 32, 32, 0, 0, true, false, RGBA, Float),

    // Vulkan Version 1.0.
    (Format::R64_UINT, 64, 0, 0, 0, 0, 0, false, false, R, Int),
    (Format::R64_SINT, 64, 0, 0, 0, 0, 0, true, false, R, Int),
    (Format::R64_SFLOAT, 64, 0, 0, 0, 0, 0, true, false, R, Float),

    // Vulkan Version 1.0.
    (Format::R64G64_UINT, 64, 64, 0, 0, 0, 0, false, false, RG, Int),
    (Format::R64G64_SINT, 64, 64, 0, 0, 0, 0, true, false, RG, Int),
    (Format::R64G64_SFLOAT, 64, 64, 0, 0, 0, 0, true, false, RG, Float),

    // Vulkan Version 1.0.
    (Format::R64G64B64_UINT, 64, 64, 64, 0, 0, 0, false, false, RGB, Int),
    (Format::R64G64B64_SINT, 64, 64, 64, 0, 0, 0, true, false, RGB, Int),
    (Format::R64G64B64_SFLOAT, 64, 64, 64, 0, 0, 0, true, false, RGB, Float),

    // Vulkan Version 1.0.
    (Format::R64G64B64A64_UINT, 64, 64, 64, 64, 0, 0, false, false, RGBA, Int),
    (Format::R64G64B64A64_SINT, 64, 64, 64, 64, 0, 0, true, false, RGBA, Int),
    (Format::R64G64B64A64_SFLOAT, 64, 64, 64, 64, 0, 0, true, false, RGBA, Float),

    // Vulkan Version 1.0.
    (Format::B10G11R11_UFLOAT_PACK32, 11, 11, 10, 0, 0, 0, false, true, BGR, Float),

    // Vulkan Version 1.0.
    (Format::D16_UNORM, 0, 0, 0, 0, 16, 0, false, false, D, Norm),
    (Format::D32_SFLOAT, 0, 0, 0, 0, 32, 0, true, false, D, Float),

    // Vulkan Version 1.1.
    (Format::R10X6_UNORM_PACK16, 10, 0, 0, 0, 0, 6, false, true, RX, Norm),
    (Format::R10X6G10X6_UNORM_2PACK16, 10, 10, 0, 0, 0, 12, false, true, RXGX, Norm),
    (Format::R10X6G10X6B10X6A10X6_UNORM_4PACK16,
        10, 10, 10, 10, 0, 24,
        false,
        true,
        RXGXBXAX,
        Norm,
    ),

    // Vulkan Version 1.1.
    (Format::R12X4_UNORM_PACK16, 12, 0, 0, 0, 0, 4, false, true, RX, Norm),
    (Format::R12X4G12X4_UNORM_2PACK16, 12, 12, 0, 0, 0, 8, false, true, RXGX, Norm),
    (
        Format::R12X4G12X4B12X4A12X4_UNORM_4PACK16,
        12, 12, 12, 12, 0, 16,
        false,
        true,
        RXGXBXAX,
        Norm,
    ),
];

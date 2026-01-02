use ash::{vk, Entry, Instance};

use std::ffi::{self, CString};

const VALIDATION_LAYER_NAMES: &[&str] = &["VK_LAYER_KHRONOS_validation"];

pub struct Renderer {
    instance: Instance,

    #[cfg(debug_assertions)]
    debug_utils_messenger: vk::DebugUtilsMessengerEXT,
}

impl Renderer {
    pub fn new(name: &str) -> Result<Renderer, RendererSetupError> {
        let entry = Entry::linked();

        let app_name = CString::new(name)?;

        // This unwrap is safe as we know the string passed to it does not contain a 0 char
        let engine_name = CString::new("Vulkan Engine")
            .expect("Hard coded string should not contain any 0 chars except at end");

        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 3, 0));

        let (_layer_names, layer_name_pointers) = Self::get_setup_layer_names();

        let available_layers = match unsafe { entry.enumerate_instance_layer_properties() } {
            Ok(layers) => layers,
            Err(error) => return Err(RendererSetupError::from(error)),
        };

        let create_info = vk::InstanceCreateInfo::default()
            .flags(vk::InstanceCreateFlags::empty())
            .application_info(&app_info)
            .enabled_layer_names(&layer_name_pointers)
            .enabled_extension_names(&[]) //TODO Implement extensions
            //TODO setup temporary debug utils messenger
            ;



        let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
        //TODO instance creation error handling
        //TODO implement cleanup for instance

        Ok(Renderer {
            instance
        })
    }

//TODO setup debug messenger

    fn get_setup_layer_names() -> (Vec<CString>, Vec<*const i8>) {

        let mut layers = vec![];

        if cfg!(debug_assertions) {
            for layer in VALIDATION_LAYER_NAMES {
                layers.push(*layer);
            }
        }

        let layer_names : Vec<_> = layers
            .iter()
            .map(|name| CString::new(*name)
                .expect("Hard coded layer names should not contain any 0 chars"))
            .collect();

        let layer_pointers = layer_names
            .iter()
            .map(|name| name.as_ptr())
            .collect();

        (layer_names, layer_pointers)
    }
}

pub enum RendererSetupError {
    StringContainingNullCharError(ffi::NulError),
    LogicError(String),
    UnknownError,
    ExpectedResourceNotReadyError,
    OperationTimedOutError,
    IncompleteResultReturnedError,
    HostMemoryExceededError,
    DeviceMemoryExceededError,
    ObjectInitialisationFailedError,
    LogicalDeviceLostError,
    MemoryMapFailedError,
    LayerSpecifiedNotPresentError,
    ExtensionSpecifiedNotPresentError,
    FeatureSpecifiedNotPresentError,
    IncompatibleDriverError,
    TooManyObjectError,
    FormatSpecifiedNotSupportedError,
    PoolMemoryFragmentationError,
    KhrSurfaceLostError,
    KhrNativeWindowInUseError,
    KhrSwapchainOutOfDateError,
    KhrIncompatibleDisplayForSwapchainError,
    ExtDebugReportValidationFailedError,
    NvGlslShaderInvalidError,
    KhrVideoQueueImageNotSupportedError,
    KhrVideoQueuePictureLayoutNotSupportedError,
    KhrVideoQueueProfileOperationNotSupportedError,
    KhrVideoQueueProfileFormatNotSupportedError,
    KhrVideoQueueProfileCodecNotSupportedError,
    KhrVideoQueueStdVersionNotSupportedError,
    PoolMemoryExceededError,
    MemoryInvalidExternalHandleError,
    ExtImageDrmFormatModifierInvalidLayoutError,
    DescriptorFragmentationError,
    GlobalPriorityNotPermittedError,
    BufferDeviceAddressInvalidError,
    ExtFullScreenExclusiveModeLostError,
    PipelineCompileRequiredError,
KhrVideoStdParametersInvalidError,
    ExtImageCompressionExhaustedError,
    ExtIncompatibleShaderBinaryError,
    UnknownVkResult(vk::Result),
}

impl From<ffi::NulError> for RendererSetupError {
    fn from(error: ffi::NulError) -> Self {
        RendererSetupError::StringContainingNullCharError(error)
    }
}

impl From<vk::Result> for RendererSetupError {
    fn from(error: vk::Result) -> Self {
        match error {
            vk::Result::SUCCESS =>
                RendererSetupError::LogicError(
                    "Success treated as failure!".to_string()),
            vk::Result::NOT_READY => RendererSetupError::ExpectedResourceNotReadyError,
            vk::Result::TIMEOUT => RendererSetupError::OperationTimedOutError,
            vk::Result::EVENT_SET =>
                RendererSetupError::LogicError(
                    "Event being set treated as failure!".to_string()),
            vk::Result::EVENT_RESET =>
                RendererSetupError::LogicError(
                    "Event being reset treated as failure!".to_string()),
            vk::Result::INCOMPLETE => RendererSetupError::IncompleteResultReturnedError,
            vk::Result::ERROR_OUT_OF_HOST_MEMORY => RendererSetupError::HostMemoryExceededError,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => RendererSetupError::DeviceMemoryExceededError,
            vk::Result::ERROR_INITIALIZATION_FAILED =>
                RendererSetupError::ObjectInitialisationFailedError,
            vk::Result::ERROR_DEVICE_LOST => RendererSetupError::LogicalDeviceLostError,
            vk::Result::ERROR_MEMORY_MAP_FAILED => RendererSetupError::MemoryMapFailedError,
            vk::Result::ERROR_LAYER_NOT_PRESENT =>
                RendererSetupError::LayerSpecifiedNotPresentError,
            vk::Result::ERROR_EXTENSION_NOT_PRESENT =>
                RendererSetupError::ExtensionSpecifiedNotPresentError,
            vk::Result::ERROR_FEATURE_NOT_PRESENT =>
                RendererSetupError::FeatureSpecifiedNotPresentError,
            vk::Result::ERROR_INCOMPATIBLE_DRIVER => RendererSetupError::IncompatibleDriverError,
            vk::Result::ERROR_TOO_MANY_OBJECTS => RendererSetupError::TooManyObjectError,
            vk::Result::ERROR_FORMAT_NOT_SUPPORTED =>
                RendererSetupError::FormatSpecifiedNotSupportedError,
            vk::Result::ERROR_FRAGMENTED_POOL => RendererSetupError::PoolMemoryFragmentationError,
            vk::Result::ERROR_UNKNOWN => RendererSetupError::UnknownError,

            vk::Result::ERROR_SURFACE_LOST_KHR => RendererSetupError::KhrSurfaceLostError,
            vk::Result::ERROR_NATIVE_WINDOW_IN_USE_KHR =>
                RendererSetupError::KhrNativeWindowInUseError,

            vk::Result::SUBOPTIMAL_KHR =>
                RendererSetupError::LogicError(
                    "Suboptimal KHR treated as failure!".to_string()),
            vk::Result::ERROR_OUT_OF_DATE_KHR => RendererSetupError::KhrSwapchainOutOfDateError,

            vk::Result::ERROR_INCOMPATIBLE_DISPLAY_KHR =>
                RendererSetupError::KhrIncompatibleDisplayForSwapchainError,

            vk::Result::ERROR_VALIDATION_FAILED_EXT =>
                RendererSetupError::ExtDebugReportValidationFailedError,

            vk::Result::ERROR_INVALID_SHADER_NV => RendererSetupError::NvGlslShaderInvalidError,

            vk::Result::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR =>
                RendererSetupError::KhrVideoQueueImageNotSupportedError,
            vk::Result::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR =>
                RendererSetupError::KhrVideoQueuePictureLayoutNotSupportedError,
            vk::Result::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR =>
                RendererSetupError::KhrVideoQueueProfileOperationNotSupportedError,
            vk::Result::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR =>
                RendererSetupError::KhrVideoQueueProfileFormatNotSupportedError,
            vk::Result::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR =>
                RendererSetupError::KhrVideoQueueProfileCodecNotSupportedError,
            vk::Result::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR =>
                RendererSetupError::KhrVideoQueueStdVersionNotSupportedError,

            vk::Result::ERROR_OUT_OF_POOL_MEMORY_KHR =>
                RendererSetupError::PoolMemoryExceededError,

            vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR =>
                RendererSetupError::MemoryInvalidExternalHandleError,

            vk::Result::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT =>
                RendererSetupError::ExtImageDrmFormatModifierInvalidLayoutError,

            vk::Result::ERROR_FRAGMENTATION_EXT =>
                RendererSetupError::DescriptorFragmentationError,

            vk::Result::ERROR_NOT_PERMITTED_EXT =>
                RendererSetupError::GlobalPriorityNotPermittedError,

            vk::Result::ERROR_INVALID_DEVICE_ADDRESS_EXT =>
                RendererSetupError::BufferDeviceAddressInvalidError,

            vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT =>
                RendererSetupError::ExtFullScreenExclusiveModeLostError,

            vk::Result::THREAD_IDLE_KHR =>
                RendererSetupError::LogicError(
                    "Thread idle treated as failure!".to_string()),
            vk::Result::THREAD_DONE_KHR =>
                RendererSetupError::LogicError(
                    "Thread done treated as failure!".to_string()),
            vk::Result::OPERATION_DEFERRED_KHR =>
                RendererSetupError::LogicError(
                    "Operation deferred treated as failure!".to_string()),
            vk::Result::OPERATION_NOT_DEFERRED_KHR =>
                RendererSetupError::LogicError(
                    "Operation not deferred treated as failure!".to_string()),

            vk::Result::PIPELINE_COMPILE_REQUIRED_EXT =>
                RendererSetupError::PipelineCompileRequiredError,

            vk::Result::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR =>
                RendererSetupError::KhrVideoStdParametersInvalidError,

            vk::Result::ERROR_COMPRESSION_EXHAUSTED_EXT =>
                RendererSetupError::ExtImageCompressionExhaustedError,

            vk::Result::INCOMPATIBLE_SHADER_BINARY_EXT =>
                RendererSetupError::ExtIncompatibleShaderBinaryError,

            other_result => RendererSetupError::UnknownVkResult(other_result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_renderer() {
        let _ = Renderer::new("test");
    }
}
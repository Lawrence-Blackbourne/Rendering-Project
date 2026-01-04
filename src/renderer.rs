use ash::{vk, Entry, Instance};
use glfw::{self, Glfw};

use std::{ffi::{self, CString},
          ops::BitOr,
          ptr
};


const VALIDATION_LAYER_NAMES: &[&str] = &["VK_LAYER_KHRONOS_validation"];
const VALIDATION_EXTENSION_NAMES: &[&str] = &["VK_EXT_debug_utils"];

pub struct Renderer {
    _vulkan_entry: Entry,
    _glfw_entry: Glfw,

    instance: Instance,
}

impl Renderer {
    pub fn new(name: &str) -> Result<Renderer, RendererError> {
        // These are used to call functions on.
        let vulkan_entry = Entry::linked();
        let glfw_entry = glfw::init_no_callbacks()?;

        // This is the instance that then holds all the vulkan state.
        let instance = Self::create_instance(name, &vulkan_entry, &glfw_entry)?;
        //TODO instance creation error handling
        //TODO implement cleanup for instance + debug messenger

        Ok(Renderer {
            _vulkan_entry: vulkan_entry,
            _glfw_entry: glfw_entry,
            instance,
        })
    }

    /// Creates the Vulkan Instance
    fn create_instance(app_name: &str, vulkan_entry: &Entry, glfw_entry: &Glfw)
        -> Result<Instance, RendererError> {
        let app_name = CString::new(app_name)?;

        // This unwrap is safe as we know the string passed to it does not contain a 0 char.
        let engine_name = CString::new("Vulkan Engine")
            .expect("Hard coded string should not contain any 0 chars except at end");

        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 3, 0));

        // We need to keep the layer names around for the pointers to have something to point to.
        // Otherwise, we get undefined behavior.
        let (layer_names, layer_name_pointers) = Self::get_setup_layer_names();
        Self::validate_setup_layers_exist(&layer_names, &vulkan_entry)?;
        let (_extension_names, extension_name_pointers) =
            Self::get_setup_extension_names(&glfw_entry)?;

        #[cfg(debug_assertions)]
        let mut debug_messenger_info = Self::get_debug_messenger_info();

        let create_info = vk::InstanceCreateInfo::default()
            .flags(vk::InstanceCreateFlags::empty())
            .application_info(&app_info)
            .enabled_layer_names(&layer_name_pointers)
            .enabled_extension_names(&extension_name_pointers);

        #[cfg(debug_assertions)]
        let create_info = create_info.push_next(&mut debug_messenger_info);

        //TODO setup temporary debug utils messenger

        Ok(unsafe { vulkan_entry.create_instance(&create_info, None) }?)
    }

    /// Gets the names of the layers needed to set up the instance including debugging.
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

    /// Validates that the layers that are passed to it are available to use.
    fn validate_setup_layers_exist(layer_names: &Vec<CString>, entry: &Entry)
        -> Result<(), RendererError> {
        let available_layers = unsafe {
            entry.enumerate_instance_layer_properties()
        }?;

        for layer_name in layer_names {
            let mut layer_exists = false;
            for available_layer in available_layers.iter() {
                if layer_name.as_c_str() == available_layer.layer_name_as_c_str()? {
                    layer_exists = true;
                    break;
                }
            }
            if !layer_exists {
                return Err(RendererError::LayerRequiredNotSupportedError);
            }
        }
        Ok(())
    }

    /// Gets the names of the extensions needed to set up the instance including debugging.
    fn get_setup_extension_names(glfw_entry: &Glfw)
        -> Result<(Vec<CString>, Vec<*const i8>), RendererError> {

        let mut extension_names = vec![];

        //let glfw_extensions = glfw_entry.get_required_instance_extensions();
        let glfw_extensions = match glfw_entry.get_required_instance_extensions() {
            Some(glfw_extensions) => {
                glfw_extensions
            }
            None => return Err(RendererError::GlfwCallFailed("get_required_instance_extensions"
                .to_string()))
        };

        for extension in glfw_extensions {
            extension_names.push(CString::new(extension)?);
        }

        let mut debug_extensions = vec![];

        if cfg!(debug_assertions) {
            for extension in VALIDATION_EXTENSION_NAMES {
                debug_extensions.push(*extension);
            }
        }

        for extension in debug_extensions {
            extension_names.push(CString::new(extension)
                .expect("Hard coded layer names should not contain any 0 chars"));
        }

        let extension_pointers = extension_names
            .iter()
            .map(|name| name.as_ptr())
            .collect();

        Ok((extension_names, extension_pointers))
    }

    /// Sets up the debug messenger extension.
    fn get_debug_messenger_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
        // We want warnings and errors but not verbose diagnostic messages
        let severity_flags = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            .bitor(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR);

        let type_flags = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            .bitor(vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
            .bitor(vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE);

        let debug_messenger_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .flags(vk::DebugUtilsMessengerCreateFlagsEXT::empty())
            .message_severity(severity_flags)
            .message_type(type_flags)
            .pfn_user_callback(Some(Self::debug_messenger_callback_function))
            .user_data(ptr::null_mut());

        debug_messenger_info
    }

    #[unsafe(no_mangle)]
    unsafe extern "system" fn debug_messenger_callback_function(
        _severity_flags: vk::DebugUtilsMessageSeverityFlagsEXT,
        _type_flags: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
        _user_data_pointer: *mut ffi::c_void,
    ) -> u32 {

        let data = match unsafe { callback_data.as_ref() } {
            Some(data) => data,
            None => {
                eprintln!("Callback data reference dangling!");
                return vk::FALSE;
            }
        };

        let message = match unsafe { data.message_as_c_str() } {
            Some(message) => message.to_str().unwrap_or_else(|_| "Message conversion failed!"),
            None => "Callback has no message!",
        };

        eprintln!("Callback message: {}", message);

        vk::FALSE
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) };
        unsafe { glfw::ffi::glfwTerminate() };
    }
}


#[derive(Debug)]
pub enum RendererError {
    StringContainingNullCharError(ffi::NulError),
    CStringDidNotContainTerminatingNullButeError,
    CStringCouleNotBeConvertedToString(ffi::IntoStringError),
    LayerRequiredNotSupportedError,
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

    GlfwEntryAlreadyExists,
    GlfwEntryCreationInternalError,
    GlfwCallFailed(String),
}

impl From<ffi::NulError> for RendererError {
    fn from(error: ffi::NulError) -> Self {
        RendererError::StringContainingNullCharError(error)
    }
}

impl From<ffi::FromBytesUntilNulError> for RendererError {
    fn from(_: ffi::FromBytesUntilNulError) -> Self {
        RendererError::CStringDidNotContainTerminatingNullButeError
    }
}

impl From<ffi::IntoStringError> for RendererError {
    fn from(error: ffi::IntoStringError) -> Self {
        RendererError::CStringCouleNotBeConvertedToString(error)
    }
}

impl From<vk::Result> for RendererError {
    fn from(error: vk::Result) -> Self {
        match error {
            vk::Result::SUCCESS =>
                RendererError::LogicError(
                    "Success treated as failure!".to_string()),
            vk::Result::NOT_READY => RendererError::ExpectedResourceNotReadyError,
            vk::Result::TIMEOUT => RendererError::OperationTimedOutError,
            vk::Result::EVENT_SET =>
                RendererError::LogicError(
                    "Event being set treated as failure!".to_string()),
            vk::Result::EVENT_RESET =>
                RendererError::LogicError(
                    "Event being reset treated as failure!".to_string()),
            vk::Result::INCOMPLETE => RendererError::IncompleteResultReturnedError,
            vk::Result::ERROR_OUT_OF_HOST_MEMORY => RendererError::HostMemoryExceededError,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => RendererError::DeviceMemoryExceededError,
            vk::Result::ERROR_INITIALIZATION_FAILED =>
                RendererError::ObjectInitialisationFailedError,
            vk::Result::ERROR_DEVICE_LOST => RendererError::LogicalDeviceLostError,
            vk::Result::ERROR_MEMORY_MAP_FAILED => RendererError::MemoryMapFailedError,
            vk::Result::ERROR_LAYER_NOT_PRESENT =>
                RendererError::LayerSpecifiedNotPresentError,
            vk::Result::ERROR_EXTENSION_NOT_PRESENT =>
                RendererError::ExtensionSpecifiedNotPresentError,
            vk::Result::ERROR_FEATURE_NOT_PRESENT =>
                RendererError::FeatureSpecifiedNotPresentError,
            vk::Result::ERROR_INCOMPATIBLE_DRIVER => RendererError::IncompatibleDriverError,
            vk::Result::ERROR_TOO_MANY_OBJECTS => RendererError::TooManyObjectError,
            vk::Result::ERROR_FORMAT_NOT_SUPPORTED =>
                RendererError::FormatSpecifiedNotSupportedError,
            vk::Result::ERROR_FRAGMENTED_POOL => RendererError::PoolMemoryFragmentationError,
            vk::Result::ERROR_UNKNOWN => RendererError::UnknownError,

            vk::Result::ERROR_SURFACE_LOST_KHR => RendererError::KhrSurfaceLostError,
            vk::Result::ERROR_NATIVE_WINDOW_IN_USE_KHR =>
                RendererError::KhrNativeWindowInUseError,

            vk::Result::SUBOPTIMAL_KHR =>
                RendererError::LogicError(
                    "Suboptimal KHR treated as failure!".to_string()),
            vk::Result::ERROR_OUT_OF_DATE_KHR => RendererError::KhrSwapchainOutOfDateError,

            vk::Result::ERROR_INCOMPATIBLE_DISPLAY_KHR =>
                RendererError::KhrIncompatibleDisplayForSwapchainError,

            vk::Result::ERROR_VALIDATION_FAILED_EXT =>
                RendererError::ExtDebugReportValidationFailedError,

            vk::Result::ERROR_INVALID_SHADER_NV => RendererError::NvGlslShaderInvalidError,

            vk::Result::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR =>
                RendererError::KhrVideoQueueImageNotSupportedError,
            vk::Result::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR =>
                RendererError::KhrVideoQueuePictureLayoutNotSupportedError,
            vk::Result::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR =>
                RendererError::KhrVideoQueueProfileOperationNotSupportedError,
            vk::Result::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR =>
                RendererError::KhrVideoQueueProfileFormatNotSupportedError,
            vk::Result::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR =>
                RendererError::KhrVideoQueueProfileCodecNotSupportedError,
            vk::Result::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR =>
                RendererError::KhrVideoQueueStdVersionNotSupportedError,

            vk::Result::ERROR_OUT_OF_POOL_MEMORY_KHR =>
                RendererError::PoolMemoryExceededError,

            vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR =>
                RendererError::MemoryInvalidExternalHandleError,

            vk::Result::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT =>
                RendererError::ExtImageDrmFormatModifierInvalidLayoutError,

            vk::Result::ERROR_FRAGMENTATION_EXT =>
                RendererError::DescriptorFragmentationError,

            vk::Result::ERROR_NOT_PERMITTED_EXT =>
                RendererError::GlobalPriorityNotPermittedError,

            vk::Result::ERROR_INVALID_DEVICE_ADDRESS_EXT =>
                RendererError::BufferDeviceAddressInvalidError,

            vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT =>
                RendererError::ExtFullScreenExclusiveModeLostError,

            vk::Result::THREAD_IDLE_KHR =>
                RendererError::LogicError(
                    "Thread idle treated as failure!".to_string()),
            vk::Result::THREAD_DONE_KHR =>
                RendererError::LogicError(
                    "Thread done treated as failure!".to_string()),
            vk::Result::OPERATION_DEFERRED_KHR =>
                RendererError::LogicError(
                    "Operation deferred treated as failure!".to_string()),
            vk::Result::OPERATION_NOT_DEFERRED_KHR =>
                RendererError::LogicError(
                    "Operation not deferred treated as failure!".to_string()),

            vk::Result::PIPELINE_COMPILE_REQUIRED_EXT =>
                RendererError::PipelineCompileRequiredError,

            vk::Result::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR =>
                RendererError::KhrVideoStdParametersInvalidError,

            vk::Result::ERROR_COMPRESSION_EXHAUSTED_EXT =>
                RendererError::ExtImageCompressionExhaustedError,

            vk::Result::INCOMPATIBLE_SHADER_BINARY_EXT =>
                RendererError::ExtIncompatibleShaderBinaryError,

            other_result => RendererError::UnknownVkResult(other_result),
        }
    }
}

impl From<glfw::InitError> for RendererError {
    fn from(error: glfw::InitError) -> Self {
        match error {
            glfw::InitError::AlreadyInitialized => RendererError::GlfwEntryAlreadyExists,
            glfw::InitError::Internal => RendererError::GlfwEntryCreationInternalError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_renderer() {
        Renderer::new("test").unwrap();
    }
}
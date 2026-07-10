pub(crate) mod debugger;
mod device_handler;
mod swapchain_handler;
mod window_handler;

#[cfg(debug_assertions)]
use ash::ext;
use ash::{Device, Entry, Instance, khr, vk};
use glfw::{self, Glfw};
use std::ffi::NulError;

use crate::string_handler::{convert_to_cstring, string_vector_to_char_vector};

pub use device_handler::device_info_handler;

pub struct Renderer {
    _vulkan_entry: Entry,

    vulkan_instance: Instance,
    glfw_instance: Glfw,

    window: glfw::PWindow,
    surface_instance: khr::surface::Instance,
    surface: vk::SurfaceKHR,

    device: Device,
    _queues: Vec<vk::Queue>,

    #[cfg(debug_assertions)]
    debug_instance: ext::debug_utils::Instance,
    #[cfg(debug_assertions)]
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl Renderer {
    /// Creates a new Renderer instance to use to render images.
    pub fn new(
        name: &str,
        settings: device_info_handler::LogicalDeviceSettings,
    ) -> Result<Renderer, RendererError> {
        // These are used to call functions on.
        let vulkan_entry = Entry::linked();

        // This is an instance that holds all the glfw state.
        let mut glfw_instance = glfw::init_no_callbacks()?;

        // This is the instance that then holds all the vulkan state.
        let mut vulkan_instance =
            Self::create_vulkan_instance(name, &vulkan_entry, &glfw_instance)?;

        // This provides the permanent debug info.
        #[cfg(debug_assertions)]
        let (debug_instance, debug_messenger) =
            debugger::get_debug_messenger(&vulkan_entry, &vulkan_instance)?;

        // This creates the window to render to.
        let window = window_handler::create_window(name, &mut glfw_instance)?;
        let surface = window_handler::create_window_surface(&mut vulkan_instance, &window)?;
        let surface_instance = khr::surface::Instance::new(&vulkan_entry, &vulkan_instance);

        // This gets the Vulkan device to use.
        let (device, queues) =
            device_handler::get_device(&vulkan_instance, &surface_instance, surface, settings)?;

        Ok(Renderer {
            _vulkan_entry: vulkan_entry,
            vulkan_instance,
            glfw_instance,

            window,
            surface_instance,
            surface,

            device,
            _queues: queues,

            #[cfg(debug_assertions)]
            debug_instance,
            #[cfg(debug_assertions)]
            debug_messenger,
        })
    }

    pub fn update(&mut self) -> RendererStatus {
        self.glfw_instance.poll_events();
        if self.window.should_close() {
            RendererStatus::ShouldClose
        } else {
            RendererStatus::Ok
        }
    }

    /// Creates the Vulkan Instance.
    fn create_vulkan_instance(
        app_name: &str,
        vulkan_entry: &Entry,
        glfw_instance: &Glfw,
    ) -> Result<Instance, RendererError> {
        let app_name = convert_to_cstring(app_name)?;

        let engine_name = convert_to_cstring("Vulkan Engine")?;

        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name.as_c_str())
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(vk::make_api_version(0, 1, 3, 0));

        let debug_extension_names = debugger::get_setup_extension_names(glfw_instance)?;
        let layer_names = debugger::get_setup_layer_names();
        debugger::validate_setup_layers_exist(&layer_names, vulkan_entry)?;
        let debug_extension_pointers = string_vector_to_char_vector(&debug_extension_names)?;
        let layer_pointers = string_vector_to_char_vector(&layer_names)?;

        #[cfg(debug_assertions)]
        let mut debug_messenger_info = debugger::get_debug_messenger_info();

        let create_info = vk::InstanceCreateInfo::default()
            .flags(vk::InstanceCreateFlags::empty())
            .application_info(&app_info)
            .enabled_layer_names(&layer_pointers.chars)
            .enabled_extension_names(&debug_extension_pointers.chars);

        // This provides the temporary debug info
        #[cfg(debug_assertions)]
        let create_info = create_info.push_next(&mut debug_messenger_info);

        Ok(unsafe { vulkan_entry.create_instance(&create_info, None) }?)
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        unsafe {
            self.device.destroy_device(None)
        };

        unsafe { self.surface_instance.destroy_surface(self.surface, None) }

        #[cfg(debug_assertions)]
        unsafe {
            self.debug_instance
                .destroy_debug_utils_messenger(self.debug_messenger, None)
        };

        unsafe { self.vulkan_instance.destroy_instance(None) };
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Size {
    pub x: u32,
    pub y: u32,
}

impl From<vk::Extent2D> for Size {
    fn from(value: vk::Extent2D) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RendererStatus {
    Ok,
    ShouldClose,
    Error(RendererError),
}

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RendererError {
    StringContainingNullCharError(NulError),
    CStringDidNotContainTerminatingNullButeError,
    //CStringCouleNotBeConvertedToString(ffi::IntoStringError),
    LayerRequiredNotSupportedError,
    UnableToFindSuitablePhysicalDeviceError,
    TooManyQueuesAvailableToHandleError,
    ImageCountNotAvailableError(u8, u8), // Stores the min and max available image count

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

impl From<vk::Result> for RendererError {
    fn from(error: vk::Result) -> Self {
        match error {
            vk::Result::SUCCESS => {
                RendererError::LogicError("Success treated as failure!".to_string())
            }
            vk::Result::NOT_READY => RendererError::ExpectedResourceNotReadyError,
            vk::Result::TIMEOUT => RendererError::OperationTimedOutError,
            vk::Result::EVENT_SET => {
                RendererError::LogicError("Event being set treated as failure!".to_string())
            }
            vk::Result::EVENT_RESET => {
                RendererError::LogicError("Event being reset treated as failure!".to_string())
            }
            vk::Result::INCOMPLETE => RendererError::IncompleteResultReturnedError,
            vk::Result::ERROR_OUT_OF_HOST_MEMORY => RendererError::HostMemoryExceededError,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => RendererError::DeviceMemoryExceededError,
            vk::Result::ERROR_INITIALIZATION_FAILED => {
                RendererError::ObjectInitialisationFailedError
            }
            vk::Result::ERROR_DEVICE_LOST => RendererError::LogicalDeviceLostError,
            vk::Result::ERROR_MEMORY_MAP_FAILED => RendererError::MemoryMapFailedError,
            vk::Result::ERROR_LAYER_NOT_PRESENT => RendererError::LayerSpecifiedNotPresentError,
            vk::Result::ERROR_EXTENSION_NOT_PRESENT => {
                RendererError::ExtensionSpecifiedNotPresentError
            }
            vk::Result::ERROR_FEATURE_NOT_PRESENT => RendererError::FeatureSpecifiedNotPresentError,
            vk::Result::ERROR_INCOMPATIBLE_DRIVER => RendererError::IncompatibleDriverError,
            vk::Result::ERROR_TOO_MANY_OBJECTS => RendererError::TooManyObjectError,
            vk::Result::ERROR_FORMAT_NOT_SUPPORTED => {
                RendererError::FormatSpecifiedNotSupportedError
            }
            vk::Result::ERROR_FRAGMENTED_POOL => RendererError::PoolMemoryFragmentationError,
            vk::Result::ERROR_UNKNOWN => RendererError::UnknownError,

            vk::Result::ERROR_SURFACE_LOST_KHR => RendererError::KhrSurfaceLostError,
            vk::Result::ERROR_NATIVE_WINDOW_IN_USE_KHR => RendererError::KhrNativeWindowInUseError,

            vk::Result::SUBOPTIMAL_KHR => {
                RendererError::LogicError("Suboptimal KHR treated as failure!".to_string())
            }
            vk::Result::ERROR_OUT_OF_DATE_KHR => RendererError::KhrSwapchainOutOfDateError,

            vk::Result::ERROR_INCOMPATIBLE_DISPLAY_KHR => {
                RendererError::KhrIncompatibleDisplayForSwapchainError
            }

            vk::Result::ERROR_VALIDATION_FAILED_EXT => {
                RendererError::ExtDebugReportValidationFailedError
            }

            vk::Result::ERROR_INVALID_SHADER_NV => RendererError::NvGlslShaderInvalidError,

            vk::Result::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR => {
                RendererError::KhrVideoQueueImageNotSupportedError
            }
            vk::Result::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR => {
                RendererError::KhrVideoQueuePictureLayoutNotSupportedError
            }
            vk::Result::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR => {
                RendererError::KhrVideoQueueProfileOperationNotSupportedError
            }
            vk::Result::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR => {
                RendererError::KhrVideoQueueProfileFormatNotSupportedError
            }
            vk::Result::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR => {
                RendererError::KhrVideoQueueProfileCodecNotSupportedError
            }
            vk::Result::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR => {
                RendererError::KhrVideoQueueStdVersionNotSupportedError
            }

            vk::Result::ERROR_OUT_OF_POOL_MEMORY_KHR => RendererError::PoolMemoryExceededError,

            vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR => {
                RendererError::MemoryInvalidExternalHandleError
            }

            vk::Result::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
                RendererError::ExtImageDrmFormatModifierInvalidLayoutError
            }

            vk::Result::ERROR_FRAGMENTATION_EXT => RendererError::DescriptorFragmentationError,

            vk::Result::ERROR_NOT_PERMITTED_EXT => RendererError::GlobalPriorityNotPermittedError,

            vk::Result::ERROR_INVALID_DEVICE_ADDRESS_EXT => {
                RendererError::BufferDeviceAddressInvalidError
            }

            vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
                RendererError::ExtFullScreenExclusiveModeLostError
            }

            vk::Result::THREAD_IDLE_KHR => {
                RendererError::LogicError("Thread idle treated as failure!".to_string())
            }
            vk::Result::THREAD_DONE_KHR => {
                RendererError::LogicError("Thread done treated as failure!".to_string())
            }
            vk::Result::OPERATION_DEFERRED_KHR => {
                RendererError::LogicError("Operation deferred treated as failure!".to_string())
            }
            vk::Result::OPERATION_NOT_DEFERRED_KHR => {
                RendererError::LogicError("Operation not deferred treated as failure!".to_string())
            }

            vk::Result::PIPELINE_COMPILE_REQUIRED_EXT => {
                RendererError::PipelineCompileRequiredError
            }

            vk::Result::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR => {
                RendererError::KhrVideoStdParametersInvalidError
            }

            vk::Result::ERROR_COMPRESSION_EXHAUSTED_EXT => {
                RendererError::ExtImageCompressionExhaustedError
            }

            vk::Result::INCOMPATIBLE_SHADER_BINARY_EXT => {
                RendererError::ExtIncompatibleShaderBinaryError
            }

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
    use debugger::tests;

    //TODO fix
    #[test]
    fn can_create_renderer() {
        let (_guard, _, _) = tests::get_entries();
        //Renderer::new("test", 2).unwrap();
    }

    #[test]
    fn can_destroy_renderer() {
        let (_guard, _, _) = tests::get_entries();
        //let renderer = Renderer::new("test", 2).unwrap();
        //drop(renderer);
    }

    #[test]
    fn can_do_empty_frame_update() {
        let (_guard, _, _) = tests::get_entries();
        //let mut renderer = Renderer::new("test", 2).unwrap();
        //let result = renderer.update();
        //assert_eq!(result, RendererStatus::Ok)
    }
}

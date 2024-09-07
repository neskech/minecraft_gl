use std::{
    collections::HashSet,
    ffi::{c_void, CStr},
};

use anyhow::{anyhow, Result};
use log::*;
use vk::ExtDebugUtilsExtension;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    window::get_required_instance_extensions,
    Entry, Instance, Version,
};

use winit::window::Window;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

/**
 * A handle onto the Vulkan API.
 * Intializes the API and allows us
 * to utilize it
 */
pub struct VulkanHandle {
    /* Entry point onto vulkan */
    entry: Entry,
    /* An instance of vulkan */
    instance: Instance,
    /**
     * A handle to the debugging callback,
     * used to log debug information from the
     * validation layers. Only enabled if compiled
     * in debug mode
     */
    messengerHandle: Option<vk::DebugUtilsMessengerEXT>,
}

impl VulkanHandle {
    pub fn Create(window: &Window) -> Result<VulkanHandle> {
        unsafe {
            let entry = Self::MakeEntry()?;
            let instance = Self::MakeInstance(&entry, window)?;
            let messengerHandle = Self::GetMessengerHandle(&instance)?;
            Ok(VulkanHandle {
                entry,
                instance,
                messengerHandle,
            })
        }
    }

    unsafe fn MakeEntry() -> Result<Entry> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        Ok(entry)
    }

    unsafe fn MakeInstance(entry: &Entry, window: &Window) -> Result<Instance> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Minecraft VK\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(b"No Engine\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 0, 0));

        let (extensions, flags) = Self::GetVulkanExtensionsAndFlags(window, entry)?;
        let validationLayers = Self::GetValidationLayers(entry)?;
        let mut instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&validationLayers)
            .flags(flags);

        /**
         * Add debugging callback for the creation and destruction of the
         * vulkan instance. This will NOT handle callbacks for any other part
         * of the application
         */
        let mut debugInfo = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .user_callback(Some(debugCallback));
        if VALIDATION_ENABLED {
            instance_info = instance_info.push_next(&mut debugInfo);
        }

        let instance = entry.create_instance(&instance_info, None)?;
        Ok(instance)
    }

    /**
     * Vulkan extensions tell the driver which global extensions and
     * validation layers we want to use
     */
    fn GetVulkanExtensionsAndFlags(
        window: &Window,
        entry: &Entry,
    ) -> Result<(Vec<*const i8>, vk::InstanceCreateFlags)> {
        /* Note: The extensions are c strings */
        let mut extensions = get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();
        /**
         * Compability flags for MacOS. Required by Vulkan
         * SDK on macOS since 1.3.216
         */
        let isMacOs = cfg!(target_os = "macos");
        let portabilityMacOsVersion = Version::new(1, 3, 216);
        let hasMinimumVersion = entry.version()? >= portabilityMacOsVersion;

        let instance_flags = if isMacOs && hasMinimumVersion {
            info!("Enabling extensions for macOS portability.");
            extensions.push(
                vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION
                    .name
                    .as_ptr(),
            );
            extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };

        Ok((extensions, instance_flags))
    }

    /**
     * Validation layers are configuration we can add onto vulkan
     * to provide debug information as our application runs
     */
    unsafe fn GetValidationLayers(entry: &Entry) -> Result<Vec<*const i8>> {
        let available_layers = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();

        if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
            return Err(anyhow!("Validation layer requested but not supported."));
        }

        if VALIDATION_ENABLED {
            return Ok(vec![VALIDATION_LAYER.as_ptr()]);
        }

        Ok(Vec::new())
    }

    /**
     * Configures debugging callback which will be used to log
     * debugging info throughout the lifetime of the vulkan
     * application
     */
    unsafe fn GetMessengerHandle(
        instance: &Instance,
    ) -> Result<Option<vk::DebugUtilsMessengerEXT>> {
        if !VALIDATION_ENABLED {
            return Ok(None);
        }

        let debugInfo = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .user_callback(Some(debugCallback));
        let messenger = instance.create_debug_utils_messenger_ext(&debugInfo, None)?;
        Ok(Some(messenger))
    }
}

impl Drop for VulkanHandle {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);

            if VALIDATION_ENABLED {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.messengerHandle.unwrap(), None)
            }
        }
    }
}

/**
 * Function for vulkan to call with its validation layers.
 * Debug information will be sent to this function and logged
 * out with varying severity levels.
 */
extern "system" fn debugCallback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("({:?}) {}", type_, message);
    } else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}

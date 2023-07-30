use std::{
    borrow::Cow,
    ffi::{c_char, c_void, CStr, CString},
};

use ash::{
    extensions::ext::DebugUtils,
    prelude::*,
    vk::{
        self, Bool32, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerEXT,
    },
    Entry,
};

pub struct UsamiInstance {
    pub vk_entry: Entry,
    pub vk_instance: ash::Instance,
    pub vk_debug_utils_loader: DebugUtils,
    pub vk_messenger: DebugUtilsMessengerEXT,
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}

impl UsamiInstance {
    pub fn new(
        app_name: &str,
        engine_name: &str,
        api_version: u32,
        extensions: &[String],
    ) -> VkResult<Self> {
        let app_name = CString::new(app_name).expect("cannot create CString for application name");
        let engine_name = CString::new(engine_name).expect("cannot create CString for engine name");
        let application_info = vk::ApplicationInfo::builder()
            .application_name(app_name.as_c_str())
            .application_version(0)
            .engine_name(engine_name.as_c_str())
            .engine_version(0)
            .api_version(api_version);
        let extensions_cstring: Vec<CString> = extensions
            .iter()
            .map(|name| CString::new(name.as_str()).unwrap())
            .collect();

        let extensions_raw: Vec<*const c_char> = extensions_cstring
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let validation_layer = CString::new("VK_LAYER_KHRONOS_validation").unwrap();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions_raw)
            .enabled_layer_names(&[validation_layer.as_ptr()])
            .build();

        let vk_entry = unsafe { Entry::load().expect("Cannot load VK library") };
        let vk_instance = unsafe { vk_entry.create_instance(&create_info, None)? };

        let vk_debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback))
            .build();

        let vk_debug_utils_loader = DebugUtils::new(&vk_entry, &vk_instance);
        let vk_messenger =
            unsafe { vk_debug_utils_loader.create_debug_utils_messenger(&vk_debug_info, None)? };

        Ok(Self {
            vk_entry,
            vk_instance,
            vk_debug_utils_loader,
            vk_messenger,
        })
    }
}

impl Drop for UsamiInstance {
    fn drop(&mut self) {
        unsafe {
            self.vk_debug_utils_loader
                .destroy_debug_utils_messenger(self.vk_messenger, None);
            self.vk_instance.destroy_instance(None)
        }
    }
}

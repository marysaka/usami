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
    pub vk_version: u32,
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
        enable_validation: bool,
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
        let validation_layers_slice = &[validation_layer.as_ptr()];

        let mut create_info_builder = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions_raw);

        if enable_validation {
            create_info_builder = create_info_builder.enabled_layer_names(validation_layers_slice);
        }
        let create_info = create_info_builder.build();

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
            vk_version: api_version,
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

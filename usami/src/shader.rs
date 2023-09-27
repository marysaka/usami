use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{Handle, ObjectType, ShaderModule, ShaderModuleCreateFlags, ShaderModuleCreateInfo},
};

use crate::UsamiDevice;

pub struct UsamiShader {
    device: Arc<UsamiDevice>,
    pub handle: ShaderModule,
}

impl UsamiShader {
    pub fn new(device: &Arc<UsamiDevice>, code: &[u32]) -> VkResult<Self> {
        let create_info = ShaderModuleCreateInfo::builder()
            .code(code)
            .flags(ShaderModuleCreateFlags::empty())
            .build();

        let handle = unsafe { device.handle.create_shader_module(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiShader {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_shader_module(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_shader(
        device: &Arc<UsamiDevice>,
        name: String,
        code: &[u32],
    ) -> VkResult<UsamiShader> {
        let shader = UsamiShader::new(device, code)?;

        device.set_debug_name(name, shader.handle.as_raw(), ObjectType::SHADER_MODULE)?;

        Ok(shader)
    }
}

use ash::{
    prelude::*,
    vk::{Handle, ObjectType, ShaderModule, ShaderModuleCreateFlags, ShaderModuleCreateInfo},
    Device,
};

use crate::UsamiDevice;

pub struct UsamiShader {
    device: Device,
    pub handle: ShaderModule,
}

impl UsamiShader {
    pub fn new(device: &UsamiDevice, code: &[u32]) -> VkResult<Self> {
        let create_info = ShaderModuleCreateInfo::builder()
            .code(code)
            .flags(ShaderModuleCreateFlags::empty())
            .build();

        let handle = unsafe { device.vk_device.create_shader_module(&create_info, None)? };

        Ok(Self {
            device: device.vk_device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiShader {
    fn drop(&mut self) {
        unsafe { self.device.destroy_shader_module(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_shader(&self, name: String, code: &[u32]) -> VkResult<UsamiShader> {
        let shader = UsamiShader::new(self, code)?;

        self.set_debug_name(name, shader.handle.as_raw(), ObjectType::SHADER_MODULE)?;

        Ok(shader)
    }
}

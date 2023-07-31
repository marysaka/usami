use ash::{
    prelude::*,
    vk::{Handle, ObjectType, RenderPass, RenderPassCreateInfo},
    Device,
};

use crate::UsamiDevice;

pub struct UsamiRenderPass {
    device: Device,
    pub handle: RenderPass,
}

impl UsamiRenderPass {
    pub fn new(device: &Device, create_info: RenderPassCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.create_render_pass(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiRenderPass {
    fn drop(&mut self) {
        unsafe { self.device.destroy_render_pass(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_render_pass(
        &self,
        name: String,
        create_info: RenderPassCreateInfo,
    ) -> VkResult<UsamiRenderPass> {
        let shader = UsamiRenderPass::new(&self.handle, create_info)?;

        self.set_debug_name(name, shader.handle.as_raw(), ObjectType::RENDER_PASS)?;

        Ok(shader)
    }
}

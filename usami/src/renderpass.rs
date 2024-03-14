use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{RenderPass, RenderPassCreateInfo},
};

use crate::UsamiDevice;

pub struct UsamiRenderPass {
    device: Arc<UsamiDevice>,
    pub handle: RenderPass,
}

impl UsamiRenderPass {
    pub fn new(device: &Arc<UsamiDevice>, create_info: RenderPassCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.handle.create_render_pass(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiRenderPass {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_render_pass(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_render_pass(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: RenderPassCreateInfo,
    ) -> VkResult<UsamiRenderPass> {
        let shader = UsamiRenderPass::new(device, create_info)?;

        device.set_debug_name(name, shader.handle)?;

        Ok(shader)
    }
}

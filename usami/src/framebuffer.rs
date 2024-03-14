use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{Framebuffer, FramebufferCreateInfo},
};

use crate::UsamiDevice;

pub struct UsamiFramebuffer {
    device: Arc<UsamiDevice>,
    pub handle: Framebuffer,
}

impl UsamiFramebuffer {
    pub fn new(device: &Arc<UsamiDevice>, create_info: FramebufferCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.handle.create_framebuffer(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiFramebuffer {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_framebuffer(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_framebuffer(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: FramebufferCreateInfo,
    ) -> VkResult<UsamiFramebuffer> {
        let framebuffer = UsamiFramebuffer::new(device, create_info)?;

        device.set_debug_name(name, framebuffer.handle)?;

        Ok(framebuffer)
    }
}

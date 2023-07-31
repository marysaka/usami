use ash::{
    prelude::*,
    vk::{Framebuffer, FramebufferCreateInfo, Handle, ObjectType},
    Device,
};

use crate::UsamiDevice;

pub struct UsamiFramebuffer {
    device: Device,
    pub create_info: FramebufferCreateInfo,
    pub handle: Framebuffer,
}

impl UsamiFramebuffer {
    pub fn new(device: &Device, create_info: FramebufferCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.create_framebuffer(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            create_info,
            handle,
        })
    }
}

impl Drop for UsamiFramebuffer {
    fn drop(&mut self) {
        unsafe { self.device.destroy_framebuffer(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_framebuffer(
        &self,
        name: String,
        create_info: FramebufferCreateInfo,
    ) -> VkResult<UsamiFramebuffer> {
        let framebuffer = UsamiFramebuffer::new(&self.handle, create_info)?;

        self.set_debug_name(name, framebuffer.handle.as_raw(), ObjectType::FRAMEBUFFER)?;

        Ok(framebuffer)
    }
}

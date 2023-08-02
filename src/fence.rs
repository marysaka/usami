use ash::{
    prelude::*,
    vk::{Fence, FenceCreateFlags, FenceCreateInfo, Handle, ObjectType},
    Device,
};

use crate::UsamiDevice;

pub struct UsamiFence {
    device: Device,
    pub handle: Fence,
}

impl UsamiFence {
    pub fn new(device: &Device, flags: FenceCreateFlags) -> VkResult<Self> {
        let create_info = FenceCreateInfo::builder().flags(flags).build();

        let handle = unsafe { device.create_fence(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }

    pub fn wait(&self, timeout: u64) -> VkResult<()> {
        unsafe { self.device.wait_for_fences(&[self.handle], true, timeout) }
    }

    pub fn reset(&self) -> VkResult<()> {
        unsafe { self.device.reset_fences(&[self.handle]) }
    }
}

impl Drop for UsamiFence {
    fn drop(&mut self) {
        unsafe { self.device.destroy_fence(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_fence(&self, name: String, flags: FenceCreateFlags) -> VkResult<UsamiFence> {
        let pipeline_layout = UsamiFence::new(&self.handle, flags)?;

        self.set_debug_name(name, pipeline_layout.handle.as_raw(), ObjectType::FENCE)?;

        Ok(pipeline_layout)
    }
}

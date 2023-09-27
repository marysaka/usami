use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{Fence, FenceCreateFlags, FenceCreateInfo, Handle, ObjectType},
};

use crate::UsamiDevice;

pub struct UsamiFence {
    device: Arc<UsamiDevice>,
    pub handle: Fence,
}

impl UsamiFence {
    pub fn new(device: &Arc<UsamiDevice>, flags: FenceCreateFlags) -> VkResult<Self> {
        let create_info = FenceCreateInfo::builder().flags(flags).build();

        let handle = unsafe { device.handle.create_fence(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }

    pub fn wait(&self, timeout: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .wait_for_fences(&[self.handle], true, timeout)
        }
    }

    pub fn reset(&self) -> VkResult<()> {
        unsafe { self.device.handle.reset_fences(&[self.handle]) }
    }
}

impl Drop for UsamiFence {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_fence(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_fence(
        device: &Arc<UsamiDevice>,
        name: String,
        flags: FenceCreateFlags,
    ) -> VkResult<UsamiFence> {
        let pipeline_layout = UsamiFence::new(device, flags)?;

        device.set_debug_name(name, pipeline_layout.handle.as_raw(), ObjectType::FENCE)?;

        Ok(pipeline_layout)
    }
}

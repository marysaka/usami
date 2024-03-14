use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{Queue, SubmitInfo},
};

use crate::{UsamiDevice, UsamiFence};

pub struct UsamiQueue {
    device: Arc<UsamiDevice>,
    pub handle: Queue,
}

impl UsamiQueue {
    pub fn new(
        device: &Arc<UsamiDevice>,
        queue_family_index: u32,
        queue_index: u32,
    ) -> VkResult<Self> {
        let handle = unsafe {
            device
                .handle
                .get_device_queue(queue_family_index, queue_index)
        };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }

    pub fn submit(&self, submits: &[SubmitInfo], fence: &UsamiFence) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .queue_submit(self.handle, submits, fence.handle)
        }
    }
}

impl UsamiDevice {
    pub fn get_device_queue(
        device: &Arc<UsamiDevice>,
        name: String,
        queue_family_index: u32,
        queue_index: u32,
    ) -> VkResult<UsamiQueue> {
        let pipeline_layout = UsamiQueue::new(device, queue_family_index, queue_index)?;

        device.set_debug_name(name, pipeline_layout.handle)?;

        Ok(pipeline_layout)
    }
}

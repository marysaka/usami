use ash::{
    prelude::*,
    vk::{Handle, ObjectType, Queue, SubmitInfo},
    Device,
};

use crate::{UsamiDevice, UsamiFence};

pub struct UsamiQueue {
    device: Device,
    pub handle: Queue,
}

impl UsamiQueue {
    pub fn new(device: &Device, queue_family_index: u32, queue_index: u32) -> VkResult<Self> {
        let handle = unsafe { device.get_device_queue(queue_family_index, queue_index) };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }

    pub fn submit(&self, submits: &[SubmitInfo], fence: &UsamiFence) -> VkResult<()> {
        unsafe { self.device.queue_submit(self.handle, submits, fence.handle) }
    }
}

impl UsamiDevice {
    pub fn get_device_queue(
        &self,
        name: String,
        queue_family_index: u32,
        queue_index: u32,
    ) -> VkResult<UsamiQueue> {
        let pipeline_layout = UsamiQueue::new(&self.handle, queue_family_index, queue_index)?;

        self.set_debug_name(name, pipeline_layout.handle.as_raw(), ObjectType::QUEUE)?;

        Ok(pipeline_layout)
    }
}

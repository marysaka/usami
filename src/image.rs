use ash::{
    prelude::*,
    vk::{Image, ImageCreateInfo, MemoryPropertyFlags},
    Device,
};

use crate::{UsamiDevice, UsamiDeviceMemory};

pub struct UsamiImage {
    device: Device,
    pub create_info: ImageCreateInfo,
    pub handle: Image,
    pub device_memory: UsamiDeviceMemory,
}

impl UsamiImage {
    pub fn new(
        device: &UsamiDevice,
        create_info: ImageCreateInfo,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<Self> {
        let vk_device: &Device = &device.vk_device;

        let handle = unsafe { vk_device.create_image(&create_info, None)? };
        let req = unsafe { vk_device.get_image_memory_requirements(handle) };
        let device_memory = UsamiDeviceMemory::new(device, req, memory_flags)?;
        unsafe {
            vk_device.bind_image_memory(handle, device_memory.handle, 0)?;
        }

        Ok(Self {
            device: device.vk_device.clone(),
            create_info,
            handle,
            device_memory,
        })
    }
}

impl Drop for UsamiImage {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image(self.handle, None) }
    }
}

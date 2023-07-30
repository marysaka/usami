use ash::{
    prelude::*,
    vk::{
        Buffer, BufferCreateFlags, BufferCreateInfo, BufferUsageFlags, MemoryPropertyFlags,
        SharingMode,
    },
    Device,
};

use crate::{UsamiDevice, UsamiDeviceMemory};

pub struct UsamiBuffer {
    device: Device,
    pub create_info: BufferCreateInfo,
    pub handle: Buffer,
    pub device_memory: UsamiDeviceMemory,
}

impl UsamiBuffer {
    pub fn new(
        device: &UsamiDevice,
        create_info: BufferCreateInfo,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<Self> {
        let vk_device: &Device = &device.vk_device;

        let handle = unsafe { vk_device.create_buffer(&create_info, None)? };
        let req = unsafe { vk_device.get_buffer_memory_requirements(handle) };
        let device_memory = UsamiDeviceMemory::new(device, req, memory_flags)?;
        unsafe {
            vk_device.bind_buffer_memory(handle, device_memory.handle, 0)?;
        }

        Ok(Self {
            device: device.vk_device.clone(),
            create_info,
            handle,
            device_memory,
        })
    }
}

impl Drop for UsamiBuffer {
    fn drop(&mut self) {
        unsafe { self.device.destroy_buffer(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_buffer<T: Copy>(
        &self,
        _name: String,
        flags: BufferCreateFlags,
        queue_family_indices: &[u32],
        sharing_mode: SharingMode,
        usage: BufferUsageFlags,
        data: &[T],
    ) -> VkResult<UsamiBuffer> {
        let create_info = BufferCreateInfo::builder()
            .flags(flags)
            .queue_family_indices(queue_family_indices)
            .sharing_mode(sharing_mode)
            .usage(usage)
            .size(std::mem::size_of_val(data) as u64)
            .build();
        let buffer = UsamiBuffer::new(self, create_info, MemoryPropertyFlags::HOST_VISIBLE)?;

        unsafe {
            let dst_slice =
                std::slice::from_raw_parts_mut(buffer.device_memory.map()? as *mut T, data.len());

            dst_slice.copy_from_slice(data);

            buffer.device_memory.unmap();
        }

        Ok(buffer)
    }
}

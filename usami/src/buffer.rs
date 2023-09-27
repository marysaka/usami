use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{
        Buffer, BufferCreateFlags, BufferCreateInfo, BufferUsageFlags, Handle, MemoryPropertyFlags,
        ObjectType, SharingMode,
    },
    Device,
};

use crate::{UsamiDevice, UsamiDeviceMemory};

pub struct UsamiBuffer {
    device: Arc<UsamiDevice>,
    pub create_info: BufferCreateInfo,
    pub handle: Buffer,
    pub device_memory: UsamiDeviceMemory,
}

impl UsamiBuffer {
    pub fn new(
        device: &Arc<UsamiDevice>,
        create_info: BufferCreateInfo,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<Self> {
        let vk_device: &Device = &device.handle;

        let handle = unsafe { vk_device.create_buffer(&create_info, None)? };
        let req = unsafe { vk_device.get_buffer_memory_requirements(handle) };
        let device_memory = UsamiDeviceMemory::new(device, req, memory_flags)?;
        unsafe {
            vk_device.bind_buffer_memory(handle, device_memory.handle, 0)?;
        }

        Ok(Self {
            device: device.clone(),
            create_info,
            handle,
            device_memory,
        })
    }

    pub fn copy_from_slice<T: Copy>(&self, data: &[T]) -> VkResult<()> {
        unsafe {
            let dst_slice =
                std::slice::from_raw_parts_mut(self.device_memory.map()? as *mut T, data.len());

            dst_slice.copy_from_slice(data);

            self.device_memory
                .flush(0, std::mem::size_of_val(dst_slice) as u64)?;

            self.device_memory.unmap();
        }

        Ok(())
    }
}

impl Drop for UsamiBuffer {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_buffer(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_buffer<T: Copy>(
        device: &Arc<UsamiDevice>,
        name: String,
        flags: BufferCreateFlags,
        sharing_mode: SharingMode,
        usage: BufferUsageFlags,
        data: &[T],
    ) -> VkResult<UsamiBuffer> {
        let size = std::mem::size_of_val(data) as u64;
        let buffer = Self::create_buffer_with_size(
            device,
            name,
            flags,
            sharing_mode,
            usage,
            size,
            MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        buffer.copy_from_slice(data)?;

        Ok(buffer)
    }

    pub fn create_buffer_with_size(
        device: &Arc<UsamiDevice>,
        name: String,
        flags: BufferCreateFlags,
        sharing_mode: SharingMode,
        usage: BufferUsageFlags,
        size: u64,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<UsamiBuffer> {
        let create_info = BufferCreateInfo::builder()
            .flags(flags)
            .sharing_mode(sharing_mode)
            .usage(usage)
            .size(size)
            .queue_family_indices(&[device.vk_queue_index])
            .build();
        let buffer = UsamiBuffer::new(device, create_info, memory_flags)?;

        device.set_debug_name(name, buffer.handle.as_raw(), ObjectType::BUFFER)?;

        Ok(buffer)
    }
}
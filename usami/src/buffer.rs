use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{
        Buffer, BufferCreateFlags, BufferCreateInfo, BufferUsageFlags, BufferView,
        BufferViewCreateFlags, BufferViewCreateInfo, DeviceSize, Format, MemoryPropertyFlags,
        SharingMode,
    },
    Device,
};

use crate::{UsamiDevice, UsamiDeviceMemory};

pub struct UsamiBuffer {
    device: Arc<UsamiDevice>,
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

    pub fn create_view(
        &self,
        name: String,
        flags: BufferViewCreateFlags,
        format: Format,
        offset: DeviceSize,
        range: DeviceSize,
    ) -> VkResult<UsamiBufferView> {
        UsamiDevice::create_buffer_view(
            &self.device,
            name,
            BufferViewCreateInfo::default()
                .flags(flags)
                .buffer(self.handle)
                .format(format)
                .offset(offset)
                .range(range),
        )
    }
}

impl Drop for UsamiBuffer {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_buffer(self.handle, None) }
    }
}

pub struct UsamiBufferView {
    device: Arc<UsamiDevice>,
    pub handle: BufferView,
}

impl UsamiBufferView {
    pub fn new(device: &Arc<UsamiDevice>, create_info: BufferViewCreateInfo) -> VkResult<Self> {
        let vk_device: &Device = &device.handle;

        let handle = unsafe { vk_device.create_buffer_view(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiBufferView {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_buffer_view(self.handle, None) }
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
        size: DeviceSize,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<UsamiBuffer> {
        let queue_family_indices = &[device.vk_queue_index];
        let create_info = BufferCreateInfo::default()
            .flags(flags)
            .sharing_mode(sharing_mode)
            .usage(usage)
            .size(size)
            .queue_family_indices(queue_family_indices);
        let buffer = UsamiBuffer::new(device, create_info, memory_flags)?;

        device.set_debug_name(name, buffer.handle)?;

        Ok(buffer)
    }

    pub fn create_buffer_view(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: BufferViewCreateInfo,
    ) -> VkResult<UsamiBufferView> {
        let buffer_view = UsamiBufferView::new(device, create_info)?;

        device.set_debug_name(name, buffer_view.handle)?;

        Ok(buffer_view)
    }
}

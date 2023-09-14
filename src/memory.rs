use std::{ffi::c_void, sync::Arc};

use ash::{
    prelude::*,
    vk::{
        self, DeviceMemory, MappedMemoryRange, MemoryAllocateInfo, MemoryMapFlags,
        MemoryPropertyFlags, MemoryRequirements,
    },
};

use crate::UsamiDevice;

pub struct UsamiDeviceMemory {
    device: Arc<UsamiDevice>,
    pub requirements: MemoryRequirements,
    pub allocate_info: MemoryAllocateInfo,
    pub handle: DeviceMemory,
}

impl UsamiDeviceMemory {
    pub fn new(
        device: &Arc<UsamiDevice>,
        requirements: MemoryRequirements,
        flags: MemoryPropertyFlags,
    ) -> VkResult<Self> {
        let allocate_info = MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(device.find_memory_type(&requirements, flags)?)
            .build();

        let handle = unsafe { device.handle.allocate_memory(&allocate_info, None)? };

        Ok(Self {
            device: device.clone(),
            requirements,
            allocate_info,
            handle,
        })
    }

    /// # Safety
    ///
    /// Must be called once.
    pub unsafe fn map(&self) -> VkResult<*mut c_void> {
        self.device.handle.map_memory(
            self.handle,
            0,
            self.allocate_info.allocation_size,
            MemoryMapFlags::empty(),
        )
    }

    /// # Safety
    ///
    /// Must be called after a sucessful [Self::map].
    pub unsafe fn unmap(&self) {
        self.device.handle.unmap_memory(self.handle)
    }

    pub fn flush(&self, offset: u64, size: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .flush_mapped_memory_ranges(&[MappedMemoryRange::builder()
                    .memory(self.handle)
                    .offset(offset)
                    .size(size)
                    .build()])
        }
    }

    pub fn invalidate(&self, offset: u64, size: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .invalidate_mapped_memory_ranges(&[MappedMemoryRange::builder()
                    .memory(self.handle)
                    .offset(offset)
                    .size(size)
                    .build()])
        }
    }

    pub fn read_to_vec(&self) -> VkResult<Vec<u8>> {
        let mut res = Vec::new();

        let allocation_size = self.allocate_info.allocation_size as usize;

        res.resize(allocation_size, 0);

        unsafe {
            let ptr = self.map()?;

            self.invalidate(0, allocation_size as u64)?;

            std::ptr::copy(ptr, res.as_mut_ptr() as *mut _, allocation_size);

            self.unmap();
        }

        Ok(res)
    }
}

impl Drop for UsamiDeviceMemory {
    fn drop(&mut self) {
        unsafe {
            self.device.handle.free_memory(self.handle, None);
        }
    }
}

impl UsamiDevice {
    pub fn find_memory_type(
        &self,
        req: &MemoryRequirements,
        flags: MemoryPropertyFlags,
    ) -> VkResult<u32> {
        self.physical_device.memory_properties.memory_types
            [..self.physical_device.memory_properties.memory_type_count as _]
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (1 << index) & req.memory_type_bits != 0
                    && memory_type.property_flags & flags == flags
            })
            .map(|(index, _memory_type)| index as _)
            .ok_or(vk::Result::ERROR_UNKNOWN) // TODO better error management
    }
}

use ash::{
    prelude::*,
    vk::{
        DescriptorPool, DescriptorPoolCreateInfo, DescriptorSet, DescriptorSetAllocateInfo,
        DescriptorSetLayout, Handle, ObjectType,
    },
    Device,
};

use crate::UsamiDevice;

pub struct UsamiDescriptorPool {
    device: Device,
    pub handle: DescriptorPool,
}

impl UsamiDescriptorPool {
    pub fn new(device: &Device, create_info: DescriptorPoolCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.create_descriptor_pool(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }

    pub fn allocate_descriptor_sets(
        &self,
        device: &UsamiDevice,
        name: String,
        layouts: &[DescriptorSetLayout],
    ) -> VkResult<Vec<UsamiDescriptorSet>> {
        assert_eq!(self.device.handle(), device.handle.handle());

        let command_buffers = UsamiDescriptorSet::new(
            &self.device,
            DescriptorSetAllocateInfo::builder()
                .descriptor_pool(self.handle)
                .set_layouts(layouts)
                .build(),
        )?;

        for (idx, command_buffer) in command_buffers.iter().enumerate() {
            device.set_debug_name(
                format!("{name}_{idx}"),
                command_buffer.handle.as_raw(),
                ObjectType::DESCRIPTOR_SET,
            )?;
        }

        Ok(command_buffers)
    }
}

impl Drop for UsamiDescriptorPool {
    fn drop(&mut self) {
        unsafe { self.device.destroy_descriptor_pool(self.handle, None) }
    }
}

pub struct UsamiDescriptorSet {
    device: Device,
    descriptor_pool: DescriptorPool,
    pub handle: DescriptorSet,
}

impl UsamiDescriptorSet {
    pub fn new(device: &Device, allocate_info: DescriptorSetAllocateInfo) -> VkResult<Vec<Self>> {
        let result = unsafe { device.allocate_descriptor_sets(&allocate_info)? };

        Ok(result
            .iter()
            .map(|handle| Self {
                device: device.clone(),
                descriptor_pool: allocate_info.descriptor_pool,
                handle: *handle,
            })
            .collect())
    }
}

impl Drop for UsamiDescriptorSet {
    fn drop(&mut self) {
        unsafe {
            self.device
                .free_descriptor_sets(self.descriptor_pool, &[self.handle])
                .expect("Cannot free descriptor set");
        }
    }
}

impl UsamiDevice {
    pub fn create_descriptor_pool(
        &self,
        name: String,
        create_info: DescriptorPoolCreateInfo,
    ) -> VkResult<UsamiDescriptorPool> {
        let shader = UsamiDescriptorPool::new(&self.handle, create_info)?;

        self.set_debug_name(name, shader.handle.as_raw(), ObjectType::DESCRIPTOR_POOL)?;

        Ok(shader)
    }
}

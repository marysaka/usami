use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{
        DescriptorPool, DescriptorPoolCreateFlags, DescriptorPoolCreateInfo, DescriptorSet,
        DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorSetLayoutCreateInfo, Handle,
        ObjectType,
    },
};

use crate::UsamiDevice;

pub struct UsamiDescriptorSetLayout {
    device: Arc<UsamiDevice>,
    pub handle: DescriptorSetLayout,
}

impl UsamiDescriptorSetLayout {
    pub fn new(
        device: &Arc<UsamiDevice>,
        create_info: DescriptorSetLayoutCreateInfo,
    ) -> VkResult<Self> {
        let handle = unsafe {
            device
                .handle
                .create_descriptor_set_layout(&create_info, None)?
        };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiDescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            println!("DROP SETLAYOUT??!");
            self.device
                .handle
                .destroy_descriptor_set_layout(self.handle, None)
        }
    }
}

pub struct UsamiDescriptorPool {
    device: Arc<UsamiDevice>,
    pub handle: DescriptorPool,
    should_free_sets: bool,
}

impl UsamiDescriptorPool {
    pub fn new(device: &Arc<UsamiDevice>, create_info: DescriptorPoolCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.handle.create_descriptor_pool(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
            should_free_sets: create_info
                .flags
                .contains(DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET),
        })
    }

    pub fn allocate_descriptor_sets(
        &self,
        name: String,
        layouts: &[DescriptorSetLayout],
    ) -> VkResult<Vec<UsamiDescriptorSet>> {
        let command_buffers = UsamiDescriptorSet::new(
            &self.device,
            DescriptorSetAllocateInfo::builder()
                .descriptor_pool(self.handle)
                .set_layouts(layouts)
                .build(),
            self.should_free_sets,
        )?;

        for (idx, command_buffer) in command_buffers.iter().enumerate() {
            self.device.set_debug_name(
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
        unsafe {
            self.device
                .handle
                .destroy_descriptor_pool(self.handle, None)
        }
    }
}

pub struct UsamiDescriptorSet {
    device: Arc<UsamiDevice>,
    descriptor_pool: DescriptorPool,
    pub handle: DescriptorSet,
    pub should_free_on_drop: bool,
}

impl UsamiDescriptorSet {
    pub fn new(
        device: &Arc<UsamiDevice>,
        allocate_info: DescriptorSetAllocateInfo,
        should_free_on_drop: bool,
    ) -> VkResult<Vec<Self>> {
        let result = unsafe { device.handle.allocate_descriptor_sets(&allocate_info)? };

        Ok(result
            .iter()
            .map(|handle| Self {
                device: device.clone(),
                descriptor_pool: allocate_info.descriptor_pool,
                handle: *handle,
                should_free_on_drop,
            })
            .collect())
    }
}

impl Drop for UsamiDescriptorSet {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                self.device
                    .handle
                    .free_descriptor_sets(self.descriptor_pool, &[self.handle])
                    .expect("Cannot free descriptor set");
            }
        }
    }
}

impl UsamiDevice {
    pub fn create_descriptor_pool(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: DescriptorPoolCreateInfo,
    ) -> VkResult<UsamiDescriptorPool> {
        let shader = UsamiDescriptorPool::new(device, create_info)?;

        device.set_debug_name(name, shader.handle.as_raw(), ObjectType::DESCRIPTOR_POOL)?;

        Ok(shader)
    }

    pub fn create_descriptor_set_layout(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: DescriptorSetLayoutCreateInfo,
    ) -> VkResult<UsamiDescriptorSetLayout> {
        let layout = UsamiDescriptorSetLayout::new(device, create_info)?;

        device.set_debug_name(
            name,
            layout.handle.as_raw(),
            ObjectType::DESCRIPTOR_SET_LAYOUT,
        )?;

        Ok(layout)
    }
}

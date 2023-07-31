use ash::{
    prelude::*,
    vk::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
        CommandPoolCreateInfo, Handle, ObjectType,
    },
    Device,
};

use crate::UsamiDevice;

pub struct UsamiCommandPool {
    device: Device,
    pub handle: CommandPool,
}

impl UsamiCommandPool {
    pub fn new(device: &Device, create_info: CommandPoolCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.create_command_pool(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }

    pub fn allocate_command_buffers(
        &self,
        device: &UsamiDevice,
        name: String,
        level: CommandBufferLevel,
        command_buffer_count: u32,
    ) -> VkResult<Vec<UsamiCommandBuffer>> {
        assert_eq!(self.device.handle(), device.vk_device.handle());

        let command_buffers = UsamiCommandBuffer::new(
            &self.device,
            CommandBufferAllocateInfo::builder()
                .command_pool(self.handle)
                .level(level)
                .command_buffer_count(command_buffer_count)
                .build(),
        )?;

        for (idx, command_buffer) in command_buffers.iter().enumerate() {
            device.set_debug_name(
                format!("{name}_{idx}"),
                command_buffer.handle.as_raw(),
                ObjectType::COMMAND_BUFFER,
            )?;
        }

        Ok(command_buffers)
    }
}

impl Drop for UsamiCommandPool {
    fn drop(&mut self) {
        unsafe { self.device.destroy_command_pool(self.handle, None) }
    }
}

pub struct UsamiCommandBuffer {
    device: Device,
    command_pool: CommandPool,
    pub handle: CommandBuffer,
}

impl UsamiCommandBuffer {
    pub fn new(device: &Device, allocate_info: CommandBufferAllocateInfo) -> VkResult<Vec<Self>> {
        let result = unsafe { device.allocate_command_buffers(&allocate_info)? };

        Ok(result
            .iter()
            .map(|handle| Self {
                device: device.clone(),
                command_pool: allocate_info.command_pool,
                handle: *handle,
            })
            .collect())
    }
}

impl Drop for UsamiCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device
                .free_command_buffers(self.command_pool, &[self.handle])
        }
    }
}

impl UsamiDevice {
    pub fn create_command_pool(
        &self,
        name: String,
        create_info: CommandPoolCreateInfo,
    ) -> VkResult<UsamiCommandPool> {
        let shader = UsamiCommandPool::new(&self.vk_device, create_info)?;

        self.set_debug_name(name, shader.handle.as_raw(), ObjectType::COMMAND_POOL)?;

        Ok(shader)
    }
}

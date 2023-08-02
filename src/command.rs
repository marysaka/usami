use ash::{
    prelude::*,
    vk::{
        AccessFlags, BufferImageCopy, CommandBuffer, CommandBufferAllocateInfo,
        CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsageFlags, CommandPool,
        CommandPoolCreateInfo, DependencyFlags, Handle, ImageAspectFlags, ImageLayout,
        ImageMemoryBarrier, ImageSubresourceLayers, ImageSubresourceRange, ObjectType,
        PipelineStageFlags,
    },
    Device,
};

use crate::{utils, UsamiBuffer, UsamiDevice, UsamiImage};

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
        assert_eq!(self.device.handle(), device.handle.handle());

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

    pub fn record<F: Fn(&UsamiDevice, &UsamiCommandBuffer) -> VkResult<()>>(
        &self,
        device: &UsamiDevice,
        flags: CommandBufferUsageFlags,
        callback: F,
    ) -> VkResult<()> {
        unsafe {
            self.device.begin_command_buffer(
                self.handle,
                &CommandBufferBeginInfo::builder().flags(flags).build(),
            )?;

            callback(device, self)?;

            self.device.end_command_buffer(self.handle)?;
        }

        Ok(())
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
        let shader = UsamiCommandPool::new(&self.handle, create_info)?;

        self.set_debug_name(name, shader.handle.as_raw(), ObjectType::COMMAND_POOL)?;

        Ok(shader)
    }

    pub fn copy_buffer_to_image(
        &self,
        buffer: &UsamiBuffer,
        image: &UsamiImage,
        new_layout: ImageLayout,
    ) -> VkResult<()> {
        utils::record_and_execute_command_buffer(
            self,
            "b2i_cmd_buffer".into(),
            |device, command_buffer| unsafe {
                let image_subresource_range = ImageSubresourceRange::builder()
                    .base_array_layer(0)
                    .layer_count(image.create_info.array_layers)
                    .base_mip_level(0)
                    .level_count(image.create_info.mip_levels)
                    .aspect_mask(ImageAspectFlags::COLOR)
                    .build();

                device.handle.cmd_pipeline_barrier(
                    command_buffer.handle,
                    PipelineStageFlags::TRANSFER,
                    PipelineStageFlags::TRANSFER,
                    DependencyFlags::empty(),
                    &[],
                    &[],
                    &[ImageMemoryBarrier::builder()
                        .src_access_mask(AccessFlags::empty())
                        .dst_access_mask(AccessFlags::TRANSFER_WRITE)
                        .old_layout(ImageLayout::UNDEFINED)
                        .new_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                        .image(image.handle)
                        .subresource_range(image_subresource_range)
                        .build()],
                );

                device.handle.cmd_copy_buffer_to_image(
                    command_buffer.handle,
                    buffer.handle,
                    image.handle,
                    ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[BufferImageCopy::builder()
                        .image_subresource(
                            ImageSubresourceLayers::builder()
                                .aspect_mask(ImageAspectFlags::COLOR)
                                .layer_count(1)
                                .build(),
                        )
                        .image_extent(image.create_info.extent)
                        .build()],
                );

                device.handle.cmd_pipeline_barrier(
                    command_buffer.handle,
                    PipelineStageFlags::TRANSFER,
                    PipelineStageFlags::TRANSFER,
                    DependencyFlags::empty(),
                    &[],
                    &[],
                    &[ImageMemoryBarrier::builder()
                        .src_access_mask(AccessFlags::TRANSFER_WRITE)
                        .dst_access_mask(AccessFlags::MEMORY_READ)
                        .old_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                        .new_layout(new_layout)
                        .image(image.handle)
                        .subresource_range(image_subresource_range)
                        .build()],
                );

                Ok(())
            },
        )
    }
}

use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{
        self, AccessFlags, BufferImageCopy, BufferMemoryBarrier, ClearColorValue, CommandBuffer,
        CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPool, CommandPoolCreateInfo, DependencyFlags, Handle,
        ImageAspectFlags, ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, MemoryBarrier,
        ObjectType, PipelineStageFlags,
    },
};

use crate::{utils, UsamiBuffer, UsamiDevice, UsamiImage};

pub struct UsamiCommandPool {
    device: Arc<UsamiDevice>,
    pub handle: CommandPool,
}

impl UsamiCommandPool {
    pub fn new(device: &Arc<UsamiDevice>, create_info: CommandPoolCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.handle.create_command_pool(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }

    pub fn allocate_command_buffers(
        &self,
        name: String,
        level: CommandBufferLevel,
        command_buffer_count: u32,
    ) -> VkResult<Vec<UsamiCommandBuffer>> {
        let command_buffers = UsamiCommandBuffer::new(
            &self.device,
            CommandBufferAllocateInfo::builder()
                .command_pool(self.handle)
                .level(level)
                .command_buffer_count(command_buffer_count)
                .build(),
        )?;

        for (idx, command_buffer) in command_buffers.iter().enumerate() {
            self.device.set_debug_name(
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
        unsafe { self.device.handle.destroy_command_pool(self.handle, None) }
    }
}

pub struct UsamiCommandBuffer {
    device: Arc<UsamiDevice>,
    command_pool: CommandPool,
    pub handle: CommandBuffer,
}

impl UsamiCommandBuffer {
    pub fn new(
        device: &Arc<UsamiDevice>,
        allocate_info: CommandBufferAllocateInfo,
    ) -> VkResult<Vec<Self>> {
        let result = unsafe { device.handle.allocate_command_buffers(&allocate_info)? };

        Ok(result
            .iter()
            .map(|handle| Self {
                device: device.clone(),
                command_pool: allocate_info.command_pool,
                handle: *handle,
            })
            .collect())
    }

    pub fn record<F: Fn(&Arc<UsamiDevice>, &UsamiCommandBuffer) -> VkResult<()>>(
        &self,
        flags: CommandBufferUsageFlags,
        callback: F,
    ) -> VkResult<()> {
        unsafe {
            self.device.handle.begin_command_buffer(
                self.handle,
                &CommandBufferBeginInfo::builder().flags(flags).build(),
            )?;

            callback(&self.device, self)?;

            self.device.handle.end_command_buffer(self.handle)?;
        }

        Ok(())
    }

    pub fn add_image_barrier(
        &self,
        image: &UsamiImage,
        image_subresource_range_opt: Option<ImageSubresourceRange>,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        src_access_mask: AccessFlags,
        dst_access_mask: AccessFlags,
        old_layout: ImageLayout,
        new_layout: ImageLayout,
    ) -> VkResult<()> {
        let image_subresource_range = image_subresource_range_opt.unwrap_or(
            ImageSubresourceRange::builder()
                .base_array_layer(0)
                .layer_count(image.create_info.array_layers)
                .base_mip_level(0)
                .level_count(image.create_info.mip_levels)
                .aspect_mask(ImageAspectFlags::COLOR)
                .build(),
        );

        unsafe {
            self.device.handle.cmd_pipeline_barrier(
                self.handle,
                src_stage_mask,
                dst_stage_mask,
                DependencyFlags::empty(),
                &[],
                &[],
                &[ImageMemoryBarrier::builder()
                    .src_access_mask(src_access_mask)
                    .dst_access_mask(dst_access_mask)
                    .old_layout(old_layout)
                    .new_layout(new_layout)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .image(image.handle)
                    .subresource_range(image_subresource_range)
                    .build()],
            );
        }

        Ok(())
    }

    pub fn add_buffer_barrier(
        &self,
        buffer: &UsamiBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        src_access_mask: AccessFlags,
        dst_access_mask: AccessFlags,
        offset: u64,
        size: u64,
    ) -> VkResult<()> {
        unsafe {
            self.device.handle.cmd_pipeline_barrier(
                self.handle,
                src_stage_mask,
                dst_stage_mask,
                DependencyFlags::empty(),
                &[],
                &[BufferMemoryBarrier::builder()
                    .src_access_mask(src_access_mask)
                    .dst_access_mask(dst_access_mask)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .buffer(buffer.handle)
                    .offset(offset)
                    .size(size)
                    .build()],
                &[],
            );
        }

        Ok(())
    }

    pub fn add_memory_barrier(
        &self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        src_access_mask: AccessFlags,
        dst_access_mask: AccessFlags,
    ) -> VkResult<()> {
        unsafe {
            self.device.handle.cmd_pipeline_barrier(
                self.handle,
                src_stage_mask,
                dst_stage_mask,
                DependencyFlags::empty(),
                &[MemoryBarrier::builder()
                    .src_access_mask(src_access_mask)
                    .dst_access_mask(dst_access_mask)
                    .build()],
                &[],
                &[],
            );
        }

        Ok(())
    }

    pub fn clear_image(&self, image: &UsamiImage, r: f32, g: f32, b: f32, a: f32) -> VkResult<()> {
        self.add_image_barrier(
            image,
            None,
            PipelineStageFlags::TOP_OF_PIPE,
            PipelineStageFlags::TRANSFER,
            AccessFlags::empty(),
            AccessFlags::TRANSFER_WRITE,
            ImageLayout::UNDEFINED,
            ImageLayout::GENERAL,
        )?;

        unsafe {
            self.device.handle.cmd_clear_color_image(
                self.handle,
                image.handle,
                ImageLayout::GENERAL,
                &ClearColorValue {
                    float32: [r, g, b, a],
                },
                &[ImageSubresourceRange::builder()
                    .aspect_mask(ImageAspectFlags::COLOR)
                    .build()],
            );
        }

        self.add_memory_barrier(
            PipelineStageFlags::TRANSFER,
            PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            AccessFlags::TRANSFER_WRITE,
            AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
        )?;

        Ok(())
    }

    pub fn copy_buffer_to_image(
        &self,
        buffer: &UsamiBuffer,
        buffer_size: u64,
        copy_regions: &[BufferImageCopy],
        image_aspect_flags: ImageAspectFlags,
        mip_levels: u32,
        array_layers: u32,
        dest_image: &UsamiImage,
        dest_image_layout: ImageLayout,
        dest_image_dst_stage_flags: PipelineStageFlags,
        dest_image_dst_access_mask: AccessFlags,
        base_mip_level: u32,
    ) -> VkResult<()> {
        let subresource_range = ImageSubresourceRange::builder()
            .aspect_mask(image_aspect_flags)
            .base_mip_level(base_mip_level)
            .level_count(mip_levels)
            .base_array_layer(0)
            .layer_count(array_layers)
            .build();

        unsafe {
            self.device.handle.cmd_pipeline_barrier(
                self.handle,
                PipelineStageFlags::HOST,
                PipelineStageFlags::TRANSFER,
                DependencyFlags::empty(),
                &[],
                &[BufferMemoryBarrier::builder()
                    .src_access_mask(AccessFlags::HOST_WRITE)
                    .dst_access_mask(AccessFlags::TRANSFER_READ)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .buffer(buffer.handle)
                    .offset(0)
                    .size(buffer_size)
                    .build()],
                &[ImageMemoryBarrier::builder()
                    .src_access_mask(AccessFlags::empty())
                    .dst_access_mask(AccessFlags::TRANSFER_WRITE)
                    .old_layout(ImageLayout::UNDEFINED)
                    .new_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .image(dest_image.handle)
                    .subresource_range(subresource_range)
                    .build()],
            );

            self.device.handle.cmd_copy_buffer_to_image(
                self.handle,
                buffer.handle,
                dest_image.handle,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                copy_regions,
            );

            self.add_image_barrier(
                dest_image,
                Some(subresource_range),
                PipelineStageFlags::TRANSFER,
                dest_image_dst_stage_flags,
                AccessFlags::TRANSFER_WRITE,
                dest_image_dst_access_mask,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                dest_image_layout,
            )?;
        }

        Ok(())
    }

    pub fn copy_image_to_buffer(
        &self,
        image: &UsamiImage,
        buffer: &UsamiBuffer,
        regions: &[BufferImageCopy],
        src_access_mask: AccessFlags,
        old_layout: ImageLayout,
        layer_count: u32,
        level_count: u32,
        barrier_aspect: ImageAspectFlags,
        src_stage_mask: PipelineStageFlags,
    ) -> VkResult<()> {
        let image_subresource_range = ImageSubresourceRange::builder()
            .base_array_layer(0)
            .layer_count(layer_count)
            .base_mip_level(0)
            .level_count(level_count)
            .aspect_mask(barrier_aspect)
            .build();

        self.add_image_barrier(
            image,
            Some(image_subresource_range),
            src_stage_mask,
            PipelineStageFlags::TRANSFER,
            src_access_mask,
            AccessFlags::TRANSFER_READ,
            old_layout,
            ImageLayout::TRANSFER_SRC_OPTIMAL,
        )?;

        unsafe {
            self.device.handle.cmd_copy_image_to_buffer(
                self.handle,
                image.handle,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                buffer.handle,
                regions,
            );
        }
        self.add_buffer_barrier(
            buffer,
            PipelineStageFlags::TRANSFER,
            PipelineStageFlags::HOST,
            AccessFlags::TRANSFER_WRITE,
            AccessFlags::HOST_READ,
            0,
            vk::WHOLE_SIZE,
        )?;

        Ok(())
    }
}

impl Drop for UsamiCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device
                .handle
                .free_command_buffers(self.command_pool, &[self.handle])
        }
    }
}

impl UsamiDevice {
    pub fn create_command_pool(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: CommandPoolCreateInfo,
    ) -> VkResult<UsamiCommandPool> {
        let shader = UsamiCommandPool::new(device, create_info)?;

        device.set_debug_name(name, shader.handle.as_raw(), ObjectType::COMMAND_POOL)?;

        Ok(shader)
    }

    pub fn copy_buffer_to_image(
        device: &Arc<UsamiDevice>,
        command_pool: &UsamiCommandPool,
        buffer: &UsamiBuffer,
        image: &UsamiImage,
        new_layout: ImageLayout,
        copy_regions: &[BufferImageCopy],
    ) -> VkResult<()> {
        utils::record_and_execute_command_buffer(
            device,
            command_pool,
            "b2i_cmd_buffer".into(),
            |_, command_buffer| {
                command_buffer.copy_buffer_to_image(
                    buffer,
                    vk::WHOLE_SIZE,
                    copy_regions,
                    ImageAspectFlags::COLOR,
                    image.create_info.mip_levels,
                    image.create_info.array_layers,
                    image,
                    new_layout,
                    PipelineStageFlags::FRAGMENT_SHADER,
                    AccessFlags::SHADER_READ,
                    0,
                )?;

                Ok(())
            },
        )
    }
}

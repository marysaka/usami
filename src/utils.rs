use ash::{
    prelude::VkResult,
    vk::{
        AccessFlags, BufferImageCopy, CommandBuffer, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsageFlags, DependencyFlags, Extent3D, FenceCreateFlags, FenceCreateInfo,
        ImageAspectFlags, ImageLayout, ImageMemoryBarrier, ImageSubresourceLayers,
        ImageSubresourceRange, Offset3D, PipelineStageFlags, SubmitInfo,
    },
};

use crate::{UsamiCommandBuffer, UsamiDevice, UsamiImage};

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
        }
    }};
}

pub fn as_u32_vec(data: &[u8]) -> Vec<u32> {
    let mut res = Vec::new();

    let size_aligned = (data.len() + 3) / 4;

    res.resize(size_aligned, 0);

    unsafe {
        let temp_slice =
            std::slice::from_raw_parts_mut(res.as_mut_ptr() as *mut u8, size_aligned * 4);

        temp_slice[..data.len()].copy_from_slice(data);
    }

    res
}

pub fn record_command_buffer_with_image_dep<
    F: Fn(&UsamiDevice, CommandBuffer, &UsamiImage) -> ImageLayout,
>(
    device: &UsamiDevice,
    command_buffer: CommandBuffer,
    image: &UsamiImage,
    callback: F,
) -> VkResult<()> {
    let image_subresource_range = ImageSubresourceRange::builder()
        .base_array_layer(0)
        .layer_count(image.create_info.array_layers)
        .base_mip_level(0)
        .level_count(image.create_info.mip_levels)
        .aspect_mask(ImageAspectFlags::COLOR)
        .build();

    unsafe {
        device.handle.begin_command_buffer(
            command_buffer,
            &CommandBufferBeginInfo::builder()
                .flags(CommandBufferUsageFlags::SIMULTANEOUS_USE)
                .build(),
        )?;
    }

    let old_image_layout = callback(device, command_buffer, image);

    unsafe {
        device.handle.cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::BOTTOM_OF_PIPE,
            PipelineStageFlags::TRANSFER,
            DependencyFlags::empty(),
            &[],
            &[],
            &[ImageMemoryBarrier::builder()
                .src_access_mask(AccessFlags::MEMORY_WRITE)
                .dst_access_mask(AccessFlags::TRANSFER_READ)
                .old_layout(old_image_layout)
                .new_layout(ImageLayout::TRANSFER_SRC_OPTIMAL)
                .src_queue_family_index(device.vk_queue_index)
                .dst_queue_family_index(device.vk_queue_index)
                .image(image.handle)
                .subresource_range(image_subresource_range)
                .build()],
        );

        device.handle.cmd_copy_image_to_buffer(
            command_buffer,
            device.presentation_image().handle,
            ImageLayout::TRANSFER_SRC_OPTIMAL,
            device.presentation_buffer_readback().handle,
            &[BufferImageCopy::builder()
                .image_offset(Offset3D::builder().x(0).y(0).z(0).build())
                .image_subresource(
                    ImageSubresourceLayers::builder()
                        .aspect_mask(ImageAspectFlags::COLOR)
                        .layer_count(1)
                        .build(),
                )
                .image_extent(Extent3D {
                    width: device.width,
                    height: device.height,
                    depth: 1,
                })
                .build()],
        );

        device.handle.end_command_buffer(command_buffer)?;
    }

    Ok(())
}

pub fn record_and_execute_command_buffer<
    F: Fn(&UsamiDevice, &UsamiCommandBuffer) -> VkResult<()>,
>(
    device: &UsamiDevice,
    command_buffer_name: String,
    callback: F,
) -> VkResult<()> {
    unsafe {
        let command_buffers = device.command_pool.allocate_command_buffers(
            device,
            command_buffer_name,
            CommandBufferLevel::PRIMARY,
            1,
        )?;

        let command_buffer = &command_buffers[0];

        device.handle.begin_command_buffer(
            command_buffer.handle,
            &CommandBufferBeginInfo::builder()
                .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT)
                .build(),
        )?;

        callback(device, command_buffer)?;

        device.handle.end_command_buffer(command_buffer.handle)?;

        // TODO: wrapper around fence
        let fence = device.handle.create_fence(
            &FenceCreateInfo::builder()
                .flags(FenceCreateFlags::empty())
                .build(),
            None,
        )?;

        device.handle.queue_submit(
            device.vk_queue,
            &[SubmitInfo::builder()
                .command_buffers(&[command_buffer.handle])
                .build()],
            fence,
        )?;
        device
            .handle
            .wait_for_fences(&[fence], true, std::u64::MAX)?;
        device.handle.reset_fences(&[fence])?;
        device.handle.destroy_fence(fence, None);
    }

    Ok(())
}

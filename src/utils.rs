use ash::{
    prelude::VkResult,
    vk::{
        AccessFlags, BufferImageCopy, CommandBufferLevel, CommandBufferUsageFlags, DependencyFlags,
        Extent3D, FenceCreateFlags, ImageAspectFlags, ImageLayout, ImageMemoryBarrier,
        ImageSubresourceLayers, ImageSubresourceRange, Offset3D, PipelineStageFlags, SubmitInfo,
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

/// Generic function to turn any Sized slice to a slice of u8.
///
/// # Safety
///
/// Follow requirements of [std::slice::from_raw_parts].
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn record_command_buffer_with_image_dep<
    F: Fn(&UsamiDevice, &UsamiCommandBuffer, &UsamiImage) -> ImageLayout,
>(
    device: &UsamiDevice,
    command_buffer: &UsamiCommandBuffer,
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

    command_buffer.record(
        device,
        CommandBufferUsageFlags::SIMULTANEOUS_USE,
        |device, command_buffer| {
            let old_image_layout = callback(device, command_buffer, image);

            unsafe {
                device.handle.cmd_pipeline_barrier(
                    command_buffer.handle,
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
                        .image(image.handle)
                        .subresource_range(image_subresource_range)
                        .build()],
                );

                device.handle.cmd_copy_image_to_buffer(
                    command_buffer.handle,
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

                device.handle.cmd_pipeline_barrier(
                    command_buffer.handle,
                    PipelineStageFlags::TRANSFER,
                    PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    DependencyFlags::empty(),
                    &[],
                    &[],
                    &[ImageMemoryBarrier::builder()
                        .src_access_mask(AccessFlags::TRANSFER_READ)
                        .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)
                        .old_layout(ImageLayout::TRANSFER_SRC_OPTIMAL)
                        .new_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                        .image(image.handle)
                        .subresource_range(image_subresource_range)
                        .build()],
                );
            }

            Ok(())
        },
    )
}

pub fn record_and_execute_command_buffer<
    F: Fn(&UsamiDevice, &UsamiCommandBuffer) -> VkResult<()>,
>(
    device: &UsamiDevice,
    command_buffer_name: String,
    callback: F,
) -> VkResult<()> {
    let command_buffers = device.command_pool.allocate_command_buffers(
        device,
        command_buffer_name.clone(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    let command_buffer = &command_buffers[0];

    command_buffer.record(device, CommandBufferUsageFlags::ONE_TIME_SUBMIT, callback)?;

    let fence = device.create_fence(
        format!("{command_buffer_name}_fence"),
        FenceCreateFlags::empty(),
    )?;

    device.get_queue()?.submit(
        &[SubmitInfo::builder()
            .command_buffers(&[command_buffer.handle])
            .build()],
        &fence,
    )?;
    fence.wait(u64::MAX)?;
    fence.reset()?;

    Ok(())
}

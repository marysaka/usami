use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, ClearColorValue, CommandBufferLevel, DependencyFlags, FenceCreateFlags,
        ImageAspectFlags, ImageLayout, ImageMemoryBarrier, ImageSubresourceRange,
        PipelineStageFlags, SubmitInfo,
    },
};
use usami::{UsamiDevice, UsamiInstance};

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let width = 500;
    let height = 500;

    let instance = UsamiInstance::new("simple", "usami", vk::API_VERSION_1_1, &extensions, true)?;
    let device: UsamiDevice = UsamiDevice::new_by_filter(
        instance,
        &[],
        width,
        height,
        Box::new(|physical_device| {
            physical_device
                .queue_familiy_properties
                .iter()
                .enumerate()
                .find_map(|(i, x)| {
                    if x.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
                .map(|x| (physical_device, x))
        }),
    )?;

    let command_buffers = device.command_pool.allocate_command_buffers(
        &device,
        "command_buffer".into(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    usami::utils::record_command_buffer_with_image_dep(
        &device,
        &command_buffers[0],
        device.presentation_image(),
        device.presentation_buffer_readback(),
        |device, command_buffer, image| {
            let image_subresource_range = ImageSubresourceRange::builder()
                .base_array_layer(0)
                .layer_count(image.create_info.array_layers)
                .base_mip_level(0)
                .level_count(image.create_info.mip_levels)
                .aspect_mask(ImageAspectFlags::COLOR)
                .build();

            unsafe {
                device.handle.cmd_pipeline_barrier(
                    command_buffer.handle,
                    PipelineStageFlags::TRANSFER,
                    PipelineStageFlags::TRANSFER,
                    DependencyFlags::empty(),
                    &[],
                    &[],
                    &[ImageMemoryBarrier::builder()
                        .src_access_mask(AccessFlags::MEMORY_READ)
                        .dst_access_mask(AccessFlags::TRANSFER_WRITE)
                        .old_layout(ImageLayout::UNDEFINED)
                        .new_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                        .src_queue_family_index(device.vk_queue_index)
                        .dst_queue_family_index(device.vk_queue_index)
                        .image(image.handle)
                        .subresource_range(image_subresource_range)
                        .build()],
                );

                let mut clear_color_value = ClearColorValue::default();
                clear_color_value.float32 = [0.6f32, 0.5f32, 1.0f32, 1.0f32];

                device.handle.cmd_clear_color_image(
                    command_buffer.handle,
                    image.handle,
                    ImageLayout::TRANSFER_DST_OPTIMAL,
                    &clear_color_value,
                    &[image_subresource_range],
                );

                ImageLayout::TRANSFER_DST_OPTIMAL
            }
        },
    )?;

    let fence = device.create_fence("fence".into(), FenceCreateFlags::empty())?;

    device.get_queue()?.submit(
        &[SubmitInfo::builder()
            .command_buffers(&[command_buffers[0].handle])
            .build()],
        &fence,
    )?;
    fence.wait(u64::MAX)?;
    fence.reset()?;

    let res: Vec<u8> = device.read_image_memory()?;

    image::save_buffer_with_format(
        "output.bmp",
        &res,
        width,
        height,
        image::ColorType::Rgba8,
        image::ImageFormat::Bmp,
    )
    .unwrap();
    Ok(())
}

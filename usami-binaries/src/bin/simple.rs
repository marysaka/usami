use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, ClearColorValue, CommandBufferLevel, CommandPoolCreateFlags,
        CommandPoolCreateInfo, DependencyFlags, FenceCreateFlags, ImageAspectFlags, ImageLayout,
        ImageMemoryBarrier, ImageSubresourceRange, PipelineStageFlags, SubmitInfo,
    },
};
use usami::{UsamiDevice, UsamiInstance, UsamiPresentation};

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let width = 500;
    let height = 500;

    let instance = UsamiInstance::new("simple", "usami", vk::API_VERSION_1_1, &extensions, true)?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[],
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
    let presentation = UsamiPresentation::new(&device, width, height)?;

    let command_pool = UsamiDevice::create_command_pool(
        &device,
        "command_pool".into(),
        CommandPoolCreateInfo::builder()
            .queue_family_index(device.vk_queue_index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .build(),
    )?;

    let command_buffers = command_pool.allocate_command_buffers(
        "command_buffer".into(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    usami::utils::record_command_buffer_with_image_dep(
        &command_buffers[0],
        &presentation.image,
        &presentation.buffer_readback,
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

    let fence = UsamiDevice::create_fence(&device, "fence".into(), FenceCreateFlags::empty())?;
    let queue = UsamiDevice::get_device_queue(&device, "queue".into(), device.vk_queue_index, 0)?;

    queue.submit(
        &[SubmitInfo::builder()
            .command_buffers(&[command_buffers[0].handle])
            .build()],
        &fence,
    )?;
    fence.wait(u64::MAX)?;
    fence.reset()?;

    let res = presentation.buffer_readback.device_memory.read_to_vec()?;

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

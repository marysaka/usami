use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, BufferCreateFlags, BufferUsageFlags, ClearColorValue, DependencyFlags,
        Fence, ImageAspectFlags, ImageLayout, ImageMemoryBarrier, ImageSubresourceRange,
        PipelineStageFlags, SharingMode, SubmitInfo,
    },
};
use usami::{UsamiDevice, UsamiInstance};

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let width = 1920;
    let height = 1080;

    let instance = UsamiInstance::new("triangle", "usami", vk::API_VERSION_1_1, &extensions)?;
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

    let _index_buffer = device.create_buffer(
        "index_buffer".into(),
        BufferCreateFlags::empty(),
        &[device.vk_queue_index],
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::INDEX_BUFFER,
        &[0u32, 1, 2],
    )?;

    usami::utils::record_command_buffer_with_image_dep(
        &device,
        device.vk_command_buffer,
        device.presentation_image(),
        |device, command_buffer, image| {
            let image_subresource_range = ImageSubresourceRange::builder()
                .base_array_layer(0)
                .layer_count(image.create_info.array_layers)
                .base_mip_level(0)
                .level_count(image.create_info.mip_levels)
                .aspect_mask(ImageAspectFlags::COLOR)
                .build();

            unsafe {
                device.vk_device.cmd_pipeline_barrier(
                    command_buffer,
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

                device.vk_device.cmd_clear_color_image(
                    command_buffer,
                    image.handle,
                    ImageLayout::TRANSFER_DST_OPTIMAL,
                    &clear_color_value,
                    &[image_subresource_range],
                );

                ImageLayout::TRANSFER_DST_OPTIMAL
            }
        },
    )?;

    unsafe {
        device.vk_device.queue_submit(
            device.vk_queue,
            &[SubmitInfo::builder()
                .command_buffers(&[device.vk_command_buffer])
                .build()],
            Fence::null(),
        )?;

        device.vk_device.device_wait_idle()?;
    }

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

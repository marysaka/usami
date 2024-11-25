use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, BufferCreateFlags, BufferUsageFlags, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPoolCreateFlags, CommandPoolCreateInfo, FenceCreateFlags,
        ImageAspectFlags, ImageLayout, ImageUsageFlags, MemoryPropertyFlags, PipelineStageFlags,
        QueueFlags, SharingMode, SubmitInfo,
    },
};
use usami::{utils, UsamiDevice, UsamiInstance};

#[derive(Clone, Debug, Copy, Default)]
#[repr(packed(1))]
#[allow(dead_code)]
struct UniformBlock {
    bias: f32,
    reference: f32,
    _padding1: f32,
    _padding2: f32,
    color_scale: [f32; 4],
    color_bias: [f32; 4],
}

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let width = 128;
    let height = 1;

    let instance = UsamiInstance::new("mipmap", "usami", vk::API_VERSION_1_1, &extensions, true)?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[],
        Box::new(|physical_device| {
            physical_device
                .queue_familiy_properties
                .iter()
                .enumerate()
                .find_map(|(i, x)| {
                    if x.queue_flags.contains(QueueFlags::GRAPHICS) {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
                .map(|x| (physical_device, x))
        }),
    )?;

    let mipmap_count = utils::compute_mip_pyramid_levels(width, height);

    let command_pool = UsamiDevice::create_command_pool(
        &device,
        "command_pool".into(),
        CommandPoolCreateInfo::default()
            .queue_family_index(device.vk_queue_index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
    )?;

    let gradient_raw_image =
        utils::create_gradient_image_with_mip_levels(width, height, mipmap_count);
    let gradient_image = UsamiDevice::import_image(
        &device,
        &command_pool,
        "gradient_image".into(),
        &gradient_raw_image,
        ImageUsageFlags::SAMPLED | ImageUsageFlags::TRANSFER_SRC,
        ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )?;

    let gradient_readback_buffer = UsamiDevice::create_buffer_with_size(
        &device,
        "gradient_readback_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::TRANSFER_DST,
        gradient_raw_image.data.len() as u64,
        MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let command_buffers = command_pool.allocate_command_buffers(
        "command_buffer".into(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    command_buffers[0].record(
        CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        |_, command_buffer| {
            command_buffer.copy_image_to_buffer(
                &gradient_image,
                &gradient_readback_buffer,
                &gradient_raw_image.copy_regions(ImageAspectFlags::COLOR),
                AccessFlags::empty(),
                ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                1,
                gradient_raw_image.level_count() as u32,
                ImageAspectFlags::COLOR,
                PipelineStageFlags::ALL_COMMANDS,
            )?;

            Ok(())
        },
    )?;

    let fence = UsamiDevice::create_fence(&device, "fence".into(), FenceCreateFlags::empty())?;
    let queue = UsamiDevice::get_device_queue(&device, "queue".into(), device.vk_queue_index, 0)?;

    queue.submit(
        &[SubmitInfo::default().command_buffers(&[command_buffers[0].handle])],
        &fence,
    )?;
    fence.wait(u64::MAX)?;
    fence.reset()?;

    let gradient_readback_raw_buffer = gradient_readback_buffer.device_memory.read_to_vec()?;

    for (index, level) in gradient_raw_image.level_infos.iter().enumerate() {
        let layer0 = &level.layers[0];
        image::save_buffer_with_format(
            format!("output_{index}.png"),
            &gradient_readback_raw_buffer[layer0.start_position
                ..layer0.start_position + layer0.size(gradient_raw_image.format) as usize],
            layer0.extent.width,
            layer0.extent.height,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }

    Ok(())
}

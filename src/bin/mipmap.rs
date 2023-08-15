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

    let gradient_raw_image =
        utils::create_gradient_image_with_mip_levels(width, height, mipmap_count);
    let gradient_image = device.import_image(
        "gradient_image".into(),
        &gradient_raw_image,
        ImageUsageFlags::SAMPLED | ImageUsageFlags::TRANSFER_SRC,
        ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )?;

    let gradient_readback_buffer = device.create_buffer_with_size(
        "gradient_readback_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::TRANSFER_DST,
        gradient_raw_image.data.len() as u64,
        MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let command_pool = device.create_command_pool(
        "command_pool".into(),
        CommandPoolCreateInfo::builder()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .build(),
    )?;

    let command_buffers = command_pool.allocate_command_buffers(
        &device,
        "command_buffer".into(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    command_buffers[0].record(
        &device,
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

    let fence = device.create_fence("fence".into(), FenceCreateFlags::empty())?;

    device.get_queue()?.submit(
        &[SubmitInfo::builder()
            .command_buffers(&[command_buffers[0].handle])
            .build()],
        &fence,
    )?;
    fence.wait(u64::MAX)?;
    fence.reset()?;

    let gradient_readback_raw_buffer = gradient_readback_buffer.device_memory.read_to_vec()?;

    for (index, level) in gradient_raw_image.level_infos.iter().enumerate() {
        image::save_buffer_with_format(
            format!("output_{index}.png"),
            &gradient_readback_raw_buffer[level.start_position
                ..level.start_position + level.size(gradient_raw_image.format) as usize],
            level.extent.width,
            level.extent.height,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }

    Ok(())
}

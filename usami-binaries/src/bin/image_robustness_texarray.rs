use std::ffi::CString;

use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, BorderColor, BufferCreateFlags, BufferUsageFlags, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPoolCreateFlags, CommandPoolCreateInfo, CompareOp,
        ComponentMapping, ComponentSwizzle, ComputePipelineCreateInfo, DescriptorImageInfo,
        DescriptorPoolCreateInfo, DescriptorSetLayoutCreateInfo, DescriptorType, Extent3D,
        FenceCreateFlags, Filter, Format, ImageAspectFlags, ImageCreateInfo, ImageLayout,
        ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageViewCreateFlags,
        ImageViewType, MemoryPropertyFlags, PipelineBindPoint, PipelineCache,
        PipelineShaderStageCreateInfo, PipelineStageFlags, QueueFlags, SampleCountFlags,
        SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode, ShaderStageFlags, SharingMode,
        SubmitInfo, WriteDescriptorSet,
    },
};
use usami::{image::RawImageData, UsamiDevice, UsamiInstance};

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
    let extensions = [
        "VK_EXT_debug_utils".into(),
        "VK_KHR_get_physical_device_properties2".into(),
    ];
    let width = 128;

    let (group_count_x, group_count_y, group_count_z) = (1, 1, 1);

    let instance = UsamiInstance::new(
        "image_robustness_texarray",
        "usami",
        vk::API_VERSION_1_0,
        &extensions,
        true,
    )?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[ash::ext::image_robustness::NAME
            .to_string_lossy()
            .to_string()],
        Box::new(|physical_device| {
            physical_device
                .queue_familiy_properties
                .iter()
                .enumerate()
                .find_map(|(i, x)| {
                    if x.queue_flags.contains(QueueFlags::COMPUTE) {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
                .map(|x| (physical_device, x))
        }),
    )?;

    let command_pool = UsamiDevice::create_command_pool(
        &device,
        "command_pool".into(),
        CommandPoolCreateInfo::default()
            .queue_family_index(device.vk_queue_index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
    )?;

    let white_image_buffer =
        image::load_from_memory(include_bytes!("../../resources/texture/white.png"))
            .unwrap()
            .to_rgba8();

    // Create image infos for import and duplicate layer 0 to get a texture array.
    let mut white_image_info = RawImageData::from(white_image_buffer);
    let level0 = white_image_info.level_infos.get_mut(0).unwrap();
    let layer0 = level0.layers.get(0).unwrap();
    level0.layers.push(layer0.clone());

    let white_image = UsamiDevice::import_image(
        &device,
        &command_pool,
        "white_image".into(),
        &white_image_info,
        ImageUsageFlags::SAMPLED,
        ImageLayout::GENERAL,
    )?;

    let white_image_view = white_image.create_simple_image_view(
        "presentation_image_view".into(),
        ImageViewType::TYPE_2D_ARRAY,
        ImageSubresourceRange::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(white_image.mip_levels)
            .base_array_layer(0)
            .layer_count(white_image.array_layers),
        ComponentMapping::default()
            .r(ComponentSwizzle::IDENTITY)
            .g(ComponentSwizzle::IDENTITY)
            .b(ComponentSwizzle::IDENTITY)
            .a(ComponentSwizzle::IDENTITY),
        ImageViewCreateFlags::empty(),
    )?;

    let white_image_sampler = UsamiDevice::create_sampler(
        &device,
        "sampler".into(),
        SamplerCreateInfo::default()
            .mag_filter(Filter::NEAREST)
            .min_filter(Filter::NEAREST)
            .mipmap_mode(SamplerMipmapMode::NEAREST)
            .address_mode_u(SamplerAddressMode::REPEAT)
            .address_mode_v(SamplerAddressMode::REPEAT)
            .address_mode_w(SamplerAddressMode::REPEAT)
            .max_anisotropy(1.0)
            .border_color(BorderColor::FLOAT_TRANSPARENT_BLACK)
            .compare_op(CompareOp::NEVER),
    )?;

    let output_image_info = ImageCreateInfo::default()
        .image_type(ImageType::TYPE_1D)
        .format(Format::R32_SFLOAT)
        .extent(Extent3D {
            width,
            height: 1,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(SampleCountFlags::TYPE_1)
        .tiling(ImageTiling::OPTIMAL)
        .usage(ImageUsageFlags::STORAGE | ImageUsageFlags::TRANSFER_SRC);

    let output_image = UsamiDevice::create_image(
        &device,
        "output_image".into(),
        output_image_info,
        MemoryPropertyFlags::empty(),
    )?;
    let output_image_view = output_image.create_simple_image_view(
        "output_image_view".into(),
        ImageViewType::TYPE_1D,
        ImageSubresourceRange::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(output_image.array_layers),
        ComponentMapping::default()
            .r(ComponentSwizzle::IDENTITY)
            .g(ComponentSwizzle::IDENTITY)
            .b(ComponentSwizzle::IDENTITY)
            .a(ComponentSwizzle::IDENTITY),
        ImageViewCreateFlags::empty(),
    )?;

    let output_readback_buffer = UsamiDevice::create_buffer_with_size(
        &device,
        "output_readback_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::TRANSFER_DST,
        u64::from(width * 4 * output_image_info.array_layers),
        MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let descriptor_pool_sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_IMAGE,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
        },
    ];
    let descriptor_pool_create_info = DescriptorPoolCreateInfo::default()
        .pool_sizes(&descriptor_pool_sizes)
        .max_sets(1);

    let descriptor_pool = UsamiDevice::create_descriptor_pool(
        &device,
        "descriptor_pool".into(),
        descriptor_pool_create_info,
    )?;

    let desc_layout_bindings = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(DescriptorType::STORAGE_IMAGE)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE),
        vk::DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE),
    ];
    let descriptor_set_layout = UsamiDevice::create_descriptor_set_layout(
        &device,
        "descriptor_set_layout".into(),
        DescriptorSetLayoutCreateInfo::default().bindings(&desc_layout_bindings),
    )?;

    let descriptor_sets = descriptor_pool
        .allocate_descriptor_sets("descriptor_set".into(), &[descriptor_set_layout.handle])?;

    unsafe {
        device.handle.update_descriptor_sets(
            &[
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(0)
                    .descriptor_type(DescriptorType::STORAGE_IMAGE)
                    .image_info(&[DescriptorImageInfo::default()
                        .image_layout(ImageLayout::GENERAL)
                        .image_view(output_image_view.handle)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(1)
                    .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&[DescriptorImageInfo::default()
                        .image_layout(ImageLayout::GENERAL)
                        .image_view(white_image_view.handle)
                        .sampler(white_image_sampler.handle)]),
            ],
            &[],
        );
    }

    let pipeline_layout = UsamiDevice::create_pipeline_layout(
        &device,
        "base_pipeline_layout".into(),
        &[descriptor_set_layout.handle],
        &[],
    )?;

    let shader_entrypoint_name = CString::new("main").unwrap();

    let compute_shader_code = usami::utils::as_u32_vec(include_bytes!(
        "../../resources/image_robustness_texarray/main.comp.spv"
    ));
    let compute_shader =
        UsamiDevice::create_shader(&device, "compute_shader".into(), &compute_shader_code)?;

    let shader_stage_create_info = PipelineShaderStageCreateInfo::default()
        .module(compute_shader.handle)
        .name(shader_entrypoint_name.as_c_str())
        .stage(ShaderStageFlags::COMPUTE);

    let compute_pipeline_create_info = ComputePipelineCreateInfo::default()
        .layout(pipeline_layout.handle)
        .stage(shader_stage_create_info);

    let pipelines = UsamiDevice::create_compute_pipelines(
        &device,
        "pipeline".into(),
        PipelineCache::null(),
        &[compute_pipeline_create_info],
    )?;

    let command_buffers = command_pool.allocate_command_buffers(
        "command_buffer".into(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    let pipeline = &pipelines[0];

    command_buffers[0].record(
        CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        |_, command_buffer| {
            let vk_device = &device.handle;
            unsafe {
                vk_device.cmd_bind_pipeline(
                    command_buffer.handle,
                    PipelineBindPoint::COMPUTE,
                    pipeline.handle,
                );

                vk_device.cmd_bind_descriptor_sets(
                    command_buffer.handle,
                    PipelineBindPoint::COMPUTE,
                    pipeline_layout.handle,
                    0,
                    &[descriptor_sets[0].handle],
                    &[],
                );
            }

            command_buffer.add_image_barrier(
                &output_image,
                None,
                PipelineStageFlags::HOST,
                PipelineStageFlags::COMPUTE_SHADER,
                AccessFlags::empty(),
                AccessFlags::SHADER_WRITE,
                ImageLayout::UNDEFINED,
                ImageLayout::GENERAL,
            )?;

            unsafe {
                vk_device.cmd_dispatch(
                    command_buffer.handle,
                    group_count_x,
                    group_count_y,
                    group_count_z,
                );
            }

            command_buffer.copy_image_to_buffer(
                &output_image,
                &output_readback_buffer,
                &[output_image.buffer_copy(ImageAspectFlags::COLOR, 0, 0, 1)],
                AccessFlags::empty(),
                ImageLayout::GENERAL,
                output_image.array_layers,
                output_image.mip_levels,
                ImageAspectFlags::COLOR,
                PipelineStageFlags::COMPUTE_SHADER,
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

    let output_readback_raw_buffer = output_readback_buffer.device_memory.read_to_vec()?;

    for i in 0..3 {
        let val = f32::from_le_bytes(
            output_readback_raw_buffer[(i) * 4..(i + 1) * 4]
                .try_into()
                .unwrap(),
        );
        println!("val{i}: {val}");
    }

    Ok(())
}

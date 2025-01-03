use std::{ffi::CString, path::PathBuf};

use argh::FromArgs;
use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, BufferCreateFlags, BufferUsageFlags, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPoolCreateFlags, CommandPoolCreateInfo, ComponentMapping,
        ComponentSwizzle, ComputePipelineCreateInfo, DescriptorBufferInfo, DescriptorImageInfo,
        DescriptorPoolCreateInfo, DescriptorSetLayoutCreateInfo, Extent3D, FenceCreateFlags,
        Format, ImageAspectFlags, ImageCreateInfo, ImageLayout, ImageSubresourceRange, ImageTiling,
        ImageType, ImageUsageFlags, ImageViewCreateFlags, ImageViewType, MemoryPropertyFlags,
        PipelineBindPoint, PipelineCache, PipelineShaderStageCreateInfo, PipelineStageFlags,
        QueueFlags, SampleCountFlags, ShaderStageFlags, SharingMode, SubmitInfo,
        WriteDescriptorSet,
    },
};
use usami::{UsamiDevice, UsamiInstance};

#[derive(FromArgs)]
/// Reach new heights.
struct Args {
    /// the path to the compute shader to use.
    #[argh(option)]
    compute_path: PathBuf,

    /// the X size of the workgroup.
    #[argh(option)]
    group_count_x: Option<u32>,

    /// the Y size of the workgroup.
    #[argh(option)]
    group_count_y: Option<u32>,

    /// the Z size of the workgroup.
    #[argh(option)]
    group_count_z: Option<u32>,

    /// the path of the file to load the input buffer.
    #[argh(option)]
    input_buffer_file: Option<PathBuf>,

    /// extra device extension to add.
    #[argh(option)]
    device_extension: Vec<String>,

    /// the path of the file to store the output buffer.
    #[argh(option)]
    output_buffer_file: Option<PathBuf>,

    /// make the input a buffer instead of a uniform.
    #[argh(option, default = "false")]
    input_as_buffer: bool,

    /// vulkan API raw version to use.
    #[argh(option, default = "0x400000")]
    vk_version: u32,
}

fn main() -> VkResult<()> {
    let args: Args = argh::from_env();

    let group_count_x = args.group_count_x.unwrap_or(1);
    let group_count_y = args.group_count_y.unwrap_or(1);
    let group_count_z = args.group_count_z.unwrap_or(1);
    let width = 128;

    let extensions = ["VK_EXT_debug_utils".into()];

    let instance = UsamiInstance::new(
        "compute_tester",
        "usami",
        args.vk_version,
        &extensions,
        true,
    )?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &args.device_extension,
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

    let mut active_shaders = Vec::new();
    let mut shader_stage_create_infos = Vec::new();

    let shader_entrypoint_name = CString::new("main").unwrap();

    let shader_code = usami::utils::read_spv_file(args.compute_path);
    let shader = UsamiDevice::create_shader(&device, "compute_shader".into(), &shader_code)?;

    shader_stage_create_infos.push(
        PipelineShaderStageCreateInfo::default()
            .module(shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::COMPUTE),
    );
    active_shaders.push(shader);

    let input_desc_type = if args.input_as_buffer {
        vk::DescriptorType::STORAGE_BUFFER
    } else {
        vk::DescriptorType::UNIFORM_BUFFER
    };

    let descriptor_pool_sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: input_desc_type,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_IMAGE,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
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
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE),
        vk::DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_type(input_desc_type)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE),
        vk::DescriptorSetLayoutBinding::default()
            .binding(2)
            .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE),
        vk::DescriptorSetLayoutBinding::default()
            .binding(3)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE),
    ];
    let descriptor_set_layout = UsamiDevice::create_descriptor_set_layout(
        &device,
        "descriptor_set_layout".into(),
        DescriptorSetLayoutCreateInfo::default().bindings(&desc_layout_bindings),
    )?;

    let descriptor_sets = descriptor_pool
        .allocate_descriptor_sets("descriptor_set".into(), &[descriptor_set_layout.handle]);

    println!("{:?}", descriptor_sets.as_ref().err());

    let descriptor_sets = descriptor_sets?;

    let uniform_block_data = if let Some(input_buffer_file) = &args.input_buffer_file {
        std::fs::read(input_buffer_file).expect("Cannot read input buffer")
    } else {
        0x42u32.to_le_bytes().to_vec()
    };

    let uniform_block = UsamiDevice::create_buffer(
        &device,
        "uniform_block".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        if args.input_as_buffer {
            BufferUsageFlags::STORAGE_BUFFER
        } else {
            BufferUsageFlags::UNIFORM_BUFFER
        },
        &uniform_block_data,
    )?;


    let uniform_block2 = UsamiDevice::create_buffer(
        &device,
        "uniform_block2".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        if args.input_as_buffer {
            BufferUsageFlags::STORAGE_BUFFER
        } else {
            BufferUsageFlags::UNIFORM_BUFFER
        },
        &uniform_block_data,
    )?;

    let data_buffer = UsamiDevice::create_buffer_with_size(
        &device,
        "data_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::STORAGE_BUFFER,
        0x1000,
        MemoryPropertyFlags::HOST_VISIBLE,
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

    unsafe {
        device.handle.update_descriptor_sets(
            &[
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(0)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(&[DescriptorBufferInfo::default()
                        .buffer(data_buffer.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(1)
                    .descriptor_type(input_desc_type)
                    .buffer_info(&[DescriptorBufferInfo::default()
                        .buffer(uniform_block.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(2)
                    .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                    .image_info(&[DescriptorImageInfo::default()
                        .image_layout(ImageLayout::GENERAL)
                        .image_view(output_image_view.handle)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(3)
                    .descriptor_type(input_desc_type)
                    .buffer_info(&[DescriptorBufferInfo::default()
                        .buffer(uniform_block2.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)]),
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

    let compute_pipeline_create_info = ComputePipelineCreateInfo::default()
        .layout(pipeline_layout.handle)
        .stage(shader_stage_create_infos[0]);

    let pipelines = UsamiDevice::create_compute_pipelines(
        &device,
        "pipeline".into(),
        PipelineCache::null(),
        &[compute_pipeline_create_info],
    )?;

    let command_pool = UsamiDevice::create_command_pool(
        &device,
        "command_pool".into(),
        CommandPoolCreateInfo::default()
            .queue_family_index(device.vk_queue_index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
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
                vk_device.cmd_bind_descriptor_sets(
                    command_buffer.handle,
                    PipelineBindPoint::COMPUTE,
                    pipeline_layout.handle,
                    0,
                    &[descriptor_sets[0].handle],
                    &[],
                );

                vk_device.cmd_bind_pipeline(
                    command_buffer.handle,
                    PipelineBindPoint::COMPUTE,
                    pipeline.handle,
                );
            }

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
                ImageLayout::UNDEFINED,
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

    let data_buffer_readback = data_buffer.device_memory.read_to_vec().unwrap();

    if let Some(output_buffer_file) = args.output_buffer_file {
        std::fs::write(output_buffer_file, &data_buffer_readback).unwrap();
    }

    Ok(())
}

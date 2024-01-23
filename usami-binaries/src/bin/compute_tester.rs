use std::{ffi::CString, path::PathBuf};

use argh::FromArgs;
use ash::{
    prelude::VkResult,
    vk::{
        self, BufferCreateFlags, BufferUsageFlags, CommandBufferLevel, CommandBufferUsageFlags,
        CommandPoolCreateFlags, CommandPoolCreateInfo, ComputePipelineCreateInfo,
        DescriptorBufferInfo, DescriptorPoolCreateInfo, DescriptorSetLayoutCreateInfo,
        FenceCreateFlags, MemoryPropertyFlags, PipelineBindPoint, PipelineCache,
        PipelineShaderStageCreateInfo, QueueFlags, ShaderStageFlags, SharingMode, SubmitInfo,
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

    /// the path of the file to store the output buffer.
    #[argh(option)]
    output_buffer_file: Option<PathBuf>,
}

fn main() -> VkResult<()> {
    let args: Args = argh::from_env();

    let group_count_x = args.group_count_x.unwrap_or(1);
    let group_count_y = args.group_count_y.unwrap_or(1);
    let group_count_z = args.group_count_z.unwrap_or(1);

    let extensions = ["VK_EXT_debug_utils".into()];

    let instance = UsamiInstance::new(
        "compute_tester",
        "usami",
        vk::API_VERSION_1_0,
        &extensions,
        false,
    )?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[],
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
        PipelineShaderStageCreateInfo::builder()
            .module(shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::COMPUTE)
            .build(),
    );
    active_shaders.push(shader);

    let descriptor_pool_sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
        },
    ];

    let descriptor_pool_create_info = DescriptorPoolCreateInfo::builder()
        .pool_sizes(&descriptor_pool_sizes)
        .max_sets(1)
        .build();

    let descriptor_pool = UsamiDevice::create_descriptor_pool(
        &device,
        "descriptor_pool".into(),
        descriptor_pool_create_info,
    )?;

    let desc_layout_bindings = [
        vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE)
            .build(),
        vk::DescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::COMPUTE)
            .build(),
    ];
    let descriptor_set_layout = UsamiDevice::create_descriptor_set_layout(
        &device,
        "descriptor_set_layout".into(),
        DescriptorSetLayoutCreateInfo::builder()
            .bindings(&desc_layout_bindings)
            .build(),
    )?;

    let descriptor_sets = descriptor_pool
        .allocate_descriptor_sets("descriptor_set".into(), &[descriptor_set_layout.handle]);

    println!("{:?}", descriptor_sets.as_ref().err());

    let descriptor_sets = descriptor_sets?;

    let uniform_block = UsamiDevice::create_buffer(
        &device,
        "uniform_block".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::UNIFORM_BUFFER,
        &[0x42u32],
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

    // Break internally in panvk2 atm
    unsafe {
        device.handle.update_descriptor_sets(
            &[
                WriteDescriptorSet::builder()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(0)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(&[DescriptorBufferInfo::builder()
                        .buffer(data_buffer.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)
                        .build()])
                    .build(),
                WriteDescriptorSet::builder()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&[DescriptorBufferInfo::builder()
                        .buffer(uniform_block.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)
                        .build()])
                    .build(),
            ],
            &[],
        );
    }

    let pipeline_layout = UsamiDevice::create_pipeline_layout(
        &device,
        "base_pipeline_layout".into(),
        &[descriptor_set_layout.handle],
    )?;

    let compute_pipeline_create_info = ComputePipelineCreateInfo::builder()
        .layout(pipeline_layout.handle)
        .stage(shader_stage_create_infos[0])
        .build();

    let pipelines = UsamiDevice::create_compute_pipelines(
        &device,
        "pipeline".into(),
        PipelineCache::null(),
        &[compute_pipeline_create_info],
    )?;

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

            Ok(())
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

    let data_buffer_readback = data_buffer.device_memory.read_to_vec().unwrap();

    if let Some(output_buffer_file) = args.output_buffer_file {
        std::fs::write(output_buffer_file, &data_buffer_readback).unwrap();
    }

    Ok(())
}

use std::ffi::CString;

use ash::{
    extensions::ext::MeshShader,
    prelude::VkResult,
    vk::{
        self, AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, BufferCreateFlags, BufferUsageFlags, ClearValue,
        ColorComponentFlags, CommandBufferLevel, CommandPoolCreateFlags, CommandPoolCreateInfo,
        CompareOp, DescriptorBufferInfo, DescriptorPoolCreateInfo, DescriptorSetLayoutCreateInfo,
        DynamicState, FenceCreateFlags, FrontFace, GraphicsPipelineCreateInfo, ImageLayout,
        LogicOp, MemoryPropertyFlags, PhysicalDeviceType, PipelineBindPoint, PipelineCache,
        PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
        PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineStageFlags, PipelineViewportStateCreateInfo,
        PolygonMode, QueueFlags, RenderPassBeginInfo, RenderPassCreateInfo, SampleCountFlags,
        ShaderStageFlags, SharingMode, StencilOp, StencilOpState, SubmitInfo, SubpassContents,
        SubpassDependency, SubpassDescription, WriteDescriptorSet,
    },
};
use usami::{UsamiDevice, UsamiInstance, UsamiPresentation};

use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs)]
/// Reach new heights.
struct Args {
    /// the path to the task shader to use.
    #[argh(option)]
    task_path: Option<PathBuf>,

    /// the path to the mesh shader to use.
    #[argh(option)]
    mesh_path: PathBuf,

    /// the X size of the subgroup.
    #[argh(option)]
    group_count_x: Option<u32>,

    /// the Y size of the subgroup.
    #[argh(option)]
    group_count_y: Option<u32>,

    /// the Z size of the subgroup.
    #[argh(option)]
    group_count_z: Option<u32>,
}

fn main() -> VkResult<()> {
    let args: Args = argh::from_env();

    let extensions = ["VK_EXT_debug_utils".into()];

    let width = 400;
    let height = 400;

    let instance = UsamiInstance::new(
        "mesh_tester",
        "usami",
        vk::API_VERSION_1_2,
        &extensions,
        true,
    )?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &["VK_EXT_mesh_shader".into()],
        Box::new(|physical_device| {
            physical_device
                .queue_familiy_properties
                .iter()
                .enumerate()
                .find_map(|(i, x)| {
                    if physical_device.properties.device_type != PhysicalDeviceType::DISCRETE_GPU {
                        return None;
                    }

                    if x.queue_flags.contains(QueueFlags::GRAPHICS) {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
                .map(|x| (physical_device, x))
        }),
    )?;
    let presentation = UsamiPresentation::new(&device, width, height)?;

    let shader_entrypoint_name = CString::new("main").unwrap();

    let descriptor_pool_sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::UNIFORM_BUFFER,
        descriptor_count: 1,
    }];
    let descriptor_pool_create_info = DescriptorPoolCreateInfo::builder()
        .pool_sizes(&descriptor_pool_sizes)
        .max_sets(1)
        .build();

    let descriptor_pool = UsamiDevice::create_descriptor_pool(
        &device,
        "descriptor_pool".into(),
        descriptor_pool_create_info,
    )?;

    let desc_layout_bindings = [vk::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(ShaderStageFlags::MESH_EXT)
        .build()];
    let descriptor_set_layout = UsamiDevice::create_descriptor_set_layout(
        &device,
        "descriptor_set_layout".into(),
        DescriptorSetLayoutCreateInfo::builder()
            .bindings(&desc_layout_bindings)
            .build(),
    )?;

    let descriptor_sets = descriptor_pool
        .allocate_descriptor_sets("descriptor_set".into(), &[descriptor_set_layout.handle])?;

    let pipeline_layout = UsamiDevice::create_pipeline_layout(
        &device,
        "base_pipeline_layout".into(),
        &[descriptor_set_layout.handle],
    )?;

    let mut active_shaders = Vec::new();
    let mut shader_stage_create_infos = Vec::new();

    if let Some(task_shader_path) = &args.task_path {
        let shader_code = usami::utils::read_spv_file(task_shader_path);
        let shader = UsamiDevice::create_shader(&device, "task_shader".into(), &shader_code)?;

        shader_stage_create_infos.push(
            PipelineShaderStageCreateInfo::builder()
                .module(shader.handle)
                .name(shader_entrypoint_name.as_c_str())
                .stage(ShaderStageFlags::TASK_EXT)
                .build(),
        );
        active_shaders.push(shader);
    }

    {
        let shader_code = usami::utils::read_spv_file(&args.mesh_path);
        let shader = UsamiDevice::create_shader(&device, "mesh_shader".into(), &shader_code)?;

        shader_stage_create_infos.push(
            PipelineShaderStageCreateInfo::builder()
                .module(shader.handle)
                .name(shader_entrypoint_name.as_c_str())
                .stage(ShaderStageFlags::MESH_EXT)
                .build(),
        );
        active_shaders.push(shader);
    }

    {
        let shader_code =
            usami::utils::as_u32_vec(include_bytes!("../../resources/mesh_tester/main.frag.spv"));
        let shader = UsamiDevice::create_shader(&device, "mesh_shader".into(), &shader_code)?;

        shader_stage_create_infos.push(
            PipelineShaderStageCreateInfo::builder()
                .module(shader.handle)
                .name(shader_entrypoint_name.as_c_str())
                .stage(ShaderStageFlags::FRAGMENT)
                .build(),
        );
        active_shaders.push(shader);
    }

    let group_count_x = args.group_count_x.unwrap_or(1);
    let group_count_y = args.group_count_y.unwrap_or(1);
    let group_count_z = args.group_count_z.unwrap_or(1);

    let scissors = [presentation.rect2d()];
    let viewports = [presentation.viewport()];

    let viewport_state_create_info = PipelineViewportStateCreateInfo::builder()
        .scissors(&scissors)
        .viewports(&viewports)
        .build();

    let rasterization_create_info = PipelineRasterizationStateCreateInfo::builder()
        .front_face(FrontFace::COUNTER_CLOCKWISE)
        .polygon_mode(PolygonMode::FILL)
        .line_width(1.0)
        .build();

    let multisample_state_create_info = PipelineMultisampleStateCreateInfo::builder()
        .rasterization_samples(SampleCountFlags::TYPE_1)
        .build();

    let noop_stencil = StencilOpState::builder()
        .fail_op(StencilOp::KEEP)
        .pass_op(StencilOp::KEEP)
        .depth_fail_op(StencilOp::KEEP)
        .compare_op(CompareOp::ALWAYS)
        .build();

    let depth_state_create_info = PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(CompareOp::LESS_OR_EQUAL)
        .front(noop_stencil)
        .back(noop_stencil)
        .max_depth_bounds(1.0)
        .build();

    let attachements = [PipelineColorBlendAttachmentState::builder()
        .blend_enable(false)
        .src_color_blend_factor(BlendFactor::SRC_COLOR)
        .dst_color_blend_factor(BlendFactor::ONE_MINUS_DST_COLOR)
        .color_blend_op(BlendOp::ADD)
        .src_alpha_blend_factor(BlendFactor::ZERO)
        .dst_alpha_blend_factor(BlendFactor::ZERO)
        .alpha_blend_op(BlendOp::ADD)
        .color_write_mask(ColorComponentFlags::RGBA)
        .build()];

    let color_blend_create_state = PipelineColorBlendStateCreateInfo::builder()
        .logic_op(LogicOp::CLEAR)
        .attachments(&attachements)
        .build();

    let dynamic_state_create_info = PipelineDynamicStateCreateInfo::builder()
        .dynamic_states(&[DynamicState::VIEWPORT, DynamicState::SCISSOR])
        .build();

    let renderpass_attachments = [AttachmentDescription::builder()
        .format(presentation.image.create_info.format)
        .samples(presentation.image.create_info.samples)
        .load_op(AttachmentLoadOp::CLEAR)
        .store_op(AttachmentStoreOp::STORE)
        .final_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build()];
    let color_attachment_refs = [AttachmentReference::builder()
        .attachment(0)
        .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build()];

    let dependencies = [SubpassDependency::builder()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE)
        .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .build()];

    let renderpass_subpasses = [SubpassDescription::builder()
        .color_attachments(&color_attachment_refs)
        .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
        .build()];

    let render_pass_create_info = RenderPassCreateInfo::builder()
        .attachments(&renderpass_attachments)
        .subpasses(&renderpass_subpasses)
        .dependencies(&dependencies)
        .build();

    let render_pass =
        UsamiDevice::create_render_pass(&device, "render_pass".into(), render_pass_create_info)?;

    let graphics_pipeline_create_info = GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stage_create_infos)
        .viewport_state(&viewport_state_create_info)
        .rasterization_state(&rasterization_create_info)
        .multisample_state(&multisample_state_create_info)
        .depth_stencil_state(&depth_state_create_info)
        .color_blend_state(&color_blend_create_state)
        .dynamic_state(&dynamic_state_create_info)
        .layout(pipeline_layout.handle)
        .render_pass(render_pass.handle)
        .build();

    let pipelines = UsamiDevice::create_graphics_pipelines(
        &device,
        "pipeline".into(),
        PipelineCache::null(),
        &[graphics_pipeline_create_info],
    )?;

    let graphic_pipeline = &pipelines[0];

    let framebuffer =
        presentation.create_framebuffer(&device, "framebuffer".into(), &render_pass)?;

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

    let data_buffer = UsamiDevice::create_buffer_with_size(
        &device,
        "data_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::UNIFORM_BUFFER,
        0x1000,
        MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    unsafe {
        device.handle.update_descriptor_sets(
            &[WriteDescriptorSet::builder()
                .dst_set(descriptor_sets[0].handle)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&[DescriptorBufferInfo::builder()
                    .buffer(data_buffer.handle)
                    .offset(0)
                    .range(vk::WHOLE_SIZE)
                    .build()])
                .build()],
            &[],
        );
    }

    usami::utils::record_command_buffer_with_image_dep(
        &command_buffers[0],
        &presentation.image,
        &presentation.buffer_readback,
        |device, command_buffer, _image| {
            let vk_instance = &device.instance.vk_instance;
            let vk_device = &device.handle;
            let clear_values = [ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            }];

            let render_pass_begin_info = RenderPassBeginInfo::builder()
                .render_pass(render_pass.handle)
                .framebuffer(framebuffer.handle)
                .render_area(presentation.rect2d())
                .clear_values(&clear_values);

            unsafe {
                vk_device.cmd_begin_render_pass(
                    command_buffer.handle,
                    &render_pass_begin_info,
                    SubpassContents::INLINE,
                );
                vk_device.cmd_bind_descriptor_sets(
                    command_buffer.handle,
                    PipelineBindPoint::GRAPHICS,
                    pipeline_layout.handle,
                    0,
                    &[descriptor_sets[0].handle],
                    &[],
                );
                vk_device.cmd_bind_pipeline(
                    command_buffer.handle,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphic_pipeline.handle,
                );
                vk_device.cmd_set_viewport(command_buffer.handle, 0, &viewports);
                vk_device.cmd_set_scissor(command_buffer.handle, 0, &scissors);

                let mesh_shader_access = MeshShader::new(vk_instance, vk_device);

                mesh_shader_access.cmd_draw_mesh_tasks(
                    command_buffer.handle,
                    group_count_x,
                    group_count_y,
                    group_count_z,
                );
                vk_device.cmd_end_render_pass(command_buffer.handle);

                ImageLayout::COLOR_ATTACHMENT_OPTIMAL
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

    let data_buffer_readback = data_buffer.device_memory.read_to_vec();
    println!("{data_buffer_readback:?}");
    Ok(())
}

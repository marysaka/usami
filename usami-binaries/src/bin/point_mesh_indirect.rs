use std::ffi::CString;

use ash::{
    extensions::ext::MeshShader,
    prelude::VkResult,
    vk::{
        self, AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, BufferCreateFlags, BufferUsageFlags, ClearValue,
        ColorComponentFlags, CommandBufferLevel, CommandPoolCreateFlags, CommandPoolCreateInfo,
        CompareOp, DrawMeshTasksIndirectCommandEXT, DynamicState, FenceCreateFlags, FrontFace,
        GraphicsPipelineCreateInfo, ImageLayout, LogicOp, PhysicalDeviceType, PipelineBindPoint,
        PipelineCache, PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
        PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineStageFlags, PipelineViewportStateCreateInfo,
        PolygonMode, QueueFlags, RenderPassBeginInfo, RenderPassCreateInfo, SampleCountFlags,
        ShaderStageFlags, SharingMode, StencilOp, StencilOpState, SubmitInfo, SubpassContents,
        SubpassDependency, SubpassDescription,
    },
};
use usami::{UsamiDevice, UsamiInstance, UsamiPresentation};

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

    let instance = UsamiInstance::new(
        "triangle_mesh",
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

    let task_shader_code = usami::utils::as_u32_vec(include_bytes!(
        "../../resources/point_mesh_indirect/main.task.spv"
    ));
    let mesh_shader_code = usami::utils::as_u32_vec(include_bytes!(
        "../../resources/point_mesh_indirect/main.mesh.spv"
    ));
    let frag_shader_code: Vec<u32> = usami::utils::as_u32_vec(include_bytes!(
        "../../resources/point_mesh_indirect/main.frag.spv"
    ));

    let task_shader = UsamiDevice::create_shader(&device, "task_shader".into(), &task_shader_code)?;
    let mesh_shader = UsamiDevice::create_shader(&device, "mesh_shader".into(), &mesh_shader_code)?;
    let frag_shader = UsamiDevice::create_shader(&device, "frag_shader".into(), &frag_shader_code)?;
    let shader_entrypoint_name = CString::new("main").unwrap();

    let pipeline_layout =
        UsamiDevice::create_pipeline_layout(&device, "base_pipeline_layout".into(), &[], &[])?;

    let shader_stage_create_infos = [
        PipelineShaderStageCreateInfo::builder()
            .module(task_shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::TASK_EXT)
            .build(),
        PipelineShaderStageCreateInfo::builder()
            .module(mesh_shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::MESH_EXT)
            .build(),
        PipelineShaderStageCreateInfo::builder()
            .module(frag_shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::FRAGMENT)
            .build(),
    ];

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

    let draw_mesh_task_indirect_command = DrawMeshTasksIndirectCommandEXT::builder()
        .group_count_x(1)
        .group_count_y(1)
        .group_count_z(1)
        .build();
    let draw_mesh_task_indirect_command_buffer = UsamiDevice::create_buffer(
        &device,
        "draw_mesh_task_indirect_command_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::INDIRECT_BUFFER,
        &[draw_mesh_task_indirect_command],
    )?;

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
                vk_device.cmd_bind_pipeline(
                    command_buffer.handle,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphic_pipeline.handle,
                );
                vk_device.cmd_set_viewport(command_buffer.handle, 0, &viewports);
                vk_device.cmd_set_scissor(command_buffer.handle, 0, &scissors);

                let mesh_shader_access = MeshShader::new(vk_instance, vk_device);

                mesh_shader_access.cmd_draw_mesh_tasks_indirect(
                    command_buffer.handle,
                    draw_mesh_task_indirect_command_buffer.handle,
                    0,
                    1,
                    0,
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
    Ok(())
}

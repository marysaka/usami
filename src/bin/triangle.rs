use std::ffi::CString;

use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, BufferCreateFlags, BufferUsageFlags, ClearValue,
        ColorComponentFlags, CommandBufferLevel, CompareOp, DynamicState, Extent2D, Fence, Format,
        FramebufferCreateInfo, FrontFace, GraphicsPipelineCreateInfo, ImageLayout, LogicOp,
        PipelineBindPoint, PipelineCache, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo,
        PipelineDynamicStateCreateInfo, PipelineInputAssemblyStateCreateInfo,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineStageFlags, PipelineVertexInputStateCreateInfo,
        PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, QueueFlags, Rect2D,
        RenderPassBeginInfo, RenderPassCreateInfo, SampleCountFlags, ShaderStageFlags, SharingMode,
        StencilOp, StencilOpState, SubmitInfo, SubpassContents, SubpassDependency,
        SubpassDescription, VertexInputAttributeDescription, VertexInputBindingDescription,
        VertexInputRate, Viewport,
    },
};
use usami::{offset_of, UsamiDevice, UsamiInstance};

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
                    if x.queue_flags.contains(QueueFlags::GRAPHICS) {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
                .map(|x| (physical_device, x))
        }),
    )?;

    let index_buffer_data = [0u32, 1, 2];
    let index_buffer = device.create_buffer(
        "index_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::INDEX_BUFFER,
        &index_buffer_data,
    )?;

    let vertices = [
        Vertex {
            pos: [-1.0, 1.0, 0.0, 1.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            pos: [1.0, 1.0, 0.0, 1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            pos: [0.0, -1.0, 0.0, 1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
    ];

    let vbo_buffer = device.create_buffer(
        "vbo_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::VERTEX_BUFFER,
        &vertices,
    )?;

    let vertex_shader_code =
        usami::utils::as_u32_vec(include_bytes!("../../resources/triangle/main.vert.spv"));
    let frag_shader_code =
        usami::utils::as_u32_vec(include_bytes!("../../resources/triangle/main.frag.spv"));

    let vertex_shader = device.create_shader("vertex_shader".into(), &vertex_shader_code)?;
    let frag_shader = device.create_shader("frag_shader".into(), &frag_shader_code)?;
    let shader_entrypoint_name = CString::new("main").unwrap();

    let pipeline_layout = device.create_pipeline_layout("base_pipeline_layout".into())?;

    let shader_stage_create_infos = [
        PipelineShaderStageCreateInfo::builder()
            .module(vertex_shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::VERTEX)
            .build(),
        PipelineShaderStageCreateInfo::builder()
            .module(frag_shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::FRAGMENT)
            .build(),
    ];

    let vertex_input_binding_descriptions = [VertexInputBindingDescription::builder()
        .binding(0)
        .stride(std::mem::size_of::<Vertex>() as u32)
        .input_rate(VertexInputRate::VERTEX)
        .build()];

    let vertex_input_attribute_descriptions = [
        VertexInputAttributeDescription::builder()
            .location(0)
            .binding(0)
            .offset(offset_of!(Vertex, pos) as u32)
            .format(Format::R32G32B32A32_SFLOAT)
            .build(),
        VertexInputAttributeDescription::builder()
            .location(1)
            .binding(0)
            .offset(offset_of!(Vertex, color) as u32)
            .format(Format::R32G32B32A32_SFLOAT)
            .build(),
    ];

    let vertex_input_state_create_info = PipelineVertexInputStateCreateInfo::builder()
        .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
        .vertex_binding_descriptions(&vertex_input_binding_descriptions)
        .build();

    let vertex_input_assembly_state_create_info = PipelineInputAssemblyStateCreateInfo::builder()
        .topology(PrimitiveTopology::TRIANGLE_LIST)
        .build();

    let scissors = [Rect2D::builder()
        .extent(Extent2D {
            width: device.width,
            height: device.height,
        })
        .build()];

    let viewports = [Viewport {
        x: 0.0,
        y: 0.0,
        width: device.width as f32,
        height: device.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    }];

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
        .format(device.presentation_image().create_info.format)
        .samples(device.presentation_image().create_info.samples)
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

    let render_pass = device.create_render_pass("render_pass".into(), render_pass_create_info)?;

    let graphics_pipeline_create_info = GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stage_create_infos)
        .vertex_input_state(&vertex_input_state_create_info)
        .input_assembly_state(&vertex_input_assembly_state_create_info)
        .viewport_state(&viewport_state_create_info)
        .rasterization_state(&rasterization_create_info)
        .multisample_state(&multisample_state_create_info)
        .depth_stencil_state(&depth_state_create_info)
        .color_blend_state(&color_blend_create_state)
        .dynamic_state(&dynamic_state_create_info)
        .layout(pipeline_layout.handle)
        .render_pass(render_pass.handle)
        .build();

    let pipelines = device.create_graphics_pipelines(
        "pipeline".into(),
        PipelineCache::null(),
        &[graphics_pipeline_create_info],
    )?;

    let graphic_pipeline = &pipelines[0];

    let framebuffer = device.create_framebuffer(
        "framebuffer".into(),
        FramebufferCreateInfo::builder()
            .render_pass(render_pass.handle)
            .attachments(&[device.presentation_image_view().handle])
            .width(device.width)
            .height(device.height)
            .layers(1)
            .build(),
    )?;

    let command_buffers = device.command_pool.allocate_command_buffers(
        &device,
        "command_buffer".into(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    usami::utils::record_command_buffer_with_image_dep(
        &device,
        command_buffers[0].handle,
        device.presentation_image(),
        |device, command_buffer, _image| {
            let vk_device = &device.handle;
            let clear_values = [ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            }];

            let render_pass_begin_info = RenderPassBeginInfo::builder()
                .render_pass(render_pass.handle)
                .framebuffer(framebuffer.handle)
                .render_area(
                    Rect2D::builder()
                        .extent(Extent2D {
                            width: device.width,
                            height: device.height,
                        })
                        .build(),
                )
                .clear_values(&clear_values);

            unsafe {
                vk_device.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin_info,
                    SubpassContents::INLINE,
                );
                vk_device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphic_pipeline.handle,
                );
                vk_device.cmd_set_viewport(command_buffer, 0, &viewports);
                vk_device.cmd_set_scissor(command_buffer, 0, &scissors);
                vk_device.cmd_bind_vertex_buffers(command_buffer, 0, &[vbo_buffer.handle], &[0]);
                vk_device.cmd_bind_index_buffer(
                    command_buffer,
                    index_buffer.handle,
                    0,
                    vk::IndexType::UINT32,
                );
                vk_device.cmd_draw_indexed(
                    command_buffer,
                    index_buffer_data.len() as u32,
                    1,
                    0,
                    0,
                    1,
                );
                // Or draw without the index buffer
                // vk_device.cmd_draw(command_buffer, 3, 1, 0, 0);
                vk_device.cmd_end_render_pass(command_buffer);

                ImageLayout::COLOR_ATTACHMENT_OPTIMAL
            }
        },
    )?;

    unsafe {
        device.handle.queue_submit(
            device.vk_queue,
            &[SubmitInfo::builder()
                .command_buffers(&[command_buffers[0].handle])
                .build()],
            Fence::null(),
        )?;

        device.handle.device_wait_idle()?;
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

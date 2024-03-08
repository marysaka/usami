use std::ffi::CString;

use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, BufferCreateFlags, BufferUsageFlags,
        ClearColorValue, ClearValue, ColorComponentFlags, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPoolCreateFlags, CommandPoolCreateInfo, CompareOp,
        ConditionalRenderingBeginInfoEXT, FenceCreateFlags, Format, FrontFace,
        GraphicsPipelineCreateInfo, ImageLayout, LogicOp, MemoryPropertyFlags, PhysicalDeviceType,
        PipelineBindPoint, PipelineCache, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineMultisampleStateCreateInfo,
        PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateInfo, PipelineStageFlags,
        PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode,
        PrimitiveTopology, PushConstantRange, QueryControlFlags, QueryPoolCreateInfo,
        QueryResultFlags, QueryType, QueueFlags, RenderPassBeginInfo, RenderPassCreateInfo,
        SampleCountFlags, ShaderStageFlags, SharingMode, StencilOp, StencilOpState, SubmitInfo,
        SubpassContents, SubpassDescription, VertexInputAttributeDescription,
        VertexInputBindingDescription, VertexInputRate,
    },
};
use usami::{offset_of, UsamiDevice, UsamiInstance, UsamiPresentation};
use usami_binaries::ash_ext::{ConditionalRendering, TransformFeedback};

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct VertexElementData {
    pos: [f32; 4],
    color: [f32; 4],
    vertex_index: i32,
}

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let width = 256;
    let height = 256;

    let instance = UsamiInstance::new(
        "xfb_cond_render",
        "usami",
        vk::API_VERSION_1_3,
        &extensions,
        true,
    )?;

    let device = UsamiDevice::new_by_filter(
        instance,
        &[
            "VK_EXT_conditional_rendering".into(),
            "VK_EXT_transform_feedback".into(),
            "VK_EXT_host_query_reset".into(),
        ],
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

    let xfb = TransformFeedback::new(&device.instance.vk_entry, &device.instance.vk_instance);
    let cond_render =
        ConditionalRendering::new(&device.instance.vk_entry, &device.instance.vk_instance);
    let presentation = UsamiPresentation::new(&device, width, height)?;
    let rgba_blue = [0.0, 0.0, 1.0, 1.0];
    let rgba_black = [0.0, 0.0, 0.0, 1.0];
    let rgba_red = [1.0, 0.0, 0.0, 1.0];
    let vertices = [
        VertexElementData {
            pos: [-0.3, 0.3, 0.5, 1.0],
            color: rgba_blue,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [-0.3, -0.3, 0.5, 1.0],
            color: rgba_blue,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [0.3, 0.3, 0.5, 1.0],
            color: rgba_blue,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [-0.3, -0.3, 0.5, 1.0],
            color: rgba_blue,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [0.3, 0.3, 0.5, 1.0],
            color: rgba_blue,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [0.3, -0.3, 0.5, 1.0],
            color: rgba_blue,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [-0.3, 0.3, 0.5, 1.0],
            color: rgba_black,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [-0.3, -0.3, 0.5, 1.0],
            color: rgba_black,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [0.3, 0.3, 0.5, 1.0],
            color: rgba_black,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [-0.3, -0.3, 0.5, 1.0],
            color: rgba_black,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [0.3, 0.3, 0.5, 1.0],
            color: rgba_black,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [0.3, -0.3, 0.5, 1.0],
            color: rgba_black,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [5.3, 6.3, 0.5, 1.0],
            color: rgba_red,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [5.3, 5.3, 0.5, 1.0],
            color: rgba_red,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [6.3, 6.3, 0.5, 1.0],
            color: rgba_red,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [5.3, 5.3, 0.5, 1.0],
            color: rgba_red,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [6.3, 6.3, 0.5, 1.0],
            color: rgba_red,
            vertex_index: 0,
        },
        VertexElementData {
            pos: [6.3, 5.3, 0.5, 1.0],
            color: rgba_red,
            vertex_index: 0,
        },
    ];

    let vbo_buffer = UsamiDevice::create_buffer(
        &device,
        "vbo_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::VERTEX_BUFFER,
        &vertices,
    )?;

    let vertex_shader_code = usami::utils::as_u32_vec(include_bytes!(
        "../../resources/xfb_cond_render/vertex_fetch.vert.spv"
    ));
    let frag_shader_code = usami::utils::as_u32_vec(include_bytes!(
        "../../resources/xfb_cond_render/vertex_fetch.frag.spv"
    ));
    let geom_shader_code = usami::utils::as_u32_vec(include_bytes!(
        "../../resources/xfb_cond_render/vertex_fetch_write_point.geom.spv"
    ));

    let vertex_shader =
        UsamiDevice::create_shader(&device, "vertex_fetch.vert".into(), &vertex_shader_code)?;
    let frag_shader =
        UsamiDevice::create_shader(&device, "vertex_fetch.frag".into(), &frag_shader_code)?;
    let geom_shader =
        UsamiDevice::create_shader(&device, "vertex_fetch.geom".into(), &geom_shader_code)?;
    let shader_entrypoint_name = CString::new("main").unwrap();

    let stream_pipeline_layout = UsamiDevice::create_pipeline_layout(
        &device,
        "stream_pipeline_layout".into(),
        &[],
        &[PushConstantRange::builder()
            .stage_flags(ShaderStageFlags::GEOMETRY)
            .offset(0)
            .size(std::mem::size_of::<u32>() as u32)
            .build()],
    )?;

    let basic_pipeline_layout =
        UsamiDevice::create_pipeline_layout(&device, "basic_pipeline_layout".into(), &[], &[])?;

    let vertex_input_binding_descriptions = [VertexInputBindingDescription::builder()
        .binding(0)
        .stride(std::mem::size_of::<VertexElementData>() as u32)
        .input_rate(VertexInputRate::VERTEX)
        .build()];

    let vertex_input_attribute_descriptions = [
        VertexInputAttributeDescription::builder()
            .location(0)
            .binding(0)
            .offset(offset_of!(VertexElementData, pos) as u32)
            .format(Format::R32G32B32A32_SFLOAT)
            .build(),
        VertexInputAttributeDescription::builder()
            .location(1)
            .binding(0)
            .offset(offset_of!(VertexElementData, color) as u32)
            .format(Format::R32G32B32A32_SFLOAT)
            .build(),
        VertexInputAttributeDescription::builder()
            .location(3)
            .binding(0)
            .offset(offset_of!(VertexElementData, vertex_index) as u32)
            .format(Format::R32_SINT)
            .build(),
    ];

    let vertex_input_state_create_info = PipelineVertexInputStateCreateInfo::builder()
        .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
        .vertex_binding_descriptions(&vertex_input_binding_descriptions)
        .build();

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
        .fail_op(StencilOp::REPLACE)
        .pass_op(StencilOp::REPLACE)
        .depth_fail_op(StencilOp::REPLACE)
        .compare_op(CompareOp::ALWAYS)
        .compare_mask(u32::MAX)
        .write_mask(u32::MAX)
        .reference(0)
        .build();

    let depth_state_create_info = PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable(false)
        .depth_write_enable(false)
        .depth_compare_op(CompareOp::ALWAYS)
        .front(noop_stencil)
        .back(noop_stencil)
        .min_depth_bounds(0.0)
        .max_depth_bounds(1.0)
        .build();

    let attachements = [PipelineColorBlendAttachmentState::builder()
        .blend_enable(false)
        .src_color_blend_factor(BlendFactor::SRC_COLOR)
        .dst_color_blend_factor(BlendFactor::DST_COLOR)
        .color_blend_op(BlendOp::ADD)
        .src_alpha_blend_factor(BlendFactor::SRC_COLOR)
        .dst_alpha_blend_factor(BlendFactor::DST_COLOR)
        .alpha_blend_op(BlendOp::ADD)
        .color_write_mask(ColorComponentFlags::RGBA)
        .build()];

    let color_blend_create_state = PipelineColorBlendStateCreateInfo::builder()
        .logic_op(LogicOp::COPY)
        .attachments(&attachements)
        .build();

    let renderpass_attachments = [AttachmentDescription::builder()
        .format(presentation.image.create_info.format)
        .samples(presentation.image.create_info.samples)
        .load_op(AttachmentLoadOp::LOAD)
        .store_op(AttachmentStoreOp::STORE)
        .stencil_load_op(AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(AttachmentStoreOp::STORE)
        .initial_layout(ImageLayout::GENERAL)
        .final_layout(ImageLayout::GENERAL)
        .build()];
    let color_attachment_refs = [AttachmentReference::builder()
        .attachment(0)
        .layout(ImageLayout::GENERAL)
        .build()];

    let renderpass_subpasses = [SubpassDescription::builder()
        .color_attachments(&color_attachment_refs)
        .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
        .build()];

    let render_pass_create_info = RenderPassCreateInfo::builder()
        .attachments(&renderpass_attachments)
        .subpasses(&renderpass_subpasses)
        .build();

    let render_pass =
        UsamiDevice::create_render_pass(&device, "render_pass".into(), render_pass_create_info)?;

    let stream_pipeline_create_info = GraphicsPipelineCreateInfo::builder()
        .stages(&[
            PipelineShaderStageCreateInfo::builder()
                .module(vertex_shader.handle)
                .name(shader_entrypoint_name.as_c_str())
                .stage(ShaderStageFlags::VERTEX)
                .build(),
            PipelineShaderStageCreateInfo::builder()
                .module(geom_shader.handle)
                .name(shader_entrypoint_name.as_c_str())
                .stage(ShaderStageFlags::GEOMETRY)
                .build(),
        ])
        .vertex_input_state(&vertex_input_state_create_info)
        .input_assembly_state(
            &PipelineInputAssemblyStateCreateInfo::builder()
                .topology(PrimitiveTopology::POINT_LIST)
                .build(),
        )
        .viewport_state(&viewport_state_create_info)
        .rasterization_state(&rasterization_create_info)
        .multisample_state(&multisample_state_create_info)
        .depth_stencil_state(&depth_state_create_info)
        .color_blend_state(&color_blend_create_state)
        .layout(stream_pipeline_layout.handle)
        .render_pass(render_pass.handle)
        .build();

    let basic_pipeline_create_info = GraphicsPipelineCreateInfo::builder()
        .stages(&[
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
        ])
        .vertex_input_state(&vertex_input_state_create_info)
        .input_assembly_state(
            &PipelineInputAssemblyStateCreateInfo::builder()
                .topology(PrimitiveTopology::TRIANGLE_LIST)
                .build(),
        )
        .viewport_state(&viewport_state_create_info)
        .rasterization_state(&rasterization_create_info)
        .multisample_state(&multisample_state_create_info)
        .depth_stencil_state(&depth_state_create_info)
        .color_blend_state(&color_blend_create_state)
        .layout(basic_pipeline_layout.handle)
        .render_pass(render_pass.handle)
        .build();

    let pipelines = UsamiDevice::create_graphics_pipelines(
        &device,
        "stream_pipeline".into(),
        PipelineCache::null(),
        &[stream_pipeline_create_info, basic_pipeline_create_info],
    )?;

    let stream_pipeline = &pipelines[0];
    let basic_pipeline = &pipelines[1];

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
    let command_buffer = &command_buffers[0];

    const XFB_STREAM_COUNT: usize = 4;
    const XFB_ENTRY_COUNT: usize = 6;
    const XFB_ENTRY_SIZE: usize = std::mem::size_of::<f32>();
    const XFB_RAW_SIZE: usize = XFB_ENTRY_SIZE * XFB_ENTRY_COUNT;

    let query_pool_handle = unsafe {
        device.handle.create_query_pool(
            &QueryPoolCreateInfo::builder()
                .query_type(QueryType::OCCLUSION)
                .query_count(2)
                .build(),
            None,
        )
    }?;

    let query_buffer_size = 2 * std::mem::size_of::<u32>() as vk::DeviceSize;
    let query_buffer = UsamiDevice::create_buffer_with_size(
        &device,
        "query_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::TRANSFER_DST | BufferUsageFlags::CONDITIONAL_RENDERING_EXT,
        query_buffer_size,
        MemoryPropertyFlags::HOST_VISIBLE,
    )?;
    let xfb_buffer = UsamiDevice::create_buffer(
        &device,
        "xfb_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::TRANSFORM_FEEDBACK_BUFFER_EXT,
        &[0.0f32; XFB_ENTRY_COUNT * XFB_STREAM_COUNT],
    )?;

    let record_draw = |device: &ash::Device, index: u32| unsafe {
        device.cmd_draw(command_buffer.handle, 6, 1, 6 * index, 0);
    };

    command_buffer.record(
        CommandBufferUsageFlags::empty(),
        |device, command_buffer| {
            command_buffer.clear_image(&presentation.image, 0.0, 0.0, 0.0, 0.0)?;

            let vk_device = &device.handle;
            unsafe {
                vk_device.cmd_reset_query_pool(command_buffer.handle, query_pool_handle, 0, 2);

                let clear_values = [ClearValue {
                    color: ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                }];

                let render_pass_begin_info = RenderPassBeginInfo::builder()
                    .render_pass(render_pass.handle)
                    .framebuffer(framebuffer.handle)
                    .render_area(presentation.rect2d())
                    .clear_values(&clear_values);

                vk_device.cmd_begin_render_pass(
                    command_buffer.handle,
                    &render_pass_begin_info,
                    SubpassContents::INLINE,
                );
                vk_device.cmd_bind_vertex_buffers(
                    command_buffer.handle,
                    0,
                    &[vbo_buffer.handle],
                    &[0],
                );
                vk_device.cmd_bind_pipeline(
                    command_buffer.handle,
                    vk::PipelineBindPoint::GRAPHICS,
                    basic_pipeline.handle,
                );

                vk_device.cmd_begin_query(
                    command_buffer.handle,
                    query_pool_handle,
                    0,
                    QueryControlFlags::empty(),
                );
                record_draw(vk_device, 2);
                vk_device.cmd_end_query(command_buffer.handle, query_pool_handle, 0);

                vk_device.cmd_begin_query(
                    command_buffer.handle,
                    query_pool_handle,
                    1,
                    QueryControlFlags::empty(),
                );
                record_draw(vk_device, 1);
                vk_device.cmd_end_query(command_buffer.handle, query_pool_handle, 1);

                vk_device.cmd_end_render_pass(command_buffer.handle);
                vk_device.cmd_copy_query_pool_results(
                    command_buffer.handle,
                    query_pool_handle,
                    0,
                    2,
                    query_buffer.handle,
                    0,
                    std::mem::size_of::<u32>() as vk::DeviceSize,
                    QueryResultFlags::WAIT,
                );

                command_buffer.add_buffer_barrier(
                    &query_buffer,
                    PipelineStageFlags::TRANSFER,
                    PipelineStageFlags::CONDITIONAL_RENDERING_EXT,
                    AccessFlags::TRANSFER_WRITE,
                    AccessFlags::CONDITIONAL_RENDERING_READ_EXT,
                    0,
                    query_buffer_size,
                )?;

                vk_device.cmd_begin_render_pass(
                    command_buffer.handle,
                    &render_pass_begin_info,
                    SubpassContents::INLINE,
                );
                vk_device.cmd_bind_pipeline(
                    command_buffer.handle,
                    vk::PipelineBindPoint::GRAPHICS,
                    stream_pipeline.handle,
                );

                let mut conditional_rendering_info = ConditionalRenderingBeginInfoEXT::builder()
                    .buffer(query_buffer.handle)
                    .offset(std::mem::size_of::<u32>() as vk::DeviceSize)
                    .build();

                for xfb_stream in 0u32..XFB_STREAM_COUNT as u32 {
                    let xfb_offset = xfb_stream * XFB_RAW_SIZE as u32;

                    xfb.bind_transform_feedback_buffers(
                        command_buffer.handle,
                        xfb_stream,
                        &[xfb_buffer.handle],
                        &[xfb_offset as vk::DeviceSize],
                        &[XFB_RAW_SIZE as vk::DeviceSize],
                    );
                    vk_device.cmd_push_constants(
                        command_buffer.handle,
                        stream_pipeline_layout.handle,
                        ShaderStageFlags::GEOMETRY,
                        0,
                        &xfb_stream.to_le_bytes(),
                    );

                    conditional_rendering_info.offset =
                        ((std::mem::size_of::<u32>() as u32) * (xfb_stream % 2)) as vk::DeviceSize;
                    cond_render.begin_conditional_rendering(
                        command_buffer.handle,
                        &conditional_rendering_info,
                    );
                    xfb.begin_transform_feedback(command_buffer.handle, 0, &[], &[]);
                    record_draw(vk_device, 1);
                    xfb.end_transform_feedback(command_buffer.handle, 0, &[], &[]);
                    cond_render.end_conditional_rendering(command_buffer.handle);
                }

                vk_device.cmd_end_render_pass(command_buffer.handle);
            }

            command_buffer.add_memory_barrier(
                PipelineStageFlags::TRANSFORM_FEEDBACK_EXT,
                PipelineStageFlags::HOST,
                AccessFlags::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT,
                AccessFlags::HOST_READ,
            )?;

            Ok(())
        },
    )?;

    let fence = UsamiDevice::create_fence(&device, "fence".into(), FenceCreateFlags::empty())?;
    let queue = UsamiDevice::get_device_queue(&device, "queue".into(), device.vk_queue_index, 0)?;

    queue.submit(
        &[SubmitInfo::builder()
            .command_buffers(&[command_buffer.handle])
            .build()],
        &fence,
    )?;
    fence.wait(u64::MAX)?;
    fence.reset()?;

    let xfb_buffer_readback = xfb_buffer.device_memory.read_to_vec().unwrap();
    std::fs::write("output.hex", &xfb_buffer_readback).unwrap();

    let query_bufer_readback = query_buffer.device_memory.read_to_vec().unwrap();

    println!("query_results_raw = {query_bufer_readback:?}");
    for query_index in 0..2 {
        let value = u32::from_le_bytes(
            query_bufer_readback[query_index * 4..(query_index + 1) * 4]
                .try_into()
                .unwrap(),
        );

        println!("query_results[{query_index}] = {value}");
    }

    for stream in 0..XFB_STREAM_COUNT {
        let start = stream * XFB_RAW_SIZE;

        let mut values = [0.0; XFB_ENTRY_COUNT];

        for (i, value) in values.iter_mut().enumerate() {
            *value = f32::from_le_bytes(
                xfb_buffer_readback
                    [start + (i * XFB_ENTRY_SIZE)..start + ((i + 1) * XFB_ENTRY_SIZE)]
                    .try_into()
                    .unwrap(),
            );
        }

        println!("xfb_stream[{stream}] = {values:?}");
    }

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

    unsafe { device.handle.destroy_query_pool(query_pool_handle, None) };
    Ok(())
}

use std::ffi::CString;

use ash::{
    prelude::VkResult,
    vk::{
        self, AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentReference,
        AttachmentStoreOp, BlendFactor, BlendOp, BorderColor, BufferCreateFlags, BufferUsageFlags,
        ClearValue, ColorComponentFlags, CommandBufferLevel, CommandPoolCreateFlags,
        CommandPoolCreateInfo, CompareOp, ComponentMapping, ComponentSwizzle, DescriptorImageInfo,
        DescriptorPoolCreateInfo, DescriptorSetLayoutCreateInfo, DescriptorType, DynamicState,
        FenceCreateFlags, Filter, Format, FrontFace, GraphicsPipelineCreateInfo, ImageAspectFlags,
        ImageLayout, ImageSubresourceRange, ImageUsageFlags, ImageViewCreateFlags, ImageViewType,
        IndexType, LogicOp, PipelineBindPoint, PipelineCache, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo,
        PipelineDynamicStateCreateInfo, PipelineInputAssemblyStateCreateInfo,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineStageFlags, PipelineVertexInputStateCreateInfo,
        PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, QueueFlags,
        RenderPassBeginInfo, RenderPassCreateInfo, SampleCountFlags, SamplerAddressMode,
        SamplerCreateInfo, SamplerMipmapMode, ShaderStageFlags, SharingMode, StencilOp,
        StencilOpState, SubmitInfo, SubpassContents, SubpassDependency, SubpassDescription,
        VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate,
        WriteDescriptorSet,
    },
};
use usami::{offset_of, UsamiDevice, UsamiInstance, UsamiPresentation};

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct Vertex {
    pos: [f32; 4],
    uv: [f32; 2],
}

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let width = 1920;
    let height = 1080;

    let instance = UsamiInstance::new("triangle", "usami", vk::API_VERSION_1_1, &extensions, true)?;
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
    let presentation = UsamiPresentation::new(&device, width, height)?;

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
    let white_image = UsamiDevice::import_image(
        &device,
        &command_pool,
        "white_image".into(),
        &white_image_buffer.into(),
        ImageUsageFlags::SAMPLED,
        ImageLayout::GENERAL,
    )?;

    let index_buffer_data = [0u32, 1, 2, 2, 3, 0];
    let index_buffer = UsamiDevice::create_buffer(
        &device,
        "index_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::INDEX_BUFFER,
        &index_buffer_data,
    )?;

    let vertices = [
        Vertex {
            pos: [-1.0, -1.0, 0.0, 1.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            pos: [-1.0, 1.0, 0.0, 1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [1.0, 1.0, 0.0, 1.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [1.0, -1.0, 0.0, 1.0],
            uv: [1.0, 0.0],
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

    let vertex_shader_code =
        usami::utils::as_u32_vec(include_bytes!("../../resources/texture/main.vert.spv"));
    let frag_shader_code =
        usami::utils::as_u32_vec(include_bytes!("../../resources/texture/main.frag.spv"));

    let vertex_shader =
        UsamiDevice::create_shader(&device, "vertex_shader".into(), &vertex_shader_code)?;
    let frag_shader = UsamiDevice::create_shader(&device, "frag_shader".into(), &frag_shader_code)?;
    let shader_entrypoint_name = CString::new("main").unwrap();

    let descriptor_pool_sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: 1,
    }];
    let descriptor_pool_create_info = DescriptorPoolCreateInfo::default()
        .pool_sizes(&descriptor_pool_sizes)
        .max_sets(1);

    let descriptor_pool = UsamiDevice::create_descriptor_pool(
        &device,
        "descriptor_pool".into(),
        descriptor_pool_create_info,
    )?;

    let desc_layout_bindings = [vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(ShaderStageFlags::FRAGMENT)];
    let descriptor_set_layout = UsamiDevice::create_descriptor_set_layout(
        &device,
        "descriptor_set_layout".into(),
        DescriptorSetLayoutCreateInfo::default().bindings(&desc_layout_bindings),
    )?;

    let descriptor_sets = descriptor_pool
        .allocate_descriptor_sets("descriptor_set".into(), &[descriptor_set_layout.handle])?;

    let white_image_view = white_image.create_simple_image_view(
        "presentation_image_view".into(),
        ImageViewType::TYPE_2D,
        ImageSubresourceRange::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1),
        ComponentMapping::default()
            .r(ComponentSwizzle::IDENTITY)
            .g(ComponentSwizzle::IDENTITY)
            .b(ComponentSwizzle::IDENTITY)
            .a(ComponentSwizzle::IDENTITY),
        ImageViewCreateFlags::empty(),
    )?;

    let sampler = UsamiDevice::create_sampler(
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

    unsafe {
        device.handle.update_descriptor_sets(
            &[WriteDescriptorSet::default()
                .dst_set(descriptor_sets[0].handle)
                .dst_binding(0)
                .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(&[DescriptorImageInfo::default()
                    .image_layout(ImageLayout::GENERAL)
                    .image_view(white_image_view.handle)
                    .sampler(sampler.handle)])],
            &[],
        );
    }

    let pipeline_layout = UsamiDevice::create_pipeline_layout(
        &device,
        "base_pipeline_layout".into(),
        &[descriptor_set_layout.handle],
        &[],
    )?;

    let shader_stage_create_infos = [
        PipelineShaderStageCreateInfo::default()
            .module(vertex_shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::VERTEX),
        PipelineShaderStageCreateInfo::default()
            .module(frag_shader.handle)
            .name(shader_entrypoint_name.as_c_str())
            .stage(ShaderStageFlags::FRAGMENT),
    ];

    let vertex_input_binding_descriptions = [VertexInputBindingDescription::default()
        .binding(0)
        .stride(std::mem::size_of::<Vertex>() as u32)
        .input_rate(VertexInputRate::VERTEX)];

    let vertex_input_attribute_descriptions = [
        VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .offset(offset_of!(Vertex, pos) as u32)
            .format(Format::R32G32B32A32_SFLOAT),
        VertexInputAttributeDescription::default()
            .location(1)
            .binding(0)
            .offset(offset_of!(Vertex, uv) as u32)
            .format(Format::R32G32_SFLOAT),
    ];

    let vertex_input_state_create_info = PipelineVertexInputStateCreateInfo::default()
        .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
        .vertex_binding_descriptions(&vertex_input_binding_descriptions);

    let vertex_input_assembly_state_create_info =
        PipelineInputAssemblyStateCreateInfo::default().topology(PrimitiveTopology::TRIANGLE_LIST);

    let scissors = [presentation.rect2d()];
    let viewports = [presentation.viewport()];

    let viewport_state_create_info = PipelineViewportStateCreateInfo::default()
        .scissors(&scissors)
        .viewports(&viewports);

    let rasterization_create_info = PipelineRasterizationStateCreateInfo::default()
        .front_face(FrontFace::COUNTER_CLOCKWISE)
        .polygon_mode(PolygonMode::FILL)
        .line_width(1.0);

    let multisample_state_create_info = PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(SampleCountFlags::TYPE_1);

    let noop_stencil = StencilOpState::default()
        .fail_op(StencilOp::KEEP)
        .pass_op(StencilOp::KEEP)
        .depth_fail_op(StencilOp::KEEP)
        .compare_op(CompareOp::ALWAYS);

    let depth_state_create_info = PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(CompareOp::LESS_OR_EQUAL)
        .front(noop_stencil)
        .back(noop_stencil)
        .max_depth_bounds(1.0);

    let attachements = [PipelineColorBlendAttachmentState::default()
        .blend_enable(false)
        .src_color_blend_factor(BlendFactor::SRC_COLOR)
        .dst_color_blend_factor(BlendFactor::ONE_MINUS_DST_COLOR)
        .color_blend_op(BlendOp::ADD)
        .src_alpha_blend_factor(BlendFactor::ZERO)
        .dst_alpha_blend_factor(BlendFactor::ZERO)
        .alpha_blend_op(BlendOp::ADD)
        .color_write_mask(ColorComponentFlags::RGBA)];

    let color_blend_create_state = PipelineColorBlendStateCreateInfo::default()
        .logic_op(LogicOp::CLEAR)
        .attachments(&attachements);

    let dynamic_state_create_info = PipelineDynamicStateCreateInfo::default()
        .dynamic_states(&[DynamicState::VIEWPORT, DynamicState::SCISSOR]);

    let renderpass_attachments = [AttachmentDescription::default()
        .format(presentation.image.format)
        .samples(presentation.image.samples)
        .load_op(AttachmentLoadOp::CLEAR)
        .store_op(AttachmentStoreOp::STORE)
        .final_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let color_attachment_refs = [AttachmentReference::default()
        .attachment(0)
        .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

    let dependencies = [SubpassDependency::default()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE)
        .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)];

    let renderpass_subpasses = [SubpassDescription::default()
        .color_attachments(&color_attachment_refs)
        .pipeline_bind_point(PipelineBindPoint::GRAPHICS)];

    let render_pass_create_info = RenderPassCreateInfo::default()
        .attachments(&renderpass_attachments)
        .subpasses(&renderpass_subpasses)
        .dependencies(&dependencies);

    let render_pass =
        UsamiDevice::create_render_pass(&device, "render_pass".into(), render_pass_create_info)?;

    let graphics_pipeline_create_info = GraphicsPipelineCreateInfo::default()
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
        .render_pass(render_pass.handle);

    let pipelines = UsamiDevice::create_graphics_pipelines(
        &device,
        "pipeline".into(),
        PipelineCache::null(),
        &[graphics_pipeline_create_info],
    )?;

    let graphic_pipeline = &pipelines[0];

    let framebuffer =
        presentation.create_framebuffer(&device, "framebuffer".into(), &render_pass)?;

    let command_buffers = command_pool.allocate_command_buffers(
        "command_buffer".into(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    usami::utils::record_command_buffer_with_image_dep(
        &command_buffers[0],
        &presentation.image,
        &presentation.buffer_readback,
        |device, command_buffer, _image| {
            let vk_device = &device.handle;
            let clear_values = [ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            }];

            let render_pass_begin_info = RenderPassBeginInfo::default()
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
                    PipelineBindPoint::GRAPHICS,
                    graphic_pipeline.handle,
                );
                vk_device.cmd_set_viewport(command_buffer.handle, 0, &viewports);
                vk_device.cmd_set_scissor(command_buffer.handle, 0, &scissors);
                vk_device.cmd_bind_vertex_buffers(
                    command_buffer.handle,
                    0,
                    &[vbo_buffer.handle],
                    &[0],
                );
                vk_device.cmd_bind_index_buffer(
                    command_buffer.handle,
                    index_buffer.handle,
                    0,
                    IndexType::UINT32,
                );
                vk_device.cmd_draw_indexed(
                    command_buffer.handle,
                    index_buffer_data.len() as u32,
                    1,
                    0,
                    0,
                    1,
                );
                vk_device.cmd_end_render_pass(command_buffer.handle);

                ImageLayout::COLOR_ATTACHMENT_OPTIMAL
            }
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

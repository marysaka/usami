use std::sync::Arc;

use ash::{
    self,
    prelude::VkResult,
    vk::{
        self, BorderColor, BufferCreateFlags, BufferUsageFlags, BufferViewCreateFlags, CompareOp,
        ComponentMapping, ComponentSwizzle, DescriptorBufferInfo, DescriptorImageInfo,
        DescriptorPoolCreateInfo, DescriptorPoolSize, DescriptorSetLayoutBinding,
        DescriptorSetLayoutCreateInfo, DescriptorType, Extent3D, Filter, Format, ImageAspectFlags,
        ImageCreateInfo, ImageLayout, ImageSubresourceRange, ImageTiling, ImageType,
        ImageUsageFlags, ImageViewCreateFlags, ImageViewType, MemoryPropertyFlags,
        SampleCountFlags, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode,
        ShaderStageFlags, SharingMode, WriteDescriptorSet, API_VERSION_1_1,
    },
};
use usami::{
    descriptor::UsamiDescriptorSetLayout, UsamiDescriptorPool, UsamiDevice, UsamiInstance,
};

fn create_simple_descriptor_pool(
    device: &Arc<UsamiDevice>,
    name: &str,
    desc_types: &[DescriptorType],
    descriptor_count_per_type: u32,
    max_sets: u32,
) -> VkResult<UsamiDescriptorPool> {
    let mut descriptor_pool_sizes = Vec::new();

    for desc_type in desc_types {
        descriptor_pool_sizes.push(DescriptorPoolSize {
            ty: *desc_type,
            descriptor_count: descriptor_count_per_type,
        });
    }

    UsamiDevice::create_descriptor_pool(
        &device,
        name.into(),
        DescriptorPoolCreateInfo::default()
            .pool_sizes(&descriptor_pool_sizes)
            .max_sets(max_sets),
    )
}

fn create_simple_descriptor_set_layout(
    device: &Arc<UsamiDevice>,
    name: &str,
    desc_types: &[DescriptorType],
    stage_flags: ShaderStageFlags,
) -> VkResult<UsamiDescriptorSetLayout> {
    let mut desc_layout_bindings = Vec::new();

    for (binding, desc_type) in desc_types.iter().enumerate() {
        let stage_flags = if *desc_type == DescriptorType::INPUT_ATTACHMENT {
            ShaderStageFlags::FRAGMENT
        } else {
            stage_flags
        };

        desc_layout_bindings.push(
            DescriptorSetLayoutBinding::default()
                .binding(binding as u32)
                .descriptor_type(*desc_type)
                .descriptor_count(1)
                .stage_flags(stage_flags),
        );
    }

    UsamiDevice::create_descriptor_set_layout(
        &device,
        name.into(),
        DescriptorSetLayoutCreateInfo::default().bindings(&desc_layout_bindings),
    )
}

fn get_dst_binding(desc_types: &[DescriptorType], t: DescriptorType) -> u32 {
    desc_types.binary_search(&t).unwrap() as u32
}

fn test_all_types(device: &Arc<UsamiDevice>) -> VkResult<()> {
    let pool_desc_types = [
        DescriptorType::SAMPLER,
        DescriptorType::COMBINED_IMAGE_SAMPLER,
        DescriptorType::SAMPLED_IMAGE,
        DescriptorType::STORAGE_IMAGE,
        DescriptorType::UNIFORM_TEXEL_BUFFER,
        DescriptorType::STORAGE_TEXEL_BUFFER,
        DescriptorType::UNIFORM_BUFFER,
        DescriptorType::STORAGE_BUFFER,
        // TODO: update
        DescriptorType::UNIFORM_BUFFER_DYNAMIC,
        // TODO: update
        DescriptorType::STORAGE_BUFFER_DYNAMIC,
        DescriptorType::INPUT_ATTACHMENT,
    ];

    let pool = create_simple_descriptor_pool(&device, "pool_1", &pool_desc_types, 1, 1)?;
    let descriptor_set_layout_1 = create_simple_descriptor_set_layout(
        &device,
        "descriptor_set_layout_1",
        &pool_desc_types,
        ShaderStageFlags::ALL,
    )?;

    let descriptor_sets =
        pool.allocate_descriptor_sets("descriptor_set".into(), &[descriptor_set_layout_1.handle])?;

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

    let format = Format::R8G8B8A8_UNORM;

    let image = UsamiDevice::create_image(
        device,
        "image".into(),
        ImageCreateInfo::default()
            .image_type(ImageType::TYPE_2D)
            .format(format)
            .extent(Extent3D {
                width: 128,
                height: 128,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(
                ImageUsageFlags::TRANSFER_SRC
                    | ImageUsageFlags::TRANSFER_DST
                    | ImageUsageFlags::SAMPLED
                    | ImageUsageFlags::STORAGE
                    | ImageUsageFlags::INPUT_ATTACHMENT,
            ),
        MemoryPropertyFlags::empty(),
    )?;

    let image_view = image.create_simple_image_view(
        "image_view".into(),
        ImageViewType::TYPE_2D,
        ImageSubresourceRange::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(image.mip_levels)
            .base_array_layer(0)
            .layer_count(image.array_layers),
        ComponentMapping::default()
            .r(ComponentSwizzle::IDENTITY)
            .g(ComponentSwizzle::IDENTITY)
            .b(ComponentSwizzle::IDENTITY)
            .a(ComponentSwizzle::IDENTITY),
        ImageViewCreateFlags::empty(),
    )?;

    let uniform_buffer = UsamiDevice::create_buffer_with_size(
        &device,
        "uniform_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::UNIFORM_BUFFER | BufferUsageFlags::UNIFORM_TEXEL_BUFFER,
        0x1000,
        MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let uniform_buffer_view = uniform_buffer.create_view(
        "uniform_buffer_view".into(),
        BufferViewCreateFlags::empty(),
        format,
        0,
        vk::WHOLE_SIZE,
    )?;

    let storage_buffer = UsamiDevice::create_buffer_with_size(
        &device,
        "storage_buffer".into(),
        BufferCreateFlags::empty(),
        SharingMode::EXCLUSIVE,
        BufferUsageFlags::STORAGE_BUFFER | BufferUsageFlags::STORAGE_TEXEL_BUFFER,
        0x1000,
        MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let storage_buffer_view = storage_buffer.create_view(
        "uniform_buffer_view".into(),
        BufferViewCreateFlags::empty(),
        Format::R8G8B8A8_UNORM,
        0,
        vk::WHOLE_SIZE,
    )?;

    unsafe {
        device.handle.update_descriptor_sets(
            &[
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(&pool_desc_types, DescriptorType::SAMPLER))
                    .descriptor_type(DescriptorType::SAMPLER)
                    .image_info(&[DescriptorImageInfo::default().sampler(sampler.handle)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::COMBINED_IMAGE_SAMPLER,
                    ))
                    .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&[DescriptorImageInfo::default()
                        .image_layout(ImageLayout::GENERAL)
                        .image_view(image_view.handle)
                        .sampler(sampler.handle)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::SAMPLED_IMAGE,
                    ))
                    .descriptor_type(DescriptorType::SAMPLED_IMAGE)
                    .image_info(&[DescriptorImageInfo::default()
                        .image_layout(ImageLayout::GENERAL)
                        .image_view(image_view.handle)
                        .sampler(Sampler::null())]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::STORAGE_IMAGE,
                    ))
                    .descriptor_type(DescriptorType::STORAGE_IMAGE)
                    .image_info(&[DescriptorImageInfo::default()
                        .image_layout(ImageLayout::GENERAL)
                        .image_view(image_view.handle)
                        .sampler(Sampler::null())]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::UNIFORM_TEXEL_BUFFER,
                    ))
                    .descriptor_type(DescriptorType::UNIFORM_TEXEL_BUFFER)
                    .texel_buffer_view(&[uniform_buffer_view.handle]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::STORAGE_TEXEL_BUFFER,
                    ))
                    .descriptor_type(DescriptorType::STORAGE_TEXEL_BUFFER)
                    .texel_buffer_view(&[storage_buffer_view.handle]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::UNIFORM_BUFFER,
                    ))
                    .descriptor_type(DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(&[DescriptorBufferInfo::default()
                        .buffer(uniform_buffer.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::STORAGE_BUFFER,
                    ))
                    .descriptor_type(DescriptorType::STORAGE_BUFFER)
                    .buffer_info(&[DescriptorBufferInfo::default()
                        .buffer(storage_buffer.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                    ))
                    .descriptor_type(DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                    .buffer_info(&[DescriptorBufferInfo::default()
                        .buffer(uniform_buffer.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::STORAGE_BUFFER_DYNAMIC,
                    ))
                    .descriptor_type(DescriptorType::STORAGE_BUFFER_DYNAMIC)
                    .buffer_info(&[DescriptorBufferInfo::default()
                        .buffer(storage_buffer.handle)
                        .offset(0)
                        .range(vk::WHOLE_SIZE)]),
                WriteDescriptorSet::default()
                    .dst_set(descriptor_sets[0].handle)
                    .dst_binding(get_dst_binding(
                        &pool_desc_types,
                        DescriptorType::INPUT_ATTACHMENT,
                    ))
                    .descriptor_type(DescriptorType::INPUT_ATTACHMENT)
                    .image_info(&[DescriptorImageInfo::default()
                        .image_layout(ImageLayout::GENERAL)
                        .image_view(image_view.handle)
                        .sampler(Sampler::null())]),
            ],
            &[],
        );
    }

    // FIXME: Right now panvk use a common DestroyBufferView entrypoint that is wrong and cause a crash on drop.
    std::mem::forget(uniform_buffer_view);
    std::mem::forget(storage_buffer_view);

    Ok(())
}

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let instance = UsamiInstance::new(
        "desc_set_simple",
        "usami",
        API_VERSION_1_1,
        &extensions,
        true,
    )?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[],
        Box::new(|physical_device| {
            physical_device
                .queue_familiy_properties
                .iter()
                .enumerate()
                .find_map(|(i, _)| Some(i as u32))
                .map(|x| (physical_device, x))
        }),
    )?;

    test_all_types(&device)?;

    Ok(())
}

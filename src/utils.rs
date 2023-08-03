use ash::{
    prelude::VkResult,
    vk::{
        AccessFlags, BufferImageCopy, CommandBufferLevel, CommandBufferUsageFlags, Extent2D,
        Extent3D, FenceCreateFlags, Format, ImageAspectFlags, ImageLayout, ImageSubresourceLayers,
        PipelineStageFlags, SubmitInfo,
    },
};
use image::{EncodableLayout, ImageBuffer, RgbaImage};

use crate::{
    image::{RawImageData, RawImageLevelInfo},
    UsamiBuffer, UsamiCommandBuffer, UsamiDevice, UsamiImage,
};

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
        }
    }};
}

pub fn as_u32_vec(data: &[u8]) -> Vec<u32> {
    let mut res = Vec::new();

    let size_aligned = (data.len() + 3) / 4;

    res.resize(size_aligned, 0);

    unsafe {
        let temp_slice =
            std::slice::from_raw_parts_mut(res.as_mut_ptr() as *mut u8, size_aligned * 4);

        temp_slice[..data.len()].copy_from_slice(data);
    }

    res
}

/// Generic function to turn any Sized slice to a slice of u8.
///
/// # Safety
///
/// Follow requirements of [std::slice::from_raw_parts].
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn get_format_size(format: Format) -> u32 {
    match format {
        Format::R8_UNORM => 1,
        Format::R8G8_UNORM => 2,
        Format::R8G8B8_UNORM => 3,
        Format::R8G8B8A8_UNORM => 4,
        _ => todo!("Unknown format size for {}", format.as_raw()),
    }
}

pub fn compute_mip_pyramid_levels(width: u32, height: u32) -> u32 {
    let max = (std::cmp::max(width, height)) as f32;

    max.log2().floor() as u32 + 1
}

pub fn create_gradient_image_rgba(width: u32, height: u32) -> RgbaImage {
    let grad = colorgrad::turbo();
    let mut buffer = ImageBuffer::new(width, height);

    for (x, _, pixel) in buffer.enumerate_pixels_mut() {
        let rgba = grad.at(x as f64 / width as f64).to_rgba8();
        *pixel = image::Rgba(rgba);
    }

    buffer
}

#[derive(Clone, Copy, Debug)]
pub struct GradientImageLevelInfo {
    pub extent: Extent2D,
    pub start_position: usize,
}

pub fn create_gradient_image_with_mip_levels(
    width: u32,
    height: u32,
    mip_levels: u32,
) -> RawImageData {
    let mut data = Vec::new();
    let mut level_infos = Vec::new();

    for level in 0..mip_levels {
        let level_width = std::cmp::max(width >> level, 1);
        let level_height = std::cmp::max(height >> level, 1);

        level_infos.push(RawImageLevelInfo {
            extent: Extent2D {
                width: level_width,
                height: level_height,
            },
            start_position: data.len(),
        });
        data.extend_from_slice(create_gradient_image_rgba(level_width, level_height).as_bytes());
    }

    RawImageData::new(Format::R8G8B8A8_UNORM, data, level_infos)
}

pub fn record_command_buffer_with_image_dep<
    F: Fn(&UsamiDevice, &UsamiCommandBuffer, &UsamiImage) -> ImageLayout,
>(
    device: &UsamiDevice,
    command_buffer: &UsamiCommandBuffer,
    image: &UsamiImage,
    buffer: &UsamiBuffer,
    callback: F,
) -> VkResult<()> {
    command_buffer.record(
        device,
        CommandBufferUsageFlags::SIMULTANEOUS_USE,
        |device, command_buffer| {
            let old_image_layout = callback(device, command_buffer, image);

            command_buffer.copy_image_to_buffer(
                image,
                buffer,
                &[BufferImageCopy::builder()
                    .image_subresource(
                        ImageSubresourceLayers::builder()
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .mip_level(0)
                            .layer_count(1)
                            .build(),
                    )
                    .image_extent(Extent3D {
                        width: device.width,
                        height: device.height,
                        depth: 1,
                    })
                    .build()],
                AccessFlags::COLOR_ATTACHMENT_WRITE,
                old_image_layout,
                1,
                1,
                ImageAspectFlags::COLOR,
                PipelineStageFlags::ALL_COMMANDS,
            )?;

            Ok(())
        },
    )
}

pub fn record_and_execute_command_buffer<
    F: Fn(&UsamiDevice, &UsamiCommandBuffer) -> VkResult<()>,
>(
    device: &UsamiDevice,
    command_buffer_name: String,
    callback: F,
) -> VkResult<()> {
    let command_buffers = device.command_pool.allocate_command_buffers(
        device,
        command_buffer_name.clone(),
        CommandBufferLevel::PRIMARY,
        1,
    )?;

    let command_buffer = &command_buffers[0];

    command_buffer.record(device, CommandBufferUsageFlags::ONE_TIME_SUBMIT, callback)?;

    let fence = device.create_fence(
        format!("{command_buffer_name}_fence"),
        FenceCreateFlags::empty(),
    )?;

    device.get_queue()?.submit(
        &[SubmitInfo::builder()
            .command_buffers(&[command_buffer.handle])
            .build()],
        &fence,
    )?;
    fence.wait(u64::MAX)?;
    fence.reset()?;

    Ok(())
}

use ash::{
    prelude::VkResult,
    vk::{
        AccessFlags, CommandBufferLevel, CommandBufferUsageFlags, Extent2D, FenceCreateFlags,
        ImageAspectFlags, ImageLayout, PipelineStageFlags, SubmitInfo,
    },
};

use crate::{UsamiBuffer, UsamiCommandBuffer, UsamiDevice, UsamiImage};

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
                Extent2D {
                    width: device.width,
                    height: device.height,
                },
                AccessFlags::COLOR_ATTACHMENT_WRITE,
                old_image_layout,
                1,
                ImageAspectFlags::COLOR,
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

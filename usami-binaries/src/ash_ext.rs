use ash::prelude::VkResult;
use ash::vk;

use ash::ext::conditional_rendering::Device as ConditionalRendering;
use ash::ext::transform_feedback::Device as TransformFeedback;
use ash::nv::cooperative_matrix::Instance as NvCooperativeMatrix;

// Copy paste from ash as it's private
pub(crate) unsafe fn read_into_defaulted_vector<
    N: Copy + Default + TryInto<usize>,
    T: Default + Clone,
>(
    f: impl Fn(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: std::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, std::ptr::null_mut()).result()?;
        let mut data =
            vec![Default::default(); count.try_into().expect("`N` failed to convert to `usize`")];

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            break err_code.set_vec_len_on_success(
                data,
                count.try_into().expect("`N` failed to convert to `usize`"),
            );
        }
    }
}

// Missing extensions definition goes here for now
// TODO: upstream this

#[inline]
pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
    instance: &NvCooperativeMatrix,
    physical_device: vk::PhysicalDevice,
) -> VkResult<Vec<vk::CooperativeMatrixPropertiesNV<'_>>> {
    read_into_defaulted_vector(|count, data| {
        (instance
            .fp()
            .get_physical_device_cooperative_matrix_properties_nv)(
            physical_device, count, data
        )
    })
}

#[inline]
pub unsafe fn begin_conditional_rendering(
    instance: &ConditionalRendering,
    command_buffer: vk::CommandBuffer,
    conditional_rendering_begin: &vk::ConditionalRenderingBeginInfoEXT,
) {
    (instance.fp().cmd_begin_conditional_rendering_ext)(
        command_buffer,
        conditional_rendering_begin,
    );
}

#[inline]
pub unsafe fn end_conditional_rendering(
    instance: &ConditionalRendering,
    command_buffer: vk::CommandBuffer,
) {
    (instance.fp().cmd_end_conditional_rendering_ext)(command_buffer);
}

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html>
#[inline]
pub unsafe fn begin_query_indexed(
    instance: &TransformFeedback,
    command_buffer: vk::CommandBuffer,
    query_pool: vk::QueryPool,
    query: u32,
    flags: vk::QueryControlFlags,
    index: u32,
) {
    (instance.fp().cmd_begin_query_indexed_ext)(command_buffer, query_pool, query, flags, index);
}

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html>
#[inline]
pub unsafe fn begin_transform_feedback(
    instance: &TransformFeedback,
    command_buffer: vk::CommandBuffer,
    first_counter_buffer: u32,
    counter_buffers: &[vk::Buffer],
    counter_buffer_offsets: &[vk::DeviceSize],
) {
    assert_eq!(counter_buffers.len(), counter_buffer_offsets.len());

    (instance.fp().cmd_begin_transform_feedback_ext)(
        command_buffer,
        first_counter_buffer,
        counter_buffers.len() as u32,
        counter_buffers.as_ptr(),
        counter_buffer_offsets.as_ptr(),
    );
}

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html>
#[inline]
pub unsafe fn bind_transform_feedback_buffers(
    instance: &TransformFeedback,
    command_buffer: vk::CommandBuffer,
    first_binding: u32,
    buffers: &[vk::Buffer],
    offsets: &[vk::DeviceSize],
    sizes: &[vk::DeviceSize],
) {
    assert_eq!(buffers.len(), offsets.len());
    assert_eq!(offsets.len(), sizes.len());

    (instance.fp().cmd_bind_transform_feedback_buffers_ext)(
        command_buffer,
        first_binding,
        buffers.len() as u32,
        buffers.as_ptr(),
        offsets.as_ptr(),
        sizes.as_ptr(),
    );
}

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html>
#[inline]
pub unsafe fn draw_indirect_byte_count(
    instance: &TransformFeedback,
    command_buffer: vk::CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: vk::Buffer,
    counter_buffer_offset: vk::DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
) {
    (instance.fp().cmd_draw_indirect_byte_count_ext)(
        command_buffer,
        instance_count,
        first_instance,
        counter_buffer,
        counter_buffer_offset,
        counter_offset,
        vertex_stride,
    );
}

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html>
#[inline]
pub unsafe fn end_query_indexed(
    instance: &TransformFeedback,
    command_buffer: vk::CommandBuffer,
    query_pool: vk::QueryPool,
    query: u32,
    index: u32,
) {
    (instance.fp().cmd_end_query_indexed_ext)(command_buffer, query_pool, query, index);
}

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html>
#[inline]
pub unsafe fn end_transform_feedback(
    instance: &TransformFeedback,
    command_buffer: vk::CommandBuffer,
    first_counter_buffer: u32,
    counter_buffers: &[vk::Buffer],
    counter_buffer_offsets: &[vk::DeviceSize],
) {
    assert_eq!(counter_buffers.len(), counter_buffer_offsets.len());

    (instance.fp().cmd_end_transform_feedback_ext)(
        command_buffer,
        first_counter_buffer,
        counter_buffers.len() as u32,
        counter_buffers.as_ptr(),
        counter_buffer_offsets.as_ptr(),
    );
}

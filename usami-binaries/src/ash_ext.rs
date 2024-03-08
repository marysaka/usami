use ash::{vk, Entry, Instance};
use std::ffi::CStr;
use std::mem;

// Missing extensions definition goes here for now
// TODO: upstream this

#[derive(Clone)]
pub struct TransformFeedback {
    handle: vk::Instance,
    fp: vk::ExtTransformFeedbackFn,
}

impl TransformFeedback {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ExtTransformFeedbackFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html>
    #[inline]
    pub unsafe fn begin_query_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
        index: u32,
    ) {
        (self.fp.cmd_begin_query_indexed_ext)(command_buffer, query_pool, query, flags, index);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html>
    #[inline]
    pub unsafe fn begin_transform_feedback(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[vk::Buffer],
        counter_buffer_offsets: &[vk::DeviceSize],
    ) {
        assert_eq!(counter_buffers.len(), counter_buffer_offsets.len());

        (self.fp.cmd_begin_transform_feedback_ext)(
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
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        buffers: &[vk::Buffer],
        offsets: &[vk::DeviceSize],
        sizes: &[vk::DeviceSize],
    ) {
        assert_eq!(buffers.len(), offsets.len());
        assert_eq!(offsets.len(), sizes.len());

        (self.fp.cmd_bind_transform_feedback_buffers_ext)(
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
        &self,
        command_buffer: vk::CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: vk::Buffer,
        counter_buffer_offset: vk::DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        (self.fp.cmd_draw_indirect_byte_count_ext)(
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
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        index: u32,
    ) {
        (self.fp.cmd_end_query_indexed_ext)(command_buffer, query_pool, query, index);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html>
    #[inline]
    pub unsafe fn end_transform_feedback(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[vk::Buffer],
        counter_buffer_offsets: &[vk::DeviceSize],
    ) {
        assert_eq!(counter_buffers.len(), counter_buffer_offsets.len());

        (self.fp.cmd_end_transform_feedback_ext)(
            command_buffer,
            first_counter_buffer,
            counter_buffers.len() as u32,
            counter_buffers.as_ptr(),
            counter_buffer_offsets.as_ptr(),
        );
    }

    #[inline]
    pub const fn name() -> &'static CStr {
        vk::ExtTransformFeedbackFn::name()
    }

    #[inline]
    pub fn fp(&self) -> &vk::ExtTransformFeedbackFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}

#[derive(Clone)]
pub struct ConditionalRendering {
    handle: vk::Instance,
    fp: vk::ExtConditionalRenderingFn,
}

impl ConditionalRendering {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ExtConditionalRenderingFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html>
    #[inline]
    pub unsafe fn begin_conditional_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        conditional_rendering_begin: &vk::ConditionalRenderingBeginInfoEXT,
    ) {
        (self.fp.cmd_begin_conditional_rendering_ext)(command_buffer, conditional_rendering_begin);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html>
    #[inline]
    pub unsafe fn end_conditional_rendering(&self, command_buffer: vk::CommandBuffer) {
        (self.fp.cmd_end_conditional_rendering_ext)(command_buffer);
    }

    #[inline]
    pub const fn name() -> &'static CStr {
        vk::ExtConditionalRenderingFn::name()
    }

    #[inline]
    pub fn fp(&self) -> &vk::ExtConditionalRenderingFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}

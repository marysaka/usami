use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{
        ComputePipelineCreateInfo, DescriptorSetLayout, GraphicsPipelineCreateInfo, Pipeline, PipelineCache, PipelineLayout, PipelineLayoutCreateFlags,
        PipelineLayoutCreateInfo, PushConstantRange,
    },
};

use crate::UsamiDevice;

pub struct UsamiPipelineLayout {
    device: Arc<UsamiDevice>,
    pub handle: PipelineLayout,
}

impl UsamiPipelineLayout {
    pub fn new(
        device: &Arc<UsamiDevice>,
        set_layouts: &[DescriptorSetLayout],
        push_constant_ranges: &[PushConstantRange],
    ) -> VkResult<Self> {
        let create_info = PipelineLayoutCreateInfo::default()
            .set_layouts(set_layouts)
            .push_constant_ranges(push_constant_ranges)
            .flags(PipelineLayoutCreateFlags::empty());

        let handle = unsafe { device.handle.create_pipeline_layout(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiPipelineLayout {
    fn drop(&mut self) {
        unsafe {
            self.device
                .handle
                .destroy_pipeline_layout(self.handle, None)
        }
    }
}

pub struct UsamiPipeline {
    device: Arc<UsamiDevice>,
    pub handle: Pipeline,
}

impl UsamiPipeline {
    pub fn new_graphics(
        device: &Arc<UsamiDevice>,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
    ) -> VkResult<Vec<Self>> {
        let result = unsafe {
            device
                .handle
                .create_graphics_pipelines(pipeline_cache, create_infos, None)
                .map_err(|(_, x)| x)?
        };

        Ok(result
            .iter()
            .map(|handle| Self {
                device: device.clone(),
                handle: *handle,
            })
            .collect())
    }

    pub fn new_compute(
        device: &Arc<UsamiDevice>,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo],
    ) -> VkResult<Vec<Self>> {
        let result = unsafe {
            device
                .handle
                .create_compute_pipelines(pipeline_cache, create_infos, None)
                .map_err(|(_, x)| x)?
        };

        Ok(result
            .iter()
            .map(|handle| Self {
                device: device.clone(),
                handle: *handle,
            })
            .collect())
    }
}

impl Drop for UsamiPipeline {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_pipeline(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_pipeline_layout(
        device: &Arc<UsamiDevice>,
        name: String,
        set_layouts: &[DescriptorSetLayout],
        push_constant_ranges: &[PushConstantRange],
    ) -> VkResult<UsamiPipelineLayout> {
        let pipeline_layout = UsamiPipelineLayout::new(device, set_layouts, push_constant_ranges)?;

        device.set_debug_name(name, pipeline_layout.handle)?;

        Ok(pipeline_layout)
    }

    pub fn create_compute_pipelines(
        device: &Arc<UsamiDevice>,
        name: String,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo],
    ) -> VkResult<Vec<UsamiPipeline>> {
        let pipelines = UsamiPipeline::new_compute(device, pipeline_cache, create_infos)?;

        for (idx, pipeline) in pipelines.iter().enumerate() {
            device.set_debug_name(format!("{name}_{idx}"), pipeline.handle)?;
        }

        Ok(pipelines)
    }

    pub fn create_graphics_pipelines(
        device: &Arc<UsamiDevice>,
        name: String,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
    ) -> VkResult<Vec<UsamiPipeline>> {
        let pipelines = UsamiPipeline::new_graphics(device, pipeline_cache, create_infos)?;

        for (idx, pipeline) in pipelines.iter().enumerate() {
            device.set_debug_name(format!("{name}_{idx}"), pipeline.handle)?;
        }

        Ok(pipelines)
    }
}

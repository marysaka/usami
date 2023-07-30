use ash::{
    prelude::*,
    vk::{
        GraphicsPipelineCreateInfo, Handle, ObjectType, Pipeline, PipelineCache, PipelineLayout,
        PipelineLayoutCreateFlags, PipelineLayoutCreateInfo,
    },
    Device,
};

use crate::UsamiDevice;

pub struct UsamiPipelineLayout {
    device: Device,
    pub handle: PipelineLayout,
}

impl UsamiPipelineLayout {
    pub fn new(device: &UsamiDevice) -> VkResult<Self> {
        let create_info = PipelineLayoutCreateInfo::builder()
            .flags(PipelineLayoutCreateFlags::empty())
            .build();

        let handle = unsafe {
            device
                .vk_device
                .create_pipeline_layout(&create_info, None)?
        };

        Ok(Self {
            device: device.vk_device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiPipelineLayout {
    fn drop(&mut self) {
        unsafe { self.device.destroy_pipeline_layout(self.handle, None) }
    }
}

pub struct UsamiGraphicsPipeline {
    device: Device,
    pub handle: Pipeline,
}

impl UsamiGraphicsPipeline {
    pub fn new(
        device: &UsamiDevice,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
    ) -> VkResult<Vec<Self>> {
        let result = unsafe {
            device
                .vk_device
                .create_graphics_pipelines(pipeline_cache, create_infos, None)
                .map_err(|(_, x)| x)?
        };

        Ok(result
            .iter()
            .map(|handle| Self {
                device: device.vk_device.clone(),
                handle: *handle,
            })
            .collect())
    }
}

impl Drop for UsamiGraphicsPipeline {
    fn drop(&mut self) {
        unsafe { self.device.destroy_pipeline(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_pipeline_layout(&self, name: String) -> VkResult<UsamiPipelineLayout> {
        let pipeline_layout = UsamiPipelineLayout::new(self)?;

        self.set_debug_name(
            name,
            pipeline_layout.handle.as_raw(),
            ObjectType::PIPELINE_LAYOUT,
        )?;

        Ok(pipeline_layout)
    }

    pub fn create_graphics_pipelines(
        &self,
        name: String,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
    ) -> VkResult<Vec<UsamiGraphicsPipeline>> {
        let pipelines = UsamiGraphicsPipeline::new(self, pipeline_cache, create_infos)?;

        for (idx, pipeline) in pipelines.iter().enumerate() {
            self.set_debug_name(
                format!("{name}_{idx}"),
                pipeline.handle.as_raw(),
                ObjectType::PIPELINE,
            )?;
        }

        Ok(pipelines)
    }
}

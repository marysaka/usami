use std::sync::Arc;

use ash::{
    prelude::*,
    vk::{
        BufferCreateFlags, BufferImageCopy, BufferUsageFlags, ComponentMapping, Extent3D, Format,
        Image, ImageAspectFlags, ImageCreateInfo, ImageLayout, ImageSubresourceLayers,
        ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView,
        ImageViewCreateFlags, ImageViewCreateInfo, ImageViewType, MemoryPropertyFlags,
        SampleCountFlags, SharingMode,
    },
    Device,
};
use image::{RgbImage, RgbaImage};

use crate::{utils, UsamiCommandPool, UsamiDevice, UsamiDeviceMemory};

pub struct UsamiImage {
    device: Arc<UsamiDevice>,
    pub extent: Extent3D,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub array_layers: u32,
    pub mip_levels: u32,
    pub handle: Image,
    pub device_memory: UsamiDeviceMemory,
}

impl UsamiImage {
    pub fn new(
        device: &Arc<UsamiDevice>,
        create_info: ImageCreateInfo,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<Self> {
        let vk_device: &Device = &device.handle;

        let handle = unsafe { vk_device.create_image(&create_info, None)? };
        let req = unsafe { vk_device.get_image_memory_requirements(handle) };
        let device_memory = UsamiDeviceMemory::new(device, req, memory_flags)?;
        unsafe {
            vk_device.bind_image_memory(handle, device_memory.handle, 0)?;
        }

        Ok(Self {
            device: device.clone(),
            extent: create_info.extent,
            format: create_info.format,
            samples: create_info.samples,
            array_layers: create_info.array_layers,
            mip_levels: create_info.mip_levels,
            handle,
            device_memory,
        })
    }

    pub fn create_simple_image_view(
        &self,
        name: String,
        view_type: ImageViewType,
        subresource_range: ImageSubresourceRange,
        components: ComponentMapping,
        flags: ImageViewCreateFlags,
    ) -> VkResult<UsamiImageView> {
        UsamiDevice::create_image_view(
            &self.device,
            name,
            ImageViewCreateInfo::default()
                .format(self.format)
                .image(self.handle)
                .view_type(view_type)
                .subresource_range(subresource_range)
                .components(components)
                .flags(flags),
        )
    }

    pub fn buffer_copy(
        &self,
        aspect_mask: ImageAspectFlags,
        mip_level: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> BufferImageCopy {
        BufferImageCopy::default()
            .buffer_offset(0)
            .image_subresource(
                ImageSubresourceLayers::default()
                    .aspect_mask(aspect_mask)
                    .mip_level(mip_level)
                    .base_array_layer(base_array_layer)
                    .layer_count(layer_count),
            )
            .image_extent(self.extent)
    }
}

impl Drop for UsamiImage {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_image(self.handle, None) }
    }
}

pub struct UsamiImageView {
    device: Arc<UsamiDevice>,
    pub handle: ImageView,
}

impl UsamiImageView {
    pub fn new(device: &Arc<UsamiDevice>, create_info: ImageViewCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.handle.create_image_view(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiImageView {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_image_view(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_image(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: ImageCreateInfo,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<UsamiImage> {
        let image = UsamiImage::new(device, create_info, memory_flags)?;

        device.set_debug_name(name, image.handle)?;

        Ok(image)
    }

    pub fn create_image_view(
        device: &Arc<UsamiDevice>,
        name: String,
        create_info: ImageViewCreateInfo,
    ) -> VkResult<UsamiImageView> {
        let image_view = UsamiImageView::new(device, create_info)?;

        device.set_debug_name(name, image_view.handle)?;

        Ok(image_view)
    }

    pub fn import_image(
        device: &Arc<UsamiDevice>,
        command_pool: &UsamiCommandPool,
        name: String,
        raw_image: &RawImageData,
        usage: ImageUsageFlags,
        layout: ImageLayout,
    ) -> VkResult<UsamiImage> {
        let queue_family_indices = &[device.vk_queue_index];
        let image_create_info = ImageCreateInfo::default()
            .image_type(ImageType::TYPE_2D)
            .format(raw_image.format)
            .extent(raw_image.level_infos[0].extent)
            .mip_levels(raw_image.level_count() as u32)
            .array_layers(1)
            .initial_layout(ImageLayout::UNDEFINED)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(usage | ImageUsageFlags::TRANSFER_DST)
            .queue_family_indices(queue_family_indices);
        let image = Self::create_image(
            device,
            name.clone(),
            image_create_info,
            MemoryPropertyFlags::empty(),
        )?;

        let temporary_buffer = Self::create_buffer(
            device,
            format!("{name}_temp_buffer"),
            BufferCreateFlags::empty(),
            SharingMode::EXCLUSIVE,
            BufferUsageFlags::TRANSFER_SRC,
            &raw_image.data,
        )?;

        UsamiDevice::copy_buffer_to_image(
            device,
            command_pool,
            &temporary_buffer,
            &image,
            layout,
            &raw_image.copy_regions(ImageAspectFlags::COLOR),
        )?;

        Ok(image)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RawImageLevelInfo {
    pub extent: Extent3D,
    pub start_position: usize,
}

impl RawImageLevelInfo {
    pub fn buffer_copy(&self, aspect_mask: ImageAspectFlags, mip_level: u32) -> BufferImageCopy {
        BufferImageCopy::default()
            .buffer_offset(self.start_position as u64)
            .image_subresource(
                ImageSubresourceLayers::default()
                    .aspect_mask(aspect_mask)
                    .mip_level(mip_level)
                    .base_array_layer(0)
                    .layer_count(1),
            )
            .image_extent(self.extent)
    }

    pub fn size(&self, format: Format) -> u32 {
        let pixel_count = self.extent.height * self.extent.width;

        pixel_count * utils::get_format_size(format)
    }
}

#[derive(Clone, Debug)]
pub struct RawImageData {
    pub format: Format,
    pub data: Vec<u8>,
    pub level_infos: Vec<RawImageLevelInfo>,
}

impl RawImageData {
    pub const fn new(format: Format, data: Vec<u8>, level_infos: Vec<RawImageLevelInfo>) -> Self {
        Self {
            format,
            data,
            level_infos,
        }
    }

    pub fn raw_size(&self) -> usize {
        self.data.len()
    }

    pub fn level_count(&self) -> usize {
        self.level_infos.len()
    }

    pub fn size(&self, level: u32) -> Option<u32> {
        self.level_infos
            .get(level as usize)
            .map(|level_info| level_info.size(self.format))
    }

    pub fn copy_regions(&self, aspect_mask: ImageAspectFlags) -> Vec<BufferImageCopy> {
        let mut res = Vec::new();

        for (mip_level, level_info) in self.level_infos.iter().enumerate() {
            res.push(level_info.buffer_copy(aspect_mask, mip_level as u32));
        }

        res
    }
}

impl From<RgbaImage> for RawImageData {
    fn from(value: RgbaImage) -> Self {
        Self {
            format: Format::R8G8B8A8_UNORM,
            data: value.to_vec(),
            level_infos: vec![RawImageLevelInfo {
                extent: Extent3D {
                    width: value.width(),
                    height: value.height(),
                    depth: 1,
                },
                start_position: 0,
            }],
        }
    }
}

impl From<RgbImage> for RawImageData {
    fn from(value: RgbImage) -> Self {
        Self {
            format: Format::R8G8B8_UNORM,
            data: value.to_vec(),
            level_infos: vec![RawImageLevelInfo {
                extent: Extent3D {
                    width: value.width(),
                    height: value.height(),
                    depth: 1,
                },
                start_position: 0,
            }],
        }
    }
}

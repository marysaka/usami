use ash::{
    prelude::*,
    vk::{
        BufferCreateFlags, BufferUsageFlags, ComponentMapping, Extent3D, Format, Handle, Image,
        ImageCreateInfo, ImageLayout, ImageSubresourceRange, ImageTiling, ImageType,
        ImageUsageFlags, ImageView, ImageViewCreateFlags, ImageViewCreateInfo, ImageViewType,
        MemoryPropertyFlags, ObjectType, SampleCountFlags, SharingMode,
    },
    Device,
};

use crate::{UsamiDevice, UsamiDeviceMemory};

pub struct UsamiImage {
    device: Device,
    pub create_info: ImageCreateInfo,
    pub handle: Image,
    pub device_memory: UsamiDeviceMemory,
}

impl UsamiImage {
    pub fn new(
        device: &UsamiDevice,
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
            device: device.handle.clone(),
            create_info,
            handle,
            device_memory,
        })
    }

    pub fn create_simple_image_view(
        &self,
        device: &UsamiDevice,
        name: String,
        view_type: ImageViewType,
        subresource_range: ImageSubresourceRange,
        components: ComponentMapping,
        flags: ImageViewCreateFlags,
    ) -> VkResult<UsamiImageView> {
        device.create_image_view(
            name,
            ImageViewCreateInfo::builder()
                .format(self.create_info.format)
                .image(self.handle)
                .view_type(view_type)
                .subresource_range(subresource_range)
                .components(components)
                .flags(flags)
                .build(),
        )
    }
}

impl Drop for UsamiImage {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image(self.handle, None) }
    }
}

pub struct UsamiImageView {
    device: Device,
    pub create_info: ImageViewCreateInfo,
    pub handle: ImageView,
}

impl UsamiImageView {
    pub fn new(device: &Device, create_info: ImageViewCreateInfo) -> VkResult<Self> {
        let handle = unsafe { device.create_image_view(&create_info, None)? };

        Ok(Self {
            device: device.clone(),
            create_info,
            handle,
        })
    }
}

impl Drop for UsamiImageView {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image_view(self.handle, None) }
    }
}

impl UsamiDevice {
    pub fn create_image(
        &self,
        name: String,
        create_info: ImageCreateInfo,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<UsamiImage> {
        let image = UsamiImage::new(self, create_info, memory_flags)?;

        self.set_debug_name(name, image.handle.as_raw(), ObjectType::IMAGE)?;

        Ok(image)
    }

    pub fn create_image_view(
        &self,
        name: String,
        create_info: ImageViewCreateInfo,
    ) -> VkResult<UsamiImageView> {
        let image_view = UsamiImageView::new(&self.handle, create_info)?;

        self.set_debug_name(name, image_view.handle.as_raw(), ObjectType::IMAGE_VIEW)?;

        Ok(image_view)
    }

    pub fn import_2d_rgba8_image(
        &self,
        name: String,
        width: u32,
        height: u32,
        data: &[u8],
        usage: ImageUsageFlags,
        layout: ImageLayout,
    ) -> VkResult<UsamiImage> {
        let image_create_info = ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .format(Format::R8G8B8A8_UNORM)
            .extent(Extent3D {
                width,
                height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .initial_layout(ImageLayout::UNDEFINED)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(usage | ImageUsageFlags::TRANSFER_DST)
            .queue_family_indices(&[self.vk_queue_index])
            .build();
        let image = self.create_image(
            name.clone(),
            image_create_info,
            MemoryPropertyFlags::empty(),
        )?;

        let temporary_buffer = self.create_buffer(
            format!("{name}_temp_buffer"),
            BufferCreateFlags::empty(),
            SharingMode::EXCLUSIVE,
            BufferUsageFlags::TRANSFER_SRC,
            data,
        )?;

        self.copy_buffer_to_image(&temporary_buffer, &image, layout)?;

        Ok(image)
    }
}

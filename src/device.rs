use std::{
    ffi::{c_char, CString},
    mem::MaybeUninit,
};

use ash::{
    prelude::*,
    vk::{
        BufferCreateFlags, BufferCreateInfo, BufferUsageFlags, CommandBuffer,
        CommandBufferAllocateInfo, CommandBufferLevel, CommandPool, CommandPoolCreateInfo,
        DebugUtilsObjectNameInfoEXT, DeviceCreateInfo, DeviceQueueCreateInfo, Extent3D, Format,
        Handle, ImageAspectFlags, ImageCreateInfo, ImageSubresourceRange, ImageTiling, ImageType,
        ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, MemoryPropertyFlags,
        ObjectType, PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties,
        PhysicalDeviceProperties, Queue, QueueFamilyProperties, SampleCountFlags, SharingMode,
    },
};

use crate::{UsamiBuffer, UsamiImage, UsamiInstance};

pub struct UsamiPhysicalDevice {
    pub handle: PhysicalDevice,
    pub properties: PhysicalDeviceProperties,
    pub features: PhysicalDeviceFeatures,
    pub memory_properties: PhysicalDeviceMemoryProperties,
    pub queue_familiy_properties: Vec<QueueFamilyProperties>,
}

pub struct UsamiDevice {
    pub width: u32,
    pub height: u32,

    pub instance: UsamiInstance,
    pub physical_device: UsamiPhysicalDevice,
    pub vk_device: ash::Device,
    pub vk_command_pool: CommandPool,
    pub vk_command_buffer: CommandBuffer,
    pub vk_queue_index: u32,
    pub vk_queue: Queue,
    pub presentation_image: MaybeUninit<UsamiImage>,
    pub presentation_image_view: MaybeUninit<ImageView>,
    pub presentation_buffer_readback: MaybeUninit<UsamiBuffer>,
}

impl UsamiDevice {
    pub fn new_by_filter(
        instance: UsamiInstance,
        extensions: &[String],
        width: u32,
        height: u32,
        should_grab: Box<dyn FnMut(UsamiPhysicalDevice) -> Option<(UsamiPhysicalDevice, u32)>>,
    ) -> VkResult<Self> {
        let (physical_device, vk_queue_index) =
            unsafe { instance.vk_instance.enumerate_physical_devices()? }
                .iter()
                .map(|x| {
                    let prop = unsafe { instance.vk_instance.get_physical_device_properties(*x) };
                    let feat = unsafe { instance.vk_instance.get_physical_device_features(*x) };
                    let memory_prop = unsafe {
                        instance
                            .vk_instance
                            .get_physical_device_memory_properties(*x)
                    };
                    let queues = unsafe {
                        instance
                            .vk_instance
                            .get_physical_device_queue_family_properties(*x)
                    };
                    UsamiPhysicalDevice {
                        handle: *x,
                        features: feat,
                        memory_properties: memory_prop,
                        properties: prop,
                        queue_familiy_properties: queues,
                    }
                })
                .find_map(should_grab)
                .expect("Cannot find a device that match requirement!");

        let extensions_cstring: Vec<CString> = extensions
            .iter()
            .map(|name| CString::new(name.as_str()).unwrap())
            .collect();

        let extensions_raw: Vec<*const c_char> = extensions_cstring
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let enabled_features = PhysicalDeviceFeatures::default();

        let create_info = DeviceCreateInfo::builder()
            .queue_create_infos(&[DeviceQueueCreateInfo::builder()
                .queue_family_index(vk_queue_index)
                .queue_priorities(&[1.0])
                .build()])
            .enabled_features(&enabled_features)
            .enabled_extension_names(&extensions_raw)
            .build();

        let vk_device = unsafe {
            instance
                .vk_instance
                .create_device(physical_device.handle, &create_info, None)?
        };

        let vk_queue = unsafe { vk_device.get_device_queue(vk_queue_index, 0) };

        let vk_command_pool = unsafe {
            vk_device.create_command_pool(
                &CommandPoolCreateInfo::builder()
                    .queue_family_index(vk_queue_index)
                    .build(),
                None,
            )
        }?;

        let vk_command_buffer = unsafe {
            vk_device.allocate_command_buffers(
                &CommandBufferAllocateInfo::builder()
                    .command_pool(vk_command_pool)
                    .level(CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1)
                    .build(),
            )?[0]
        };

        let mut result = Self {
            width,
            height,
            instance,
            physical_device,
            vk_device,
            vk_queue_index,
            vk_queue,
            vk_command_pool,
            vk_command_buffer,
            presentation_image: MaybeUninit::uninit(),
            presentation_image_view: MaybeUninit::uninit(),
            presentation_buffer_readback: MaybeUninit::uninit(),
        };

        let vk_device = &result.vk_device;

        let presentation_image_info = ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .format(Format::R8G8B8A8_UNORM)
            .extent(Extent3D {
                width,
                height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::LINEAR)
            .usage(
                ImageUsageFlags::COLOR_ATTACHMENT
                    | ImageUsageFlags::SAMPLED
                    | ImageUsageFlags::TRANSFER_DST
                    | ImageUsageFlags::TRANSFER_SRC,
            )
            .build();

        let presentation_image = UsamiImage::new(
            &result,
            presentation_image_info,
            MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        result.set_debug_name(
            "presentation_image".into(),
            presentation_image.handle.as_raw(),
            ObjectType::IMAGE,
        )?;

        result.presentation_image.write(presentation_image);
        let presentation_image_view = unsafe {
            vk_device.create_image_view(
                &ImageViewCreateInfo::builder()
                    .view_type(ImageViewType::TYPE_2D)
                    .format(Format::R8G8B8A8_UNORM)
                    .image(result.presentation_image().handle)
                    .subresource_range(
                        ImageSubresourceRange::builder()
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1)
                            .build(),
                    )
                    .build(),
                None,
            )?
        };
        result
            .presentation_image_view
            .write(presentation_image_view);

        let presentation_buffer_readback_info = BufferCreateInfo::builder()
            .flags(BufferCreateFlags::empty())
            .sharing_mode(SharingMode::EXCLUSIVE)
            .usage(BufferUsageFlags::TRANSFER_DST)
            .queue_family_indices(&[vk_queue_index])
            .size(u64::from(width * height * 4))
            .build();

        let presentation_buffer_readback = UsamiBuffer::new(
            &result,
            presentation_buffer_readback_info,
            MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        result.set_debug_name(
            "presentation_buffer_readback".into(),
            presentation_buffer_readback.handle.as_raw(),
            ObjectType::BUFFER,
        )?;

        result
            .presentation_buffer_readback
            .write(presentation_buffer_readback);

        Ok(result)
    }

    pub fn presentation_image(&self) -> &UsamiImage {
        unsafe { self.presentation_image.assume_init_ref() }
    }

    pub fn presentation_buffer_readback(&self) -> &UsamiBuffer {
        unsafe { self.presentation_buffer_readback.assume_init_ref() }
    }

    pub fn set_debug_name(
        &self,
        name: String,
        object_handle: u64,
        object_type: ObjectType,
    ) -> VkResult<()> {
        unsafe {
            let name = CString::new(name).unwrap();

            self.instance
                .vk_debug_utils_loader
                .set_debug_utils_object_name(
                    self.vk_device.handle(),
                    &DebugUtilsObjectNameInfoEXT::builder()
                        .object_handle(object_handle)
                        .object_type(object_type)
                        .object_name(name.as_c_str())
                        .build(),
                )
        }
    }

    pub fn read_image_memory(&self) -> VkResult<Vec<u8>> {
        self.presentation_buffer_readback()
            .device_memory
            .read_to_vec()
    }
}

impl Drop for UsamiDevice {
    fn drop(&mut self) {
        unsafe {
            self.vk_device
                .free_command_buffers(self.vk_command_pool, &[self.vk_command_buffer]);
            self.vk_device
                .destroy_command_pool(self.vk_command_pool, None);
            self.vk_device
                .destroy_image_view(self.presentation_image_view.assume_init(), None);

            self.presentation_buffer_readback.assume_init_drop();
            self.presentation_image.assume_init_drop();
            self.vk_device.destroy_device(None);
        }
    }
}

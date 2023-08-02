use std::{
    ffi::{c_char, CString},
    mem::{ManuallyDrop, MaybeUninit},
};

use ash::{
    prelude::*,
    vk::{
        BufferCreateFlags, BufferUsageFlags, CommandPoolCreateInfo, ComponentMapping,
        ComponentSwizzle, DebugUtilsObjectNameInfoEXT, DeviceCreateInfo, DeviceQueueCreateInfo,
        Extent3D, Format, ImageAspectFlags, ImageCreateInfo, ImageSubresourceRange, ImageTiling,
        ImageType, ImageUsageFlags, ImageViewCreateFlags, ImageViewType, MemoryPropertyFlags,
        ObjectType, PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties,
        PhysicalDeviceProperties, QueueFamilyProperties, SampleCountFlags, SharingMode,
    },
};

use crate::{UsamiBuffer, UsamiCommandPool, UsamiImage, UsamiImageView, UsamiInstance, UsamiQueue};

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
    pub handle: ash::Device,
    pub command_pool: ManuallyDrop<UsamiCommandPool>,
    pub vk_queue_index: u32,
    pub presentation_image: MaybeUninit<UsamiImage>,
    pub presentation_image_view: MaybeUninit<UsamiImageView>,
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

        let handle = unsafe {
            instance
                .vk_instance
                .create_device(physical_device.handle, &create_info, None)?
        };

        let command_pool = UsamiCommandPool::new(
            &handle,
            CommandPoolCreateInfo::builder()
                .queue_family_index(vk_queue_index)
                .build(),
        )?;

        let mut result = Self {
            width,
            height,
            instance,
            physical_device,
            handle,
            vk_queue_index,
            command_pool: ManuallyDrop::new(command_pool),
            presentation_image: MaybeUninit::uninit(),
            presentation_image_view: MaybeUninit::uninit(),
            presentation_buffer_readback: MaybeUninit::uninit(),
        };

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
            .tiling(ImageTiling::OPTIMAL)
            .usage(
                ImageUsageFlags::COLOR_ATTACHMENT
                    | ImageUsageFlags::SAMPLED
                    | ImageUsageFlags::TRANSFER_DST
                    | ImageUsageFlags::TRANSFER_SRC,
            )
            .build();

        result.presentation_image.write(result.create_image(
            "presentation_image".into(),
            presentation_image_info,
            MemoryPropertyFlags::empty(),
        )?);
        let presentation_image_view = result.presentation_image().create_simple_image_view(
            &result,
            "presentation_image_view".into(),
            ImageViewType::TYPE_2D,
            ImageSubresourceRange::builder()
                .aspect_mask(ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(presentation_image_info.mip_levels)
                .base_array_layer(0)
                .layer_count(presentation_image_info.array_layers)
                .build(),
            ComponentMapping::builder()
                .r(ComponentSwizzle::IDENTITY)
                .g(ComponentSwizzle::IDENTITY)
                .b(ComponentSwizzle::IDENTITY)
                .a(ComponentSwizzle::IDENTITY)
                .build(),
            ImageViewCreateFlags::empty(),
        )?;
        result
            .presentation_image_view
            .write(presentation_image_view);

        let presentation_buffer_readback = result.create_buffer_with_size(
            "presentation_buffer_readback".into(),
            BufferCreateFlags::empty(),
            SharingMode::EXCLUSIVE,
            BufferUsageFlags::TRANSFER_DST,
            u64::from(width * height * 4),
            MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        result
            .presentation_buffer_readback
            .write(presentation_buffer_readback);

        Ok(result)
    }

    pub fn presentation_image(&self) -> &UsamiImage {
        unsafe { self.presentation_image.assume_init_ref() }
    }

    pub fn presentation_image_view(&self) -> &UsamiImageView {
        unsafe { self.presentation_image_view.assume_init_ref() }
    }

    pub fn presentation_buffer_readback(&self) -> &UsamiBuffer {
        unsafe { self.presentation_buffer_readback.assume_init_ref() }
    }

    pub fn get_queue(&self) -> VkResult<UsamiQueue> {
        self.get_device_queue("queue".into(), self.vk_queue_index, 0)
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
                    self.handle.handle(),
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
            ManuallyDrop::drop(&mut self.command_pool);
            self.presentation_image_view.assume_init_drop();
            self.presentation_buffer_readback.assume_init_drop();
            self.presentation_image.assume_init_drop();
            self.handle.destroy_device(None);
        }
    }
}

use std::{
    ffi::{c_char, CString},
    sync::Arc,
};

use ash::{
    extensions::ext::{MeshShader, ShaderObject},
    prelude::*,
    vk::{
        BufferCreateFlags, BufferUsageFlags, ComponentMapping, ComponentSwizzle,
        DebugUtilsObjectNameInfoEXT, DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, Extent3D,
        Format, FramebufferCreateInfo, ImageAspectFlags, ImageCreateInfo, ImageSubresourceRange,
        ImageTiling, ImageType, ImageUsageFlags, ImageViewCreateFlags, ImageViewType,
        MemoryPropertyFlags, ObjectType, PhysicalDevice, PhysicalDeviceFeatures,
        PhysicalDeviceMemoryProperties, PhysicalDeviceMeshShaderFeaturesEXT,
        PhysicalDeviceProperties, PhysicalDeviceShaderObjectFeaturesEXT, QueueFamilyProperties,
        Rect2D, SampleCountFlags, SharingMode, Viewport,
    },
};

use crate::{
    utils, UsamiBuffer, UsamiFramebuffer, UsamiImage, UsamiImageView, UsamiInstance,
    UsamiRenderPass,
};

pub struct UsamiPhysicalDevice {
    pub handle: PhysicalDevice,
    pub properties: PhysicalDeviceProperties,
    pub features: PhysicalDeviceFeatures,
    pub memory_properties: PhysicalDeviceMemoryProperties,
    pub queue_familiy_properties: Vec<QueueFamilyProperties>,
}

pub struct UsamiDevice {
    pub instance: UsamiInstance,
    pub physical_device: UsamiPhysicalDevice,
    pub handle: ash::Device,
    pub vk_queue_index: u32,
}

impl UsamiDevice {
    pub fn new_by_filter(
        instance: UsamiInstance,
        extensions: &[String],
        should_grab: Box<dyn FnMut(UsamiPhysicalDevice) -> Option<(UsamiPhysicalDevice, u32)>>,
    ) -> VkResult<Arc<Self>> {
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

        let mut has_shader_object_extension = false;
        let mut has_mesh_shader_extension = false;

        for extension in &extensions_cstring {
            if ShaderObject::name() == extension.as_c_str() {
                has_shader_object_extension = true;
            }

            if MeshShader::name() == extension.as_c_str() {
                has_mesh_shader_extension = true;
            }
        }

        let extensions_raw: Vec<*const c_char> = extensions_cstring
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let enabled_features = PhysicalDeviceFeatures::default();

        let device_queue_create_info = [DeviceQueueCreateInfo::builder()
            .queue_family_index(vk_queue_index)
            .queue_priorities(&[1.0])
            .build()];

        let mut shader_object_features = PhysicalDeviceShaderObjectFeaturesEXT::builder()
            .shader_object(true)
            .build();

        let mut mesh_shader_features = PhysicalDeviceMeshShaderFeaturesEXT::builder()
            .mesh_shader(true)
            .task_shader(true)
            .build();

        let mut create_info_builder = DeviceCreateInfo::builder()
            .queue_create_infos(&device_queue_create_info)
            .enabled_features(&enabled_features)
            .enabled_extension_names(&extensions_raw);

        if has_shader_object_extension {
            create_info_builder = create_info_builder.push_next(&mut shader_object_features);
        }

        if has_mesh_shader_extension {
            create_info_builder = create_info_builder.push_next(&mut mesh_shader_features);
        }

        let create_info = create_info_builder.build();

        let handle = unsafe {
            instance
                .vk_instance
                .create_device(physical_device.handle, &create_info, None)?
        };

        Ok(Arc::new(Self {
            instance,
            physical_device,
            handle,
            vk_queue_index,
        }))
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
}

impl Drop for UsamiDevice {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_device(None);
        }
    }
}

pub struct UsamiPresentation {
    pub image: UsamiImage,
    pub image_view: UsamiImageView,
    pub buffer_readback: UsamiBuffer,
}

impl UsamiPresentation {
    pub fn new(device: &Arc<UsamiDevice>, width: u32, height: u32) -> VkResult<Self> {
        let format = Format::R8G8B8A8_UNORM;
        let presentation_image_info = ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .format(format)
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

        let image = UsamiDevice::create_image(
            device,
            "presentation_image".into(),
            presentation_image_info,
            MemoryPropertyFlags::empty(),
        )?;

        let image_view = image.create_simple_image_view(
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

        let buffer_readback = UsamiDevice::create_buffer_with_size(
            device,
            "presentation_buffer_readback".into(),
            BufferCreateFlags::empty(),
            SharingMode::EXCLUSIVE,
            BufferUsageFlags::TRANSFER_DST,
            u64::from(width * height * utils::get_format_size(format)),
            MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        Ok(Self {
            image,
            image_view,
            buffer_readback,
        })
    }

    pub fn dimensions(&self) -> Extent2D {
        Extent2D {
            width: self.image.create_info.extent.width,
            height: self.image.create_info.extent.height,
        }
    }

    pub fn rect2d(&self) -> Rect2D {
        Rect2D::builder().extent(self.dimensions()).build()
    }

    pub fn viewport(&self) -> Viewport {
        let dimensions = self.dimensions();

        Viewport {
            x: 0.0,
            y: 0.0,
            width: dimensions.width as f32,
            height: dimensions.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }
    }

    pub fn create_framebuffer(
        &self,
        device: &Arc<UsamiDevice>,
        name: String,
        render_pass: &UsamiRenderPass,
    ) -> VkResult<UsamiFramebuffer> {
        let dimensions = self.dimensions();

        UsamiDevice::create_framebuffer(
            device,
            name,
            FramebufferCreateInfo::builder()
                .render_pass(render_pass.handle)
                .attachments(&[self.image_view.handle])
                .width(dimensions.width)
                .height(dimensions.height)
                .layers(1)
                .build(),
        )
    }
}

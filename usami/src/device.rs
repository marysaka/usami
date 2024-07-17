use std::{
    ffi::{c_char, CString},
    sync::Arc,
};

use ash::{
    ext::debug_utils::Device as DebugUtilsDevice,
    prelude::*,
    vk::{
        self, BufferCreateFlags, BufferUsageFlags, ComponentMapping, ComponentSwizzle,
        DebugUtilsObjectNameInfoEXT, DeviceCreateInfo, DeviceQueueCreateInfo,
        Extent2D, Extent3D, Format,
        FramebufferCreateInfo, ImageAspectFlags, ImageCreateInfo, ImageSubresourceRange,
        ImageTiling, ImageType, ImageUsageFlags, ImageViewCreateFlags, ImageViewType,
        MemoryPropertyFlags, PhysicalDevice,
        PhysicalDeviceConditionalRenderingFeaturesEXT, PhysicalDeviceCooperativeMatrixFeaturesKHR,
        PhysicalDeviceCooperativeMatrixFeaturesNV, PhysicalDeviceFeatures, PhysicalDeviceFeatures2,
        PhysicalDeviceMemoryProperties, PhysicalDeviceMeshShaderFeaturesEXT,
        PhysicalDeviceProperties, PhysicalDeviceShaderObjectFeaturesEXT,
        PhysicalDeviceTransformFeedbackFeaturesEXT, PhysicalDeviceVulkan11Features,
        PhysicalDeviceVulkan12Features, PhysicalDeviceVulkan13Features, QueueFamilyProperties,
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
    pub vk_debug_utils_device: DebugUtilsDevice,
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
        let mut has_conditional_rendering_extension = false;
        let mut has_xfb_extension = false;
        let mut has_cooperative_matrix = false;
        let mut has_cooperative_matrix_nv = false;

        for extension in &extensions_cstring {
            if ash::ext::shader_object::NAME == extension.as_c_str() {
                has_shader_object_extension = true;
            }

            if ash::ext::mesh_shader::NAME == extension.as_c_str() {
                has_mesh_shader_extension = true;
            }

            if ash::ext::conditional_rendering::NAME == extension.as_c_str() {
                has_conditional_rendering_extension = true;
            }

            if ash::ext::transform_feedback::NAME == extension.as_c_str() {
                has_xfb_extension = true;
            }

            if ash::khr::cooperative_matrix::NAME == extension.as_c_str() {
                has_cooperative_matrix = true;
            }

            if ash::nv::cooperative_matrix::NAME == extension.as_c_str() {
                has_cooperative_matrix_nv = true;
            }
        }

        let extensions_raw: Vec<*const c_char> = extensions_cstring
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let enabled_features = PhysicalDeviceFeatures::default()
            .geometry_shader(physical_device.features.geometry_shader != 0)
            .shader_tessellation_and_geometry_point_size(
                physical_device
                    .features
                    .shader_tessellation_and_geometry_point_size
                    != 0,
            );

        let device_queue_create_info = [DeviceQueueCreateInfo::default()
            .queue_family_index(vk_queue_index)
            .queue_priorities(&[1.0])];

        let mut shader_object_features =
            PhysicalDeviceShaderObjectFeaturesEXT::default().shader_object(true);

        let mut mesh_shader_features = PhysicalDeviceMeshShaderFeaturesEXT::default()
            .mesh_shader(true)
            .task_shader(true);

        let mut conditional_rendering_features =
            PhysicalDeviceConditionalRenderingFeaturesEXT::default().conditional_rendering(true);

        let mut xfb_features = PhysicalDeviceTransformFeedbackFeaturesEXT::default()
            .transform_feedback(true)
            .geometry_streams(true);

        let mut cooperative_matrix_features =
            PhysicalDeviceCooperativeMatrixFeaturesKHR::default().cooperative_matrix(true);

        let mut cooperative_matrix_nv_features =
            PhysicalDeviceCooperativeMatrixFeaturesNV::default().cooperative_matrix(true);

        let mut create_info = DeviceCreateInfo::default()
            .queue_create_infos(&device_queue_create_info)
            .enabled_features(&enabled_features)
            .enabled_extension_names(&extensions_raw);

        if has_shader_object_extension {
            create_info = create_info.push_next(&mut shader_object_features);
        }

        if has_mesh_shader_extension {
            create_info = create_info.push_next(&mut mesh_shader_features);
        }

        if has_conditional_rendering_extension {
            create_info = create_info.push_next(&mut conditional_rendering_features);
        }

        if has_xfb_extension {
            create_info = create_info.push_next(&mut xfb_features);
        }

        if has_cooperative_matrix {
            create_info = create_info.push_next(&mut cooperative_matrix_features);
        }

        if has_cooperative_matrix_nv {
            create_info = create_info.push_next(&mut cooperative_matrix_nv_features);
        }

        let mut vk11_features = PhysicalDeviceVulkan11Features::default();
        let mut vk12_features = PhysicalDeviceVulkan12Features::default();
        let mut vk13_features = PhysicalDeviceVulkan13Features::default();

        if instance.vk_version >= vk::API_VERSION_1_2 {
            let mut tmp_feat = PhysicalDeviceFeatures2::default();
            tmp_feat = tmp_feat.push_next(&mut vk11_features);
            tmp_feat = tmp_feat.push_next(&mut vk12_features);

            if instance.vk_version >= vk::API_VERSION_1_3 {
                tmp_feat = tmp_feat.push_next(&mut vk13_features);
            }

            unsafe {
                instance
                    .vk_instance
                    .get_physical_device_features2(physical_device.handle, &mut tmp_feat)
            };

            // Ensure the next pointer is NULL as get_physical_device_features2 could create garbage...
            vk11_features.p_next = core::ptr::null_mut();
            vk12_features.p_next = core::ptr::null_mut();
            vk13_features.p_next = core::ptr::null_mut();

            create_info = create_info.push_next(&mut vk11_features);
            create_info = create_info.push_next(&mut vk12_features);

            if instance.vk_version >= vk::API_VERSION_1_3 {
                create_info = create_info.push_next(&mut vk13_features);
            }
        }

        let handle = unsafe {
            instance
                .vk_instance
                .create_device(physical_device.handle, &create_info, None)?
        };

        let vk_debug_utils_device = DebugUtilsDevice::new(&instance.vk_instance, &handle);

        Ok(Arc::new(Self {
            instance,
            physical_device,
            handle,
            vk_debug_utils_device,
            vk_queue_index,
        }))
    }

    pub fn set_debug_name<T: vk::Handle>(&self, name: String, object_handle: T) -> VkResult<()> {
        unsafe {
            let name = CString::new(name).unwrap();

            self.vk_debug_utils_device.set_debug_utils_object_name(
                &DebugUtilsObjectNameInfoEXT::default()
                    .object_handle(object_handle)
                    .object_name(name.as_c_str()),
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
        let presentation_image_info = ImageCreateInfo::default()
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
            );

        let image = UsamiDevice::create_image(
            device,
            "presentation_image".into(),
            presentation_image_info,
            MemoryPropertyFlags::empty(),
        )?;

        let image_view = image.create_simple_image_view(
            "presentation_image_view".into(),
            ImageViewType::TYPE_2D,
            ImageSubresourceRange::default()
                .aspect_mask(ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(presentation_image_info.mip_levels)
                .base_array_layer(0)
                .layer_count(presentation_image_info.array_layers),
            ComponentMapping::default()
                .r(ComponentSwizzle::IDENTITY)
                .g(ComponentSwizzle::IDENTITY)
                .b(ComponentSwizzle::IDENTITY)
                .a(ComponentSwizzle::IDENTITY),
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
            width: self.image.extent.width,
            height: self.image.extent.height,
        }
    }

    pub fn rect2d(&self) -> Rect2D {
        Rect2D::default().extent(self.dimensions())
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
            FramebufferCreateInfo::default()
                .render_pass(render_pass.handle)
                .attachments(&[self.image_view.handle])
                .width(dimensions.width)
                .height(dimensions.height)
                .layers(1),
        )
    }
}

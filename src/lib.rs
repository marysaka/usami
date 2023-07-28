use std::{
    borrow::Cow,
    ffi::{c_char, CStr, CString},
};

use ash::{
    extensions::ext::DebugUtils,
    prelude::*,
    vk::{
        self, AccessFlags, Buffer, BufferCreateFlags, BufferCreateInfo, BufferImageCopy,
        BufferUsageFlags, CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo,
        CommandBufferLevel, CommandBufferUsageFlags, CommandPool, CommandPoolCreateFlags,
        CommandPoolCreateInfo, DependencyFlags, DeviceMemory, DeviceQueueCreateInfo,
        DeviceQueueCreateInfoBuilder, Format, Image, ImageAspectFlags, ImageLayout,
        ImageMemoryBarrier, ImageSubresourceLayers, ImageSubresourceRange, ImageTiling, ImageType,
        ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, MemoryAllocateInfo,
        MemoryBarrier, MemoryMapFlags, MemoryPropertyFlags, MemoryRequirements, Offset3D,
        PipelineStageFlags, Queue, SampleCountFlags, SharingMode,
    },
    Entry,
};

pub struct UsamiInstance {
    pub vk_entry: Entry,
    pub vk_instance: ash::Instance,
    pub vk_debug_utils_loader: DebugUtils,
    pub vk_messenger: vk::DebugUtilsMessengerEXT,
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}

impl UsamiInstance {
    pub fn new(
        app_name: &str,
        engine_name: &str,
        api_version: u32,
        extensions: &[String],
    ) -> VkResult<Self> {
        let app_name = CString::new(app_name).expect("cannot create CString for application name");
        let engine_name = CString::new(engine_name).expect("cannot create CString for engine name");
        let application_info = vk::ApplicationInfo::builder()
            .application_name(app_name.as_c_str())
            .application_version(0)
            .engine_name(engine_name.as_c_str())
            .engine_version(0)
            .api_version(api_version);
        let extensions_cstring: Vec<CString> = extensions
            .iter()
            .map(|name| CString::new(name.as_str()).unwrap())
            .collect();

        let extensions_raw: Vec<*const c_char> = extensions_cstring
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let validation_layer = CString::new("VK_LAYER_KHRONOS_validation").unwrap();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions_raw)
            .enabled_layer_names(&[validation_layer.as_ptr()])
            .build();

        let vk_entry = unsafe { Entry::load().expect("Cannot load VK library") };
        let vk_instance = unsafe { vk_entry.create_instance(&create_info, None)? };

        let vk_debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback))
            .build();

        let vk_debug_utils_loader = DebugUtils::new(&vk_entry, &vk_instance);
        let vk_messenger =
            unsafe { vk_debug_utils_loader.create_debug_utils_messenger(&vk_debug_info, None)? };

        Ok(Self {
            vk_entry,
            vk_instance,
            vk_debug_utils_loader,
            vk_messenger,
        })
    }
}

impl Drop for UsamiInstance {
    fn drop(&mut self) {
        unsafe {
            self.vk_debug_utils_loader
                .destroy_debug_utils_messenger(self.vk_messenger, None);
            self.vk_instance.destroy_instance(None)
        }
    }
}

pub struct UsamiDevice {
    pub width: u32,
    pub height: u32,
    pub instance: UsamiInstance,
    pub vk_device: ash::Device,
    pub vk_command_pool: CommandPool,
    pub vk_command_buffer: CommandBuffer,
    pub vk_queue_index: u32,
    pub vk_queue: Queue,
    pub presentation_image: Image,
    pub presentation_image_req: MemoryRequirements,
    pub presentation_image_allocation: DeviceMemory,
    pub presentation_image_view: ImageView,
    pub presentation_buffer_readback: Buffer,
    pub presentation_buffer_readback_allocation: DeviceMemory,
}

impl UsamiDevice {
    pub fn new_by_filter(
        instance: UsamiInstance,
        extensions: &[String],
        width: u32,
        height: u32,
        should_grab: Box<
            dyn FnMut(
                (
                    vk::PhysicalDevice,
                    vk::PhysicalDeviceProperties,
                    vk::PhysicalDeviceFeatures,
                    Vec<vk::QueueFamilyProperties>,
                ),
            ) -> Option<(
                vk::PhysicalDevice,
                vk::PhysicalDeviceProperties,
                vk::PhysicalDeviceFeatures,
                Vec<vk::QueueFamilyProperties>,
            )>,
        >,
    ) -> VkResult<Self> {
        let (physical_device, prop, feat, queues) =
            unsafe { instance.vk_instance.enumerate_physical_devices()? }
                .iter()
                .map(|x| {
                    let prop = unsafe { instance.vk_instance.get_physical_device_properties(*x) };
                    let feat = unsafe { instance.vk_instance.get_physical_device_features(*x) };
                    let queues = unsafe {
                        instance
                            .vk_instance
                            .get_physical_device_queue_family_properties(*x)
                    };
                    (*x, prop, feat, queues)
                })
                .find_map(should_grab)
                .expect("Cannot find a device that match requirement!");

        let vk_queue_index = queues
            .iter()
            .enumerate()
            .find_map(|(i, x)| {
                if x.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    Some(i as u32)
                } else {
                    None
                }
            })
            .expect("No graphics queue");

        let extensions_cstring: Vec<CString> = extensions
            .iter()
            .map(|name| CString::new(name.as_str()).unwrap())
            .collect();

        let extensions_raw: Vec<*const c_char> = extensions_cstring
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let enabled_features = vk::PhysicalDeviceFeatures::default();

        let create_info = vk::DeviceCreateInfo::builder()
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
                .create_device(physical_device, &create_info, None)?
        };

        let physical_device_memory_properties = unsafe {
            instance
                .vk_instance
                .get_physical_device_memory_properties(physical_device)
        };

        let presentation_image_info = vk::ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .format(Format::R8G8B8A8_UNORM)
            .extent(vk::Extent3D {
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

        let presentation_image = unsafe { vk_device.create_image(&presentation_image_info, None)? };
        let presentation_image_req =
            unsafe { vk_device.get_image_memory_requirements(presentation_image) };
        let presentation_image_allocation_info = MemoryAllocateInfo::builder()
            .allocation_size(presentation_image_req.size)
            .memory_type_index(
                Self::find_memory_type_index(
                    &presentation_image_req,
                    &physical_device_memory_properties,
                    MemoryPropertyFlags::HOST_VISIBLE,
                )
                .expect("cannot find memory type for presentation image"),
            )
            .build();
        let presentation_image_allocation =
            unsafe { vk_device.allocate_memory(&presentation_image_allocation_info, None)? };
        unsafe {
            vk_device.bind_image_memory(presentation_image, presentation_image_allocation, 0)?;
        }
        let presentation_image_view = unsafe {
            vk_device.create_image_view(
                &ImageViewCreateInfo::builder()
                    .view_type(ImageViewType::TYPE_2D)
                    .format(Format::R8G8B8A8_UNORM)
                    .image(presentation_image)
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

        let presentation_buffer_readback_allocation = unsafe {
            vk_device.allocate_memory(
                &MemoryAllocateInfo::builder()
                    .allocation_size(u64::from(width * height * 4))
                    .memory_type_index(presentation_image_allocation_info.memory_type_index)
                    .build(),
                None,
            )?
        };

        let presentation_buffer_readback = unsafe {
            vk_device.create_buffer(
                &BufferCreateInfo::builder()
                    .flags(BufferCreateFlags::empty())
                    .sharing_mode(SharingMode::EXCLUSIVE)
                    .usage(BufferUsageFlags::TRANSFER_DST)
                    .queue_family_indices(&[vk_queue_index])
                    .size(u64::from(width * height * 4))
                    .build(),
                None,
            )?
        };

        unsafe {
            vk_device.bind_buffer_memory(
                presentation_buffer_readback,
                presentation_buffer_readback_allocation,
                0,
            )?;
        }

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

        Ok(Self {
            width,
            height,
            instance,
            vk_device,
            vk_queue_index,
            vk_queue,
            vk_command_pool,
            vk_command_buffer,
            presentation_image,
            presentation_image_req,
            presentation_image_allocation,
            presentation_image_view,
            presentation_buffer_readback,
            presentation_buffer_readback_allocation,
        })
    }

    pub fn find_memory_type_index(
        memory_req: &vk::MemoryRequirements,
        memory_prop: &vk::PhysicalDeviceMemoryProperties,
        flags: vk::MemoryPropertyFlags,
    ) -> Option<u32> {
        memory_prop.memory_types[..memory_prop.memory_type_count as _]
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (1 << index) & memory_req.memory_type_bits != 0
                    && memory_type.property_flags & flags == flags
            })
            .map(|(index, _memory_type)| index as _)
    }

    pub fn read_image_memory(&self) -> VkResult<Vec<u8>> {
        let mut res = Vec::new();

        let image_subresource_range = ImageSubresourceRange::builder()
            .base_array_layer(0)
            .layer_count(1)
            .base_mip_level(0)
            .level_count(1)
            .aspect_mask(ImageAspectFlags::COLOR)
            .build();

        let image_size = self.width * self.height * 4;

        res.resize(image_size as usize, 0);

        unsafe {
            let vk_command_buffer = self.vk_device.allocate_command_buffers(
                &CommandBufferAllocateInfo::builder()
                    .command_pool(self.vk_command_pool)
                    .level(CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1)
                    .build(),
            )?[0];

            self.vk_device.begin_command_buffer(
                vk_command_buffer,
                &vk::CommandBufferBeginInfo::builder()
                    .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT)
                    .build(),
            )?;

            self.vk_device.cmd_copy_image_to_buffer(
                vk_command_buffer,
                self.presentation_image,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
                self.presentation_buffer_readback,
                &[BufferImageCopy::builder()
                    .image_offset(Offset3D::builder().x(0).y(0).z(0).build())
                    .image_subresource(
                        ImageSubresourceLayers::builder()
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .layer_count(1)
                            .build(),
                    )
                    .image_extent(vk::Extent3D {
                        width: self.width,
                        height: self.height,
                        depth: 1,
                    })
                    .build()],
            );
            self.vk_device.cmd_pipeline_barrier(
                vk_command_buffer,
                PipelineStageFlags::TRANSFER,
                PipelineStageFlags::HOST,
                DependencyFlags::empty(),
                &[MemoryBarrier::builder()
                    .src_access_mask(AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(AccessFlags::HOST_READ)
                    .build()],
                &[],
                &[],
            );

            self.vk_device.end_command_buffer(vk_command_buffer)?;

            self.vk_device.device_wait_idle()?;
            self.vk_device
                .free_command_buffers(self.vk_command_pool, &[vk_command_buffer]);

            let ptr = self.vk_device.map_memory(
                self.presentation_buffer_readback_allocation,
                0,
                u64::from(image_size),
                MemoryMapFlags::empty(),
            )?;

            std::ptr::copy(ptr, res.as_mut_ptr() as *mut _, image_size as usize);

            self.vk_device
                .unmap_memory(self.presentation_buffer_readback_allocation);
        }

        Ok(res)
    }
}

impl Drop for UsamiDevice {
    fn drop(&mut self) {
        unsafe {
            self.vk_device
                .destroy_buffer(self.presentation_buffer_readback, None);
            self.vk_device
                .free_memory(self.presentation_buffer_readback_allocation, None);
            self.vk_device
                .free_command_buffers(self.vk_command_pool, &[self.vk_command_buffer]);
            self.vk_device
                .destroy_command_pool(self.vk_command_pool, None);
            self.vk_device
                .destroy_image_view(self.presentation_image_view, None);
            self.vk_device.destroy_image(self.presentation_image, None);
            self.vk_device
                .free_memory(self.presentation_image_allocation, None);
            self.vk_device.destroy_device(None);
        }
    }
}

pub fn record_command_buffer_with_image_dep<F: Fn(&UsamiDevice, CommandBuffer, Image)>(
    device: &UsamiDevice,
    command_buffer: CommandBuffer,
    image: Image,
    callback: F,
) -> VkResult<()> {
    let image_subresource_range = ImageSubresourceRange::builder()
        .base_array_layer(0)
        .layer_count(1)
        .base_mip_level(0)
        .level_count(1)
        .aspect_mask(ImageAspectFlags::COLOR)
        .build();

    unsafe {
        device.vk_device.begin_command_buffer(
            command_buffer,
            &vk::CommandBufferBeginInfo::builder()
                .flags(CommandBufferUsageFlags::SIMULTANEOUS_USE)
                .build(),
        )?;
    }

    callback(device, command_buffer, image);

    unsafe {
        /*device.vk_device.cmd_copy_image_to_buffer(
            command_buffer,
            device.presentation_image,
            ImageLayout::TRANSFER_SRC_OPTIMAL,
            device.presentation_buffer_readback,
            &[BufferImageCopy::builder()
                .image_offset(Offset3D::builder().x(0).y(0).z(0).build())
                .image_subresource(
                    ImageSubresourceLayers::builder()
                        .aspect_mask(ImageAspectFlags::COLOR)
                        .layer_count(1)
                        .build(),
                )
                .image_extent(vk::Extent3D {
                    width: device.width,
                    height: device.height,
                    depth: 1,
                })
                .build()],
        );
        device.vk_device.cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::TRANSFER,
            PipelineStageFlags::HOST,
            DependencyFlags::empty(),
            &[MemoryBarrier::builder()
                .src_access_mask(AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(AccessFlags::HOST_READ)
                .build()],
            &[],
            &[],
        );*/
        device.vk_device.cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::TRANSFER,
            PipelineStageFlags::HOST,
            DependencyFlags::empty(),
            &[MemoryBarrier::builder()
                .src_access_mask(AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(AccessFlags::HOST_READ)
                .build()],
            &[],
            &[],
        );

        device.vk_device.end_command_buffer(command_buffer)?;
    }

    Ok(())
}

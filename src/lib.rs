pub mod utils;

use std::{
    borrow::Cow,
    ffi::{c_char, c_void, CStr, CString},
    mem::MaybeUninit,
};

use ash::{
    extensions::ext::DebugUtils,
    prelude::*,
    util::Align,
    vk::{
        self, AccessFlags, Buffer, BufferCreateFlags, BufferCreateInfo, BufferImageCopy,
        BufferUsageFlags, CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPool, CommandPoolCreateInfo, DependencyFlags, DeviceMemory,
        DeviceQueueCreateInfo, Format, Image, ImageAspectFlags, ImageCreateInfo, ImageLayout,
        ImageMemoryBarrier, ImageSubresourceLayers, ImageSubresourceRange, ImageTiling, ImageType,
        ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, MemoryAllocateInfo,
        MemoryMapFlags, MemoryPropertyFlags, MemoryRequirements, Offset3D, PhysicalDevice,
        PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties, PhysicalDeviceProperties,
        PipelineStageFlags, Queue, SampleCountFlags, ShaderModule, ShaderModuleCreateFlags,
        ShaderModuleCreateInfo, SharingMode,
    },
    Device, Entry,
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
    pub physical_device: PhysicalDevice,
    pub physical_device_properties: PhysicalDeviceProperties,
    pub physical_device_features: PhysicalDeviceFeatures,
    pub physical_device_memory_properties: PhysicalDeviceMemoryProperties,
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
        let (
            physical_device,
            physical_device_properties,
            physical_device_features,
            queue_familiy_properties,
        ) = unsafe { instance.vk_instance.enumerate_physical_devices()? }
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

        let vk_queue_index = queue_familiy_properties
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
            physical_device_properties,
            physical_device_features,
            physical_device_memory_properties,
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

        result.presentation_image.write(UsamiImage::new(
            &result,
            presentation_image_info,
            MemoryPropertyFlags::HOST_VISIBLE,
        )?);
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

        result.presentation_buffer_readback.write(UsamiBuffer::new(
            &result,
            presentation_buffer_readback_info,
            MemoryPropertyFlags::HOST_VISIBLE,
        )?);

        Ok(result)
    }

    pub fn presentation_image(&self) -> &UsamiImage {
        unsafe { self.presentation_image.assume_init_ref() }
    }

    pub fn presentation_buffer_readback(&self) -> &UsamiBuffer {
        unsafe { self.presentation_buffer_readback.assume_init_ref() }
    }

    pub fn find_memory_type(
        &self,
        req: &MemoryRequirements,
        flags: MemoryPropertyFlags,
    ) -> VkResult<u32> {
        self.physical_device_memory_properties.memory_types
            [..self.physical_device_memory_properties.memory_type_count as _]
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (1 << index) & req.memory_type_bits != 0
                    && memory_type.property_flags & flags == flags
            })
            .map(|(index, _memory_type)| index as _)
            .ok_or(vk::Result::ERROR_UNKNOWN) // TODO better error management
    }

    pub fn read_image_memory(&self) -> VkResult<Vec<u8>> {
        self.presentation_buffer_readback()
            .device_memory
            .read_to_vec()
    }

    pub fn create_buffer<T: Copy>(
        &self,
        _name: String,
        flags: BufferCreateFlags,
        queue_family_indices: &[u32],
        sharing_mode: SharingMode,
        usage: BufferUsageFlags,
        data: &[T],
    ) -> VkResult<UsamiBuffer> {
        let create_info = BufferCreateInfo::builder()
            .flags(flags)
            .queue_family_indices(queue_family_indices)
            .sharing_mode(sharing_mode)
            .usage(usage)
            .size(std::mem::size_of_val(data) as u64)
            .build();
        let buffer = UsamiBuffer::new(self, create_info, MemoryPropertyFlags::HOST_VISIBLE)?;

        unsafe {
            let dst_slice =
                std::slice::from_raw_parts_mut(buffer.device_memory.map()? as *mut T, data.len());

            dst_slice.copy_from_slice(data);

            buffer.device_memory.unmap();
        }

        Ok(buffer)
    }

    pub fn create_shader(&self, _name: String, code: &[u32]) -> VkResult<UsamiShader> {
        UsamiShader::new(self, code)
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

pub struct UsamiDeviceMemory {
    // TODO: Arc
    device: Device,
    pub requirements: MemoryRequirements,
    pub allocate_info: MemoryAllocateInfo,
    pub handle: DeviceMemory,
}

impl UsamiDeviceMemory {
    pub fn new(
        device: &UsamiDevice,
        requirements: MemoryRequirements,
        flags: MemoryPropertyFlags,
    ) -> VkResult<Self> {
        let allocate_info = MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(device.find_memory_type(&requirements, flags)?)
            .build();

        let handle = unsafe { device.vk_device.allocate_memory(&allocate_info, None)? };

        Ok(Self {
            device: device.vk_device.clone(),
            requirements,
            allocate_info,
            handle,
        })
    }

    pub unsafe fn map(&self) -> VkResult<*mut c_void> {
        self.device.map_memory(
            self.handle,
            0,
            self.allocate_info.allocation_size,
            MemoryMapFlags::empty(),
        )
    }

    pub unsafe fn unmap(&self) {
        self.device.unmap_memory(self.handle)
    }

    pub fn read_to_vec(&self) -> VkResult<Vec<u8>> {
        let mut res = Vec::new();

        let allocation_size = self.allocate_info.allocation_size as usize;

        res.resize(allocation_size, 0);

        unsafe {
            let ptr = self.map()?;

            std::ptr::copy(ptr, res.as_mut_ptr() as *mut _, allocation_size);

            self.unmap();
        }

        Ok(res)
    }
}

impl Drop for UsamiDeviceMemory {
    fn drop(&mut self) {
        unsafe {
            self.device.free_memory(self.handle, None);
        }
    }
}

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
        let vk_device: &Device = &device.vk_device;

        let handle = unsafe { vk_device.create_image(&create_info, None)? };
        let req = unsafe { vk_device.get_image_memory_requirements(handle) };
        let device_memory = UsamiDeviceMemory::new(device, req, memory_flags)?;
        unsafe {
            vk_device.bind_image_memory(handle, device_memory.handle, 0)?;
        }

        Ok(Self {
            device: device.vk_device.clone(),
            create_info,
            handle,
            device_memory,
        })
    }
}

impl Drop for UsamiImage {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image(self.handle, None) }
    }
}

pub struct UsamiBuffer {
    device: Device,
    pub create_info: BufferCreateInfo,
    pub handle: Buffer,
    pub device_memory: UsamiDeviceMemory,
}

impl UsamiBuffer {
    pub fn new(
        device: &UsamiDevice,
        create_info: BufferCreateInfo,
        memory_flags: MemoryPropertyFlags,
    ) -> VkResult<Self> {
        let vk_device: &Device = &device.vk_device;

        let handle = unsafe { vk_device.create_buffer(&create_info, None)? };
        let req = unsafe { vk_device.get_buffer_memory_requirements(handle) };
        let device_memory = UsamiDeviceMemory::new(device, req, memory_flags)?;
        unsafe {
            vk_device.bind_buffer_memory(handle, device_memory.handle, 0)?;
        }

        Ok(Self {
            device: device.vk_device.clone(),
            create_info,
            handle,
            device_memory,
        })
    }
}

impl Drop for UsamiBuffer {
    fn drop(&mut self) {
        unsafe { self.device.destroy_buffer(self.handle, None) }
    }
}

pub struct UsamiShader {
    device: Device,
    pub handle: ShaderModule,
}

impl UsamiShader {
    pub fn new(device: &UsamiDevice, code: &[u32]) -> VkResult<Self> {
        let create_info = ShaderModuleCreateInfo::builder().code(code).build();

        let handle = unsafe { device.vk_device.create_shader_module(&create_info, None)? };

        Ok(Self {
            device: device.vk_device.clone(),
            handle,
        })
    }
}

impl Drop for UsamiShader {
    fn drop(&mut self) {
        unsafe { self.device.destroy_shader_module(self.handle, None) }
    }
}

pub fn record_command_buffer_with_image_dep<
    F: Fn(&UsamiDevice, CommandBuffer, Image) -> ImageLayout,
>(
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

    let old_image_layout = callback(device, command_buffer, image);

    unsafe {
        device.vk_device.cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::BOTTOM_OF_PIPE,
            PipelineStageFlags::TRANSFER,
            DependencyFlags::empty(),
            &[],
            &[],
            &[ImageMemoryBarrier::builder()
                .src_access_mask(AccessFlags::MEMORY_WRITE)
                .dst_access_mask(AccessFlags::TRANSFER_READ)
                .old_layout(old_image_layout)
                .new_layout(ImageLayout::TRANSFER_SRC_OPTIMAL)
                .src_queue_family_index(device.vk_queue_index)
                .dst_queue_family_index(device.vk_queue_index)
                .image(image)
                .subresource_range(image_subresource_range)
                .build()],
        );

        device.vk_device.cmd_copy_image_to_buffer(
            command_buffer,
            device.presentation_image().handle,
            ImageLayout::TRANSFER_SRC_OPTIMAL,
            device.presentation_buffer_readback().handle,
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

        device.vk_device.end_command_buffer(command_buffer)?;
    }

    Ok(())
}

use std::thread::JoinHandle;

use ash::{
    prelude::VkResult,
    vk::{self, QueryPoolCreateInfo, QueryType, QueueFlags},
};
use usami::{UsamiDevice, UsamiInstance};

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct Vertex {
    pub pos: Vec4,
    pub uv: Vec2,
}

#[derive(Clone, Debug, Copy, Default)]
#[repr(packed(1))]
#[allow(dead_code)]
struct UniformBlock {
    bias: f32,
    reference: f32,
    _padding1: f32,
    _padding2: f32,
    color_scale: [f32; 4],
    color_bias: [f32; 4],
}

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let image_buffer =
        image::load_from_memory(include_bytes!("../../resources/debug/images/128x64.png"))
            .unwrap()
            .to_rgba8();

    let width = image_buffer.width();
    let height = image_buffer.height();

    let instance = UsamiInstance::new("debug", "usami", vk::API_VERSION_1_1, &extensions, true)?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[],
        width,
        height,
        Box::new(|physical_device| {
            physical_device
                .queue_familiy_properties
                .iter()
                .enumerate()
                .find_map(|(i, x)| {
                    if x.queue_flags.contains(QueueFlags::GRAPHICS) {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
                .map(|x| (physical_device, x))
        }),
    )?;

    let test = |vk_device: ash::Device| {
        let max_object_count = 100;

        for _ in 0..max_object_count {
            unsafe {
                let query_pool = vk_device
                    .create_query_pool(
                        &QueryPoolCreateInfo::builder()
                            .query_type(QueryType::OCCLUSION)
                            .query_count(1)
                            .build(),
                        None,
                    )
                    .unwrap();
                vk_device.destroy_query_pool(query_pool, None);
            }
        }
    };

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let thread_count = 8;

    for _ in 0..thread_count {
        let vk_device = device.handle.clone();

        handles.push(
            std::thread::Builder::new()
                .spawn(move || test(vk_device))
                .unwrap(),
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

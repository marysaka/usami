use ash::{prelude::VkResult, vk};
use usami::{UsamiDevice, UsamiInstance};

fn main() -> VkResult<()> {
    let extensions = [
        "VK_EXT_debug_utils".into(),
        "VK_KHR_get_physical_device_properties2".into(),
    ];

    let instance = UsamiInstance::new("simple", "usami", vk::API_VERSION_1_0, &extensions, true)?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[],
        Box::new(|physical_device| {
            physical_device
                .queue_familiy_properties
                .iter()
                .enumerate()
                .find_map(|(i, x)| {
                    if x.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        Some(i as u32)
                    } else {
                        None
                    }
                })
                .map(|x| (physical_device, x))
        }),
    )?;

    let queue = UsamiDevice::get_device_queue(&device, "queue".into(), device.vk_queue_index, 0)?;
    Ok(())
}

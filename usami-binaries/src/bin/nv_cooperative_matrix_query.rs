use ash::{
    prelude::VkResult,
    vk::{self},
};
use usami::{UsamiDevice, UsamiInstance};
use usami_binaries::ash_ext::NvCooperativeMatrix;

fn main() -> VkResult<()> {
    let extensions = ["VK_EXT_debug_utils".into()];

    let instance = UsamiInstance::new(
        "nv_cooperative_matrix_query",
        "usami",
        vk::API_VERSION_1_1,
        &extensions,
        true,
    )?;
    let device = UsamiDevice::new_by_filter(
        instance,
        &[NvCooperativeMatrix::NAME.to_string_lossy().to_string()],
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

    let cooperative_matrix =
        NvCooperativeMatrix::new(&device.instance.vk_entry, &device.instance.vk_instance);

    let cooperative_matrix_props = unsafe {
        cooperative_matrix
            .get_physical_device_cooperative_matrix_properties(device.physical_device.handle)
    }?;

    for (idx, prop) in cooperative_matrix_props.iter().enumerate() {
        println!("props[{idx}] = {prop:?}");
    }

    Ok(())
}

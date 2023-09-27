use std::{
    ffi::CString,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use argh::FromArgs;
use ash::{
    extensions::ext::ShaderObject,
    prelude::VkResult,
    vk::{self, DescriptorSet, DescriptorSetLayout, ShaderCodeTypeEXT, ShaderStageFlags},
};
use spirv_reflect::{types::ReflectDescriptorType, ShaderModule};
use usami::{descriptor::UsamiDescriptorSetLayout, UsamiDevice, UsamiInstance};

#[derive(FromArgs, Debug)]
/// Reach new heights.
struct Args {
    /// spir-v file path.
    #[argh(positional)]
    spirv_path: PathBuf,

    /// output directory path.s
    #[argh(positional)]
    output_directory_path: PathBuf,

    /// id of the vendor to use.
    #[argh(option)]
    vendor_id: Option<usize>,

    /// specific entrypoint to dump.
    #[argh(option)]
    entry_point: Option<String>,

    /// enable update after bind.
    #[argh(switch)]
    update_after_bind: bool,
}

fn read_spirv_file(file_path: &Path) -> Vec<u32> {
    let f = File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read file");

    let mut spirv = Vec::new();
    assert!(buffer.len() % 4 == 0);
    for i in 0..(buffer.len() / 4) {
        let bytes = buffer[(i * 4)..(i * 4 + 4)].try_into().unwrap();
        spirv.push(u32::from_ne_bytes(bytes));
    }
    spirv
}

fn create_device(vendor_id: Option<usize>) -> VkResult<Arc<UsamiDevice>> {
    let instance = UsamiInstance::new(
        "shader_dumper",
        "usami",
        vk::API_VERSION_1_2,
        &["VK_EXT_debug_utils".into()],
        false,
    )?;
    UsamiDevice::new_by_filter(
        instance,
        &[ShaderObject::name().to_string_lossy().into()],
        Box::new(move |physical_device| {
            if let Some(vendor_id) = vendor_id {
                if physical_device.properties.vendor_id != vendor_id as u32 {
                    return None;
                }
            }

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
    )
}

fn next_stages(stage: vk::ShaderStageFlags) -> vk::ShaderStageFlags {
    if stage == vk::ShaderStageFlags::VERTEX {
        vk::ShaderStageFlags::TESSELLATION_CONTROL
            | vk::ShaderStageFlags::GEOMETRY
            | vk::ShaderStageFlags::FRAGMENT
    } else if stage == vk::ShaderStageFlags::TESSELLATION_CONTROL {
        vk::ShaderStageFlags::TESSELLATION_EVALUATION
    } else if stage == vk::ShaderStageFlags::TESSELLATION_EVALUATION {
        vk::ShaderStageFlags::GEOMETRY | vk::ShaderStageFlags::FRAGMENT
    } else if stage == vk::ShaderStageFlags::GEOMETRY {
        vk::ShaderStageFlags::FRAGMENT
    } else if stage == vk::ShaderStageFlags::FRAGMENT {
        vk::ShaderStageFlags::empty()
    } else {
        panic!("Unsupported shader stage");
    }
}

fn create_descriptor_set_layouts(
    device: &Arc<UsamiDevice>,
    reflection_module: &ShaderModule,
    entry_point: &str,
    stage: vk::ShaderStageFlags,
    args: &Args,
) -> Result<Vec<UsamiDescriptorSetLayout>, String> {
    let reflection_sets = reflection_module
        .enumerate_descriptor_sets(Some(entry_point))?;

    let max_set = reflection_sets.iter().map(|s| s.set).max().unwrap_or(0);
    let num_sets = usize::try_from(max_set).unwrap() + 1;

    let mut reflection_set_bindings = Vec::new();
    reflection_set_bindings.resize_with(num_sets, || Vec::new());
    for reflection_set in reflection_sets {
        let set_idx = usize::try_from(reflection_set.set).unwrap();
        reflection_set_bindings[set_idx] = reflection_set.bindings;
    }

    let mut sets = Vec::new();
    for reflection_set_binding in reflection_set_bindings {
        let mut has_update_after_bind = false;
        let mut binding_flags = Vec::new();
        let mut bindings = Vec::new();
        for reflection_binding in reflection_set_binding {
            if args.update_after_bind {
                let mut flags = vk::DescriptorBindingFlags::empty();
                match reflection_binding.descriptor_type {
                    ReflectDescriptorType::Sampler
                    | ReflectDescriptorType::CombinedImageSampler
                    | ReflectDescriptorType::SampledImage
                    | ReflectDescriptorType::StorageImage => {
                        has_update_after_bind = true;
                        flags |= vk::DescriptorBindingFlags::UPDATE_AFTER_BIND;
                    }
                    _ => (),
                }
                binding_flags.push(flags);
            }

            let desc_type = match reflection_binding.descriptor_type {
                ReflectDescriptorType::Undefined => panic!("Unknown descriptor type"),
                ReflectDescriptorType::Sampler => vk::DescriptorType::SAMPLER,
                ReflectDescriptorType::CombinedImageSampler => {
                    vk::DescriptorType::COMBINED_IMAGE_SAMPLER
                }
                ReflectDescriptorType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
                ReflectDescriptorType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
                ReflectDescriptorType::UniformTexelBuffer => {
                    vk::DescriptorType::UNIFORM_TEXEL_BUFFER
                }
                ReflectDescriptorType::StorageTexelBuffer => {
                    vk::DescriptorType::STORAGE_TEXEL_BUFFER
                }
                ReflectDescriptorType::UniformBuffer => vk::DescriptorType::UNIFORM_BUFFER,
                ReflectDescriptorType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
                ReflectDescriptorType::UniformBufferDynamic => {
                    vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
                }
                ReflectDescriptorType::StorageBufferDynamic => {
                    vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
                }
                ReflectDescriptorType::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
                ReflectDescriptorType::AccelerationStructureNV => {
                    vk::DescriptorType::ACCELERATION_STRUCTURE_KHR
                }
            };
            bindings.push(vk::DescriptorSetLayoutBinding {
                binding: reflection_binding.binding,
                descriptor_type: desc_type,
                descriptor_count: reflection_binding.count,
                stage_flags: stage,
                ..Default::default()
            });
        }

        let mut layout_flags = vk::DescriptorSetLayoutBindingFlagsCreateInfo {
            binding_count: binding_flags.len().try_into().unwrap(),
            p_binding_flags: binding_flags.as_ptr(),
            ..Default::default()
        };

        let mut layout_info_builder =
            vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);
        if has_update_after_bind {
            layout_info_builder.flags |= vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL;
            layout_info_builder = layout_info_builder.push_next(&mut layout_flags);
        }

        let layout_info = layout_info_builder.build();
        let set_layout =
            UsamiDevice::create_descriptor_set_layout(device, "set_layout".into(), layout_info)
                .map_err(|x| format!("Vulkan error: {x}"))?;
        sets.push(set_layout);
    }

    Ok(sets)
}

pub struct Shader {
    pub name: String,
    pub stage: String,
    pub data: Vec<u8>,
}

fn compile_shaders(
    device: &Arc<UsamiDevice>,
    spirv: &Vec<u32>,
    args: &Args,
) -> Result<Vec<Shader>, String> {
    let reflection_module = ShaderModule::load_u32_data(spirv)?;
    let entrypoints = reflection_module.enumerate_entry_points()?;
    let eso = ShaderObject::new(&device.instance.vk_instance, &device.handle);

    let mut shaders = Vec::new();
    for e in entrypoints {
        if let Some(arg_ename) = &args.entry_point {
            if &e.name != arg_ename {
                continue;
            }
        }

        let stage = vk::ShaderStageFlags::from_raw(e.shader_stage.bits());

        let c_name = CString::new(e.name.as_str()).unwrap();

        {
            let set_layouts =
                create_descriptor_set_layouts(&device, &reflection_module, &e.name, stage, args)
                    .map_err(|x| format!("Vulkan error: {x}"))?;
            let set_layouts_handle = set_layouts
                .as_slice()
                .iter()
                .map(|x| x.handle)
                .collect::<Vec<DescriptorSetLayout>>();

            let shader_info = vk::ShaderCreateInfoEXT::builder()
                .stage(stage)
                .next_stage(next_stages(stage))
                .code_type(ShaderCodeTypeEXT::SPIRV)
                .code(unsafe {
                    std::slice::from_raw_parts(spirv.as_ptr() as *const _, spirv.len() * 4)
                })
                .name(c_name.as_c_str())
                .set_layouts(&set_layouts_handle)
                .build();

            let bin = unsafe {
                let shader_object = eso
                    .create_shaders(&[shader_info], None)
                    .map_err(|x| format!("Vulkan error: {x}"))?[0];

                let bin: Vec<u8> = eso
                    .get_shader_binary_data(shader_object)
                    .map_err(|x| format!("Vulkan error: {x}"))?;

                eso.destroy_shader(shader_object, None);

                bin
            };

            std::mem::drop(set_layouts);

            let mut stages = Vec::new();

            if stage & ShaderStageFlags::VERTEX == ShaderStageFlags::VERTEX {
                stages.push("vert");
            }

            if stage & ShaderStageFlags::TESSELLATION_CONTROL == ShaderStageFlags::TESSELLATION_CONTROL {
                stages.push("tesc");
            }

            if stage & ShaderStageFlags::TESSELLATION_EVALUATION == ShaderStageFlags::TESSELLATION_EVALUATION {
                stages.push("tese");
            }

            if stage & ShaderStageFlags::GEOMETRY == ShaderStageFlags::GEOMETRY {
                stages.push("geom");
            }

            if stage & ShaderStageFlags::FRAGMENT == ShaderStageFlags::FRAGMENT {
                stages.push("frag");
            }

            if stage & ShaderStageFlags::COMPUTE == ShaderStageFlags::COMPUTE {
                stages.push("comp");
            }

            let stage = stages.join(".");

            shaders.push(Shader {
                name: e.name.clone(),
                stage,
                data: bin,
            });
        }
    }

    Ok(shaders)
}

fn main() {
    let args: Args = argh::from_env();
    println!("{:?}", args);

    let spirv_data = read_spirv_file(&args.spirv_path);
    let device: Arc<UsamiDevice> = create_device(args.vendor_id).unwrap();
    let shaders = compile_shaders(&device, &spirv_data, &args).unwrap();
}

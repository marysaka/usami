use ash::{
    extensions::{ext::{MeshShader, ShaderObject}, khr::CooperativeMatrix},
    prelude::VkResult,
    vk::{self, DescriptorSetLayout, ShaderCodeTypeEXT, ShaderStageFlags},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use hyper::{body::Bytes, header};
use serde_json::json;
use spirv_reflect::{types::ReflectDescriptorType, ShaderModule};
use std::{ffi::CString, net::SocketAddr, sync::Arc};
use tower_http::limit::RequestBodyLimitLayer;
use usami::{descriptor::UsamiDescriptorSetLayout, UsamiDevice, UsamiInstance};

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;

fn create_instance() -> VkResult<UsamiInstance> {
    UsamiInstance::new(
        "shader_dumper",
        "usami",
        vk::API_VERSION_1_2,
        &["VK_EXT_debug_utils".into()],
        false,
    )
}

fn create_device(vendor_id: Option<usize>, device_id: Option<usize>) -> VkResult<Arc<UsamiDevice>> {
    UsamiDevice::new_by_filter(
        create_instance()?,
        &[
            ShaderObject::NAME.to_string_lossy().into(),
            MeshShader::NAME.to_string_lossy().into(),
            CooperativeMatrix::NAME.to_string_lossy().into(),
        ],
        Box::new(move |physical_device| {
            if let Some(vendor_id) = vendor_id {
                if physical_device.properties.vendor_id != vendor_id as u32 {
                    return None;
                }
            }

            if let Some(device_id) = device_id {
                if physical_device.properties.device_id != device_id as u32 {
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
    } else if stage == vk::ShaderStageFlags::FRAGMENT || stage == vk::ShaderStageFlags::COMPUTE {
        vk::ShaderStageFlags::empty()
    } else if stage == vk::ShaderStageFlags::TASK_EXT {
        vk::ShaderStageFlags::MESH_EXT
    } else if stage == vk::ShaderStageFlags::MESH_EXT {
        vk::ShaderStageFlags::FRAGMENT
    } else {
        panic!("Unsupported shader stage");
    }
}

fn create_descriptor_set_layouts(
    device: &Arc<UsamiDevice>,
    reflection_module: &ShaderModule,
    entry_point: &str,
    stage: vk::ShaderStageFlags,
    update_after_bind: bool,
) -> Result<Vec<UsamiDescriptorSetLayout>, String> {
    let reflection_sets = reflection_module.enumerate_descriptor_sets(Some(entry_point))?;

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
            if update_after_bind {
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
                ReflectDescriptorType::AccelerationStructureKHR => {
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

        let mut layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        if has_update_after_bind {
            layout_info.flags |= vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL;
            layout_info = layout_info.push_next(&mut layout_flags);
        }

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
    spirv: &[u8],
    entry_point: &str,
) -> Result<Vec<Shader>, String> {
    let reflection_module = ShaderModule::load_u8_data(spirv)?;
    let entrypoints = reflection_module.enumerate_entry_points()?;
    let eso = ShaderObject::new(&device.instance.vk_instance, &device.handle);

    let mut shaders = Vec::new();
    for e in entrypoints {
        if e.name != entry_point {
            continue;
        }

        let stage = vk::ShaderStageFlags::from_raw(e.shader_stage.bits());

        let c_name = CString::new(e.name.as_str()).unwrap();

        {
            let set_layouts =
                create_descriptor_set_layouts(&device, &reflection_module, &e.name, stage, true)
                    .map_err(|x| format!("Vulkan error: {x}"))?;
            let set_layouts_handle = set_layouts
                .as_slice()
                .iter()
                .map(|x| x.handle)
                .collect::<Vec<DescriptorSetLayout>>();

            let shader_info = vk::ShaderCreateInfoEXT::default()
                .stage(stage)
                .next_stage(next_stages(stage))
                .code_type(ShaderCodeTypeEXT::SPIRV)
                .code(spirv)
                .name(c_name.as_c_str())
                .set_layouts(&set_layouts_handle);

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

            if stage & ShaderStageFlags::TESSELLATION_CONTROL
                == ShaderStageFlags::TESSELLATION_CONTROL
            {
                stages.push("tesc");
            }

            if stage & ShaderStageFlags::TESSELLATION_EVALUATION
                == ShaderStageFlags::TESSELLATION_EVALUATION
            {
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

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/devices", get(list_devices))
        .route(
            "/get_shader_binary",
            get(show_get_shader_binary_form).post(get_shader_binary_form),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250MiB */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 9999));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct PhysicalDeviceInformation {
    pub device_name: String,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
}

enum ServerError {
    ErrorMessage(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::ErrorMessage(error) => (StatusCode::SERVICE_UNAVAILABLE, error),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

async fn list_devices() -> Result<Json<Vec<PhysicalDeviceInformation>>, ServerError> {
    let instance = match create_instance() {
        Ok(instance) => instance,
        Err(error) => {
            return Err(ServerError::ErrorMessage(format!(
                "Cannot create instance: {error}"
            )))
        }
    };

    let physical_devices = match unsafe { instance.vk_instance.enumerate_physical_devices() } {
        Ok(physical_devices) => physical_devices,
        Err(error) => {
            return Err(ServerError::ErrorMessage(format!(
                "enumerate_physical_devices failed: {error}"
            )))
        }
    };

    let result = physical_devices
        .iter()
        .map(|x| {
            let prop: vk::PhysicalDeviceProperties =
                unsafe { instance.vk_instance.get_physical_device_properties(*x) };
            let device_name = prop.device_name;
            let device_name_size = device_name
                .iter()
                .enumerate()
                .find(|(_, x)| **x == 0)
                .map(|(i, _)| i)
                .unwrap_or(device_name.len());

            PhysicalDeviceInformation {
                device_name: unsafe {
                    CString::new(std::slice::from_raw_parts(
                        prop.device_name.as_ptr() as *const _,
                        device_name_size,
                    ))
                }
                .unwrap()
                .to_string_lossy()
                .into(),
                driver_version: prop.driver_version,
                device_id: prop.device_id,
                vendor_id: prop.vendor_id,
            }
        })
        .collect::<Vec<PhysicalDeviceInformation>>();

    Ok(Json(result))
}

#[derive(TryFromMultipart)]
struct ShaderBinaryRequestData {
    pub vendor_id: usize,
    pub device_id: usize,
    pub entry_point: String,
    pub file: FieldData<Bytes>,
}

async fn show_get_shader_binary_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/get_shader_binary" method="post" enctype="multipart/form-data">
                    <label>
                        Vendor ID:
                        <input type="text" name="vendor_id" value="4318" required />
                    </label>

                    <label>
                        Device ID:
                        <input type="text" name="device_id" value="7956" required />
                    </label>

                    <label>
                        Entrypoint:
                        <input type="text" name="entry_point" value="main" required />
                    </label>

                    <label>
                        Upload SPIR-V file:
                        <input type="file" name="file" required />
                    </label>

                    <input type="submit" value="Upload file">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn get_shader_binary_form(
    TypedMultipart(ShaderBinaryRequestData {
        vendor_id,
        device_id,
        entry_point,
        file,
    }): TypedMultipart<ShaderBinaryRequestData>,
) -> Result<Response, Response> {
    let file_name = file.metadata.file_name.unwrap_or(String::from("data.spv"));
    let file_data = file.contents.to_vec();
    let output_file_name = file_name.replace(".spv", ".bin");
    let device = create_device(Some(vendor_id), Some(device_id)).map_err(|error| {
        ServerError::ErrorMessage(format!("create_device failed: {error}")).into_response()
    })?;
    let mut shaders =
        compile_shaders(&device, &file_data, entry_point.as_str()).map_err(|error| {
            ServerError::ErrorMessage(format!("compile_shaders failed: {error}")).into_response()
        })?;
    assert!(shaders.len() == 1);
    let shader_data = shaders.remove(0).data;

    let headers = [
        (header::CONTENT_TYPE, "application/octet-stream".into()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{output_file_name}\""),
        ),
    ];

    Ok((headers, shader_data).into_response())
}

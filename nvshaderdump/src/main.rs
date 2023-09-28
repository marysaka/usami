use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use argh::FromArgs;
use reqwest::multipart::Part;

#[derive(FromArgs)]
/// Reach new heights.
struct Args {
    /// the path to the SPIR-V file to send.
    #[argh(positional)]
    spirv_file_path: PathBuf,

    /// the hostname of the server to do the request.
    #[argh(option)]
    hostname: String,

    /// the port of the server to do the request.
    #[argh(option)]
    port: u32,

    /// the SPIR-V entrypoint.
    #[argh(option)]
    entrypoint: Option<String>,

    /// the vendor id.
    #[argh(option)]
    vendor_id: usize,

    /// the device id.
    #[argh(option)]
    device_id: usize,

    /// the optional output directory to store the shader code and header.
    #[argh(option)]
    output_directory: Option<PathBuf>,
}

async fn get_shader_binary(args: &Args) -> Vec<u8> {
    let url = format!("http://{}:{}/get_shader_binary", args.hostname, args.port);

    let mut spirv_data = Vec::new();

    let mut file = File::open(&args.spirv_file_path).unwrap();

    file.read_to_end(&mut spirv_data).unwrap();

    let form = reqwest::multipart::Form::new()
        .text(
            "entry_point",
            args.entrypoint.clone().unwrap_or("main".into()),
        )
        .text("vendor_id", args.vendor_id.to_string())
        .text("device_id", args.device_id.to_string())
        .part("file", Part::bytes(spirv_data));

    let response = reqwest::Client::new()
        .post(url)
        .multipart(form)
        .send()
        .await
        .expect("send");

    let mut result = Vec::new();

    if response.status() != 200 {
        let error_txt = response.text_with_charset("utf-8").await.unwrap();

        panic!("Server replied with error: {error_txt}")
    }

    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut result).unwrap();

    result
}

pub fn find_zstd_header(bin: &[u8]) -> Option<usize> {
    if bin.len() < 4 {
        return None;
    }

    let header = 0xfd2fb528_u32.to_ne_bytes();
    for i in 0..(bin.len() - 3) {
        if bin[i..(i + 4)] == header {
            return Some(i);
        }
    }
    None
}
pub fn get_zstd_shader_blob(bin: &[u8]) -> Option<Vec<u8>> {
    if let Some(offset) = find_zstd_header(bin) {
        Some(zstd::stream::decode_all(&bin[offset..]).unwrap())
    } else {
        None
    }
}

pub fn get_shader_data(dec: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let sz = u32::from_ne_bytes(dec[112..116].try_into().unwrap());
    let hdr_start = usize::try_from(sz).unwrap() + 56; /* Why? */
    let hdr_end = hdr_start + 128;

    let bin_start = hdr_end;
    let mut bin_end = hdr_end;
    while bin_end < dec.len() {
        if dec[bin_end..(bin_end + 16)] == [0; 16] {
            break;
        }
        bin_end += 16;
    }

    (
        dec[hdr_start..hdr_end].into(),
        dec[bin_start..bin_end].into(),
    )
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    let shader_binary = get_shader_binary(&args).await;

    if let Some(output_directory) = &args.output_directory {
        std::fs::create_dir_all(output_directory).unwrap();
    }

    if let Some(output_directory) = &args.output_directory {
        let mut file = File::create(output_directory.join("shader_binary_raw.bin")).unwrap();
        file.write_all(&shader_binary).unwrap();
    }

    if let Some(dec) = get_zstd_shader_blob(&shader_binary) {
        if let Some(output_directory) = &args.output_directory {
            let mut file = File::create(output_directory.join("shader_zstd_dec.bin")).unwrap();
            file.write_all(&dec).unwrap();
        }

        let (shader_header_data, shader_binary_data) = get_shader_data(&dec);

        if let Some(output_directory) = &args.output_directory {
            let mut file = File::create(output_directory.join("shader_header.bin")).unwrap();
            file.write_all(&shader_header_data).unwrap();
        }

        if let Some(output_directory) = &args.output_directory {
            let mut file = File::create(output_directory.join("shader_data.bin")).unwrap();
            file.write_all(&shader_binary_data).unwrap();
        }
    } else {
        eprintln!("ZSTD header not found!");
    }
}

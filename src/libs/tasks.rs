use std::fs;
use std::path::{Path, PathBuf};

use super::cli::InputOutputArgs;
use super::files::TextureOutputFormat;
use super::{wilay, compressonator};

pub fn decode_wilay(
        io_args: InputOutputArgs,
        format: TextureOutputFormat
) {
    io_args.assert_valid_args();

    if io_args.input_file.is_some() {
        let input_file = io_args.input_file.as_ref().unwrap().to_owned();
        let output_file = io_args.get_output_file(".dds".to_string()).expect("failed to process task!");

        decode_wilay_internal(input_file, output_file, format);
    } else if io_args.input_dir.is_some() {
        let input_dir = Path::new(io_args.input_dir.as_ref().unwrap());
        let output_dir = PathBuf::from(io_args.output_dir.as_ref().unwrap());

        if !input_dir.is_dir() {
            panic!()
        }

        if !output_dir.exists() {
            fs::create_dir(&output_dir).unwrap();
        }

        for path in fs::read_dir(input_dir).unwrap() {
            let path = path.unwrap().path();
            let input_file = path.clone().to_str().unwrap().to_string();
            let output_file = format!("{}.dds", path.file_stem().unwrap().to_str().unwrap());
            let output_file = output_dir.join(output_file);

            decode_wilay_internal(input_file, output_file.to_str().unwrap().to_string(), format.clone())
        }
    }
}

fn decode_wilay_internal(
        input_file: String,
        output_file: String,
        format: TextureOutputFormat
) {
    wilay::decode_wilay(input_file, output_file.clone());

    let out_dir = Path::new(&output_file).parent().expect("failed");

    if matches!(format, TextureOutputFormat::Png) {
        compressonator::decompress_dds(output_file.clone(), out_dir)
    }
}
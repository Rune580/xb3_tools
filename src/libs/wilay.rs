use std::{path::Path, fs};

use binary_reader::BinaryReader;
use clap::Parser;

use super::{textures::LahdTexture, dds::write_dds, lbim, swizzler};

pub fn decode_wilay(
    file_path_string: String,
    output_string: String
) {
    let file_path = Path::new(&file_path_string);
    let bytes = fs::read(file_path).expect("failed to read file!");
    let mut reader = BinaryReader::from_vec(&bytes);
    reader.set_endian(binary_reader::Endian::Little);

    let magic = String::from_utf8(reader.read_bytes(4).expect("failed to read bytes!").to_vec()).expect("failed to read string!");

    if magic != "LAHD" {
        panic!("File is not a LAHD texture!");
    }

    reader.jmp(36);
    let texture_offset = reader.read_i32().unwrap();
    reader.jmp(texture_offset as usize);

    let offset = reader.read_i32().unwrap();
    let texture_count = reader.read_i32().unwrap() as usize;

    reader.jmp((texture_offset + offset) as usize);

    let mut offsets: Vec<TextureOffset> = Vec::new();
    let mut textures: Vec<LahdTexture> = Vec::new();

    for _ in 0..texture_count {
        let tex_offset = TextureOffset {
            _field0: reader.read_i32().unwrap(),
            offset: reader.read_i32().unwrap(),
            length: reader.read_i32().unwrap()
        };
        offsets.push(tex_offset);
    }

    for i in 0..texture_count {
        let footer_start = (texture_offset + offsets[i].offset + offsets[i].length - 56) as usize;

        reader.jmp((texture_offset + offsets[i].offset) as usize);
        let raw = reader.read_bytes(offsets[i].length as usize).unwrap().to_owned();

        reader.jmp(footer_start);
        let mut texture = LahdTexture{
            field0: reader.read_i32().unwrap(),
            field4: reader.read_i32().unwrap(),
            field8: reader.read_i32().unwrap(),
            fieldC: reader.read_i32().unwrap(),
            field10: reader.read_i32().unwrap(),
            field14: reader.read_i32().unwrap(),
            width: reader.read_u32().unwrap(),
            height: reader.read_u32().unwrap(),
            depth: reader.read_i32().unwrap(),
            field24: reader.read_i32().unwrap(),
            format: reader.read_i32().unwrap().into(),
            field30: reader.read_i32().unwrap(),
            field34: reader.read_i32().unwrap(),
            raw: raw
        };

        //swizzler::deswizzle(&mut texture);

        lbim::decode_lahd(&mut texture);

        textures.push(texture);
    }

    for i in 0..texture_count {
        write_dds(&mut textures.get(i).unwrap(), &output_string);
    }
}

#[derive(Debug, Clone, Copy)]
struct TextureOffset {
    _field0: i32,
    offset: i32,
    length: i32
}
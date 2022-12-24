use std::{fs::File, path::Path};

use binary_rw::{
    BinaryWriter,
    FileStream
};

use super::textures::LahdTexture;

pub fn write_dds(
        texture: &LahdTexture,
        out_file: &String
) {
    let file = File::create(Path::new(&out_file)).expect("Failed to create dds file!");
    let mut fs = FileStream::new(file);
    let mut writer = BinaryWriter::new(&mut fs, binary_rw::Endian::Little);

    // Header
    writer.write_u32(0x20534444);
    let normal_header = DDSHeader::new(texture);
    normal_header.serialize(&mut writer);
    deafult_dx10_header.serialize(&mut writer);

    // Texture
    writer.write_bytes(texture.raw.to_owned());
}

#[derive(Debug, Clone, Copy)]
struct DDSHeader {
    flags: u32,
    height: u32,
    width: u32,
    pitch_or_linear_size: u32,
    depth: u32,
    mip_map_count: u32,
    pixel_format: DDSPixelFormat,
    caps1: u32,
    caps2: u32,
    caps3: u32,
    caps4: u32,
    reserved2: u32
}

impl DDSHeader {
    pub fn new(
            texture: &LahdTexture
    ) -> DDSHeader {
        let flags: u32 = 0x01 | 0x02 | 0x04 | 0x1000 | 0x80000;

        let pitch: u32 = std::cmp::max(1, ((texture.width + 3) / 4) as u32) * 16;

        DDSHeader {
            flags: flags,
            height: texture.height as u32,
            width: texture.width as u32,
            pitch_or_linear_size: pitch,
            depth: texture.depth as u32,
            mip_map_count: 0,
            pixel_format: default_pixel_format,
            caps1: 0x1000,
            caps2: 0x0,
            caps3: 0x0,
            caps4: 0x0,
            reserved2: 0
        }
    }
}

impl BinarySerializable for DDSHeader {
    fn serialize(&self, writer: &mut BinaryWriter) {
        writer.write_u32(124);
        writer.write_u32(self.flags);
        writer.write_u32(self.height);
        writer.write_u32(self.width);
        writer.write_u32(self.pitch_or_linear_size);
        writer.write_u32(self.depth);
        writer.write_u32(self.mip_map_count);
        for i in 0..11 {
            writer.write_u32(0);
        }
        self.pixel_format.serialize(writer);
        writer.write_u32(self.caps1);
        writer.write_u32(self.caps2);
        writer.write_u32(self.caps3);
        writer.write_u32(self.caps4);
        writer.write_u32(self.reserved2);
    }
}

const default_pixel_format: DDSPixelFormat = DDSPixelFormat {
    flags: 0x04,
    four_cc: 0x30315844,
    rgb_bit_count: 0,
    r_bit_mask: 0,
    g_bit_mask: 0,
    b_bit_mask: 0,
    a_bit_mask: 0
};

#[derive(Debug, Clone, Copy)]
struct  DDSPixelFormat {
    flags: u32,
    four_cc: u32,
    rgb_bit_count: u32,
    r_bit_mask: u32,
    g_bit_mask: u32,
    b_bit_mask: u32,
    a_bit_mask: u32
}

impl BinarySerializable for DDSPixelFormat {
    fn serialize(&self, writer: &mut BinaryWriter) {
        writer.write_u32(32);
        writer.write_u32(self.flags);
        writer.write_u32(self.four_cc);
        writer.write_u32(self.rgb_bit_count);
        writer.write_u32(self.r_bit_mask);
        writer.write_u32(self.g_bit_mask);
        writer.write_u32(self.b_bit_mask);
        writer.write_u32(self.a_bit_mask);
    }
}

const deafult_dx10_header: DDSHeaderDX10 = DDSHeaderDX10 {
    format: 98,
    dimension: 3,
    misc_flag1: 0x0,
    array_size: 0x0,
    misc_flag2: 0x0
};

#[derive(Debug, Clone, Copy)]
struct DDSHeaderDX10 {
    format: u32,
    dimension: u32,
    misc_flag1: u32,
    array_size: u32,
    misc_flag2: u32
}

impl BinarySerializable for DDSHeaderDX10 {
    fn serialize(&self, writer: &mut BinaryWriter) {
        writer.write_u32(self.format);
        writer.write_u32(self.dimension);
        writer.write_u32(self.misc_flag1);
        writer.write_u32(self.array_size);
        writer.write_u32(self.misc_flag2);
    }
}

pub trait BinarySerializable {
    fn serialize(&self, writer: &mut BinaryWriter);
}
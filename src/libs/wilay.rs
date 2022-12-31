use std::{fs, fmt::Display};

use binary_reader::BinaryReader;

use super::{textures::LahdTexture, dds::write_dds, lbim, files::InputOutputPair};

pub fn decode_wilay(
        io: InputOutputPair
) -> Result<(), Error> {
    let bytes = fs::read(io.input).expect("failed to read file!");
    let mut reader = BinaryReader::from_vec(&bytes);
    reader.set_endian(binary_reader::Endian::Little);

    let magic = String::from_utf8(reader.read_bytes(4)?.to_vec()).expect("failed to read string!");

    if magic != "LAHD" {
        return Err(Error::from("File is not a LAHD texture!"));
    }

    reader.jmp(36);
    let texture_offset = reader.read_i32()?;
    reader.jmp(texture_offset as usize);

    let offset = reader.read_i32()?;
    let texture_count = reader.read_i32()? as usize;

    reader.jmp((texture_offset + offset) as usize);

    let mut offsets: Vec<TextureOffset> = Vec::new();
    let mut textures: Vec<LahdTexture> = Vec::new();

    for _ in 0..texture_count {
        let tex_offset = TextureOffset {
            _field0: reader.read_i32()?,
            offset: reader.read_i32()?,
            length: reader.read_i32()?
        };
        offsets.push(tex_offset);
    }

    for i in 0..texture_count {
        let footer_start = (texture_offset + offsets[i].offset + offsets[i].length - 56) as usize;

        reader.jmp((texture_offset + offsets[i].offset) as usize);
        let raw = reader.read_bytes(offsets[i].length as usize)?.to_owned();

        reader.jmp(footer_start);
        let mut texture = LahdTexture{
            field0: reader.read_i32()?,
            field4: reader.read_i32()?,
            field8: reader.read_i32()?,
            fieldC: reader.read_i32()?,
            field10: reader.read_i32()?,
            field14: reader.read_i32()?,
            width: reader.read_u32()?,
            height: reader.read_u32()?,
            depth: reader.read_i32()?,
            field24: reader.read_i32()?,
            format: reader.read_i32()?.into(),
            field30: reader.read_i32()?,
            field34: reader.read_i32()?,
            raw: raw
        };

        lbim::decode_lahd(&mut texture);

        textures.push(texture);
    }

    for i in 0..texture_count {
        write_dds(&mut textures.get(i).unwrap(), io.output.clone());
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct TextureOffset {
    _field0: i32,
    offset: i32,
    length: i32
}

#[derive(Debug, Clone)]
pub struct Error {
    msg: String
}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Self {
        Error { msg: io_err.kind().to_string() }
    }
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Error { msg: msg.to_string() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}
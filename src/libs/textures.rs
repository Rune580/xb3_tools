#[derive(Debug)]
pub struct LahdTexture {
    pub field0: i32,
    pub field4: i32,
    pub field8: i32,
    pub fieldC: i32,
    pub field10: i32,
    pub field14: i32,
    pub width: u32,
    pub height: u32,
    pub depth: i32,
    pub field24: i32,
    pub format: TextureFormat,
    pub field30: i32,
    pub field34: i32,
    pub raw: Vec<u8>
}

#[derive(Debug)]
pub enum TextureFormat {
    BC1 = 0x42,
    BC3 = 0x44,
    BC4 = 0x49,
    BC7 = 0x4D,
    BC6HUF16 = 0x50,
    R8G8B8A8UNORM = 0x25
}

impl From<i32> for TextureFormat {
    fn from(val: i32) -> Self {
        match val {
            0x42 => TextureFormat::BC1,
            0x44 => TextureFormat::BC3,
            0x49 => TextureFormat::BC4,
            0x4D => TextureFormat::BC7,
            0x50 => TextureFormat::BC6HUF16,
            0x25 => TextureFormat::R8G8B8A8UNORM,
            _ => panic!()
        }
    }
}
use std::cmp;

use super::textures::{LahdTexture, TextureFormat};

pub fn decode_lahd(
        texture: &mut LahdTexture
) {
    let mut result: Vec<u8> = vec![0; texture.raw.len()];

    let mut bpp = 0;
    let mut ppb = 1;

    if matches!(texture.format, TextureFormat::BC7) {
        bpp = 16;
        ppb = 4;
    }

    let b_width = texture.width / ppb;
    let b_height = texture.height / ppb;
    let surface_size = b_width * b_height * bpp;
    let aligned_surface_size = b_width * round_up_pow_two(b_height) * bpp;

    let bpp_shift = num_leading_zeros(bpp);
    let line_shift = find_last_bit_index(b_width * bpp) - 1;

    let mut x_bits_shift = 3;
    let h_bit_mask = round_up_pow_two(b_height) - 1;
    for i in 3u32..7u32 {
        if (h_bit_mask & (1 << i)) != 0 {
            x_bits_shift += 1;
        }
    }

    let num_slices: u32 = cmp::max(texture.depth as u32, 1);

    for slice in 0..num_slices {
        let input_addr = surface_size * slice;
        let output_addr = aligned_surface_size * slice;

        for w in 0..b_width {
            for h in 0..b_height {
                let x = w << bpp_shift;
                let addr = (h & Y_BITS_TAIL) << line_shift |
                           ((h & Y_BITS_9_12) << 6) |
                           ((h & Y_BITS_6_7) << 5) |
                           ((h & Y_BITS_HEAD) << 4) |
                           ((x & X_BITS_TAIL) << x_bits_shift) |
                           ((x & X_BITS_EIGHTH) << 3) |
                           ((x & X_BITS_FITH) << 1) |
                           (x & X_BITS_0_3);

                if addr + bpp > aligned_surface_size {
                    continue;
                }

                let start = (input_addr + addr) as usize;
                let end = start + bpp as usize;
                let block = texture.raw[start..end].to_owned();

                let start = (output_addr + ((h * b_width + w) * bpp)) as usize;
                let end = start + bpp as usize;
                result.splice(start..end, block);
            }
        }
    }

    texture.raw = result;
}

fn num_leading_zeros(
        value: u32
) -> u32 {
    let mut num_zeros = 0u32;
    while ((value >> num_zeros) & 1) == 0 {
        num_zeros += 1;
    }
    num_zeros
}

fn find_last_bit(
        value: u32
) -> u32 {
    let mut value = value;
    let mut last_bit = 0u32;
    let mut index = 0u32;
    while value != 0 {
        if (value & 1) != 0 {
            last_bit = 1 << index;
        }
        value >>= 1;
        index += 1;
    }
    last_bit
}

fn find_last_bit_index(
        value: u32
) -> u32 {
    let mut value = value;
    let mut index = 0u32;
    while value != 0 {
        value >>= 1;
        index += 1;
    }
    index
}

fn round_up_pow_two(
        value: u32
) -> u32 {
    let mut value = value - 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value += 1;
    return value;
}

fn round_size(
        size: u32,
        pad: u32
) -> u32 {
    let mask = pad - 1;
    let mut size = size;
    if (size & mask) != 0 {
        size &= !mask;
        size += pad;
    }
    size
}

fn get_addr(
        x: u32,
        y: u32,
        x_bit_shift: u32,
        y_bit_shift: u32,
        width: u32
) -> u32 {
    let mut x = x;
    let mut y = y;
    let mut x_count = 0u32;
    let mut y_count = 1u32;
    let mut x_used = 0u32;
    let mut y_used = 0u32;
    let mut addr = 0u32;

    while x_used < 2 && x_used + x_count < x_bit_shift {
        let x_mask = (1u32 << x_count) - 1;
        let y_mask = (1u32 << y_count) - 1;

        addr |= (x & x_mask) << x_used + y_used;
        addr |= (y & y_mask) << x_used + y_used + x_count;

        x >>= x_count;
        y >>= y_count;

        x_used += x_count;
        y_used += y_count;

        x_count = cmp::max(cmp::min(x_bit_shift - x_used, 1), 0);
        y_count = cmp::max(cmp::min(y_bit_shift - y_used, y_count << 1), 0);
    }

    addr |= (x + y * (width >> x_used)) << (x_used + y_used);
    addr
}

const Y_BITS_TAIL: u32 = 0b1111111110000000;
const Y_BITS_9_12: u32 = 0b0000000001111000;
const Y_BITS_6_7: u32 = 0b0000000000000110;
const Y_BITS_HEAD: u32 = 0b0000000000000001;
const X_BITS_TAIL: u32 = 0b1111111111000000;
const X_BITS_EIGHTH: u32 = 0b0000000000100000;
const X_BITS_FITH: u32 = 0b0000000000010000;
const X_BITS_0_3: u32 = 0b0000000000001111;
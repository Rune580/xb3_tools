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

    let b_width = (texture.width + 3) / ppb;
    let b_height = (texture.height + 3) / ppb;
    let surface_size = b_width * b_height * bpp;
    let aligned_surface_size = b_width * round_up_pow_two(b_height) * bpp;

    let xb = num_leading_zeros(round_up_pow_two(b_width));
    let mut yb = num_leading_zeros(round_up_pow_two(b_height));
    let hh = round_up_pow_two(b_height) >> 1;

    if (b_height & (b_height - 1)) != 0 && b_height <= hh + hh / 3 && yb > 3 {
        yb -= 1;
    }

    let width = round_size(b_width, 64 >> 4);

    let num_slices: u32 = cmp::max(texture.depth as u32, 1);

    for slice in 0..num_slices {
        let input_addr = surface_size * slice;
        let output_addr = aligned_surface_size * slice;

        for h in 0..b_height {
            for w in 0..b_width {
                let addr = get_addr(w, h, xb, yb, width) * bpp;

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
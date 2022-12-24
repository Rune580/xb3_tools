use super::textures::LahdTexture;

pub fn deswizzle(
        texture: &mut LahdTexture
) {
    let mut result: Vec<u8> = texture.raw.clone();
    let blocks: usize = texture.raw.len() / 16;
    let rows = (texture.height / 4) as f32;
    let columns = (texture.width / 4) as f32;

    for i in 0..(blocks / 4) {
        let block: usize = i * 4;

        let block_x0_y0 = slice_block(&texture.raw, block);
        let block_x1_y0 = slice_block(&texture.raw, block + 2);
        let block_x0_y1 = slice_block(&texture.raw, block + 1);
        let block_x1_y1 = slice_block(&texture.raw, block + 3);

        let y = ((block as f32) / rows).floor() as usize;
        let x = block - (y * (rows as usize));

        splice_block(&mut result, block_x0_y0, columns, rows, x, y);
        splice_block(&mut result, block_x1_y0, columns, rows, x + 1, y);
        splice_block(&mut result, block_x0_y1, columns, rows, x, y + 1);
        splice_block(&mut result, block_x1_y1, columns, rows, x + 1, y + 1);
    }

    texture.raw = result;
}

fn slice_block(
        raw: &Vec<u8>,
        block: usize
) -> Vec<u8> {
    let start = block * 16;
    let end = (block + 1) * 16;
    raw[start..end].to_owned()
}

fn splice_block(
        raw: &mut Vec<u8>,
        slice: Vec<u8>,
        columns: f32,
        rows: f32,
        x: usize,
        y: usize
) {
    if y >= rows as usize {
        return;
    }

    let start = block_pos(columns, x, y);
    let end = block_pos(columns, x + 1, y);

    raw.splice(start..end, slice);
}

fn block_pos(
        columns: f32,
        column: usize,
        row: usize
) -> usize {
    (column * 16) + ((columns as usize) * row * 16)
}
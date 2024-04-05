use crate::Image;

pub(crate) fn blockhash<
    I: Image,
    const BITS: u32,
    const NUM_BLOCKS: usize,
    const DIGEST_SIZE: usize,
>(
    img: &I,
) -> [u8; DIGEST_SIZE] {
    debug_assert_eq!(BITS % 4, 0);
    debug_assert_ne!(BITS, 0);

    let (width, height) = img.dimensions();

    let values = if width % BITS == 0 && height % BITS == 0 {
        get_values_aligned::<I, BITS, NUM_BLOCKS>(img)
    } else if width >= BITS && height >= BITS {
        get_values_larger::<I, BITS, NUM_BLOCKS>(img)
    } else {
        get_values_generic::<I, BITS, NUM_BLOCKS>(img)
    };

    convert_to_bits(width, height, &values, I::MAX_BRIGHTNESS)
}

fn get_values_aligned<I: Image, const BITS: u32, const NUM_BLOCKS: usize>(
    img: &I,
) -> [u64; NUM_BLOCKS] {
    // These values are related, but need to be passed in separately due to
    // limitations with const generics.
    debug_assert_eq!(NUM_BLOCKS, (BITS * BITS) as usize);

    let (width, height) = img.dimensions();
    let block_width = width / BITS;
    let block_height = height / BITS;

    let mut values = [0_u64; NUM_BLOCKS];

    for y in 0..height {
        let block_y = y / block_height;
        let idx_row = (block_y * BITS) as usize;

        for x in 0..width {
            let block_x = x / block_width;
            let idx_x = block_x as usize;

            let brightness = u64::from(img.brightness(x, y));

            values[idx_row + idx_x] += brightness * NUM_BLOCKS as u64;
        }
    }

    values
}

fn get_values_larger<I: Image, const BITS: u32, const NUM_BLOCKS: usize>(
    img: &I,
) -> [u64; NUM_BLOCKS] {
    // These values are related, but need to be passed in separately due to
    // limitations with const generics.
    debug_assert_eq!(NUM_BLOCKS, (BITS * BITS) as usize);

    let (width, height) = img.dimensions();
    let (width, height) = (u64::from(width), u64::from(height));

    let mut values = [0_u64; NUM_BLOCKS];

    let mut block_top;
    let mut block_bottom = 0;

    let mut weight_top = u64::from(BITS);
    let mut weight_bottom = 0;

    for y in 0..height {
        block_top = block_bottom;

        let end_y = (y + 1) * u64::from(BITS) % height;
        if end_y < u64::from(BITS) {
            block_bottom += 1;
            weight_top = u64::from(BITS) - end_y;
            weight_bottom = end_y;
        }

        let idx_top = (block_top * BITS) as usize;
        let idx_bottom = if block_bottom < BITS {
            (block_bottom * BITS) as usize
        } else {
            0 // to avoid out-of-bounds access (the weight will be zero)
        };

        let mut block_left;
        let mut block_right = 0;

        let mut weight_left = u64::from(BITS);
        let mut weight_right = 0;

        for x in 0..width {
            block_left = block_right;

            let end_x = (x + 1) * u64::from(BITS) % width;
            if end_x < u64::from(BITS) {
                block_right += 1;
                weight_left = u64::from(BITS) - end_x;
                weight_right = end_x;
            }

            let idx_left = block_left as usize;
            let idx_right = if block_right < BITS {
                block_right as usize
            } else {
                0 // to avoid out-of-bounds access (the weight will be zero)
            };

            let brightness = u64::from(img.brightness(x as u32, y as u32));

            values[idx_top + idx_left] += brightness * weight_top * weight_left;
            values[idx_top + idx_right] += brightness * weight_top * weight_right;
            values[idx_bottom + idx_left] += brightness * weight_bottom * weight_left;
            values[idx_bottom + idx_right] += brightness * weight_bottom * weight_right;
        }
    }

    values
}

fn get_values_generic<I: Image, const BITS: u32, const NUM_BLOCKS: usize>(
    img: &I,
) -> [u64; NUM_BLOCKS] {
    // These values are related, but need to be passed in separately due to
    // limitations with const generics.
    debug_assert_eq!(NUM_BLOCKS, (BITS * BITS) as usize);

    let (width, height) = img.dimensions();
    let (width, height) = (u64::from(width), u64::from(height));

    let mut values = [0_u64; NUM_BLOCKS];

    let mut block_top;
    let mut block_bottom = 0;

    let mut weight_top = u64::from(BITS);
    let mut weight_bottom = 0;

    for y in 0..height {
        block_top = block_bottom;

        let end_y = (y + 1) * u64::from(BITS) % height;
        if end_y < u64::from(BITS) {
            block_bottom = (y + 1) * u64::from(BITS) / height;
            weight_top = (u64::from(BITS) - 1 - end_y) % height + 1;
            weight_bottom = end_y;
        }

        let idx_top = (block_top * u64::from(BITS)) as usize;
        let idx_bottom = if block_bottom < u64::from(BITS) {
            (block_bottom * u64::from(BITS)) as usize
        } else {
            0 // to avoid out-of-bounds access (the weight will be zero)
        };

        let mut block_left;
        let mut block_right = 0;

        let mut weight_left = u64::from(BITS);
        let mut weight_right = 0;

        for x in 0..width {
            block_left = block_right;

            let end_x = (x + 1) * u64::from(BITS) % width;
            if end_x < u64::from(BITS) {
                block_right = (x + 1) * u64::from(BITS) / width;
                weight_left = (u64::from(BITS) - 1 - end_x) % width + 1;
                weight_right = end_x;
            }

            let idx_left = block_left as usize;
            let idx_right = if block_right < u64::from(BITS) {
                block_right as usize
            } else {
                0 // to avoid out-of-bounds access (the weight will be zero)
            };

            let brightness = u64::from(img.brightness(x as u32, y as u32));

            values[idx_top + idx_left] += brightness * weight_top * weight_left;
            values[idx_top + idx_right] += brightness * weight_top * weight_right;
            values[idx_bottom + idx_left] += brightness * weight_bottom * weight_left;
            values[idx_bottom + idx_right] += brightness * weight_bottom * weight_right;

            for bx in (block_left + 1)..block_right {
                let idx_x = bx as usize;
                values[idx_top + idx_x] += brightness * weight_top * width;
                values[idx_bottom + idx_x] += brightness * weight_bottom * width;
            }

            for by in (block_top + 1)..block_bottom {
                let idx_y = (by * u64::from(BITS)) as usize;
                values[idx_y + idx_left] += brightness * height * weight_left;
                values[idx_y + idx_right] += brightness * height * weight_right;
            }

            let full_value = brightness * width * height;
            for by in (block_top + 1)..block_bottom {
                let idx_y = (by * u64::from(BITS)) as usize;
                for bx in (block_left + 1)..block_right {
                    let idx_x = bx as usize;
                    values[idx_y + idx_x] += full_value;
                }
            }
        }
    }

    values
}

fn convert_to_bits<const NUM_BLOCKS: usize, const DIGEST_SIZE: usize>(
    width: u32,
    height: u32,
    values: &[u64; NUM_BLOCKS],
    max_value: u32,
) -> [u8; DIGEST_SIZE] {
    // These values are related, but need to be passed in separately due to
    // limitations with const generics.
    debug_assert_eq!(NUM_BLOCKS, DIGEST_SIZE * 8);

    let band_size: usize = NUM_BLOCKS / 4;
    let half_value = u64::from(max_value) * u64::from(width) * u64::from(height) / 2;

    let mut bands = *values;
    let mut bits = [0_u8; NUM_BLOCKS];

    for i in 0..4 {
        let offset = i * band_size;

        let band = &mut bands[offset..(offset + band_size)];
        band.sort_unstable();

        let median = (band[band_size / 2 - 1] + band[band_size / 2]) / 2;

        for (n, &val) in values.iter().enumerate().skip(offset).take(band_size) {
            if val > median || (val == median && val > half_value) {
                bits[n] = 1;
            } else {
                bits[n] = 0;
            }
        }
    }

    let mut res = [0_u8; DIGEST_SIZE];

    for (i, octet) in bits.chunks(8).enumerate() {
        for &bit in octet {
            res[i] <<= 1;
            res[i] |= bit;
        }
    }

    res
}

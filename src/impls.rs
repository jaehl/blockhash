use crate::{BlockhashParseError, Image};

#[inline(always)]
#[allow(clippy::many_single_char_names)]
fn get_value<I: Image>(img: &I, x: u32, y: u32) -> u64 {
    match img.get_pixel(x, y) {
        [.., 0] => u64::from(u8::MAX) * 3,
        [r, g, b, _] => u64::from(r) + u64::from(g) + u64::from(b),
    }
}

fn parse_char(c: u8) -> Result<u8, BlockhashParseError> {
    let val = match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - (b'a' - 10),
        b'A'..=b'F' => c - (b'A' - 10),
        _ => return Err(BlockhashParseError),
    };
    Ok(val)
}

macro_rules! blockhash_impl {
    ($mod:ident, $bits:expr) => {
        pub mod $mod {
            use super::*;
            use core::fmt::{self, Formatter};

            const HASH_SIZE: usize = $bits * $bits;
            const HASH_BYTES: usize = HASH_SIZE / 8;
            const BAND_SIZE: usize = HASH_SIZE / 4;

            pub fn blockhash<I: Image>(img: &I) -> [u8; HASH_BYTES] {
                let (width, height) = img.dimensions();

                let values = if width % $bits == 0 && height % $bits == 0 {
                    get_values_aligned(img)
                } else if width >= $bits && height >= $bits {
                    get_values_larger(img)
                } else {
                    get_values(img)
                };

                convert_to_bits(width, height, values)
            }

            pub fn from_str(s: &str) -> Result<[u8; HASH_BYTES], BlockhashParseError> {
                let s = s.as_bytes();

                if s.len() != HASH_BYTES * 2 {
                    return Err(BlockhashParseError);
                }

                let mut bytes = [0; HASH_BYTES];

                for i in 0..HASH_BYTES {
                    bytes[i] = (parse_char(s[2 * i])? << 4) | parse_char(s[2 * i + 1])?;
                }

                Ok(bytes)
            }

            pub fn fmt(f: &mut Formatter, hash: [u8; HASH_BYTES]) -> fmt::Result {
                for byte in &hash {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }

            pub fn distance(left: &[u8; HASH_BYTES], right: &[u8; HASH_BYTES]) -> u32 {
                let mut dist = 0;

                for i in 0..HASH_BYTES {
                    dist += (left[i] ^ right[i]).count_ones();
                }

                dist
            }

            fn get_values_aligned<I: Image>(img: &I) -> [u64; HASH_SIZE] {
                let (width, height) = img.dimensions();
                let block_width = width / $bits;
                let block_height = height / $bits;

                let mut values = [0u64; HASH_SIZE];

                for y in 0..height {
                    let block_y = y / block_height;
                    let idx_y = block_y as usize * $bits;

                    for x in 0..width {
                        let block_x = x / block_width;
                        let idx_x = block_x as usize;

                        let value = get_value(img, x, y);

                        values[idx_y + idx_x] += value * ($bits * $bits);
                    }
                }

                values
            }

            fn get_values_larger<I: Image>(img: &I) -> [u64; HASH_SIZE] {
                let (width, height) = img.dimensions();
                let (width, height) = (u64::from(width), u64::from(height));

                let mut values = [0u64; HASH_SIZE];

                let mut block_top;
                let mut block_bottom = 0;

                let mut weight_top = $bits;
                let mut weight_bottom = 0;

                for y in 0..height {
                    block_top = block_bottom;

                    let end_y = (y + 1) * $bits % height;
                    if end_y < $bits {
                        block_bottom += 1;
                        weight_top = $bits - end_y;
                        weight_bottom = end_y;
                    }

                    let idx_top = block_top as usize * $bits;
                    let idx_bottom = if block_bottom < $bits {
                        block_bottom as usize * $bits
                    } else {
                        0 // to avoid overflows (the weight will be zero)
                    };

                    let mut block_left;
                    let mut block_right = 0;

                    let mut weight_left = $bits;
                    let mut weight_right = 0;

                    for x in 0..width {
                        block_left = block_right;

                        let end_x = (x + 1) * $bits % width;
                        if end_x < $bits {
                            block_right += 1;
                            weight_left = $bits - end_x;
                            weight_right = end_x;
                        }

                        let idx_left = block_left as usize;
                        let idx_right = if block_right < $bits {
                            block_right as usize
                        } else {
                            0 // to avoid overflows (the weight will be zero)
                        };

                        let value = get_value(img, x as u32, y as u32);

                        values[idx_top + idx_left] += value * weight_top * weight_left;
                        values[idx_top + idx_right] += value * weight_top * weight_right;
                        values[idx_bottom + idx_left] += value * weight_bottom * weight_left;
                        values[idx_bottom + idx_right] += value * weight_bottom * weight_right;
                    }
                }

                values
            }

            fn get_values<I: Image>(img: &I) -> [u64; HASH_SIZE] {
                let (width, height) = img.dimensions();
                let (width, height) = (u64::from(width), u64::from(height));

                let mut values = [0u64; HASH_SIZE];

                let mut block_top;
                let mut block_bottom = 0;

                let mut weight_top = $bits;
                let mut weight_bottom = 0;

                for y in 0..height {
                    block_top = block_bottom;

                    let end_y = (y + 1) * $bits % height;
                    if end_y < $bits {
                        block_bottom = (y + 1) * $bits / height;
                        weight_top = ($bits - 1 - end_y) % height + 1;
                        weight_bottom = end_y;
                    }

                    let idx_top = block_top as usize * $bits;
                    let idx_bottom = if block_bottom < $bits {
                        block_bottom as usize * $bits
                    } else {
                        0 // to avoid overflows (the weight will be zero)
                    };

                    let mut block_left;
                    let mut block_right = 0;

                    let mut weight_left = $bits;
                    let mut weight_right = 0;

                    for x in 0..width {
                        block_left = block_right;

                        let end_x = (x + 1) * $bits % width;
                        if end_x < $bits {
                            block_right = (x + 1) * $bits / width;
                            weight_left = ($bits - 1 - end_x) % width + 1;
                            weight_right = end_x;
                        }

                        let idx_left = block_left as usize;
                        let idx_right = if block_right < $bits {
                            block_right as usize
                        } else {
                            0 // to avoid overflows (the weight will be zero)
                        };

                        let value = get_value(img, x as u32, y as u32);

                        values[idx_top + idx_left] += value * weight_top * weight_left;
                        values[idx_top + idx_right] += value * weight_top * weight_right;
                        values[idx_bottom + idx_left] += value * weight_bottom * weight_left;
                        values[idx_bottom + idx_right] += value * weight_bottom * weight_right;

                        for bx in (block_left + 1)..block_right {
                            let idx_x = bx as usize;
                            values[idx_top + idx_x] += value * weight_top * width;
                            values[idx_bottom + idx_x] += value * weight_bottom * width;
                        }

                        for by in (block_top + 1)..block_bottom {
                            let idx_y = by as usize * $bits;
                            values[idx_y + idx_left] += value * height * weight_left;
                            values[idx_y + idx_right] += value * height * weight_right;
                        }

                        let full_value = value * width * height;
                        for by in (block_top + 1)..block_bottom {
                            let idx_y = by as usize * $bits;
                            for bx in (block_left + 1)..block_right {
                                let idx_x = bx as usize;
                                values[idx_y + idx_x] += full_value;
                            }
                        }
                    }
                }

                values
            }

            fn convert_to_bits(
                width: u32,
                height: u32,
                mut values: [u64; HASH_SIZE],
            ) -> [u8; HASH_BYTES] {
                let half_value = u64::from(u8::MAX) * 3 * u64::from(width) * u64::from(height) / 2;

                let mut buf = [0u64; BAND_SIZE];

                for band in values.chunks_mut(BAND_SIZE) {
                    buf.copy_from_slice(band);
                    buf.sort_unstable();
                    let median = (buf[BAND_SIZE / 2 - 1] + buf[BAND_SIZE / 2]) / 2;

                    for x in band {
                        if *x > median || (*x == median && *x > half_value) {
                            *x = 1;
                        } else {
                            *x = 0;
                        }
                    }
                }

                let mut res = [0u8; HASH_BYTES];

                for (i, octet) in values.chunks(8).enumerate() {
                    for bit in octet {
                        res[i] <<= 1;
                        res[i] |= *bit as u8;
                    }
                }

                res
            }
        }
    };
}

blockhash_impl!(impl16, 4);
blockhash_impl!(impl64, 8);
blockhash_impl!(impl144, 12);
blockhash_impl!(impl256, 16);

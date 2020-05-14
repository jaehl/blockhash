//! A perceptual hashing algorithm for detecting similar images.
//!
//! This is an implementation of the [Blockhash] algorithm, and can produce 16-,
//! 64-, 144-, and 256-bit perceptual hashes.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! # #[cfg(feature = "image")]
//! # {
//! use blockhash::blockhash256_string;
//!
//! let img = image::open("tests/images/512x512_rgb.png").unwrap();
//! let hash = blockhash256_string(&img);
//!
//! assert_eq!(
//!     hash,
//!     "9cfde03dc4198467ad671d171c071c5b1ff81bf919d9181838f8f890f807ff01",
//! );
//! # }
//! ```
//!
//! # Feature flags
//!
//! * `std`: Enables features that require the Rust Standard Library (enabled by
//!   default).
//! * `image`: Enables integration with the `image` crate.
//!
//! [Blockhash]: http://blockhash.io

#![cfg_attr(not(feature = "std"), no_std)]

/// Provides access to image data.
pub trait Image {
    /// Returns the dimensions of the image.
    fn dimensions(&self) -> (u32, u32);

    /// Returns the channel data for a given pixel, in RGBA format.
    fn get_pixel(&self, x: u32, y: u32) -> [u8; 4];
}

#[cfg(feature = "image")]
impl<T, P> Image for T
where
    T: image::GenericImageView<Pixel = P>,
    P: image::Pixel<Subpixel = u8>,
{
    #[inline(always)]
    fn dimensions(&self) -> (u32, u32) {
        image::GenericImageView::dimensions(self)
    }

    #[inline(always)]
    fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        image::GenericImageView::get_pixel(self, x, y).to_rgba().0
    }
}

#[inline(always)]
#[allow(clippy::many_single_char_names)]
fn get_value<I: Image>(img: &I, x: u32, y: u32) -> u64 {
    match img.get_pixel(x, y) {
        [.., 0] => u64::from(u8::MAX) * 3,
        [r, g, b, _] => u64::from(r) + u64::from(g) + u64::from(b),
    }
}

macro_rules! blockhash_impl {
    ($mod:ident, $bits:expr) => {
        mod $mod {
            use super::*;

            const HASH_SIZE: usize = $bits * $bits;
            const HASH_BYTES: usize = HASH_SIZE / 8;
            const BAND_SIZE: usize = HASH_SIZE / 4;

            pub(super) fn blockhash<I: Image>(img: &I) -> [u8; HASH_BYTES] {
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

            #[cfg(feature = "std")]
            pub(super) fn blockhash_string<I: Image>(img: &I) -> String {
                use std::fmt::Write;

                let hash = blockhash(img);
                let mut res = String::new();
                for byte in &hash {
                    write!(res, "{:02x}", byte).unwrap();
                }
                res
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

/// Generates a 16-bit perceptual hash of an image as an array of bytes.
#[inline(always)]
pub fn blockhash16<I: Image>(img: &I) -> [u8; 2] {
    impl16::blockhash(img)
}

/// Generates a 16-bit perceptual hash of an image as an integer.
#[inline(always)]
pub fn blockhash16_int<I: Image>(img: &I) -> u16 {
    u16::from_be_bytes(blockhash16(img))
}

/// Generates a 16-bit perceptual hash of an image as a hex string.
#[cfg(feature = "std")]
#[inline(always)]
pub fn blockhash16_string<I: Image>(img: &I) -> String {
    impl16::blockhash_string(img)
}

/// Generates a 64-bit perceptual hash of an image as an array of bytes.
#[inline(always)]
pub fn blockhash64<I: Image>(img: &I) -> [u8; 8] {
    impl64::blockhash(img)
}

/// Generates a 64-bit perceptual hash of an image as an integer.
#[inline(always)]
pub fn blockhash64_int<I: Image>(img: &I) -> u64 {
    u64::from_be_bytes(blockhash64(img))
}

/// Generates a 64-bit perceptual hash of an image as a hex string.
#[cfg(feature = "std")]
#[inline(always)]
pub fn blockhash64_string<I: Image>(img: &I) -> String {
    impl64::blockhash_string(img)
}

/// Generates a 144-bit perceptual hash of an image as an array of bytes.
#[inline(always)]
pub fn blockhash144<I: Image>(img: &I) -> [u8; 18] {
    impl144::blockhash(img)
}

/// Generates a 144-bit perceptual hash of an image as a hex string.
#[cfg(feature = "std")]
#[inline(always)]
pub fn blockhash144_string<I: Image>(img: &I) -> String {
    impl144::blockhash_string(img)
}

/// Generates a 256-bit perceptual hash of an image as an array of bytes.
#[inline(always)]
pub fn blockhash256<I: Image>(img: &I) -> [u8; 32] {
    impl256::blockhash(img)
}

/// Generates a 256-bit perceptual hash of an image as a hex string.
#[cfg(feature = "std")]
#[inline(always)]
pub fn blockhash256_string<I: Image>(img: &I) -> String {
    impl256::blockhash_string(img)
}

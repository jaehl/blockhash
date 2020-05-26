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
//! # #[cfg(all(feature = "image"))] {
//! use blockhash::blockhash256;
//!
//! let img = image::open("tests/images/512x512_rgb.png").unwrap();
//! let hash = blockhash256(&img);
//!
//! assert_eq!(
//!     hash.to_string(),
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
#![cfg_attr(docsrs, feature(doc_cfg))]

use core::fmt::{self, Display, Formatter};
use core::str::FromStr;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BlockhashParseError;

impl Display for BlockhashParseError {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("invalid hash string")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BlockhashParseError {}

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

            pub(super) fn from_str(s: &str) -> Result<[u8; HASH_BYTES], BlockhashParseError> {
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

            pub(super) fn fmt(f: &mut Formatter, hash: [u8; HASH_BYTES]) -> fmt::Result {
                for byte in &hash {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }

            pub(super) fn distance(left: &[u8; HASH_BYTES], right: &[u8; HASH_BYTES]) -> u32 {
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

/// Generates a 16-bit perceptual hash of an image.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "image")] {
/// use blockhash::{blockhash16, Blockhash16};
///
/// let img = image::open("tests/images/512x512_rgb.png").unwrap();
/// let hash = blockhash16(&img);
///
/// assert_eq!(hash, Blockhash16::from([0x35, 0x6c]));
/// # }
/// ```
#[inline(always)]
pub fn blockhash16<I: Image>(img: &I) -> Blockhash16 {
    Blockhash16(impl16::blockhash(img))
}

/// A 16-bit hash digest.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blockhash16([u8; 2]);

impl Blockhash16 {
    /// Returns the Hamming distance between two hashes.
    ///
    /// # Examples
    ///
    /// ```
    /// use blockhash::Blockhash16;
    ///
    /// let a = Blockhash16::from([0xff, 0x80]);
    /// let b = Blockhash16::from([0xf7, 0xc1]);
    ///
    /// assert_eq!(a.distance(&b), 3);
    /// ```
    #[inline(always)]
    pub fn distance(&self, other: &Self) -> u32 {
        impl16::distance(&self.0, &other.0)
    }
}

impl FromStr for Blockhash16 {
    type Err = BlockhashParseError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl16::from_str(s).map(Self)
    }
}

impl Display for Blockhash16 {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl16::fmt(f, self.0)
    }
}

impl From<[u8; 2]> for Blockhash16 {
    #[inline(always)]
    fn from(bytes: [u8; 2]) -> Self {
        Blockhash16(bytes)
    }
}

impl From<Blockhash16> for [u8; 2] {
    #[inline(always)]
    fn from(hash: Blockhash16) -> Self {
        hash.0
    }
}

impl From<u16> for Blockhash16 {
    #[inline(always)]
    fn from(int: u16) -> Self {
        Blockhash16(int.to_be_bytes())
    }
}

impl From<Blockhash16> for u16 {
    #[inline(always)]
    fn from(hash: Blockhash16) -> Self {
        u16::from_be_bytes(hash.0)
    }
}

/// Generates a 64-bit perceptual hash of an image.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "image")] {
/// use blockhash::{blockhash64, Blockhash64};
///
/// let img = image::open("tests/images/512x512_rgb.png").unwrap();
/// let hash = blockhash64(&img);
///
/// assert_eq!(
///     hash,
///     Blockhash64::from([0xaf, 0x05, 0x75, 0x29, 0x7c, 0x4c, 0x4c, 0xe3]),
/// );
/// # }
/// ```
#[inline(always)]
pub fn blockhash64<I: Image>(img: &I) -> Blockhash64 {
    Blockhash64(impl64::blockhash(img))
}

/// A 64-bit hash digest.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blockhash64([u8; 8]);

impl Blockhash64 {
    /// Returns the Hamming distance between two hashes.
    ///
    /// # Examples
    ///
    /// ```
    /// use blockhash::Blockhash64;
    ///
    /// let a = Blockhash64::from([
    ///     0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef,
    /// ]);
    /// let b = Blockhash64::from([
    ///     0xd0, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xff,
    /// ]);
    ///
    /// assert_eq!(a.distance(&b), 4);
    /// ```
    #[inline(always)]
    pub fn distance(&self, other: &Self) -> u32 {
        impl64::distance(&self.0, &other.0)
    }
}

impl FromStr for Blockhash64 {
    type Err = BlockhashParseError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl64::from_str(s).map(Self)
    }
}

impl Display for Blockhash64 {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl64::fmt(f, self.0)
    }
}

impl From<[u8; 8]> for Blockhash64 {
    #[inline(always)]
    fn from(bytes: [u8; 8]) -> Self {
        Blockhash64(bytes)
    }
}

impl From<Blockhash64> for [u8; 8] {
    #[inline(always)]
    fn from(hash: Blockhash64) -> Self {
        hash.0
    }
}

impl From<u64> for Blockhash64 {
    #[inline(always)]
    fn from(int: u64) -> Self {
        Blockhash64(int.to_be_bytes())
    }
}

impl From<Blockhash64> for u64 {
    #[inline(always)]
    fn from(hash: Blockhash64) -> Self {
        u64::from_be_bytes(hash.0)
    }
}

/// Generates a 144-bit perceptual hash of an image.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "image")] {
/// use blockhash::{blockhash144, Blockhash144};
///
/// let img = image::open("tests/images/512x512_rgb.png").unwrap();
/// let hash = blockhash144(&img);
///
/// assert_eq!(
///     hash,
///     Blockhash144::from([
///         0x93, 0xfc, 0x0d, 0x91, 0x3b, 0xd3, 0x18, 0x33, 0x2b,
///         0x37, 0xc3, 0x7d, 0x30, 0x83, 0x28, 0xe2, 0xef, 0x83,
///     ]),
/// );
/// # }
/// ```
#[inline(always)]
pub fn blockhash144<I: Image>(img: &I) -> Blockhash144 {
    Blockhash144(impl144::blockhash(img))
}

/// A 144-bit hash digest.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blockhash144([u8; 18]);

impl Blockhash144 {
    /// Returns the Hamming distance between two hashes.
    ///
    /// # Examples
    ///
    /// ```
    /// use blockhash::Blockhash144;
    ///
    /// let a = Blockhash144::from([
    ///     0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x12, 0x34,
    /// ]);
    /// let b = Blockhash144::from([
    ///     0x00, 0x11, 0x22, 0x33, 0x22, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xe7, 0xff, 0x12, 0x34,
    /// ]);
    ///
    /// assert_eq!(a.distance(&b), 6);
    /// ```
    #[inline(always)]
    pub fn distance(&self, other: &Self) -> u32 {
        impl144::distance(&self.0, &other.0)
    }
}

impl FromStr for Blockhash144 {
    type Err = BlockhashParseError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl144::from_str(s).map(Self)
    }
}

impl Display for Blockhash144 {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl144::fmt(f, self.0)
    }
}

impl From<[u8; 18]> for Blockhash144 {
    #[inline(always)]
    fn from(bytes: [u8; 18]) -> Self {
        Blockhash144(bytes)
    }
}

impl From<Blockhash144> for [u8; 18] {
    #[inline(always)]
    fn from(hash: Blockhash144) -> Self {
        hash.0
    }
}

/// Generates a 256-bit perceptual hash of an image.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "image")] {
/// use blockhash::{blockhash256, Blockhash256};
///
/// let img = image::open("tests/images/512x512_rgb.png").unwrap();
/// let hash = blockhash256(&img);
///
/// assert_eq!(
///     hash,
///     Blockhash256::from([
///         0x9c, 0xfd, 0xe0, 0x3d, 0xc4, 0x19, 0x84, 0x67,
///         0xad, 0x67, 0x1d, 0x17, 0x1c, 0x07, 0x1c, 0x5b,
///         0x1f, 0xf8, 0x1b, 0xf9, 0x19, 0xd9, 0x18, 0x18,
///         0x38, 0xf8, 0xf8, 0x90, 0xf8, 0x07, 0xff, 0x01,
///     ]),
/// );
/// # }
/// ```
#[inline(always)]
pub fn blockhash256<I: Image>(img: &I) -> Blockhash256 {
    Blockhash256(impl256::blockhash(img))
}

/// A 256-bit hash digest.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blockhash256([u8; 32]);

impl Blockhash256 {
    /// Returns the Hamming distance between two hashes.
    ///
    /// # Examples
    ///
    /// ```
    /// use blockhash::Blockhash256;
    ///
    /// let a = Blockhash256::from([
    ///     0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
    ///     0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
    ///     0xff, 0xef, 0xdf, 0xcf, 0xbf, 0xaf, 0x9f, 0x8f,
    ///     0x7f, 0x6f, 0x5f, 0x4f, 0x3f, 0x2f, 0x1f, 0x0f,
    /// ]);
    /// let b = Blockhash256::from([
    ///     0x00, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
    ///     0xf8, 0xf9, 0x3a, 0xfb, 0xfc, 0xfd, 0x0e, 0xff,
    ///     0xff, 0xff, 0xdf, 0xcf, 0xbf, 0xaf, 0x9f, 0x8f,
    ///     0x7f, 0x6f, 0x5f, 0x4f, 0x3f, 0x2f, 0x1f, 0x0f,
    /// ]);
    ///
    /// assert_eq!(a.distance(&b), 11);
    /// ```
    #[inline(always)]
    pub fn distance(&self, other: &Self) -> u32 {
        impl256::distance(&self.0, &other.0)
    }
}

impl FromStr for Blockhash256 {
    type Err = BlockhashParseError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl256::from_str(s).map(Self)
    }
}

impl Display for Blockhash256 {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl256::fmt(f, self.0)
    }
}

impl From<[u8; 32]> for Blockhash256 {
    #[inline(always)]
    fn from(bytes: [u8; 32]) -> Self {
        Blockhash256(bytes)
    }
}

impl From<Blockhash256> for [u8; 32] {
    #[inline(always)]
    fn from(hash: Blockhash256) -> Self {
        hash.0
    }
}

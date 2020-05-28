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
//! let img = image::open("images/512x512_rgb.png").unwrap();
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
#![forbid(unsafe_code)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_qualifications)]

mod impls;

#[cfg(test)]
mod tests;

use core::fmt::{self, Display, Formatter};
use core::str::FromStr;
use impls::*;

/// Provides access to image data.
pub trait Image {
    /// Returns the dimensions of the image.
    fn dimensions(&self) -> (u32, u32);

    /// Returns the channel data for a given pixel, in RGBA format.
    fn get_pixel(&self, x: u32, y: u32) -> [u8; 4];
}

#[cfg(any(feature = "image", test))]
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

/// Generates a 16-bit perceptual hash of an image.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "image")] {
/// use blockhash::{blockhash16, Blockhash16};
///
/// let img = image::open("images/512x512_rgb.png").unwrap();
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
    #[allow(clippy::trivially_copy_pass_by_ref)]
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
/// let img = image::open("images/512x512_rgb.png").unwrap();
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
    #[allow(clippy::trivially_copy_pass_by_ref)]
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
/// let img = image::open("images/512x512_rgb.png").unwrap();
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
/// let img = image::open("images/512x512_rgb.png").unwrap();
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

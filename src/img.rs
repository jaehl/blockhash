/// Image data.
///
/// This trait can be implemented on image types in order to add support for
/// hashing.
///
/// If the `image` feature is enabled (the default), this trait is automatically
/// implemented for images from the [`image`] crate.
pub trait Image {
    /// The type of the pixels in the image.
    type Pixel: Pixel;

    /// Returns the dimensions of the image.
    fn dimensions(&self) -> (u32, u32);

    /// Returns the pixel at the given position in the image.
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel;
}

#[cfg(feature = "image")]
impl<T, IP, P> Image for T
where
    T: image::GenericImageView<Pixel = IP>,
    IP: IntoPixel<Pixel = P>,
    P: Pixel,
{
    type Pixel = P;

    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        image::GenericImageView::dimensions(self)
    }

    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        image::GenericImageView::get_pixel(self, x, y).into_pixel()
    }
}

/// An image pixel.
pub trait Pixel: Copy {
    /// The maximum brightness for a pixel.
    const MAX_BRIGHTNESS: u32;

    /// Returns the brightness of the pixel, in the range `0..=MAX_BRIGHTNESS`.
    fn brightness(self) -> u32;
}

/// A grayscale pixel.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Luma<T>(pub [T; 1]);

impl Pixel for Luma<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32;

    fn brightness(self) -> u32 {
        let Self([y]) = self;
        u32::from(y)
    }
}

impl Pixel for Luma<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32;

    fn brightness(self) -> u32 {
        let Self([y]) = self;
        u32::from(y)
    }
}

/// A grayscale pixel with an alpha channel.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LumaA<T>(pub [T; 2]);

impl Pixel for LumaA<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32;

    fn brightness(self) -> u32 {
        match self.0 {
            [_, 0] => Self::MAX_BRIGHTNESS,
            [y, _] => u32::from(y),
        }
    }
}

impl Pixel for LumaA<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32;

    fn brightness(self) -> u32 {
        match self.0 {
            [_, 0] => Self::MAX_BRIGHTNESS,
            [y, _] => u32::from(y),
        }
    }
}

/// An RGB pixel.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Rgb<T>(pub [T; 3]);

impl Pixel for Rgb<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32 * 3;

    fn brightness(self) -> u32 {
        let Self([r, g, b]) = self;
        u32::from(r) + u32::from(g) + u32::from(b)
    }
}

impl Pixel for Rgb<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32 * 3;

    fn brightness(self) -> u32 {
        let Self([r, g, b]) = self;
        u32::from(r) + u32::from(g) + u32::from(b)
    }
}

/// An RGB pixel with an alpha channel.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Rgba<T>(pub [T; 4]);

impl Pixel for Rgba<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32 * 3;

    fn brightness(self) -> u32 {
        match self.0 {
            [_, _, _, 0] => Self::MAX_BRIGHTNESS,
            [r, g, b, _] => u32::from(r) + u32::from(g) + u32::from(b),
        }
    }
}

impl Pixel for Rgba<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32 * 3;

    fn brightness(self) -> u32 {
        match self.0 {
            [_, _, _, 0] => Self::MAX_BRIGHTNESS,
            [r, g, b, _] => u32::from(r) + u32::from(g) + u32::from(b),
        }
    }
}

/// Helper trait for implementing [`Image`] on [`image::GenericImageView`].
#[cfg(feature = "image")]
pub trait IntoPixel {
    /// The type of pixel to convert to.
    type Pixel;

    /// Convert to a pixel.
    #[must_use]
    fn into_pixel(self) -> Self::Pixel;
}

macro_rules! impl_into_pixel {
    ($T:path, $U:path) => {
        #[cfg(feature = "image")]
        impl IntoPixel for $T {
            type Pixel = $U;

            #[inline]
            fn into_pixel(self) -> $U {
                $U(self.0)
            }
        }
    };
}

impl_into_pixel!(image::Luma<u8>, Luma<u8>);
impl_into_pixel!(image::LumaA<u8>, LumaA<u8>);
impl_into_pixel!(image::Rgb<u8>, Rgb<u8>);
impl_into_pixel!(image::Rgba<u8>, Rgba<u8>);
impl_into_pixel!(image::Luma<u16>, Luma<u16>);
impl_into_pixel!(image::LumaA<u16>, LumaA<u16>);
impl_into_pixel!(image::Rgb<u16>, Rgb<u16>);
impl_into_pixel!(image::Rgba<u16>, Rgba<u16>);

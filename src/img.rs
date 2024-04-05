use crate::Image;
use image::{GenericImageView, Luma, LumaA, Rgb, Rgba};

impl<T, P> Image for T
where
    T: GenericImageView<Pixel = P>,
    P: PixelExt,
{
    const MAX_BRIGHTNESS: u32 = P::MAX_BRIGHTNESS;

    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        GenericImageView::dimensions(self)
    }

    #[inline]
    fn brightness(&self, x: u32, y: u32) -> u32 {
        PixelExt::brightness(self.get_pixel(x, y))
    }
}

/// Extension trait for [`image`] pixel types.
trait PixelExt: Copy {
    /// The maximum possible brightness for a pixel.
    const MAX_BRIGHTNESS: u32;

    /// Returns the brightness of the pixel, in the range `0..=MAX_BRIGHTNESS`.
    fn brightness(self) -> u32;
}

impl PixelExt for Luma<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([y]) = self;
        u32::from(y)
    }
}

impl PixelExt for Luma<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([y]) = self;
        u32::from(y)
    }
}

impl PixelExt for LumaA<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([y, a]) = self;
        match a {
            0 => Self::MAX_BRIGHTNESS,
            _ => u32::from(y),
        }
    }
}

impl PixelExt for LumaA<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([y, a]) = self;
        match a {
            0 => Self::MAX_BRIGHTNESS,
            _ => u32::from(y),
        }
    }
}

impl PixelExt for Rgb<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32 * 3;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([r, g, b]) = self;
        u32::from(r) + u32::from(g) + u32::from(b)
    }
}

impl PixelExt for Rgb<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32 * 3;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([r, g, b]) = self;
        u32::from(r) + u32::from(g) + u32::from(b)
    }
}

impl PixelExt for Rgba<u8> {
    const MAX_BRIGHTNESS: u32 = u8::MAX as u32 * 3;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([r, g, b, a]) = self;
        match a {
            0 => Self::MAX_BRIGHTNESS,
            _ => u32::from(r) + u32::from(g) + u32::from(b),
        }
    }
}

impl PixelExt for Rgba<u16> {
    const MAX_BRIGHTNESS: u32 = u16::MAX as u32 * 3;

    #[inline]
    fn brightness(self) -> u32 {
        let Self([r, g, b, a]) = self;
        match a {
            0 => Self::MAX_BRIGHTNESS,
            _ => u32::from(r) + u32::from(g) + u32::from(b),
        }
    }
}

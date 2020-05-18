#![feature(test)]

extern crate test;

use blockhash::*;
use image::{DynamicImage, GenericImageView, Pixel};
use test::Bencher;

pub struct ImageProxy(pub DynamicImage);

impl Image for ImageProxy {
    #[inline(always)]
    fn dimensions(&self) -> (u32, u32) {
        GenericImageView::dimensions(&self.0)
    }

    #[inline(always)]
    fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        GenericImageView::get_pixel(&self.0, x, y).to_rgba().0
    }
}

macro_rules! bench_impl {
    ($name:ident, $func:ident, $path:expr) => {
        #[bench]
        fn $name(bencher: &mut Bencher) {
            let im = ImageProxy(image::open(concat!("tests/images/", $path)).unwrap());
            bencher.iter(|| $func(&im));
        }
    };
}

bench_impl!(blockhash16_256x256, blockhash16, "256x256_rgb.png");
bench_impl!(blockhash16_450x300, blockhash16, "450x300_rgb.png");
bench_impl!(blockhash16_512x512, blockhash16, "512x512_rgb.png");

bench_impl!(blockhash64_256x256, blockhash64, "256x256_rgb.png");
bench_impl!(blockhash64_450x300, blockhash64, "450x300_rgb.png");
bench_impl!(blockhash64_512x512, blockhash64, "512x512_rgb.png");

bench_impl!(blockhash144_256x256, blockhash144, "256x256_rgb.png");
bench_impl!(blockhash144_450x300, blockhash144, "450x300_rgb.png");
bench_impl!(blockhash144_512x512, blockhash144, "512x512_rgb.png");

bench_impl!(blockhash256_256x256, blockhash256, "256x256_rgb.png");
bench_impl!(blockhash256_450x300, blockhash256, "450x300_rgb.png");
bench_impl!(blockhash256_512x512, blockhash256, "512x512_rgb.png");

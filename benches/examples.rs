#![feature(test)]
#![cfg(feature = "image")]

extern crate test;

use blockhash::*;
use test::Bencher;

macro_rules! bench_impl {
    ($name:ident, $func:ident, $path:expr) => {
        #[bench]
        fn $name(bencher: &mut Bencher) {
            let im = image::open(concat!("images/", $path, ".png")).unwrap();
            bencher.iter(|| $func(&im));
        }
    };
}

bench_impl!(blockhash16_512x512_y, blockhash16, "512x512_y");
bench_impl!(blockhash16_241x159_ya, blockhash16, "241x159_ya");
bench_impl!(blockhash16_256x256_rgb, blockhash16, "256x256_rgb");
bench_impl!(blockhash16_450x300_rgb, blockhash16, "450x300_rgb");
bench_impl!(blockhash16_512x512_rgb, blockhash16, "512x512_rgb");
bench_impl!(blockhash16_256x256_rgb16, blockhash16, "256x256_rgb16");

bench_impl!(blockhash64_512x512_y, blockhash64, "512x512_y");
bench_impl!(blockhash64_241x159_ya, blockhash64, "241x159_ya");
bench_impl!(blockhash64_256x256_rgb, blockhash64, "256x256_rgb");
bench_impl!(blockhash64_450x300_rgb, blockhash64, "450x300_rgb");
bench_impl!(blockhash64_512x512_rgb, blockhash64, "512x512_rgb");
bench_impl!(blockhash64_256x256_rgb16, blockhash64, "256x256_rgb16");

bench_impl!(blockhash144_512x512_y, blockhash144, "512x512_y");
bench_impl!(blockhash144_241x159_ya, blockhash144, "241x159_ya");
bench_impl!(blockhash144_256x256_rgb, blockhash144, "256x256_rgb");
bench_impl!(blockhash144_450x300_rgb, blockhash144, "450x300_rgb");
bench_impl!(blockhash144_512x512_rgb, blockhash144, "512x512_rgb");
bench_impl!(blockhash144_256x256_rgb16, blockhash144, "256x256_rgb16");

bench_impl!(blockhash256_512x512_y, blockhash256, "512x512_y");
bench_impl!(blockhash256_241x159_ya, blockhash256, "241x159_ya");
bench_impl!(blockhash256_256x256_rgb, blockhash256, "256x256_rgb");
bench_impl!(blockhash256_450x300_rgb, blockhash256, "450x300_rgb");
bench_impl!(blockhash256_512x512_rgb, blockhash256, "512x512_rgb");
bench_impl!(blockhash256_256x256_rgb16, blockhash256, "256x256_rgb16");

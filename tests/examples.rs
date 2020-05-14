#![cfg(feature = "std")]

use blockhash::*;
use image::{DynamicImage, GenericImageView, Pixel};

pub struct ImageProxy(pub DynamicImage);

impl Image for ImageProxy {
    fn dimensions(&self) -> (u32, u32) {
        GenericImageView::dimensions(&self.0)
    }

    fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        GenericImageView::get_pixel(&self.0, x, y).to_rgba().0
    }
}

#[test]
fn example_16x16_rgb() {
    let im = ImageProxy(image::open("tests/images/16x16_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "c9cc");
    assert_eq!(blockhash64_string(&im), "f0f0e7c0d8f0f864");
    assert_eq!(
        blockhash144_string(&im),
        "fc0fc0f88f8ff04e40ee0ee0fa0ff07f8300",
    );
    assert_eq!(
        blockhash256_string(&im),
        "ff00ff00ff00fe20fc3efc18f900f980f3c0f7c0ef80fe00fee07ee05e7a1804",
    );
}

#[test]
fn example_1x1_rgb() {
    let im = ImageProxy(image::open("tests/images/1x1_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "ffff");
    assert_eq!(blockhash64_string(&im), "ffffffffffffffff");
    assert_eq!(
        blockhash144_string(&im),
        "ffffffffffffffffffffffffffffffffffff",
    );
    assert_eq!(
        blockhash256_string(&im),
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    );
}

#[test]
fn example_241x159_l() {
    let im = ImageProxy(image::open("tests/images/241x159_l.png").unwrap());
    assert_eq!(blockhash16_string(&im), "63a9");
    assert_eq!(blockhash64_string(&im), "3c3c2e4ecf84819f");
    assert_eq!(
        blockhash144_string(&im),
        "1fc1f80f81fc03c27ee3fa1dc01c01803fff",
    );
    assert_eq!(
        blockhash256_string(&im),
        "0ff80ff807f007f007f80df821fc11fcf0ffb0ff5031c021e001c00180ffffff",
    );
}

#[test]
fn example_256x256_rgb() {
    let im = ImageProxy(image::open("tests/images/256x256_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "9399");
    assert_eq!(blockhash64_string(&im), "f30300fff36083f8");
    assert_eq!(
        blockhash144_string(&im),
        "f8f30700f0018f8feffe7703700407e07ff0",
    );
    assert_eq!(
        blockhash256_string(&im),
        "ff1ffc0f000f001f000103a0f3bffbdffbeff90f70013804e01f800ffc03bff0",
    );
}

#[test]
fn example_26x17_rgb() {
    let im = ImageProxy(image::open("tests/images/26x17_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "6666");
    assert_eq!(blockhash64_string(&im), "047f3e1c3c3c7e0c");
    assert_eq!(
        blockhash144_string(&im),
        "0007f87fe1fe0f80f80e87fc1f07fe0f8038",
    );
    assert_eq!(
        blockhash256_string(&im),
        "00001db07ff43ffe0ffc07fc07f003f00bd01ff83ff803e03ffc1ff807f00070",
    );
}

#[test]
fn example_35x2_rgb() {
    let im = ImageProxy(image::open("tests/images/35x2_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "cccc");
    assert_eq!(blockhash64_string(&im), "f0f0f0f0f0f0f0f0");
    assert_eq!(
        blockhash144_string(&im),
        "fc0fc0fc0fc0fc0fc0fc0fc0fc0fc0fc0fc0",
    );
    assert_eq!(
        blockhash256_string(&im),
        "fe02fe02fe02fe02fe02fe02fe02fe02fe02fe02fe02fe02fe02fe02fe02fe02",
    );
}

#[test]
fn example_3x20_rgb() {
    let im = ImageProxy(image::open("tests/images/3x20_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "3633");
    assert_eq!(blockhash64_string(&im), "1f1c3c1c3f181f07");
    assert_eq!(
        blockhash144_string(&im),
        "0ff0ff0f00f00f00ff0ff00f0f000f0ff00f",
    );
    assert_eq!(
        blockhash256_string(&im),
        "03ff03ff03ff03c007ff07e003c007ffffff003f03c007e003ff07ff003f001f",
    );
}

#[test]
fn example_450x300_rgb() {
    let im = ImageProxy(image::open("tests/images/450x300_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "9995");
    assert_eq!(blockhash64_string(&im), "00ff01f702f70377");
    assert_eq!(
        blockhash144_string(&im),
        "000c0ffff00085ffdf00030ffff0003df3df",
    );
    assert_eq!(
        blockhash256_string(&im),
        "00000000ffffffff0000010fe73ffeff0000000ffebffe7f0000013f1f7f3f7f",
    );
}

#[test]
fn example_4x1_rgb() {
    let im = ImageProxy(image::open("tests/images/4x1_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "5555");
    assert_eq!(blockhash64_string(&im), "3333333333333333");
    assert_eq!(
        blockhash144_string(&im),
        "1c71c71c71c71c71c71c71c71c71c71c71c7",
    );
    assert_eq!(
        blockhash256_string(&im),
        "0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f",
    );
}

#[test]
fn example_4x4_rgb() {
    let im = ImageProxy(image::open("tests/images/4x4_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "c339");
    assert_eq!(blockhash64_string(&im), "f0f00f0f0f0fc3c3");
    assert_eq!(
        blockhash144_string(&im),
        "fc0fc0fc003f03f03f03f03f03fe07e07e07",
    );
    assert_eq!(
        blockhash256_string(&im),
        "ff00ff00ff00ff0000ff00ff00ff00ff00ff00ff00ff00fff00ff00ff00ff00f",
    );
}

#[test]
fn example_512x512_l() {
    let im = ImageProxy(image::open("tests/images/512x512_l.png").unwrap());
    assert_eq!(blockhash16_string(&im), "cccc");
    assert_eq!(blockhash64_string(&im), "39f0f8c8d8f0f0b8");
    assert_eq!(
        blockhash144_string(&im),
        "2f37e26427e0fe0e30ff8be0e00d807f8de0",
    );
    assert_eq!(
        blockhash256_string(&im),
        "033f5fc07f8f19803bd0fb90fd81f0c0fffde7c09f00e0005e00fff07bc0e780",
    );
}

#[test]
fn example_512x512_rgb() {
    let im = ImageProxy(image::open("tests/images/512x512_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "356c");
    assert_eq!(blockhash64_string(&im), "af0575297c4c4ce3");
    assert_eq!(
        blockhash144_string(&im),
        "93fc0d913bd318332b37c37d308328e2ef83",
    );
    assert_eq!(
        blockhash256_string(&im),
        "9cfde03dc4198467ad671d171c071c5b1ff81bf919d9181838f8f890f807ff01",
    );
}

#[test]
fn example_5x2_rgb() {
    let im = ImageProxy(image::open("tests/images/5x2_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "33cc");
    assert_eq!(blockhash64_string(&im), "2727272763636363");
    assert_eq!(
        blockhash144_string(&im),
        "11f11f11f11f11f11f387387387387387387",
    );
    assert_eq!(
        blockhash256_string(&im),
        "0c3f0c3f0c3f0c3f0c3f0c3f0c3f0c3f1c071c071c071c071c071c071c071c07",
    );
}

#[test]
fn example_5x5_rgb() {
    let im = ImageProxy(image::open("tests/images/5x5_rgb.png").unwrap());
    assert_eq!(blockhash16_string(&im), "cccc");
    assert_eq!(blockhash64_string(&im), "d8d8c7e0e0f8e0f8");
    assert_eq!(
        blockhash144_string(&im),
        "e60e60e7cc1fe07f80e00fe0ff0f00fe0fe0",
    );
    assert_eq!(
        blockhash256_string(&im),
        "e380e380e380f3ffe07fe07ffc00fe00f000fe00fff0fff0e000ffc0ffc0ffc0",
    );
}

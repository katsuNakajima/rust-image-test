extern crate image;

use image::imageops::invert;
use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;

fn main() {
    let input = "/Users/nakaji/sample.png";
    let output = "/Users/nakaji/sample-test.png";

    let mut img: DynamicImage = image::open(input).unwrap();
    invert(&mut img);
    img.save(output).unwrap();
}

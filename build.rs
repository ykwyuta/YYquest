use image::{imageops, RgbaImage};
use std::{env, fs, path::PathBuf};

#[path = "build/map_art.rs"]
mod map_art;

#[path = "src/art_sizes.rs"]
mod art_sizes;

fn load(path: &str) -> RgbaImage {
    println!("cargo:rerun-if-changed={path}");
    image::open(path)
        .unwrap_or_else(|error| panic!("{path}: {error}"))
        .into_rgba8()
}

fn export(image: &RgbaImage, name: &str, side: usize, out: &std::path::Path) {
    fs::write(
        out.join(format!("{name}.rgba")),
        fit_sprite(image, name, side).into_raw(),
    )
    .unwrap();
}

fn fit_sprite(image: &RgbaImage, name: &str, side: usize) -> RgbaImage {
    let side = side as u32;
    let (mut left, mut top) = (image.width(), image.height());
    let (mut right, mut bottom) = (0, 0);
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel[3] >= 16 {
            left = left.min(x);
            top = top.min(y);
            right = right.max(x + 1);
            bottom = bottom.max(y + 1);
        }
    }
    assert!(right > left && bottom > top, "{name} has no visible pixels");
    let mut crop = imageops::crop_imm(image, left, top, right - left, bottom - top).to_image();
    for pixel in crop.pixels_mut() {
        let alpha = u32::from(pixel[3]);
        for channel in &mut pixel.0[..3] {
            *channel = ((u32::from(*channel) * alpha + 127) / 255) as u8;
        }
    }
    let longest = crop.width().max(crop.height());
    let width = ((crop.width() * side + longest / 2) / longest).max(1);
    let height = ((crop.height() * side + longest / 2) / longest).max(1);
    let mut sprite = imageops::resize(&crop, width, height, imageops::FilterType::Lanczos3);
    for pixel in sprite.pixels_mut() {
        let alpha = pixel[3];
        for channel in &mut pixel.0[..3] {
            *channel = (*channel).min(alpha);
        }
    }
    let mut cell = RgbaImage::new(side, side);
    imageops::replace(
        &mut cell,
        &sprite,
        i64::from((side - width) / 2),
        i64::from(side - height),
    );
    cell
}

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    map_art::build(&out);
    let heroes = load("assets/prologue-characters.png");
    let half = heroes.width() / 2;
    for (name, offset) in [("allen", 0), ("lilith", half)] {
        let cell = imageops::crop_imm(&heroes, offset, 0, half, heroes.height()).to_image();
        export(&cell, name, art_sizes::PORTRAIT, &out);
    }
    for (name, path) in [
        ("slime", "assets/slime.png"),
        ("bat", "assets/bat.png"),
        ("echo", "assets/echo.png"),
    ] {
        export(&load(path), name, art_sizes::PORTRAIT, &out);
    }
}

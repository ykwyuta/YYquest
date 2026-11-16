use image::{imageops, RgbaImage};
use std::{env, fs, path::PathBuf};

#[path = "build/map_art.rs"]
mod map_art;

const SIZE: u32 = 64;

fn load(path: &str) -> RgbaImage {
    println!("cargo:rerun-if-changed={path}");
    image::open(path)
        .unwrap_or_else(|error| panic!("{path}: {error}"))
        .into_rgba8()
}

fn export(image: &RgbaImage, name: &str, out: &std::path::Path) {
    fs::write(
        out.join(format!("{name}.rgba")),
        fit_sprite(image, name).into_raw(),
    )
    .unwrap();
}

fn fit_sprite(image: &RgbaImage, name: &str) -> RgbaImage {
    let (mut left, mut top) = (image.width(), image.height());
    let (mut right, mut bottom) = (0, 0);
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel[3] >= 128 {
            left = left.min(x);
            top = top.min(y);
            right = right.max(x + 1);
            bottom = bottom.max(y + 1);
        }
    }
    assert!(right > left && bottom > top, "{name} has no visible pixels");
    let crop = imageops::crop_imm(image, left, top, right - left, bottom - top).to_image();
    let longest_edge = crop.width().max(crop.height());
    let width = (crop.width() * SIZE / longest_edge).max(1);
    let height = (crop.height() * SIZE / longest_edge).max(1);
    let sprite = imageops::resize(&crop, width, height, imageops::FilterType::Nearest);
    let mut cell = RgbaImage::new(SIZE, SIZE);
    imageops::replace(
        &mut cell,
        &sprite,
        i64::from((SIZE - width) / 2),
        i64::from(SIZE - height),
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
        export(&cell, name, &out);
    }
    for (name, path) in [
        ("slime", "assets/slime.png"),
        ("bat", "assets/bat.png"),
        ("echo", "assets/echo.png"),
    ] {
        export(&load(path), name, &out);
    }
}

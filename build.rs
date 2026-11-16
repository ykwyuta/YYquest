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
    export(
        &load("assets/guide-spirit.png"),
        "guide-spirit",
        art_sizes::MAP_OBJECT,
        &out,
    );
    let title = load("assets/title-arcadia.png");
    let title = imageops::resize(
        &title,
        art_sizes::TITLE_WIDTH as u32,
        art_sizes::TITLE_HEIGHT as u32,
        imageops::FilterType::Lanczos3,
    );
    let title: Vec<u8> = title
        .pixels()
        .flat_map(|pixel| [pixel[0], pixel[1], pixel[2]])
        .collect();
    fs::write(out.join("title.rgb"), title).unwrap();
    let mut characters = Vec::new();
    // Each sheet holds both characters facing south, west, east and north.
    for path in [
        "assets/prologue-characters.png",
        "assets/prologue-walk.png",
        "assets/prologue-walk-opposite.png",
        "assets/prologue-walk-passing.png",
    ] {
        let heroes = load(path);
        for row in 0..2 {
            for col in 0..4 {
                let cell = map_art::cell(&heroes, 4, 2, col, row);
                let name = format!("{path} character {row}/{col}");
                let sprite = fit_sprite(&cell, &name, art_sizes::PORTRAIT);
                characters.extend(sprite.into_raw());
            }
        }
    }
    fs::write(out.join("prologue-characters.rgba"), characters).unwrap();
    let mut faces = Vec::new();
    for name in [
        "allen",
        "lilith",
        "hearth",
        "lightkeeper",
        "mason",
        "baldo",
        "villager",
        "worker",
        "guard",
        "traveler",
    ] {
        let atlas = load(&format!("assets/portraits/{name}.png"));
        for row in 0..2 {
            for col in 0..5 {
                let portrait = map_art::cell(&atlas, 5, 2, col, row);
                faces.extend(fit_sprite(&portrait, name, art_sizes::FACE).into_raw());
            }
        }
    }
    fs::write(out.join("dialogue-faces.rgba"), faces).unwrap();
    let mut icons = Vec::new();
    // Keep the packed artwork in Item's discriminant order.
    for name in [
        "rusty-sword",
        "ash-herb",
        "bitter-root",
        "bog-iron",
        "ash-wood",
        "beast-hide",
        "echo-shard",
        "poultice",
        "tonic",
        "spark-tube",
        "iron-sword",
        "echo-blade",
        "leather-coat",
        "riveted-coat",
        "mist-charm",
    ] {
        let icon = load(&format!("assets/items/{name}.png"));
        icons.extend(fit_sprite(&icon, name, art_sizes::ITEM).into_raw());
    }
    fs::write(out.join("items.rgba"), icons).unwrap();
    for (name, path) in [
        ("slime", "assets/slime.png"),
        ("bat", "assets/bat.png"),
        ("echo", "assets/echo.png"),
    ] {
        export(&load(path), name, art_sizes::PORTRAIT, &out);
    }
}

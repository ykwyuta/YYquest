use image::{imageops, RgbaImage};
use std::{fs, path::Path};

use super::{art_sizes, export, fit_sprite, load};

const ART_DIR: &str = "assets/maps";
const TERRAIN_SIZE: u32 = art_sizes::TERRAIN as u32;

pub(super) fn build(out: &Path) {
    build_textures(
        &load(&format!("{ART_DIR}/terrain.png")),
        out,
        "map-terrain.rgb",
    );
    build_textures(
        &load(&format!("{ART_DIR}/terrain-variants.png")),
        out,
        "map-terrain-variants.rgb",
    );
    build_roadside(out);
    build_fauna(out);

    build_textures(
        &load(&format!("{ART_DIR}/fine-ground.png")),
        out,
        "map-fine-ground.rgb",
    );
    let sprites = load(&format!("{ART_DIR}/local-landmarks.png"));
    for (index, name) in [(1, "nox-cottage"), (4, "elder-tree"), (6, "memorial")] {
        let sprite = cell(&sprites, 4, 3, index % 4, index / 4);
        export(&sprite, &format!("map-{name}"), art_sizes::MAP_OBJECT, out);
    }

    build_settlements(out);
    let nox_town = load(&format!("{ART_DIR}/nox-town.png"));
    let mut sprites = Vec::new();
    for index in 0..16 {
        let tile = cell(&nox_town, 4, 4, index % 4, index / 4);
        let name = format!("Nox town sprite {index}");
        let tile = isolate_settlement(tile, &name);
        sprites.extend(fit_sprite(&tile, &name, art_sizes::MAP_OBJECT).into_raw());
    }
    fs::write(out.join("nox-town.rgba"), sprites).unwrap();
    for name in ["forest", "mountain", "farm"] {
        export(
            &load(&format!("{ART_DIR}/{name}.png")),
            &format!("map-{name}"),
            art_sizes::MAP_OBJECT,
            out,
        );
    }
}

fn build_textures(textures: &RgbaImage, out: &Path, filename: &str) {
    let mut terrain = Vec::new();
    for row in 0..4 {
        for col in 0..4 {
            let cell = cell(textures, 4, 4, col, row);
            // Sample the center so a generation artifact near an atlas boundary
            // cannot become a repeating edge in the map.
            let size = cell.width().min(cell.height()) * 3 / 4;
            let square = imageops::crop_imm(
                &cell,
                (cell.width() - size) / 2,
                (cell.height() - size) / 2,
                size,
                size,
            )
            .to_image();
            let mut tile = imageops::resize(
                &square,
                TERRAIN_SIZE,
                TERRAIN_SIZE,
                imageops::FilterType::Lanczos3,
            );
            soften_edges(&mut tile);
            terrain.extend(
                tile.pixels()
                    .flat_map(|pixel| [pixel[0], pixel[1], pixel[2]]),
            );
        }
    }
    fs::write(out.join(filename), terrain).unwrap();
}

fn soften_edges(tile: &mut RgbaImage) {
    // Cross-fade the opposing edges. Equal endpoints can then repeat without
    // reflecting wheel ruts or creating symmetrical pairs of waves.
    const BLEND: u32 = 18;
    for vertical in [false, true] {
        for across in 0..TERRAIN_SIZE {
            for edge in 0..BLEND {
                let far = TERRAIN_SIZE - 1 - edge;
                let ((x, y), (nx, ny)) = if vertical {
                    ((across, edge), (across, far))
                } else {
                    ((edge, across), (far, across))
                };
                let a = tile.get_pixel(x, y).0;
                let b = tile.get_pixel(nx, ny).0;
                let weight = BLEND - edge;
                let blend = |first: [u8; 4], second: [u8; 4]| {
                    image::Rgba(std::array::from_fn(|c| {
                        ((u32::from(first[c]) * (2 * BLEND - weight)
                            + u32::from(second[c]) * weight)
                            / (2 * BLEND)) as u8
                    }))
                };
                tile.put_pixel(x, y, blend(a, b));
                tile.put_pixel(nx, ny, blend(b, a));
            }
        }
    }
}

fn build_roadside(out: &Path) {
    let atlas = load(&format!("{ART_DIR}/roadside-objects.png"));
    let names = [
        "dark-fir",
        "gold-oak",
        "green-fir",
        "gnarled-oak",
        "coast-pine",
        "birch",
        "heather-bush",
        "ash-tree",
        "ferns",
        "reeds",
        "wildflowers",
        "dune-grass",
        "train-east",
        "train-south",
        "carriage-east",
        "carriage-south",
    ];
    for (index, name) in names.into_iter().enumerate() {
        let sprite = cell(&atlas, 4, 4, index as u32 % 4, index as u32 / 4);
        export(
            &isolate_settlement(sprite, name),
            &format!("map-{name}"),
            art_sizes::MAP_OBJECT,
            out,
        );
    }
    let atlas = load(&format!("{ART_DIR}/vehicles-north.png"));
    for (index, name) in ["train-north", "carriage-north"].into_iter().enumerate() {
        let sprite = cell(&atlas, 2, 1, index as u32, 0);
        export(
            &isolate_settlement(sprite, name),
            &format!("map-{name}"),
            art_sizes::MAP_OBJECT,
            out,
        );
    }
}

fn build_fauna(out: &Path) {
    let original = load(&format!("{ART_DIR}/fauna.png"));
    for (region_index, region) in ["nox", "celestia", "sylva", "liberta", "vulcan"]
        .into_iter()
        .enumerate()
    {
        let additional = load(&format!("{ART_DIR}/fauna/{region}.png"));
        let mut sprites = Vec::new();
        for index in 0..15u32 {
            let (atlas, rows, cell_index) = match index {
                0..=1 => (&original, 4, region_index as u32 * 2 + index),
                10 => (&original, 4, 10 + region_index as u32),
                2..=9 => (&additional, 3, index - 2),
                _ => (&additional, 3, index - 3),
            };
            let sprite = cell(atlas, 4, rows, cell_index % 4, cell_index / 4);
            let name = format!("{region} fauna {}", index + 1);
            let clean = isolate_settlement(sprite, &name);
            let side = if index < 10 {
                art_sizes::ANIMAL
            } else {
                art_sizes::BEAST
            };
            sprites.extend(fit_sprite(&clean, &name, side).into_raw());
        }
        fs::write(out.join(format!("fauna-{region}.rgba")), sprites).unwrap();
    }
}

fn build_settlements(out: &Path) {
    for region in ["nox", "celestia", "sylva", "liberta", "vulcan"] {
        let atlas = load(&format!("{ART_DIR}/settlements/{region}.png"));
        assert_eq!(
            atlas.width(),
            atlas.height(),
            "{region}: expected a square atlas"
        );
        let mut sprites = Vec::new();
        for index in 0..13 {
            let sprite = cell(&atlas, 4, 4, index % 4, index / 4);
            let name = format!("{region} settlement {}", index + 1);
            let clean = isolate_settlement(sprite, &name);
            sprites.extend(fit_sprite(&clean, &name, art_sizes::SETTLEMENT).into_raw());
        }
        fs::write(out.join(format!("settlements-{region}.rgba")), sprites).unwrap();
    }
}

// The object in each cell is a connected silhouette. Keep its largest opaque
// component so stray atlas-edge pixels cannot be visible on a different site.
fn isolate_settlement(mut image: RgbaImage, name: &str) -> RgbaImage {
    let (width, height) = image.dimensions();
    let area = (width * height) as usize;
    let mut pending: Vec<_> = image.pixels().map(|pixel| pixel[3] >= 192).collect();
    let mut largest = Vec::new();
    let mut queue = Vec::new();
    for index in 0..area {
        if !pending[index] {
            continue;
        }
        pending[index] = false;
        queue.clear();
        queue.push(index);
        let mut cursor = 0;
        while let Some(&pixel) = queue.get(cursor) {
            cursor += 1;
            let x = (pixel as u32) % width;
            let y = (pixel as u32) / width;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if !(0..width as i32).contains(&nx) || !(0..height as i32).contains(&ny) {
                    continue;
                }
                let next = ny as usize * width as usize + nx as usize;
                if pending[next] {
                    pending[next] = false;
                    queue.push(next);
                }
            }
        }
        if queue.len() > largest.len() {
            std::mem::swap(&mut largest, &mut queue);
        }
    }
    assert!(
        largest.len() > area / 8,
        "{name}: missing or incomplete sprite"
    );
    let mut keep = vec![false; area];
    for index in largest {
        keep[index] = true;
    }
    // Keep the original alpha values around the main silhouette, including
    // antialiased edges, while rejecting fragments from adjacent atlas cells.
    for _ in 0..3 {
        let previous = keep.clone();
        for (index, pixel) in image.pixels().enumerate() {
            if keep[index] || pixel[3] == 0 {
                continue;
            }
            let (x, y) = (index as i32 % width as i32, index as i32 / width as i32);
            keep[index] = (-1..=1).any(|dy| {
                (-1..=1).any(|dx| {
                    let (nx, ny) = (x + dx, y + dy);
                    (0..width as i32).contains(&nx)
                        && (0..height as i32).contains(&ny)
                        && previous[ny as usize * width as usize + nx as usize]
                })
            });
        }
    }
    for (pixel, keep) in image.pixels_mut().zip(keep) {
        if !keep {
            *pixel = image::Rgba([0, 0, 0, 0]);
        }
    }
    image
}

pub(super) fn cell(atlas: &RgbaImage, cols: u32, rows: u32, col: u32, row: u32) -> RgbaImage {
    let left = atlas.width() * col / cols;
    let right = atlas.width() * (col + 1) / cols;
    let top = atlas.height() * row / rows;
    let bottom = atlas.height() * (row + 1) / rows;
    imageops::crop_imm(atlas, left, top, right - left, bottom - top).to_image()
}

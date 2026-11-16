use super::{Frame, Sprite, DISPLAY_SCALE};
use crate::art_sizes;
use crate::noise::sample as natural_noise;
use std::sync::OnceLock;

fn decode_lz(src: &[u8]) -> Vec<u8> {
    let total = u32::from_le_bytes(src[..4].try_into().unwrap()) as usize;
    let mut dst = Vec::with_capacity(total);
    let mut i = 4usize;
    while i < src.len() {
        let tag = src[i];
        i += 1;
        if tag < 0x80 {
            let len = usize::from(tag) + 1;
            dst.extend_from_slice(&src[i..i + len]);
            i += len;
        } else {
            let len = usize::from(tag & 0x7f) + 3;
            let off = usize::from(u16::from_le_bytes([src[i], src[i + 1]]));
            i += 2;
            let start = dst.len() - off;
            for k in 0..len {
                let b = dst[start + k];
                dst.push(b);
            }
        }
    }
    dst
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub(super) enum Ground {
    Grass,
    Woodland,
    Coastal,
    Heath,
    Sea,
    River,
    Sand,
    Marsh,
    Trail,
    Stone,
    Dock,
    Lava,
    Crops,
    Rubble,
    Chalk,
    Obsidian,
    DryMeadow,
    LeafMoss,
    Heather,
    Dune,
    Rutted,
    Gravel,
    MudRoad,
    Cinder,
    ShellSand,
    WetSand,
    Shingle,
    SlateShore,
    MossCobble,
    Flower,
    DeepLitter,
    DirtVerge,
}

impl Ground {
    pub(super) fn water(self) -> bool {
        matches!(self, Self::Sea | Self::River)
    }
}

const TEXTURES: &[u8; 16 * art_sizes::TERRAIN * art_sizes::TERRAIN * 3] =
    include_bytes!(concat!(env!("OUT_DIR"), "/map-terrain.rgb"));
const VARIED_TEXTURES: &[u8; 16 * art_sizes::TERRAIN * art_sizes::TERRAIN * 3] =
    include_bytes!(concat!(env!("OUT_DIR"), "/map-terrain-variants.rgb"));

macro_rules! sprite_assets {
    ($($asset:ident => $name:literal),* $(,)?) => {
        $(pub(super) const $asset: Sprite = Sprite::new(include_bytes!(concat!(env!("OUT_DIR"), "/map-", $name, ".rgba")), art_sizes::MAP_OBJECT);)*
    };
}

sprite_assets! {
    FOREST => "forest",
    MOUNTAIN => "mountain",
    FARM => "farm",
    COTTAGE => "nox-cottage",
    ELDER_TREE => "elder-tree",
    MEMORIAL => "memorial",
    DARK_FIR => "dark-fir",
    GOLD_OAK => "gold-oak",
    GREEN_FIR => "green-fir",
    GNARLED_OAK => "gnarled-oak",
    COAST_PINE => "coast-pine",
    BIRCH => "birch",
    HEATHER_BUSH => "heather-bush",
    ASH_TREE => "ash-tree",
    FERNS => "ferns",
    REEDS => "reeds",
    WILDFLOWERS => "wildflowers",
    DUNE_GRASS => "dune-grass",
    TRAIN_EAST => "train-east",
    TRAIN_SOUTH => "train-south",
    CARRIAGE_EAST => "carriage-east",
    CARRIAGE_SOUTH => "carriage-south",
    TRAIN_NORTH => "train-north",
    CARRIAGE_NORTH => "carriage-north",
}

pub(super) fn town_sprite(region: crate::world_map::Region, index: usize) -> Sprite {
    use crate::world_map::Region;

    const SIZE: usize = art_sizes::MAP_OBJECT;
    const STRIDE: usize = SIZE * SIZE * 4;
    static NOX: OnceLock<Vec<u8>> = OnceLock::new();
    static CELESTIA: OnceLock<Vec<u8>> = OnceLock::new();
    static SYLVA: OnceLock<Vec<u8>> = OnceLock::new();
    static LIBERTA: OnceLock<Vec<u8>> = OnceLock::new();
    static VULCAN: OnceLock<Vec<u8>> = OnceLock::new();
    let atlas: &'static [u8] = match region {
        Region::Nox => NOX.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-nox.rgba.lz")))),
        Region::Celestia => CELESTIA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-celestia.rgba.lz")))),
        Region::Sylva => SYLVA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-sylva.rgba.lz")))),
        Region::Liberta => LIBERTA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-liberta.rgba.lz")))),
        Region::Vulcan => VULCAN.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-vulcan.rgba.lz")))),
    };
    Sprite::new(&atlas[index * STRIDE..(index + 1) * STRIDE], SIZE)
}

pub(super) fn town_landmark(region: crate::world_map::Region, index: usize) -> Sprite {
    use crate::world_map::Region;

    const SIZE: usize = art_sizes::MAP_OBJECT;
    const STRIDE: usize = SIZE * SIZE * 4;
    static CELESTIA: OnceLock<Vec<u8>> = OnceLock::new();
    static SYLVA: OnceLock<Vec<u8>> = OnceLock::new();
    static LIBERTA: OnceLock<Vec<u8>> = OnceLock::new();
    static VULCAN: OnceLock<Vec<u8>> = OnceLock::new();
    let atlas: &'static [u8] = match region {
        Region::Nox => return town_sprite(Region::Nox, 0),
        Region::Celestia => CELESTIA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-landmarks-celestia.rgba.lz")))),
        Region::Sylva => SYLVA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-landmarks-sylva.rgba.lz")))),
        Region::Liberta => LIBERTA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-landmarks-liberta.rgba.lz")))),
        Region::Vulcan => VULCAN.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/town-landmarks-vulcan.rgba.lz")))),
    };
    Sprite::new(&atlas[index * STRIDE..(index + 1) * STRIDE], SIZE)
}

pub(super) fn fauna_sprite(species: crate::island::fauna::Species) -> Sprite {
    use crate::{island::fauna::ANIMAL_SPECIES, world_map::Region};

    const ANIMAL: usize = art_sizes::ANIMAL * art_sizes::ANIMAL * 4;
    const BEAST: usize = art_sizes::BEAST * art_sizes::BEAST * 4;
    let atlas: &[u8] = match species.region {
        Region::Nox => include_bytes!(concat!(env!("OUT_DIR"), "/fauna-nox.rgba")),
        Region::Celestia => include_bytes!(concat!(env!("OUT_DIR"), "/fauna-celestia.rgba")),
        Region::Sylva => include_bytes!(concat!(env!("OUT_DIR"), "/fauna-sylva.rgba")),
        Region::Liberta => include_bytes!(concat!(env!("OUT_DIR"), "/fauna-liberta.rgba")),
        Region::Vulcan => include_bytes!(concat!(env!("OUT_DIR"), "/fauna-vulcan.rgba")),
    };
    let (start, stride, side) = if species.is_beast() {
        (
            ANIMAL_SPECIES * ANIMAL + (species.index - ANIMAL_SPECIES) * BEAST,
            BEAST,
            art_sizes::BEAST,
        )
    } else {
        (species.index * ANIMAL, ANIMAL, art_sizes::ANIMAL)
    };
    Sprite::new(&atlas[start..start + stride], side)
}

pub(super) fn settlement(region: crate::world_map::Region, index: usize) -> Sprite {
    use crate::world_map::Region;

    const STRIDE: usize = art_sizes::SETTLEMENT * art_sizes::SETTLEMENT * 4;
    static NOX: OnceLock<Vec<u8>> = OnceLock::new();
    static CELESTIA: OnceLock<Vec<u8>> = OnceLock::new();
    static SYLVA: OnceLock<Vec<u8>> = OnceLock::new();
    static LIBERTA: OnceLock<Vec<u8>> = OnceLock::new();
    static VULCAN: OnceLock<Vec<u8>> = OnceLock::new();
    let atlas: &'static [u8] = match region {
        Region::Nox => NOX.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/settlements-nox.rgba.lz")))),
        Region::Celestia => CELESTIA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/settlements-celestia.rgba.lz")))),
        Region::Sylva => SYLVA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/settlements-sylva.rgba.lz")))),
        Region::Liberta => LIBERTA.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/settlements-liberta.rgba.lz")))),
        Region::Vulcan => VULCAN.get_or_init(|| decode_lz(include_bytes!(concat!(env!("OUT_DIR"), "/settlements-vulcan.rgba.lz")))),
    };
    Sprite::new(
        &atlas[index * STRIDE..(index + 1) * STRIDE],
        art_sizes::SETTLEMENT,
    )
}

#[cfg(test)]
mod tests {
    use super::settlement;
    use crate::{island::SETTLEMENTS, world_map::Region};
    use std::collections::HashSet;

    #[test]
    fn every_species_has_a_visible_unique_sprite() {
        use crate::island::fauna::{Species, SPECIES_PER_ISLAND};

        let mut sprites = HashSet::new();
        for region in Region::ALL {
            for index in 0..SPECIES_PER_ISLAND {
                let sprite = super::fauna_sprite(Species::new(region, index)).pixels;
                assert!(sprites.insert(sprite), "{region:?} {index}");
                let visible = sprite
                    .chunks_exact(4)
                    .filter(|pixel| pixel[3] >= 128)
                    .count();
                let area = sprite.len() / 4;
                assert!(
                    visible > area / 10 && visible < area * 9 / 10,
                    "{region:?} {index}: {visible}"
                );
            }
        }
        assert_eq!(sprites.len(), 75);
    }

    #[test]
    fn all_settlement_sprites_are_unique_and_visible() {
        let mut sprites = HashSet::new();
        for region in Region::ALL {
            for index in 0..SETTLEMENTS {
                let sprite = settlement(region, index).pixels;
                assert!(sprites.insert(sprite), "{region:?} sprite {index}");
                let visible = sprite
                    .chunks_exact(4)
                    .filter(|pixel| pixel[3] >= 128)
                    .count();
                assert!(visible > 500, "{region:?} sprite {index}: {visible}");
            }
        }
        assert_eq!(sprites.len(), 65);
    }

    #[test]
    fn all_town_atlases_and_landmarks_are_unique_and_visible() {
        let mut regional = HashSet::new();
        for region in Region::ALL {
            for index in 0..16 {
                let sprite = super::town_sprite(region, index).pixels;
                assert!(regional.insert(sprite), "{region:?} town sprite {index}");
                let visible = sprite
                    .chunks_exact(4)
                    .filter(|pixel| pixel[3] >= 128)
                    .count();
                assert!(visible > 200, "{region:?} town sprite {index}: {visible}");
            }
        }
        assert_eq!(regional.len(), 80);

        let mut landmarks = HashSet::new();
        for region in [
            Region::Celestia,
            Region::Sylva,
            Region::Liberta,
            Region::Vulcan,
        ] {
            for index in 0..crate::town::COUNT {
                let sprite = super::town_landmark(region, index).pixels;
                assert!(landmarks.insert(sprite), "{region:?} landmark {index}");
                let visible = sprite
                    .chunks_exact(4)
                    .filter(|pixel| pixel[3] >= 128)
                    .count();
                assert!(visible > 500, "{region:?} landmark {index}: {visible}");
            }
        }
        assert_eq!(landmarks.len(), 40);
    }
}

impl Frame {
    pub(super) fn terrain(
        &mut self,
        screen: (i32, i32),
        tile: (i32, i32),
        materials: [Ground; 9],
        size: i32,
        clip: [i32; 4],
    ) {
        let (sx, sy) = (
            screen.0 * DISPLAY_SCALE as i32,
            screen.1 * DISPLAY_SCALE as i32,
        );
        let size = size * DISPLAY_SCALE as i32;
        let [left, top, right, bottom] = clip.map(|edge| edge * DISPLAY_SCALE as i32);
        let (map_x, map_y) = (tile.0 * size, tile.1 * size);
        let stride = self.buffer_width();
        let center = materials[4];
        let uniform = materials.iter().all(|&ground| ground == center);
        for py in (top - sy).max(0)..(bottom - sy).min(size) {
            for px in (left - sx).max(0)..(right - sx).min(size) {
                let (x, y) = (map_x + px, map_y + py);
                let index = (sy + py) as usize * stride + (sx + px) as usize;
                self.pixels[index] = if uniform || center == Ground::Dock {
                    texture(center, x, y)
                } else {
                    let gx = if px < size / 2 { 3 } else { 5 };
                    let gy = if py < size / 2 { 1 } else { 7 };
                    let h = (((px * 2 + 1) * 16 / size) - 16).abs();
                    let v = (((py * 2 + 1) * 16 / size) - 16).abs();
                    let corners = [
                        (materials[4], (32 - h) * (32 - v)),
                        (materials[gx], h * (32 - v)),
                        (materials[gy], (32 - h) * v),
                        (materials[gx + gy - 4], h * v),
                    ];
                    transition(center, corners, x, y)
                };
            }
        }
    }
}

fn texture(ground: Ground, x: i32, y: i32) -> u32 {
    const FINE: &[u8; 16 * art_sizes::TERRAIN * art_sizes::TERRAIN * 3] =
        include_bytes!(concat!(env!("OUT_DIR"), "/map-fine-ground.rgb"));
    let micro = match ground {
        Ground::Stone => Some(0),
        Ground::MossCobble => Some(1),
        Ground::Gravel | Ground::Chalk => Some(2),
        Ground::Obsidian => Some(3),
        Ground::Rubble => Some(4),
        Ground::SlateShore => Some(5),
        Ground::Shingle | Ground::WetSand => Some(6),
        Ground::Sand | Ground::ShellSand => Some(7),
        Ground::Coastal | Ground::DryMeadow => Some(8),
        Ground::LeafMoss => Some(9),
        Ground::DeepLitter | Ground::Woodland => Some(10),
        Ground::Heather | Ground::Heath => Some(11),
        Ground::Rutted => Some(12),
        Ground::MudRoad => Some(13),
        Ground::Cinder => Some(14),
        Ground::DirtVerge => Some(15),
        _ => None,
    };
    let material = ground as usize;
    let (texels, cell) = match micro {
        Some(cell) => (FINE, cell),
        None if material < 16 => (TEXTURES, material),
        None => (VARIED_TEXTURES, material - 16),
    };
    let frequency = if matches!(ground, Ground::Crops | Ground::Dock | Ground::Flower) {
        2
    } else {
        1
    };
    let side = art_sizes::TERRAIN;
    let row = (y * frequency).rem_euclid(side as i32) as usize;
    let col = (x * frequency).rem_euclid(side as i32) as usize;
    let offset = (cell * side * side + row * side + col) * 3;
    let [r, g, b] = [texels[offset], texels[offset + 1], texels[offset + 2]];
    u32::from(r) << 16 | u32::from(g) << 8 | u32::from(b)
}

fn mix(a: u32, b: u32, amount: i32) -> u32 {
    let w = amount.clamp(0, 256) as u32;
    let channel =
        |shift: u32| (((a >> shift & 255) * (256 - w) + (b >> shift & 255) * w) >> 8) << shift;
    channel(16) | channel(8) | channel(0)
}

fn transition(center: Ground, colors: [(Ground, i32); 4], x: i32, y: i32) -> u32 {
    let water: i32 = colors
        .iter()
        .filter(|(ground, _)| ground.water())
        .map(|(_, weight)| weight)
        .sum();
    if water > 0 && water < 1024 {
        let wet = colors
            .iter()
            .filter(|(ground, _)| ground.water())
            .max_by_key(|(_, weight)| weight)
            .unwrap()
            .0;
        let dry = colors
            .iter()
            .filter(|(ground, _)| !ground.water())
            .max_by_key(|(_, weight)| weight)
            .unwrap()
            .0;
        return shore(water, dry, wet, x, y);
    }
    let mut result = texture(center, x, y);
    let grain = (natural_noise(x, y, 7 * DISPLAY_SCALE as i32) - 128) / 8;
    for (other, weight) in colors.into_iter().skip(1) {
        if weight == 0 || other == center {
            continue;
        }
        let amount = ((weight - 100) * 3 / 4 + grain).clamp(0, 224);
        result = mix(result, texture(other, x, y), amount);
    }
    result
}

fn shore(water: i32, land: Ground, sea: Ground, x: i32, y: i32) -> u32 {
    let ripple = natural_noise(x, y, 11 * DISPLAY_SCALE as i32) - 128;
    let water = water + ripple;
    let bank = match land {
        Ground::Obsidian | Ground::Cinder | Ground::Lava => Ground::SlateShore,
        Ground::Chalk | Ground::Gravel | Ground::Heath | Ground::Heather => Ground::Shingle,
        _ if sea == Ground::River => Ground::WetSand,
        _ => Ground::ShellSand,
    };
    let soil = texture(land, x, y);
    if water < 245 {
        return soil;
    }
    let dry = texture(bank, x, y);
    if water < 460 {
        return mix(soil, dry, (water - 245) * 256 / 215);
    }
    let dark = match bank {
        Ground::SlateShore | Ground::Shingle => mix(dry, 0x293b49, 82),
        _ => texture(Ground::WetSand, x, y),
    };
    let wet = mix(dry, dark, (water - 460) * 3);
    let ocean = texture(sea, x, y);
    let shallows = mix(
        ocean,
        if sea == Ground::Sea {
            0x428f8b
        } else {
            0x548f81
        },
        100,
    );
    if water < 620 {
        let base = mix(wet, shallows, (water - 508) * 3);
        let crest = (water - 561).abs();
        if crest < 29 && natural_noise(x, y, 5 * DISPLAY_SCALE as i32) > 64 {
            mix(base, 0xa3c5ad, (29 - crest) * 5)
        } else {
            base
        }
    } else {
        mix(shallows, ocean, (water - 620) * 2)
    }
}

pub(super) fn around(x: i32, y: i32, mut material: impl FnMut(i32, i32) -> Ground) -> [Ground; 9] {
    std::array::from_fn(|index| material(x + index as i32 % 3 - 1, y + index as i32 / 3 - 1))
}

pub(super) fn patch(x: i32, y: i32) -> i32 {
    natural_noise(x, y, 9)
}

pub(super) fn tree(region: crate::world_map::Region, variation: usize) -> Sprite {
    use crate::world_map::Region;
    let palette = match region {
        Region::Celestia => [BIRCH, GOLD_OAK, GREEN_FIR, GNARLED_OAK, FOREST],
        Region::Sylva => [DARK_FIR, GNARLED_OAK, GREEN_FIR, FOREST, BIRCH],
        Region::Liberta => [COAST_PINE, COAST_PINE, GOLD_OAK, GNARLED_OAK, BIRCH],
        Region::Nox => [DARK_FIR, BIRCH, HEATHER_BUSH, GREEN_FIR, FOREST],
        Region::Vulcan => [ASH_TREE, ASH_TREE, HEATHER_BUSH, COAST_PINE, DARK_FIR],
    };
    palette[variation % palette.len()]
}

fn prologue_ground(x: i32, y: i32) -> Ground {
    let tile = crate::game::MAP
        .get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied();
    match tile.unwrap_or(b'~') {
        b'~' => Ground::Sea,
        b'#' => Ground::Woodland,
        b'.' => Ground::Trail,
        b'H' => Ground::Stone,
        b'R' => Ground::Rubble,
        _ => Ground::Heath,
    }
}

impl Frame {
    pub(super) fn draw_prologue_landscape(&mut self, view: super::view::View) {
        use super::view::{ROWS, TILE_SIZE};
        use crate::game::MAP;

        self.rect(
            view.left - 1,
            view.clip[1] - 1,
            view.width() + 2,
            ROWS * TILE_SIZE + 2,
            super::LINE,
        );
        for (x, y) in view.visible_tiles() {
            self.terrain(
                view.point((x, y)),
                (x, y),
                around(x, y, prologue_ground),
                TILE_SIZE,
                view.clip,
            );
        }

        for (row, tiles) in MAP.iter().enumerate() {
            for (col, &tile) in tiles.iter().enumerate() {
                let (x, y) = (col as i32, row as i32);
                let (sprite, size) = match tile {
                    b'#' => {
                        let variation = (x * 7 + y * 3).rem_euclid(13) as usize;
                        (
                            tree(crate::world_map::Region::Nox, variation),
                            25 + variation % 8,
                        )
                    }
                    b'H' => (COTTAGE, 40),
                    b'R' => (MEMORIAL, 39),
                    b'S' => (
                        town_sprite(
                            crate::world_map::Region::Nox,
                            crate::town::Art::Sluice as usize,
                        ),
                        36,
                    ),
                    _ => continue,
                };
                let (px, py) = view.feet((x, y));
                self.art_clipped(
                    sprite,
                    px - size as i32 / 2,
                    py - size as i32,
                    size,
                    view.clip,
                );
            }
        }
    }
}

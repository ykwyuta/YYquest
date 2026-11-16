use super::{
    map_art::{self as art, Ground},
    Frame, ALLEN, GOLD, INK, IVORY, LINE, MINT, MIST,
};
use crate::{
    island::{Island, Kind, Tile, HEIGHT, WIDTH},
    world_map::Region,
};

pub(super) const LEFT: i32 = 16;
pub(super) const TOP: i32 = 22;
pub(super) const TILE_SIZE: i32 = 12;
pub(super) const COLUMNS: i32 = 24;
pub(super) const ROWS: i32 = 14;
pub(super) const VIEW: [i32; 4] = [
    LEFT,
    TOP,
    LEFT + COLUMNS * TILE_SIZE,
    TOP + ROWS * TILE_SIZE,
];

impl Frame {
    pub(super) fn draw_island(&mut self, island: &Island) {
        let (title, _) = island.region.caption();
        self.label(16, 6, 12, GOLD, title);
        self.label(203, 8, 9, MIST, "M / Esc  全体図");
        self.rect(
            LEFT - 1,
            TOP - 1,
            COLUMNS * TILE_SIZE + 2,
            ROWS * TILE_SIZE + 2,
            LINE,
        );

        let camera_x = (island.x - COLUMNS / 2).clamp(0, WIDTH - COLUMNS);
        let camera_y = (island.y - ROWS / 2).clamp(0, HEIGHT - ROWS);
        for screen_y in 0..ROWS {
            for screen_x in 0..COLUMNS {
                let map_x = camera_x + screen_x;
                let map_y = camera_y + screen_y;
                self.terrain(
                    (LEFT + screen_x * TILE_SIZE, TOP + screen_y * TILE_SIZE),
                    (map_x, map_y),
                    art::around(map_x, map_y, |x, y| Self::ground(island, x, y)),
                    TILE_SIZE,
                );
                if island.tile(map_x, map_y) == Tile::Cliff
                    && island.tile(map_x, map_y + 1) != Tile::Cliff
                {
                    let screen_x = LEFT + screen_x * TILE_SIZE;
                    let foot = TOP + (screen_y + 1) * TILE_SIZE;
                    let rock = if island.region == Region::Celestia {
                        0x777568
                    } else {
                        0x4a4244
                    };
                    let shade = if island.region == Region::Celestia {
                        0x56564d
                    } else {
                        0x36313a
                    };
                    for offset in 0..TILE_SIZE {
                        let height = 1 + i32::from(
                            crate::noise::hash(map_x * TILE_SIZE + offset, map_y) & 7 < 3,
                        );
                        self.rect(screen_x + offset, foot - height - 1, 1, 1, rock);
                        self.rect(screen_x + offset, foot - height, 1, height, shade);
                    }
                }
                self.draw_rail_tile(
                    LEFT + screen_x * TILE_SIZE,
                    TOP + screen_y * TILE_SIZE,
                    island.traffic.tracks_at(map_x, map_y),
                );
            }
        }
        self.draw_island_objects(island, camera_x, camera_y);
        self.draw_settlements(island, camera_x, camera_y);
        self.draw_traffic(island, (camera_x, camera_y), VIEW);
        self.draw_fauna(island, camera_x, camera_y);
        let px = LEFT + (island.x - camera_x) * TILE_SIZE;
        let py = TOP + (island.y - camera_y) * TILE_SIZE;
        self.rect(px + 2, py + 10, 9, 2, INK);
        self.art_clipped(ALLEN, px - 1, py - 2, 14, VIEW);

        self.panel(15, 191, 290, 46);
        let site = island.current_landmark();
        if let Some(site) = site {
            self.label(24, 196, 11, GOLD, site.name);
            self.label(24, 210, 10, IVORY, site.detail);
            match site.kind {
                Kind::Harbor => self.label(253, 197, 9, MINT, "港町"),
                Kind::Town => self.label(253, 197, 9, GOLD, "街・村"),
                Kind::Ruin => self.label(253, 197, 9, MIST, "廃墟"),
                _ => {}
            }
        } else if let Some((_, enemy)) = island.fauna.nearby_beast((island.x, island.y)) {
            self.label(24, 196, 11, GOLD, enemy.name());
            self.label(24, 210, 10, IVORY, "Enter / Z  戦う");
        } else if let Some(animal) = island.fauna.nearby_animal((island.x, island.y)) {
            self.label(24, 196, 11, MINT, animal.name());
            self.label(24, 210, 10, IVORY, "近くに野生動物がいる。");
        } else {
            let ground = match island.tile(island.x, island.y) {
                Tile::Bridge => "橋・桟橋",
                Tile::Road => "街道",
                Tile::Field => "畑",
                Tile::Grove => "林",
                Tile::Mire => "湿地",
                Tile::Ash => "荒野",
                Tile::Ruin => "廃墟",
                Tile::Beach => "浜辺",
                Tile::Plaza => "広場",
                _ => "フィールド",
            };
            let ground = if island.traffic.tracks_at(island.x, island.y) != 0 {
                "魔導鉄道"
            } else {
                ground
            };
            self.label(24, 196, 11, GOLD, ground);
            self.label(
                24,
                210,
                10,
                IVORY,
                "街や廃墟の手前にある目印まで歩いてみよう。",
            );
        }
        self.label(24, 224, 9, MIST, "矢印 / WASD  移動");
        if island.region == Region::Nox
            && site.is_some_and(|site| matches!(site.kind, Kind::Town | Kind::Harbor))
        {
            self.label(126, 224, 9, MINT, "Enter / Z  入る");
        } else if site.is_some_and(|site| site.kind == Kind::Harbor) {
            self.label(126, 224, 9, MINT, "Enter / Z  出港");
        }
        self.label(
            237,
            224,
            9,
            MINT,
            format!("{:02} / {:02}", island.x + 1, island.y + 1),
        );
    }

    fn ground(island: &Island, x: i32, y: i32) -> Ground {
        let patch = art::patch(x, y);
        let dry = patch > 150;
        let base = match island.region {
            Region::Celestia if dry => Ground::DryMeadow,
            Region::Celestia => Ground::Grass,
            Region::Sylva if dry => Ground::LeafMoss,
            Region::Sylva => Ground::Woodland,
            Region::Liberta if dry => Ground::Dune,
            Region::Liberta => Ground::Coastal,
            Region::Nox if dry => Ground::Heather,
            Region::Nox => Ground::Heath,
            Region::Vulcan => Ground::Obsidian,
        };
        match island.tile(x, y) {
            Tile::Sea => Ground::Sea,
            Tile::Water => Ground::River,
            Tile::Bridge => Ground::Dock,
            Tile::Beach if dry => Ground::ShellSand,
            Tile::Beach => Ground::Sand,
            Tile::Cliff if island.region == Region::Celestia => Ground::Chalk,
            Tile::Cliff | Tile::Ash => Ground::Obsidian,
            Tile::Forest | Tile::Grove if patch > 172 => Ground::DeepLitter,
            Tile::Forest | Tile::Grove if patch < 100 => Ground::LeafMoss,
            Tile::Forest | Tile::Grove => Ground::Woodland,
            Tile::Lava => Ground::Lava,
            Tile::Mire => Ground::Marsh,
            Tile::Field => Ground::Crops,
            Tile::Road => match island.region {
                Region::Celestia if dry => Ground::Gravel,
                Region::Celestia => Ground::MossCobble,
                Region::Vulcan if dry => Ground::Gravel,
                Region::Vulcan => Ground::Cinder,
                Region::Nox if dry => Ground::Rutted,
                Region::Nox => Ground::MudRoad,
                Region::Sylva if dry => Ground::DirtVerge,
                Region::Liberta if dry => Ground::Gravel,
                _ if patch > 100 => Ground::Rutted,
                _ => Ground::Trail,
            },
            Tile::Plaza => {
                if patch < 98 {
                    Ground::MossCobble
                } else {
                    Ground::Stone
                }
            }
            Tile::Ruin => Ground::Rubble,
            Tile::Settlement(index) | Tile::Building(index) => {
                match island.settlements()[index].landmark.kind {
                    Kind::Harbor if island.region == Region::Liberta => Ground::ShellSand,
                    Kind::Harbor if island.region == Region::Vulcan => Ground::Obsidian,
                    Kind::Harbor => base,
                    Kind::Ruin => Ground::Rubble,
                    _ => Ground::Stone,
                }
            }
            Tile::Meadow if patch > 190 && island.region != Region::Vulcan => Ground::Flower,
            Tile::Landmark(_) | Tile::Meadow => base,
        }
    }

    fn draw_island_objects(&mut self, island: &Island, camera_x: i32, camera_y: i32) {
        for sy in -2..ROWS + 3 {
            for sx in -2..COLUMNS + 2 {
                let x = camera_x + sx;
                let y = camera_y + sy;
                let variation = crate::noise::hash(x, y);
                // Each 2×2 group has an independently chosen anchor and ground
                // offset. Understory fills the spaces between forest crowns.
                let cluster = crate::noise::hash(x.div_euclid(2), y.div_euclid(2));
                let anchor = x.rem_euclid(2) == (cluster & 1) as i32
                    && y.rem_euclid(2) == ((cluster >> 1) & 1) as i32;
                let tile = island.tile(x, y);
                let tree = || art::tree(island.region, (variation >> 5) as usize);
                let edge = |needle: Tile| {
                    [(0, -1), (-1, 0), (1, 0), (0, 1)]
                        .into_iter()
                        .any(|(dx, dy)| island.tile(x + dx, y + dy) == needle)
                };
                let sprite = match tile {
                    Tile::Landmark(index) => {
                        if island.landmark(index).kind == Kind::AncientTree {
                            Some((art::ELDER_TREE, 40))
                        } else {
                            Some((art::MEMORIAL, 27))
                        }
                    }
                    Tile::Forest if anchor => Some((tree(), 16 + (variation % 11) as usize)),
                    Tile::Forest if variation % 7 == 0 => Some((art::FERNS, 9)),
                    Tile::Grove if anchor && variation % 4 != 0 => {
                        Some((tree(), 12 + (variation % 7) as usize))
                    }
                    Tile::Mire if variation % 9 == 4 => Some((art::REEDS, 10)),
                    Tile::Beach if variation % 13 == 2 => Some((art::DUNE_GRASS, 10)),
                    Tile::Meadow | Tile::Grove if variation % 8 == 2 && edge(Tile::Road) => Some((
                        if island.region == Region::Sylva {
                            art::FERNS
                        } else {
                            art::WILDFLOWERS
                        },
                        10,
                    )),
                    Tile::Cliff if anchor && variation % 5 < 3 => {
                        let rim = island.tile(x, y + 1) != Tile::Cliff;
                        Some((
                            art::MOUNTAIN,
                            if rim {
                                14 + (variation % 7) as usize
                            } else {
                                11 + (variation % 6) as usize
                            },
                        ))
                    }
                    Tile::Field if anchor && variation % 19 == 5 => Some((art::FARM, 18)),
                    _ => None,
                };
                if let Some((sprite, size)) = sprite {
                    let jitter_x = (variation & 3) as i32 - 1;
                    let jitter_y = ((variation >> 2) & 3) as i32 - 1;
                    self.art_clipped(
                        sprite,
                        LEFT + sx * TILE_SIZE + TILE_SIZE / 2 - size as i32 / 2 + jitter_x,
                        TOP + sy * TILE_SIZE + TILE_SIZE - size as i32 + jitter_y,
                        size,
                        VIEW,
                    );
                }
            }
        }
        for sy in 0..ROWS {
            for sx in 0..COLUMNS {
                if matches!(island.tile(camera_x + sx, camera_y + sy), Tile::Landmark(_)) {
                    let px = LEFT + sx * TILE_SIZE + TILE_SIZE / 2;
                    let py = TOP + sy * TILE_SIZE + TILE_SIZE - 4;
                    self.site_marker(px, py, GOLD);
                }
            }
        }
    }

    fn draw_settlements(&mut self, island: &Island, camera_x: i32, camera_y: i32) {
        const CLIP: [i32; 4] = [
            LEFT,
            TOP,
            LEFT + COLUMNS * TILE_SIZE,
            TOP + ROWS * TILE_SIZE,
        ];

        for (index, site) in island.settlements().iter().enumerate() {
            let size = site.sprite_size();
            let door_x = LEFT + (site.door.0 - camera_x) * TILE_SIZE;
            let door_y = TOP + (site.door.1 - camera_y) * TILE_SIZE;
            self.art_clipped(
                art::settlement(island.region, index),
                door_x + TILE_SIZE / 2 - size as i32 / 2,
                door_y + TILE_SIZE - size as i32,
                size,
                CLIP,
            );

            if (CLIP[0]..CLIP[2]).contains(&door_x) && (CLIP[1]..CLIP[3]).contains(&door_y) {
                let marker = if site.landmark.kind == Kind::Ruin {
                    MIST
                } else {
                    GOLD
                };
                self.site_marker(door_x + TILE_SIZE / 2, door_y + TILE_SIZE - 4, marker);
            }
        }
    }

    fn draw_fauna(&mut self, island: &Island, camera_x: i32, camera_y: i32) {
        for creature in island
            .fauna
            .creatures()
            .iter()
            .filter(|creature| creature.active)
        {
            let (x, y) = creature.position;
            let center = LEFT + (x - camera_x) * TILE_SIZE + TILE_SIZE / 2;
            let feet = TOP + (y - camera_y + 1) * TILE_SIZE - 1;
            let beast = creature.species.is_beast();
            let size = if beast { 24 } else { 16 };
            if (LEFT + 4..VIEW[2] - 4).contains(&center) && (TOP..VIEW[3]).contains(&feet) {
                let shadow = if beast { 4 } else { 3 };
                self.rect(center - shadow, feet, 2 * shadow + 1, 1, 0x272334);
            }
            self.art_oriented(
                art::fauna_sprite(creature.species),
                center - size as i32 / 2,
                feet - size as i32,
                size,
                VIEW,
                creature.facing_west,
            );
        }
    }

    fn site_marker(&mut self, x: i32, y: i32, color: u32) {
        self.rect(x - 3, y - 1, 7, 3, INK);
        self.rect(x - 1, y - 3, 3, 7, INK);
        self.rect(x - 2, y, 5, 1, color);
        self.rect(x, y - 2, 1, 5, color);
        self.rect(x, y, 1, 1, IVORY);
    }
}

use super::{
    allen,
    guide::{Area, Guidance},
    map_art::{self as art, Ground},
    minimap::{Ground as MapGround, Landmark, Minimap},
    view::{View, ROWS, TILE_SIZE, TOP, ZOOM},
    Frame, GOLD, INK, IVORY, LINE, MINT, MIST,
};
use crate::{
    island::{Island, Kind, Tile, HEIGHT, WIDTH},
    world_map::Region,
};

impl Frame {
    pub(super) fn draw_island(
        &mut self,
        island: &Island,
        guide_site: Option<usize>,
        event_action: Option<&str>,
        show_minimap: bool,
    ) {
        let player = (island.x, island.y);
        let view = View::following(self.width as i32, player, (WIDTH, HEIGHT), &island.walk);
        let (title, _) = island.region.caption();
        self.label(view.left, 6, 12, GOLD, title);
        self.label(view.clip[2] - 101, 8, 9, MIST, "M / Esc  全体図");
        self.rect(
            view.left - 1,
            TOP - 1,
            view.columns * TILE_SIZE + 2,
            ROWS * TILE_SIZE + 2,
            LINE,
        );

        for (map_x, map_y) in view.visible_tiles() {
            let (screen_x, screen_y) = view.point((map_x, map_y));
            self.terrain(
                (screen_x, screen_y),
                (map_x, map_y),
                art::around(map_x, map_y, |x, y| Self::ground(island, x, y)),
                TILE_SIZE,
                view.clip,
            );
            if island.tile(map_x, map_y) == Tile::Cliff
                && island.tile(map_x, map_y + 1) != Tile::Cliff
            {
                let foot = screen_y + TILE_SIZE;
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
                    self.rect_clipped(screen_x + offset, foot - height - 1, 1, 1, view.clip, rock);
                    self.rect_clipped(
                        screen_x + offset,
                        foot - height,
                        1,
                        height,
                        view.clip,
                        shade,
                    );
                }
            }
            self.draw_rail_tile(
                screen_x,
                screen_y,
                island.traffic.tracks_at(map_x, map_y),
                view.clip,
            );
        }
        self.draw_island_objects(island, view);
        self.draw_settlements(island, view);
        self.draw_traffic(island, view);
        self.draw_fauna(island, view);
        self.draw_explorer(
            allen(island.facing, island.walk.pose()),
            view,
            player,
            &island.walk,
        );
        let destination = guide_site.map(|index| &island.settlements()[index]);
        let remaining = destination.and_then(|destination| {
            self.draw_guide(
                Guidance {
                    area: Area::Island(island.region),
                    from: (island.x, island.y),
                    to: destination.door,
                    size: (WIDTH, HEIGHT),
                },
                |(x, y)| island.tile(x, y).walkable(),
                |(x, y)| matches!(island.tile(x, y), Tile::Road | Tile::Bridge | Tile::Plaza),
                |at| view.feet(at),
                view.clip,
            )
        });
        if show_minimap {
            self.draw_minimap(
                view,
                Minimap {
                    area: Area::Island(island.region),
                    size: (WIDTH, HEIGHT),
                    player,
                    target: destination.map(|site| site.door),
                },
                |(x, y)| match island.tile(x, y) {
                    Tile::Sea | Tile::Water => MapGround::Water,
                    Tile::Road | Tile::Bridge | Tile::Plaza => MapGround::Road,
                    Tile::Forest | Tile::Grove | Tile::Mire => MapGround::Woods,
                    Tile::Ash | Tile::Cliff | Tile::Ruin => MapGround::Rock,
                    Tile::Beach => MapGround::Sand,
                    Tile::Lava => MapGround::Lava,
                    _ => MapGround::Meadow,
                },
                island.scenic_sites().map(|at| (at, Landmark::Place)).chain(
                    island.settlements().iter().map(|settlement| {
                        let kind = if settlement.landmark.kind == Kind::Harbor {
                            Landmark::Exit
                        } else {
                            Landmark::Place
                        };
                        (settlement.door, kind)
                    }),
                ),
            );
        }

        self.panel(view.left - 1, 191, view.width() + 2, 46);
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
            if let Some(destination) = destination {
                self.label(
                    24,
                    210,
                    10,
                    MINT,
                    format!("案内精霊トモル　{}へ", destination.landmark.name),
                );
            } else {
                self.label(
                    24,
                    210,
                    10,
                    IVORY,
                    "街や廃墟の手前にある目印まで歩いてみよう。",
                );
            }
        }
        self.label(24, 224, 9, MIST, "J 荷袋");
        if let Some(action) = event_action {
            self.label(126, 224, 9, MINT, action);
        } else if island.region == Region::Nox
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
            match remaining.filter(|&distance| distance > 0) {
                Some(distance) => format!("あと{distance}歩"),
                None => format!("{:02} / {:02}", island.x + 1, island.y + 1),
            },
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

    fn draw_island_objects(&mut self, island: &Island, view: View) {
        let (camera_x, camera_y) = view.camera;
        for sy in -2..ROWS + 3 {
            for sx in -2..view.columns + 2 {
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
                    let size = size * ZOOM as usize;
                    let jitter_x = ((variation & 3) as i32 - 1) * ZOOM;
                    let jitter_y = (((variation >> 2) & 3) as i32 - 1) * ZOOM;
                    let (px, py) = view.feet((x, y));
                    self.art_clipped(
                        sprite,
                        px - size as i32 / 2 + jitter_x,
                        py + 3 - size as i32 + jitter_y,
                        size,
                        view.clip,
                    );
                }
            }
        }
        for (x, y) in view.visible_tiles() {
            if matches!(island.tile(x, y), Tile::Landmark(_)) {
                let (px, py) = view.feet((x, y));
                self.site_marker(px, py - 1, view.clip, GOLD);
            }
        }
    }

    fn draw_settlements(&mut self, island: &Island, view: View) {
        for (index, site) in island.settlements().iter().enumerate() {
            let size = site.sprite_size() * ZOOM as usize;
            let (door_x, door_y) = view.point(site.door);
            self.art_clipped(
                art::settlement(island.region, index),
                door_x + TILE_SIZE / 2 - size as i32 / 2,
                door_y + TILE_SIZE - size as i32,
                size,
                view.clip,
            );

            if (view.clip[0]..view.clip[2]).contains(&door_x)
                && (view.clip[1]..view.clip[3]).contains(&door_y)
            {
                let marker = if site.landmark.kind == Kind::Ruin {
                    MIST
                } else {
                    GOLD
                };
                self.site_marker(
                    door_x + TILE_SIZE / 2,
                    door_y + TILE_SIZE - 4,
                    view.clip,
                    marker,
                );
            }
        }
    }

    fn draw_fauna(&mut self, island: &Island, view: View) {
        for creature in island
            .fauna
            .creatures()
            .iter()
            .filter(|creature| creature.active)
        {
            let (center, y) = view.feet(creature.position);
            let feet = y + 2;
            let beast = creature.species.is_beast();
            let size = (if beast { 24 } else { 16 }) * ZOOM as usize;
            if (view.left + 4..view.clip[2] - 4).contains(&center)
                && (TOP..view.clip[3]).contains(&feet)
            {
                let shadow = if beast { 4 } else { 3 };
                self.rect(center - shadow, feet, 2 * shadow + 1, 1, 0x272334);
            }
            self.art_oriented(
                art::fauna_sprite(creature.species),
                center - size as i32 / 2,
                feet - size as i32,
                size,
                view.clip,
                creature.facing_west,
            );
        }
    }

    fn site_marker(&mut self, x: i32, y: i32, clip: [i32; 4], color: u32) {
        self.rect_clipped(x - 3, y - 1, 7, 3, clip, INK);
        self.rect_clipped(x - 1, y - 3, 3, 7, clip, INK);
        self.rect_clipped(x - 2, y, 5, 1, clip, color);
        self.rect_clipped(x, y - 2, 1, 5, clip, color);
        self.rect_clipped(x, y, 1, 1, clip, IVORY);
    }
}

use super::{Frame, GOLD, INK, IVORY, LINE, MINT, MIST};
use crate::world_map::{Region, WorldMap, MAP};

const LEFT: i32 = 16;
const TOP: i32 = 24;
const CELL: i32 = 12;

impl Frame {
    pub(super) fn draw_world_map(&mut self, world: &WorldMap) {
        if let Some(town) = &world.town {
            self.draw_town(town, &world.town_progress[town.index]);
            return;
        }
        if let Some(island) = &world.island {
            self.draw_island(island, world.guide_site(island));
            if let Some(beat) = world.chapter_at_ruin() {
                if world.campaign.reading {
                    let width = (self.width as i32 - 20).min(344);
                    let x = (self.width as i32 - width) / 2;
                    self.panel(x, 38, width, 145);
                    self.label(x + 11, 47, 12, GOLD, beat.title);
                    self.dialogue_content((x + 11, 65), width - 22, &[beat.text], beat.delivery);
                }
                self.rect(122, 222, 96, 13, super::PANEL);
                self.label(
                    126,
                    224,
                    9,
                    MINT,
                    if world.campaign.reading {
                        "決定　続ける"
                    } else {
                        "決定　今の目的"
                    },
                );
            }
            return;
        }
        self.label(16, 6, 12, GOLD, "環状列島ヴェルデリア");
        self.label(152, 8, 9, MIST, "矢印  島を選ぶ");
        self.label(224, 8, 9, MIST, "M / Esc  戻る");
        self.rect(LEFT - 1, TOP - 1, 290, 182, LINE);

        for (row, tiles) in MAP.iter().enumerate() {
            for (col, &tile) in tiles.iter().enumerate() {
                self.world_tile(col as i32, row as i32, tile);
            }
        }

        let (col, row) = world.selected.port();
        let px = LEFT + col * CELL;
        let py = TOP + row * CELL;
        self.rect(px - 1, py - 1, 5, 1, GOLD);
        self.rect(px - 1, py - 1, 1, 5, GOLD);
        self.rect(px + 8, py + 12, 5, 1, GOLD);
        self.rect(px + 12, py + 8, 1, 5, GOLD);

        for (x, y, name) in [
            (44, 52, "セレスティア"),
            (225, 53, "シルヴァ"),
            (143, 69, "リベルタ"),
            (135, 123, "虚無の海"),
            (43, 163, "ノクス"),
            (232, 159, "ヴァルカン"),
        ] {
            let width = name.chars().count() as i32 * 9 + 6;
            self.rect(x, y, width, 12, 0x172437);
            self.label(x + 3, y + 1, 9, IVORY, name);
        }

        self.panel(15, 207, 290, 30);
        let (location, detail) = world.selected.caption();
        self.label(23, 210, 11, GOLD, location);
        self.label(23, 223, 10, IVORY, detail);
        self.label(205, 210, 9, MINT, "Enter / Z  上陸");
    }

    fn world_tile(&mut self, col: i32, row: i32, tile: u8) {
        let x = LEFT + col * CELL;
        let y = TOP + row * CELL;
        let variant = (col * 7 + row * 11) % 5;

        let Some(region) = Region::at(tile) else {
            if tile == b'#' {
                self.rect(x, y, CELL, CELL, 0x13172e);
                self.rect(x + variant, y + 2, 7, 1, 0x282440);
                self.rect(x + 2, y + 9 - variant, 5, 1, 0x1e203c);
            } else {
                self.rect(x, y, CELL, CELL, 0x244358);
                if variant % 2 == 0 {
                    self.rect(x + variant, y + 3, 4, 1, 0x3a6170);
                }
                self.rect(x + 6, y + 8 + variant / 2, 3, 1, 0x305569);
            }
            return;
        };

        let (ground, edge, shade, highlight) = match region {
            Region::Celestia => (0x798270, 0xb4b9a0, 0x58666a, 0xe4dbc4),
            Region::Sylva => (0x316049, 0x83a67b, 0x20483d, 0x609359),
            Region::Liberta => (0x48726a, 0x9bb7a1, 0x395969, 0x8da8a3),
            Region::Nox => (0x62596e, 0x95819a, 0x423c56, 0x9987a0),
            Region::Vulcan => (0x805b4a, 0xb18369, 0x553d47, 0xe19058),
        };
        self.rect(x, y, CELL, CELL, ground);
        for (dx, dy, ox, oy, w, h) in [
            (-1, 0, 0, 0, 1, CELL),
            (1, 0, CELL - 1, 0, 1, CELL),
            (0, -1, 0, 0, CELL, 1),
            (0, 1, 0, CELL - 1, CELL, 1),
        ] {
            if WorldMap::tile(col + dx, row + dy) == Some(b'.') {
                self.rect(x + ox, y + oy, w, h, edge);
            }
        }

        if tile.is_ascii_uppercase() {
            self.rect(x + 2, y + 8, 8, 2, INK);
            self.rect(x + 4, y + 3, 1, 6, IVORY);
            self.rect(x + 5, y + 3, 4, 3, IVORY);
            self.rect(x + 2, y + 7, 8, 2, GOLD);
            return;
        }

        match region {
            Region::Celestia | Region::Vulcan if variant != 0 => {
                self.rect(x + 3, y + 6, 7, 3, shade);
                self.rect(x + 4, y + 4, 5, 2, shade);
                self.rect(x + 5, y + 3, 3, 2, highlight);
            }
            Region::Sylva => {
                let tx = x + 2 + variant;
                self.rect(tx + 2, y + 7, 2, 3, shade);
                self.rect(tx, y + 5, 5, 3, shade);
                self.rect(tx + 1, y + 3, 3, 3, highlight);
            }
            _ => {
                self.rect(x + 2 + variant, y + 4, 4, 1, highlight);
                self.rect(x + 4, y + 8, 5, 1, shade);
            }
        }
    }
}

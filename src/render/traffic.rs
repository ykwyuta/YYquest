use super::{
    map_art as art,
    view::{TILE_SIZE, ZOOM},
    Frame, INK,
};
use crate::island::{
    traffic::{Vehicle, EAST, NORTH, SOUTH, WEST},
    Island,
};

impl Frame {
    pub(super) fn draw_rail_tile(&mut self, x: i32, y: i32, directions: u8) {
        if directions == 0 {
            return;
        }
        const BED: u32 = 0x61564f;
        const TIE: u32 = 0x7c6550;
        const TRACK: u32 = 0xbfb5a4;
        let mut paint = |left, top, w, h, color| {
            self.rect(x + left * ZOOM, y + top * ZOOM, w * ZOOM, h * ZOOM, color);
        };
        for direction in [NORTH, SOUTH] {
            if directions & direction == 0 {
                continue;
            }
            let top = if direction == NORTH { 0 } else { 6 };
            paint(2, top, 8, 6, BED);
            for offset in [1, 4] {
                paint(1, top + offset, 10, 1, TIE);
            }
            paint(3, top, 1, 6, TRACK);
            paint(8, top, 1, 6, TRACK);
        }
        for direction in [WEST, EAST] {
            if directions & direction == 0 {
                continue;
            }
            let left = if direction == WEST { 0 } else { 6 };
            paint(left, 2, 6, 8, BED);
            for offset in [1, 4] {
                paint(left + offset, 1, 1, 10, TIE);
            }
            paint(left, 3, 6, 1, TRACK);
            paint(left, 8, 6, 1, TRACK);
        }
        if directions.count_ones() == 2
            && directions & (NORTH | SOUTH) != 0
            && directions & (EAST | WEST) != 0
        {
            paint(2, 2, 8, 8, BED);
            for (left, top, width, height) in
                [(3, 0, 1, 9), (3, 8, 9, 1), (8, 0, 1, 4), (8, 3, 4, 1)]
            {
                let px = if directions & WEST != 0 {
                    12 - left - width
                } else {
                    left
                };
                let py = if directions & SOUTH != 0 {
                    12 - top - height
                } else {
                    top
                };
                paint(px, py, width, height, TRACK);
            }
        }
        if directions.count_ones() == 1 {
            paint(5, 5, 2, 2, 0x86d7c5);
        }
    }

    pub(super) fn draw_traffic(&mut self, island: &Island, camera: (i32, i32), clip: [i32; 4]) {
        for motion in island.traffic.vehicles() {
            let x = clip[0] + (motion.x - camera.0 * 16) * TILE_SIZE / 16;
            let y = clip[1] + (motion.y - camera.1 * 16) * TILE_SIZE / 16;
            let train = motion.vehicle == Vehicle::Train;
            let sprite = match (motion.vehicle, motion.direction) {
                (Vehicle::Train, NORTH) => art::TRAIN_NORTH,
                (Vehicle::Train, SOUTH) => art::TRAIN_SOUTH,
                (Vehicle::Train, _) => art::TRAIN_EAST,
                (Vehicle::Carriage, NORTH) => art::CARRIAGE_NORTH,
                (Vehicle::Carriage, SOUTH) => art::CARRIAGE_SOUTH,
                (Vehicle::Carriage, _) => art::CARRIAGE_EAST,
            };
            let size = (if train { 30 } else { 26 }) * ZOOM as usize;
            let mark = |frame: &mut Self, px: i32, py: i32, color| {
                if (clip[0]..clip[2]).contains(&px) && (clip[1]..clip[3]).contains(&py) {
                    frame.rect(px, py, 1, 1, color);
                }
            };
            if train {
                let (dx, dy) = match motion.direction {
                    NORTH => (0, 1),
                    SOUTH => (0, -1),
                    WEST => (1, 0),
                    _ => (-1, 0),
                };
                for (offset, color) in [(8, 0x87f2e3), (12, 0x57adbd), (15, 0x56849f)] {
                    let px = x + (dx * offset + dy) * ZOOM;
                    let py = y + (dy * offset - 2 + dx) * ZOOM;
                    mark(self, px, py, color);
                    mark(self, px + 1, py, color);
                }
            }
            for shadow in -6 * ZOOM..=6 * ZOOM {
                mark(self, x + shadow, y + 4 * ZOOM, INK);
            }
            self.art_oriented(
                sprite,
                x - size as i32 / 2,
                y + 5 * ZOOM - size as i32,
                size,
                clip,
                motion.direction == WEST,
            );
        }
    }
}

use super::{Frame, Sprite, GOLD, INK};

pub(super) const ZOOM: i32 = 2;
pub(super) const TILE_SIZE: i32 = 12 * ZOOM;
pub(super) const ROWS: i32 = 7;
pub(super) const TOP: i32 = 22;

#[derive(Clone, Copy)]
pub(super) struct View {
    pub left: i32,
    pub columns: i32,
    pub camera: (i32, i32),
    pub clip: [i32; 4],
}

impl View {
    pub(super) fn new(width: i32, (x, y): (i32, i32), (w, h): (i32, i32)) -> Self {
        let columns = (width - 8) / TILE_SIZE;
        let left = (width - columns * TILE_SIZE) / 2;
        let camera_x = if w < columns {
            (w - columns).div_euclid(2)
        } else {
            (x - columns / 2).clamp(0, w - columns)
        };
        Self {
            left,
            columns,
            camera: (camera_x, (y - ROWS / 2).clamp(0, h - ROWS)),
            clip: [
                left,
                TOP,
                left + columns * TILE_SIZE,
                TOP + ROWS * TILE_SIZE,
            ],
        }
    }

    pub(super) fn point(self, (x, y): (i32, i32)) -> (i32, i32) {
        (
            self.left + (x - self.camera.0) * TILE_SIZE,
            TOP + (y - self.camera.1) * TILE_SIZE,
        )
    }

    pub(super) fn feet(self, at: (i32, i32)) -> (i32, i32) {
        let (x, y) = self.point(at);
        (x + TILE_SIZE / 2, y + TILE_SIZE - 3)
    }

    pub(super) fn width(self) -> i32 {
        self.columns * TILE_SIZE
    }
}

impl Frame {
    pub(super) fn draw_explorer(&mut self, sprite: Sprite, view: View, at: (i32, i32)) {
        let (x, y) = view.feet(at);
        // A light rim at Allen's feet keeps his position readable over any ground.
        self.rect(x - 8, y - 1, 16, 4, INK);
        self.rect(x - 5, y - 2, 10, 6, INK);
        self.rect(x - 6, y, 12, 2, GOLD);
        self.rect(x - 3, y - 1, 6, 3, 0x6f5836);
        const SIZE: usize = 32;
        self.art_clipped(
            sprite,
            x - SIZE as i32 / 2,
            y - SIZE as i32,
            SIZE,
            view.clip,
        );
    }
}

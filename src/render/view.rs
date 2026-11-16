use super::{Frame, Sprite, GOLD, INK};
use crate::walk::Walk;

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
    scroll: (i32, i32),
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
            scroll: (0, 0),
        }
    }

    pub(super) fn following(width: i32, at: (i32, i32), size: (i32, i32), walk: &Walk) -> Self {
        let mut view = Self::new(width, at, size);
        let (dx, dy) = walk.offset(TILE_SIZE);
        let previous = Self::new(width, (at.0 + dx.signum(), at.1 + dy.signum()), size);
        if previous.camera.0 != view.camera.0 {
            view.scroll.0 = -dx;
        }
        if previous.camera.1 != view.camera.1 {
            view.scroll.1 = -dy;
        }
        view
    }

    pub(super) fn visible_tiles(self) -> impl Iterator<Item = (i32, i32)> {
        let start = |scroll: i32| if scroll > 0 { -1 } else { 0 };
        let extra = |scroll: i32| i32::from(scroll < 0);
        (start(self.scroll.1)..ROWS + extra(self.scroll.1)).flat_map(move |row| {
            (start(self.scroll.0)..self.columns + extra(self.scroll.0))
                .map(move |col| (self.camera.0 + col, self.camera.1 + row))
        })
    }

    pub(super) fn point(self, (x, y): (i32, i32)) -> (i32, i32) {
        (
            self.left + (x - self.camera.0) * TILE_SIZE + self.scroll.0,
            TOP + (y - self.camera.1) * TILE_SIZE + self.scroll.1,
        )
    }

    pub(super) fn feet(self, at: (i32, i32)) -> (i32, i32) {
        let (x, y) = self.point(at);
        (x + TILE_SIZE / 2, y + TILE_SIZE - 3)
    }

    pub(super) fn width(self) -> i32 {
        self.columns * TILE_SIZE
    }

    pub(super) fn walking_feet(self, at: (i32, i32), walk: &Walk) -> (i32, i32) {
        let (x, y) = self.feet(at);
        let (dx, dy) = walk.offset(TILE_SIZE);
        (x + dx, y + dy)
    }
}

impl Frame {
    pub(super) fn draw_explorer(
        &mut self,
        sprite: Sprite,
        view: View,
        at: (i32, i32),
        walk: &Walk,
    ) {
        let (x, y) = view.walking_feet(at, walk);
        // A light rim at Allen's feet keeps his position readable over any ground.
        self.rect_clipped(x - 8, y - 1, 16, 4, view.clip, INK);
        self.rect_clipped(x - 5, y - 2, 10, 6, view.clip, INK);
        self.rect_clipped(x - 6, y, 12, 2, view.clip, GOLD);
        self.rect_clipped(x - 3, y - 1, 6, 3, view.clip, 0x6f5836);
        const SIZE: usize = 32;
        self.art_clipped(
            sprite,
            x - SIZE as i32 / 2,
            y - SIZE as i32 - walk.lift(),
            SIZE,
            view.clip,
        );
    }
}

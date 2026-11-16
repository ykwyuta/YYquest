use super::{guide::Area, view::View, Frame, GOLD, INK, IVORY};
use crate::{game::Game, intro, island::Position};

const PLAYER: u32 = 0x78ffbe;
const EXIT: u32 = 0x8ddfff;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub(super) enum Ground {
    Water = 0x205774,
    Road = 0xb38c5d,
    Meadow = 0x516b4c,
    Woods = 0x2d4b3a,
    Rock = 0x4c4559,
    Sand = 0x897658,
    Lava = 0x993e2b,
}

#[derive(Clone, Copy)]
pub(super) enum Landmark {
    Place,
    Exit,
}

pub(super) struct Minimap {
    pub area: Area,
    pub size: Position,
    pub player: Position,
    pub target: Option<Position>,
}

pub(super) struct Terrain {
    area: Area,
    size: Position,
    colors: Vec<Ground>,
}

impl Terrain {
    fn new(
        area: Area,
        (map_width, map_height): Position,
        (width, height): Position,
        ground: impl Fn(Position) -> Ground,
    ) -> Self {
        let mut colors = Vec::with_capacity((map_width * map_height) as usize);
        for y in 0..map_height {
            let top = y * height / map_height;
            let bottom = ((y + 1) * height / map_height).max(top + 1);
            for x in 0..map_width {
                let left = x * width / map_width;
                let right = ((x + 1) * width / map_width).max(left + 1);
                // Keep narrow roads visible when several tiles share one map pixel.
                let road = (top..bottom)
                    .any(|row| (left..right).any(|col| ground((col, row)) == Ground::Road));
                colors.push(if road {
                    Ground::Road
                } else {
                    ground(((left + right - 1) / 2, (top + bottom - 1) / 2))
                });
            }
        }
        Self {
            area,
            size: (map_width, map_height),
            colors,
        }
    }
}

impl Frame {
    pub(super) fn draw_minimap(
        &mut self,
        view: View,
        map: Minimap,
        ground: impl Fn(Position) -> Ground,
        landmarks: impl IntoIterator<Item = (Position, Landmark)>,
    ) {
        let compact = view.width() - self.minimap_inset < 236;
        let (width, height, margin) = if compact { (68, 41, 0) } else { (100, 58, 4) };
        let left = view.clip[2] - width - 10 - margin - self.minimap_inset;
        let top = view.clip[1] + 4;
        self.panel(left, top, width + 10, height + 31);
        let origin = (left + 5, top + 5);
        let cached = self
            .minimap
            .as_ref()
            .map(|terrain| (terrain.area, terrain.size));
        if cached != Some((map.area, (width, height))) {
            self.minimap = Some(Terrain::new(map.area, (width, height), map.size, ground));
        }
        for i in 0..width * height {
            let color = self.minimap.as_ref().unwrap().colors[i as usize];
            self.rect(
                origin.0 + i % width,
                origin.1 + i / width,
                1,
                1,
                color as u32,
            );
        }

        let point = |(x, y): Position| {
            (
                origin.0 + (2 * x + 1) * width / (2 * map.size.0),
                origin.1 + (2 * y + 1) * height / (2 * map.size.1),
            )
        };
        for (at, kind) in landmarks {
            let (x, y) = point(at);
            match kind {
                Landmark::Place => self.minimap_dot(x, y),
                Landmark::Exit => self.minimap_exit(x, y),
            }
        }
        if let Some(target) = map.target {
            let (x, y) = point(target);
            self.minimap_goal(x, y);
        }
        let (x, y) = point(map.player);
        self.minimap_player(x, y);

        let row = top + height + 12;
        let right = left + if compact { 45 } else { 61 };
        let text_size = if compact { 8 } else { 9 };
        self.minimap_player(left + 9, row);
        self.label(left + 16, row - 5, text_size, IVORY, "現在地");
        if map.target.is_some() {
            self.minimap_goal(right, row);
            self.label(right + 7, row - 5, text_size, IVORY, "目的地");
        }
        self.minimap_dot(left + 9, row + 12);
        self.label(left + 16, row + 7, text_size, IVORY, "名所");
        let exit_label = match map.area {
            Area::Prologue => None,
            Area::Island(_) => Some("港"),
            Area::Town { .. } => Some("出口"),
        };
        if let Some(label) = exit_label {
            self.minimap_exit(right, row + 12);
            self.label(right + 7, row + 7, text_size, IVORY, label);
        }
    }

    fn minimap_dot(&mut self, x: i32, y: i32) {
        self.rect(x - 2, y - 2, 5, 5, INK);
        self.rect(x - 1, y - 1, 3, 3, IVORY);
    }

    fn minimap_exit(&mut self, x: i32, y: i32) {
        self.rect(x - 3, y - 3, 7, 7, INK);
        self.rect(x - 2, y, 5, 1, EXIT);
        self.rect(x, y - 2, 1, 5, EXIT);
    }

    fn minimap_goal(&mut self, x: i32, y: i32) {
        for edge in [-4, 4] {
            self.rect(x - 4, y + edge, 9, 1, GOLD);
            self.rect(x + edge, y - 4, 1, 9, GOLD);
        }
    }

    fn minimap_player(&mut self, x: i32, y: i32) {
        for dy in -3_i32..=3 {
            let radius = 3 - dy.abs();
            self.rect(x - radius, y + dy, 2 * radius + 1, 1, INK);
            if radius > 0 {
                self.rect(x - radius + 1, y + dy, 2 * radius - 1, 1, PLAYER);
            }
        }
    }

    pub(super) fn draw_prologue_minimap(&mut self, view: View, game: &Game) {
        let map = Minimap {
            area: Area::Prologue,
            size: crate::game::PROLOGUE_SIZE,
            player: (game.x, game.y),
            target: (game.intro != intro::Intro::Nox).then(|| game.intro.destination().0),
        };
        self.draw_minimap(
            view,
            map,
            |(x, y)| match Game::tile(x, y) {
                b'~' => Ground::Water,
                b'#' => Ground::Woods,
                b'.' | b'H' | b'L' | b'R' | b'S' => Ground::Road,
                _ => Ground::Meadow,
            },
            [
                intro::HEARTH,
                intro::MEETING,
                intro::SLUICE,
                intro::MEMORIAL,
            ]
            .map(|at| (at, Landmark::Place)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{DISPLAY_SCALE, WIDTH};

    #[test]
    fn current_position_moves_without_hiding_the_destination() {
        let mut frame = Frame::new();
        let size = (100, 58);
        let view = View::new(WIDTH as i32, (20, 20), size);
        let goal = (20, 20);
        let draw = |frame: &mut Frame, player| {
            frame.draw_minimap(
                view,
                Minimap {
                    area: Area::Prologue,
                    size,
                    player,
                    target: Some(goal),
                },
                |_| Ground::Meadow,
                [(goal, Landmark::Place)],
            );
        };
        let origin = (view.clip[2] - 109, view.clip[1] + 9);
        let pixel = |frame: &Frame, (x, y): Position| {
            frame.pixels[(origin.1 + y) as usize * DISPLAY_SCALE * frame.buffer_width()
                + (origin.0 + x) as usize * DISPLAY_SCALE]
        };

        draw(&mut frame, goal);
        assert_eq!(pixel(&frame, goal), PLAYER);
        assert_eq!(pixel(&frame, (24, 20)), GOLD);
        draw(&mut frame, (40, 30));
        assert_eq!(pixel(&frame, goal), IVORY);
        assert_eq!(pixel(&frame, (40, 30)), PLAYER);
    }
}

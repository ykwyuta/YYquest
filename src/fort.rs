use crate::{
    game::{Facing, Key},
    island::Position,
};
use serde::{Deserialize, Serialize};

pub(crate) const MAP: [&[u8; 13]; 7] = [
    b"#############",
    b"#....#..R...#",
    b"#....#......#",
    b"#....G..B...#",
    b"#....#......#",
    b"#.L..#......#",
    b"#############",
];
pub(crate) const ENTRY: Position = (1, 3);

#[derive(Clone, Copy, Default)]
pub(crate) enum Panel {
    #[default]
    Walk,
    Lamps(usize),
    Wrong,
    Opened,
    Recovered,
    Record,
    Return(usize),
}

pub(crate) enum Action {
    Stay,
    Leave,
    Battle,
    Deliver { sail: bool },
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Fort {
    pub pos: Position,
    pub facing: Facing,
    pub gate: bool,
    #[serde(skip)]
    pub panel: Panel,
}

impl Fort {
    pub(crate) fn new(cleared: bool) -> Self {
        Self {
            pos: ENTRY,
            facing: Facing::East,
            gate: cleared,
            panel: Panel::Walk,
        }
    }

    pub(crate) fn tile((x, y): Position) -> u8 {
        MAP.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(b'#')
    }

    pub(crate) fn walkable(&self, at: Position) -> bool {
        match Self::tile(at) {
            b'#' => false,
            b'G' => self.gate,
            _ => self.gate || at.0 < 5,
        }
    }

    pub(crate) fn input(&mut self, key: Key, cleared: bool) -> Action {
        use Panel::*;
        if let Lamps(selected) = &mut self.panel {
            match key {
                Key::Up => *selected = (*selected + 2) % 3,
                Key::Down => *selected = (*selected + 1) % 3,
                Key::Confirm if *selected == 2 => {
                    self.gate = true;
                    self.panel = Opened;
                }
                Key::Confirm => self.panel = Wrong,
                Key::Back | Key::Map => self.panel = Walk,
                _ => {}
            }
            return Action::Stay;
        }
        if let Return(selected) = &mut self.panel {
            match key {
                Key::Up | Key::Down => *selected = 1 - *selected,
                Key::Confirm => {
                    return Action::Deliver {
                        sail: *selected == 0,
                    }
                }
                Key::Back | Key::Map => self.panel = Record,
                _ => {}
            }
            return Action::Stay;
        }
        if !matches!(self.panel, Walk) {
            if key == Key::Confirm && matches!(self.panel, Record) {
                self.panel = Return(0);
                return Action::Stay;
            }
            if matches!(key, Key::Confirm | Key::Back | Key::Map) {
                self.panel = Walk;
            }
            return Action::Stay;
        }
        if matches!(key, Key::Back | Key::Map) || (key == Key::Confirm && self.pos == ENTRY) {
            return Action::Leave;
        }
        if key == Key::Confirm {
            match Self::tile(self.pos) {
                b'L' if !self.gate => self.panel = Lamps(0),
                b'B' if !cleared => return Action::Battle,
                b'R' if cleared => self.panel = Record,
                _ => {}
            }
        } else if let Some((dx, dy)) = key.direction() {
            self.facing = Facing::from_step(dx, dy).unwrap_or(self.facing);
            let target = (self.pos.0 + dx, self.pos.1 + dy);
            if self.walkable(target) {
                self.pos = target;
            }
        }
        Action::Stay
    }
}

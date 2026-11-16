use super::{Frame, Sprite, DISPLAY_SCALE, GOLD, INK, MINT};
use crate::{
    art_sizes,
    island::Position,
    path::{Distances, DIRECTIONS, UNREACHABLE},
    town::State,
};

const SPIRIT: Sprite = Sprite::new(
    include_bytes!(concat!(env!("OUT_DIR"), "/guide-spirit.rgba")),
    art_sizes::MAP_OBJECT,
);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Area {
    Prologue,
    Nox,
    Town {
        index: usize,
        low_tide: bool,
        outcome: State,
    },
}

pub(super) struct Guidance {
    pub area: Area,
    pub from: Position,
    pub to: Position,
    pub size: Position,
}

pub(super) struct Route {
    area: Area,
    goal: Position,
    size: Position,
    remaining: Distances,
}

impl Route {
    fn new(guide: &Guidance, walkable: impl Fn(Position) -> bool) -> Self {
        Self {
            area: guide.area,
            goal: guide.to,
            size: guide.size,
            remaining: Distances::new(guide.size, guide.to, walkable),
        }
    }

    fn distance(&self, at: Position) -> u16 {
        self.remaining.get(at)
    }
    fn lead(&self, (x, y): Position, prefer_road: impl Fn(Position) -> bool) -> Option<Position> {
        let current = self.distance((x, y));
        if current == 0 || current == UNREACHABLE {
            return None;
        }
        DIRECTIONS
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|&at| self.distance(at) < current)
            .min_by_key(|&at| !prefer_road(at))
    }
}

impl Frame {
    pub(super) fn draw_guide(
        &mut self,
        guide: Guidance,
        walkable: impl Fn(Position) -> bool,
        prefer_road: impl Fn(Position) -> bool,
        point: impl Fn(Position) -> Position,
        clip: [i32; 4],
    ) -> Option<u16> {
        let route = self
            .guide
            .get_or_insert_with(|| Route::new(&guide, &walkable));
        if route.area != guide.area || route.goal != guide.to || route.size != guide.size {
            *route = Route::new(&guide, walkable);
        }
        let mut spirit = guide.from;
        let distance = route.distance(spirit);
        if distance == UNREACHABLE {
            return None;
        }
        let mut trail = [None; 3];
        for step in &mut trail {
            if let Some(next) = route.lead(spirit, &prefer_road) {
                *step = Some(next);
                spirit = next;
            } else {
                break;
            }
        }
        let [left, top, right, bottom] = clip;
        for at in trail.into_iter().flatten().filter(|&at| at != spirit) {
            let (x, y) = point(at);
            if (left + 2..right - 2).contains(&x) && (top + 2..bottom - 2).contains(&y) {
                self.rect(x - 1, y - 1, 3, 3, INK);
                self.rect(x, y, 1, 1, MINT);
            }
        }
        let (x, y) = point(spirit);
        let arrived = spirit == guide.to;
        if arrived && (left + 3..right - 3).contains(&x) && (top + 3..bottom - 3).contains(&y) {
            self.rect(x - 2, y - 1, 5, 3, INK);
            self.rect(x - 1, y, 3, 1, GOLD);
        }
        const SIDE: usize = 18 * DISPLAY_SCALE / 4;
        let scale = DISPLAY_SCALE as i32;
        let side = SIDE as i32;
        self.art_pixels(
            SPIRIT,
            (
                (x * scale - side / 2).clamp(left * scale, right * scale - side),
                ((y - 3) * scale - side).clamp(top * scale, bottom * scale - side),
            ),
            SIDE,
            clip,
            false,
        );
        Some(distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_passable_tiles_around_a_wall_and_stops_at_the_event() {
        let guide = Guidance {
            area: Area::Prologue,
            from: (1, 1),
            to: (3, 1),
            size: (5, 4),
        };
        let walkable = |(x, y)| x != 2 || y == 2;
        let route = Route::new(&guide, walkable);
        let mut at = guide.from;
        for expected in [(1, 2), (2, 2), (3, 2), (3, 1)] {
            at = route.lead(at, |_| false).unwrap();
            assert_eq!(at, expected);
        }
        assert_eq!(route.lead(at, |_| false), None);
        let blocked = Route::new(&guide, |(x, _)| x != 2);
        assert_eq!(blocked.lead(guide.from, |_| false), None);
    }
}

use std::collections::VecDeque;

use crate::island::Position;

pub(crate) const DIRECTIONS: [Position; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
pub(crate) const UNREACHABLE: u16 = u16::MAX;

pub(crate) struct Distances {
    size: Position,
    steps: Vec<u16>,
}

impl Distances {
    pub(crate) fn new(size: Position, goal: Position, walkable: impl Fn(Position) -> bool) -> Self {
        let (w, h) = size;
        let mut distances = Self {
            size,
            steps: vec![UNREACHABLE; (w * h) as usize],
        };
        let Some(end) = distances.index(goal).filter(|_| walkable(goal)) else {
            return distances;
        };
        distances.steps[end] = 0;
        let mut pending = VecDeque::from([goal]);
        while let Some((x, y)) = pending.pop_front() {
            let steps = distances.get((x, y)) + 1;
            for (dx, dy) in DIRECTIONS {
                let next = (x + dx, y + dy);
                let Some(index) = distances.index(next) else {
                    continue;
                };
                if distances.steps[index] == UNREACHABLE && walkable(next) {
                    distances.steps[index] = steps;
                    pending.push_back(next);
                }
            }
        }
        distances
    }

    fn index(&self, (x, y): Position) -> Option<usize> {
        let (w, h) = self.size;
        ((0..w).contains(&x) && (0..h).contains(&y)).then(|| (y * w + x) as usize)
    }

    pub(crate) fn get(&self, at: Position) -> u16 {
        self.index(at)
            .map_or(UNREACHABLE, |index| self.steps[index])
    }
}

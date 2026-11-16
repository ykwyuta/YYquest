use crate::island::Position;

pub(crate) const STEP_MS: u32 = 144;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Pose {
    Standing,
    Stride,
    Opposite,
    Passing,
}

pub(crate) struct Walk {
    from: Position,
    elapsed: u32,
    opposite: bool,
}

impl Default for Walk {
    fn default() -> Self {
        Self {
            from: (0, 0),
            elapsed: u32::MAX,
            opposite: false,
        }
    }
}

impl Walk {
    pub(crate) fn step(&mut self, from: Position, to: Position) {
        self.from = (from.0 - to.0, from.1 - to.1);
        if self.from.0.abs() + self.from.1.abs() != 1 {
            *self = Self::default();
            return;
        }
        self.opposite = self.elapsed <= STEP_MS + 60 && !self.opposite;
        self.elapsed = 0;
    }

    pub(crate) fn offset(&self, size: i32) -> Position {
        let distance = (STEP_MS.saturating_sub(self.elapsed) as i32 * size) / STEP_MS as i32;
        (self.from.0 * distance, self.from.1 * distance)
    }

    pub(crate) fn pose(&self) -> Pose {
        match self.elapsed {
            49..=95 => Pose::Passing,
            0..STEP_MS if self.opposite => Pose::Opposite,
            0..STEP_MS => Pose::Stride,
            _ => Pose::Standing,
        }
    }

    pub(crate) fn lift(&self) -> i32 {
        i32::from(matches!(self.elapsed, 35..=100))
    }

    pub(crate) fn advance(&mut self, milliseconds: u32) -> bool {
        let before = self.elapsed;
        self.elapsed = before.saturating_add(milliseconds);
        before < STEP_MS && before != self.elapsed
    }
}

#[derive(Default)]
pub(crate) struct Walk {
    alternate: bool,
    remaining: u32,
}

impl Walk {
    pub(crate) fn step(&mut self) {
        self.alternate = !self.alternate;
        self.remaining = 155;
    }

    pub(crate) fn alternate(&self) -> bool {
        self.remaining != 0 && self.alternate
    }

    pub(crate) fn lift(&self) -> i32 {
        i32::from((48..=110).contains(&self.remaining))
    }

    pub(crate) fn advance(&mut self, milliseconds: u32) -> bool {
        let before = (self.alternate(), self.lift());
        self.remaining = self.remaining.saturating_sub(milliseconds);
        before != (self.alternate(), self.lift())
    }
}

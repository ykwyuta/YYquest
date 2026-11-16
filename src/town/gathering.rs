use crate::{
    economy::{Goods, Item},
    island::Position,
};

use super::{distance, Panel, Town};

#[derive(Clone, Copy)]
pub(crate) struct Spot {
    pub(crate) at: Position,
    pub(crate) item: Item,
    pub(crate) quantity: u8,
}

impl Spot {
    pub(crate) fn prompt(self) -> &'static str {
        match self.item {
            Item::AshHerb => "決定で刈り取る",
            Item::BitterRoot => "決定で引き抜く",
            _ => "決定で拾う",
        }
    }

    pub(crate) fn receipt(self) -> String {
        let action = match self.item {
            Item::AshHerb => "刈り取った",
            Item::BitterRoot => "引き抜いた",
            _ => "拾った",
        };
        format!("{}を{}個{action}。", self.item.name(), self.quantity)
    }
}

pub(crate) struct Gathering {
    pub(crate) spot: Spot,
    pub(crate) result: Result<(), &'static str>,
    pub(crate) elapsed: u32,
}

impl Gathering {
    pub(crate) const EFFECT_MS: u32 = 300;
    const NOTICE_MS: u32 = 1700;
}

impl Town {
    pub(crate) fn gathering_spots(&self) -> &'static [Spot] {
        match self.index {
            0 => &[
                Spot {
                    at: (219, 82),
                    item: Item::AshHerb,
                    quantity: 3,
                },
                Spot {
                    at: (225, 82),
                    item: Item::AshWood,
                    quantity: 2,
                },
            ],
            1 => &[Spot {
                at: (80, 146),
                item: Item::BitterRoot,
                quantity: 2,
            }],
            5 => &[Spot {
                at: (164, 154),
                item: Item::BogIron,
                quantity: 3,
            }],
            _ => &[],
        }
    }

    pub(crate) fn nearby_spot(&self) -> Option<Spot> {
        self.gathering_spots()
            .iter()
            .copied()
            .find(|spot| distance(spot.at, self.pos) <= 1)
    }

    pub(crate) fn gather(&mut self, goods: &mut Goods) -> bool {
        if !matches!(self.panel, Panel::Explore) {
            return false;
        }
        let Some(spot) = self.nearby_spot() else {
            return false;
        };
        if goods.gather_wait(spot.item) == 0 {
            self.gathering = Some(Gathering {
                spot,
                result: goods.gather(spot.item, spot.quantity),
                elapsed: 0,
            });
        }
        true
    }

    pub(crate) fn advance_gathering(&mut self, milliseconds: u32) -> bool {
        let Some(gathering) = &mut self.gathering else {
            return false;
        };
        let animating = gathering.result.is_ok() && gathering.elapsed < Gathering::EFFECT_MS;
        gathering.elapsed = gathering.elapsed.saturating_add(milliseconds);
        if gathering.elapsed >= Gathering::NOTICE_MS {
            self.gathering = None;
            return true;
        }
        animating
    }
}

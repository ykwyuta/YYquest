use crate::{
    campaign::{Campaign, ChapterBeat},
    economy::Service,
    game::Key,
    island::{Island, Kind, Tile},
    town::{self, Town, Travel},
};

pub(crate) const MAP: [&[u8; 24]; 15] = [
    b"........................",
    b"....ccc.cc.....ss..s....",
    b"..ccccccccc..sssss.sss..",
    b".cccccccccc..ssssssssss.",
    b".cccccccccc..ssssssssss.",
    b"..cccccCc..lL..sSsssss..",
    b"...cccc....ll.l...sss...",
    b"......c...###..l..ss....",
    b"..nn.....#####.....vv...",
    b".nnnnnn..#####...vvvvvv.",
    b".nnnnnnN..###..Vvvvvvvv.",
    b"..nnnnnnnn....vvvvvvvv..",
    b"...nnnnnnnn..vvvvvvvv...",
    b".....nnn......vv..vv....",
    b"........................",
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Region {
    Celestia,
    Sylva,
    Liberta,
    Nox,
    Vulcan,
}

impl Region {
    pub(crate) const ALL: [Self; 5] = [
        Self::Celestia,
        Self::Sylva,
        Self::Liberta,
        Self::Nox,
        Self::Vulcan,
    ];

    pub(crate) fn port(self) -> (i32, i32) {
        match self {
            Self::Celestia => (7, 5),
            Self::Sylva => (16, 5),
            Self::Liberta => (12, 5),
            Self::Nox => (7, 10),
            Self::Vulcan => (15, 10),
        }
    }

    fn neighbor(self, dx: i32, dy: i32) -> Self {
        let (x, y) = self.port();
        Self::ALL
            .into_iter()
            .filter_map(|region| {
                let (nx, ny) = region.port();
                let forward = (nx - x) * dx + (ny - y) * dy;
                let cross = (nx - x) * dy - (ny - y) * dx;
                (forward > 0).then_some((forward * forward + 4 * cross * cross, region))
            })
            .min_by_key(|&(distance, _)| distance)
            .map_or(self, |(_, region)| region)
    }

    pub(crate) fn at(tile: u8) -> Option<Self> {
        Some(match tile.to_ascii_lowercase() {
            b'c' => Self::Celestia,
            b's' => Self::Sylva,
            b'l' => Self::Liberta,
            b'n' => Self::Nox,
            b'v' => Self::Vulcan,
            _ => return None,
        })
    }

    pub(crate) fn caption(self) -> (&'static str, &'static str) {
        match self {
            Self::Celestia => ("聖律皇国セレスティア", "白亜の断崖に聖堂と騎士の砦が並ぶ。"),
            Self::Sylva => ("深緑の樹海国シルヴァ", "長命種と精霊使いが暮らす巨木の森。"),
            Self::Liberta => (
                "蒼海の自由都市群リベルタ",
                "内海の航路に浮かぶ、帆船と商人の島々。",
            ),
            Self::Nox => ("黄昏の辺境領ノクス", "灰色の霧が漂う、アレンの故郷。"),
            Self::Vulcan => ("機工連邦ヴァルカン", "火山の裾野で蒸気と魔導炉がうなる。"),
        }
    }
}

pub(crate) struct WorldMap {
    pub(crate) selected: Region,
    pub(crate) island: Option<Island>,
    pub(crate) town: Option<Town>,
    pub(crate) town_progress: town::Progress,
    pub(crate) campaign: Campaign,
}

impl WorldMap {
    pub(crate) fn new() -> Self {
        Self {
            selected: Region::Nox,
            island: None,
            town: None,
            town_progress: Default::default(),
            campaign: Campaign::default(),
        }
    }

    pub(crate) fn tile(x: i32, y: i32) -> Option<u8> {
        MAP.get(y as usize)?.get(x as usize).copied()
    }

    pub(crate) fn step(&mut self, dx: i32, dy: i32) {
        if let Some(island) = &mut self.island {
            island.step(dx, dy);
        } else {
            self.selected = self.selected.neighbor(dx, dy);
        }
    }

    pub(crate) fn confirm(&mut self) {
        if let Some(island) = &self.island {
            if island.region == Region::Nox {
                if let Tile::Settlement(index) = island.tile(island.x, island.y) {
                    if index < town::COUNT {
                        let mut town = Town::new(index, &self.town_progress[index]);
                        town.main = self.campaign.beat();
                        self.town = Some(town);
                        return;
                    }
                }
            }
            if island
                .current_landmark()
                .is_some_and(|site| site.kind == Kind::Harbor)
            {
                self.island = None;
            }
        } else {
            self.island = Some(Island::new(self.selected));
        }
    }

    pub(crate) fn guide_site(&self, island: &Island) -> Option<usize> {
        if island.region != Region::Nox {
            return None;
        }
        if let Some(beat) = self.campaign.beat() {
            return Some(beat.site);
        }
        self.town_progress
            .iter()
            .enumerate()
            .filter(|(_, states)| {
                states
                    .iter()
                    .any(|state| !matches!(state, town::State::Done(_)))
            })
            .min_by_key(|(index, states)| {
                let active = states
                    .iter()
                    .any(|state| matches!(state, town::State::Active(_)));
                let (x, y) = island.settlements()[*index].door;
                (!active, (x - island.x).abs() + (y - island.y).abs())
            })
            .map(|(index, _)| index)
    }

    pub(crate) fn town_input(&mut self, key: Key) -> Option<Service> {
        let town = self.town.as_mut()?;
        match town.input(key, &mut self.town_progress[town.index]) {
            Travel::Stay => {}
            Travel::AdvanceMain => {
                self.campaign.advance();
                town.main = self.campaign.beat();
            }
            Travel::Island => self.town = None,
            Travel::Sea => {
                self.town = None;
                self.island = None;
            }
            Travel::Service(service) => return Some(service),
        }
        None
    }

    pub(crate) fn begin_nox(&mut self) {
        self.campaign.start();
        self.selected = Region::Nox;
        self.island = Some(Island::new(Region::Nox));
        self.town = None;
        if let Some(island) = &mut self.island {
            (island.x, island.y) = island.settlements()[0].door;
        }
        self.confirm();
    }

    pub(crate) fn chapter_at_ruin(&self) -> Option<&'static ChapterBeat> {
        let step = self.campaign.beat()?;
        let island = self.island.as_ref()?;
        (island.region == Region::Nox
            && step.place.is_none()
            && (island.x, island.y) == island.settlements()[step.site].door)
            .then_some(step)
    }

    pub(crate) fn chapter_input(&mut self, key: Key) -> bool {
        if self.campaign.reading {
            match key {
                Key::Confirm => self.campaign.advance(),
                Key::Map | Key::Back => self.campaign.reading = false,
                _ => {}
            }
            return true;
        }
        if key == Key::Confirm && self.chapter_at_ruin().is_some() {
            self.campaign.reading = true;
            return true;
        }
        false
    }

    pub(crate) fn close_island(&mut self) -> bool {
        self.island.take().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_can_select_each_island_from_nox() {
        let mut world = WorldMap::new();
        world.step(0, -1);
        assert_eq!(world.selected, Region::Celestia);
        world.step(1, 0);
        assert_eq!(world.selected, Region::Liberta);
        world.step(1, 0);
        assert_eq!(world.selected, Region::Sylva);
        world.step(0, 1);
        assert_eq!(world.selected, Region::Vulcan);
        for region in Region::ALL {
            let (x, y) = region.port();
            let tile = WorldMap::tile(x, y).unwrap();
            assert!(tile.is_ascii_uppercase());
            assert_eq!(Region::at(tile), Some(region));
        }
    }
}

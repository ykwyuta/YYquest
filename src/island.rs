pub(crate) mod fauna;
mod maps;
mod terrain;
pub(crate) mod traffic;

use crate::{game::Facing, walk::Walk, world_map::Region};

const PLAN_WIDTH: usize = 26;
const PLAN_HEIGHT: usize = 18;
pub(crate) const WIDTH: i32 = 312;
pub(crate) const HEIGHT: i32 = 192;

pub(crate) type Position = (i32, i32);
pub(crate) const SETTLEMENTS: usize = 13;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Harbor,
    Town,
    Ruin,
    Shrine,
    AncientTree,
}

#[derive(Clone, Copy)]
pub(crate) struct Landmark {
    pub(crate) kind: Kind,
    pub(crate) name: &'static str,
    pub(crate) detail: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettlementScale {
    Village,
    Town,
    Center,
}

#[derive(Clone, Copy)]
pub(crate) struct Settlement {
    pub(crate) door: Position,
    pub(crate) landmark: Landmark,
    scale: SettlementScale,
}

impl Settlement {
    const fn new(x: i32, y: i32, kind: Kind, name: &'static str, detail: &'static str) -> Self {
        Self {
            door: (x, y),
            landmark: Landmark { kind, name, detail },
            scale: match kind {
                Kind::Harbor => SettlementScale::Town,
                _ => SettlementScale::Village,
            },
        }
    }

    const fn with_scale(mut self, scale: SettlementScale) -> Self {
        self.scale = scale;
        self
    }

    pub(crate) fn sprite_size(&self) -> usize {
        if self.landmark.kind == Kind::Ruin {
            return 42;
        }
        match self.scale {
            SettlementScale::Village => 48,
            SettlementScale::Town => 56,
            SettlementScale::Center => 68,
        }
    }

    fn footprint(&self) -> impl Iterator<Item = Position> {
        let (x, y) = self.door;
        let (half_width, depth) = match self.scale {
            SettlementScale::Village => (1, 2),
            SettlementScale::Town => (2, 3),
            SettlementScale::Center => (2, 4),
        };
        (y - depth..y)
            .flat_map(move |row| (x - half_width..=x + half_width).map(move |col| (col, row)))
    }
}

struct Layout {
    rows: [&'static [u8; PLAN_WIDTH]; PLAN_HEIGHT],
    landmarks: &'static [Landmark],
    settlements: [Settlement; SETTLEMENTS],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tile {
    Sea,
    Meadow,
    Beach,
    Cliff,
    Forest,
    Grove,
    Road,
    Bridge,
    Mire,
    Water,
    Ash,
    Lava,
    Ruin,
    Plaza,
    Field,
    Landmark(usize),
    Settlement(usize),
    Building(usize),
}

impl Tile {
    pub(crate) fn walkable(self) -> bool {
        !matches!(
            self,
            Self::Sea | Self::Cliff | Self::Forest | Self::Water | Self::Lava | Self::Building(_)
        )
    }
}

pub(crate) struct Island {
    pub(crate) region: Region,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) facing: Facing,
    pub(crate) walk: Walk,
    layout: &'static Layout,
    tiles: Vec<Tile>,
    settlements: [Settlement; SETTLEMENTS],
    pub(crate) traffic: traffic::Traffic,
    pub(crate) fauna: fauna::Fauna,
    harbor: Position,
}

impl Island {
    pub(crate) fn new(region: Region) -> Self {
        let layout = maps::layout(region);
        let settlements = std::array::from_fn(|index| {
            let site = layout.settlements[index];
            Settlement {
                door: terrain::scale_door(site.door),
                ..site
            }
        });
        let mut tiles = terrain::expand(layout, region);
        for (index, site) in settlements.iter().enumerate() {
            let (x, y) = site.door;
            for (col, row) in site.footprint() {
                tiles[terrain::address(col, row)] = Tile::Building(index);
            }
            tiles[terrain::address(x, y)] = Tile::Settlement(index);
        }
        let (x, y) = settlements
            .iter()
            .find(|site| site.landmark.kind == Kind::Harbor)
            .expect("Island has a harbor")
            .door;
        let harbor = (x, y);
        let traffic = traffic::Traffic::new(&mut tiles, &settlements, harbor);
        let fauna = fauna::Fauna::populate(&tiles, &settlements, region, harbor);
        Self {
            region,
            x,
            y,
            facing: Facing::South,
            walk: Walk::default(),
            layout,
            tiles,
            settlements,
            traffic,
            fauna,
            harbor,
        }
    }

    pub(crate) fn tile(&self, x: i32, y: i32) -> Tile {
        if !(0..WIDTH).contains(&x) || !(0..HEIGHT).contains(&y) {
            return Tile::Sea;
        }

        self.tiles[terrain::address(x, y)]
    }

    pub(crate) fn settlement_definitions(region: Region) -> &'static [Settlement; SETTLEMENTS] {
        &maps::layout(region).settlements
    }

    pub(crate) fn settlements(&self) -> &[Settlement; SETTLEMENTS] {
        &self.settlements
    }

    pub(crate) fn landmark(&self, index: usize) -> &Landmark {
        &self.layout.landmarks[index]
    }

    pub(crate) fn current_landmark(&self) -> Option<&Landmark> {
        match self.tile(self.x, self.y) {
            Tile::Landmark(index) => Some(self.landmark(index)),
            Tile::Settlement(index) => Some(&self.settlements()[index].landmark),
            _ => None,
        }
    }

    pub(crate) fn advance(&mut self, milliseconds: u32) -> bool {
        let traffic = self.traffic.advance(milliseconds);
        let fauna = self
            .fauna
            .advance(&self.tiles, (self.x, self.y), milliseconds);
        self.walk.advance(milliseconds) | traffic | fauna
    }

    pub(crate) fn return_to_harbor(&mut self) {
        (self.x, self.y) = self.harbor;
    }

    pub(crate) fn step(&mut self, dx: i32, dy: i32) {
        self.facing = Facing::from_step(dx, dy).unwrap_or(self.facing);
        let (x, y) = (self.x + dx, self.y + dy);
        if self.tile(x, y).walkable() {
            (self.x, self.y) = (x, y);
            self.walk.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn all_town_and_ruin_entrances_are_reachable_and_have_clear_footprints() {
        assert_eq!(WIDTH * HEIGHT, 156 * 96 * 4);
        let mut names = HashSet::new();
        for region in Region::ALL {
            let island = Island::new(region);
            let start = (island.x, island.y);
            let mut visited = HashSet::from([start]);
            let mut queue = VecDeque::from([start]);
            let mut landmarks = HashSet::new();
            while let Some((x, y)) = queue.pop_front() {
                if let Tile::Landmark(index) = island.tile(x, y) {
                    landmarks.insert(index);
                }
                for position in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if island.tile(position.0, position.1).walkable() && visited.insert(position) {
                        queue.push_back(position);
                    }
                }
            }

            assert_eq!(landmarks.len(), island.layout.landmarks.len(), "{region:?}");
            let sites = island.settlements();
            let ruins = sites
                .iter()
                .filter(|site| site.landmark.kind == Kind::Ruin)
                .count();
            assert_eq!(ruins, 3);
            assert_eq!(sites.len() - ruins, 10);
            assert!(
                sites
                    .iter()
                    .filter(|site| site.scale == SettlementScale::Center)
                    .count()
                    >= 2
            );
            for (index, site) in sites.iter().enumerate() {
                let (x, y) = site.door;
                assert!(names.insert(site.landmark.name));
                assert!(
                    visited.contains(&(x, y)),
                    "{region:?}: {}",
                    site.landmark.name
                );
                assert!(island.tile(x, y) == Tile::Settlement(index));
                assert!((site.sprite_size() == 42) == (site.landmark.kind == Kind::Ruin));
                for (col, row) in site.footprint() {
                    assert!(island.tile(col, row) == Tile::Building(index));
                }
            }
        }
        assert_eq!(names.len(), 65);
    }

    #[test]
    fn port_building_blocks_movement_but_player_can_leave_the_entrance() {
        let mut island = Island::new(Region::Nox);
        let start = (island.x, island.y);
        island.step(0, -1);
        assert_eq!((island.x, island.y), start);
        island.step(1, 0);
        assert_eq!((island.x, island.y), (start.0 + 1, start.1));
    }
}

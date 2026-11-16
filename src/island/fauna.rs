mod catalog;

use std::collections::VecDeque;

use super::{terrain::address, Kind, Position, Settlement, Tile, HEIGHT, WIDTH};
use crate::{
    game::{Beast, Enemy},
    noise,
    world_map::Region,
};

const DIRECTIONS: [Position; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
pub(crate) const ANIMAL_SPECIES: usize = 10;
pub(crate) const SPECIES_PER_ISLAND: usize = 15;

#[derive(Clone, Copy)]
enum Habitat {
    Open,
    Woodland,
    Wetland,
    Shore,
    Stone,
    Farmland,
}

impl Habitat {
    fn matches(self, tile: Tile) -> bool {
        match self {
            Self::Open => matches!(tile, Tile::Ash | Tile::Meadow),
            Self::Woodland => tile == Tile::Grove,
            Self::Wetland => matches!(tile, Tile::Mire | Tile::Ruin),
            Self::Shore => tile == Tile::Beach,
            Self::Stone => matches!(tile, Tile::Ash | Tile::Ruin),
            Self::Farmland => matches!(tile, Tile::Field | Tile::Meadow),
        }
    }
}

#[derive(Clone, Copy)]
struct SpeciesData {
    name: &'static str,
    habitat: Habitat,
    combat: (u8, u8),
}

impl SpeciesData {
    const fn animal(name: &'static str, habitat: Habitat) -> Self {
        Self {
            name,
            habitat,
            combat: (0, 0),
        }
    }

    const fn beast(name: &'static str, habitat: Habitat, health: u8, damage: u8) -> Self {
        Self {
            name,
            habitat,
            combat: (health, damage),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Species {
    pub(crate) region: Region,
    pub(crate) index: usize,
}

impl Species {
    pub(crate) const fn new(region: Region, index: usize) -> Self {
        Self { region, index }
    }

    pub(crate) fn name(self) -> &'static str {
        catalog::get(self).name
    }

    pub(crate) fn enemy(self) -> Option<Enemy> {
        self.is_beast().then_some(Enemy::Beast(Beast(self)))
    }

    pub(crate) fn is_beast(self) -> bool {
        self.index >= ANIMAL_SPECIES
    }

    pub(crate) fn combat(self) -> (u8, u8) {
        catalog::get(self).combat
    }

    fn affinity(self, tile: Tile) -> u32 {
        if catalog::get(self).habitat.matches(tile) {
            24
        } else {
            0
        }
    }
}

pub(crate) struct Creature {
    pub(crate) species: Species,
    pub(crate) position: Position,
    home: Position,
    pub(crate) facing_west: bool,
    pub(crate) active: bool,
}

pub(crate) struct Fauna {
    creatures: Vec<Creature>,
    clock: u32,
    turn: u32,
}

impl Fauna {
    pub(super) fn populate(
        tiles: &[Tile],
        sites: &[Settlement],
        region: Region,
        harbor: Position,
    ) -> Self {
        let mut fauna = Self {
            creatures: Vec::with_capacity(135),
            clock: 0,
            turn: 0,
        };
        let mut locations = SpawnGround::new(tiles, harbor);
        let mut counts = [0; SPECIES_PER_ISLAND];
        let mut place_near = |species: usize, origin, radius, ground: &mut SpawnGround| {
            let species = Species::new(region, species);
            if let Some(position) = ground.near(species, origin, radius, fauna.creatures.len()) {
                fauna.push(species, position);
                ground.reserve(position);
                counts[species.index] += 1;
            }
        };

        // Retain the recognizable harbor animals and add newer species along
        // local trails, wooded edges, and the outskirts of other settlements.
        place_near(10, harbor, 6, &mut locations);
        place_near(0, harbor, 6, &mut locations);
        place_near(1, harbor, 7, &mut locations);
        place_near(2, harbor, 10, &mut locations);
        place_near(11, harbor, 13, &mut locations);
        for (index, site) in sites
            .iter()
            .enumerate()
            .filter(|(_, site)| site.landmark.kind != Kind::Harbor)
        {
            place_near((index + 2) % ANIMAL_SPECIES, site.door, 10, &mut locations);
            if site.landmark.kind == Kind::Ruin {
                place_near(10 + (index + 1) % 5, site.door, 12, &mut locations);
            } else {
                place_near((index + 6) % ANIMAL_SPECIES, site.door, 12, &mut locations);
            }
        }

        for (index, count) in counts.into_iter().enumerate() {
            let target = match index {
                0..=4 => 12,
                5..=9 => 11,
                _ => 4,
            };
            let species = Species::new(region, index);
            for _ in count..target {
                let position = locations.scattered(species, fauna.creatures.len());
                fauna.push(species, position);
                locations.reserve(position);
            }
        }
        fauna
    }

    fn push(&mut self, species: Species, position: Position) {
        self.creatures.push(Creature {
            species,
            position,
            home: position,
            facing_west: noise::hash(position.0, position.1) & 1 != 0,
            active: true,
        });
    }

    pub(crate) fn creatures(&self) -> &[Creature] {
        &self.creatures
    }

    pub(crate) fn nearby_beast(&self, position: Position) -> Option<(usize, Enemy)> {
        self.creatures
            .iter()
            .enumerate()
            .filter_map(|(index, creature)| {
                let distance = distance_to(creature, position);
                if !creature.active || distance > 2 {
                    return None;
                }
                Some((distance, (index, creature.species.enemy()?)))
            })
            .min_by_key(|(distance, _)| *distance)
            .map(|(_, encounter)| encounter)
    }

    pub(crate) fn nearby_animal(&self, position: Position) -> Option<Species> {
        self.creatures
            .iter()
            .filter(|creature| {
                creature.active
                    && !creature.species.is_beast()
                    && distance_to(creature, position) <= 3
            })
            .min_by_key(|creature| distance_to(creature, position))
            .map(|creature| creature.species)
    }

    pub(crate) fn defeat(&mut self, index: usize) {
        self.creatures[index].active = false;
    }

    pub(super) fn advance(&mut self, tiles: &[Tile], player: Position, elapsed: u32) -> bool {
        self.clock += elapsed.min(250);
        if self.clock < 500 {
            return false;
        }
        self.clock -= 500;
        self.turn = self.turn.wrapping_add(1);
        let mut changed = false;
        for id in 0..self.creatures.len() {
            let creature = &self.creatures[id];
            let predator = creature.species.is_beast();
            if !creature.active
                || self.turn.wrapping_add(id as u32) % if predator { 5 } else { 2 } != 0
            {
                continue;
            }
            // A nearby player can choose the encounter without the target
            // walking away while the input prompt is on screen.
            if predator && distance_to(creature, player) <= 2 {
                continue;
            }
            let (x, y) = creature.position;
            let offset = (noise::hash(id as i32, self.turn as i32) & 3) as usize;
            let next = (0..4)
                .map(|step| DIRECTIONS[(step + offset) % 4])
                .find_map(|(dx, dy)| {
                    let position = (x + dx, y + dy);
                    if position != player
                        && distance(position, creature.home) <= 3
                        && terrain(tiles, position).is_some_and(habitat)
                        && !self
                            .creatures
                            .iter()
                            .any(|other| other.active && other.position == position)
                    {
                        Some((position, dx))
                    } else {
                        None
                    }
                });
            if let Some((position, dx)) = next {
                let creature = &mut self.creatures[id];
                creature.position = position;
                if dx != 0 {
                    creature.facing_west = dx < 0;
                }
                changed = true;
            }
        }
        changed
    }
}

struct SpawnGround<'a> {
    tiles: &'a [Tile],
    harbor: Position,
    reachable: Vec<bool>,
    habitat_tiles: Vec<Position>,
    reserved: Vec<bool>,
}

impl<'a> SpawnGround<'a> {
    fn new(tiles: &'a [Tile], harbor: Position) -> Self {
        let reachable = reachable_from(tiles, harbor);
        let habitat_tiles = tiles
            .iter()
            .enumerate()
            .filter_map(|(index, &tile)| {
                (reachable[index] && habitat(tile))
                    .then_some((index as i32 % WIDTH, index as i32 / WIDTH))
            })
            .collect();
        Self {
            tiles,
            harbor,
            reachable,
            habitat_tiles,
            reserved: vec![false; tiles.len()],
        }
    }

    fn reserve(&mut self, (x, y): Position) {
        for dy in -3i32..=3 {
            for dx in (-3 + dy.abs())..=(3 - dy.abs()) {
                let position = (x + dx, y + dy);
                if terrain(self.tiles, position).is_some() {
                    self.reserved[address(position.0, position.1)] = true;
                }
            }
        }
    }

    fn near(
        &self,
        species: Species,
        origin: Position,
        radius: i32,
        serial: usize,
    ) -> Option<Position> {
        let (cx, cy) = origin;
        let mut best = None;
        let mut best_score = 0;
        for y in (cy - radius)..=(cy + radius) {
            for x in (cx - radius)..=(cx + radius) {
                let distance = distance(origin, (x, y));
                let Some(tile) = terrain(self.tiles, (x, y)) else {
                    continue;
                };
                let index = address(x, y);
                if !(4..=radius).contains(&distance)
                    || !habitat(tile)
                    || !self.reachable[index]
                    || self.reserved[index]
                {
                    continue;
                }
                let score = (radius - distance) as u32 * 4
                    + species.affinity(tile)
                    + (noise::hash(x ^ (serial as i32 * 73), y) & 31);
                if best.is_none() || score > best_score {
                    best = Some((x, y));
                    best_score = score;
                }
            }
        }
        best
    }

    fn scattered(&self, species: Species, serial: usize) -> Position {
        let seed = serial as i32 * 97;
        self.habitat_tiles
            .iter()
            .copied()
            .filter(|&(x, y)| !self.reserved[address(x, y)] && distance(self.harbor, (x, y)) >= 13)
            .max_by_key(|&(x, y)| {
                species.affinity(self.tiles[address(x, y)]) * 3
                    + (noise::hash(x ^ seed, y + seed) & 63)
            })
            .expect("Island has enough habitat for its wildlife")
    }
}

fn distance(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn distance_to(creature: &Creature, position: Position) -> i32 {
    distance(creature.position, position)
}

fn terrain(tiles: &[Tile], (x, y): Position) -> Option<Tile> {
    if !(0..WIDTH).contains(&x) || !(0..HEIGHT).contains(&y) {
        return None;
    }
    Some(tiles[address(x, y)])
}

fn habitat(tile: Tile) -> bool {
    matches!(
        tile,
        Tile::Meadow
            | Tile::Grove
            | Tile::Beach
            | Tile::Mire
            | Tile::Ash
            | Tile::Ruin
            | Tile::Field
    )
}

fn reachable_from(tiles: &[Tile], start: Position) -> Vec<bool> {
    let mut reachable = vec![false; tiles.len()];
    reachable[address(start.0, start.1)] = true;
    let mut queue = VecDeque::from([start]);
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in DIRECTIONS {
            let next = (x + dx, y + dy);
            if !terrain(tiles, next).is_some_and(Tile::walkable) {
                continue;
            }
            let index = address(next.0, next.1);
            if !reachable[index] {
                reachable[index] = true;
                queue.push_back(next);
            }
        }
    }
    reachable
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::island::Island;

    #[test]
    fn each_island_has_135_creatures_from_15_unique_species() {
        let mut names = std::collections::HashSet::new();
        for region in Region::ALL {
            let mut island = Island::new(region);
            let origin = (island.x, island.y);
            let creatures = island.fauna.creatures();
            assert_eq!(creatures.len(), 135, "{region:?}");
            assert_eq!(
                creatures
                    .iter()
                    .filter(|creature| creature.species.is_beast())
                    .count(),
                20
            );
            let mut counts = [0; SPECIES_PER_ISLAND];
            let mut positions = std::collections::HashSet::new();
            for creature in creatures {
                assert_eq!(creature.species.region, region);
                assert!(positions.insert(creature.position));
                counts[creature.species.index] += 1;
            }
            assert_eq!(
                counts,
                [12, 12, 12, 12, 12, 11, 11, 11, 11, 11, 4, 4, 4, 4, 4]
            );
            let close: Vec<_> = creatures
                .iter()
                .filter(|creature| distance_to(creature, origin) <= 7)
                .map(|creature| creature.species)
                .collect();
            assert!(
                close.iter().any(|species| species.enemy().is_some()),
                "{region:?}"
            );
            assert!(
                close
                    .iter()
                    .filter(|species| species.enemy().is_none())
                    .count()
                    >= 2,
                "{region:?}"
            );
            for index in 0..SPECIES_PER_ISLAND {
                let species = Species::new(region, index);
                assert!(names.insert(species.name()), "{}", species.name());
            }
            let before: Vec<_> = creatures.iter().map(|creature| creature.position).collect();
            for _ in 0..12 {
                island.advance(250);
            }
            assert!(island
                .fauna
                .creatures()
                .iter()
                .zip(before)
                .any(|(creature, before)| creature.position != before));
            for creature in island.fauna.creatures() {
                assert!(habitat(
                    island.tile(creature.position.0, creature.position.1)
                ));
            }
        }
    }
}

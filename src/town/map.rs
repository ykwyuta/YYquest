use super::{Art, Plan, State};
use crate::{island, island::Position, noise, world_map::Region};

pub(crate) const WIDTH: i32 = island::WIDTH;
pub(crate) const HEIGHT: i32 = island::HEIGHT;

#[derive(Clone, Copy)]
pub(crate) enum Landscape {
    Meadow,
    Coast,
    Heath,
    Marsh,
    Farm,
    Stone,
    Crossing,
    Fort,
    Valley,
    River,
    Chalk,
    Canopy,
    Canal,
    Basalt,
    Square,
    Yard,
    Wood,
    Shore,
    Dock,
    Pasture,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Surface {
    Grass,
    Woods,
    Heath,
    Mud,
    Reeds,
    Wheat,
    Gravel,
    Rock,
    Sand,
    Sea,
    Water,
    Trail,
    Paving,
    Deck,
}

impl Surface {
    fn water(self) -> bool {
        matches!(self, Self::Sea | Self::Water)
    }

    pub(crate) fn road(self) -> bool {
        matches!(self, Self::Trail | Self::Paving | Self::Deck)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Prop {
    pub(crate) at: Position,
    pub(crate) art: Art,
    pub(crate) size: i32,
}

pub(crate) struct TownMap {
    surface: Vec<Surface>,
    blocked: Vec<bool>,
    tidal_flats: Vec<Position>,
    pub(crate) props: Vec<Prop>,
}

impl TownMap {
    pub(super) fn new(plan: &Plan, region: Region, index: usize) -> Self {
        let seed = match region {
            Region::Nox => index,
            Region::Celestia => index + 17,
            Region::Sylva => index + 37,
            Region::Liberta => index + 57,
            Region::Vulcan => index + 77,
        };
        let surface = (0..HEIGHT)
            .flat_map(|y| (0..WIDTH).map(move |x| base(plan.terrain, seed, x, y)))
            .collect();
        let mut map = Self {
            surface,
            blocked: vec![false; (WIDTH * HEIGHT) as usize],
            props: Vec::new(),
            tidal_flats: Vec::new(),
        };

        for (i, place) in plan.places.iter().enumerate() {
            let (x, y) = place.at;
            let radius = match (region, index, i) {
                (Region::Nox, 7 | 9, 0 | 1) => 10,
                (_, _, 0..=3) => 6,
                _ => 4,
            };
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let edge = dx * dx + 2 * dy * dy;
                    if edge < radius * radius + variation(seed, x + dx, y + dy) % 9 {
                        let tile = patch(place.ground, x + dx, y + dy);
                        map.paint((x + dx, y + dy), tile);
                    }
                }
            }
        }
        let surface = match plan.terrain {
            Landscape::Stone | Landscape::Fort | Landscape::River | Landscape::Chalk | Landscape::Basalt => Surface::Paving,
            Landscape::Marsh | Landscape::Canal => Surface::Deck,
            _ => Surface::Trail,
        };
        let hub = if region == Region::Nox {
            match index {
                1 | 2 | 4 | 7 => 1,
                _ => 0,
            }
        } else {
            0
        };
        let center = plan.places[hub].at;
        map.route(plan.entry, center, surface, 2, seed % 2 == 0);
        for (i, place) in plan.places.iter().enumerate() {
            if i == hub {
                continue;
            }
            // Connect outlying work areas to adjacent districts rather than
            // running a separate avenue from the center to every doorstep.
            let mut parent = center;
            if region == Region::Nox && index == 8 && matches!(i, 2 | 3 | 4 | 6 | 7) {
                parent = (center.0, 114);
            }
            for (j, site) in plan.places[..i].iter().enumerate() {
                if region == Region::Nox && index == 8 && (matches!(i, 1 | 5) != matches!(j, 1 | 5)) {
                    continue;
                }
                if distance(place.at, site.at) < distance(place.at, parent) {
                    parent = site.at;
                }
            }
            map.route(parent, place.at, surface, 2, (i + seed) % 2 == 0);
        }

        let [a, b] = if region == Region::Nox {
            match index {
                0 => [1, 7],
                1 => [0, 7],
                2 => [6, 7],
                3 => [0, 5],
                4 => [2, 4],
                5 => [3, 6],
                6 => [0, 3],
                7 => [1, 3],
                8 => [2, 7],
                _ => [2, 6],
            }
        } else {
            [(index + 1) % 4 + 1, 5 + (index % 3)]
        };
        map.route(
            plan.places[a].at,
            plan.places[b].at,
            Surface::Trail,
            1,
            seed % 2 != 0,
        );

        for place in &plan.places {
            if let Some(art) = place.art {
                let pos = place.at;
                // The bridge spans the deck. Buildings stand north of their
                // interaction tile and leave a clear frontage for travelers.
                let footprint = !matches!(art, Art::Bridge);
                map.add_prop(
                    Prop {
                        at: if footprint {
                            (pos.0, pos.1 - 2)
                        } else {
                            (pos.0, pos.1 - 6)
                        },
                        art,
                        size: art.size(),
                    },
                    footprint,
                );
            }
        }
        map.add_houses(plan, region, seed, index);
        map.add_houses_at_entrance(plan.entry, seed);
        if region == Region::Nox && index == 1 {
            let west = (83..=105).flat_map(|x| [154, 155].map(move |y| (x, y)));
            let east = (105..=143).flat_map(|x| [155, 156].map(move |y| (x, y)));
            let ramps = (141..=155).flat_map(|y| [83, 143].map(move |x| (x, y)));
            map.tidal_flats = west
                .chain(east)
                .chain(ramps)
                .filter(|&at| map.surface(at) == Surface::Sea)
                .collect();
        }
        map.props.sort_unstable_by_key(|prop| prop.at.1);
        map
    }

    pub(crate) fn surface(&self, at: Position) -> Surface {
        address(at).map_or(Surface::Rock, |index| self.surface[index])
    }

    pub(crate) fn walkable(&self, at: Position) -> bool {
        address(at).is_some_and(|i| {
            !self.blocked[i] && !self.surface[i].water() && self.surface[i] != Surface::Rock
        })
    }

    pub(super) fn set_low_tide(&mut self, low: bool) {
        let surface = if low { Surface::Sand } else { Surface::Sea };
        for &at in &self.tidal_flats {
            self.surface[address(at).unwrap()] = surface;
        }
    }

    pub(super) fn apply_outcome(&mut self, region: Region, index: usize, state: State) {
        if region != Region::Nox {
            return;
        }
        match (index, state) {
            (2, State::Done(0)) => self.route((64, 102), (77, 59), Surface::Trail, 1, false),
            (3, State::Done(_)) => {
                self.route((165, 118), (170, 137), Surface::Deck, 2, false);
                self.route((170, 137), (191, 137), Surface::Deck, 2, true);
                self.route((191, 137), (191, 153), Surface::Deck, 2, false);
            }
            (8, State::Done(0)) => self.route((72, 104), (165, 48), Surface::Trail, 1, false),
            _ => {}
        }
    }

    fn paint(&mut self, at: Position, surface: Surface) {
        if let Some(index) = address(at) {
            self.surface[index] = surface;
        }
    }

    fn route(
        &mut self,
        a: Position,
        b: Position,
        surface: Surface,
        width: i32,
        across_first: bool,
    ) {
        let corner = if across_first { (b.0, a.1) } else { (a.0, b.1) };
        for (start, end) in [(a, corner), (corner, b)] {
            for x in start.0.min(end.0)..=start.0.max(end.0) {
                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    for w in 0..width {
                        let point = if start.1 == end.1 {
                            (x, y + w)
                        } else {
                            (x + w, y)
                        };
                        let previous = self.surface(point);
                        if !previous.road() {
                            self.paint(
                                point,
                                if previous.water() {
                                    Surface::Deck
                                } else {
                                    surface
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    fn add_prop(&mut self, prop: Prop, footprint: bool) {
        if footprint {
            let (x, y) = prop.at;
            let half = (prop.size - 20) / 20;
            for dx in -half..=half {
                for dy in -2..=0 {
                    if !self.surface((x + dx, y + dy)).road() {
                        if let Some(index) = address((x + dx, y + dy)) {
                            self.blocked[index] = true;
                        }
                    }
                }
            }
        }
        self.props.push(prop);
    }

    fn add_houses(&mut self, plan: &Plan, region: Region, seed: usize, index: usize) {
        const LOTS: [(i32, i32); 16] = [
            (-7, 3),
            (8, 5),
            (-7, -5),
            (8, -6),
            (-15, -4),
            (14, -4),
            (-7, 12),
            (8, 15),
            (-18, 12),
            (19, 12),
            (-11, -13),
            (9, -12),
            (-24, 1),
            (25, 1),
            (20, -12),
            (-19, -13),
        ];
        let depth = match index {
            7 | 9 => 14,
            1 | 2 | 5 | 6 => 10,
            3 | 4 | 8 => 6,
            _ => 8,
        };
        for (i, place) in plan.places[..4].iter().enumerate() {
            for &(dx, dy) in &LOTS[..depth - i] {
                let (x, y) = (place.at.0 + dx, place.at.1 + dy);
                let v = variation(seed, x, y);
                let spaced = self.props.iter().all(|prop| distance((x, y), prop.at) >= 5);
                let on_land = (-2..=1).all(|dy| {
                    (-1..=1).all(|dx| {
                        let at = (x + dx, y + dy);
                        self.walkable(at)
                            && !self.surface(at).road()
                            && self.surface(at) != Surface::Wheat
                    })
                });
                if !spaced
                    || !on_land
                    || v % 5 == 0
                    || plan.places.iter().any(|site| distance(site.at, (x, y)) < 4)
                {
                    continue;
                }
                let art = if (region == Region::Nox && matches!(index, 2 | 9) && v % 4 == 0)
                    || (region != Region::Nox && v % 7 == 0)
                {
                    Art::Stalls
                } else {
                    Art::Cottage
                };
                let size = 31 + v % 15;
                if matches!(
                    plan.terrain,
                    Landscape::Fort | Landscape::Stone | Landscape::River | Landscape::Chalk | Landscape::Basalt
                ) {
                    self.route((x, y + 1), (x, y + 3), Surface::Paving, 1, false);
                } else {
                    self.route((x, y + 1), (x, y + 2), Surface::Trail, 1, false);
                }
                self.add_prop(
                    Prop {
                        at: (x, y),
                        art,
                        size,
                    },
                    true,
                );
            }
        }
    }

    fn add_houses_at_entrance(&mut self, entry: Position, index: usize) {
        let (x, y) = entry;
        let edge = match index {
            2 => [(x + 6, y - 5), (x + 11, y + 6)],
            4 | 5 => [(x - 8, y - 4), (x - 14, y + 6)],
            1 => [(x - 7, y + 5), (x + 7, y + 6)],
            _ => [(x - 7, y - 5), (x + 7, y - 8)],
        };
        for (i, at) in edge.into_iter().enumerate() {
            if self.walkable(at) && !self.surface(at).road() {
                self.add_prop(
                    Prop {
                        at,
                        art: Art::Cottage,
                        size: 34 + i as i32 * 4,
                    },
                    true,
                );
            }
        }
    }
}

fn address((x, y): Position) -> Option<usize> {
    ((0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y)).then(|| (y * WIDTH + x) as usize)
}

fn distance(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn variation(index: usize, x: i32, y: i32) -> i32 {
    (noise::hash(x + 313 * index as i32, y - 113 * index as i32) >> 24) as i32
}

fn base(landscape: Landscape, index: usize, x: i32, y: i32) -> Surface {
    let n = noise::sample(x + 211 * index as i32, y, 19);
    let broad = noise::sample(x, y + 311 * index as i32, 43);
    let coast = 146 + (n - 128) / 20 - (46 - (x - 143).abs()).max(0) / 5;
    let wet_x = 121 + (y - 88) / 5 + (broad - 128) / 18;
    let river_y = 151 + (broad - 128) / 15;
    let valley_y = 92 + (n - 128) / 17 + (159 - x) / 31;
    let distance = (x - 155).abs() + ((y - 96) * 4 / 3).abs();
    match landscape {
        Landscape::Coast if y >= coast + 5 => Surface::Sea,
        Landscape::Coast if y >= coast - 5 => Surface::Sand,
        Landscape::Coast if (x > 239 && n > 124) || (x < 55 && broad > 141) => Surface::Heath,
        Landscape::Marsh if (x - wet_x).abs() <= 5 || (y > 130 && (x - 179).abs() < 7) => {
            Surface::Water
        }
        Landscape::Marsh if n < 108 => Surface::Reeds,
        Landscape::Marsh if n < 153 => Surface::Mud,
        Landscape::Valley if (y - valley_y).abs() <= 3 => Surface::Water,
        Landscape::Valley if (y - valley_y).abs() <= 11 && broad > 84 => Surface::Rock,
        Landscape::Valley if (y - valley_y).abs() <= 17 => Surface::Gravel,
        Landscape::River if (y - river_y).abs() <= 4 => Surface::Water,
        Landscape::River if (y - river_y).abs() <= 14 => Surface::Sand,
        Landscape::Farm if (104..168).contains(&y) && n > 82 => {
            if (x + y / 8).rem_euclid(33) >= 25 || y.rem_euclid(24) > 20 {
                Surface::Grass
            } else {
                Surface::Wheat
            }
        }
        Landscape::Chalk if distance < 130 || n > 185 => Surface::Gravel,
        Landscape::Chalk if n > 140 => Surface::Heath,
        Landscape::Canopy if n > 95 && broad < 160 => Surface::Woods,
        Landscape::Canal if (x - wet_x).abs() <= 4 || (y - river_y).abs() <= 4 => Surface::Water,
        Landscape::Canal if distance < 120 => Surface::Gravel,
        Landscape::Basalt if ((y < 52 || x > 245) && n > 165) || n > 224 => Surface::Rock,
        Landscape::Basalt if distance < 135 || n > 120 => Surface::Heath,
        Landscape::Stone if distance < 137 || n > 191 => Surface::Gravel,
        Landscape::Fort if distance < 119 && n > 92 => Surface::Gravel,
        Landscape::River if distance < 114 && n > 114 => Surface::Gravel,
        Landscape::Heath if ((y < 52 || x > 245) && n > 151) || n > 217 => Surface::Rock,
        Landscape::Heath if n > 116 => Surface::Heath,
        Landscape::Crossing if y < 69 && broad < 123 => Surface::Heath,
        Landscape::Meadow if n > 117 && broad < 143 => Surface::Woods,
        _ if n > 130 && broad < 115 => Surface::Woods,
        _ if broad > 176 => Surface::Heath,
        _ => Surface::Grass,
    }
}

fn patch(landscape: Landscape, x: i32, y: i32) -> Surface {
    match landscape {
        Landscape::Square => Surface::Paving,
        Landscape::Yard => Surface::Trail,
        Landscape::Wood => Surface::Woods,
        Landscape::Shore => Surface::Sand,
        Landscape::Dock => Surface::Deck,
        Landscape::Farm if (x + y / 8).rem_euclid(5) < 4 => Surface::Wheat,
        Landscape::Heath | Landscape::Basalt => Surface::Heath,
        Landscape::Stone | Landscape::Chalk => Surface::Gravel,
        Landscape::Canopy => Surface::Woods,
        Landscape::Canal => Surface::Deck,
        _ => Surface::Grass,
    }
}

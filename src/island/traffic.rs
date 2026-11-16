use super::{terrain::address, Kind, Position, Settlement, Tile, HEIGHT, WIDTH};
use std::{cmp::Reverse, collections::BinaryHeap};

pub(crate) const NORTH: u8 = 1;
pub(crate) const EAST: u8 = 2;
pub(crate) const SOUTH: u8 = 4;
pub(crate) const WEST: u8 = 8;

const NEIGHBORS: [(i32, i32, u8, u8); 4] = [
    (0, -1, NORTH, SOUTH),
    (1, 0, EAST, WEST),
    (0, 1, SOUTH, NORTH),
    (-1, 0, WEST, EAST),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Vehicle {
    Train,
    Carriage,
}

pub(crate) struct Motion {
    pub(crate) vehicle: Vehicle,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) direction: u8,
}

struct Route {
    points: Vec<Position>,
    vehicle: Vehicle,
    offset_ms: u32,
}

pub(crate) struct Traffic {
    routes: Vec<Route>,
    tracks: Vec<u8>,
    elapsed_ms: u32,
}

impl Traffic {
    pub(super) fn new(tiles: &mut [Tile], sites: &[Settlement], harbor: Position) -> Self {
        let mut towns: Vec<_> = sites
            .iter()
            .filter(|site| site.landmark.kind == Kind::Town)
            .map(|site| site.door)
            .collect();
        towns.sort_by_key(|&(x, y)| (x - harbor.0).abs() + (y - harbor.1).abs());
        let mut tracks = vec![0; tiles.len()];
        let mut routes = Vec::new();
        let mut candidates = towns.iter().copied().filter_map(|town| {
            path(tiles, &tracks, harbor, town, Vehicle::Train).map(|points| (town, points))
        });
        let rail = candidates.find(|(_, points)| points.len() >= 12);
        if let Some((_, points)) = rail {
            let points = points[1..points.len() - 1].to_vec();
            for pair in points.windows(2) {
                let (x, y) = pair[0];
                let (nx, ny) = pair[1];
                let (_, _, forward, back) = NEIGHBORS
                    .iter()
                    .find(|&&(dx, dy, _, _)| (x + dx, y + dy) == (nx, ny))
                    .unwrap();
                tracks[address(x, y)] |= forward;
                tracks[address(nx, ny)] |= back;
            }
            routes.push(Route {
                points,
                vehicle: Vehicle::Train,
                offset_ms: 750,
            });
        }

        for (index, &town) in towns.iter().skip(1).take(2).enumerate() {
            if let Some(points) = path(tiles, &tracks, harbor, town, Vehicle::Carriage) {
                if points.len() < 5 {
                    continue;
                }
                let points = points[1..points.len() - 1].to_vec();
                for (index, &(x, y)) in points.iter().enumerate() {
                    let tangent = if index + 1 < points.len() {
                        points[index + 1]
                    } else {
                        points[index - 1]
                    };
                    if follows_road_shoulder(tiles, (x, y), tangent) {
                        continue;
                    }
                    let tile = &mut tiles[address(x, y)];
                    if matches!(
                        tile,
                        Tile::Meadow | Tile::Grove | Tile::Beach | Tile::Ash | Tile::Mire
                    ) {
                        *tile = Tile::Road;
                    }
                }
                let offset_ms = if index == 0 {
                    1_030
                } else {
                    points.len() as u32 * 230
                };
                routes.push(Route {
                    points,
                    vehicle: Vehicle::Carriage,
                    offset_ms,
                });
            }
        }
        Self {
            routes,
            tracks,
            elapsed_ms: 0,
        }
    }

    pub(crate) fn tracks_at(&self, x: i32, y: i32) -> u8 {
        self.tracks[address(x, y)]
    }

    pub(crate) fn advance(&mut self, elapsed_ms: u32) -> bool {
        if self.routes.is_empty() || elapsed_ms == 0 {
            return false;
        }
        self.elapsed_ms = self.elapsed_ms.wrapping_add(elapsed_ms.min(250));
        true
    }

    pub(crate) fn vehicles(&self) -> impl Iterator<Item = Motion> + '_ {
        self.routes
            .iter()
            .map(|route| route.motion(self.elapsed_ms))
    }
}

impl Route {
    fn motion(&self, clock: u32) -> Motion {
        let speed_ms = match self.vehicle {
            Vehicle::Train => 205,
            Vehicle::Carriage => 345,
        };
        let len = self.points.len();
        let span = (len - 1) as u32 * speed_ms;
        let dwell = 625;
        let half = span + dwell;
        let phase = clock.wrapping_add(self.offset_ms) % (half * 2);
        let reverse = phase >= half;
        let moving = (phase % half).min(span);
        let step = ((moving / speed_ms) as usize).min(len - 2);
        let remainder = moving - step as u32 * speed_ms;
        let (start, end) = if reverse {
            (len - 1 - step, len - 2 - step)
        } else {
            (step, step + 1)
        };
        let (x, y) = self.points[start];
        let (nx, ny) = self.points[end];
        let dx = nx - x;
        let dy = ny - y;
        let direction = match (dx, dy) {
            (1, 0) => EAST,
            (-1, 0) => WEST,
            (0, -1) => NORTH,
            _ => SOUTH,
        };
        let factor = remainder as i32 * 16 / speed_ms as i32;
        Motion {
            vehicle: self.vehicle,
            x: x * 16 + 8 + dx * factor,
            y: y * 16 + 8 + dy * factor,
            direction,
        }
    }
}

fn follows_road_shoulder(tiles: &[Tile], (x, y): Position, (nx, ny): Position) -> bool {
    let (across_x, across_y) = (ny - y, nx - x);
    let road = |x, y| {
        (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y) && tiles[address(x, y)] == Tile::Road
    };
    [-1, 1].into_iter().any(|side| {
        let (dx, dy) = (across_x * side, across_y * side);
        road(x + dx, y + dy) && road(nx + dx, ny + dy)
    })
}

fn path(
    tiles: &[Tile],
    tracks: &[u8],
    start: Position,
    end: Position,
    vehicle: Vehicle,
) -> Option<Vec<Position>> {
    let start = address(start.0, start.1);
    let end = address(end.0, end.1);
    let mut distance = vec![u32::MAX; tiles.len()];
    let mut previous = vec![usize::MAX; tiles.len()];
    let mut open = BinaryHeap::from([Reverse((0, start))]);
    distance[start] = 0;
    while let Some(Reverse((cost, index))) = open.pop() {
        if index == end {
            let mut route = vec![point(index)];
            let mut here = index;
            while here != start {
                here = previous[here];
                route.push(point(here));
            }
            route.reverse();
            return Some(route);
        }
        if cost != distance[index] {
            continue;
        }

        let (x, y) = point(index);
        for (dx, dy, _, _) in NEIGHBORS {
            let (nx, ny) = (x + dx, y + dy);
            if !(0..WIDTH).contains(&nx) || !(0..HEIGHT).contains(&ny) {
                continue;
            }
            let next = address(nx, ny);
            let tile = tiles[next];
            if !tile.walkable()
                || (next != end && matches!(tile, Tile::Settlement(_) | Tile::Landmark(_)))
            {
                continue;
            }
            let travel_cost = match (vehicle, tile) {
                (_, Tile::Bridge) | (Vehicle::Carriage, Tile::Road) => 2,
                (Vehicle::Train, Tile::Road) => 4,
                (_, Tile::Field | Tile::Mire | Tile::Ruin) => 8,
                _ => 5,
            };
            let rail_penalty = if vehicle == Vehicle::Carriage && tracks[next] != 0 {
                19
            } else {
                0
            };
            let candidate = cost + travel_cost + rail_penalty;
            if candidate < distance[next] {
                distance[next] = candidate;
                previous[next] = index;
                open.push(Reverse((candidate, next)));
            }
        }
    }
    None
}

fn point(index: usize) -> Position {
    (index as i32 % WIDTH, index as i32 / WIDTH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{island::Island, world_map::Region};

    #[test]
    fn all_islands_have_passable_routes_and_animated_vehicles() {
        for region in Region::ALL {
            let mut island = Island::new(region);
            let traffic = &island.traffic;
            assert_eq!(traffic.routes.len(), 3, "{region:?}");
            assert_eq!(traffic.routes[0].vehicle, Vehicle::Train);
            for route in &traffic.routes {
                for &(x, y) in &route.points {
                    assert!(island.tile(x, y).walkable(), "{region:?}: {x}, {y}");
                    if route.vehicle == Vehicle::Train {
                        assert_ne!(traffic.tracks_at(x, y), 0);
                    }
                }
                for pair in route.points.windows(2) {
                    assert_eq!(
                        (pair[0].0 - pair[1].0).abs() + (pair[0].1 - pair[1].1).abs(),
                        1
                    );
                }
            }
            let before: Vec<_> = island
                .traffic
                .vehicles()
                .map(|vehicle| (vehicle.x, vehicle.y))
                .collect();
            for _ in 0..10 {
                island.traffic.advance(75);
            }
            assert!(island
                .traffic
                .vehicles()
                .zip(before)
                .any(|(vehicle, (x, y))| (vehicle.x, vehicle.y) != (x, y)));
        }
    }
}

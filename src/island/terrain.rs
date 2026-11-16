use super::{Layout, Position, Tile, HEIGHT, PLAN_HEIGHT, PLAN_WIDTH, WIDTH};
use crate::world_map::Region;

pub(super) fn expand(layout: &Layout, region: Region) -> Vec<Tile> {
    let settlements = layout
        .settlements
        .each_ref()
        .map(|site| scale_door(site.door));
    let mut tiles = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let base = expanded_tile(layout, region, x, y);
            tiles.push(land_use(base, &settlements, region, x, y));
        }
    }

    for (row, cells) in layout.rows.iter().enumerate() {
        for (col, cell) in cells.iter().enumerate() {
            if cell.is_ascii_digit() {
                let (x, y) = scale_door((col as i32 * 2 + 1, row as i32 * 2 + 1));
                tiles[address(x, y)] = Tile::Landmark((cell - b'0') as usize);
            }
        }
    }
    tiles
}

fn land_use(base: Tile, settlements: &[Position], region: Region, x: i32, y: i32) -> Tile {
    match base {
        Tile::Plaza => {
            // The old civic outlines are zoning hints. Keep paved courtyards
            // close to buildings as the surrounding land expands.
            let edge = (crate::noise::sample(x - 203, y + 81, 6) - 128) / 12;
            let near_building = settlements
                .iter()
                .any(|&(cx, cy)| (cx - x).pow(2) + (cy - y).pow(2) < 49 + edge);
            if near_building {
                Tile::Plaza
            } else {
                decode(b'.', region)
            }
        }
        Tile::Field => {
            let drift = crate::noise::sample(x, y, 14) / 75;
            if (x + drift).rem_euclid(19) == 0 || (y + drift).rem_euclid(17) == 0 {
                Tile::Meadow
            } else {
                Tile::Field
            }
        }
        other => other,
    }
}

pub(super) const fn scale_door((x, y): Position) -> Position {
    (x * 6 + 3, y * 16 / 3 + 2)
}

pub(super) const fn address(x: i32, y: i32) -> usize {
    (y * WIDTH + x) as usize
}

fn coarse(layout: &Layout, col: i32, row: i32) -> u8 {
    layout
        .rows
        .get(row as usize)
        .and_then(|cells| cells.get(col as usize))
        .copied()
        .unwrap_or(b'~')
}

fn dimensions(x: i32, y: i32) -> (i32, i32) {
    (
        x * PLAN_WIDTH as i32 / WIDTH,
        y * PLAN_HEIGHT as i32 / HEIGHT,
    )
}

fn expanded_tile(layout: &Layout, region: Region, x: i32, y: i32) -> Tile {
    let (col, row) = dimensions(x, y);
    let source = coarse(layout, col, row);
    if matches!(source, b'=' | b'+') {
        return route_tile(layout, region, x, y, col, row, source);
    }

    // Coastlines and biome edges follow separate low-frequency curves. Keep
    // the authored roads and bridges intact through the displaced ground.
    let (offset_x, offset_y) = (
        (crate::noise::sample(x, y, 17) - 128) / 39,
        (crate::noise::sample(x + 613, y - 271, 13) - 128) / 39,
    );
    let (shift_col, shift_row) = dimensions((x + offset_x).max(0), (y + offset_y).max(0));
    let shifted = coarse(layout, shift_col, shift_row);
    let terrain = if matches!(shifted, b'=' | b'+' | b'0'..=b'9') || source.is_ascii_digit() {
        source
    } else {
        shifted
    };

    let spread = crate::noise::sample(x - 711, y + 179, 11);
    match decode(terrain, region) {
        Tile::Forest if spread > 187 => Tile::Grove,
        Tile::Meadow if spread > 181 && region == Region::Sylva => Tile::Grove,
        Tile::Cliff if spread > 194 => Tile::Ash,
        ground => ground,
    }
}

fn route_tile(
    layout: &Layout,
    region: Region,
    x: i32,
    y: i32,
    col: i32,
    row: i32,
    source: u8,
) -> Tile {
    let center_x = (col * 2 + 1) * WIDTH / (2 * PLAN_WIDTH as i32);
    let center_y = (row * 2 + 1) * HEIGHT / (2 * PLAN_HEIGHT as i32);
    let (dx, dy) = (x - center_x, y - center_y);
    let up = coarse(layout, col, row - 1);
    let down = coarse(layout, col, row + 1);
    let left = coarse(layout, col - 1, row);
    let right = coarse(layout, col + 1, row);
    let connected = |cell: u8| matches!(cell, b'=' | b'+');
    if in_lane(dx, dy, [up, right, down, left].map(connected)) {
        return if source == b'+' {
            Tile::Bridge
        } else {
            Tile::Road
        };
    }
    if source == b'+' {
        return match region {
            Region::Liberta => Tile::Sea,
            Region::Vulcan if [up, down, left, right].contains(&b'!') => Tile::Lava,
            _ => Tile::Water,
        };
    }
    // The verge is walkable, even where the original road cut through a forest.
    if [up, down, left, right].contains(&b'f') {
        Tile::Grove
    } else {
        decode(b'.', region)
    }
}

fn in_lane(dx: i32, dy: i32, [north, east, south, west]: [bool; 4]) -> bool {
    let column = (0..=1).contains(&dx);
    let row = (0..=1).contains(&dy);
    (column && (row || if dy < 0 { north } else { south }))
        || (row && if dx < 0 { west } else { east })
}

#[cfg(test)]
mod tests {
    use super::in_lane;

    #[test]
    fn straight_routes_and_turns_keep_two_tile_lanes() {
        let horizontal = [false, true, false, true];
        let vertical = [true, false, true, false];
        for distance in -5..=5 {
            for across in -2..=3 {
                assert_eq!(
                    in_lane(distance, across, horizontal),
                    (0..=1).contains(&across)
                );
                assert_eq!(
                    in_lane(across, distance, vertical),
                    (0..=1).contains(&across)
                );
            }
        }

        let corner = [true, true, false, false];
        assert!(in_lane(0, -2, corner));
        assert!(in_lane(1, -2, corner));
        assert!(in_lane(2, 0, corner));
        assert!(in_lane(2, 1, corner));
        assert!(!in_lane(-1, -2, corner));
        assert!(!in_lane(2, 2, corner));
        assert!(!in_lane(0, 2, corner));
    }
}

fn decode(material: u8, region: Region) -> Tile {
    match material {
        b'~' => Tile::Sea,
        b':' => Tile::Beach,
        b'#' => Tile::Cliff,
        b'f' => Tile::Forest,
        b'g' => Tile::Grove,
        b'r' => Tile::Mire,
        b'w' => Tile::Water,
        b'a' => Tile::Ash,
        b'!' => Tile::Lava,
        b'u' => Tile::Ruin,
        b'p' => Tile::Plaza,
        b'o' => Tile::Field,
        _ if region == Region::Liberta => Tile::Beach,
        _ => Tile::Meadow,
    }
}

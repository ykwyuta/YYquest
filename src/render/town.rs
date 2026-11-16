use super::{
    allen,
    guide::{Area, Guidance},
    island::{COLUMNS, LEFT, ROWS, TILE_SIZE, TOP, VIEW},
    map_art::{self as art, Ground},
    Frame, GOLD, INK, IVORY, LINE, MINT, MIST,
};
use crate::{
    island::Position,
    noise,
    town::{Action, Followup, Panel, State, Surface, Town, HEIGHT, WIDTH},
    world_map::Region,
};

impl Frame {
    pub(super) fn draw_town(&mut self, town: &Town, states: &[State; 5]) {
        let plan = town.plan();
        let camera = (
            (town.pos.0 - COLUMNS / 2).clamp(0, WIDTH - COLUMNS),
            (town.pos.1 - ROWS / 2).clamp(0, HEIGHT - ROWS),
        );
        let point = |(x, y): Position| {
            (
                LEFT + (x - camera.0) * TILE_SIZE,
                TOP + (y - camera.1) * TILE_SIZE,
            )
        };
        self.rect(
            LEFT - 1,
            TOP - 1,
            COLUMNS * TILE_SIZE + 2,
            ROWS * TILE_SIZE + 2,
            LINE,
        );
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let at = (camera.0 + col, camera.1 + row);
                self.terrain(
                    point(at),
                    at,
                    art::around(at.0, at.1, |x, y| {
                        ground(town.landscape.surface((x, y)), x, y)
                    }),
                    TILE_SIZE,
                );
            }
        }

        // Low roadside foliage is drawn before buildings and people.
        for row in -1..=ROWS + 2 {
            for col in -2..=COLUMNS + 2 {
                let at = (camera.0 + col, camera.1 + row);
                let site = town.landscape.surface(at);
                let v = noise::hash(at.0, at.1 + 53 * town.index as i32);
                let (sprite, side) = match site {
                    Surface::Woods if v % 11 < 2 => (
                        art::tree(Region::Nox, (v >> 16) as usize),
                        17 + (v % 15) as i32,
                    ),
                    Surface::Reeds if v % 5 < 2 => (art::REEDS, 11 + (v % 7) as i32),
                    Surface::Rock if v % 13 == 0 => (art::MOUNTAIN, 15 + (v % 6) as i32),
                    Surface::Heath if v % 19 == 0 => (art::HEATHER_BUSH, 13),
                    Surface::Sand if v % 29 == 0 => (art::DUNE_GRASS, 12),
                    Surface::Grass if v % 41 == 0 => (art::WILDFLOWERS, 9 + (v % 5) as i32),
                    Surface::Grass if v % 117 == 0 => (art::ASH_TREE, 18 + (v % 6) as i32),
                    Surface::Gravel if v % 33 == 0 => (art::FERNS, 9),
                    _ => continue,
                };
                if town
                    .landscape
                    .props
                    .iter()
                    .any(|prop| (prop.at.0 - at.0).abs() <= 2 && (prop.at.1 - at.1).abs() <= 3)
                {
                    continue;
                }
                let (x, y) = point(at);
                self.art_clipped(sprite, x + 6 - side / 2, y + 9 - side, side as usize, VIEW);
            }
        }

        let player_and_npcs = |frame: &mut Frame, row: i32| {
            for (index, place) in plan
                .places
                .iter()
                .enumerate()
                .filter(|(_, place)| place.at.1 == row)
            {
                let (x, y) = point(place.at);
                if x < LEFT + 2 || x > VIEW[2] - 14 || y < TOP + 10 || y > VIEW[3] - 12 {
                    continue;
                }
                let has_story = town.local_stories(index, states).next().is_some();
                if place.art.is_some() || has_story {
                    let worker = (index + town.index) % 4;
                    frame.art_clipped(art::town_sprite(12 + worker), x + 1, y - 7, 18, VIEW);
                }
                let color =
                    if plan.stories[town.tracked].objective(states[town.tracked]) == Some(index) {
                        GOLD
                    } else if has_story {
                        MINT
                    } else {
                        MIST
                    };
                let marker = if has_story { 1 } else { 3 };
                frame.rect(x + 8, y - 10, 3, 6 - marker, INK);
                frame.rect(x + 9, y - 9, 1, 5 - marker, color);
                if has_story {
                    frame.rect(x + 9, y - 3, 1, 1, color);
                }
            }
            if town.pos.1 == row {
                let (x, y) = point(town.pos);
                frame.rect(x + 2, y + 10, 9, 2, INK);
                frame.art_clipped(allen(town.facing), x - 1, y - 2, 14, VIEW);
            }
        };
        let mut props = town.landscape.props.iter().peekable();
        for row in (camera.1 - 2)..=(camera.1 + ROWS + 6) {
            while let Some(prop) = props.peek().filter(|prop| prop.at.1 <= row) {
                let (x, y) = point(prop.at);
                if prop.at.1 >= camera.1 - 2 {
                    self.art_clipped(
                        art::town_sprite(prop.art as usize),
                        x + 6 - prop.size / 2,
                        y + if matches!(prop.art, crate::town::Art::Bridge) {
                            26
                        } else {
                            9
                        } - prop.size,
                        prop.size as usize,
                        VIEW,
                    );
                }
                props.next();
            }
            if (camera.1..camera.1 + ROWS).contains(&row) {
                player_and_npcs(self, row);
            }
        }
        let (x, y) = point((plan.entry.0 + 2, plan.entry.1 - 2));
        if (LEFT + 2..VIEW[2] - 12).contains(&x) && (TOP + 7..VIEW[3] - 8).contains(&y) {
            self.rect(x + 4, y + 1, 3, 11, 0x44322e);
            self.rect(x + 5, y + 1, 1, 11, 0xa27c55);
            self.rect(x - 2, y - 3, 14, 7, 0x382b2a);
            self.rect(x - 1, y - 2, 12, 5, 0x916a48);
            self.rect(x + 1, y, 8, 1, GOLD);
        }

        let destination = town.guide_destination(states);
        let remaining = if let (Panel::Explore, Some(place)) = (town.panel, destination) {
            self.draw_guide(
                Guidance {
                    area: Area::Town {
                        index: town.index,
                        low_tide: town.low_tide,
                        outcome: states[0],
                    },
                    from: town.pos,
                    to: place.at,
                    size: (WIDTH, HEIGHT),
                },
                |at| town.landscape.walkable(at),
                |at| town.landscape.surface(at).road(),
                |at| {
                    let (x, y) = point(at);
                    (x + 6, y + 8)
                },
                VIEW,
            )
        } else {
            None
        };

        self.label(16, 6, 12, GOLD, plan.name);
        self.label(204, 8, 9, MIST, "J 手記　M 島へ");
        let completed = states
            .iter()
            .filter(|state| matches!(state, State::Done(_)))
            .count();
        self.panel(15, 191, 290, 46);
        self.label(248, 224, 9, MINT, format!("解決 {completed}/5"));

        match town.panel {
            Panel::Explore => {
                if town.at_exit() {
                    self.label(24, 196, 11, GOLD, "島街道への出口");
                    self.label(24, 210, 10, IVORY, "決定で島フィールドに戻る");
                } else if let Some(index) = town.nearby() {
                    self.label(24, 196, 11, GOLD, plan.places[index].name);
                    self.label(24, 210, 10, IVORY, "決定で話す・調べる");
                } else {
                    let area = match town.landscape.surface(town.pos) {
                        Surface::Deck => "板道",
                        Surface::Paving => "石畳の通り",
                        Surface::Wheat => "麦畑",
                        Surface::Reeds | Surface::Mud => "葦の湿地",
                        Surface::Sand => "浜辺",
                        Surface::Woods => "林",
                        Surface::Heath | Surface::Gravel => "荒れ地",
                        _ => "街はずれ",
                    };
                    self.label(24, 196, 11, GOLD, area);
                    self.label(24, 210, 10, IVORY, "街の施設や住民を訪ねよう");
                }
                if let Some(place) = destination {
                    let mut route = format!("精霊　{}へ", place.name);
                    if let Some(steps) = remaining {
                        if steps == 0 {
                            route.push_str("　到着");
                        } else {
                            route.push_str(&format!("　あと{steps}歩"));
                        }
                    }
                    self.label(24, 223, 9, MINT, route);
                } else {
                    self.label(24, 223, 9, MIST, "この街の依頼をすべて解決した");
                }
            }
            Panel::Journal => self.draw_town_journal(town, states),
            Panel::Local { place, row } => {
                self.panel(29, 40, 262, 147);
                self.label(41, 49, 12, GOLD, plan.places[place].name);
                for (i, text) in town
                    .local_actions(place, states)
                    .map(|action| match action {
                        Action::Story(index) => plan.stories[index].title,
                        Action::Look => "この場所について",
                        Action::Profile => "街の取り決め",
                        Action::WaitForTide => "潮が変わるのを待つ",
                        Action::Sail => "出港して全体図へ",
                    })
                    .enumerate()
                {
                    self.label(
                        42,
                        70 + 14 * i as i32,
                        11,
                        if i == row { MINT } else { IVORY },
                        format!("{}　{text}", if i == row { "▶" } else { "　" }),
                    );
                }
                self.label(24, 211, 9, MIST, "上下  選ぶ　　決定  読む　　M  閉じる");
            }
            Panel::Talk { title, text, next } => {
                self.panel(20, 44, 280, 143);
                self.label(33, 54, 12, GOLD, title);
                self.wrapped_town_text(33, 76, text, 23, 5);
                if let Followup::Choose(index, pick) = next {
                    let choices = plan.stories[index].choices.as_ref().unwrap();
                    for (i, choice) in choices.iter().enumerate() {
                        self.label(
                            33,
                            150 + 15 * i as i32,
                            11,
                            if i == pick { MINT } else { IVORY },
                            format!("{}　{}", if i == pick { "▶" } else { "　" }, choice.label),
                        );
                    }
                    self.label(24, 211, 9, MIST, "上下  選ぶ　　決定  選択");
                } else {
                    self.label(24, 211, 9, MIST, "決定  進む　　M  閉じる");
                }
            }
        }
    }

    fn draw_town_journal(&mut self, town: &Town, states: &[State; 5]) {
        let plan = town.plan();
        self.panel(18, 27, 284, 162);
        self.label(29, 34, 11, GOLD, "街の手記");
        self.label(172, 35, 9, MIST, "× 出口　◆ 現在地");
        let tracked = plan.stories[town.tracked].objective(states[town.tracked]);
        const MAP_W: i32 = 114;
        const MAP_H: i32 = 70;
        let map_at = |(x, y): Position| (173 + x * MAP_W / WIDTH, 51 + y * MAP_H / HEIGHT);
        for sy in 0..MAP_H {
            for sx in 0..MAP_W {
                let left = sx * WIDTH / MAP_W;
                let top = sy * HEIGHT / MAP_H;
                let right = (sx + 1) * WIDTH / MAP_W;
                let bottom = (sy + 1) * HEIGHT / MAP_H;
                let road = (top..bottom)
                    .any(|y| (left..right).any(|x| town.landscape.surface((x, y)).road()));
                let surface = if road {
                    Surface::Trail
                } else {
                    town.landscape
                        .surface(((left + right) / 2, (top + bottom) / 2))
                };
                let color = match surface {
                    Surface::Sea | Surface::Water => 0x304d65,
                    Surface::Paving | Surface::Trail | Surface::Deck => 0xb39774,
                    Surface::Grass | Surface::Woods => 0x485946,
                    Surface::Rock => 0x493746,
                    Surface::Wheat => 0x978253,
                    _ => 0x60554f,
                };
                self.rect(173 + sx, 51 + sy, 1, 1, color);
            }
        }
        for (i, place) in plan.places.iter().enumerate() {
            let (x, y) = map_at(place.at);
            self.rect(
                x - 1,
                y - 1,
                3,
                3,
                if tracked == Some(i) { GOLD } else { MIST },
            );
        }
        let (x, y) = map_at(plan.entry);
        self.rect(x - 2, y, 5, 1, INK);
        self.rect(x, y - 2, 1, 5, INK);
        let (x, y) = map_at(town.pos);
        self.rect(x - 1, y - 1, 3, 3, MINT);
        for (index, (story, state)) in plan.stories.iter().zip(states).enumerate() {
            let active = if index == town.tracked { MINT } else { IVORY };
            let mark = match state {
                State::New => "・",
                State::Active(_) => "▶",
                State::Done(_) => "済",
            };
            self.label(
                29,
                52 + index as i32 * 14,
                9,
                active,
                format!("{mark} {}", story.title),
            );
        }
        let message = plan.stories[town.tracked]
            .outcome(states[town.tracked])
            .map(str::to_owned)
            .or_else(|| {
                tracked.map(|i| {
                    let prefix = if states[town.tracked] == State::New {
                        "始まり"
                    } else {
                        "次に訪ねる場所"
                    };
                    format!("{prefix}　{}", plan.places[i].name)
                })
            })
            .unwrap_or_else(|| plan.profile.to_owned());
        self.wrapped_town_text(29, 128, &message, 23, 4);
        self.label(24, 208, 9, MIST, "上下  話を選ぶ　　決定 / J  閉じる");
    }

    fn wrapped_town_text(&mut self, x: i32, y: i32, text: &str, width: usize, lines: usize) {
        let chars: Vec<_> = text.chars().collect();
        for (i, row) in chars.chunks(width).take(lines).enumerate() {
            self.label(
                x,
                y + 14 * i as i32,
                11,
                IVORY,
                row.iter().collect::<String>(),
            );
        }
    }
}

fn ground(tile: Surface, x: i32, y: i32) -> Ground {
    let patch = art::patch(x, y);
    match tile {
        Surface::Grass if patch > 124 => Ground::DryMeadow,
        Surface::Grass => Ground::Grass,
        Surface::Woods if patch < 128 => Ground::LeafMoss,
        Surface::Woods => Ground::Woodland,
        Surface::Heath => Ground::Heather,
        Surface::Mud => Ground::MudRoad,
        Surface::Reeds => Ground::Marsh,
        Surface::Wheat => Ground::Crops,
        Surface::Gravel => Ground::Gravel,
        Surface::Rock => Ground::Rubble,
        Surface::Sand if patch < 103 => Ground::WetSand,
        Surface::Sand => Ground::Sand,
        Surface::Sea => Ground::Sea,
        Surface::Water => Ground::River,
        Surface::Trail if patch < 119 => Ground::Rutted,
        Surface::Trail => Ground::Trail,
        Surface::Paving if patch > 154 => Ground::MossCobble,
        Surface::Paving => Ground::Stone,
        Surface::Deck => Ground::Dock,
    }
}

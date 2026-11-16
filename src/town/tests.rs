use super::*;
use crate::{game::Game, island::Tile};
use std::collections::VecDeque;

fn entered_region_town(region: Region, index: usize) -> Game {
    let mut game = Game::new();
    game.input(Key::Map);
    game.world_map.selected = region;
    game.input(Key::Confirm);
    let island = game.world_map.island.as_mut().unwrap();
    (island.x, island.y) = island.settlements()[index].door;
    assert_eq!(
        island.current_landmark().unwrap().name,
        plan(region, index).name
    );
    game.input(Key::Confirm);
    let town = game.world_map.town.as_ref().unwrap();
    assert_eq!((town.region, town.index), (region, index));
    game
}

fn entered_town(index: usize) -> Game {
    entered_region_town(Region::Nox, index)
}

#[test]
fn every_town_place_is_reachable_from_the_island_entrance() {
    for region in Region::ALL {
        for index in 0..COUNT {
            let plan = plan(region, index);
            let mut game = entered_region_town(region, index);
            let town = game.world_map.town.as_mut().unwrap();
            assert_eq!((WIDTH, HEIGHT), (312, 192));
            assert!(
                town.landscape.walkable(town.pos),
                "{region:?} / {}",
                plan.name
            );

            let mut seen = vec![false; (WIDTH * HEIGHT) as usize];
            let mut pending = VecDeque::from([town.pos]);
            let address = |(x, y): Position| (y * WIDTH + x) as usize;
            seen[address(town.pos)] = true;
            while let Some((x, y)) = pending.pop_front() {
                for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if town.landscape.walkable(next) && !seen[address(next)] {
                        seen[address(next)] = true;
                        pending.push_back(next);
                    }
                }
            }
            for place in &plan.places {
                assert!(
                    seen[address(place.at)],
                    "{region:?} / {} / {}",
                    plan.name,
                    place.name
                );
            }
            for spot in town.gathering_spots() {
                assert!(
                    seen[address(spot.at)],
                    "{region:?} / {} / {}",
                    plan.name,
                    spot.item.name()
                );
                assert!(plan
                    .places
                    .iter()
                    .all(|place| distance(spot.at, place.at) > 3));
            }
        }
    }
}

fn read_beat(
    game: &mut Game,
    region: Region,
    index: usize,
    story: usize,
    beat: &Beat,
    prefer_second: bool,
) {
    let states = game.world_map.region_progress(region)[index];
    let town = game.world_map.town.as_mut().unwrap();
    // The independent reachability test checks the route to each of these
    // locations. This test drives every conversation through its normal inputs.
    town.pos = town.plan().places[beat.place].at;
    let row = town
        .local_stories(beat.place, &states)
        .position(|id| id == story)
        .unwrap();
    game.input(Key::Confirm);
    for _ in 0..row {
        game.input(Key::Down);
    }
    game.input(Key::Confirm);
    let panel = game.world_map.town.as_ref().unwrap().panel;
    assert!(matches!(panel, Panel::Talk { text, delivery, .. }
        if text == beat.text && delivery == beat.delivery));
    if prefer_second
        && matches!(
            panel,
            Panel::Talk {
                next: Followup::Choose(_, _),
                ..
            }
        )
    {
        game.input(Key::Down);
    }
    game.input(Key::Confirm);
    if let Panel::Talk {
        delivery,
        next: Followup::Close,
        ..
    } = game.world_map.town.as_ref().unwrap().panel
    {
        assert_eq!(delivery, Delivery::Scene);
        game.input(Key::Confirm);
    }
}

#[test]
fn all_two_hundred_fifty_stories_and_both_choices_can_be_completed() {
    for alternate in [false, true] {
        for region in Region::ALL {
            for index in 0..COUNT {
                let plan = plan(region, index);
                let mut game = entered_region_town(region, index);
                for (story_index, story) in plan.stories.iter().enumerate() {
                    for beat in story.beats {
                        read_beat(&mut game, region, index, story_index, beat, alternate);
                    }
                    let result = usize::from(alternate && story.choices.is_some());
                    assert_eq!(
                        game.world_map.region_progress(region)[index][story_index],
                        State::Done(result),
                        "{region:?} / {} / {}",
                        plan.name,
                        story.title
                    );
                }
                game.input(Key::Map);
                let island = game.world_map.island.as_ref().unwrap();
                assert!(matches!(island.tile(island.x, island.y), Tile::Settlement(id) if id == index));
                game.input(Key::Map);
                assert!(game.world_map.island.is_none());
                game.input(Key::Confirm);
                let island = game.world_map.island.as_mut().unwrap();
                (island.x, island.y) = island.settlements()[index].door;
                game.input(Key::Confirm);
                assert!(game.world_map.town.is_some());
                assert!(game.world_map.region_progress(region)[index]
                    .iter()
                    .all(|state| matches!(state, State::Done(_))));
            }
        }
    }
}

#[test]
fn harbor_dock_offers_departure_without_hiding_its_story() {
    let mut game = entered_town(1);
    let plan = &NOX[1];
    let story = &plan.stories[4];
    read_beat(&mut game, Region::Nox, 1, 4, &story.beats[0], false);
    read_beat(&mut game, Region::Nox, 1, 4, &story.beats[1], false);
    assert_eq!(game.world_map.town_progress[1][4], State::Active(2));
    game.input(Key::Confirm);
    game.input(Key::Up);
    game.input(Key::Confirm);
    assert!(game.world_map.island.is_none());
    assert!(game.world_map.town.is_none());
}

#[test]
fn waiting_at_the_harbor_reveals_and_submerges_the_tidal_path() {
    let mut game = entered_town(1);
    let place = NOX[1].places[1].at;
    let tidal_flat = (100, 155);
    game.world_map.town.as_mut().unwrap().pos = place;
    for passable in [true, false] {
        game.input(Key::Confirm);
        game.input(Key::Up);
        game.input(Key::Confirm);
        assert_eq!(
            game.world_map
                .town
                .as_ref()
                .unwrap()
                .landscape
                .walkable(tidal_flat),
            passable
        );
        game.input(Key::Confirm);
    }
}

#[test]
fn finishing_the_marsh_story_opens_the_new_boardwalk() {
    let mut game = entered_town(3);
    let point = (180, 137);
    assert!(!game
        .world_map
        .town
        .as_ref()
        .unwrap()
        .landscape
        .walkable(point));
    for beat in NOX[3].stories[0].beats {
        read_beat(&mut game, Region::Nox, 3, 0, beat, false);
    }
    assert!(game
        .world_map
        .town
        .as_ref()
        .unwrap()
        .landscape
        .walkable(point));
    game.input(Key::Map);
    game.input(Key::Confirm);
    assert!(game
        .world_map
        .town
        .as_ref()
        .unwrap()
        .landscape
        .walkable(point));
}

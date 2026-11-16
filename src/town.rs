mod celestia;
mod gathering;
mod liberta;
mod map;
mod nox;
mod sylva;
#[cfg(test)]
mod tests;
mod vulcan;

use crate::{
    campaign::ChapterBeat,
    dialogue::{Delivery, Expression, Speaker},
    economy::Service,
    walk::Walk,
    world_map::Region,
};
use crate::{
    game::{Facing, Key},
    island::Position,
};
pub(crate) use gathering::Gathering;
pub(crate) use map::{Landscape, Surface, HEIGHT, WIDTH};
use nox::NOX;

#[derive(Clone, Copy)]
pub(crate) enum Art {
    Hearth,
    Lighthouse,
    Palisade,
    Sluice,
    Mill,
    Well,
    Inn,
    StoneGate,
    Bridge,
    Clock,
    Stalls,
    Cottage,
    Landmark,
}

impl Art {
    pub(crate) fn size(self) -> i32 {
        match self {
            Self::Landmark => 64,
            Self::StoneGate | Self::Palisade | Self::Inn => 62,
            Self::Cottage | Self::Well | Self::Stalls => 37,
            _ => 49,
        }
    }
}

pub(crate) struct Place {
    pub(crate) name: &'static str,
    pub(crate) at: Position,
    pub(crate) art: Option<Art>,
    ground: Landscape,
    detail: &'static str,
}

pub(crate) struct Beat {
    place: usize,
    text: &'static str,
    delivery: Delivery,
}

impl Beat {
    const fn speech(
        place: usize,
        name: &'static str,
        speaker: Speaker,
        text: &'static str,
    ) -> Self {
        Self {
            place,
            text,
            delivery: Delivery::spoken(name, speaker, Expression::Speaking),
        }
    }

    const fn voice(place: usize, name: &'static str, text: &'static str) -> Self {
        Self {
            place,
            text,
            delivery: Delivery::Speech { name, face: None },
        }
    }

    const fn scene(place: usize, text: &'static str) -> Self {
        Self {
            place,
            text,
            delivery: Delivery::Scene,
        }
    }

    const fn record(place: usize, source: &'static str, text: &'static str) -> Self {
        Self {
            place,
            text,
            delivery: Delivery::Record(source),
        }
    }
}

pub(crate) struct Ending {
    pub(crate) label: &'static str,
    pub(crate) text: &'static str,
}

pub(crate) struct Story {
    pub(crate) title: &'static str,
    beats: &'static [Beat],
    pub(crate) choices: Option<[Ending; 2]>,
}

pub(crate) struct Plan {
    pub(crate) name: &'static str,
    pub(crate) profile: &'static str,
    terrain: Landscape,
    pub(crate) entry: Position,
    pub(crate) places: [Place; 8],
    pub(crate) stories: [Story; 5],
}

pub(crate) const COUNT: usize = NOX.len();

pub(crate) fn plan(region: Region, index: usize) -> &'static Plan {
    match region {
        Region::Nox => &NOX[index],
        Region::Celestia => &celestia::CELESTIA[index],
        Region::Sylva => &sylva::SYLVA[index],
        Region::Liberta => &liberta::LIBERTA[index],
        Region::Vulcan => &vulcan::VULCAN[index],
    }
}
pub(crate) type Progress = [[State; 5]; COUNT];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) enum State {
    #[default]
    New,
    Active(usize),
    Done(usize),
}

impl Story {
    pub(crate) fn beat_count(&self) -> usize {
        self.beats.len()
    }
    pub(crate) fn objective(&self, state: State) -> Option<usize> {
        let beat = match state {
            State::New => 0,
            State::Active(beat) => beat,
            State::Done(_) => return None,
        };
        Some(self.beats[beat].place)
    }

    pub(crate) fn outcome(&self, state: State) -> Option<&'static str> {
        let State::Done(result) = state else {
            return None;
        };
        Some(self.choices.as_ref().map_or_else(
            || self.beats.last().unwrap().text,
            |choices| choices[result].text,
        ))
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Panel {
    Explore,
    Journal,
    Local {
        place: usize,
        row: usize,
    },
    Talk {
        title: &'static str,
        text: &'static str,
        delivery: Delivery,
        next: Followup,
    },
}

#[derive(Clone, Copy)]
pub(crate) enum Followup {
    Close,
    Beat(usize),
    Main,
    Choose(usize, usize),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Travel {
    Stay,
    AdvanceMain,
    Island,
    Sea,
    Service(Service),
}

#[derive(Clone, Copy)]
pub(crate) enum Action {
    Story(usize),
    Main,
    Look,
    Profile,
    WaitForTide,
    Sail,
    Service(Service),
}

pub(crate) struct Town {
    pub(crate) region: Region,
    pub(crate) index: usize,
    pub(crate) pos: Position,
    pub(crate) facing: Facing,
    pub(crate) landscape: map::TownMap,
    pub(crate) panel: Panel,
    pub(crate) tracked: usize,
    pub(crate) low_tide: bool,
    pub(crate) main: Option<&'static ChapterBeat>,
    pub(crate) walk: Walk,
    pub(crate) gathering: Option<Gathering>,
    track_side: bool,
}

impl Town {
    pub(crate) fn tracking_side(&self) -> bool {
        self.track_side
    }

    pub(crate) fn restore_exploration(&mut self, tracked: usize, side: bool, low_tide: bool) {
        self.tracked = tracked;
        self.track_side = side;
        self.low_tide = low_tide && self.region == Region::Nox && self.index == 1;
        if self.low_tide {
            self.landscape.set_low_tide(true);
        }
    }

    pub(crate) fn new(region: Region, index: usize, states: &[State; 5]) -> Self {
        let plan = plan(region, index);
        let mut landscape = map::TownMap::new(plan, region, index);
        landscape.apply_outcome(region, index, states[0]);
        let tracked = states
            .iter()
            .position(|state| matches!(state, State::Active(_)))
            .or_else(|| states.iter().position(|state| *state == State::New))
            .unwrap_or(0);
        Self {
            region,
            index,
            pos: plan.entry,
            facing: Facing::South,
            landscape,
            panel: Panel::Explore,
            tracked,
            low_tide: false,
            main: None,
            walk: Walk::default(),
            gathering: None,
            track_side: false,
        }
    }

    pub(crate) fn plan(&self) -> &'static Plan {
        plan(self.region, self.index)
    }

    pub(crate) fn main_here(&self) -> Option<&'static ChapterBeat> {
        self.main
            .filter(|beat| beat.site == self.index && beat.place.is_some())
    }

    pub(crate) fn guide_destination(
        &self,
        states: &[State; 5],
    ) -> Option<(Position, &'static str)> {
        let plan = self.plan();
        if !self.track_side && self.main.is_some() {
            let place = self
                .main_here()
                .and_then(|beat| beat.place)
                .map(|index| &plan.places[index]);
            return Some(place.map_or((plan.entry, "島街道への出口"), |place| {
                (place.at, place.name)
            }));
        }
        let selected = plan.stories[self.tracked].objective(states[self.tracked]);
        let next = || {
            let index = states
                .iter()
                .position(|state| matches!(state, State::Active(_)))
                .or_else(|| states.iter().position(|state| *state == State::New))?;
            plan.stories[index].objective(states[index])
        };
        selected.or_else(next).map(|index| {
            let place = &plan.places[index];
            (place.at, place.name)
        })
    }

    pub(crate) fn nearby(&self) -> Option<usize> {
        self.plan()
            .places
            .iter()
            .enumerate()
            .filter_map(|(index, place)| {
                let distance = distance(self.pos, place.at);
                (distance <= 2).then_some((distance, index))
            })
            .min()
            .map(|(_, index)| index)
    }

    pub(crate) fn at_exit(&self) -> bool {
        distance(self.pos, self.plan().entry) <= 2
    }

    pub(crate) fn local_stories(
        &self,
        place: usize,
        states: &[State; 5],
    ) -> impl Iterator<Item = usize> {
        self.plan()
            .stories
            .iter()
            .zip(*states)
            .enumerate()
            .filter_map(move |(index, (story, state))| {
                (story.objective(state) == Some(place)).then_some(index)
            })
    }

    pub(crate) fn local_actions(
        &self,
        place: usize,
        states: &[State; 5],
    ) -> impl Iterator<Item = Action> {
        let can_sail = match (self.region, self.index, place) {
            (Region::Nox, 1, 7) => true,
            (Region::Celestia | Region::Liberta | Region::Vulcan, 0, 0) => true,
            (Region::Sylva, 1, 0) => true,
            _ => false,
        };
        self.main_here()
            .filter(|beat| beat.place == Some(place))
            .map(|_| Action::Main)
            .into_iter()
            .chain(self.local_stories(place, states).map(Action::Story))
            .chain(std::iter::once(Action::Look))
            .chain((place == 0).then_some(Action::Profile))
            .chain(Service::at(self.region, self.index, place).map(Action::Service))
            .chain(
                (self.region == Region::Nox && self.index == 1 && (place == 1 || place == 2))
                    .then_some(Action::WaitForTide),
            )
            .chain(can_sail.then_some(Action::Sail))
    }

    pub(crate) fn step(&mut self, dx: i32, dy: i32) {
        self.facing = Facing::from_step(dx, dy).unwrap_or(self.facing);
        let next = (self.pos.0 + dx, self.pos.1 + dy);
        if self.landscape.walkable(next) {
            self.walk.step(self.pos, next);
            self.pos = next;
        }
    }

    pub(crate) fn input(&mut self, key: Key, states: &mut [State; 5]) -> Travel {
        if matches!(key, Key::Back | Key::Map) {
            if matches!(self.panel, Panel::Explore) {
                return Travel::Island;
            }
            self.panel = Panel::Explore;
            return Travel::Stay;
        }
        match self.panel {
            Panel::Explore => match key {
                Key::Journal => {
                    self.track_side = false;
                    self.panel = Panel::Journal;
                }
                Key::Confirm if self.at_exit() => return Travel::Island,
                Key::Confirm => {
                    if let Some(place) = self.nearby() {
                        self.panel = Panel::Local { place, row: 0 };
                    }
                }
                _ => {
                    if let Some((dx, dy)) = key.direction() {
                        self.step(dx, dy);
                    }
                }
            },
            Panel::Journal => match key {
                Key::Up | Key::Left => {
                    self.tracked = (self.tracked + 4) % 5;
                    self.track_side = true;
                }
                Key::Down | Key::Right => {
                    self.tracked = (self.tracked + 1) % 5;
                    self.track_side = true;
                }
                Key::Confirm | Key::Journal => self.panel = Panel::Explore,
                _ => {}
            },
            Panel::Local { place, row } => match key {
                Key::Up | Key::Down => {
                    let count = self.local_actions(place, states).count();
                    let row = if key == Key::Up {
                        (row + count - 1) % count
                    } else {
                        (row + 1) % count
                    };
                    self.panel = Panel::Local { place, row };
                }
                Key::Confirm => {
                    let Some(action) = self.local_actions(place, states).nth(row) else {
                        return Travel::Stay;
                    };
                    if matches!(action, Action::Main) {
                        let beat = self.main_here().unwrap();
                        self.panel = Panel::Talk {
                            title: beat.title,
                            text: beat.text,
                            delivery: beat.delivery,
                            next: Followup::Main,
                        };
                    } else if let Action::Story(index) = action {
                        self.track_side = true;
                        let beat = match states[index] {
                            State::New => 0,
                            State::Active(beat) => beat,
                            State::Done(_) => unreachable!(),
                        };
                        self.open_story(index, beat);
                    } else {
                        let site = &self.plan().places[place];
                        let (title, text) = match action {
                            Action::Sail => return Travel::Sea,
                            Action::Service(service) => return Travel::Service(service),
                            Action::Profile => (self.plan().name, self.plan().profile),
                            Action::Look => (site.name, site.detail),
                            Action::WaitForTide => {
                                self.low_tide = !self.low_tide;
                                self.landscape.set_low_tide(self.low_tide);
                                if self.low_tide {
                                    ("引き潮", "浜に細い道が現れた。艇庫と荷揚げ桟橋の間を砂浜づたいに歩ける。")
                                } else {
                                    ("満ち潮", "波が浜の細道を覆った。艇庫へは陸側の道を回ろう。")
                                }
                            }
                            Action::Story(_) | Action::Main => unreachable!(),
                        };
                        self.panel = Panel::Talk {
                            title,
                            text,
                            delivery: Delivery::Scene,
                            next: Followup::Close,
                        };
                    }
                }
                Key::Journal => self.panel = Panel::Journal,
                _ => {}
            },
            Panel::Talk {
                title,
                text,
                delivery,
                next,
            } => match (next, key) {
                (Followup::Choose(index, pick), Key::Up | Key::Down | Key::Left | Key::Right) => {
                    self.panel = Panel::Talk {
                        title,
                        text,
                        delivery,
                        next: Followup::Choose(index, 1 - pick),
                    };
                }
                (Followup::Choose(index, pick), Key::Confirm) => {
                    self.complete(states, index, pick);
                    let text = self.plan().stories[index].outcome(states[index]).unwrap();
                    self.panel = Panel::Talk {
                        title,
                        text,
                        delivery: Delivery::Scene,
                        next: Followup::Close,
                    };
                }
                (Followup::Beat(index), Key::Confirm) => {
                    let step = match states[index] {
                        State::New => 1,
                        State::Active(beat) => beat + 1,
                        State::Done(_) => unreachable!(),
                    };
                    if step == self.plan().stories[index].beats.len() {
                        self.complete(states, index, 0);
                    } else {
                        states[index] = State::Active(step);
                        self.tracked = index;
                    }
                    self.panel = Panel::Explore;
                }
                (Followup::Main, Key::Confirm) => {
                    self.panel = Panel::Explore;
                    self.track_side = false;
                    return Travel::AdvanceMain;
                }
                (Followup::Close, Key::Confirm) => self.panel = Panel::Explore,
                _ => {}
            },
        }
        Travel::Stay
    }

    pub(crate) fn preview_story(&mut self, states: &mut [State; 5], index: usize, beat: usize) {
        let place = self.plan().stories[index].beats[beat].place;
        self.pos = self.plan().places[place].at;
        self.tracked = index;
        if beat > 0 {
            states[index] = State::Active(beat);
        }
        self.open_story(index, beat);
    }

    fn open_story(&mut self, index: usize, beat: usize) {
        let story = &self.plan().stories[index];
        let next = if beat + 1 == story.beats.len() && story.choices.is_some() {
            Followup::Choose(index, 0)
        } else {
            Followup::Beat(index)
        };
        self.panel = Panel::Talk {
            title: story.title,
            text: story.beats[beat].text,
            delivery: story.beats[beat].delivery,
            next,
        };
    }

    fn complete(&mut self, states: &mut [State; 5], index: usize, result: usize) {
        states[index] = State::Done(result);
        self.tracked = index;
        if index == 0 {
            self.landscape
                .apply_outcome(self.region, self.index, states[index]);
        }
    }
}

fn distance(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

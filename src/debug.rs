use crate::{game::Key, island::Island, town, world_map::Region};

mod dialogue;
pub(crate) use dialogue::Reader;

pub(crate) const TITLE_CODE: [u32; 8] = [38, 38, 40, 40, 37, 39, 37, 39];

pub(crate) const PROLOGUE: [&str; 8] = [
    "オープニング",
    "村の炉で回復",
    "水門を守るリリスと出会う",
    "石碑でリリスと合流",
    "霧のスライム戦",
    "洞窟コウモリ戦",
    "忘却の獣戦",
    "リリスと同行を決める",
];

#[derive(Clone, Copy, Default)]
enum Page {
    #[default]
    Root,
    Islands,
    Sites(Region),
    StoryTowns,
    Stories(usize),
    Beats(usize, usize),
    Prologue,
    Dialogue,
    DialogueRegions,
    DialogueTowns(Region),
    DialogueStories(Region, usize),
}

pub(crate) enum Target {
    Site(Region, usize),
    Story(usize, usize, usize),
    Prologue(usize),
}

pub(crate) enum Action {
    Stay,
    Title,
    Jump(Target),
}

#[derive(Default)]
pub(crate) struct Menu {
    page: Page,
    pub(crate) selected: usize,
    pub(crate) previewing: bool,
    pub(crate) reader: Option<Reader>,
}

impl Menu {
    pub(crate) fn heading(&self) -> &'static str {
        match self.page {
            Page::Root => "移動先を選ぶ",
            Page::Islands => "島を選ぶ",
            Page::Sites(region) => region.caption().0,
            Page::StoryTowns => "ノクスのサイドストーリー",
            Page::Stories(town) => town::plan(Region::Nox, town).name,
            Page::Beats(town, story) => town::plan(Region::Nox, town).stories[story].title,
            Page::Prologue => "序章のイベント",
            Page::Dialogue => "会話を通読",
            Page::DialogueRegions => "会話を読む島を選ぶ",
            Page::DialogueTowns(region) => region.caption().0,
            Page::DialogueStories(region, town) => town::plan(region, town).name,
        }
    }

    pub(crate) fn entries(&self) -> Vec<String> {
        let names: Vec<&str> = match self.page {
            Page::Root => vec![
                "町・村・廃墟",
                "ノクスのサイドストーリー",
                "序章のイベント",
                "会話を通読",
            ],
            Page::Islands | Page::DialogueRegions => Region::ALL
                .iter()
                .map(|region| region.caption().0)
                .collect(),
            Page::Sites(region) => Island::settlement_definitions(region)
                .iter()
                .map(|site| site.landmark.name)
                .collect(),
            Page::StoryTowns => (0..town::COUNT)
                .map(|index| town::plan(Region::Nox, index).name)
                .collect(),
            Page::Stories(index) => town::plan(Region::Nox, index)
                .stories
                .iter()
                .map(|story| story.title)
                .collect(),
            Page::Beats(index, story) => {
                let plan = town::plan(Region::Nox, index);
                let story = &plan.stories[story];
                return (0..story.beat_count())
                    .map(|beat| {
                        let place = story.objective(town::State::Active(beat)).unwrap();
                        let part = if beat == 0 {
                            "発端"
                        } else if beat + 1 == story.beat_count() {
                            "結末"
                        } else {
                            "途中"
                        };
                        format!("{}  {part}　{}", beat + 1, plan.places[place].name)
                    })
                    .collect();
            }
            Page::Prologue => PROLOGUE.to_vec(),
            Page::Dialogue => vec![
                "同行中の会話",
                "序章の会話",
                "本編の会話",
                "街のサイドストーリー",
            ],
            Page::DialogueTowns(region) => (0..town::COUNT)
                .map(|index| town::plan(region, index).name)
                .collect(),
            Page::DialogueStories(region, town) => std::iter::once("この街の会話を通読")
                .chain(
                    town::plan(region, town)
                        .stories
                        .iter()
                        .map(|story| story.title),
                )
                .collect(),
        };
        names.into_iter().map(str::to_owned).collect()
    }

    pub(crate) fn input(&mut self, key: Key) -> Action {
        if let Some(reader) = &mut self.reader {
            if reader.input(key) {
                self.reader = None;
            }
            return Action::Stay;
        }
        let row = self.selected;
        let next = match key {
            Key::Up => {
                let count = self.entries().len();
                self.selected = (row + count - 1) % count;
                return Action::Stay;
            }
            Key::Down => {
                self.selected = (row + 1) % self.entries().len();
                return Action::Stay;
            }
            Key::Back => match self.page {
                Page::Root => return Action::Title,
                Page::Islands => (Page::Root, 0),
                Page::StoryTowns => (Page::Root, 1),
                Page::Prologue => (Page::Root, 2),
                Page::Dialogue => (Page::Root, 3),
                Page::DialogueRegions => (Page::Dialogue, 3),
                Page::Sites(region) | Page::DialogueTowns(region) => (
                    if matches!(self.page, Page::Sites(_)) {
                        Page::Islands
                    } else {
                        Page::DialogueRegions
                    },
                    Region::ALL
                        .iter()
                        .position(|&candidate| candidate == region)
                        .unwrap(),
                ),
                Page::Stories(index) => (Page::StoryTowns, index),
                Page::Beats(index, story) => (Page::Stories(index), story),
                Page::DialogueStories(region, town) => (Page::DialogueTowns(region), town),
            },
            Key::Confirm => {
                let page = match self.page {
                    Page::Root => [
                        Page::Islands,
                        Page::StoryTowns,
                        Page::Prologue,
                        Page::Dialogue,
                    ][row],
                    Page::Islands => Page::Sites(Region::ALL[row]),
                    Page::Sites(region) => return Action::Jump(Target::Site(region, row)),
                    Page::StoryTowns => Page::Stories(row),
                    Page::Stories(index) => Page::Beats(index, row),
                    Page::Beats(index, story) => {
                        return Action::Jump(Target::Story(index, story, row))
                    }
                    Page::Prologue => return Action::Jump(Target::Prologue(row)),
                    Page::Dialogue if row == 3 => Page::DialogueRegions,
                    Page::Dialogue => {
                        self.reader =
                            Some([Reader::party, Reader::prologue, Reader::campaign][row]());
                        return Action::Stay;
                    }
                    Page::DialogueRegions => Page::DialogueTowns(Region::ALL[row]),
                    Page::DialogueTowns(region) => Page::DialogueStories(region, row),
                    Page::DialogueStories(region, town) => {
                        self.reader = Some(Reader::town(region, town, row.checked_sub(1)));
                        return Action::Stay;
                    }
                };
                (page, 0)
            }
            _ => return Action::Stay,
        };
        (self.page, self.selected) = next;
        Action::Stay
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        game::{Game, Scene},
        town::{Panel, State},
    };

    fn press(game: &mut Game, codes: &[u32]) {
        for &code in codes {
            game.input_code(code, false);
        }
    }

    #[test]
    fn title_command_requires_distinct_dpad_presses_and_confirm() {
        for wrong in [
            [38, 40, 40, 37, 39, 37, 39, 13],
            [87, 87, 83, 83, 65, 68, 65, 68],
            [38, 38, 40, 40, 37, 39, 37, 87],
        ] {
            let mut game = Game::new();
            press(&mut game, &wrong);
            press(&mut game, &[13]);
            assert!(matches!(game.scene, Scene::Field));
        }
        let mut game = Game::new();
        press(&mut game, &[38]);
        assert!(!game.input_code(38, true));
        press(&mut game, &TITLE_CODE[2..]);
        press(&mut game, &[13]);
        assert!(matches!(game.scene, Scene::Field));

        let mut game = Game::new();
        press(&mut game, &TITLE_CODE);
        assert!(matches!(game.scene, Scene::Title));
        press(&mut game, &[13]);
        assert!(matches!(game.scene, Scene::Debug));
        press(&mut game, &[27]);
        assert!(matches!(game.scene, Scene::Title));
    }

    #[test]
    fn debug_menu_opens_real_town_dialogue_at_selected_stage_and_returns() {
        let mut game = Game::new();
        press(&mut game, &TITLE_CODE);
        press(&mut game, &[13, 40, 13, 13, 13, 40, 40, 13]);
        let town = game.world_map.town.as_ref().unwrap();
        assert_eq!(town.plan().name, "灰灯りの村");
        assert_eq!(town.pos, town.plan().places[4].at);
        assert!(matches!(
            town.panel,
            Panel::Talk {
                title: "炉の名札",
                ..
            }
        ));
        assert_eq!(game.world_map.town_progress[0][0], State::Active(2));
        press(&mut game, &[40, 13]);
        assert_eq!(game.world_map.town_progress[0][0], State::Done(1));
        press(&mut game, &[27, 27, 27, 27]);
        assert!(matches!(game.scene, Scene::Debug) && !game.map_open);
        assert_eq!(game.debug_menu.selected, 2);
        assert_eq!(game.world_map.town_progress[0][0], State::New);
    }
}

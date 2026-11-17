use crate::{
    campaign::{CHAPTER, CONSENT_OPTIONS, CONSENT_RESPONSES, VOYAGE},
    dialogue::{Delivery, Message},
    game::Key,
    intro,
    party_talk::Event,
    town,
    world_map::Region,
};

pub(crate) struct Entry {
    pub section: &'static str,
    pub title: &'static str,
    pub choice: Option<&'static str>,
    pub message: Message,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        game::{Game, Scene},
        render::Frame,
    };

    #[test]
    fn dialogue_pages_stay_above_the_navigation_and_cannot_save() {
        let towns = Region::ALL.into_iter().flat_map(|region| {
            (0..town::COUNT).map(move |index| Reader::town(region, index, None))
        });
        let readers = [Reader::party(), Reader::prologue(), Reader::campaign()]
            .into_iter()
            .chain(towns);
        let mut game = Game::new();
        game.scene = Scene::Debug;
        let mut frame = Frame::new();
        for reader in readers {
            game.debug_menu.reader = Some(reader);
            while let Some(reader) = &game.debug_menu.reader {
                frame.draw(&game);
                assert!(
                    frame
                        .labels
                        .iter()
                        .rev()
                        .skip(1)
                        .all(|label| label.y + label.size <= 211),
                    "{} / {}",
                    reader.current().section,
                    reader.current().title
                );
                assert!(game.checkpoint().is_none());
                game.input(Key::Confirm);
            }
        }
    }
}

impl Entry {
    fn new(section: &'static str, title: &'static str, message: Message) -> Self {
        Self {
            section,
            title,
            choice: None,
            message,
        }
    }
}

pub(crate) struct Reader {
    pages: Vec<Entry>,
    index: usize,
}

impl Reader {
    fn new(pages: impl IntoIterator<Item = Entry>) -> Self {
        Self {
            pages: pages.into_iter().collect(),
            index: 0,
        }
    }

    pub(super) fn party() -> Self {
        Self::new(Event::ALL.into_iter().flat_map(|event| {
            event
                .lines()
                .iter()
                .map(move |&line| Entry::new("同行中の会話", event.title(), line))
        }))
    }

    pub(super) fn prologue() -> Self {
        Self::new(
            intro::script::REVIEW
                .iter()
                .map(|&(title, message)| Entry::new("序章の会話", title, message)),
        )
    }

    pub(super) fn campaign() -> Self {
        let chapter = CHAPTER.iter().map(|beat| {
            Entry::new(
                "ノクス本編",
                beat.title,
                Message::line(beat.text, beat.delivery),
            )
        });
        let voyage =
            VOYAGE.iter().flat_map(|beat| {
                let section = beat.region.caption().0;
                let opening = Entry::new(
                    section,
                    beat.title,
                    Message::line(beat.text, beat.delivery()),
                );
                let choices = CONSENT_OPTIONS.into_iter().zip(CONSENT_RESPONSES).map(
                    move |(label, message)| Entry {
                        choice: Some(label),
                        ..Entry::new(section, beat.title, message)
                    },
                );
                std::iter::once(opening).chain(choices.take(if beat.consent() { 2 } else { 0 }))
            });
        Self::new(chapter.chain(voyage))
    }

    pub(super) fn town(region: Region, index: usize, selected: Option<usize>) -> Self {
        let plan = town::plan(region, index);
        let mut pages = Vec::new();
        for (index, story) in plan.stories.iter().enumerate() {
            if selected.is_some_and(|story| story != index) {
                continue;
            }
            pages.extend(
                story.script().map(|(place, message)| {
                    Entry::new(plan.places[place].name, story.title, message)
                }),
            );
            if let Some(choices) = &story.choices {
                pages.extend(choices.iter().map(|choice| Entry {
                    choice: Some(choice.label),
                    ..Entry::new(
                        plan.name,
                        story.title,
                        Message::line(choice.text, Delivery::Scene),
                    )
                }));
            }
        }
        Self::new(pages)
    }

    pub(crate) fn current(&self) -> &Entry {
        &self.pages[self.index]
    }

    pub(crate) fn position(&self) -> (usize, usize) {
        (self.index + 1, self.pages.len())
    }

    pub(super) fn input(&mut self, key: Key) -> bool {
        let last = self.index + 1 == self.pages.len();
        match key {
            Key::Back => return true,
            Key::Confirm if last => return true,
            Key::Left | Key::Up => self.index = self.index.saturating_sub(1),
            Key::Right | Key::Down | Key::Confirm if !last => self.index += 1,
            _ => {}
        }
        false
    }
}

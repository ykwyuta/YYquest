use super::{Facing, Game, Intro, Key, Scene, MAX_HP, MEMORIES};
use crate::{
    campaign::Campaign,
    economy::Goods,
    fort::Fort,
    island::{Island, Position, Tile},
    town::{self, State, Town},
    world_map::Region,
};
use serde::{Deserialize, Serialize};

const VERSION: u8 = 1;

#[derive(Serialize, Deserialize)]
struct Save {
    version: u8,
    intro: Intro,
    position: Position,
    facing: Facing,
    companion: Option<(Position, Facing)>,
    hp: u8,
    memories: usize,
    ending: bool,
    map_open: bool,
    region: Region,
    island: Option<IslandSave>,
    town: Option<TownSave>,
    fort: Option<Fort>,
    quests: town::Progress,
    campaign: Campaign,
    goods: Goods,
}

#[derive(Serialize, Deserialize)]
struct IslandSave {
    position: Position,
    facing: Facing,
    defeated: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
struct TownSave {
    position: Position,
    facing: Facing,
    tracked: usize,
    track_side: bool,
    low_tide: bool,
}

impl Game {
    pub const MAX_SAVE_BYTES: usize = 16 * 1024;

    pub fn has_continue(&self) -> bool {
        self.continue_game.is_some()
    }

    pub fn offer_continue(&mut self, bytes: &[u8]) -> bool {
        if !matches!(self.scene, Scene::Title) || bytes.len() > Self::MAX_SAVE_BYTES {
            return false;
        }
        let Ok(save) = serde_json::from_slice::<Save>(bytes) else {
            return false;
        };
        let Some(game) = save.restore() else {
            return false;
        };
        self.continue_game = Some(Box::new(game));
        true
    }

    pub fn checkpoint(&self) -> Option<Vec<u8>> {
        if !matches!(self.scene, Scene::Field | Scene::Ending)
            || self.debug_menu.previewing
            || !matches!(self.after, super::After::None)
        {
            return None;
        }
        let world = &self.world_map;
        serde_json::to_vec(&Save {
            version: VERSION,
            intro: self.intro,
            position: (self.x, self.y),
            facing: self.facing,
            companion: self
                .companion_position
                .map(|at| (at, self.companion_facing)),
            hp: self.hp,
            memories: self.memories,
            ending: matches!(self.scene, Scene::Ending),
            map_open: self.map_open,
            region: world.selected,
            island: world.island.as_ref().map(|island| IslandSave {
                position: (island.x, island.y),
                facing: island.facing,
                defeated: island
                    .fauna
                    .creatures()
                    .iter()
                    .enumerate()
                    .filter_map(|(index, creature)| (!creature.active).then_some(index))
                    .collect(),
            }),
            town: world.town.as_ref().map(|town| TownSave {
                position: town.pos,
                facing: town.facing,
                tracked: town.tracked,
                track_side: town.tracking_side(),
                low_tide: town.low_tide,
            }),
            fort: world.fort.clone(),
            quests: world.town_progress,
            campaign: world.campaign.clone(),
            goods: self.goods.clone(),
        })
        .ok()
    }

    pub(super) fn reset_to_title(&mut self) {
        let resume = self.continue_game.take();
        *self = Self::new();
        self.continue_game = resume;
    }

    pub(super) fn title_input(&mut self, key: Key) {
        match key {
            Key::Back | Key::Map if self.confirm_overwrite => self.confirm_overwrite = false,
            Key::Up | Key::Down if self.has_continue() && !self.confirm_overwrite => {
                self.title_new_game = !self.title_new_game;
            }
            Key::Confirm if self.confirm_overwrite => {
                self.reset_to_title();
                self.continue_game = None;
                self.start_prologue();
            }
            Key::Confirm if self.title_new_game && self.has_continue() => {
                self.confirm_overwrite = true;
            }
            Key::Confirm => {
                if let Some(saved) = self.continue_game.take() {
                    *self = *saved;
                } else {
                    self.start_prologue();
                }
            }
            _ => {}
        }
    }
}

impl Save {
    fn restore(self) -> Option<Game> {
        if self.version != VERSION
            || !(1..=MAX_HP).contains(&self.hp)
            || self.memories > MEMORIES.len()
            || !Game::field_walkable(self.position)
            || self
                .companion
                .is_some_and(|(at, _)| !Game::field_walkable(at))
            || self.intro == Intro::Choice
            || !self.campaign.valid()
            || !self.goods.valid()
            || (self.island.is_some() && !self.map_open)
        {
            return None;
        }
        for (index, states) in self.quests.iter().enumerate() {
            for (story, state) in town::plan(index).stories.iter().zip(states) {
                match *state {
                    State::Active(beat) if beat >= story.beat_count() => return None,
                    State::Done(choice) if choice > usize::from(story.choices.is_some()) => {
                        return None
                    }
                    _ => {}
                }
            }
        }
        let mut game = Game::new();
        game.scene = if self.ending {
            Scene::Ending
        } else {
            Scene::Field
        };
        game.intro = self.intro;
        (game.x, game.y) = self.position;
        game.facing = self.facing;
        if let Some((at, facing)) = self.companion {
            game.lilith = true;
            game.companion_position = Some(at);
            game.companion_facing = facing;
        }
        game.hp = self.hp;
        game.memories = self.memories;
        game.goods = self.goods;
        game.map_open = self.map_open;
        let world = &mut game.world_map;
        world.selected = self.region;
        world.town_progress = self.quests;
        world.campaign = self.campaign;
        if let Some(saved) = self.island {
            let mut island = Island::new(self.region);
            (island.x, island.y) = saved.position;
            if !island.tile(island.x, island.y).walkable() {
                return None;
            }
            island.facing = saved.facing;
            for index in saved.defeated {
                if !island.fauna.creatures().get(index)?.species.is_beast() {
                    return None;
                }
                island.fauna.defeat(index);
            }
            world.island = Some(island);
        }
        if let Some(saved) = self.town {
            let island = world.island.as_ref()?;
            let Tile::Settlement(index) = island.tile(island.x, island.y) else {
                return None;
            };
            if self.region != Region::Nox || index >= town::COUNT || saved.tracked >= 5 {
                return None;
            }
            let mut town = Town::new(index, &world.town_progress[index]);
            town.restore_exploration(saved.tracked, saved.track_side, saved.low_tide);
            if !town.landscape.walkable(saved.position) {
                return None;
            }
            town.pos = saved.position;
            town.facing = saved.facing;
            town.main = world.campaign.beat();
            world.town = Some(town);
        }
        if let Some(fort) = self.fort {
            if world.town.is_some()
                || world.chapter_at_ruin().is_none()
                || !fort.walkable(fort.pos)
                || (world.campaign.fort_cleared && !fort.gate)
            {
                return None;
            }
            world.fort = Some(fort);
        }
        Some(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::{Item, OptionRow};

    #[test]
    fn restores_quests_inventory_equipment_and_reward_history() {
        let mut game = Game::new();
        game.input(Key::Confirm);
        game.intro = Intro::Nox;
        game.memories = 1;
        game.world_map.begin_nox();
        game.map_open = true;
        game.world_map.campaign.advance();
        game.world_map.town_progress[0][0] = State::Done(1);
        game.world_map.town_progress[0][1] = State::Active(1);
        let town = game.world_map.town.as_mut().unwrap();
        town.restore_exploration(1, true, false);
        game.goods.earn(100);
        game.goods.hp = 13;
        game.goods.act(OptionRow::Buy(Item::AshWood)).unwrap();
        game.goods.gather(Item::BogIron, 3).unwrap();
        let recipe = crate::economy::FORGE
            .iter()
            .find(|recipe| recipe.output == Item::IronSword)
            .unwrap();
        game.goods.act(OptionRow::Craft(recipe)).unwrap();
        game.goods.act(OptionRow::Item(Item::IronSword)).unwrap();
        game.goods.battle_reward(Region::Nox, 0, 18);
        game.world_map.island.as_mut().unwrap().fauna.defeat(0);
        let coins = game.goods.coins;

        let mut title = Game::new();
        assert!(title.offer_continue(&game.checkpoint().unwrap()));
        title.input(Key::Down);
        title.input(Key::Confirm);
        assert!(title.confirm_overwrite);
        title.input(Key::Back);
        title.input(Key::Up);
        title.input(Key::Confirm);

        assert_eq!(
            title.world_map.town_progress[0][..2],
            [State::Done(1), State::Active(1)]
        );
        assert_eq!(title.world_map.campaign.beat().unwrap().site, 1);
        let town = title.world_map.town.as_ref().unwrap();
        assert_eq!(town.tracked, 1);
        assert!(town.tracking_side());
        assert_eq!((title.goods.hp, title.memories), (13, 1));
        assert_eq!(title.goods.attack(), 8);
        assert_eq!(title.goods.count(Item::BogIron), 1);
        assert_eq!(title.goods.gather_wait(Item::BogIron), 96);
        assert!(!title.world_map.island.as_ref().unwrap().fauna.creatures()[0].active);
        title.goods.battle_reward(Region::Nox, 0, 18);
        assert_eq!(title.goods.coins, coins);
    }

    #[test]
    fn refuses_malformed_and_unsupported_saves_and_skips_battles() {
        let mut game = Game::new();
        game.input(Key::Confirm);
        let checkpoint = game.checkpoint().unwrap();
        game.start_battle(super::super::Enemy::Slime);
        assert!(game.checkpoint().is_none());

        let mut title = Game::new();
        let mut save = serde_json::from_slice::<serde_json::Value>(&checkpoint).unwrap();
        save["version"] = 255.into();
        assert!(!title.offer_continue(&serde_json::to_vec(&save).unwrap()));
        assert!(!title.offer_continue(b"{broken"));
        assert!(title.offer_continue(&checkpoint));
        assert!(title.has_continue());
    }
}

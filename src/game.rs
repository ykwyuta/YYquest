use crate::{
    campaign::VoyagePanel,
    debug::{self, Target},
    dialogue::{Expression, Message, Speaker},
    economy::{Goods, Menu, OptionRow, Page},
    fort,
    intro::{After, Intro, Lesson, HEARTH, MEETING, MEMORIAL, SLUICE},
    path::Distances,
    walk::{Walk, STEP_MS},
    world_map::{Region, WorldMap},
};

mod save;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Journal,
    Map,
    Back,
}

impl Key {
    pub fn from_code(code: u32) -> Option<Self> {
        Some(match code {
            0x26 | 0x57 => Self::Up,
            0x28 | 0x53 => Self::Down,
            0x25 | 0x41 => Self::Left,
            0x27 | 0x44 => Self::Right,
            0x0D | 0x20 | 0x5A => Self::Confirm,
            0x4A => Self::Journal,
            0x4D => Self::Map,
            0x1B | 0x58 => Self::Back,
            _ => return None,
        })
    }

    pub(crate) fn direction(self) -> Option<(i32, i32)> {
        Some(match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(usize)]
pub(crate) enum Facing {
    #[default]
    South,
    West,
    East,
    North,
}

impl Facing {
    pub(crate) fn from_step(dx: i32, dy: i32) -> Option<Self> {
        match (dx.signum(), dy.signum()) {
            (0, 1) => Some(Self::South),
            (-1, 0) => Some(Self::West),
            (1, 0) => Some(Self::East),
            (0, -1) => Some(Self::North),
            _ => None,
        }
    }
}

pub const MAP: [&[u8; 18]; 10] = [
    b"~~~~~~~~~~~~~~~~~~",
    b"~######.#########~",
    b"~#gHgg..gg###gRg#~",
    b"~#g.ggg.gg##g..g#~",
    b"~#g.........g.gg#~",
    b"~#ggg.g#gg#g..g##~",
    b"~##gg.g####..Lg##~",
    b"~#ggg.....S.ggg##~",
    b"~#ggggg##########~",
    b"~~~~~~~~~~~~~~~~~~",
];
pub(crate) const PROLOGUE_SIZE: (i32, i32) = (MAP[0].len() as i32, MAP.len() as i32);

pub const MAX_HP: u8 = 24;
pub const MEMORIES: [&str; 3] = ["父が剣を研ぐ音", "帰りを待つ家の灯り", "父の最後の言葉"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Beast(pub(crate) crate::island::fauna::Species);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enemy {
    Slime,
    Bat,
    Echo,
    FortEcho,
    AbyssLord,
    Beast(Beast),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tactic {
    Ordinary,
    Shell,
    Charge,
}

impl Enemy {
    pub(crate) fn tactic(self) -> Tactic {
        match self {
            Self::FortEcho | Self::AbyssLord => Tactic::Charge,
            Self::Beast(Beast(species)) => match (species.region, species.index) {
                (Region::Nox, 12) | (Region::Liberta, 14) => Tactic::Shell,
                (Region::Celestia, 14) | (Region::Vulcan, 10) => Tactic::Charge,
                _ => Tactic::Ordinary,
            },
            _ => Tactic::Ordinary,
        }
    }

    fn charge_damage(self) -> u8 {
        self.damage()
            + if matches!(self, Self::FortEcho | Self::AbyssLord) {
                6
            } else {
                4
            }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Slime => "霧のスライム",
            Self::Bat => "洞窟コウモリ",
            Self::Echo => "忘却の獣",
            Self::FortEcho => "砦の霧守",
            Self::AbyssLord => "アビス・ロード",
            Self::Beast(beast) => beast.0.name(),
        }
    }

    pub(crate) fn hp(self) -> u8 {
        match self {
            Self::Slime => 12,
            Self::Bat => 16,
            Self::Echo => 34,
            Self::FortEcho => 38,
            Self::AbyssLord => 48,
            Self::Beast(beast) => beast.0.combat().0,
        }
    }

    fn damage(self) -> u8 {
        match self {
            Self::Slime => 3,
            Self::Bat => 4,
            Self::Echo => 6,
            Self::FortEcho => 4,
            Self::AbyssLord => 5,
            Self::Beast(beast) => beast.0.combat().1,
        }
    }
}

pub const COMMANDS: [&str; 4] = ["たたかう", "刻印魔法", "精霊共鳴", "まもる"];

pub struct Battle {
    pub enemy: Enemy,
    pub hp: u8,
    pub selected: usize,
    pub log: Option<[String; 2]>,
    island: Option<IslandBattle>,
    lesson: Option<(Lesson, usize)>,
    story: Option<(Scene, u8)>,
    pub(crate) charged: bool,
}

impl Battle {
    pub(crate) fn expedition(&self) -> bool {
        self.island.is_some() || self.story.is_some()
    }

    fn new(enemy: Enemy) -> Self {
        Self {
            enemy,
            hp: enemy.hp(),
            selected: 0,
            log: None,
            island: None,
            lesson: None,
            story: None,
            charged: false,
        }
    }
}

#[derive(Clone, Copy)]
struct IslandBattle {
    creature: usize,
    scene: Scene,
    hp: u8,
}

#[derive(Clone, Copy)]
pub enum Scene {
    Title,
    Debug,
    Field,
    Battle,
    Defeat,
    Ending,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SoundCue {
    Confirm = 1,
    Hit,
    Victory,
    Chapter,
}

pub struct Game {
    pub scene: Scene,
    pub x: i32,
    pub y: i32,
    pub hp: u8,
    pub memories: usize,
    pub lilith: bool,
    pub companion_position: Option<(i32, i32)>,
    pub(crate) facing: Facing,
    pub(crate) companion_facing: Facing,
    pub journal: bool,
    pub(crate) message: Option<Message>,
    pub battle: Option<Battle>,
    pub(crate) map_open: bool,
    pub(crate) world_map: WorldMap,
    pub(crate) debug_menu: debug::Menu,
    title_code: usize,
    pub(crate) intro: Intro,
    after: After,
    pub(crate) walk: Walk,
    pub(crate) companion_walk: Walk,
    pub(crate) goods: Goods,
    pub(crate) economy_menu: Option<Menu>,
    held_direction: Option<HeldDirection>,
    continue_game: Option<Box<Game>>,
    pub(crate) title_new_game: bool,
    pub(crate) confirm_overwrite: bool,
    sound: Option<SoundCue>,
}

struct HeldDirection {
    code: u32,
    source: u32,
    until_step: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            scene: Scene::Title,
            x: 3,
            y: 4,
            hp: MAX_HP,
            memories: MEMORIES.len(),
            lilith: false,
            companion_position: None,
            facing: Facing::South,
            companion_facing: Facing::South,
            journal: false,
            message: None,
            battle: None,
            map_open: false,
            world_map: WorldMap::new(),
            debug_menu: debug::Menu::default(),
            title_code: 0,
            intro: Intro::Hearth,
            after: After::None,
            walk: Walk::default(),
            companion_walk: Walk::default(),
            goods: Goods::new(),
            economy_menu: None,
            held_direction: None,
            continue_game: None,
            title_new_game: false,
            confirm_overwrite: false,
            sound: None,
        }
    }

    pub fn tile(x: i32, y: i32) -> u8 {
        MAP.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(b'#')
    }

    pub(crate) fn field_walkable(at: (i32, i32)) -> bool {
        !matches!(Self::tile(at.0, at.1), b'#' | b'~')
    }

    pub(crate) fn can_open_map(&self) -> bool {
        match self.scene {
            Scene::Title | Scene::Ending => true,
            Scene::Field => !self.journal && self.message.is_none(),
            _ => false,
        }
    }

    pub fn advance_traffic(&mut self, milliseconds: u32) -> bool {
        let animated = if self.map_open {
            if let Some(town) = &mut self.world_map.town {
                town.walk.advance(milliseconds) | town.advance_gathering(milliseconds)
            } else {
                self.world_map
                    .island
                    .as_mut()
                    .is_some_and(|island| island.advance(milliseconds))
            }
        } else if matches!(self.scene, Scene::Field) {
            self.walk.advance(milliseconds) | self.companion_walk.advance(milliseconds)
        } else {
            false
        };
        animated | self.advance_held_direction(milliseconds)
    }

    fn can_repeat_walk(&self) -> bool {
        if self.economy_menu.is_some() {
            return false;
        }
        if self.map_open {
            self.world_map.campaign.panel.is_none()
                && self.world_map.island.is_some()
                && self
                    .world_map
                    .fort
                    .as_ref()
                    .is_none_or(|fort| matches!(fort.panel, fort::Panel::Walk))
                && self
                    .world_map
                    .town
                    .as_ref()
                    .is_none_or(|town| matches!(town.panel, crate::town::Panel::Explore))
        } else {
            matches!(self.scene, Scene::Field) && self.message.is_none() && !self.journal
        }
    }

    pub fn press_direction(&mut self, code: u32, source: u32) -> bool {
        if !Key::from_code(code).is_some_and(|key| key.direction().is_some()) {
            return false;
        }
        self.clear_held_direction();
        let exploring = self.can_repeat_walk();
        let handled = self.input_code(code, false);
        if exploring && self.can_repeat_walk() {
            self.held_direction = Some(HeldDirection {
                code,
                source,
                until_step: STEP_MS,
            });
        }
        handled
    }

    pub fn release_direction(&mut self, code: u32, source: u32) {
        if self
            .held_direction
            .as_ref()
            .is_some_and(|held| held.code == code && held.source == source)
        {
            self.clear_held_direction();
        }
    }

    pub fn clear_held_direction(&mut self) {
        self.held_direction = None;
    }

    fn advance_held_direction(&mut self, milliseconds: u32) -> bool {
        if !self.can_repeat_walk() {
            self.clear_held_direction();
            return false;
        }
        let Some(held) = &mut self.held_direction else {
            return false;
        };
        if milliseconds < held.until_step {
            held.until_step -= milliseconds;
            return false;
        }
        let code = held.code;
        held.until_step = STEP_MS - (milliseconds - held.until_step) % STEP_MS;
        self.input_code(code, true)
    }

    pub fn debug_back_control(&self) -> bool {
        !self.map_open && (matches!(self.scene, Scene::Debug) || self.debug_menu.previewing)
    }

    pub fn input_code(&mut self, code: u32, repeat: bool) -> bool {
        let Some(key) = Key::from_code(code) else {
            return false;
        };
        let title = !self.map_open && matches!(self.scene, Scene::Title);
        if repeat && (!self.can_repeat_walk() || key.direction().is_none()) {
            return false;
        }
        if key.direction().is_none() {
            self.clear_held_direction();
        }
        if title {
            let sequence = debug::TITLE_CODE;
            if key == Key::Confirm && self.title_code == sequence.len() {
                self.title_code = 0;
                self.scene = Scene::Debug;
                return true;
            }
            self.title_code = if sequence.get(self.title_code) == Some(&code) {
                self.title_code + 1
            } else if code == sequence[0] {
                if self.title_code == 2 {
                    2
                } else {
                    1
                }
            } else {
                0
            };
        }
        self.input(key);
        true
    }

    pub fn take_sound(&mut self) -> Option<SoundCue> {
        self.sound.take()
    }

    pub fn input(&mut self, key: Key) {
        let hp = self.hp;
        let restoring = matches!(self.scene, Scene::Title | Scene::Debug);
        let chapter = self.world_map.campaign.complete();
        let finished = self.world_map.campaign.finished();
        let pending_round = self
            .battle
            .as_ref()
            .is_some_and(|battle| battle.log.is_some());
        self.handle_input(key);
        if key != Key::Confirm {
            return;
        }
        self.sound = Some(
            if !restoring
                && ((!chapter && self.world_map.campaign.complete())
                    || (!finished && self.world_map.campaign.finished()))
            {
                SoundCue::Chapter
            } else if let Some(battle) = self
                .battle
                .as_ref()
                .filter(|battle| !pending_round && battle.log.is_some())
            {
                if battle.hp == 0 {
                    SoundCue::Victory
                } else if self.hp < hp {
                    SoundCue::Hit
                } else {
                    SoundCue::Confirm
                }
            } else {
                SoundCue::Confirm
            },
        );
    }

    fn handle_input(&mut self, key: Key) {
        if self.economy_menu.is_some() {
            self.economy_input(key);
            return;
        }
        if !self.map_open && matches!(self.scene, Scene::Debug) {
            let key = if key == Key::Map { Key::Back } else { key };
            match self.debug_menu.input(key) {
                debug::Action::Title => self.reset_to_title(),
                debug::Action::Jump(target) => self.debug_jump(target),
                debug::Action::Stay => {}
            }
            return;
        }
        if self.debug_back_control()
            && (matches!(key, Key::Back | Key::Map)
                || (key == Key::Confirm && matches!(self.scene, Scene::Ending | Scene::Defeat)))
        {
            self.return_to_debug();
            return;
        }
        if self.map_open {
            if let Some(town) = &mut self.world_map.town {
                if key == Key::Confirm && town.gather(&mut self.goods) {
                    return;
                }
                let region = town.region;
                let index = town.index;
                let position = town.pos;
                let previous = self.world_map.region_progress(region)[index];
                let was_complete = self.world_map.campaign.complete();
                if let Some(service) = self.world_map.town_input(key) {
                    self.economy_menu = Some(Menu::service(service));
                }
                if self
                    .world_map
                    .town
                    .as_ref()
                    .is_some_and(|town| town.pos != position)
                {
                    self.goods.walked();
                }
                for (old, new) in previous
                    .into_iter()
                    .zip(self.world_map.region_progress(region)[index])
                {
                    if !matches!(old, crate::town::State::Done(_))
                        && matches!(new, crate::town::State::Done(_))
                    {
                        self.goods.earn(12);
                    }
                }
                if !was_complete && self.world_map.campaign.complete() {
                    self.world_map.town = None;
                    self.world_map.island = None;
                    self.world_map.selected = Region::Celestia;
                    self.map_open = false;
                    self.scene = Scene::Ending;
                }
                return;
            }
            if let Some(interior) = &mut self.world_map.fort {
                let was = interior.pos;
                let action = interior.input(key, self.world_map.campaign.fort_cleared);
                if interior.pos != was {
                    self.goods.walked();
                }
                match action {
                    fort::Action::Leave => self.world_map.fort = None,
                    fort::Action::Battle => self.start_story_battle(Enemy::FortEcho),
                    fort::Action::Deliver { sail } => {
                        self.world_map.fort = None;
                        self.world_map.campaign.advance();
                        if sail {
                            self.world_map.island.as_mut().unwrap().return_to_harbor();
                        }
                    }
                    fort::Action::Stay
                        if key == Key::Journal && matches!(interior.panel, fort::Panel::Walk) =>
                    {
                        self.economy_menu = Some(Menu::new(Page::Bag));
                    }
                    fort::Action::Stay => {}
                }
                return;
            }
            if self.world_map.enter_fort(key) {
                return;
            }
            if self.voyage_input(key) {
                return;
            }
            if key == Key::Journal && self.world_map.island.is_some() {
                self.economy_menu = Some(Menu::new(Page::Bag));
                return;
            }
            if matches!(key, Key::Map | Key::Back) {
                self.map_open = self.world_map.close_island();
                if !self.map_open && self.debug_menu.previewing {
                    self.return_to_debug();
                }
            } else if key == Key::Confirm && !self.begin_island_battle() {
                self.world_map.confirm();
            } else if let Some((dx, dy)) = key.direction() {
                let position = self
                    .world_map
                    .island
                    .as_ref()
                    .map(|island| (island.x, island.y));
                self.world_map.step(dx, dy);
                if self
                    .world_map
                    .island
                    .as_ref()
                    .is_some_and(|island| Some((island.x, island.y)) != position)
                {
                    self.goods.walked();
                }
            }
            return;
        }

        if key == Key::Map && self.can_open_map() && !self.confirm_overwrite {
            self.title_code = 0;
            self.map_open = true;
            return;
        }

        if self.journal {
            if matches!(key, Key::Journal | Key::Back | Key::Confirm) {
                self.journal = false;
            }
            return;
        }

        match self.scene {
            Scene::Title => self.title_input(key),
            Scene::Field => self.field_input(key),
            Scene::Battle => self.battle_input(key),
            Scene::Ending if key == Key::Confirm => self.map_open = true,
            Scene::Defeat if key == Key::Confirm => self.reset_to_title(),
            _ => {}
        }
    }

    fn voyage_input(&mut self, key: Key) -> bool {
        let Some(beat) = self.world_map.voyage_here() else {
            return false;
        };
        let campaign = &mut self.world_map.campaign;
        let Some(panel) = campaign.panel else {
            if key == Key::Confirm {
                campaign.panel = Some(VoyagePanel::Read);
                return true;
            }
            return false;
        };
        match (panel, key) {
            (_, Key::Back | Key::Map) => campaign.panel = None,
            (VoyagePanel::Consent(selected), Key::Up | Key::Down) => {
                campaign.panel = Some(VoyagePanel::Consent(1 - selected));
            }
            (VoyagePanel::Consent(selected), Key::Confirm) => {
                campaign.panel = Some(VoyagePanel::Response(selected == 0));
            }
            (VoyagePanel::Read, Key::Confirm) if beat.consent() => {
                campaign.panel = Some(VoyagePanel::Consent(0));
            }
            (VoyagePanel::Read, Key::Confirm) if beat.battle() => {
                campaign.panel = None;
                self.start_story_battle(Enemy::AbyssLord);
            }
            (VoyagePanel::Read | VoyagePanel::Response(_), Key::Confirm) => {
                if let VoyagePanel::Response(sealed) = panel {
                    campaign.sealed_names = sealed;
                }
                let previous_region = beat.region;
                campaign.advance_voyage();
                if beat.epilogue() {
                    self.world_map.island = None;
                    self.map_open = false;
                    self.scene = Scene::Ending;
                } else if let Some(next) = campaign.voyage() {
                    if next.region != previous_region {
                        self.world_map.selected = next.region;
                        self.world_map.island = None;
                    } else if next.battle() {
                        self.goods.hp = MAX_HP;
                    }
                }
            }
            _ => {}
        }
        true
    }

    fn start_prologue(&mut self) {
        self.scene = Scene::Field;
        self.message = Some(Message::narration([
            "東の森から、灰灯りの村へ水が来なくなった。",
            "アレンは村の炉を守る「炉番」のもとへ向かう。",
        ]));
    }

    fn return_to_debug(&mut self) {
        let mut menu = std::mem::take(&mut self.debug_menu);
        menu.previewing = false;
        self.reset_to_title();
        self.scene = Scene::Debug;
        self.debug_menu = menu;
    }

    fn debug_jump(&mut self, target: Target) {
        self.return_to_debug();
        self.debug_menu.previewing = true;
        match target {
            Target::Site(region, index) => self.debug_site(region, index),
            Target::Story(town, story, beat) => {
                self.debug_site(Region::Nox, town);
                self.world_map.town.as_mut().unwrap().preview_story(
                    &mut self.world_map.town_progress[town],
                    story,
                    beat,
                );
            }
            Target::Prologue(event) => {
                self.scene = Scene::Field;
                match event {
                    0 => self.start_prologue(),
                    1 => {
                        (self.x, self.y) = HEARTH;
                        self.hp = 1;
                        self.inspect_intro();
                    }
                    2 => {
                        self.intro = Intro::Meeting;
                        (self.x, self.y) = MEETING;
                        self.inspect_intro();
                    }
                    3 => {
                        self.intro = Intro::Memorial;
                        (self.x, self.y) = MEMORIAL;
                        self.inspect_intro();
                    }
                    4 => {
                        self.intro = Intro::Sluice;
                        (self.x, self.y) = SLUICE;
                        self.start_lesson(Lesson::Sluice);
                    }
                    5 => self.start_battle(Enemy::Bat),
                    6 => {
                        self.intro = Intro::Memorial;
                        (self.x, self.y) = MEMORIAL;
                        self.start_lesson(Lesson::Memorial);
                    }
                    7 => {
                        self.intro = Intro::Choice;
                        (self.x, self.y) = MEMORIAL;
                        self.after = After::Ask;
                        self.finish_dialogue();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn debug_site(&mut self, region: Region, index: usize) {
        self.map_open = true;
        self.world_map.selected = region;
        self.world_map.confirm();
        let island = self.world_map.island.as_mut().unwrap();
        (island.x, island.y) = island.settlements()[index].door;
        if region == Region::Nox && index < crate::town::COUNT {
            self.world_map.confirm();
        }
    }

    fn field_input(&mut self, key: Key) {
        if self.message.is_some() {
            if matches!(key, Key::Confirm | Key::Back) {
                self.finish_dialogue();
            }
            return;
        }
        if key == Key::Journal {
            self.journal = true;
            return;
        }
        if key == Key::Confirm {
            self.inspect_intro();
            return;
        }
        let Some((dx, dy)) = key.direction() else {
            return;
        };
        self.facing = Facing::from_step(dx, dy).unwrap_or(self.facing);
        let current = (self.x, self.y);
        let next = (self.x + dx, self.y + dy);
        if !Self::field_walkable(next) {
            return;
        }
        if self.intro != Intro::Nox {
            let route = Distances::new(
                PROLOGUE_SIZE,
                self.intro.destination().0,
                Self::field_walkable,
            );
            if route.get(next) >= route.get(current) {
                self.clear_held_direction();
                self.after = After::None;
                self.message = Some(Message::guide([
                    "案内精霊が光で道を照らしている。",
                    "光のあとをたどって、目的地へ向かおう。",
                ]));
                return;
            }
        }
        self.walk.step(current, next);
        if self.lilith {
            if let Some((cx, cy)) = self.companion_position {
                self.companion_facing =
                    Facing::from_step(self.x - cx, self.y - cy).unwrap_or(self.companion_facing);
                self.companion_walk.step((cx, cy), current);
            } else {
                self.companion_facing = self.facing;
            }
            self.companion_position = Some(current);
        }
        (self.x, self.y) = next;
    }

    fn inspect_intro(&mut self) {
        use Expression::*;
        use Speaker::*;
        if (self.x, self.y) != self.intro.destination().0 {
            if (self.x, self.y) == HEARTH {
                self.hp = MAX_HP;
                self.message = Some(Message::narration([
                    "炉のそばで休むと、手足の冷えが引いた。",
                    "アレンの体力が回復した。",
                ]));
            }
            return;
        }
        self.message = Some(match self.intro {
            Intro::Hearth => {
                self.hp = MAX_HP;
                self.after = After::Continue(Intro::Meeting);
                Message::spoken(
                    Hearth,
                    Worried,
                    [
                        "森の水路が止まったうえ、東の森から笛の音が聞こえたよ。",
                        "古い水門は父さんの剣で開けられる。持って見てきておくれ。",
                    ],
                )
            }
            Intro::Meeting => {
                self.after = After::Continue(Intro::Sluice);
                Message::spoken(
                    Lilith,
                    Speaking,
                    [
                        "村の人？　さっき笛を吹いたの、私。リリスっていうの。",
                        "水門が引っかかって開かないの。剣で持ち上げるのを手伝ってくれない？",
                    ],
                )
            }
            Intro::Sluice => {
                self.after = After::Fight(Lesson::Sluice);
                Message::narration([
                    "アレンが父の剣を留め具に差し込むと、重い水門が持ち上がった。",
                    "リリスが門を支えたその瞬間、水路の奥から魔物が飛び出した！",
                ])
            }
            Intro::Memorial => {
                self.after = After::Fight(Lesson::Memorial);
                Message::spoken(
                    Lilith,
                    Resolute,
                    [
                        "石碑が濃い霧に覆われて、さっきの音をかき消しているみたい。",
                        "私が精霊の力で霧を払うから、少しの間だけ魔物を引き受けて！",
                    ],
                )
            }
            Intro::Choice => return,
            Intro::Nox => {
                self.hp = MAX_HP;
                Message::spoken(
                    Hearth,
                    Relieved,
                    [
                        "水はもう来てるよ。火のそばにおいで。",
                        "顔色がよくなったね。出る前に、もう少し温まっていきな。",
                    ],
                )
            }
            Intro::Return => {
                self.after = After::BeginNox;
                Message::spoken(
                    Hearth,
                    Smile,
                    [
                        "村に水が戻ったよ。二人とも、本当に助かった！",
                        "そうだ、お父さんの帰還記録なら、村の旧宅にしまってあるよ。",
                    ],
                )
            }
        });
    }

    fn finish_dialogue(&mut self) {
        self.message = None;
        match std::mem::take(&mut self.after) {
            After::None => {}
            After::Continue(next) => self.intro = next,
            After::Fight(lesson) => self.start_lesson(lesson),
            After::Ask => {
                self.message = Some(Message::spoken(
                    Speaker::Allen,
                    Expression::Thoughtful,
                    [
                        "今の声……父さんは救助船を見送って、あの日、砦に残ったのか？",
                        "父さんの家に記録があるはずだ。リリスも一緒に来てくれないか？",
                    ],
                ));
                self.after = After::Reply;
            }
            After::Reply => {
                self.message = Some(Message::spoken(
                    Speaker::Lilith,
                    Expression::Resolute,
                    [
                        "うん、私も行く。自分の名前以外の記憶がなくて……",
                        "なぜ石碑から私を呼ぶ声が聞こえたのか、確かめたいの。",
                    ],
                ));
                self.after = After::Recruit;
            }
            After::Recruit => {
                self.lilith = true;
                self.companion_position = Some(self.intro.waiting_lilith());
                self.intro = Intro::Return;
                self.message = Some(Message::narration([
                    "二人はガウェインの記録を探すことにした。",
                    "リリスが仲間になった。",
                ]));
            }
            After::BeginNox => {
                self.intro = Intro::Nox;
                self.world_map.begin_nox();
                self.map_open = true;
            }
        }
    }

    fn start_lesson(&mut self, lesson: Lesson) {
        self.hp = MAX_HP;
        self.start_battle(match lesson {
            Lesson::Sluice => Enemy::Slime,
            Lesson::Memorial => Enemy::Echo,
        });
        self.battle.as_mut().unwrap().lesson = Some((lesson, self.memories));
    }

    fn finish_lesson(&mut self, lesson: Lesson, memories: usize) {
        self.battle = None;
        self.scene = Scene::Field;
        if self.hp == 0 {
            self.hp = MAX_HP;
            self.memories = memories;
            self.message = Some(Message::spoken(
                Speaker::Lilith,
                Expression::Worried,
                [
                    "よかった、気がついたんだね。急いで霧の外まで運んだよ。",
                    "無理しないで。もう一度挑むなら、今度は私もそばにいるから。",
                ],
            ));
            return;
        }
        self.message = Some(match lesson {
            Lesson::Sluice => {
                self.after = After::Continue(Intro::Memorial);
                Message::spoken(
                    Speaker::Lilith,
                    Expression::Relieved,
                    [
                        "水路が通ったね！　……あれ、今アレンの剣が鳴らなかった？",
                        "北の石碑からも同じ音が返ってくる。先に様子を見てくるね！",
                    ],
                )
            }
            Lesson::Memorial => {
                self.intro = Intro::Choice;
                self.after = After::Ask;
                Message::record(
                    "石碑から響く過去の声",
                    [
                        "『ガウェイン、砦の三つの灯が見えた。救助船をそっちへ向かわせる』",
                        "『リリス、船が着くまで砦の灯を絶やすな』",
                    ],
                )
            }
        });
    }

    pub(crate) fn can_resonate(&self) -> bool {
        self.lilith
            || self
                .battle
                .as_ref()
                .is_some_and(|battle| matches!(battle.lesson, Some((Lesson::Memorial, _))))
    }

    pub(crate) fn lesson_tip(&self) -> Option<&'static str> {
        self.battle
            .as_ref()?
            .lesson
            .map(|(lesson, _)| match lesson {
                Lesson::Sluice => "「刻印魔法」は強いが、使うと記憶が失われる",
                Lesson::Memorial => "「精霊共鳴」は記憶を使わずリリスと協力して攻撃",
            })
    }

    fn start_battle(&mut self, enemy: Enemy) {
        self.battle = Some(Battle::new(enemy));
        self.scene = Scene::Battle;
    }

    fn begin_island_battle(&mut self) -> bool {
        let Some(island) = self.world_map.island.as_ref() else {
            return false;
        };
        if island.current_landmark().is_some() {
            return false;
        }
        let Some((creature, enemy)) = island.fauna.nearby_beast((island.x, island.y)) else {
            return false;
        };
        let mut battle = Battle::new(enemy);
        battle.island = Some(IslandBattle {
            creature,
            scene: self.scene,
            hp: self.hp,
        });
        self.battle = Some(battle);
        self.hp = self.goods.hp;
        self.scene = Scene::Battle;
        self.map_open = false;
        true
    }

    fn finish_island_battle(&mut self, expedition: IslandBattle) {
        let island = self
            .world_map
            .island
            .as_mut()
            .expect("Island remained open for its battle");
        if self.hp == 0 {
            island.return_to_harbor();
            self.goods.hp = MAX_HP;
        } else {
            island.fauna.defeat(expedition.creature);
            self.goods.hp = self.hp;
        }
        self.hp = expedition.hp;
        self.scene = expedition.scene;
        self.map_open = true;
        self.battle = None;
    }

    fn start_story_battle(&mut self, enemy: Enemy) {
        let (scene, hp) = (self.scene, self.hp);
        self.start_battle(enemy);
        self.battle.as_mut().unwrap().story = Some((scene, hp));
        self.hp = self.goods.hp;
        self.map_open = false;
    }

    fn finish_story_battle(&mut self, scene: Scene, hp: u8) {
        if self.hp == 0 {
            if let Some(fort) = &mut self.world_map.fort {
                fort.pos = fort::ENTRY;
                fort.panel = fort::Panel::Recovered;
            }
            self.goods.hp = MAX_HP;
        } else {
            if self.world_map.fort.is_some() {
                self.world_map.campaign.fort_cleared = true;
            } else {
                self.world_map.campaign.advance_voyage();
            }
            self.goods.hp = self.hp;
        }
        self.hp = hp;
        self.scene = scene;
        self.map_open = true;
        self.battle = None;
    }

    fn battle_input(&mut self, key: Key) {
        let can_resonate = self.can_resonate();
        let battle = self.battle.as_mut().expect("Battle scene has an enemy");
        if battle.log.is_some() {
            if key != Key::Confirm {
                return;
            }
            if let Some(expedition) = battle.island {
                if self.hp == 0 || battle.hp == 0 {
                    self.finish_island_battle(expedition);
                } else {
                    battle.log = None;
                }
                return;
            }
            if let Some((scene, hp)) = battle.story {
                if self.hp == 0 || battle.hp == 0 {
                    self.finish_story_battle(scene, hp);
                } else {
                    battle.log = None;
                }
                return;
            }
            if let Some((lesson, memories)) = battle.lesson {
                if self.hp == 0 || battle.hp == 0 {
                    self.finish_lesson(lesson, memories);
                } else {
                    battle.log = None;
                }
                return;
            }
            if self.hp == 0 {
                self.scene = Scene::Defeat;
            } else if battle.hp == 0 {
                self.scene = Scene::Field;
            } else {
                battle.log = None;
                return;
            }
            self.battle = None;
            return;
        }

        let commands = COMMANDS.len() + usize::from(battle.expedition());
        match key {
            Key::Up => battle.selected = (battle.selected + commands - 1) % commands,
            Key::Down => battle.selected = (battle.selected + 1) % commands,
            Key::Confirm if battle.selected == COMMANDS.len() && battle.expedition() => {
                self.economy_menu = Some(Menu::new(Page::Battle));
            }
            Key::Confirm => {
                let mut attack = if battle.expedition() {
                    self.goods.attack()
                } else {
                    6
                };
                if battle.enemy.tactic() == Tactic::Shell {
                    attack = attack.saturating_sub(3).max(1);
                }
                let (damage, incoming, description) = match battle.selected {
                    0 => (attack, battle.enemy.damage(), "アレンの攻撃！".into()),
                    1 if self.memories > 0 => {
                        self.memories -= 1;
                        (
                            16,
                            battle.enemy.damage(),
                            format!("刻印魔法！　記憶「{}」が失われた…", MEMORIES[self.memories]),
                        )
                    }
                    2 if can_resonate => (9, battle.enemy.damage(), "リリスの精霊共鳴！".into()),
                    3 => (0, 1, "アレンは身を守った。".into()),
                    _ => return,
                };
                self.battle_round(damage, incoming, description);
            }
            _ => {}
        }
    }

    fn battle_round(&mut self, damage: u8, incoming: u8, description: String) {
        let battle = self.battle.as_mut().expect("A round requires an enemy");
        battle.hp = battle.hp.saturating_sub(damage);
        let result = if battle.hp == 0 {
            if let Some(expedition) = battle.island {
                let region = self.world_map.island.as_ref().unwrap().region;
                self.goods
                    .battle_reward(region, expedition.creature, battle.enemy.hp())
            } else {
                format!("{}を倒した！", battle.enemy.name())
            }
        } else {
            let mut incoming = if battle.expedition() {
                incoming.saturating_sub(self.goods.defense()).max(1)
            } else {
                incoming
            };
            let mut interrupted = false;
            if battle.enemy.tactic() == Tactic::Charge {
                if battle.charged {
                    match battle.selected {
                        2 => {
                            incoming = 0;
                            interrupted = true;
                        }
                        3 => incoming = 1,
                        _ => {
                            incoming = battle
                                .enemy
                                .charge_damage()
                                .saturating_sub(self.goods.defense())
                                .max(1)
                        }
                    }
                }
                battle.charged = !battle.charged;
            }
            self.hp = self.hp.saturating_sub(incoming);
            if self.hp == 0 && battle.enemy == Enemy::FortEcho {
                "アレンは倒れた。決定で砦の入口へ戻る。".into()
            } else if self.hp == 0 && battle.expedition() {
                "アレンは倒れた。決定で港へ戻る。".into()
            } else if interrupted {
                "リリスの共鳴が敵の大技を封じた！".into()
            } else {
                format!(
                    "{}の反撃。アレンは {} のダメージ。",
                    battle.enemy.name(),
                    incoming
                )
            }
        };
        battle.log = Some([description, result]);
    }

    fn economy_input(&mut self, key: Key) {
        let mut menu = self.economy_menu.take().unwrap();
        if matches!(key, Key::Right | Key::Journal)
            && !matches!(menu.page, Page::Bag | Page::Battle)
        {
            let mut bag = Menu::new(Page::Bag);
            bag.return_to = Some(menu.page);
            self.economy_menu = Some(bag);
            return;
        }
        if matches!(key, Key::Back | Key::Map | Key::Left | Key::Journal) {
            if let Some(page) = menu.return_to {
                self.economy_menu = Some(Menu::new(page));
            } else if matches!(menu.page, Page::Buy | Page::Sell) {
                self.economy_menu = Some(Menu::new(Page::Shop));
            }
            return;
        }
        let rows = menu.rows(&self.goods);
        if !rows.is_empty() {
            match key {
                Key::Up => menu.selected = (menu.selected + rows.len() - 1) % rows.len(),
                Key::Down => menu.selected = (menu.selected + 1) % rows.len(),
                Key::Confirm => match rows[menu.selected] {
                    OptionRow::BuyList => menu = Menu::new(Page::Buy),
                    OptionRow::SellList => menu = Menu::new(Page::Sell),
                    OptionRow::Item(item) if menu.page == Page::Battle => {
                        match self.goods.use_item(item, &mut self.hp) {
                            Ok(damage) => {
                                let incoming = self.battle.as_ref().unwrap().enemy.damage();
                                self.battle_round(
                                    damage,
                                    incoming,
                                    format!("{}を使った。", item.name()),
                                );
                                return;
                            }
                            Err(reason) => menu.note = reason.into(),
                        }
                    }
                    row => {
                        menu.note = self.goods.act(row).unwrap_or_else(str::to_owned);
                        menu.selected = menu
                            .selected
                            .min(menu.rows(&self.goods).len().saturating_sub(1));
                    }
                },
                _ => {}
            }
        }
        self.economy_menu = Some(menu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(game: &mut Game, keys: &[Key]) {
        for key in keys {
            game.input(*key);
        }
    }

    fn walk_to(game: &mut Game, goal: crate::island::Position) -> usize {
        use crate::island::{HEIGHT, WIDTH};
        let position = |game: &Game| {
            if let Some(fort) = &game.world_map.fort {
                fort.pos
            } else if let Some(town) = &game.world_map.town {
                town.pos
            } else {
                let island = game.world_map.island.as_ref().unwrap();
                (island.x, island.y)
            }
        };
        let route = if let Some(fort) = &game.world_map.fort {
            Distances::new((13, 7), goal, |at| fort.walkable(at))
        } else if let Some(town) = &game.world_map.town {
            Distances::new((WIDTH, HEIGHT), goal, |at| town.landscape.walkable(at))
        } else {
            let island = game.world_map.island.as_ref().unwrap();
            Distances::new((WIDTH, HEIGHT), goal, |(x, y)| island.tile(x, y).walkable())
        };
        let expected = route.get(position(game));
        assert_ne!(expected, crate::path::UNREACHABLE, "route to {goal:?}");
        for remaining in (1..=expected).rev() {
            let (x, y) = position(game);
            let key = [Key::Up, Key::Down, Key::Left, Key::Right]
                .into_iter()
                .find(|key| {
                    let (dx, dy) = key.direction().unwrap();
                    route.get((x + dx, y + dy)) == remaining - 1
                })
                .unwrap();
            game.input(key);
            assert_eq!(route.get(position(game)), remaining - 1);
        }
        usize::from(expected)
    }

    fn win_story_battle(game: &mut Game, expected: Enemy) {
        assert_eq!(game.battle.as_ref().unwrap().enemy, expected);
        for _ in 0..10 {
            let Some(battle) = &game.battle else {
                return;
            };
            assert_ne!(game.hp, 0);
            let target = if battle.charged { 2 } else { 0 };
            while game.battle.as_ref().unwrap().selected != target {
                game.input(Key::Down);
            }
            press(game, &[Key::Confirm, Key::Confirm]);
        }
        panic!("battle did not end");
    }

    #[test]
    fn nox_services_collect_brew_craft_equip_and_trade() {
        use crate::{economy::Item, town::Action};
        use Key::*;

        fn enter(game: &mut Game, town: usize) {
            game.map_open = true;
            game.world_map.selected = Region::Nox;
            if game.world_map.island.is_none() {
                game.input(Confirm);
            }
            let island = game.world_map.island.as_mut().unwrap();
            (island.x, island.y) = island.settlements()[town].door;
            game.input(Confirm);
        }

        fn visit(game: &mut Game, town: usize, place: usize) {
            enter(game, town);
            let map = game.world_map.town.as_mut().unwrap();
            map.pos = map.plan().places[place].at;
            let row = map
                .local_actions(place, &game.world_map.town_progress[town])
                .position(|action| matches!(action, Action::Service(_)))
                .unwrap();
            game.input(Confirm);
            for _ in 0..row {
                game.input(Down);
            }
            game.input(Confirm);
            assert!(game.economy_menu.is_some());
        }

        fn collect(game: &mut Game, item: Item) {
            let map = game.world_map.town.as_mut().unwrap();
            let spot = map
                .gathering_spots()
                .iter()
                .find(|spot| spot.item == item)
                .unwrap();
            map.pos = (spot.at.0, spot.at.1 - 1);
            game.input(Confirm);
            assert!(game.economy_menu.is_none());
            let gathering = game
                .world_map
                .town
                .as_ref()
                .unwrap()
                .gathering
                .as_ref()
                .unwrap();
            assert_eq!(gathering.spot.item, item);
            assert!(gathering.result.is_ok());
        }

        let mut game = Game::new();
        enter(&mut game, 0);
        collect(&mut game, Item::AshHerb);
        game.input(Confirm);
        collect(&mut game, Item::AshWood);
        assert_eq!(game.goods.count(Item::AshHerb), 3);
        assert_eq!(game.goods.count(Item::AshWood), 2);
        assert!(game.goods.gather_wait(Item::AshHerb) > 0);
        game.input(Back);

        visit(&mut game, 0, 7);
        game.input(Confirm);
        assert_eq!(game.goods.count(Item::AshHerb), 1);
        assert_eq!(game.goods.count(Item::Poultice), 3);
        assert_eq!(game.goods.coins, 24);
        press(&mut game, &[Back, Back, Map]);

        visit(&mut game, 5, 1);
        game.input(Confirm);
        assert_eq!(game.goods.coins, 24);
        assert_eq!(game.goods.count(Item::AshWood), 2);
        press(&mut game, &[Back, Back, Map]);

        enter(&mut game, 5);
        collect(&mut game, Item::BogIron);
        game.input(Back);
        visit(&mut game, 5, 1);
        press(&mut game, &[Confirm, Confirm]);
        assert_eq!(game.goods.coins, 6);
        assert_eq!(game.goods.count(Item::BogIron), 1);
        assert_eq!(game.goods.count(Item::IronSword), 1);
        game.input(Right);
        game.input(Up);
        game.input(Confirm);
        assert!(game.goods.equipped(Item::IronSword));
        assert_eq!(game.goods.attack(), 8);
        press(&mut game, &[Back, Back, Back, Map]);

        visit(&mut game, 2, 1);
        press(&mut game, &[Down, Confirm, Confirm]);
        assert_eq!(game.goods.count(Item::AshHerb), 0);
        assert_eq!(game.goods.coins, 8);
    }

    #[test]
    fn battle_gauges_follow_damage_and_expedition_items() {
        use crate::{economy::Item, render::Frame};
        use Key::*;

        let mut game = Game::new();
        press(&mut game, &[Map, Confirm]);
        let island = game.world_map.island.as_mut().unwrap();
        let position = island
            .fauna
            .creatures()
            .iter()
            .find(|creature| creature.species.enemy().is_some())
            .unwrap()
            .position;
        (island.x, island.y) = position;
        game.goods.hp = 16;
        game.input(Confirm);
        assert_eq!(game.hp, 16);
        let enemy = game.battle.as_ref().unwrap().enemy;
        game.input(Confirm);
        let damaged = game.hp;
        assert!(damaged < 16);
        let mut frame = Frame::new();
        frame.draw(&game);
        assert!(frame
            .labels
            .iter()
            .any(|label| label.text == format!("HP {}/{}", enemy.hp() - 6, enemy.hp())));
        assert!(frame
            .labels
            .iter()
            .any(|label| label.text == format!("アレン  HP {damaged:02}/{MAX_HP}")));
        let row = &frame.pixels[38 * crate::render::BUFFER_WIDTH..39 * crate::render::BUFFER_WIDTH];
        assert_ne!(row[17 * 2], row[166 * 2]);
        assert_ne!(row[208 * 2], row[301 * 2]);

        press(&mut game, &[Confirm, Up, Confirm, Confirm]);
        assert_eq!(game.goods.count(Item::Poultice), 1);
        assert_eq!(game.hp, (damaged + 8).min(MAX_HP) - enemy.damage());
    }

    #[test]
    fn party_members_face_their_own_direction_when_turning() {
        use crate::walk::Pose;
        let mut game = Game::new();
        game.scene = Scene::Field;
        game.intro = Intro::Nox;
        game.lilith = true;
        (game.x, game.y) = (13, 6);
        game.input(Key::Left);
        assert_eq!(game.facing, Facing::West);
        assert_eq!(game.companion_facing, Facing::West);
        assert_eq!(game.companion_position, Some((13, 6)));
        assert_eq!(game.walk.offset(24), (24, 0));
        assert!(game.advance_traffic(80));
        assert_eq!(game.walk.pose(), Pose::Passing);
        assert!(game.walk.offset(24).0 > 0 && game.walk.offset(24).0 < 24);
        assert!(game.advance_traffic(80));
        assert_eq!(game.walk.pose(), Pose::Standing);

        game.input(Key::Up);
        assert_eq!((game.x, game.y), (12, 5));
        assert_eq!(game.facing, Facing::North);
        assert_eq!(game.companion_facing, Facing::West);
        assert_eq!(game.companion_position, Some((12, 6)));
    }

    #[test]
    fn held_direction_walks_until_released_and_alternates_the_stride() {
        use crate::walk::Pose;
        let mut game = Game::new();
        game.scene = Scene::Field;
        game.intro = Intro::Nox;
        assert!(game.press_direction(39, 1));
        assert_eq!((game.x, game.y), (4, 4));
        game.advance_traffic(48);
        assert_eq!(game.walk.pose(), Pose::Stride);
        game.advance_traffic(STEP_MS - 48);
        assert_eq!((game.x, game.y), (5, 4));
        game.advance_traffic(48);
        assert_eq!(game.walk.pose(), Pose::Opposite);
        game.release_direction(39, 1);
        game.advance_traffic(STEP_MS * 2);
        assert_eq!((game.x, game.y), (5, 4));
        assert_eq!(game.walk.pose(), Pose::Standing);
    }

    #[test]
    fn guardian_stops_detours_and_cancels_a_held_direction() {
        let mut game = Game::new();
        game.scene = Scene::Field;
        game.intro = Intro::Meeting;
        (game.x, game.y) = (12, 4);
        game.press_direction(39, 1);
        assert_eq!((game.x, game.y), (13, 4));
        game.advance_traffic(STEP_MS);
        assert_eq!((game.x, game.y), (13, 4));
        assert!(game.message.unwrap().lines[0].contains("案内精霊"));
        game.input(Key::Confirm);
        game.advance_traffic(STEP_MS * 2);
        assert!(game.message.is_none());
        assert_eq!(game.intro, Intro::Meeting);
        assert_eq!((game.x, game.y), (13, 4));
        game.press_direction(40, 1);
        game.advance_traffic(STEP_MS);
        assert_eq!((game.x, game.y), MEETING);
        game.input(Key::Confirm);
        assert!(game.message.unwrap().lines[0].contains("さっき笛を吹いた"));
    }

    fn through_prologue() -> Game {
        use Key::*;
        let mut game = Game::new();
        press(&mut game, &[Confirm, Confirm]);
        press(&mut game, &[Up, Up, Confirm, Confirm, Down, Down]);
        press(&mut game, &[Right; 10]);
        press(&mut game, &[Down, Down, Confirm, Confirm]);
        assert_eq!(game.intro, Intro::Sluice);
        assert!(!game.lilith);
        press(&mut game, &[Down, Left, Left, Left, Confirm, Confirm]);
        assert_eq!(game.battle.as_ref().unwrap().enemy, Enemy::Slime);
        press(&mut game, &[Down, Confirm, Confirm, Confirm]);
        assert_eq!(game.memories, 2);
        assert_eq!(game.intro, Intro::Memorial);

        press(&mut game, &[Right; 3]);
        press(&mut game, &[Up; 5]);
        press(&mut game, &[Right, Confirm, Confirm]);
        assert_eq!(game.battle.as_ref().unwrap().enemy, Enemy::Echo);
        assert!(game.can_resonate());
        assert!(!game.lilith);
        press(&mut game, &[Down, Down]);
        for _ in 0..4 {
            press(&mut game, &[Confirm, Confirm]);
        }
        assert_eq!(game.intro, Intro::Choice);
        assert!(!game.lilith);
        game.input(Confirm);
        assert!(!game.lilith);
        assert_eq!(
            game.message
                .as_ref()
                .unwrap()
                .delivery
                .portrait()
                .unwrap()
                .speaker,
            Speaker::Allen
        );
        game.input(Confirm);
        assert!(!game.lilith);
        assert_eq!(
            game.message
                .as_ref()
                .unwrap()
                .delivery
                .portrait()
                .unwrap()
                .speaker,
            Speaker::Lilith
        );
        press(&mut game, &[Confirm, Confirm]);
        assert!(game.lilith);
        assert_eq!(game.intro, Intro::Return);

        press(&mut game, &[Left, Down, Down]);
        press(&mut game, &[Left; 10]);
        press(&mut game, &[Up, Up, Confirm, Confirm]);
        assert!(game.map_open);
        assert_eq!(
            game.world_map.town.as_ref().unwrap().plan().name,
            "灰灯りの村"
        );
        assert_eq!(
            game.world_map.campaign.beat().unwrap().title,
            "途切れた帰還記録"
        );
        game
    }

    #[test]
    fn tutorial_earns_liliths_company_then_opens_nox() {
        through_prologue();
    }

    #[test]
    fn nox_chapter_connects_towns_and_the_ruin_to_the_next_voyage() {
        use Key::*;
        let mut game = through_prologue();
        let mut nox_steps = 0;
        for (index, place) in [
            (0, Some(1)),
            (1, Some(3)),
            (5, Some(1)),
            (7, Some(3)),
            (12, None),
            (1, Some(1)),
        ] {
            let step = game.world_map.campaign.beat().unwrap();
            assert_eq!((step.site, step.place), (index, place));
            if game.world_map.town.is_none() {
                let door = game.world_map.island.as_ref().unwrap().settlements()[index].door;
                nox_steps += walk_to(&mut game, door);
                game.input(Confirm);
            }
            if let Some(place) = place {
                let destination = game.world_map.town.as_ref().unwrap().plan().places[place].at;
                nox_steps += walk_to(&mut game, destination);
                press(&mut game, &[Confirm, Confirm]);
                assert!(matches!(
                    game.world_map.town.as_ref().unwrap().panel,
                    crate::town::Panel::Talk {
                        next: crate::town::Followup::Main,
                        ..
                    }
                ));
                game.input(Confirm);
                if game.map_open {
                    game.input(Map);
                }
            } else {
                assert!(game.world_map.fort.is_some());
                game.input(Back);
                assert_eq!(game.world_map.campaign.beat().unwrap().site, 12);
                game.input(Confirm);
                nox_steps += walk_to(&mut game, (2, 5));
                press(&mut game, &[Confirm, Confirm, Confirm]);
                assert!(!game.world_map.fort.as_ref().unwrap().gate);
                press(&mut game, &[Confirm, Down, Down, Confirm, Confirm]);
                nox_steps += walk_to(&mut game, (8, 3));
                game.input(Confirm);
                for _ in 0..4 {
                    press(&mut game, &[Confirm, Confirm]);
                }
                assert!(game.battle.is_none());
                assert_eq!(game.goods.hp, MAX_HP);
                assert!(game.world_map.fort.as_ref().unwrap().gate);
                assert!(!game.world_map.campaign.fort_cleared);
                game.input(Confirm);
                walk_to(&mut game, (8, 3));
                game.input(Confirm);
                win_story_battle(&mut game, Enemy::FortEcho);
                assert!(game.world_map.campaign.fort_cleared);
                nox_steps += walk_to(&mut game, (8, 1));
                press(&mut game, &[Confirm, Confirm, Confirm]);
            }
        }
        assert!(matches!(game.scene, Scene::Ending));
        eprintln!("Nox chapter shortest walk via normal controls: {nox_steps} steps");
        let mut resumed = Game::new();
        assert!(resumed.offer_continue(&game.checkpoint().unwrap()));
        resumed.input(Confirm);
        assert!(matches!(resumed.scene, Scene::Ending));
        assert!(resumed.world_map.campaign.complete());
        let mut game = resumed;
        game.input(Confirm);
        assert!(game.map_open);
        assert_eq!(game.world_map.selected, Region::Celestia);
        game.input(Confirm);
        while let Some(beat) = game.world_map.campaign.voyage() {
            if game.world_map.island.is_none() {
                assert_eq!(game.world_map.selected, beat.region);
                game.input(Confirm);
            }
            let island = game.world_map.island.as_ref().unwrap();
            assert_eq!(island.region, beat.region);
            let door = island.settlements()[beat.site].door;
            walk_to(&mut game, door);
            press(&mut game, &[Confirm, Confirm]);
            if beat.consent() {
                press(&mut game, &[Confirm, Confirm]);
                assert!(game.world_map.campaign.sealed_names);
            } else if beat.battle() {
                win_story_battle(&mut game, Enemy::AbyssLord);
            }
        }
        assert!(matches!(game.scene, Scene::Ending));
        assert!(game.world_map.campaign.finished());
        let mut frame = crate::render::Frame::new();
        frame.draw(&game);
        assert!(frame
            .labels
            .iter()
            .any(|label| label.text == "残響のアルカディア　完"));
    }

    #[test]
    fn visible_beast_battle_returns_to_the_same_place_and_preserves_the_prologue() {
        use Key::*;
        let mut game = Game::new();
        game.hp = 11;
        game.memories = 1;
        let wolf = Enemy::Beast(Beast(crate::island::fauna::Species::new(
            crate::world_map::Region::Nox,
            10,
        )));
        press(&mut game, &[Map, Confirm]);
        let island = game.world_map.island.as_mut().unwrap();
        let (id, beast) = island
            .fauna
            .creatures()
            .iter()
            .enumerate()
            .find(|(_, creature)| creature.species.enemy() == Some(wolf))
            .unwrap();
        let position = beast.position;
        (island.x, island.y) = position;
        assert!(game.battle.is_none());
        press(&mut game, &[Confirm]);
        assert_eq!(game.battle.as_ref().unwrap().enemy, wolf);
        press(&mut game, &[Down, Confirm, Confirm, Up, Confirm, Confirm]);
        let island = game.world_map.island.as_ref().unwrap();
        assert_eq!((island.x, island.y), position);
        assert!(!island.fauna.creatures()[id].active);
        assert!(matches!(game.scene, Scene::Title) && game.map_open);
        assert_eq!((game.hp, game.memories), (11, 0));
    }

    #[test]
    fn failed_tutorial_battle_can_be_retried_with_the_same_memories() {
        let mut game = Game::new();
        game.intro = Intro::Memorial;
        (game.x, game.y) = MEMORIAL;
        game.start_lesson(Lesson::Memorial);
        let battle = game.battle.as_mut().unwrap();
        battle.log = Some(["反撃".into(), "倒れた。".into()]);
        game.hp = 0;
        game.memories = 0;
        game.input(Key::Confirm);
        assert_eq!((game.hp, game.memories), (MAX_HP, MEMORIES.len()));
        assert_eq!(game.intro, Intro::Memorial);
        press(&mut game, &[Key::Confirm, Key::Confirm, Key::Confirm]);
        assert_eq!(game.battle.as_ref().unwrap().enemy, Enemy::Echo);
    }
}

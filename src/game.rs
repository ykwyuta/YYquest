use crate::{
    debug::{self, Target},
    dialogue::{Expression, Message, Speaker},
    economy::{Goods, Menu, OptionRow, Page},
    intro::{After, Intro, Lesson, HEARTH, MEETING, MEMORIAL, SLUICE},
    walk::Walk,
    world_map::{Region, WorldMap},
};

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

pub const MAX_HP: u8 = 24;
pub const MEMORIES: [&str; 3] = ["父が剣を研ぐ音", "帰りを待つ家の灯り", "父の最後の言葉"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Beast(pub(crate) crate::island::fauna::Species);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enemy {
    Slime,
    Bat,
    Echo,
    Beast(Beast),
}

impl Enemy {
    pub fn name(self) -> &'static str {
        match self {
            Self::Slime => "霧のスライム",
            Self::Bat => "洞窟コウモリ",
            Self::Echo => "忘却の獣",
            Self::Beast(beast) => beast.0.name(),
        }
    }

    pub(crate) fn hp(self) -> u8 {
        match self {
            Self::Slime => 12,
            Self::Bat => 16,
            Self::Echo => 34,
            Self::Beast(beast) => beast.0.combat().0,
        }
    }

    fn damage(self) -> u8 {
        match self {
            Self::Slime => 3,
            Self::Bat => 4,
            Self::Echo => 6,
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
}

impl Battle {
    pub(crate) fn expedition(&self) -> bool {
        self.island.is_some()
    }

    fn new(enemy: Enemy) -> Self {
        Self {
            enemy,
            hp: enemy.hp(),
            selected: 0,
            log: None,
            island: None,
            lesson: None,
        }
    }
}

#[derive(Clone, Copy)]
struct IslandBattle {
    creature: usize,
    scene: Scene,
    hp: u8,
    memories: usize,
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
        if self.map_open {
            if let Some(town) = &mut self.world_map.town {
                return town.walk.advance(milliseconds);
            }
            return self
                .world_map
                .island
                .as_mut()
                .is_some_and(|island| island.advance(milliseconds));
        }
        if matches!(self.scene, Scene::Field) {
            self.walk.advance(milliseconds) | self.companion_walk.advance(milliseconds)
        } else {
            false
        }
    }

    pub fn debug_back_control(&self) -> bool {
        !self.map_open && (matches!(self.scene, Scene::Debug) || self.debug_menu.previewing)
    }

    pub fn input_code(&mut self, code: u32, repeat: bool) -> bool {
        let Some(key) = Key::from_code(code) else {
            return false;
        };
        let title = !self.map_open && matches!(self.scene, Scene::Title);
        if repeat && (title || key.direction().is_none()) {
            return false;
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

    pub fn input(&mut self, key: Key) {
        if self.economy_menu.is_some() {
            self.economy_input(key);
            return;
        }
        if !self.map_open && matches!(self.scene, Scene::Debug) {
            let key = if key == Key::Map { Key::Back } else { key };
            match self.debug_menu.input(key) {
                debug::Action::Title => *self = Self::new(),
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
            if let Some(town) = &self.world_map.town {
                let index = town.index;
                let previous = self.world_map.town_progress[index];
                let position = town.pos;
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
                    .zip(self.world_map.town_progress[index])
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
            if self.world_map.chapter_input(key) {
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

        if key == Key::Map && self.can_open_map() {
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
            Scene::Title if key == Key::Confirm => self.start_prologue(),
            Scene::Field => self.field_input(key),
            Scene::Battle => self.battle_input(key),
            Scene::Ending if key == Key::Confirm => self.map_open = true,
            Scene::Defeat if key == Key::Confirm => *self = Self::new(),
            _ => {}
        }
    }

    fn start_prologue(&mut self) {
        self.scene = Scene::Field;
        self.message = Some(Message::narration([
            "灰灯りの村に、森から水が届かなくなった。",
            "アレンは父の剣を携え、炉番を訪ねる。",
        ]));
    }

    fn return_to_debug(&mut self) {
        let mut menu = std::mem::take(&mut self.debug_menu);
        menu.previewing = false;
        *self = Self::new();
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
        let (x, y) = (self.x + dx, self.y + dy);
        if !Self::field_walkable((x, y)) {
            return;
        }
        self.walk.step();
        if self.lilith {
            self.companion_facing = self.companion_position.map_or(self.facing, |(cx, cy)| {
                Facing::from_step(self.x - cx, self.y - cy).unwrap_or(self.companion_facing)
            });
            self.companion_position = Some((self.x, self.y));
            self.companion_walk.step();
        }
        (self.x, self.y) = (x, y);
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
                        "東の森で笛が鳴った。水門を見てきて。",
                        "父さんの剣は、水門の留め具にも合うはずだ。",
                    ],
                )
            }
            Intro::Meeting => {
                self.after = After::Continue(Intro::Sluice);
                Message::spoken(
                    Lilith,
                    Speaking,
                    [
                        "村の水が濁る前に、門を持ち上げたい。",
                        "私は門を支える。あなたは霧を追い払って。",
                    ],
                )
            }
            Intro::Sluice => {
                self.after = After::Fight(Lesson::Sluice);
                Message::spoken(
                    Allen,
                    Surprised,
                    [
                        "剣を留め具に差すと、門の奥から魔物が現れた。",
                        "決定で戦う。剣で攻撃し、まもると被害が減る。",
                    ],
                )
            }
            Intro::Memorial => {
                self.after = After::Fight(Lesson::Memorial);
                Message::spoken(
                    Lilith,
                    Resolute,
                    [
                        "あなたの剣と碑が、同じ音で鳴る。",
                        "私が精霊共鳴で霧を裂く。剣で守って。",
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
                        "水路はまた流れているよ。",
                        "炉のそばで休み、アレンの体力が回復した。",
                    ],
                )
            }
            Intro::Return => {
                self.after = After::BeginNox;
                Message::spoken(
                    Hearth,
                    Smile,
                    [
                        "水が戻ったよ。旧宅に父さんの帰還簿がある。",
                        "明るいうちに、二人で調べてみるといい。",
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
                    ["父が残った理由を知りたい。", "一緒に行く？"],
                ));
                self.after = After::Reply;
            }
            After::Reply => {
                self.message = Some(Message::spoken(
                    Speaker::Lilith,
                    Expression::Resolute,
                    [
                        "碑は私の名を呼んだ。答えを探したい。",
                        "うん、私も一緒に行く。",
                    ],
                ));
                self.after = After::Recruit;
            }
            After::Recruit => {
                self.lilith = true;
                self.companion_position = Some(self.intro.waiting_lilith());
                self.intro = Intro::Return;
                self.message = Some(Message::narration([
                    "二人は帰還簿を探すと約束した。",
                    "リリスが仲間になった。炉番に報告しよう。",
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
                    "霧の外まで引っ張ったよ。もう大丈夫。",
                    "体力と記憶が戻った。決定で再び調べられる。",
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
                        "水が流れ出したね。剣の音に、北の碑が応えてる。",
                        "先に石碑へ行って待つね。",
                    ],
                )
            }
            Lesson::Memorial => {
                self.intro = Intro::Choice;
                self.after = After::Ask;
                Message::narration([
                    "碑に、父が三つの灯で船を呼ぶ夜が浮かんだ。",
                    "声「ガウェイン、船は着く。リリス、灯を守れ」",
                ])
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
                Lesson::Sluice => "たたかう・まもる　刻印魔法は記憶を使う",
                Lesson::Memorial => "リリスの精霊共鳴は記憶を消費しない",
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
            memories: self.memories,
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
        self.memories = expedition.memories;
        self.scene = expedition.scene;
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
                let attack = if battle.expedition() {
                    self.goods.attack()
                } else {
                    6
                };
                let (damage, incoming, description) = match battle.selected {
                    0 => (attack, battle.enemy.damage(), "アレンの攻撃！".into()),
                    1 if self.memories > 0 => {
                        self.memories -= 1;
                        (
                            16,
                            battle.enemy.damage(),
                            format!("刻印魔法！「{}」が風化した。", MEMORIES[self.memories]),
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
            let incoming = if battle.expedition() {
                incoming.saturating_sub(self.goods.defense()).max(1)
            } else {
                incoming
            };
            self.hp = self.hp.saturating_sub(incoming);
            if self.hp == 0 && battle.expedition() {
                "アレンは倒れた。決定で港へ戻る。".into()
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

    #[test]
    fn nox_services_collect_brew_craft_equip_and_trade() {
        use crate::{
            economy::{Item, Service},
            town::Action,
        };
        use Key::*;

        fn visit(game: &mut Game, town: usize, place: usize) {
            game.map_open = true;
            game.world_map.selected = Region::Nox;
            if game.world_map.island.is_none() {
                game.input(Confirm);
            }
            let island = game.world_map.island.as_mut().unwrap();
            (island.x, island.y) = island.settlements()[town].door;
            game.input(Confirm);
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

        let mut game = Game::new();
        visit(&mut game, 0, 3);
        press(&mut game, &[Confirm, Confirm, Down, Confirm]);
        assert_eq!(game.goods.count(Item::AshHerb), 3);
        assert_eq!(game.goods.count(Item::AshWood), 2);
        assert!(game.goods.gather_wait(Item::AshHerb) > 0);
        press(&mut game, &[Back, Back, Map]);

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

        visit(&mut game, 5, 3);
        assert!(matches!(Service::at(5, 3), Some(Service::Gather(_))));
        game.input(Confirm);
        press(&mut game, &[Back, Back, Map]);
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
        let mut game = Game::new();
        game.scene = Scene::Field;
        game.lilith = true;
        (game.x, game.y) = (13, 6);
        game.input(Key::Left);
        assert_eq!(game.facing, Facing::West);
        assert_eq!(game.companion_facing, Facing::West);
        assert_eq!(game.companion_position, Some((13, 6)));
        assert!(game.walk.alternate());
        assert!(game.advance_traffic(80));
        assert!(game.advance_traffic(80));
        assert!(!game.walk.alternate());

        game.input(Key::Up);
        assert_eq!((game.x, game.y), (12, 5));
        assert_eq!(game.facing, Facing::North);
        assert_eq!(game.companion_facing, Facing::West);
        assert_eq!(game.companion_position, Some((12, 6)));
    }

    #[test]
    fn tutorial_earns_liliths_company_then_opens_nox() {
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
            game.message.as_ref().unwrap().face.unwrap().speaker,
            Speaker::Allen
        );
        game.input(Confirm);
        assert!(!game.lilith);
        assert_eq!(
            game.message.as_ref().unwrap().face.unwrap().speaker,
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
            "途切れた帰還簿"
        );
    }

    #[test]
    fn nox_chapter_connects_towns_and_the_ruin_to_the_next_voyage() {
        use Key::*;
        let mut game = Game::new();
        game.world_map.begin_nox();
        game.map_open = true;
        if let Some(town) = &game.world_map.town {
            assert_eq!(town.index, 0);
        }
        game.scene = Scene::Field;
        game.intro = Intro::Nox;
        press(&mut game, &[Map, Map, Back]);
        assert!(!game.map_open);
        assert!(game.world_map.island.is_none());
        assert_eq!(game.world_map.campaign.beat().unwrap().site, 0);
        press(&mut game, &[Map, Confirm]);
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
            let island = game.world_map.island.as_mut().unwrap();
            (island.x, island.y) = island.settlements()[index].door;
            game.input(Confirm);
            if let Some(place) = place {
                let town = game.world_map.town.as_mut().unwrap();
                town.pos = town.plan().places[place].at;
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
                assert!(game.world_map.campaign.reading);
                game.input(Back);
                assert_eq!(game.world_map.campaign.beat().unwrap().site, 12);
                press(&mut game, &[Confirm, Confirm]);
            }
        }
        assert!(matches!(game.scene, Scene::Ending));
        game.input(Confirm);
        assert!(game.map_open);
        assert_eq!(game.world_map.selected, Region::Celestia);
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
        for _ in 0..3 {
            press(&mut game, &[Confirm, Confirm]);
        }
        let island = game.world_map.island.as_ref().unwrap();
        assert_eq!((island.x, island.y), position);
        assert!(!island.fauna.creatures()[id].active);
        assert!(matches!(game.scene, Scene::Title) && game.map_open);
        assert_eq!((game.hp, game.memories), (11, 1));
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

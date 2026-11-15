#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Journal,
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
            0x1B | 0x58 => Self::Back,
            _ => return None,
        })
    }

    fn direction(self) -> Option<(i32, i32)> {
        Some(match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            _ => return None,
        })
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
    b"~#ggg.......ggg##~",
    b"~#ggggg##########~",
    b"~~~~~~~~~~~~~~~~~~",
];

pub const MAX_HP: u8 = 24;
pub const MEMORIES: [&str; 3] = ["父が剣を研ぐ音", "帰りを待つ家の灯り", "父の最後の言葉"];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Enemy {
    Slime,
    Bat,
    Echo,
}

impl Enemy {
    pub fn name(self) -> &'static str {
        match self {
            Self::Slime => "霧のスライム",
            Self::Bat => "洞窟コウモリ",
            Self::Echo => "忘却の獣",
        }
    }

    fn hp(self) -> u8 {
        match self {
            Self::Slime => 12,
            Self::Bat => 16,
            Self::Echo => 34,
        }
    }

    fn damage(self) -> u8 {
        match self {
            Self::Slime => 3,
            Self::Bat => 4,
            Self::Echo => 6,
        }
    }
}

pub const COMMANDS: [&str; 4] = ["たたかう", "刻印魔法", "精霊共鳴", "まもる"];

pub struct Battle {
    pub enemy: Enemy,
    pub hp: u8,
    pub selected: usize,
    pub log: Option<[String; 2]>,
}

#[derive(Clone, Copy)]
pub enum Scene {
    Title,
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
    pub journal: bool,
    pub message: Option<[&'static str; 2]>,
    pub battle: Option<Battle>,
    steps: u8,
    next_bat: bool,
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
            journal: false,
            message: None,
            battle: None,
            steps: 0,
            next_bat: false,
        }
    }

    pub fn tile(x: i32, y: i32) -> u8 {
        MAP.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(b'#')
    }

    pub fn input(&mut self, key: Key) {
        if self.journal {
            if matches!(key, Key::Journal | Key::Back | Key::Confirm) {
                self.journal = false;
            }
            return;
        }

        match self.scene {
            Scene::Title if key == Key::Confirm => {
                self.scene = Scene::Field;
                self.message = Some([
                    "新聖律歴30年、黄昏の辺境領ノクス。",
                    "父の錆びた大剣が、森の奥を指している。",
                ]);
            }
            Scene::Field => self.field_input(key),
            Scene::Battle => self.battle_input(key),
            Scene::Defeat | Scene::Ending if key == Key::Confirm => *self = Self::new(),
            _ => {}
        }
    }

    fn field_input(&mut self, key: Key) {
        if self.message.is_some() {
            if matches!(key, Key::Confirm | Key::Back) {
                self.message = None;
            }
            return;
        }
        if key == Key::Journal {
            self.journal = true;
            return;
        }
        let Some((dx, dy)) = key.direction() else {
            return;
        };
        let (x, y) = (self.x + dx, self.y + dy);
        let tile = Self::tile(x, y);
        if matches!(tile, b'#' | b'~') {
            return;
        }
        if self.lilith {
            self.companion_position = Some((self.x, self.y));
        }
        self.x = x;
        self.y = y;
        match tile {
            b'H' => {
                self.hp = MAX_HP;
                self.message = Some(["村の炉辺でひと休みした。", "アレンの体力が回復した。"]);
            }
            b'L' if !self.lilith => {
                self.lilith = true;
                self.hp = MAX_HP;
                self.message = Some([
                    "「わたしはリリス。あとは、思い出せない」",
                    "森の精霊がざわめく。リリスが仲間になった。",
                ]);
            }
            b'R' if self.lilith => self.start_battle(Enemy::Echo),
            b'R' => {
                self.message = Some([
                    "錆びた剣が、苔むす碑に反応する。",
                    "森の奥から、誰かの声が聞こえた。",
                ]);
            }
            b'g' => {
                self.steps += 1;
                if self.steps == 6 {
                    let enemy = if self.next_bat {
                        Enemy::Bat
                    } else {
                        Enemy::Slime
                    };
                    self.next_bat = !self.next_bat;
                    self.start_battle(enemy);
                }
            }
            _ => {}
        }
    }

    fn start_battle(&mut self, enemy: Enemy) {
        self.steps = 0;
        self.battle = Some(Battle {
            enemy,
            hp: enemy.hp(),
            selected: 0,
            log: None,
        });
        self.scene = Scene::Battle;
    }

    fn battle_input(&mut self, key: Key) {
        let battle = self.battle.as_mut().expect("Battle scene has an enemy");
        if battle.log.is_some() {
            if key != Key::Confirm {
                return;
            }
            if self.hp == 0 {
                self.scene = Scene::Defeat;
            } else if battle.hp == 0 {
                self.scene = if battle.enemy == Enemy::Echo {
                    Scene::Ending
                } else {
                    Scene::Field
                };
            } else {
                battle.log = None;
                return;
            }
            self.battle = None;
            return;
        }

        match key {
            Key::Up => battle.selected = (battle.selected + COMMANDS.len() - 1) % COMMANDS.len(),
            Key::Down => battle.selected = (battle.selected + 1) % COMMANDS.len(),
            Key::Confirm => {
                let (damage, incoming, description) = match battle.selected {
                    0 => (6, battle.enemy.damage(), "アレンの攻撃！".into()),
                    1 if self.memories > 0 => {
                        self.memories -= 1;
                        (
                            16,
                            battle.enemy.damage(),
                            format!("刻印魔法！「{}」が風化した。", MEMORIES[self.memories]),
                        )
                    }
                    2 if self.lilith => (9, battle.enemy.damage(), "リリスの精霊共鳴！".into()),
                    3 => (0, 1, "アレンは身を守った。".into()),
                    _ => return,
                };
                battle.hp = battle.hp.saturating_sub(damage);
                let result = if battle.hp == 0 {
                    format!("{}を倒した！", battle.enemy.name())
                } else {
                    self.hp = self.hp.saturating_sub(incoming);
                    format!(
                        "{}の反撃。アレンは {} のダメージ。",
                        battle.enemy.name(),
                        incoming
                    )
                };
                battle.log = Some([description, result]);
            }
            _ => {}
        }
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
    fn prologue_is_reachable_and_magic_erases_a_memory() {
        use Key::*;
        let mut game = Game::new();
        press(&mut game, &[Confirm, Confirm]);
        press(&mut game, &[Right; 10]);
        press(&mut game, &[Down, Down, Confirm]);
        assert!(game.lilith);
        press(&mut game, &[Up, Up, Up, Right, Up]);
        assert!(game.battle.as_ref().unwrap().enemy == Enemy::Echo);
        press(
            &mut game,
            &[
                Down, Confirm, Confirm, Down, Confirm, Confirm, Confirm, Confirm,
            ],
        );
        assert!(matches!(game.scene, Scene::Ending));
        assert_eq!(game.memories, 2);
        assert_eq!(game.hp, 12);
    }

    #[test]
    fn encounters_return_to_field_and_village_heals() {
        use Key::*;
        let mut game = Game::new();
        press(
            &mut game,
            &[
                Confirm, Confirm, Down, Up, Down, Up, Down, Up, Down, Up, Down, Up, Down,
            ],
        );
        assert!(matches!(game.scene, Scene::Battle));
        press(&mut game, &[Confirm, Confirm, Confirm, Confirm]);
        assert!(matches!(game.scene, Scene::Field));
        assert_eq!(game.hp, MAX_HP - 3);
        press(&mut game, &[Up, Up, Up]);
        assert_eq!(game.hp, MAX_HP);
    }
}

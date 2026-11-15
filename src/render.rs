use crate::game::{Enemy, Game, Scene, COMMANDS, MAP, MAX_HP, MEMORIES};

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;

const NIGHT: u32 = 0x111524;
const INK: u32 = 0x181524;
const PANEL: u32 = 0x241f31;
const LINE: u32 = 0x725971;
const IVORY: u32 = 0xf6e9cf;
const GOLD: u32 = 0xf0bd75;
const MIST: u32 = 0xcab9ce;
const MINT: u32 = 0x95d5b4;
const RED: u32 = 0xf48b78;

const ART_SIZE: usize = 64;
const ALLEN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/allen.rgba"));
const LILITH: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/lilith.rgba"));
const SLIME: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/slime.rgba"));
const BAT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bat.rgba"));
const ECHO: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/echo.rgba"));

pub struct Label {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub color: u32,
    pub text: String,
}

pub struct Frame {
    pub pixels: Vec<u32>,
    pub labels: Vec<Label>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT],
            labels: Vec::new(),
        }
    }

    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let left = x.clamp(0, WIDTH as i32) as usize;
        let right = (x + w).clamp(0, WIDTH as i32) as usize;
        let top = y.clamp(0, HEIGHT as i32) as usize;
        let bottom = (y + h).clamp(0, HEIGHT as i32) as usize;
        for row in top..bottom {
            self.pixels[row * WIDTH + left..row * WIDTH + right].fill(color);
        }
    }

    fn label(&mut self, x: i32, y: i32, size: i32, color: u32, text: impl Into<String>) {
        self.labels.push(Label {
            x,
            y,
            size,
            color,
            text: text.into(),
        });
    }

    fn panel(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.rect(x, y, w, h, INK);
        self.rect(x + 1, y + 1, w - 2, h - 2, LINE);
        self.rect(x + 2, y + 2, w - 4, h - 4, PANEL);
    }

    fn art(&mut self, pixels: &[u8], x: i32, y: i32, size: usize) {
        for row in 0..size {
            for col in 0..size {
                let index = (row * ART_SIZE / size * ART_SIZE + col * ART_SIZE / size) * 4;
                let [r, g, b, a] = pixels[index..index + 4].try_into().unwrap();
                if a >= 128 {
                    let rgb = u32::from(r) << 16 | u32::from(g) << 8 | u32::from(b);
                    self.rect(x + col as i32, y + row as i32, 1, 1, rgb);
                }
            }
        }
    }

    pub fn draw(&mut self, game: &Game) {
        self.pixels.fill(NIGHT);
        self.labels.clear();
        match game.scene {
            Scene::Title => self.title(),
            Scene::Field => self.field(game),
            Scene::Battle => self.battle(game),
            Scene::Defeat => self.conclusion(game, false),
            Scene::Ending => self.conclusion(game, true),
        }
    }

    fn landscape(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.rect(x, y, w, h, 0x1e253c);
        self.rect(x, y + h / 3, w, h / 2, 0x3b314b);
        self.rect(x + w - 60, y + 10, 24, 24, 0xe0b98c);
        self.rect(x + w - 57, y + 8, 18, 28, 0xe0b98c);
        for i in 0..w / 16 {
            let height = 17 + (i * 31 % 12);
            self.rect(x + i * 16, y + h - height, 16, height, 0x26354a);
            let height = 7 + (i * 11 % 9);
            self.rect(x + i * 16, y + h - height, 16, height, 0x314e54);
            self.rect(x + i * 16 + 3, y + 7 + (i * 13 % 19), 2, 1, 0x9f8ca7);
        }
    }

    fn title(&mut self) {
        self.landscape(0, 0, 320, 155);
        self.rect(0, 155, 320, 2, LINE);
        self.art(ALLEN, 201, 91, 66);
        self.art(LILITH, 249, 100, 56);
        self.label(22, 30, 10, MINT, "環状列島ヴェルデリア");
        self.label(21, 51, 23, IVORY, "残響のアルカディア");
        self.label(23, 91, 12, MIST, "序章　辺境の残響");
        self.panel(19, 171, 282, 48);
        self.label(33, 179, 14, GOLD, "▶　旅をはじめる");
        self.label(35, 202, 9, MIST, "Enter / Z で決定");
    }

    fn field(&mut self, game: &Game) {
        self.label(16, 7, 12, GOLD, "黄昏の辺境領ノクス");
        self.label(
            222,
            9,
            10,
            IVORY,
            format!("アレン  HP {:02}/{}", game.hp, MAX_HP),
        );
        for (y, row) in MAP.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let tile = if *tile == b'L' && game.lilith {
                    b'g'
                } else {
                    *tile
                };
                self.tile(
                    tile,
                    16 + x as i32 * 16,
                    25 + y as i32 * 16,
                    (x + y * 5) as i32,
                );
            }
        }
        if let Some((x, y)) = game.companion_position {
            self.art(LILITH, 15 + x * 16, 23 + y * 16, 18);
        }
        self.art(ALLEN, 15 + game.x * 16, 23 + game.y * 16, 18);
        if let Some([first, second]) = game.message {
            self.panel(15, 174, 290, 55);
            self.label(24, 180, 11, IVORY, first);
            self.label(24, 196, 11, IVORY, second);
            self.label(249, 213, 9, GOLD, "▼ 決定");
        } else {
            self.panel(15, 191, 290, 37);
            let objective = if game.lilith {
                "リリスと北の石碑へ。"
            } else {
                "東の森へ。村のかまどで体力を回復できます。"
            };
            self.label(24, 196, 11, IVORY, objective);
            self.label(24, 214, 9, MIST, "矢印 / WASD  移動     J  手記");
        }
        if game.journal {
            self.labels.truncate(2);
            self.panel(38, 51, 244, 119);
            self.label(50, 62, 14, GOLD, "アレンの手記");
            for (i, memory) in MEMORIES.iter().enumerate() {
                let (line, color) = if i < game.memories {
                    (*memory, IVORY)
                } else {
                    ("……記憶が風化した", MIST)
                };
                self.label(51, 86 + i as i32 * 17, 11, color, line);
            }
            self.label(200, 151, 9, MINT, "J / Esc  閉じる");
        }
    }

    fn tile(&mut self, tile: u8, x: i32, y: i32, variant: i32) {
        let shift = variant % 5;
        match tile {
            b'~' => {
                self.rect(x, y, 16, 16, 0x283351);
                self.rect(x + 2 + shift, y + 4, 7, 1, 0x465571);
                self.rect(x + 6, y + 10 + shift, 6, 1, 0x394767);
            }
            b'#' => {
                self.rect(x, y, 16, 16, 0x203c3b);
                self.rect(x + 6, y + 10, 4, 6, 0x4a3b44);
                self.rect(x + 2, y + 5, 12, 7, 0x315950);
                self.rect(x + 4, y + 2, 8, 8, 0x3b6e5c);
                self.rect(x + 6, y + 2, 3, 2, 0x619179);
            }
            b'.' => {
                self.rect(x, y, 16, 16, 0x80645a);
                self.rect(x + shift, y + 3, 3, 1, 0xa88770);
                self.rect(x + 9, y + 11, 2 + shift / 3, 1, 0x674e50);
            }
            _ => {
                self.rect(
                    x,
                    y,
                    16,
                    16,
                    if variant % 3 == 0 { 0x4a6d53 } else { 0x4f765b },
                );
                self.rect(x + 2 + shift, y + 4, 3, 1, 0x789675);
                self.rect(x + 10, y + 11, 2, 2, 0x3e624c);
                match tile {
                    b'H' => {
                        self.rect(x + 3, y + 8, 11, 6, 0x8e7168);
                        self.rect(x + 2, y + 5, 13, 4, 0xba765f);
                        self.rect(x + 5, y + 3, 7, 2, 0xd89578);
                        self.rect(x + 7, y + 10, 3, 4, GOLD);
                    }
                    b'R' => {
                        self.rect(x + 4, y + 3, 8, 12, 0x697083);
                        self.rect(x + 5, y + 2, 6, 2, 0xa29cae);
                        self.rect(x + 7, y + 6, 2, 5, GOLD);
                    }
                    b'L' => self.art(LILITH, x - 1, y - 2, 18),
                    _ => {}
                }
            }
        }
    }

    fn battle(&mut self, game: &Game) {
        let battle = game.battle.as_ref().expect("Battle scene has an enemy");
        let art = match battle.enemy {
            Enemy::Slime => SLIME,
            Enemy::Bat => BAT,
            Enemy::Echo => ECHO,
        };
        self.landscape(0, 23, 320, 125);
        self.rect(114, 122, 92, 3, 0x283449);
        self.art(art, 112, 34, 96);
        self.label(16, 6, 12, GOLD, battle.enemy.name());
        self.label(
            207,
            9,
            10,
            IVORY,
            format!("アレン  HP {:02}/{}", game.hp, MAX_HP),
        );
        self.label(
            18,
            136,
            9,
            MIST,
            format!("記憶  {}/{}", game.memories, MEMORIES.len()),
        );
        self.panel(15, 155, 290, 74);
        if let Some([first, second]) = &battle.log {
            self.label(24, 166, 12, IVORY, first.as_str());
            self.label(24, 185, 11, IVORY, second.as_str());
            self.label(248, 209, 9, GOLD, "▼ 決定");
        } else {
            for (i, command) in COMMANDS.iter().enumerate() {
                let enabled = match i {
                    1 => game.memories > 0,
                    2 => game.lilith,
                    _ => true,
                };
                let color = if !enabled {
                    0x9d8fa1
                } else if i == battle.selected {
                    GOLD
                } else {
                    IVORY
                };
                let marker = if i == battle.selected { "▶" } else { "　" };
                self.label(
                    22,
                    161 + i as i32 * 15,
                    10,
                    color,
                    format!("{marker} {command}"),
                );
            }
            let help = match battle.selected {
                1 if game.memories > 0 => "大きな一撃。記憶をひとつ失う。",
                1 => "燃やせる記憶が残っていない。",
                2 if game.lilith => "リリスが森の精霊を呼ぶ。",
                2 => "リリスが仲間になると使える。",
                3 => "敵の反撃を弱める。",
                _ => "錆びた大剣で斬りつける。",
            };
            self.rect(115, 166, 1, 52, LINE);
            self.label(126, 173, 10, MINT, help);
            self.label(126, 206, 9, MIST, "↑ ↓ 選ぶ     Enter / Z 決定");
        }
    }

    fn conclusion(&mut self, game: &Game, won: bool) {
        self.landscape(0, 0, 320, 144);
        let (title, color, first, second) = if won {
            self.art(ALLEN, 109, 73, 56);
            self.art(LILITH, 155, 74, 56);
            (
                "序章　完",
                GOLD,
                "石碑に残るのは、かつての戦いの残響。",
                "「この先で、わたしの記憶を探してもいい？」",
            )
        } else {
            (
                "旅の終わり",
                RED,
                "アレンは霧の中に倒れた。",
                "森の奥で、誰かが名を呼んでいる。",
            )
        };
        self.label(20, 31, 23, color, title);
        self.panel(19, 151, 282, 76);
        self.label(29, 159, 11, IVORY, first);
        self.label(29, 176, 11, IVORY, second);
        if won {
            self.label(
                29,
                205,
                10,
                MIST,
                format!("残った記憶  {}/{}", game.memories, MEMORIES.len()),
            );
        }
        self.label(180, 205, 10, GOLD, "Enter  題名へ");
    }
}

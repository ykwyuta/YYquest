use crate::game::{Enemy, Game, Scene, COMMANDS, MAX_HP, MEMORIES};

mod debug;
mod island;
mod map_art;
mod sprite;
mod town;
mod traffic;
mod world_map;

use crate::art_sizes;
use sprite::Sprite;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub(crate) const DISPLAY_SCALE: usize = 2;
pub const BUFFER_WIDTH: usize = WIDTH * DISPLAY_SCALE;
pub const BUFFER_HEIGHT: usize = HEIGHT * DISPLAY_SCALE;

const NIGHT: u32 = 0x111524;
const INK: u32 = 0x181524;
const PANEL: u32 = 0x241f31;
const LINE: u32 = 0x725971;
const IVORY: u32 = 0xf6e9cf;
const GOLD: u32 = 0xf0bd75;
const MIST: u32 = 0xcab9ce;
const MINT: u32 = 0x95d5b4;
const RED: u32 = 0xf48b78;

const TITLE: &[u8; BUFFER_WIDTH * BUFFER_HEIGHT * 3] =
    include_bytes!(concat!(env!("OUT_DIR"), "/title.rgb"));
const _: () = {
    assert!(art_sizes::TITLE_WIDTH == BUFFER_WIDTH);
    assert!(art_sizes::TITLE_HEIGHT == BUFFER_HEIGHT);
};

const ALLEN: Sprite = Sprite::new(
    include_bytes!(concat!(env!("OUT_DIR"), "/allen.rgba")),
    art_sizes::PORTRAIT,
);
const LILITH: Sprite = Sprite::new(
    include_bytes!(concat!(env!("OUT_DIR"), "/lilith.rgba")),
    art_sizes::PORTRAIT,
);
const SLIME: Sprite = Sprite::new(
    include_bytes!(concat!(env!("OUT_DIR"), "/slime.rgba")),
    art_sizes::PORTRAIT,
);
const BAT: Sprite = Sprite::new(
    include_bytes!(concat!(env!("OUT_DIR"), "/bat.rgba")),
    art_sizes::PORTRAIT,
);
const ECHO: Sprite = Sprite::new(
    include_bytes!(concat!(env!("OUT_DIR"), "/echo.rgba")),
    art_sizes::PORTRAIT,
);

pub struct Label {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub color: u32,
    pub text: String,
    pub serif: bool,
}

pub struct Frame {
    pub pixels: Vec<u32>,
    pub labels: Vec<Label>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; BUFFER_WIDTH * BUFFER_HEIGHT],
            labels: Vec::new(),
        }
    }

    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let left = x.clamp(0, WIDTH as i32) as usize;
        let right = (x + w).clamp(0, WIDTH as i32) as usize;
        let top = y.clamp(0, HEIGHT as i32) as usize;
        let bottom = (y + h).clamp(0, HEIGHT as i32) as usize;
        let (left, right) = (left * DISPLAY_SCALE, right * DISPLAY_SCALE);
        for row in top * DISPLAY_SCALE..bottom * DISPLAY_SCALE {
            self.pixels[row * BUFFER_WIDTH + left..row * BUFFER_WIDTH + right].fill(color);
        }
    }

    fn label(&mut self, x: i32, y: i32, size: i32, color: u32, text: impl Into<String>) {
        self.labels.push(Label {
            x,
            y,
            size,
            color,
            text: text.into(),
            serif: false,
        });
    }

    fn panel(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.rect(x, y, w, h, INK);
        self.rect(x + 1, y + 1, w - 2, h - 2, LINE);
        self.rect(x + 2, y + 2, w - 4, h - 4, PANEL);
    }

    fn art(&mut self, sprite: Sprite, x: i32, y: i32, size: usize) {
        self.art_clipped(sprite, x, y, size, [0, 0, WIDTH as i32, HEIGHT as i32]);
    }

    fn art_clipped(&mut self, sprite: Sprite, x: i32, y: i32, size: usize, clip: [i32; 4]) {
        self.art_oriented(sprite, x, y, size, clip, false);
    }

    fn art_oriented(
        &mut self,
        sprite: Sprite,
        x: i32,
        y: i32,
        size: usize,
        clip: [i32; 4],
        flip: bool,
    ) {
        let [left, top, right, bottom] = clip.map(|edge| edge * DISPLAY_SCALE as i32);
        let (x, y) = (x * DISPLAY_SCALE as i32, y * DISPLAY_SCALE as i32);
        let target = size * DISPLAY_SCALE;
        for dy in y.max(top)..(y + target as i32).min(bottom) {
            let row = (dy - y) as usize;
            for dx in x.max(left)..(x + target as i32).min(right) {
                let col = (dx - x) as usize;
                let col = if flip { target - 1 - col } else { col };
                let sample = sprite.sample(col, row, target);
                if sample[3] != 0 {
                    let dest = &mut self.pixels[dy as usize * BUFFER_WIDTH + dx as usize];
                    *dest = sprite::over_rgb(sample, *dest);
                }
            }
        }
    }

    pub fn draw(&mut self, game: &Game) {
        self.pixels.fill(NIGHT);
        self.labels.clear();
        if game.map_open {
            self.draw_world_map(&game.world_map);
            return;
        }
        match game.scene {
            Scene::Title => self.title(),
            Scene::Debug => self.draw_debug_menu(&game.debug_menu),
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
        for (dest, [red, green, blue]) in self.pixels.iter_mut().zip(TITLE.as_chunks::<3>().0) {
            *dest = u32::from_be_bytes([0, *red, *green, *blue]);
        }
        let bronze = 0x9e805b;
        self.rect(8, 8, 304, 1, bronze);
        self.rect(8, 231, 304, 1, bronze);
        for x in [8, 311] {
            self.rect(x, 8, 1, 13, bronze);
            self.rect(x, 219, 1, 13, bronze);
        }
        for row in 0_i32..7 {
            let half = 3 - (row - 3).abs();
            self.rect(24 - half, 26 + row, half * 2 + 1, 1, 0xd7b681);
        }
        self.label(34, 24, 9, 0xdeccaa, "環状列島ヴェルデリア");
        self.label(19, 45, 29, 0xf1ddaf, "残響の");
        self.labels.last_mut().unwrap().serif = true;
        self.label(17, 76, 23, 0xffebc4, "アルカディア");
        self.labels.last_mut().unwrap().serif = true;
        self.rect(21, 112, 143, 1, bronze);
        self.rect(21, 111, 26, 3, 0xc09a6a);
        self.label(22, 123, 11, 0xe4cda4, "序章　辺境の残響");
        self.label(21, 149, 13, 0xffdc9e, "▶　旅をはじめる");
        self.label(23, 173, 9, 0xe2d6c2, "Enter / Z で決定");
        self.label(23, 188, 9, 0xe2d6c2, "M  ワールドマップ");
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
        self.draw_prologue_landscape(game.lilith);
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
            self.label(
                24,
                214,
                9,
                MIST,
                "矢印 / WASD  移動     J  手記     M  地図",
            );
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

    fn battle(&mut self, game: &Game) {
        let battle = game.battle.as_ref().expect("Battle scene has an enemy");
        let art = match battle.enemy {
            Enemy::Slime => SLIME,
            Enemy::Bat => BAT,
            Enemy::Echo => ECHO,
            Enemy::Beast(beast) => map_art::fauna_sprite(beast.0),
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
            self.label(180, 189, 10, MINT, "M  ワールドマップ");
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

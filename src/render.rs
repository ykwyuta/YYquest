use crate::game::{Enemy, Facing, Game, Scene, Tactic, COMMANDS, MAX_HP, MEMORIES};
use crate::walk::Pose;

mod debug;
mod economy;
mod fort;
mod guide;
mod island;
mod map_art;
mod minimap;
mod portrait;
mod sprite;
mod town;
mod traffic;
mod view;
mod world_map;

use crate::art_sizes;
use sprite::Sprite;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub(crate) const DISPLAY_SCALE: usize = 3;
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

fn allen(facing: Facing, pose: Pose) -> Sprite {
    character_sprite(8 * pose as usize + facing as usize)
}

fn lilith(facing: Facing, pose: Pose) -> Sprite {
    character_sprite(8 * pose as usize + 4 + facing as usize)
}

fn character_sprite(index: usize) -> Sprite {
    const SIDE: usize = art_sizes::PORTRAIT;
    const STRIDE: usize = SIDE * SIDE * 4;
    const ATLAS: &[u8; 32 * STRIDE] =
        include_bytes!(concat!(env!("OUT_DIR"), "/prologue-characters.rgba"));
    Sprite::new(&ATLAS[index * STRIDE..(index + 1) * STRIDE], SIDE)
}
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
    guide: Option<guide::Route>,
    minimap: Option<minimap::Terrain>,
    minimap_inset: i32,
    width: usize,
    exploration_width: usize,
    terrain_cache: map_art::TerrainCache,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; BUFFER_WIDTH * BUFFER_HEIGHT],
            labels: Vec::new(),
            guide: None,
            minimap: None,
            minimap_inset: 0,
            width: WIDTH,
            exploration_width: WIDTH,
            terrain_cache: map_art::TerrainCache::new(),
        }
    }

    pub(crate) fn buffer_width(&self) -> usize {
        self.width * DISPLAY_SCALE
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn set_exploration_view(&mut self, width: usize, minimap_inset: usize) -> bool {
        let width = width.clamp(WIDTH, 560);
        let minimap_inset = minimap_inset.min(width / 2) as i32;
        let changed = width != self.exploration_width || minimap_inset != self.minimap_inset;
        self.exploration_width = width;
        self.minimap_inset = minimap_inset;
        changed
    }

    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let left = x.clamp(0, self.width as i32) as usize;
        let right = (x + w).clamp(0, self.width as i32) as usize;
        let top = y.clamp(0, HEIGHT as i32) as usize;
        let bottom = (y + h).clamp(0, HEIGHT as i32) as usize;
        let (left, right) = (left * DISPLAY_SCALE, right * DISPLAY_SCALE);
        let stride = self.buffer_width();
        for row in top * DISPLAY_SCALE..bottom * DISPLAY_SCALE {
            self.pixels[row * stride + left..row * stride + right].fill(color);
        }
    }

    fn rect_clipped(&mut self, x: i32, y: i32, w: i32, h: i32, clip: [i32; 4], color: u32) {
        let [left, top, right, bottom] = clip;
        let (start_x, start_y) = (x.max(left), y.max(top));
        let (end_x, end_y) = ((x + w).min(right), (y + h).min(bottom));
        if start_x < end_x && start_y < end_y {
            self.rect(start_x, start_y, end_x - start_x, end_y - start_y, color);
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
        self.art_clipped(sprite, x, y, size, [0, 0, self.width as i32, HEIGHT as i32]);
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
        self.art_pixels(
            sprite,
            (x * DISPLAY_SCALE as i32, y * DISPLAY_SCALE as i32),
            size * DISPLAY_SCALE,
            clip,
            flip,
        );
    }

    fn art_pixels(
        &mut self,
        sprite: Sprite,
        (x, y): (i32, i32),
        target: usize,
        clip: [i32; 4],
        flip: bool,
    ) {
        let [left, top, right, bottom] = clip.map(|edge| edge * DISPLAY_SCALE as i32);
        let stride = self.buffer_width();
        for dy in y.max(top)..(y + target as i32).min(bottom) {
            let row = (dy - y) as usize;
            for dx in x.max(left)..(x + target as i32).min(right) {
                let col = (dx - x) as usize;
                let col = if flip { target - 1 - col } else { col };
                let sample = sprite.sample(col, row, target);
                if sample[3] != 0 {
                    let dest = &mut self.pixels[dy as usize * stride + dx as usize];
                    *dest = sprite::over_rgb(sample, *dest);
                }
            }
        }
    }

    pub fn draw(&mut self, game: &Game) {
        let exploring = if game.map_open {
            game.world_map.island.is_some()
        } else {
            matches!(game.scene, Scene::Field)
        };
        self.width = if exploring {
            self.exploration_width
        } else {
            WIDTH
        };
        self.pixels
            .resize(self.buffer_width() * BUFFER_HEIGHT, NIGHT);
        self.pixels.fill(NIGHT);
        self.labels.clear();
        if game.map_open {
            self.draw_world_map(&game.world_map, &game.goods, game.economy_menu.is_none());
            if let Some(menu) = &game.economy_menu {
                self.draw_economy(menu, &game.goods, game.goods.hp);
            }
            return;
        }
        match game.scene {
            Scene::Title => self.title(game),
            Scene::Debug => self.draw_debug_menu(&game.debug_menu),
            Scene::Field => self.field(game),
            Scene::Battle => self.battle(game),
            Scene::Defeat => self.conclusion(game, false),
            Scene::Ending => self.conclusion(game, true),
        }
        if let Some(menu) = &game.economy_menu {
            self.draw_economy(menu, &game.goods, game.hp);
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

    fn title(&mut self, game: &Game) {
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
        self.label(22, 123, 11, 0xe4cda4, "第一章　帰らなかった灯");
        if game.confirm_overwrite {
            self.panel(16, 140, 208, 68);
            self.label(23, 150, 10, GOLD, "前の冒険の記録を上書きしますか？");
            self.label(23, 178, 9, IVORY, "決定　新しく始める　　Esc　戻る");
        } else if game.has_continue() {
            let marker = |selected| if selected { "▶" } else { "　" };
            self.label(
                21,
                147,
                12,
                GOLD,
                format!("{}　つづきから", marker(!game.title_new_game)),
            );
            self.label(
                21,
                166,
                12,
                GOLD,
                format!("{}　旅をはじめる", marker(game.title_new_game)),
            );
            self.label(23, 188, 9, IVORY, "↑ ↓ 選ぶ　 Enter / Z 決定");
            self.label(23, 204, 9, IVORY, "M  ワールドマップ");
        } else {
            self.label(21, 149, 13, 0xffdc9e, "▶　旅をはじめる");
            self.label(23, 173, 9, 0xe2d6c2, "Enter / Z で決定");
            self.label(23, 188, 9, 0xe2d6c2, "M  ワールドマップ");
        }
    }

    fn field(&mut self, game: &Game) {
        let map_size = crate::game::PROLOGUE_SIZE;
        let player = (game.x, game.y);
        let view = view::View::following(self.width as i32, player, map_size, &game.walk);
        self.label(view.left, 7, 12, GOLD, "黄昏の辺境領ノクス");
        self.label(
            view.clip[2] - 82,
            9,
            10,
            IVORY,
            format!("アレン  HP {:02}/{}", game.hp, MAX_HP),
        );
        self.draw_prologue_landscape(view);
        if let Some(at) = game.companion_position {
            let (x, y) = view.walking_feet(at, &game.companion_walk);
            self.art_clipped(
                lilith(game.companion_facing, game.companion_walk.pose()),
                x - 14,
                y - 28 - game.companion_walk.lift(),
                28,
                view.clip,
            );
        } else if !game.lilith {
            let at = game.intro.waiting_lilith();
            let (x, y) = view.feet(at);
            let offset = if at == player { 12 } else { 0 };
            self.art_clipped(
                lilith(Facing::South, Pose::Standing),
                x - 14 + offset,
                y - 28,
                28,
                view.clip,
            );
        }
        self.draw_explorer(
            allen(game.facing, game.walk.pose()),
            view,
            player,
            &game.walk,
        );
        if game.message.is_none() && !game.journal && game.intro != crate::intro::Intro::Nox {
            let target = game.intro.destination().0;
            self.draw_guide(
                guide::Guidance {
                    area: guide::Area::Prologue,
                    from: (game.x, game.y),
                    to: target,
                    size: map_size,
                },
                Game::field_walkable,
                |(x, y)| Game::tile(x, y) == b'.',
                |at| view.feet(at),
                view.clip,
            );
        }
        if game.message.is_none() && !game.journal && game.economy_menu.is_none() {
            self.draw_prologue_minimap(view, game);
        }
        if let Some(message) = game.message {
            let top = if message.delivery.is_speech() {
                143
            } else {
                159
            };
            self.panel(view.left - 1, top, view.width() + 2, 237 - top);
            self.dialogue_content(
                (view.left + 9, top + 8),
                view.width() - 18,
                &message.lines,
                message.delivery,
            );
            self.label(view.clip[0] + 12, 225, 9, GOLD, "▼ 決定");
        } else {
            self.panel(view.left - 1, 191, view.width() + 2, 46);
            self.label(24, 196, 11, IVORY, game.intro.destination().1);
            self.label(
                24,
                214,
                9,
                MIST,
                "矢印　移動　　決定　会話・調査　　J　手記",
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
            Enemy::Echo | Enemy::FortEcho | Enemy::AbyssLord => ECHO,
            Enemy::Beast(beast) => map_art::fauna_sprite(beast.0),
        };
        self.landscape(0, 23, 320, 125);
        self.rect(114, 122, 92, 3, 0x283449);
        self.art(art, 112, 34, 96);
        let enemy_name = battle.enemy.name();
        let name_size = if enemy_name.chars().count() > 9 {
            9
        } else {
            11
        };
        self.label(16, 5, name_size, GOLD, enemy_name);
        let enemy_max = battle.enemy.hp();
        self.label(125, 7, 9, IVORY, format!("HP {}/{enemy_max}", battle.hp));
        self.label(
            207,
            7,
            10,
            IVORY,
            format!("アレン  HP {:02}/{}", game.hp, MAX_HP),
        );
        self.hp_gauge(16, 18, 152, battle.hp, enemy_max, RED);
        let color = if game.hp * 4 <= MAX_HP { RED } else { MINT };
        self.hp_gauge(207, 18, 96, game.hp, MAX_HP, color);
        self.label(
            18,
            136,
            9,
            MIST,
            format!("記憶  {}/{}", game.memories, MEMORIES.len()),
        );
        if let Some(tip) = game.lesson_tip() {
            self.label(89, 136, 9, MINT, tip);
        } else if battle.enemy.tactic() == Tactic::Charge {
            let tip = if battle.charged {
                "大技の構え！　精霊共鳴か防御で防ごう"
            } else {
                "敵は様子をうかがっている"
            };
            self.label(89, 136, 9, MINT, tip);
        } else if battle.enemy.tactic() == Tactic::Shell {
            self.label(89, 136, 9, MINT, "硬い殻が剣を弾く！　魔法や精霊共鳴が有効");
        }
        self.panel(15, 155, 290, 74);
        if let Some([first, second]) = &battle.log {
            self.label(24, 166, 12, IVORY, first.as_str());
            self.label(24, 185, 11, IVORY, second.as_str());
            self.label(24, 209, 9, GOLD, "▼ 決定");
        } else {
            let extra = battle.expedition().then_some("道具");
            let spacing = if battle.expedition() { 13 } else { 15 };
            for (i, command) in COMMANDS.into_iter().chain(extra).enumerate() {
                let enabled = match i {
                    1 => game.memories > 0,
                    2 => game.can_resonate(),
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
                    161 + i as i32 * spacing,
                    10,
                    color,
                    format!("{marker} {command}"),
                );
            }
            let help = match battle.selected {
                1 if game.memories > 0 => "大きな一撃。記憶をひとつ失う。",
                1 => "使える記憶がもう残っていない。",
                2 if game.can_resonate() => "リリスが森の精霊を呼ぶ。",
                2 => "リリスが仲間になると使える。",
                3 => "敵の反撃を弱める。",
                4 => "回復薬や魔道具を使う。",
                _ if battle.expedition() => "装備した武器で攻撃する。",
                _ => "錆びた大剣で斬りつける。",
            };
            self.rect(115, 166, 1, 52, LINE);
            self.label(126, 173, 10, MINT, help);
            self.label(126, 206, 9, MIST, "↑ ↓ 選ぶ     Enter / Z 決定");
        }
    }

    fn hp_gauge(&mut self, x: i32, y: i32, width: i32, hp: u8, max: u8, color: u32) {
        let filled = (width - 2) * i32::from(hp.min(max)) / i32::from(max);
        self.rect(x, y, width, 5, LINE);
        self.rect(x + 1, y + 1, width - 2, 3, INK);
        self.rect(x + 1, y + 1, filled, 3, color);
    }

    fn conclusion(&mut self, game: &Game, won: bool) {
        self.landscape(0, 0, 320, 144);
        let finished = game.world_map.campaign.finished();
        let (title, color, first, second) = if won {
            self.art(allen(Facing::South, Pose::Standing), 109, 73, 56);
            self.art(lilith(Facing::South, Pose::Standing), 155, 74, 56);
            if finished {
                let allen = if game.memories == MEMORIES.len() {
                    "アレンは父の思いと炉番たちの名を碑に刻んだ。"
                } else {
                    "アレンは父の働きと炉番たちの名を碑に刻んだ。"
                };
                (
                    "残響のアルカディア　完",
                    GOLD,
                    allen,
                    "リリスは旅の日付を自分の手記の最初に書いた。",
                )
            } else {
                (
                    "第一章　ノクス　完",
                    GOLD,
                    "父が残した救助の記録は霧灯の港へ届いた。",
                    "二人は次の島・聖律皇国セレスティアへ向かう。",
                )
            }
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
        self.label(
            180,
            205,
            10,
            GOLD,
            if finished {
                "決定　列島を巡る"
            } else if won {
                "決定　次の航路へ"
            } else {
                "Enter　題名へ"
            },
        );
    }
}

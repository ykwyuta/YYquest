use super::{allen, Frame, GOLD, IVORY, LINE, MINT, MIST, RED};
use crate::{
    campaign::ChapterBeat,
    fort::{self, Fort, Panel},
};

impl Frame {
    pub(super) fn draw_fort(&mut self, fort: &Fort, cleared: bool, beat: &ChapterBeat) {
        let left = (self.width as i32 - 286) / 2;
        self.label(left, 7, 12, GOLD, "天墜ち砦跡　封印庫");
        self.label(left + 205, 9, 9, MIST, "M / Esc  外へ");
        for (row, tiles) in fort::MAP.iter().enumerate() {
            for (col, tile) in tiles.iter().copied().enumerate() {
                let x = left + col as i32 * 22;
                let y = 26 + row as i32 * 22;
                let color = match tile {
                    b'#' => 0x35323b,
                    b'G' if !fort.gate => 0x77614c,
                    _ => 0x4b4248 + if (col + row) % 2 == 0 { 0x070708 } else { 0 },
                };
                self.rect(x, y, 22, 22, color);
                self.rect(x, y + 21, 22, 1, LINE);
                let mark = match tile {
                    b'L' => Some((GOLD, "灯")),
                    b'R' => Some((IVORY, "書")),
                    b'B' if !cleared => Some((RED, "霧")),
                    _ if (col as i32, row as i32) == fort::ENTRY => Some((MIST, "口")),
                    _ => None,
                };
                if let Some((color, text)) = mark {
                    self.label(x + 5, y + 5, 12, color, text);
                }
            }
        }
        self.art(
            allen(fort.facing, super::Pose::Standing),
            left + fort.pos.0 * 22 - 1,
            23 + fort.pos.1 * 22,
            24,
        );
        self.panel(left, 188, 286, 43);
        let goal = if cleared {
            "奥の記録棚を調べ、救助の証言を書き写す。"
        } else if fort.gate {
            "霧の奥へ進み、記録棚への道を開く。"
        } else {
            "拓本を手に、入口の近くにある信号灯を調べる。"
        };
        self.label(left + 9, 195, 10, MINT, goal);
        self.label(left + 9, 214, 9, MIST, "決定　調べる　　J　持ち物");
        match fort.panel {
            Panel::Walk => {}
            Panel::Lamps(selected) => {
                self.panel(left + 10, 35, 266, 139);
                self.label(left + 22, 44, 12, GOLD, "救助船を呼んだのは、いくつの灯？");
                self.label(
                    left + 22,
                    65,
                    10,
                    IVORY,
                    "拓本には、削られた記録の脇に灯の印が残る。",
                );
                for (index, label) in ["一つ灯す", "二つ灯す", "三つ灯す"].into_iter().enumerate()
                {
                    let marker = if selected == index { "▶" } else { "　" };
                    self.label(
                        left + 22,
                        88 + 19 * index as i32,
                        11,
                        IVORY,
                        format!("{marker} {label}"),
                    );
                }
                self.label(left + 163, 151, 9, MIST, "↑ ↓ 選ぶ　決定");
            }
            Panel::Return(selected) => {
                self.panel(left + 10, 38, 266, 138);
                self.label(left + 22, 49, 12, GOLD, "写しを霧灯の港へ届ける");
                self.label(
                    left + 22,
                    72,
                    10,
                    IVORY,
                    "三灯を見た港の救難艇が、砦の浜へ来た。",
                );
                for (index, label) in ["救難艇で港へ戻る", "島の街道を歩いて戻る"]
                    .into_iter()
                    .enumerate()
                {
                    let marker = if selected == index { "▶" } else { "　" };
                    self.label(
                        left + 22,
                        99 + 22 * index as i32,
                        11,
                        IVORY,
                        format!("{marker} {label}"),
                    );
                }
                self.label(left + 163, 149, 9, MIST, "↑ ↓ 選ぶ　決定");
            }
            panel => {
                self.panel(left, 38, 286, 140);
                if matches!(panel, Panel::Record) {
                    self.label(left + 10, 48, 12, GOLD, beat.title);
                    self.dialogue_content((left + 10, 67), 266, &[beat.text], beat.delivery);
                } else {
                    let text = match panel {
                        Panel::Wrong => "扉は動かない。港の信号台で聞いた合図を思い出そう。",
                        Panel::Opened => "三つの灯がともり、拓本と同じ印のある扉が開いた。",
                        Panel::Recovered => "リリスが入口まで運び、手当てをした。扉は開いている。",
                        _ => unreachable!(),
                    };
                    self.dialogue_content(
                        (left + 10, 57),
                        266,
                        &[text],
                        crate::dialogue::Delivery::Scene,
                    );
                }
                self.label(left + 190, 153, 9, GOLD, "▼ 決定");
            }
        }
    }
}

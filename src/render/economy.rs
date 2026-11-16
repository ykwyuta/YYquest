use super::{Frame, GOLD, IVORY, LINE, MINT, MIST};
use crate::{
    economy::{Goods, Menu, OptionRow, Page},
    game::MAX_HP,
};

impl Frame {
    pub(super) fn draw_economy(&mut self, menu: &Menu, goods: &Goods, hp: u8) {
        let width = 280;
        let x = (self.width as i32 - width) / 2;
        self.labels.clear();
        self.panel(x, 17, width, 200);
        let title = match menu.page {
            Page::Bag => "荷袋",
            Page::Battle => "戦闘道具",
            Page::Shop => "よろず商",
            Page::Buy => "品物を買う",
            Page::Sell => "素材を売る",
            Page::Brew => "炊事場の調薬",
            Page::Forge => "ドワーフ鍛冶師の工房",
            Page::Rest(_) => "休息・手当て",
            Page::Gather(_) => "採集できる素材",
        };
        self.label(x + 12, 25, 12, GOLD, title);
        self.label(
            x + 165,
            27,
            9,
            MINT,
            format!("HP {hp}/{MAX_HP}　{}銀環", goods.coins),
        );
        self.rect(x + 12, 43, width - 24, 1, LINE);

        const VISIBLE: usize = 6;
        let rows = menu.rows(goods);
        let first = menu.selected.saturating_sub(VISIBLE - 1);
        for (index, row) in rows.iter().enumerate().skip(first).take(VISIBLE) {
            let label = match row {
                OptionRow::BuyList => "品物を買う".into(),
                OptionRow::SellList => "素材を売る".into(),
                OptionRow::Item(item) => {
                    let suffix = if goods.equipped(*item) {
                        "　装備中"
                    } else {
                        ""
                    };
                    format!("{} ×{}{}", item.name(), goods.count(*item), suffix)
                }
                OptionRow::Buy(item) => format!("{}　{}銀環", item.name(), item.data().2),
                OptionRow::Sell(item) => format!(
                    "{} ×{}　{}銀環",
                    item.name(),
                    goods.count(*item),
                    item.data().2 / 2
                ),
                OptionRow::Craft(recipe) => format!("{}　{}銀環", recipe.output.name(), recipe.fee),
                OptionRow::Rest(cost) => format!("体力を全回復　{cost}銀環"),
                OptionRow::Gather(item, quantity) => {
                    let wait = goods.gather_wait(*item);
                    if wait == 0 {
                        format!("{} ×{quantity}", item.name())
                    } else {
                        format!("{}　再採集まで {wait} 歩", item.name())
                    }
                }
            };
            let selected = index == menu.selected;
            let mark = if selected { "▶" } else { "　" };
            self.label(
                x + 12,
                52 + 14 * (index - first) as i32,
                10,
                if selected { MINT } else { IVORY },
                format!("{mark} {label}"),
            );
        }
        self.rect(x + 12, 139, width - 24, 1, LINE);
        let mut y = 146;
        match rows.get(menu.selected) {
            Some(OptionRow::Craft(recipe)) => {
                for &(item, required) in recipe.needs {
                    let held = goods.count(item);
                    self.label(
                        x + 12,
                        y,
                        9,
                        if held >= required { MINT } else { MIST },
                        format!("{}　所持 {held} / 必要 {required}", item.name()),
                    );
                    y += 11;
                }
                self.label(x + 12, y, 9, IVORY, recipe.output.description());
            }
            Some(OptionRow::Item(item) | OptionRow::Buy(item)) => {
                self.label(x + 12, y, 9, IVORY, item.description());
            }
            Some(OptionRow::Gather(..)) => {
                self.label(
                    x + 12,
                    y,
                    9,
                    IVORY,
                    "採集した素材は、街道を歩くと補充される。",
                );
            }
            None => self.label(x + 12, 54, 10, MIST, "ここで使える品がない。"),
            _ => {}
        }
        self.label(x + 12, 183, 9, GOLD, &menu.note);
        self.label(x + 12, 201, 9, MIST, "上下 選ぶ　決定 実行　← 戻る");
        if !matches!(menu.page, Page::Bag | Page::Battle) {
            self.label(x + 204, 201, 9, MINT, "→ 荷袋");
        }
    }
}

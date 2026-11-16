use super::{Frame, Sprite, GOLD, INK, IVORY, LINE, MINT, MIST};
use crate::{
    art_sizes,
    economy::{Goods, Item, Menu, OptionRow, Page},
    game::MAX_HP,
};

pub(super) fn item_icon(item: Item) -> Sprite {
    const SIDE: usize = art_sizes::ITEM;
    const STRIDE: usize = SIDE * SIDE * 4;
    const ICONS: &[u8; Item::ALL.len() * STRIDE] =
        include_bytes!(concat!(env!("OUT_DIR"), "/items.rgba"));
    let offset = item as usize * STRIDE;
    Sprite::new(&ICONS[offset..offset + STRIDE], SIDE)
}

impl Frame {
    pub(super) fn draw_economy(&mut self, menu: &Menu, goods: &Goods, hp: u8) {
        let width = 280;
        let x = (self.width as i32 - width) / 2;
        self.labels.clear();
        self.panel(x, 17, width, 200);
        let title = match menu.page {
            Page::Bag => "持ち物",
            Page::Battle => "戦闘道具",
            Page::Shop => "よろず屋",
            Page::Buy => "品物を買う",
            Page::Sell => "素材を売る",
            Page::Brew => "薬の調合",
            Page::Forge => "鍛冶工房",
            Page::Rest(_) => "休息・手当て",
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
            };
            let selected = index == menu.selected;
            let y = 52 + 14 * (index - first) as i32;
            if selected {
                self.label(x + 12, y, 10, MINT, "▶");
            }
            let text_x = if let Some(item) = row.item() {
                self.art(item_icon(item), x + 27, y - 1, 13);
                x + 45
            } else {
                x + 27
            };
            self.label(text_x, y, 10, if selected { MINT } else { IVORY }, label);
        }
        self.rect(x + 12, 139, width - 24, 1, LINE);
        if let Some(item) = rows.get(menu.selected).and_then(|row| row.item()) {
            self.rect(x + 214, 145, 51, 49, INK);
            self.art(item_icon(item), x + 218, 148, 43);
        }
        let mut y = 146;
        match rows.get(menu.selected) {
            Some(OptionRow::Craft(recipe)) => {
                for &(item, required) in recipe.needs {
                    let held = goods.count(item);
                    self.art(item_icon(item), x + 12, y - 1, 10);
                    self.label(
                        x + 26,
                        y,
                        9,
                        if held >= required { MINT } else { MIST },
                        format!("{}　所持 {held} / 必要 {required}", item.name()),
                    );
                    y += 11;
                }
                self.label(x + 12, y, 9, IVORY, recipe.output.description());
            }
            Some(OptionRow::Item(item) | OptionRow::Buy(item) | OptionRow::Sell(item)) => {
                self.label(x + 12, y, 9, IVORY, item.description());
            }
            None => self.label(x + 12, 54, 10, MIST, "ここで使える品がない。"),
            _ => {}
        }
        self.label(x + 12, 183, 9, GOLD, &menu.note);
        self.label(x + 12, 201, 9, MIST, "上下 選ぶ　決定 実行　← 戻る");
        if !matches!(menu.page, Page::Bag | Page::Battle) {
            self.label(x + 204, 201, 9, MINT, "→ 持ち物");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_item_has_its_own_visible_icon() {
        let mut art = std::collections::HashSet::new();
        for item in Item::ALL {
            let sprite = item_icon(item);
            assert!(
                sprite.pixels.chunks_exact(4).any(|pixel| pixel[3] > 220),
                "{item:?}"
            );
            assert!(
                sprite.pixels.chunks_exact(4).any(|pixel| pixel[3] == 0),
                "{item:?}"
            );
            assert!(art.insert(sprite.pixels), "{item:?}");
        }
    }
}

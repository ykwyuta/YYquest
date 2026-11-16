use super::{Frame, GOLD, IVORY, MINT, MIST};
use crate::debug::Menu;

impl Frame {
    pub(super) fn draw_debug_menu(&mut self, menu: &Menu) {
        const ROWS: usize = 9;
        self.landscape(0, 0, 320, 150);
        self.panel(13, 15, 294, 211);
        self.label(25, 24, 15, GOLD, "デバッグモード");
        self.label(25, 46, 11, MIST, menu.heading());
        let items = menu.entries();
        let first = menu
            .selected
            .saturating_sub(ROWS / 2)
            .min(items.len().saturating_sub(ROWS));
        for (i, item) in items.iter().enumerate().skip(first).take(ROWS) {
            let y = 66 + (i - first) as i32 * 14;
            if i == menu.selected {
                self.rect(22, y - 1, 276, 14, 0x383746);
                self.label(27, y, 10, MINT, "▶");
            }
            self.label(
                44,
                y,
                10,
                if i == menu.selected { MINT } else { IVORY },
                item.as_str(),
            );
        }
        self.label(25, 207, 9, MIST, "十字キー  選択　　決定　　Esc / B  戻る");
        self.label(
            252,
            207,
            9,
            MIST,
            format!("{}/{}", menu.selected + 1, items.len()),
        );
    }
}

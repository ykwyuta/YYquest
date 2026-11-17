use super::{Frame, GOLD, IVORY, MINT, MIST};
use crate::debug::{Menu, Reader};

impl Frame {
    pub(super) fn draw_debug_menu(&mut self, menu: &Menu) {
        if let Some(reader) = &menu.reader {
            self.draw_dialogue_reader(reader);
            return;
        }
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

    fn draw_dialogue_reader(&mut self, reader: &Reader) {
        let page = reader.current();
        let (number, count) = reader.position();
        self.landscape(0, 0, 320, 240);
        self.panel(13, 12, 294, 218);
        self.label(26, 22, 14, GOLD, "会話を通読");
        self.label(255, 25, 9, MINT, format!("{number}/{count}"));
        self.label(26, 44, 10, MIST, page.section);
        self.label(26, 61, 12, GOLD, page.title);
        let top = if let Some(choice) = page.choice {
            self.label(26, 81, 10, MINT, format!("分岐　{choice}"));
            100
        } else {
            84
        };
        self.dialogue_content((27, top), 266, page.message.text(), page.message.delivery);
        let next = if number == count {
            "決定　一覧へ"
        } else {
            "決定　次へ"
        };
        self.label(
            26,
            211,
            9,
            MIST,
            format!("← / ↑　前へ　　{next}　　Esc / B　戻る"),
        );
    }
}

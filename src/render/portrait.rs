use super::{Frame, Sprite, GOLD, INK, IVORY, LINE, MINT, MIST};
use crate::{
    art_sizes,
    dialogue::{Delivery, Portrait, Speaker},
};

fn face_art(portrait: Portrait) -> Sprite {
    const SIDE: usize = art_sizes::FACE;
    const STRIDE: usize = SIDE * SIDE * 4;
    const FACES: &[u8; 100 * STRIDE] =
        include_bytes!(concat!(env!("OUT_DIR"), "/dialogue-faces.rgba"));
    let frame = portrait.speaker as usize * 10 + portrait.expression as usize;
    Sprite::new(&FACES[frame * STRIDE..(frame + 1) * STRIDE], SIDE)
}

impl Frame {
    pub(super) fn dialogue_content(
        &mut self,
        (x, y): (i32, i32),
        width: i32,
        lines: &[&str],
        delivery: Delivery,
    ) {
        const FACE_SIZE: i32 = 64;
        let (text_x, text_width) = if let Some(face) = delivery.portrait() {
            self.rect(x - 1, y - 1, FACE_SIZE + 2, FACE_SIZE + 2, INK);
            self.rect(x, y, FACE_SIZE, FACE_SIZE, LINE);
            self.rect(x + 1, y + 1, FACE_SIZE - 2, FACE_SIZE - 2, 0x191d2a);
            self.art_oriented(
                face_art(face),
                x + 1,
                y + 1,
                (FACE_SIZE - 2) as usize,
                [0, 0, self.width as i32, super::HEIGHT as i32],
                !matches!(face.speaker, Speaker::Lightkeeper | Speaker::Villager),
            );
            (x + FACE_SIZE + 12, width - FACE_SIZE - 12)
        } else {
            (x, width)
        };
        let (header, accent) = match delivery {
            Delivery::Speech { name, .. } => (name, GOLD),
            Delivery::Scene => ("状況", MIST),
            Delivery::Record(source) => (source, MINT),
            Delivery::Guide => ("案内", MINT),
        };
        self.label(text_x, y, 11, accent, header);
        let columns = (text_width / 10).max(1) as usize;
        let mut row = 0;
        for (index, line) in lines.iter().enumerate() {
            let prefix = if delivery.is_speech() && index == 0 {
                "「"
            } else {
                ""
            };
            let suffix = if delivery.is_speech() && index + 1 == lines.len() {
                "」"
            } else {
                ""
            };
            let chars: Vec<_> = prefix
                .chars()
                .chain(line.chars())
                .chain(suffix.chars())
                .collect();
            let mut start = 0;
            while start < chars.len() {
                let end = line_end(&chars, start, columns);
                self.label(
                    text_x,
                    y + 15 + 12 * row,
                    10,
                    IVORY,
                    chars[start..end].iter().collect::<String>(),
                );
                start = end;
                row += 1;
            }
        }
    }
}

fn line_end(chars: &[char], start: usize, columns: usize) -> usize {
    const NO_START: &str = "、。，．！？）」』】〉》］｝・ー…ぁぃぅぇぉっゃゅょァィゥェォッャュョ";
    const NO_END: &str = "（「『【〈《［｛";
    let mut end = (start + columns).min(chars.len());
    while end > start + 1
        && end < chars.len()
        && (NO_START.contains(chars[end]) || NO_END.contains(chars[end - 1]))
    {
        end -= 1;
    }
    end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogue::Expression;

    #[test]
    fn presentation_marks_only_direct_speech_as_quoted() {
        let mut frame = Frame::new();
        for (delivery, heading, quoted) in [
            (
                Delivery::spoken("夜番", Speaker::Worker, Expression::Speaking),
                "夜番",
                true,
            ),
            (Delivery::Scene, "状況", false),
            (Delivery::Record("積み荷控え"), "積み荷控え", false),
            (Delivery::Guide, "案内", false),
        ] {
            frame.labels.clear();
            frame.dialogue_content(
                (12, 12),
                280,
                &["薪を運んだ。", "雪が降っていた。"],
                delivery,
            );
            let lines: Vec<_> = frame
                .labels
                .iter()
                .map(|label| label.text.as_str())
                .collect();
            assert_eq!(lines[0], heading);
            assert_eq!(lines[1].starts_with('「'), quoted);
            assert_eq!(lines[2].ends_with('」'), quoted);
        }
    }

    #[test]
    fn japanese_wrap_keeps_punctuation_and_brackets_with_text() {
        let text: Vec<_> = "薪を運、ぶ「炉」".chars().collect();
        assert_eq!(line_end(&text, 0, 3), 2);
        assert_eq!(line_end(&text, 2, 4), 5);
        assert_eq!(line_end(&text, 5, 3), text.len());
        let quoted: Vec<_> = "薪を「炉」へ".chars().collect();
        assert_eq!(line_end(&quoted, 0, 4), 2);
    }
}

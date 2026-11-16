use super::{Frame, Sprite, GOLD, INK, IVORY, LINE};
use crate::{
    art_sizes,
    dialogue::{Portrait, Speaker},
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
        face: Option<Portrait>,
    ) {
        const FACE_SIZE: i32 = 64;
        let (text_x, text_y, text_width) = if let Some(face) = face {
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
            self.label(x + FACE_SIZE + 12, y, 11, GOLD, face.speaker.name());
            (x + FACE_SIZE + 12, y + 15, width - FACE_SIZE - 12)
        } else {
            (x, y, width)
        };
        let columns = (text_width / 10).max(1) as usize;
        let mut row = 0;
        for line in lines {
            let chars: Vec<_> = line.chars().collect();
            for chunk in chars.chunks(columns) {
                self.label(
                    text_x,
                    text_y + 12 * row,
                    10,
                    IVORY,
                    chunk.iter().collect::<String>(),
                );
                row += 1;
            }
        }
    }
}

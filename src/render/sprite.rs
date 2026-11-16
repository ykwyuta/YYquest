#[derive(Clone, Copy)]
pub(super) struct Sprite {
    pub(super) pixels: &'static [u8],
    side: usize,
}

impl Sprite {
    pub(super) const fn new(pixels: &'static [u8], side: usize) -> Self {
        assert!(pixels.len() == side * side * 4);
        Self { pixels, side }
    }

    fn pixel(self, x: usize, y: usize) -> [u8; 4] {
        let offset = (y * self.side + x) * 4;
        self.pixels[offset..offset + 4].try_into().unwrap()
    }

    // Average the covered source texels using premultiplied RGB so transparent
    // edges retain the colors of the artwork when reduced and composited.
    pub(super) fn sample(self, x: usize, y: usize, target: usize) -> [u8; 4] {
        if target == self.side {
            return self.pixel(x, y);
        }

        let (left, right) = (x * self.side, (x + 1) * self.side);
        let (top, bottom) = (y * self.side, (y + 1) * self.side);
        let mut channels = [0; 4];
        for row in top / target..bottom.div_ceil(target) {
            let height = bottom.min((row + 1) * target) - top.max(row * target);
            for col in left / target..right.div_ceil(target) {
                let width = right.min((col + 1) * target) - left.max(col * target);
                let weight = (width * height) as u32;
                for (sum, value) in channels.iter_mut().zip(self.pixel(col, row)) {
                    *sum += weight * u32::from(value);
                }
            }
        }
        let coverage = (self.side * self.side) as u32;
        channels.map(|sum| ((sum + coverage / 2) / coverage) as u8)
    }
}

pub(super) fn over_rgb([r, g, b, alpha]: [u8; 4], background: u32) -> u32 {
    let remaining = 255 - u32::from(alpha);
    let channel = |source: u8, shift: u32| {
        let under = (background >> shift) & 255u32;
        (u32::from(source) + (under * remaining + 127u32) / 255) << shift
    };
    channel(r, 16) | channel(g, 8) | channel(b, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shrinking_carries_partial_alpha_and_original_color() {
        const SPRITE: Sprite = Sprite::new(
            &[200, 100, 40, 255, 200, 100, 40, 255, 0, 0, 0, 0, 0, 0, 0, 0],
            2,
        );
        let pixel = SPRITE.sample(0, 0, 1);
        assert_eq!(pixel, [100, 50, 20, 128]);
        assert_eq!(over_rgb(pixel, 0x646464), 0x966446);
    }
}

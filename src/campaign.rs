use crate::dialogue::{Expression::*, Portrait, Speaker::*};

#[derive(Clone, Copy)]
pub(crate) struct ChapterBeat {
    pub site: usize,
    pub place: Option<usize>,
    pub title: &'static str,
    pub text: &'static str,
    pub face: Portrait,
}

const CHAPTER: [ChapterBeat; 6] = [
    ChapterBeat {
        site: 0, place: Some(1), title: "途切れた帰還簿",
        text: "これがガウェインの帰還簿だ。同じ旗が二度帰ってるのに、片方は人数が削られてる。欄外に三つ灯の印があるだろう。霧灯の港の信号台で聞いてみな。",
        face: Portrait::new(Hearth, Thoughtful),
    },
    ChapterBeat {
        site: 1, place: Some(3), title: "霧の向こうの三灯",
        text: "三灯は『救助船を求む』だ。昨夜も天墜ち砦から同じ合図が来た。この控えはにじんでるが、石継ぎの拓本師なら、煤けた旧村の石から印を読める。",
        face: Portrait::new(Lightkeeper, Worried),
    },
    ChapterBeat {
        site: 5, place: Some(1), title: "石に残った船名",
        text: "拓本を重ねてごらん。削られた欄に船の名が戻る。兵を運ぶ船が、名簿の外にいた人々を救った。出港印の主は、ノクス北門街の閲覧所で調べられるよ。",
        face: Portrait::new(Mason, Speaking),
    },
    ChapterBeat {
        site: 7, place: Some(3), title: "二つの旗、ひとつの門",
        text: "この印は私のものだ。船に乗った人々の名が思い出せない。これで砦を開けてくれ。名は本人に返し、救助したという事実は、門前に掲げてほしい。",
        face: Portrait::new(Baldo, Sad),
    },
    ChapterBeat {
        site: 12, place: None, title: "天墜ち砦の残響",
        text: "あなたのお父さんが見える。船を通すために、最後まで砦に残ったんだ。声も、まだ聴こえる。封印は閉じたまま記録を写して、港へ届けよう。",
        face: Portrait::new(Lilith, Surprised),
    },
    ChapterBeat {
        site: 1, place: Some(1), title: "次の港への約束",
        text: "写しは港で預かろう。原本の一部はセレスティアの聖堂へ運ばれた。確かめに行くなら、次の船に乗れる。ほら、乗船札を二枚用意してある。",
        face: Portrait::new(Lightkeeper, Smile),
    },
];

#[derive(Default)]
pub(crate) struct Campaign {
    next: Option<usize>,
    pub reading: bool,
}

impl Campaign {
    pub(crate) fn start(&mut self) {
        self.next = Some(0);
        self.reading = false;
    }

    pub(crate) fn beat(&self) -> Option<&'static ChapterBeat> {
        CHAPTER.get(self.next?)
    }

    pub(crate) fn advance(&mut self) {
        if self.beat().is_some() {
            self.next = self.next.map(|index| index + 1);
        }
        self.reading = false;
    }

    pub(crate) fn complete(&self) -> bool {
        self.next == Some(CHAPTER.len())
    }
}

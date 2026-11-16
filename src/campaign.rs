use crate::dialogue::{Delivery, Expression::*, Speaker::*};

#[derive(Clone, Copy)]
pub(crate) struct ChapterBeat {
    pub site: usize,
    pub place: Option<usize>,
    pub title: &'static str,
    pub text: &'static str,
    pub delivery: Delivery,
}

const CHAPTER: [ChapterBeat; 6] = [
    ChapterBeat {
        site: 0, place: Some(1), title: "途切れた帰還簿",
        text: "これが父さんの帰還簿だよ。ほら、同じ旗が二度戻ったと書いてある。二度目は人数だけ削ってあるね。欄外の三つの灯……霧灯の港の信号台で、灯台守に聞いてみな。",
        delivery: Delivery::spoken("炉番", Hearth, Thoughtful),
    },
    ChapterBeat {
        site: 1, place: Some(3), title: "霧の向こうの三灯",
        text: "灯が三つなら、救助船を出せって合図だ。昨夜も天墜ち砦で光った。こっちの控えはもう読めんが、石継ぎの拓本師に聞いてみな。旧村から運んだ石に、あの晩の印が残ってる。",
        delivery: Delivery::spoken("灯台守", Lightkeeper, Worried),
    },
    ChapterBeat {
        site: 5, place: Some(1), title: "石に残った船名",
        text: "拓本をここに重ねてみて。削られた欄に船の名が出たでしょう？　兵を運ぶ船が、名簿にない人たちも助けたのね。誰が船を出したかは、ノクス北門街の閲覧所で聞いてみて。",
        delivery: Delivery::spoken("拓本師", Mason, Neutral),
    },
    ChapterBeat {
        site: 7, place: Some(3), title: "二つの旗、ひとつの門",
        text: "ああ、船を出す許可を出したのは私だ。それなのに、乗せた人たちの名が思い出せん。これで砦を開けてくれ。名前は本人にだけ伝えて、助かった人がいたことは門前に掲げてほしい。",
        delivery: Delivery::spoken("バルド", Baldo, Sad),
    },
    ChapterBeat {
        site: 12, place: None, title: "天墜ち砦の残響",
        text: "アレン、あれ……あなたのお父さんだ。船を通すために、ここに残ったんだね。まだ声が聞こえる。封印には触らずに、記録だけ写して港へ持っていこう。",
        delivery: Delivery::spoken("リリス", Lilith, Surprised),
    },
    ChapterBeat {
        site: 1, place: Some(1), title: "次の港への約束",
        text: "写しはここで預かるよ。元の記録は、一部がセレスティアの聖堂に運ばれたそうだ。確かめに行くかい？　次の船なら間に合う。ほら、二人ぶんの乗船札だ。",
        delivery: Delivery::spoken("灯台守", Lightkeeper, Smile),
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

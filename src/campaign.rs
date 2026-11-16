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
        site: 0, place: Some(1), title: "途切れた帰還記録",
        text: "これがお父さんの残した帰還記録だよ。同じ船が二度戻ったとあるのに、二度目の人数だけが削り消されている。欄外に「三つの灯」という印があるね……霧灯の港町の信号台で、灯台守に意味を聞いてごらん。",
        delivery: Delivery::spoken("炉番", Hearth, Thoughtful),
    },
    ChapterBeat {
        site: 1, place: Some(3), title: "霧の向こうの三つの灯",
        text: "「三つの灯」は救助船を出せという緊急の合図だ。戦役の夜、天墜ち砦でもその灯がともった。港の控えは潮で滲んで読めないが、石継ぎの町の拓本師なら、旧村から運んだ石に残るあの晩の記録を写しているはずだ。",
        delivery: Delivery::spoken("灯台守", Lightkeeper, Worried),
    },
    ChapterBeat {
        site: 5, place: Some(1), title: "石に刻まれた船の名",
        text: "石から写した文字を、帰還記録の削られた欄に重ねてみて。ほら、救助船の名前が浮かび上がったでしょう？　軍の船が、名簿に載っていない避難民も密かに助けていたのね。誰がその船を出したのか、ノクス北門街の閲覧所で確かめてみて。",
        delivery: Delivery::spoken("拓本師", Mason, Neutral),
    },
    ChapterBeat {
        site: 7, place: Some(3), title: "二つの旗、ひとつの門",
        text: "そうだ、救助船の出航を許したのは私だ。だが禁呪のせいで、助けた人々の名をどうしても思い出せない。この鍵を持って天墜ち砦へ向かい、残る記録を見てきてくれ。助かった人の名は家族だけに、生存の知らせは門前に掲げてほしい。",
        delivery: Delivery::spoken("バルド", Baldo, Sad),
    },
    ChapterBeat {
        site: 12, place: None, title: "天墜ち砦の残響",
        text: "アレン、見て……お父さんの声だ。救助船を安全に通すため、砦の封印を守ってここに残ったんだね。封印には触れずに、残された記録だけを書き写して霧灯の港へ届けよう。",
        delivery: Delivery::spoken("リリス", Lilith, Surprised),
    },
    ChapterBeat {
        site: 1, place: Some(1), title: "次の島への船出",
        text: "写しは確かに預かったよ。元の記録は一部がセレスティアの聖堂へ運ばれたそうだ。真実を確かめに行くかい？　次の船ならまだ間に合う。ほら、二人ぶんの乗船札を持っていきな。",
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

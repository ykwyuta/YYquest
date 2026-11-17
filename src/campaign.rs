use crate::dialogue::{Delivery, Expression::*, Speaker::*};
use crate::world_map::Region;

mod voyage;
pub(crate) use voyage::{VoyageBeat, VoyagePanel, CONSENT_OPTIONS, CONSENT_RESPONSES, STEPS as VOYAGE};

#[derive(Clone, Copy)]
pub(crate) struct ChapterBeat {
    pub site: usize,
    pub place: Option<usize>,
    pub title: &'static str,
    pub text: &'static str,
    pub delivery: Delivery,
}

pub(crate) const CHAPTER: [ChapterBeat; 6] = [
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
        text: "救助船の出航を許したのは私だ。この鍵と拓本を持って、天墜ち砦の奥で記録を探してくれ。砦の霧は濃いから、傷薬が要るなら港のよろず屋か灰灯りの村の炊事場で備えるといい。",
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

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub(crate) struct Campaign {
    next: Option<usize>,
    pub fort_cleared: bool,
    voyage: usize,
    pub sealed_names: bool,
    #[serde(skip)]
    pub panel: Option<VoyagePanel>,
}

impl Campaign {
    pub(crate) fn valid(&self) -> bool {
        self.next.is_none_or(|index| index <= CHAPTER.len())
            && self.voyage <= voyage::STEPS.len()
            && (self.voyage == 0 || self.complete())
    }

    pub(crate) fn start(&mut self) {
        *self = Self {
            next: Some(0),
            ..Self::default()
        };
    }

    pub(crate) fn beat(&self) -> Option<&'static ChapterBeat> {
        CHAPTER.get(self.next?)
    }

    pub(crate) fn advance(&mut self) {
        if self.beat().is_some() {
            self.next = self.next.map(|index| index + 1);
        }
    }

    pub(crate) fn complete(&self) -> bool {
        self.next == Some(CHAPTER.len())
    }

    pub(crate) fn voyage(&self) -> Option<&'static VoyageBeat> {
        self.complete()
            .then(|| voyage::STEPS.get(self.voyage))
            .flatten()
    }

    pub(crate) fn advance_voyage(&mut self) {
        if self.voyage().is_some() {
            self.voyage += 1;
        }
        self.panel = None;
    }

    pub(crate) fn finished(&self) -> bool {
        self.voyage == voyage::STEPS.len()
    }
}

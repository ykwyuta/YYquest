use crate::dialogue::{Delivery, Expression, Message, Speaker};
use crate::world_map::Region;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Event {
    Homeward = 0,
    NoxVillage = 1,
    NoxRoad = 2,
    Celestia = 3,
    Liberta = 4,
    Sylva = 5,
    Vulcan = 6,
}

impl Event {
    pub(crate) const ALL: [Self; 7] = [
        Self::Homeward,
        Self::NoxVillage,
        Self::NoxRoad,
        Self::Celestia,
        Self::Liberta,
        Self::Sylva,
        Self::Vulcan,
    ];

    pub(crate) fn on_island(region: Region, in_town: bool) -> Self {
        match region {
            Region::Nox if in_town => Self::NoxVillage,
            Region::Nox => Self::NoxRoad,
            Region::Celestia => Self::Celestia,
            Region::Liberta => Self::Liberta,
            Region::Sylva => Self::Sylva,
            Region::Vulcan => Self::Vulcan,
        }
    }

    pub(crate) fn title(self) -> &'static str {
        match self {
            Self::Homeward => "炉へ帰る道",
            Self::NoxVillage => "二人で聞く",
            Self::NoxRoad => "街道の道しるべ",
            Self::Celestia => "鐘の音",
            Self::Liberta => "波止場で待つ人",
            Self::Sylva => "忘れたときのために",
            Self::Vulcan => "剣の握り",
        }
    }

    pub(crate) fn lines(self) -> &'static [Message] {
        use Expression::*;
        use Speaker::{Allen, Lilith};
        match self {
            Self::Homeward => {
                &const {
                    [
                say(Allen, Worried, "村まで、少しゆっくり歩こうか。石碑の霧を払ったあと、手が震えてた。"),
                say(Lilith, Surprised, "見えてたの？　じゃあ炉の前で少し休みたい。アレンもでしょう。ずっと剣を握ったままだよ。"),
                say(Allen, Relieved, "……気づかなかった。炉番に報告する前に、二人で温まっていこう。"),
            ]
                }
            }
            Self::NoxVillage => {
                &const {
                    [
                say(Lilith, Thoughtful, "聞き込み、私から話しかけてもいい？　アレン、お父さんのことになると急いで聞きすぎるから。"),
                say(Allen, Worried, "いきなり答えを聞こうとしてたな。じゃあ僕は、聞けたことを手記に書くよ。"),
                say(Lilith, Smile, "よろしく。私が何を聞くつもりだったか忘れたら、そっちも教えてね。"),
            ]
                }
            }
            Self::NoxRoad => {
                &const {
                    [
                say(Allen, Thoughtful, "帰り道の目印も覚えてる？　行く先ばかり見てた。"),
                say(Lilith, Smile, "分かれ道に来たら一度振り返ろう。帰るときに見える景色を、先に見ておくの。"),
                say(Allen, Speaking, "次は僕が振り返るよ。二人で同じ方ばかり見てても仕方ないものな。"),
            ]
                }
            }
            Self::Celestia => {
                &const {
                    [
                say(Allen, Thoughtful, "鐘が鳴るたびに、村の炉番が火の番を交代する時間を思い出す。ここの鐘は遠くまで聞こえるな。"),
                say(Lilith, Speaking, "私はまだ、鐘を聞いて帰る場所は思い浮かばない。でも、アレンが足を止める理由は一つ覚えた。"),
                say(Allen, Smile, "じゃあ次は君が足を止めたら、僕も待つよ。何の音か教えて。"),
            ]
                }
            }
            Self::Liberta => {
                &const {
                    [
                say(Lilith, Thoughtful, "船から降りる人を、一人ずつ数えていた人がいたよ。最後の一人が降りるまで、ずっと桟橋にいた。"),
                say(Allen, Sad, "うちでも父さんの船を待った。人違いだって分かっても、次の人が降りるまで帰れなかった。"),
                say(Lilith, Speaking, "話してくれてありがとう。もう少しそこで潮風に当たってから、行こうか。"),
            ]
                }
            }
            Self::Sylva => {
                &const {
                    [
                say(Lilith, Worried, "この森の匂い、どこかで嗅いだ気がする。そう書くと、思い出したことみたいに読めちゃうかな。"),
                say(Allen, Thoughtful, "『気がする』まで書いておこう。次に読んだとき、今日のリリスがどう思ったか分かるように。"),
                say(Lilith, Smile, "そうする。あと、アレンが私の返事を急かさず待ってくれたことも書いておく。"),
            ]
                }
            }
            Self::Vulcan => {
                &const {
                    [
                say(Lilith, Surprised, "その剣、握りの布がほつれてる。さっきから持ち直してたの、手が痛いから？"),
                say(Allen, Thoughtful, "父さんが巻いた布だから、替えるのをためらってた。……でも振るたびにずれる。"),
                say(Lilith, Speaking, "古い布は取っておこう。新しい布を巻くとき、端を押さえるよ。"),
                say(Allen, Smile, "頼む。次に手を離せる場所で止まろう。"),
            ]
                }
            }
        }
    }

    fn bit(self) -> u16 {
        1 << self as u8
    }
}

const fn say(speaker: Speaker, expression: Expression, text: &'static str) -> Message {
    Message::line(text, Delivery::spoken(speaker.name(), speaker, expression))
}

#[derive(Default)]
pub(crate) struct PartyTalk {
    pub(crate) seen: u16,
    area: Option<Event>,
    steps: u8,
    idle: u32,
    active: Option<(Event, usize)>,
}

impl PartyTalk {
    pub(crate) fn walked(&mut self, event: Event) {
        if self.seen & event.bit() != 0 {
            return;
        }
        if self.area != Some(event) {
            self.area = Some(event);
            self.steps = 0;
        }
        self.steps = self.steps.saturating_add(1);
        self.idle = 0;
    }

    pub(crate) fn pause(&mut self, event: Event, milliseconds: u32) -> bool {
        let steps = if matches!(event, Event::Homeward) {
            3
        } else {
            8
        };
        if self.area != Some(event) || self.steps < steps || self.seen & event.bit() != 0 {
            return false;
        }
        self.idle = self.idle.saturating_add(milliseconds);
        if self.idle < 700 {
            return false;
        }
        self.active = Some((event, 0));
        true
    }

    pub(crate) fn current(&self) -> Option<Message> {
        let (event, page) = self.active?;
        Some(event.lines()[page])
    }

    pub(crate) fn advance(&mut self, skip: bool) {
        let Some((event, page)) = self.active else {
            return;
        };
        if skip || page + 1 == event.lines().len() {
            self.seen |= event.bit();
            self.active = None;
        } else {
            self.active = Some((event, page + 1));
        }
    }
}

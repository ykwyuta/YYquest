use crate::dialogue::{
    Expression::{self, *},
    Message,
    Speaker::{self, *},
};

pub(crate) const OPENING: Message = Message::narration([
    "東の森から水が止まり、炉のそばを緑の精霊が舞い始めた。",
    "案内精霊トモルに導かれ、アレンは「炉番」のもとへ向かう。",
]);

pub(crate) const GUIDE: Message = Message::guide([
    "案内精霊トモルが、三つの光の雫で安全な道を照らしている。",
    "光のあとをたどって、目的地へ向かおう。",
]);

pub(crate) const REST: Message = Message::narration([
    "炉のそばで休むと、手足の冷えが引いた。",
    "アレンの体力が回復した。",
]);

pub(crate) const HEARTH_LORE: Message = Message::spoken(
    Hearth,
    Thoughtful,
    [
        "『トモル』が目覚めたのかい。父さんの剣のそばで眠っていた灯の精霊さ。",
        "助けを呼ぶ声や灯の記憶に共鳴して、安全な道を光で示してくれるんだよ。",
    ],
);

pub(crate) const MEETING: Message = Message::spoken(
    Lilith,
    Speaking,
    [
        "村の人？　さっき笛を吹いたの、私。リリスっていうの。",
        "水門が引っかかって開かないの。剣で持ち上げるのを手伝ってくれない？",
    ],
);

pub(crate) const SLUICE: Message = Message::narration([
    "アレンが父の剣を留め具に差し込むと、重い水門が持ち上がった。",
    "リリスが門を支えたその瞬間、水路の奥から魔物が飛び出した！",
]);

pub(crate) const MEMORIAL: Message = Message::spoken(
    Lilith,
    Resolute,
    [
        "石碑が濃い霧に覆われて、さっきの音をかき消しているみたい。",
        "私が精霊の力で霧を払うから、少しの間だけ魔物を引き受けて！",
    ],
);

pub(crate) const HEARTH_REVISIT: Message = Message::spoken(
    Hearth,
    Relieved,
    [
        "水はもう来てるよ。火のそばにおいで。",
        "顔色がよくなったね。出る前に、もう少し温まっていきな。",
    ],
);

pub(crate) const HEARTH_REPORT: Message = Message::spoken(
    Hearth,
    Smile,
    [
        "村に水が戻ったよ。二人とも、本当に助かった！",
        "そうだ、お父さんの帰還記録なら、村の旧宅にしまってあるよ。",
    ],
);

pub(crate) const HEARTH_TASK: Message = Message::spoken(
    Speaker::Hearth,
    Expression::Worried,
    [
        "森の水路が止まったうえ、東の森から笛の音が聞こえたよ。",
        "古い水門は父さんの剣で開けられる。トモルと見てきておくれ。",
    ],
);

pub(crate) const ASK: Message = Message::spoken(
    Speaker::Allen,
    Expression::Thoughtful,
    [
        "今の声……父さんは救助船を見送って、あの日、砦に残ったのか？",
        "父さんの家に記録があるはずだ。リリスも一緒に来てくれないか？",
    ],
);

pub(crate) const REPLY: Message = Message::spoken(
    Speaker::Lilith,
    Expression::Resolute,
    [
        "うん、私も行く。自分の名前以外の記憶がなくて……",
        "なぜ石碑から私を呼ぶ声が聞こえたのか、確かめたいの。",
    ],
);

pub(crate) const RECRUIT: Message = Message::narration([
    "二人はガウェインの記録を探すことにした。",
    "リリスが仲間になった。",
]);

pub(crate) const RESCUE: Message = Message::spoken(
    Speaker::Lilith,
    Expression::Worried,
    [
        "よかった、気がついたんだね。急いで霧の外まで運んだよ。",
        "無理しないで。もう一度挑むなら、今度は私もそばにいるから。",
    ],
);

pub(crate) const WATER_RESTORED: Message = Message::spoken(
    Speaker::Lilith,
    Expression::Relieved,
    [
        "水路が通ったね！　……あれ、今アレンの剣とトモルが共鳴しなかった？",
        "北の石碑からも同じ音が返ってくる。先に様子を見てくるね！",
    ],
);

pub(crate) const MEMORIAL_RECORD: Message = Message::record(
    "石碑から響く過去の声",
    [
        "『ガウェイン、砦の三つの灯が見えた。救助船をそっちへ向かわせる』",
        "『リリス、船が着くまで砦の灯を絶やすな』",
    ],
);

pub(crate) const REVIEW: &[(&str, Message)] = &[
    ("旅の始まり", OPENING),
    ("炉番とトモル", HEARTH_LORE),
    ("炉番の頼み", HEARTH_TASK),
    ("トモルの案内", GUIDE),
    ("リリスとの出会い", MEETING),
    ("旧水門", SLUICE),
    ("水が戻る", WATER_RESTORED),
    ("霧に包まれた石碑", MEMORIAL),
    ("石碑の残響", MEMORIAL_RECORD),
    ("同行の誘い", ASK),
    ("リリスの返事", REPLY),
    ("二人の旅", RECRUIT),
    ("炉番に報告", HEARTH_REPORT),
    ("炉でひと休み", REST),
    ("炉を再訪", HEARTH_REVISIT),
    ("戦闘で倒れたとき", RESCUE),
];

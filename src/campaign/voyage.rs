use super::Region::{self, Celestia, Liberta, Sylva, Vulcan};
use crate::dialogue::Delivery;

#[derive(Clone, Copy)]
pub(crate) enum VoyagePanel {
    Read,
    Consent(usize),
    Response(bool),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Event {
    Read,
    Consent,
    Battle,
    Epilogue,
}

pub(crate) struct VoyageBeat {
    pub region: Region,
    pub site: usize,
    pub title: &'static str,
    pub speaker: &'static str,
    pub text: &'static str,
    pub event: Event,
}

impl VoyageBeat {
    pub(crate) fn delivery(&self) -> Delivery {
        Delivery::Speech {
            name: self.speaker,
            face: None,
        }
    }

    pub(crate) fn consent(&self) -> bool {
        self.event == Event::Consent
    }
    pub(crate) fn battle(&self) -> bool {
        self.event == Event::Battle
    }
    pub(crate) fn epilogue(&self) -> bool {
        self.event == Event::Epilogue
    }
}

pub(super) const STEPS: [VoyageBeat; 12] = [
    VoyageBeat { region: Celestia, site: 0, title: "第二章　名前を預ける国", speaker: "カイン",
        text: "ノクスの三灯印だな。私はカイン、元審問官だ。聖堂はその写しを攻略記録として収蔵した。中身は避難民の証言だ。巡礼神官のミナが聖堂の前で待っている。", event: Event::Read },
    VoyageBeat { region: Celestia, site: 1, title: "聖堂の写本", speaker: "ミナ",
        text: "帳面に救助船の名があるわ。ヴァレリウスは七つの調律器を使い、私たちの記憶を命令一つで公の帳面に移すつもりよ。湖畔の旧記録庫に、まだ話をしていない証言者がいるの。", event: Event::Read },
    VoyageBeat { region: Celestia, site: 10, title: "誰の名前で残すか", speaker: "避難船の生き残り",
        text: "船を降りたことは話せる。でも名を出すなら、行方を捜している家族だけに教えてほしい。私の名を家族宛ての封書に書くか、今回は記さずに預かってくれるかい？", event: Event::Consent },
    VoyageBeat { region: Liberta, site: 0, title: "第三章　灯の届く潮", speaker: "フィーネ",
        text: "兄の航海日誌が帆隠しの旧港にあるの。自治会が貸し出しを認めてくれた。海賊船には私から話したわ。旧港の桟橋で、兄が虚無の海を測った頁を探そう。", event: Event::Read },
    VoyageBeat { region: Liberta, site: 10, title: "濡れた航海日誌", speaker: "フィーネ",
        text: "兄の字だ。三潮灯台の光が海面から消え、また灯るまでの間だけ船が通れる。灯台守にこの頁を見せて、出られる時刻を確かめよう。", event: Event::Read },
    VoyageBeat { region: Liberta, site: 3, title: "船出の時刻", speaker: "灯台守",
        text: "日誌どおりなら入れる。船は潮待ちの港に置こう。海の向こうへ声を届ける装置が要るなら、ヴァルカンの元技師ガルダに相談するといい。", event: Event::Read },
    VoyageBeat { region: Vulcan, site: 0, title: "第四章　声を運ぶ炉", speaker: "ガルダ",
        text: "封印を継ぎ直す器械だと聞いて、人から記憶を抜く装置を作ったのは私だ。赤炉工区で職工が炉を止めて待っている。図面を持って行って、私たちの手で解体しよう。", event: Event::Read },
    VoyageBeat { region: Vulcan, site: 2, title: "共鳴機の試運転", speaker: "ガルダ",
        text: "記憶を吸い出す管は外した。ここへ声を吹き込めば、共鳴機から調律器へそのまま届く。調律の順を知るには、シルヴァの古根の封門を開いてくれ。", event: Event::Read },
    VoyageBeat { region: Sylva, site: 10, title: "第五章　リリスの記憶", speaker: "リリス",
        text: "私は封印を支えた七人の一人だった。調律の順は思い出したけれど、全部を開けば今の旅を忘れる。手順はみんなで分けて覚えて。潮待ちの港で船に乗ろう。", event: Event::Read },
    VoyageBeat { region: Liberta, site: 0, title: "終章　虚無の海へ", speaker: "フィーネ",
        text: "灯台の光が消えた。乗って！　神殿で共鳴機をつなぐわ。アビス・ロードの霧が集まったら、リリスと声を合わせて。手当てを済ませて船を出そう。", event: Event::Read },
    VoyageBeat { region: Liberta, site: 0, title: "虚無の海の神殿", speaker: "ヴァレリウス",
        text: "また忘れれば同じ戦が起こる。すべてを私の帳面に残すしかないと思っていた。……その共鳴機で、術者の記憶を焼かずに封印を保てるというなら、私も調律器を支えよう。", event: Event::Battle },
    VoyageBeat { region: Liberta, site: 0, title: "列島の声", speaker: "リリス",
        text: "ノクスの三灯が見える。みんな、自分で残すと決めた記録を順に読んで。私も最後に、自分の声で名を言う。これで封印を、一人の記憶に預けずに済む。", event: Event::Epilogue },
];

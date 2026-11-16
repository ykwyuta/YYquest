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
        text: "ノクスの「三つの灯」の写しか。私は元審問官のカインだ。聖堂は昔の記録を『戦の記録』という名目で書庫に隠しているが、中身は避難民たちの証言だ。巡礼神官のミナが聖堂の前で待っている。", event: Event::Read },
    VoyageBeat { region: Celestia, site: 1, title: "聖堂の写本", speaker: "ミナ",
        text: "この帳面に救助船の名前が載っているわ。けれどヴァレリウス卿は、人々から記憶を抜き出して一冊の公的な記録に固定しようとしているの。湖畔の旧記録庫に、当時の避難船に乗っていた生存者がいるわ。", event: Event::Read },
    VoyageBeat { region: Celestia, site: 10, title: "誰の名前で残すか", speaker: "避難船の生き残り",
        text: "船で助けられたことは証言できる。でも、名前を出すなら私を捜している家族だけに知らせてほしいんだ。家族宛ての封書に名前を残すか、名前は伏せたまま証言だけを預かってくれるかい？", event: Event::Consent },
    VoyageBeat { region: Liberta, site: 0, title: "第三章　灯の届く潮", speaker: "フィーネ",
        text: "あなたがノクスから来たアレンね。私はフィーネ。封印のある虚無の海へ渡るには、兄が残した航海日誌が要るの。海賊船にも話を通してあるから、帆隠しの旧港の桟橋で日誌を探しましょう。", event: Event::Read },
    VoyageBeat { region: Liberta, site: 10, title: "濡れた航海日誌", speaker: "フィーネ",
        text: "あった、兄さんの字だわ！　『三潮灯台の光が霧に消えてから再び灯るまでのあいだだけ、虚無の海へ通じる潮が開く』……灯台守にこの頁を見せて、出航できる時刻を確かめましょう。", event: Event::Read },
    VoyageBeat { region: Liberta, site: 3, title: "船出の時刻", speaker: "灯台守",
        text: "日誌のとおりなら次の引き潮で虚無の海へ入れるな。船は潮待ちの港に用意しておこう。ただ、封印を支えるには各地の声を同時に届ける装置が要る。ヴァルカンの元技師ガルダに相談するといい。", event: Event::Read },
    VoyageBeat { region: Vulcan, site: 0, title: "第四章　声を運ぶ炉", speaker: "ガルダ",
        text: "待っていたぞ。封印を保つためだと騙されて、人から記憶を抜き取る装置を設計してしまったのは私だ。赤炉工区で職工たちが炉を止めて待っている。図面を持って、みんなの力で装置を解体しよう。", event: Event::Read },
    VoyageBeat { region: Vulcan, site: 2, title: "共鳴機の試運転", speaker: "ガルダ",
        text: "記憶を吸い出す管はすべて外し、代わりに人々の声を届ける『共鳴機』へ組み替えたぞ。あとは各島の封印をどの順番で鳴らすかが分かればいい。シルヴァの古根の封門で手順を確かめてきてくれ。", event: Event::Read },
    VoyageBeat { region: Sylva, site: 10, title: "第五章　リリスの記憶", speaker: "リリス",
        text: "思い出した……トモルは昔、私がガウェインに預けた森の灯精霊だったんだね。調律の順はみんなで分け合って覚えよう。全部を背負えば今の旅を忘れてしまうから。準備ができたら潮待ちの港へ！", event: Event::Read },
    VoyageBeat { region: Liberta, site: 0, title: "終章　虚無の海へ", speaker: "フィーネ",
        text: "灯台の光が消えたわ、潮が開く合図よ！　船に乗って、虚無の海の神殿へ向かいましょう。封印の奥からアビス・ロードの霧が迫ってきたら、リリスと力を合わせて戦って。準備ができたら出航よ！", event: Event::Read },
    VoyageBeat { region: Liberta, site: 0, title: "虚無の海の神殿", speaker: "ヴァレリウス",
        text: "人々が戦役を忘れれば、また同じ悲劇が起こる。だからすべてを私の帳面に残すしかないと思っていた……。だが、その共鳴機で誰の記憶も犠牲にせず封印を保てるというなら、まずは目の前の災厄を退けてみせろ！", event: Event::Battle },
    VoyageBeat { region: Liberta, site: 0, title: "列島の声", speaker: "リリス",
        text: "見て、ノクスの海岸に三つの灯がともった！　みんな、自分の意思で残すと決めた記録を順番に読んで。私も最後に、自分の声で名乗るよ。これで封印を、誰か一人の記憶に預けずに済むんだ。", event: Event::Epilogue },
];

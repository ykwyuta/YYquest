use super::{Art, Beat, Ending, Landscape, Place, Plan, Story};
use crate::dialogue::Speaker::*;

pub(super) static VULCAN: [Plan; 10] = [
    Plan {
        name: "黒鉄港町",
        profile: "溶岩の岸に蒸気クレーンを建てて鉄材と燃料を運ぶ港町。荷役・機関・船員の三組が港務評議会を開いている。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "蒸気クレーン埠頭", at: (156, 124), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "巨大な蒸気クレーンと黒鉄の倉庫が並ぶ中心の埠頭。" },
            Place { name: "西の港務評議所と待合", at: (84, 104), art: Some(Art::Inn), ground: Landscape::Square, detail: "評議会の会議室と、寄港した船員の待合所。" },
            Place { name: "東の計量所", at: (224, 108), art: Some(Art::Cottage), ground: Landscape::Square, detail: "積み込む鉄材や石炭の正確な重さを量る詰所。" },
            Place { name: "北の封印倉", at: (156, 68), art: Some(Art::StoneGate), ground: Landscape::Basalt, detail: "連邦の輸送官が管理する厳重な封印貨物の倉庫。" },
            Place { name: "徒歩の綱道", at: (192, 134), art: None, ground: Landscape::Dock, detail: "積み込みを待つ荷の刻印と待機位置を確かめられる道。" },
            Place { name: "東の給油詰所", at: (236, 76), art: Some(Art::Sluice), ground: Landscape::Basalt, detail: "寒い朝に小型クレーンの油を温める機関番の小屋。" },
            Place { name: "待合の護符屋", at: (96, 134), art: Some(Art::Stalls), ground: Landscape::Square, detail: "船出の無事を祈る黒い小石の護符を売る屋台。" },
            Place { name: "西の温かい岩棚", at: (76, 136), art: None, ground: Landscape::Shore, detail: "排水の熱が流れ込む波打ち際の岩棚。" },
        ],
        stories: [
            Story {
                title: "積荷表より重い箱",
                beats: &[
                    Beat::speech(0, "荷役長", Worker, "封印貨物が重すぎて蒸気クレーンが過熱してるんだ！　止めてくれって言ってるのに、輸送官が待たないんだよ。"),
                    Beat::scene(4, "計量所の控えと荷の刻印を比べると、公表されている積荷表よりもずっと重い鉄箱が紛れ込んでいた。"),
                    Beat::speech(1, "機関番", Worker, "危険なままだ。評議会へ持ち込んで正式な再計量を求めるか、通常便だけ別のクレーンへ移してこの積込を一時止めようか？"),
                ],
                choices: Some([
                    Ending { label: "評議会へ再計量を求める", text: "正確な重さが公表され、無理のない荷役計画へ改められた。" },
                    Ending { label: "通常便を移して積込を止める", text: "過熱したクレーンが冷めるまで作業を止め、事故を未然に防いだ。" },
                ]),
            },
            Story {
                title: "油が届かない歯",
                beats: &[
                    Beat::speech(5, "機関番", Worker, "小さいクレーンが寒い朝だけうまく動かないんだ。故障じゃないはずなんだけどなあ。"),
                    Beat::scene(5, "見回りに同行すると、冷えて固くなった油が管の途中で詰まっていた。夜番の油壺を温かい機関室に置くようにした。"),
                    Beat::scene(0, "次の冷え込んだ朝も、小型クレーンは最初から滑らかに動き出した。"),
                ],
                choices: None,
            },
            Story {
                title: "船員待合の壁",
                beats: &[
                    Beat::speech(1, "待合の世話役", Villager, "言葉の違う遠くの船員さんたちが、出港の時間も食事の場所も分からずに不安そうに座ってるんだ。"),
                    Beat::scene(1, "文字が読めなくても分かるよう出港時刻と料理を絵札にして壁へ掛け、手持ち無沙汰な船員が炊事を手伝えるようにした。"),
                    Beat::scene(1, "身振り手振りで笑い合う輪が広がり、船員待合が明るく居心地のよい場所になった。"),
                ],
                choices: None,
            },
            Story {
                title: "黒い小石の護符",
                beats: &[
                    Beat::speech(6, "護符売りの子", Villager, "売ってた黒い小石のお守りの産地を聞かれたんだけど、どこで拾ったのか思い出せないんだよ……。"),
                    Beat::scene(1, "昔の待合の記録から、漂着した船が積んでいた故郷の石だったと分かった。"),
                    Beat::speech(6, "待合の世話役", Villager, "由来が分かったね。探している遺族に返すか、由来を添えて船員の無事を祈る待合の壁に飾らせてもらおうか？"),
                ],
                choices: Some([
                    Ending { label: "遺族のもとへ返す", text: "遠い海から戻ってきた大切な形見に、遺族は涙を流して喜んだ。" },
                    Ending { label: "由来を添えて壁に飾る", text: "船の無事を願う象徴として壁に収められ、出航前の船員たちが手を合わせるようになった。" },
                ]),
            },
            Story {
                title: "海へ消える熱",
                beats: &[
                    Beat::speech(7, "荷役人", Worker, "温かい排水が出る岸壁に甲殻獣が集まって、荷揚げの邪魔になって困ってるんだ。"),
                    Beat::speech(5, "機関番", Worker, "排水の熱が落ちる夕方のうちに小魚のいる湾へ誘うか、配管の出口を離れた岩棚のほうへ逃がそうか？"),
                ],
                choices: Some([
                    Ending { label: "小魚のいる湾へ誘う", text: "甲殻獣は餌を求めて静かな湾へ移り、岸壁での作業が安全に戻った。" },
                    Ending { label: "離れた岩棚へ熱を回す", text: "岩棚に温かい潮だまりができ、生き物も人間も互いを邪魔せず過ごせるようになった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "歯車街",
        profile: "刻印魔法に頼らず動く機械と大きな歯車を作る工都。共同旋盤の使用順と安全審査は工房の投票で決める。",
        terrain: Landscape::Basalt,
        entry: (156, 174),
        places: [
            Place { name: "歯車組立工場", at: (156, 92), art: Some(Art::Landmark), ground: Landscape::Square, detail: "銅の歯車が回る丸窓を備えた巨大な組立工場。" },
            Place { name: "北の規格試験場", at: (156, 48), art: Some(Art::Palisade), ground: Landscape::Square, detail: "新しい歯車や車軸の安全性を試す試験場。" },
            Place { name: "西の旧型修理横丁", at: (84, 104), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "古い機械を大事に直し続ける小工房の横丁。" },
            Place { name: "東の街区会所と旋盤", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "みんなで使う共同旋盤と街区会の事務所。" },
            Place { name: "工場裏の試運転線", at: (192, 68), art: None, ground: Landscape::Basalt, detail: "古い部品と新規格を動かして比べられる線路。" },
            Place { name: "時計台下の歯車場", at: (116, 128), art: Some(Art::Clock), ground: Landscape::Square, detail: "遅い歯車の音に耳を澄ますと仕事歌が聞こえる場所。" },
            Place { name: "横丁の作業台", at: (96, 74), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "片手でも作業できる踏み板の仕掛けを試す台。" },
            Place { name: "老工房主の作業場", at: (196, 134), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "工程札が並んだ歯車仕掛けの作業台がある部屋。" },
        ],
        stories: [
            Story {
                title: "まだ回る旧型",
                beats: &[
                    Beat::speech(2, "横丁の職人", Worker, "新規格ばかり安くなって、旧型機を直そうにも部品が手に入らないんだよ。自分たちで継ぎ足して直すしかないかな。"),
                    Beat::scene(4, "試運転線で確かめると、無理な継ぎ手をした車軸は危ないが、安全基準を満たす限定部品なら共同旋盤で作れると分かった。"),
                    Beat::speech(3, "街区会の技師", Worker, "安全が条件だ。安全な限定部品だけ共同旋盤で作り続けるか、新型への改修費を街区会で積み立てて支援しようか？"),
                ],
                choices: Some([
                    Ending { label: "限定部品を共同旋盤で続ける", text: "愛着のある旧型機が安全な部品で蘇り、横丁に活気が戻った。" },
                    Ending { label: "改修費の積立制度を作る", text: "無理のない負担で新しい安全な機械へ更新でき、事故の心配がなくなった。" },
                ]),
            },
            Story {
                title: "歯車が隠す音",
                beats: &[
                    Beat::speech(0, "組立工", Worker, "工場の機械が回ってるとき、どこからか途切れ途切れの奇妙なメロディが聞こえてくるんだよ。"),
                    Beat::scene(5, "回転が遅れている歯車の噛み合わせを直して音を揃えると、昔の職人が遊び心で仕込んだ仕事歌のオルゴールが綺麗に鳴り響いた。"),
                    Beat::scene(0, "休憩時間を知らせる優しい合図として、街じゅうの職人たちに親しまれるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "盗まれた設計図の余白",
                beats: &[
                    Beat::speech(1, "試験場の見張り", Guard, "徒弟が試験場から設計図を勝手に持ち出したって騒ぎになってるんだ。確かめてきてくれないか。"),
                    Beat::scene(1, "借出札を調べると正規の許可を得て借りており、徒弟は事故を防ぐための安全案を余白に書き込んでいた。"),
                    Beat::scene(3, "街区会で徒弟の名前付きで正式に審査されることになり、優れた工夫として表彰された。"),
                ],
                choices: None,
            },
            Story {
                title: "片手の旋盤番",
                beats: &[
                    Beat::speech(6, "旋盤番", Worker, "腕を痛めて工具を両手で握れなくなっちまった。最後の注文を断って引退するしかないかな……。"),
                    Beat::speech(6, "見習い職人", Villager, "踏み板の支点を変えれば片手でも回せるよ！　僕と二人で組んで続けるか、大切な小部品の検査・受付担当として残ってください！"),
                ],
                choices: Some([
                    Ending { label: "見習いと二人で作業を続ける", text: "息の合った師弟コンビとして見事な歯車を削り出し、注文をやり遂げた。" },
                    Ending { label: "小部品の検査担当になる", text: "厳しい目で不良品を見抜く名物番人として、街の信頼を一身に集めた。" },
                ]),
            },
            Story {
                title: "刻を数える機械",
                beats: &[
                    Beat::speech(7, "工房主の家族", Villager, "記憶の一部を失ってから、何十年もやってきた作業の手順が思い出せなくて落ち込んでるんです……。"),
                    Beat::scene(7, "工房を回って工程札を集め、歯車仕掛けの台に順番に並べて一緒に手を動かすと、体に染みついた技がすっと蘇った。"),
                    Beat::scene(7, "工房主は自分の手だけで最初の歯車を仕上げ、満面の笑みを見せた。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "赤炉工区",
        profile: "火山の地熱で大炉を動かす巨大工区。工程は工区長が決めるが、安全に関しては炉ごとの職工会が異議を出せる。",
        terrain: Landscape::Basalt,
        entry: (156, 174),
        places: [
            Place { name: "双煙突の赤炉群", at: (156, 92), art: Some(Art::Landmark), ground: Landscape::Square, detail: "二本の太い煙突から蒸気を上げる巨大な溶鉱炉。" },
            Place { name: "南の職工会所と医務室", at: (156, 140), art: Some(Art::Inn), ground: Landscape::Square, detail: "職工たちの安全相談や体調不良を診る医務室。" },
            Place { name: "東の夜勤搬出口", at: (224, 104), art: Some(Art::StoneGate), ground: Landscape::Basalt, detail: "夜間に作られた部品が運び出される秘密の搬出口。" },
            Place { name: "北の排気見回り廊", at: (156, 48), art: Some(Art::Bridge), ground: Landscape::Basalt, detail: "交代笛の響きと換気の状態を確かめる上の通路。" },
            Place { name: "炉辺の受け渡し台", at: (116, 128), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "家族が夜勤の職工へ弁当を手渡せる安全な場所。" },
            Place { name: "試験炉と燃料場", at: (84, 96), art: Some(Art::Hearth), ground: Landscape::Basalt, detail: "火の色の変化で燃料の質を確かめる試験用の炉。" },
            Place { name: "保守棚の通路", at: (218, 132), art: None, ground: Landscape::Basalt, detail: "放置された小型清掃機を誘導する保守用の通路。" },
            Place { name: "保守門の受付", at: (192, 74), art: Some(Art::Stalls), ground: Landscape::Square, detail: "荷と人の出入りを記録する夜間の詰所。" },
        ],
        stories: [
            Story {
                title: "勤務票の空白",
                beats: &[
                    Beat::speech(1, "医務室の医師", Villager, "最近、夜勤の病欠が急に増えたんだ。本人に聞いても『自分が最後に何を作ったか覚えていない』って言うんだよ……。"),
                    Beat::scene(2, "勤務票の欠けた日時と夜勤搬出口の部品を調べると、ガルダがかつて作った試作装置の印と同じマークが見つかった。"),
                    Beat::speech(1, "職工会の代表", Worker, "記憶を抜く装置がまだ動いているのかもしれない！　名前を伏せて安全検査を求めるか、職工会で夜の搬出自体を今すぐ止めようか？"),
                ],
                choices: Some([
                    Ending { label: "名前を伏せて安全検査を求める", text: "危険な試作部品の使用がただちに禁止され、夜勤者の安全が守られた。" },
                    Ending { label: "職工会で夜の搬出を止める", text: "全炉の点検が行われるまで搬出が止まり、怪しい装置の部品は解体された。" },
                ]),
            },
            Story {
                title: "交代笛を取り戻す",
                beats: &[
                    Beat::speech(3, "見回り廊の係", Worker, "奥の作業区画まで交代の笛が聞こえなくて、みんな時間を過ぎて働き続けちまうんだよ。"),
                    Beat::scene(3, "配管の共鳴を抑えて音を通りやすくし、通路の先にも交代を知らせる合図灯を追加した。"),
                    Beat::scene(1, "定刻どおりに誰もが仕事を終えて休めるようになり、工区の疲れがぐっと減った。"),
                ],
                choices: None,
            },
            Story {
                title: "炉辺の弁当",
                beats: &[
                    Beat::speech(4, "職工の子ども", Villager, "お父さんに温かいお弁当を届けたいのに、危ないからって保守門で止められちゃったんだ……。"),
                    Beat::scene(4, "医務室と職工会所に相談し、門の外の安全な場所に交代時間まで待てる屋根付きの受け渡し台を作った。"),
                    Beat::scene(4, "お父さんは休憩時間に温かいお弁当を受け取れるようになり、親子の笑顔が見られた。"),
                ],
                choices: None,
            },
            Story {
                title: "赤い炉の青い炎",
                beats: &[
                    Beat::speech(5, "燃料担当", Worker, "試験炉の火が急に真っ青に燃え出して、俺が変な油を混ぜたって疑われてるんだ！"),
                    Beat::scene(5, "保管場の鉱屑をひとつずつ試すと、特定の鉱石に混じっていた銅の粉が青い炎の原因だと分かった。"),
                    Beat::scene(5, "疑いはすっきり晴れ、不良の鉱屑だけを別に取り分けて安全に処分した。"),
                ],
                choices: None,
            },
            Story {
                title: "火花を食べる野良機",
                beats: &[
                    Beat::speech(6, "保守員", Worker, "放置されてた小型の清掃機が、炉から飛ぶ火花を追いかけて危ない場所まで走り回ってるんだよ！"),
                    Beat::scene(6, "保守棚から古い制御札を見つけ出し、炉に近づかず安全な通路の屑だけを集める設定へ切り替えた。"),
                    Beat::scene(6, "小さな清掃機はおとなしく通路を掃除する働き者になり、みんなのアイドルになった。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "火口の見張り町",
        profile: "火山の噴気を見張り、採掘班へ避難の合図を送る見張り町。見張り頭が警報を鳴らし、避難組が備蓄を守る。",
        terrain: Landscape::Basalt,
        entry: (156, 174),
        places: [
            Place { name: "鉄鋲の火口監視塔", at: (156, 74), art: Some(Art::Landmark), ground: Landscape::Square, detail: "噴火ごとの灰の高さが刻まれた鋲留めの高い監視塔。" },
            Place { name: "東の噴気採取所", at: (224, 98), art: Some(Art::Sluice), ground: Landscape::Basalt, detail: "噴気の温度と成分を測るための採取所。" },
            Place { name: "西の避難路と待避庫", at: (84, 106), art: Some(Art::Inn), ground: Landscape::Square, detail: "万が一の噴火に備えた食糧と避難通路。" },
            Place { name: "南の鉱区通用門", at: (156, 142), art: Some(Art::Palisade), ground: Landscape::Basalt, detail: "採掘場へ向かう運鉱車が通る大きな門。" },
            Place { name: "塔の日当たり踊り場", at: (184, 86), art: None, ground: Landscape::Square, detail: "子どもたちがこっそり背比べの印を付けた柱。" },
            Place { name: "冷たい北の岩斜面", at: (116, 62), art: None, ground: Landscape::Heath, detail: "いつもは冷えているのに、角獣が集まっている斜面。" },
            Place { name: "谷沿いの二軒家", at: (218, 134), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "谷の死角で避難鐘の音が届きにくい家並み。" },
            Place { name: "新旧の避難道角", at: (106, 134), art: None, ground: Landscape::Basalt, detail: "古い裏道と待避庫へ続く新しい道が分かれる角。" },
        ],
        stories: [
            Story {
                title: "閉じた避難路",
                beats: &[
                    Beat::speech(2, "避難組の住民", Villager, "噴気の温度が上がってるのに、鉱区主は『計器の故障だ』って言って操業を続けるんだ。しかも避難用の裏道に運鉱車が停まったままだよ！"),
                    Beat::scene(1, "測候記録を確かめると、確かに地中の熱が上がっていた。避難訓練の日に合わせて車をどかすか、待避庫から別の避難道を先に開く必要がありそうだ。"),
                    Beat::speech(0, "見張り頭", Guard, "命には代えられない。訓練名目で運鉱車を動かさせるか、待避庫から裏の安全な別道を先に切り開こうか？"),
                ],
                choices: Some([
                    Ending { label: "運鉱車を移動させて道を開ける", text: "避難訓練で全員が通れる道が確認され、鉱区主も安全対策を受け入れた。" },
                    Ending { label: "待避庫から新しい別道を開く", text: "運鉱車と重ならない広い避難路が整い、いつでも安全に逃げられるようになった。" },
                ]),
            },
            Story {
                title: "灰の高さの背比べ",
                beats: &[
                    Beat::speech(0, "見張り頭", Guard, "監視塔の柱に、去年よりずっと高い灰の印が付いてるんだ！　こんな高さまで積もったはずがないんだが……。"),
                    Beat::scene(4, "よく見ると、街の子どもたちが背比べをして付けた落書きの印だった。子どもたちには別の計測柱を貸してやった。"),
                    Beat::scene(0, "誤報だと分かって見張り頭も苦笑いし、子どもたちと並んで背を測った。"),
                ],
                choices: None,
            },
            Story {
                title: "空の採取瓶",
                beats: &[
                    Beat::speech(1, "観測員", Worker, "せっかく採ってきた噴気の瓶が、試験場に持っていく頃にはなぜか空っぽになっちまうんだ！"),
                    Beat::scene(1, "採取所と塔の気温差で、古いゴムの蓋が熱で緩んでいた。耐熱の真鍮蓋に交換すると、しっかり密閉できるようになった。"),
                    Beat::scene(1, "正確な噴気の分析ができるようになり、火山の監視がいっそう確かになった。"),
                ],
                choices: None,
            },
            Story {
                title: "熱を追う角獣",
                beats: &[
                    Beat::speech(5, "牧童", Villager, "いつも冷たいはずの北の斜面に、なぜか寒がりの角獣たちがぎゅうぎゅう集まってるんだよ。"),
                    Beat::scene(5, "岩肌に触れると、地中からほのかな地熱が伝わっていた。見張り頭へ新しい監視地点として知らせ、早めに観測を始められるようにした。"),
                    Beat::scene(0, "動物たちの異変のおかげで、地下の変化をいち早く察知できた。"),
                ],
                choices: None,
            },
            Story {
                title: "子のための避難鐘",
                beats: &[
                    Beat::speech(6, "谷の母親", Villager, "子守りをしていると風向きで避難鐘が聞こえにくいときがあって、いつも不安なんです……。"),
                    Beat::speech(0, "見張り頭", Guard, "窓の向きに合わせて音の反射板を取り付けるか、隣近所の二軒同士で必ず戸を叩き合う当番を決めようか？"),
                ],
                choices: Some([
                    Ending { label: "音の反射板を取り付ける", text: "鐘の音がくっきり家の中まで届くようになり、安心して暮らせるようになった。" },
                    Ending { label: "隣同士で声を掛け合う当番を作る", text: "いざというとき支え合う約束ができ、子育ての心強さも増した。" },
                ]),
            },
        ],
    },
    Plan {
        name: "煤坂の村",
        profile: "廃村を離れた職工たちが風上の坂に煉瓦屋根の家を築いた村。共同洗い場を労賃の一部で守り続けている。",
        terrain: Landscape::Basalt,
        entry: (156, 174),
        places: [
            Place { name: "段状住宅と共同洗い場", at: (156, 98), art: Some(Art::Landmark), ground: Landscape::Square, detail: "石段に沿って煉瓦の家と共同洗い場が並ぶ村の中心。" },
            Place { name: "北丘の灰受け場", at: (156, 48), art: Some(Art::Well), ground: Landscape::Basalt, detail: "風上から飛んでくる煤と灰の量を測る受け皿。" },
            Place { name: "東の職工寮", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "工場で働く独身の職工たちが暮らす大きな寮。" },
            Place { name: "南の陳情小屋", at: (156, 142), art: Some(Art::Cottage), ground: Landscape::Square, detail: "居住組と代表が工場への要望をまとめる小屋。" },
            Place { name: "寮の夜勤口", at: (212, 74), art: None, ground: Landscape::Yard, detail: "夜勤帰りの職工が出入りする坂上の裏階段。" },
            Place { name: "共同の石鉢場", at: (92, 104), art: Some(Art::Hearth), ground: Landscape::Square, detail: "屋外でお湯を沸かせる静かな石鉢の広場。" },
            Place { name: "坂上の壁広場", at: (116, 68), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "朝の光がきれいに差し込む白い漆喰の壁。" },
            Place { name: "雨樋の溜め枡", at: (192, 132), art: None, ground: Landscape::Yard, detail: "灰と一緒に光る小さな石が溜まっている雨樋。" },
        ],
        stories: [
            Story {
                title: "洗い水に残る煤",
                beats: &[
                    Beat::speech(2, "寮の職工", Worker, "夜の煤が増えたのは工場の煙じゃなく寮の炊事煙だって言われてるんだ。でも反論したら寮を追い出されそうで名を出せないよ……。"),
                    Beat::scene(1, "北丘の灰受け場と洗い場の煤を比べると、明らかに風上の工場から飛んできた重い炉の煤だった。"),
                    Beat::speech(3, "居住組の代表", Villager, "証拠は揃った。無記名で試料を連邦へ提出するか、寮を追い出されないよう先に居住組の受け入れ態勢を整えてから訴えようか？"),
                ],
                choices: Some([
                    Ending { label: "無記名で試料を提出する", text: "工場の集塵機の故障が見つかり、職工を守ったまま空気がきれいになった。" },
                    Ending { label: "先に居住組の支えを整える", text: "村全体が味方についたことで寮の職工も勇気を出し、堂々と改善を勝ち取った。" },
                ]),
            },
            Story {
                title: "同じ煉瓦を運ぶ",
                beats: &[
                    Beat::speech(0, "道直しの職人", Worker, "道を直す石材として古い廃村の煉瓦が運ばれてきたんだけど、どれに持ち主の印があるか探してくれないか。"),
                    Beat::scene(0, "元住民と一緒に煉瓦を確かめ、家族の印が刻まれた数個だけを共同洗い場の壁に丁寧に組み込んで大切に残した。"),
                    Beat::scene(0, "元の家の面影が村の中心に残り、通るたびに懐かしい思い出話に花が咲いた。"),
                ],
                choices: None,
            },
            Story {
                title: "夜勤明けの絵描き",
                beats: &[
                    Beat::speech(6, "若い職工", Villager, "夜勤明けの朝の光がきれいで絵を描いてたら、寮の窓を汚したって叱られちゃってさ……。"),
                    Beat::speech(6, "寮の世話役", Villager, "絵を描くのはいいことだ。共同洗い場が見える室内用の作業台を作るか、坂上の白い壁を絵描き用に貸してあげようか？"),
                ],
                choices: Some([
                    Ending { label: "光が差す室内作業台を作る", text: "窓を汚さず思う存分絵を描けるようになり、素敵な風景画が寮に飾られた。" },
                    Ending { label: "坂上の壁を貸してもらう", text: "村の壁に美しい壁画が描かれ、通りすがりの誰もが笑顔で見上げる名所になった。" },
                ]),
            },
            Story {
                title: "雨樋で拾った星",
                beats: &[
                    Beat::speech(7, "村の子ども", Villager, "雨樋の灰の中から、夜になると星みたいにきらきら光る石を見つけたんだよ！"),
                    Beat::scene(1, "灰受け皿と見比べると、火山の噴煙と一緒に空から降ってきた珍しい鉱物の粒だった。"),
                    Beat::speech(7, "鉱物好きの職工", Worker, "きれいな記念品として村に飾っておくか、珍しい成分を調べるために工区の研究所へ分析に出そうか？"),
                ],
                choices: Some([
                    Ending { label: "記念品として村に飾る", text: "共同洗い場の棚に小さな星石が飾られ、子どもたちの宝物になった。" },
                    Ending { label: "工区へ分析に出す", text: "火山の地下を知る貴重な手がかりになり、お礼に研究員が学校を開いてくれた。" },
                ]),
            },
            Story {
                title: "弔いの湯気",
                beats: &[
                    Beat::speech(5, "旧村の家族", Villager, "亡くなった旧村の知人を弔うために、温かいお湯で供養の湯気を立てたいんだけど、洗い場は夜勤の人が使う時間だよね……。"),
                    Beat::scene(5, "夜勤帯の炊事番の分を確保したうえで、屋外の静かな石鉢で火を据えてお湯を沸かせるよう手配した。"),
                    Beat::scene(5, "夕暮れの空へ立ち上る穏やかな湯気を囲み、家族は静かに手を合わせた。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "蒸気弁の集落",
        profile: "幹線蒸気管の圧力を逃がす大弁を守りながら、余った熱で温室を営む集落。臨時遮断には弁番二人の立会いが要る。",
        terrain: Landscape::Basalt,
        entry: (156, 174),
        places: [
            Place { name: "大蒸気弁と温室堂", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "巨大な真鍮の蒸気弁と、余熱で花を育てる丸屋根温室。" },
            Place { name: "南の丸屋根温室", at: (156, 140), art: Some(Art::Inn), ground: Landscape::Farm, detail: "冬でも色とりどりの花や苗が育つ温かい温室。" },
            Place { name: "東の低圧槽", at: (224, 102), art: Some(Art::Sluice), ground: Landscape::Basalt, detail: "圧力の変動を和らげて温室へ安定した熱を送る槽。" },
            Place { name: "西の保守溝入口", at: (84, 104), art: Some(Art::Cottage), ground: Landscape::Basalt, detail: "地下に伸びる枝管の振動や漏れを点検する入口。" },
            Place { name: "温室の裏道", at: (192, 128), art: None, ground: Landscape::Yard, detail: "各所の圧力計を順番に見て回れる点検用の裏道。" },
            Place { name: "補助弁の広場", at: (116, 72), art: Some(Art::Hearth), ground: Landscape::Square, detail: "子どもたちが蒸気の音真似をして遊ぶ広場。" },
            Place { name: "冬の霧通り", at: (92, 134), art: None, ground: Landscape::Yard, detail: "放熱のせいで冬になると白い霧が立ち込める通り。" },
            Place { name: "低温観察の棚", at: (218, 68), art: Some(Art::Stalls), ground: Landscape::Farm, detail: "水路から迷い込んだ珍しい植物を育てる棚。" },
        ],
        stories: [
            Story {
                title: "二通りの枝管",
                beats: &[
                    Beat::speech(1, "温室の世話役", Villager, "蒸気弁がずっと鳴りっぱなしで圧力が安定せず、温室の大切な苗が枯れそうなんだ。古い図面には枝管が二通り描かれてて……。"),
                    Beat::scene(3, "保守溝の振動と低圧槽の目盛りを確かめ、廃工場へ漏れていた枝管を特定した。"),
                    Beat::scene(2, "温室を低圧槽へつなぎ替えたあと廃工場の枝管を止めたところ、圧力がぴたりと安定して苗も元気を取り戻した。"),
                ],
                choices: None,
            },
            Story {
                title: "朝だけ咲く温室",
                beats: &[
                    Beat::speech(1, "花好きの病人", Villager, "寝込んでしまって温室まで行けないんだ。でも今年の最初の開花はどうしても見たくてね……。"),
                    Beat::speech(0, "弁の当番", Worker, "少し蒸気を緩めて咲く日を数日遅らせるか、咲いた瞬間の鉢植えをそのまま病室の窓辺へ届けてあげようか？"),
                ],
                choices: Some([
                    Ending { label: "蒸気を緩めて開花を遅らせる", text: "元気になって外へ出られた朝に、満開の花々が揃って出迎えてくれた。" },
                    Ending { label: "花鉢を病室の窓へ届ける", text: "枕元で開いた鮮やかな一輪の花を見て、病人の顔色もぱっと明るくなった。" },
                ]),
            },
            Story {
                title: "一斉に鳴く弁",
                beats: &[
                    Beat::speech(5, "集落の子ども", Villager, "僕が笛で蒸気の音を真似すると、いつも古い補助弁が同じ高さの音で返事してくれるんだよ！"),
                    Beat::scene(5, "弁番の立会いで音の高さを聴き比べ、調子の悪くなりかけた弁を見分ける『音聴き表』をみんなで作った。"),
                    Beat::scene(0, "壊れる前に音の違いで不調に気づけるようになり、集落の点検作業がずっと安全になった。"),
                ],
                choices: None,
            },
            Story {
                title: "うちの苗ではない",
                beats: &[
                    Beat::speech(7, "温室の見習い", Villager, "誰も植えていないはずの鉢から、見たこともない珍しい芽が出てるんだ！"),
                    Beat::scene(4, "散水の水路をたどると、上の池から水に流れて入った種だった。元の池へ戻すか、低温の棚で育てて観察するか見守ることにした。"),
                    Beat::scene(7, "珍しい水草として大切に育てられ、図鑑に新しいページが加わった。"),
                ],
                choices: None,
            },
            Story {
                title: "白い通り道",
                beats: &[
                    Beat::speech(6, "住民", Villager, "冬になると放熱の霧が濃くなって、この通りが真っ白な壁みたいに前が見えなくなっちまうんだ。"),
                    Beat::speech(0, "弁番", Worker, "地面の熱を畑側へ逃がして通りの霧を晴らすか、安全な見回り道に案内綱を張って迷わないようにしようか？"),
                ],
                choices: Some([
                    Ending { label: "熱を畑側へ逃がす", text: "通りの視界がすっきり晴れ、余った温もりで畑の冬野菜もよく育つようになった。" },
                    Ending { label: "見回り道と案内綱を作る", text: "白い霧の中でも安全に歩けるようになり、冬の幻想的な名所になった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "鎚音の町",
        profile: "巨大な落とし鎚を共同で使って車輪や農具を鍛える鍛冶の町。稼働時間と防音は住戸会と協議して決める。",
        terrain: Landscape::Valley,
        entry: (156, 174),
        places: [
            Place { name: "大鎚の鍛冶工房", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "水と蒸気の力で動く巨大な落とし鎚の共同工房。" },
            Place { name: "東の治具試作場", at: (224, 102), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "少ない打撃で効率よく鍛えられる治具を試す作業場。" },
            Place { name: "南の小児療養所", at: (156, 142), art: Some(Art::Inn), ground: Landscape::Square, detail: "静かな環境で子どもたちが体を休める療養所。" },
            Place { name: "西の岩斜面と音見台", at: (84, 104), art: Some(Art::Palisade), ground: Landscape::Heath, detail: "谷のどちら側へ鎚音が響くか確かめられる高台。" },
            Place { name: "大鎚外の練習台", at: (126, 76), art: Some(Art::Hearth), ground: Landscape::Yard, detail: "耳の遠い師匠と徒弟が振動で息を合わせる練習台。" },
            Place { name: "荷役路の曲がり角", at: (218, 134), art: None, ground: Landscape::Yard, detail: "急な切り返しで車輪が痛んでいないか調べられる角。" },
            Place { name: "西谷の岩陰", at: (92, 136), art: None, ground: Landscape::Heath, detail: "休みの日に不思議な太鼓の音が響き返る岩陰。" },
            Place { name: "退所祝いの細工台", at: (186, 122), art: Some(Art::Stalls), ground: Landscape::Square, detail: "子どもが自分で鳴らせる小鈴を調整している作業台。" },
        ],
        stories: [
            Story {
                title: "夜の鎚を振る前に",
                beats: &[
                    Beat::speech(0, "鍛冶組の親方", Worker, "車輪の大量注文が入って夜間も大鎚を動かしたいんだが、それだと南の療養所の子どもたちが眠れなくなっちまう……。"),
                    Beat::scene(3, "音見台から谷の響きを確かめると、治具で昼の作業効率を上げるか、岩斜面側に遮音壁を立てれば夜の影響を抑えられると分かった。"),
                    Beat::speech(1, "若手職人", Worker, "新しい治具で昼間の空き枠を増やすか、谷側に遮音の壁を立てて防音しようか？"),
                ],
                choices: Some([
                    Ending { label: "治具で昼の作業効率を上げる", text: "夜に動かさなくても注文に間に合うようになり、療養所の夜も静かなまま保たれた。" },
                    Ending { label: "谷側に遮音壁を立てる", text: "音が見事に遮られて療養所へ届かなくなり、街の暮らしと仕事が両立した。" },
                ]),
            },
            Story {
                title: "一打ちずつの共作",
                beats: &[
                    Beat::speech(4, "若い徒弟", Villager, "耳が遠くなった師匠と息が合わなくて、大鎚を打つ合図を何度も間違えちゃうんです……。"),
                    Beat::scene(4, "練習台で試しながら、声の代わりに手振りと足元の板の振動で合図を送る方法を二人で決めた。"),
                    Beat::scene(0, "見事な息の合い方で立派な農具が次々と鍛え上がり、師匠も目を細めて頷いた。"),
                ],
                choices: None,
            },
            Story {
                title: "曲がった車輪の里帰り",
                beats: &[
                    Beat::speech(0, "運送組の男", Traveler, "納品された車輪がもう歪んでるぞ！　焼き入れの手抜きをしたんじゃないのか？"),
                    Beat::scene(5, "荷役路の曲がり角と摩耗の向きを調べると、狭い角を無理に切り返して車輪を傷めていたことが分かった。"),
                    Beat::scene(5, "誤解が解けたあと、鍛冶屋が無料で直し、馬車道の角も少し広げて通りやすくした。"),
                ],
                choices: None,
            },
            Story {
                title: "岩の向こうの太鼓",
                beats: &[
                    Beat::speech(3, "見張り番", Villager, "休炉日で誰も工房にいないはずなのに、谷の向こうからコンコンと鎚音が聞こえてくるんだ。"),
                    Beat::scene(6, "岩陰へ回ると、音の真似をする小さな岩の精霊が石を叩いて遊んでいた。"),
                    Beat::speech(6, "鍛冶組の親方", Worker, "休日の可愛い太鼓役として町のみんなに紹介するか、静かな元の谷へそっと案内して帰してあげようか？"),
                ],
                choices: Some([
                    Ending { label: "休日の太鼓役に迎える", text: "休みの昼下がりに心地よいリズムを奏でてくれるようになり、町の人気者になった。" },
                    Ending { label: "元の静かな谷へ返す", text: "お礼に珍しい黒鉄の欠片を残して、精霊は嬉しそうに山の奥へ帰っていった。" },
                ]),
            },
            Story {
                title: "療養所に届く小鈴",
                beats: &[
                    Beat::speech(7, "職人", Worker, "もうすぐ療養所を退所する子に、歩いたときにきれいな音で知らせてくれる小鈴を贈りたいんだ。"),
                    Beat::scene(2, "走る子の歩幅や手の力に合わせて金属片を薄く削り直し、一番透き通る優しい音色に調律した。"),
                    Beat::scene(7, "小鈴を鳴らして嬉しそうに歩く姿に、療養所のみんなも笑顔で拍手を送った。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "炉守の宿場",
        profile: "馬車と魔導列車の乗り継ぎを担う街道の宿場。宿帳に交代の時刻を記し、山間への薬便と燃料貨物を支えている。",
        terrain: Landscape::Crossing,
        entry: (156, 174),
        places: [
            Place { name: "煉瓦煙突の炉守宿", at: (156, 98), art: Some(Art::Landmark), ground: Landscape::Square, detail: "大きな煉瓦の煙突と荷物台を備えた街道の中心宿。" },
            Place { name: "北の積替駅", at: (156, 48), art: Some(Art::StoneGate), ground: Landscape::Square, detail: "列車から街道の荷馬車へ荷を積み替える駅舎。" },
            Place { name: "東の馬車厩", at: (224, 102), art: Some(Art::Cottage), ground: Landscape::Pasture, detail: "山間便の馬が休む厩と運行控えの詰め所。" },
            Place { name: "西の荷預かり倉", at: (84, 102), art: Some(Art::Inn), ground: Landscape::Yard, detail: "乗り継ぎ待ちの薬や食糧、工具箱を預かる倉庫。" },
            Place { name: "宿の屋根裏点検口", at: (176, 82), art: None, ground: Landscape::Square, detail: "煙突の熱気や駅からの温風の流れを確かめられる場所。" },
            Place { name: "駅前の誘導歩道", at: (126, 68), art: None, ground: Landscape::Square, detail: "目の見えない旅人も音で歩ける駅の前の通路。" },
            Place { name: "西の荷役搬入口", at: (98, 134), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "駅前の混雑を避けて荷を運べる別の搬入口。" },
            Place { name: "南の山間馬車乗り場", at: (186, 138), art: None, ground: Landscape::Yard, detail: "列車が入れない山間の村へ向かう最後の馬車乗り場。" },
        ],
        stories: [
            Story {
                title: "積替駅に残った薬",
                beats: &[
                    Beat::speech(2, "馬方", Worker, "列車の優先貨物が増えたせいで、山間へ送る大切な薬と食糧が倉に置きっぱなしになっちまってるんだ！"),
                    Beat::scene(1, "積替駅の時刻表と厩の控えを見比べると、薬だけでも小口便で先に出すか、積み替えの順番を組み直せば間に合いそうだ。"),
                    Beat::speech(1, "駅務長", Guard, "燃料も急ぎだが薬も待たせられない。薬を小口便で先に出すか、燃料を止めずに積替の作業順を組み直そうか？"),
                ],
                choices: Some([
                    Ending { label: "薬を小口便で先に送る", text: "山間の診療所へ薬が時間どおり届き、村の人々から感謝の手紙が届いた。" },
                    Ending { label: "積替の作業順を組み直す", text: "手際よい段取りのおかげで燃料も薬も遅れることなく出発できた。" },
                ]),
            },
            Story {
                title: "煙突の火照り",
                beats: &[
                    Beat::speech(0, "宿泊客", Traveler, "暖炉は消えてるのに、部屋の中の煙突が夜通しぽかぽか熱くて寝苦しいんだよ……。"),
                    Beat::scene(4, "屋根の点検口を調べると、隣の駅舎から抜ける温風が煙突を逆流していた。"),
                    Beat::speech(0, "宿の主人", Villager, "原因が分かってよかった。逆流を止める仕切りを付けるか、この余熱をそのまま共同浴場へ回して役立てようか？"),
                ],
                choices: Some([
                    Ending { label: "仕切りを付けて熱を止める", text: "客室が涼しく快適になり、疲れた職工たちもぐっすり眠れるようになった。" },
                    Ending { label: "共同浴場へ熱を回す", text: "薪を使わずにいつでも温かいお風呂が沸くようになり、宿の名物になった。" },
                ]),
            },
            Story {
                title: "発車を知らせる靴音",
                beats: &[
                    Beat::speech(5, "目の見えない旅人", Traveler, "駅の案内放送が反響して聞き取りづらくて、発車の時刻を聞き逃してしまいそうなんだ……。"),
                    Beat::scene(5, "厩から待合まで踏むと違う音が鳴る木の誘導板を試し、出発前に駅員が必ず声を掛けに行く約束を作った。"),
                    Beat::scene(5, "旅人は迷わず列車の乗り口まで歩けるようになり、安心して旅を続けられた。"),
                ],
                choices: None,
            },
            Story {
                title: "忘れ物の工具箱",
                beats: &[
                    Beat::speech(3, "倉庫番", Worker, "預かり期限が過ぎたまま引き取りに来ない工具箱があるんだ。炉番さんの手書きの印が付いてるんだけどね。"),
                    Beat::scene(2, "宿帳と荷役の交代記録を照合して持ち主の行き先を突き止め、帰りの荷車便に箱の届け先を託した。"),
                    Beat::scene(3, "数日後、大切な商売道具を受け取った炉番から立派なお礼の手紙が届いた。"),
                ],
                choices: None,
            },
            Story {
                title: "一番遅い馬車",
                beats: &[
                    Beat::speech(7, "最後の馬車番", Worker, "列車にほとんどのお客を取られてしまって、もう俺の馬車を必要とする人なんていないのかな……。"),
                    Beat::speech(2, "馬方仲間", Worker, "そんなことはないさ！　列車が止まらない途中の集落向けに近距離便を始めるか、希望日の小荷物集荷便を引き受けようよ！"),
                ],
                choices: Some([
                    Ending { label: "近距離の定期便を始める", text: "お年寄りや買い物帰りの住民に大好評で、毎日満席の楽しい馬車になった。" },
                    Ending { label: "希望日の集荷便を始める", text: "山の農家から『助かる』と注文が相次ぎ、馬車番は誇りを取り戻した。" },
                ]),
            },
        ],
    },
    Plan {
        name: "黄銅の工房村",
        profile: "精密な時計と調圧器を分業で作る細工師の村。共同の試作品には製作者と検査係二人の印が刻まれる。",
        terrain: Landscape::Basalt,
        entry: (156, 174),
        places: [
            Place { name: "黄銅の時計会館", at: (156, 92), art: Some(Art::Landmark), ground: Landscape::Square, detail: "大きな天文時計が正面で時を刻む細工師たちの会館。" },
            Place { name: "西の時計工房", at: (84, 102), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "繊細な歯車やばねを組み上げる時計職人の工房。" },
            Place { name: "東の圧力試験室", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "調圧器の安全性や見習いの試験を行う検査室。" },
            Place { name: "南の部品市", at: (156, 140), art: Some(Art::Stalls), ground: Landscape::Square, detail: "ねじやばねなど精密な部品が並ぶ専門の市。" },
            Place { name: "裏路地の隠れ工房", at: (116, 68), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "引退した元検査係や独立した修理人が暮らす路地。" },
            Place { name: "風の通る時計屋", at: (192, 128), art: Some(Art::Clock), ground: Landscape::Square, detail: "なぜか正午になると時計が止まってしまう家。" },
            Place { name: "会館裏のからくり台", at: (196, 68), art: None, ground: Landscape::Yard, detail: "夜中に鳴き出すからくり鳥の仕掛けを点検する場所。" },
            Place { name: "試験室の工具棚", at: (234, 128), art: None, ground: Landscape::Square, detail: "背の高い細工台に合わせて工具が並べられた棚。" },
        ],
        stories: [
            Story {
                title: "銘のない調圧器",
                beats: &[
                    Beat::speech(3, "親方", Worker, "作者の銘がない調圧器が出回っていて、危険な偽造品じゃないかと疑われてるんだ。すべて回収して処分すべきかな。"),
                    Beat::scene(4, "引退した検査係に見せると、わざと銘を消して試験したガルダの旧式試作品だと分かり、分解すると試験記号も一致した。"),
                    Beat::speech(2, "検査官", Guard, "安全な品だと証明できた。安全に使える部品だけを部品市に残すか、旧式の来歴ごと資料として保存しようか？"),
                ],
                choices: Some([
                    Ending { label: "安全な部品を市に残す", text: "貴重な精密部品が無駄にならず、村の修理屋たちの助けになった。" },
                    Ending { label: "来歴ごと資料として保存する", text: "ガルダの誠実な試行錯誤の記録として会館に飾られ、若い職人の手本になった。" },
                ]),
            },
            Story {
                title: "時計の遅い家",
                beats: &[
                    Beat::speech(5, "家の住人", Villager, "いくら標準時計に合わせても、うちの壁時計だけ正午になるとピタッと止まっちゃうんだよ！"),
                    Beat::scene(5, "小窓から差し込む昼の日差しと炉の熱で金属の針が伸び、文字盤に引っかかっていた。風通しのよい反対の壁へ掛け直すと元気に動き出した。"),
                    Beat::scene(5, "それからは一秒の狂いもなく時を刻み続け、家の時計が頼りになった。"),
                ],
                choices: None,
            },
            Story {
                title: "ばねを継ぐ姉妹",
                beats: &[
                    Beat::speech(1, "細工師の姉妹", Villager, "同じばねの注文なのに、姉さんは『頑丈さ』、私は『しなやかさ』が一番だって譲らなくて決まらないんです……。"),
                    Beat::speech(2, "試験室の親方", Worker, "それぞれの強みを試してみたよ。用途別に二種類を分けて納めるか、二つを組み合わせた新しい二重ばねを作ってみようか？"),
                ],
                choices: Some([
                    Ending { label: "用途別に二種類を分ける", text: "それぞれの長所を活かした注文が増え、姉妹はいいライバルとして認め合った。" },
                    Ending { label: "二種類を合わせて一つにする", text: "強くてしなやかな画期的なばねが完成し、姉妹の名を冠した名品になった。" },
                ]),
            },
            Story {
                title: "緊急鐘のからくり鳥",
                beats: &[
                    Beat::speech(0, "会館の番人", Worker, "標準時計のからくり鳥が夜中に鳴き出して、みんな炉の緊急停止の合図と間違えて飛び起きちまうんだよ！"),
                    Beat::scene(6, "工房裏からからくりを点検し、緊急鐘と重ならない優しい音色と昼間だけ鳴く時刻へ調整し直した。"),
                    Beat::scene(0, "夜は静かに眠れるようになり、昼の合図としては楽しく親しまれるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "細工台の背丈",
                beats: &[
                    Beat::speech(7, "小柄な見習い", Villager, "試験室の細工台が高すぎて、僕だけ工具に手が届きにくくて資格試験で実力を出し切れないんです……。"),
                    Beat::scene(2, "試験室の規則を見直し、高さを選べる台や安定した足場を使ってもよいことにしてもらった。"),
                    Beat::scene(7, "見習いは見事な腕前を披露して一発で試験に合格し、誰もが働きやすい試験室になった。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "黒曜の市場町",
        profile: "黒曜石や燃料を売る広場と廃熱を利用した温水噴水がある市場町。鉱石の質は検定所が厳しく調べている。",
        terrain: Landscape::Basalt,
        entry: (156, 174),
        places: [
            Place { name: "黒曜広場と温水噴水", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "赤い天幕が並び、温水の噴水で人がくつろぐ黒曜広場。" },
            Place { name: "北の鉱石検定所", at: (156, 48), art: Some(Art::Inn), ground: Landscape::Square, detail: "鉱石の質と採掘元の見本を保管している検定所。" },
            Place { name: "東の燃料市", at: (224, 102), art: Some(Art::Stalls), ground: Landscape::Square, detail: "石炭や火口石が山積みにされて取引される市。" },
            Place { name: "西の再生材置場", at: (84, 102), art: Some(Art::Cottage), ground: Landscape::Basalt, detail: "再利用できる鉄くずや鉱石を集めた小さな工房向けの置場。" },
            Place { name: "廃熱樋沿いの道", at: (116, 128), art: None, ground: Landscape::Basalt, detail: "噴水へ温かいお湯を送る石の樋に沿った裏道。" },
            Place { name: "見本棚の記念館", at: (192, 68), art: Some(Art::Cottage), ground: Landscape::Square, detail: "旧魔鉱坑から採掘された古い見本が並ぶ棚。" },
            Place { name: "石飾り屋台", at: (126, 74), art: Some(Art::Stalls), ground: Landscape::Square, detail: "光を当てると不思議な映像が映る黒曜石の店。" },
            Place { name: "再生材の競り台", at: (98, 134), art: None, ground: Landscape::Yard, detail: "小工房の職人たちが集まって資材を競り合う台。" },
        ],
        stories: [
            Story {
                title: "品位の違う見本",
                beats: &[
                    Beat::speech(1, "検定官", Guard, "鉱石の値ばかり上がるのに、最近持ち込まれる荷の質が落ちてるんだ。公表された採掘先と食い違ってる気もしてね。"),
                    Beat::scene(5, "古い見本と照合すると、公表されていない別の坑道から質の悪い鉱石を混ぜて運んでいたことが分かった。"),
                    Beat::speech(0, "商人会の代表", Villager, "不正は見逃せない。連邦との契約を審査し直すか、まずは小工房が困らないよう再生材の買付枠を作ろうか？"),
                ],
                choices: Some([
                    Ending { label: "連邦との契約を審査し直す", text: "不正な混ぜ物が一掃され、適正な価格で良質な鉱石が市場に戻った。" },
                    Ending { label: "再生材の買付枠を先に作る", text: "小工房が安定した価格で資材を手に入れられるようになり、町が活気づいた。" },
                ]),
            },
            Story {
                title: "火の粉を映す石",
                beats: &[
                    Beat::speech(6, "飾り職人", Worker, "拾った綺麗な黒曜石を磨いてたら、景色が一瞬遅れて映る不思議な石だったんだ。"),
                    Beat::scene(0, "噴水と燃料市の光で比べると、天墜ちのときの熱と残響がかすかに残る珍しい欠片だった。"),
                    Beat::speech(1, "検定官", Guard, "珍しい品だな。研究のために検定所へ預けるか、みんなが見られるよう広場の噴水のそばに由来を添えて飾ろうか？"),
                ],
                choices: Some([
                    Ending { label: "検定所へ預けて調べる", text: "島の歴史を知る大切な手がかりになり、職人には十分な礼金が渡された。" },
                    Ending { label: "広場の噴水脇に飾る", text: "不思議な輝きに旅人たちが足を止め、町の新しいシンボルになった。" },
                ]),
            },
            Story {
                title: "噴水に湯が戻る日",
                beats: &[
                    Beat::speech(0, "広場の休み客", Villager, "みんなが手を温めてた噴水のお湯が、今日はすっかり冷たくなっちまったんだよ……。"),
                    Beat::scene(4, "廃熱樋をたどって漏れを塞ぎ、燃料市の余熱を繋ぎ直したところ、再び心地よい湯気が噴水から立ち上った。"),
                    Beat::scene(0, "冷えた手を温めながらお喋りするいつもの光景が広場に戻ってきた。"),
                ],
                choices: None,
            },
            Story {
                title: "小工房の競り札",
                beats: &[
                    Beat::speech(7, "小工房の職人", Worker, "再生材の競りで大店ばかりが全部買い占めちまって、俺たち小さな工房のぶんがまったく回ってこないんだ！"),
                    Beat::speech(3, "置場の管理人", Worker, "大店と競り合わずに買える最小の小口枠を用意するか、小工房のみんなで共同の札を出せる仕組みにしようか？"),
                ],
                choices: Some([
                    Ending { label: "小口で買える枠を確保する", text: "必要な量だけ無理なく買えるようになり、どの小さな工房も仕事を続けられた。" },
                    Ending { label: "小工房が共同で札を出す", text: "職人同士が力を合わせて大店とも対等に競れるようになり、絆も深まった。" },
                ]),
            },
            Story {
                title: "見本棚に戻る石",
                beats: &[
                    Beat::speech(5, "引退した鉱夫", Villager, "自分が若い頃に掘り当てた記念の鉱石を、資料館の見本棚に寄贈したいんだ。"),
                    Beat::scene(1, "古い見本と見比べてどの坑道で採れた石かを突き止め、家族に思い出を伝える展示か、次の探鉱者に役立つ地層の並びに収めた。"),
                    Beat::scene(5, "鉱夫は誇らしげに見本棚を眺め、孫たちへ昔の冒険を語って聞かせた。"),
                ],
                choices: None,
            },
        ],
    },
];

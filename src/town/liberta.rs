use super::{Art, Beat, Ending, Landscape, Place, Plan, Story};
use crate::dialogue::Speaker::*;

pub(super) static LIBERTA: [Plan; 10] = [
    Plan {
        name: "潮待ちの港町",
        profile: "潮向きの変わる晩を過ごす船が集まる自由港。普段の係留順は港会が決めるが、海難時は救難船が最優先となる。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "青クレーンの岸壁", at: (156, 126), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "青い荷役クレーンと倉庫が並ぶ大きな深い岸壁。" },
            Place { name: "東の救難艇庫", at: (226, 124), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "霧の日もすぐに出航できるよう救難船が待機している。" },
            Place { name: "北の港会所", at: (156, 74), art: Some(Art::Inn), ground: Landscape::Square, detail: "出港予定と係留の順番が掲示されている港の中心。" },
            Place { name: "西の潮見台", at: (84, 108), art: Some(Art::Lighthouse), ground: Landscape::Shore, detail: "干潮の水深と沖の波の向きを見渡せる高台。" },
            Place { name: "倉庫裏の引き綱道", at: (126, 134), art: None, ground: Landscape::Shore, detail: "干潮時の浅瀬の深さを確かめられる裏道。" },
            Place { name: "風待ちの食堂", at: (196, 86), art: Some(Art::Hearth), ground: Landscape::Square, detail: "足止めされた船乗りたちが温かい食事をとる広場。" },
            Place { name: "東の外れ岸壁", at: (242, 102), art: Some(Art::Sluice), ground: Landscape::Dock, detail: "水深は浅めだが小さな舟なら安全に着けられる岸壁。" },
            Place { name: "波力クレーンの足場", at: (176, 104), art: None, ground: Landscape::Dock, detail: "波の力で動く古い仕掛けが組み込まれた整備足場。" },
        ],
        stories: [
            Story {
                title: "深い岸壁は一つ",
                beats: &[
                    Beat::speech(2, "倉主", Worker, "救難船に岸壁を譲ったら、穀物船の荷揚げが遅れて食糧が傷んじまうよ！"),
                    Beat::speech(1, "救難船の船員", Worker, "でも霧の中にまだ戻らない小舟がいるんだ。潮見台から干潮の水深を見れば、別の出し方があるかもしれない。"),
                    Beat::speech(0, "港会の代表", Villager, "東の岸壁なら救難艇を出せるし、小舟で穀物を分けて運ぶ手もある。どっちにする？"),
                ],
                choices: Some([
                    Ending { label: "救難艇を東の岸壁から出す", text: "救難艇はすぐに出航して小舟を見つけ出し、穀物も傷む前に倉へ運ばれた。" },
                    Ending { label: "穀物を小舟で分けて運ぶ", text: "深い岸壁を救難船に空けながら、食糧も濡らさずに陸揚げできた。" },
                ]),
            },
            Story {
                title: "何も積まない帰り船",
                beats: &[
                    Beat::speech(4, "港の荷役人", Worker, "荷物は空っぽなのに『帰り船』とだけ書かれた古い荷札が見つかったんだ。誰が何のために出した船なんだろう？"),
                    Beat::scene(4, "船倉の擦れ跡を調べると、船に乗れない家族のために空の小舟で海から見送る古い習わしだと分かった。"),
                    Beat::scene(1, "ちょうど今日、その見送りの小舟が出るという。家族は静かな入江から遠い海へ向けて手を合わせた。"),
                ],
                choices: None,
            },
            Story {
                title: "夜番のクレーン",
                beats: &[
                    Beat::speech(7, "夜警の船員", Guard, "誰もいない夜中に、青いクレーンが勝手にギシギシ動いてるんだよ！　気味が悪くてさ。"),
                    Beat::scene(7, "整備足場を調べると、波の力を蓄えて動く古い仕掛けの留め金が外れていた。"),
                    Beat::speech(0, "クレーン番", Worker, "留め金を修理して夜の間だけ完全に止めるか、この仕掛けを昼間の計量台へ移して役立てようか？"),
                ],
                choices: Some([
                    Ending { label: "留め金を直して夜は止める", text: "クレーンは昼だけ動くようになり、夜中の不気味な音もぴたりと止んだ。" },
                    Ending { label: "昼の計量台へ仕掛けを移す", text: "波の力で重い荷を楽に持ち上げられるようになり、昼の作業がぐっと楽になった。" },
                ]),
            },
            Story {
                title: "倉番の天気図",
                beats: &[
                    Beat::speech(3, "倉番", Worker, "文字を読むのが苦手で、毎日の波の高さや風の向きを全部絵で描いてきたんだ。船乗りの役に立つかな……。"),
                    Beat::scene(3, "潮見台でその絵の予報を試してみると、驚くほど正確に天気の変化が分かった。読み方を掲示板に添えて貼り出した。"),
                    Beat::scene(2, "言葉の違う遠くの船乗りまで絵を見て出航を相談できるようになり、倉番の天気図は港の名物になった。"),
                ],
                choices: None,
            },
            Story {
                title: "出港までの一皿",
                beats: &[
                    Beat::speech(5, "食堂の料理番", Worker, "時化で足止めされた船乗りたちに腹いっぱい食べさせたいんだけど、食材が倉庫ごとに少しずつしか余ってなくてね。"),
                    Beat::speech(5, "船乗り代表", Traveler, "集めた食材で今すぐ温かい鍋にして届けるか、次の航海に持って行ける塩漬けの保存食に仕立てようか？"),
                ],
                choices: Some([
                    Ending { label: "温かい鍋にして船へ届ける", text: "冷えた体が芯まで温まり、船乗りたちは元気を取り戻して出航の日を迎えた。" },
                    Ending { label: "次の航海用の保存食にする", text: "日持ちする美味しい保存食ができ、長い航海でも安心して食べられるようになった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "帆布の市場街",
        profile: "縞帆布の天幕の下に店が並ぶ市場街。取引の揉め事は買い手代表を含む仲裁会が公平に裁いている。",
        terrain: Landscape::Square,
        entry: (156, 174),
        places: [
            Place { name: "縞天幕の帆布市場", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "青と白の縞帆布が市場全体を覆うにぎやかな市場。" },
            Place { name: "西の試帆場", at: (84, 102), art: Some(Art::Palisade), ground: Landscape::Shore, detail: "海風に帆を張って丈夫さを確かめる広場。" },
            Place { name: "南の仕立屋横丁", at: (156, 142), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "腕利きの帆縫い職人がミシンや針を動かす通り。" },
            Place { name: "北の卸倉と仲裁所", at: (156, 48), art: Some(Art::Inn), ground: Landscape::Square, detail: "布地の保管倉と、取引の争いを解決する仲裁所。" },
            Place { name: "天幕の支柱置場", at: (216, 86), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "大風のあと天幕を張り直すための予備の柱。" },
            Place { name: "紋章外しの工房", at: (116, 128), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "古い家の紋章を外して無地の帆へ仕立て直す店。" },
            Place { name: "市場の屋根上道", at: (192, 118), art: None, ground: Landscape::Square, detail: "風の向きで案内旗がどう揺れるか見える通り。" },
            Place { name: "桟橋前の待合所", at: (224, 134), art: None, ground: Landscape::Dock, detail: "船旅へ出ようとする若い縫い手が荷物を抱えている。" },
        ],
        stories: [
            Story {
                title: "裂けたのは糸か布か",
                beats: &[
                    Beat::speech(1, "試帆場の船乗り", Traveler, "安売りの帆を買ったら、強い風を受けた途端に縫い目からバリッと裂けたんだ！　危なくて海に出られないよ。"),
                    Beat::scene(3, "縫い糸と出荷札を調べると、卸から届いた糸の一巻だけが水濡れで弱っていた。ほかの帆はすべて丈夫だ。"),
                    Beat::scene(0, "不良の一巻を使った帆だけを回収して無料で縫い直し、市場全体の信用が守られた。"),
                ],
                choices: None,
            },
            Story {
                title: "天幕を張る朝",
                beats: &[
                    Beat::speech(4, "市場の世話役", Worker, "昨夜の大風で市場の天幕が外れちまってね。開市までに全部は張り直せないんだ。どこから先に張ろうか？"),
                    Beat::speech(0, "食品店の主人", Villager, "陽に当たると傷みやすい食品の区画を先に覆うか、お客さんが並ぶ風上の歩道を先に覆うか、どっちがいいだろう？"),
                ],
                choices: Some([
                    Ending { label: "食品の区画から張る", text: "生鮮品が新鮮なまま保たれ、朝一番の買い物客が次々と集まった。" },
                    Ending { label: "風上の歩道を先に覆う", text: "買い物を待つお客さんが風を避けられ、みんなで協力して残りの天幕も張り終えた。" },
                ]),
            },
            Story {
                title: "色を持たない帆",
                beats: &[
                    Beat::speech(5, "旅立つ船乗り", Traveler, "古い家のしがらみを離れて自分の船を持ちたいんだ。家の紋章を外した無地の帆に仕立て直せないかな。"),
                    Beat::speech(2, "仕立屋", Worker, "縫い跡がまったく見えない一色の新しい帆に仕立てるか、家族の記念になる小さな縁取りだけ残した帆にするか、どっちがいいかい？"),
                ],
                choices: Some([
                    Ending { label: "縛られない一色の帆にする", text: "真っ白な帆が朝の風をいっぱいにはらみ、新しい旅立ちにぴったりだった。" },
                    Ending { label: "家族の縁取りだけ残す", text: "自分の色と家族への感謝がひとつになった美しい帆が完成した。" },
                ]),
            },
            Story {
                title: "お針子の乗船券",
                beats: &[
                    Beat::speech(7, "若い縫い手", Villager, "せっかく船に乗る準備をしたのに、急ぎの修理依頼が入って出航に間に合いそうにないんです……。"),
                    Beat::speech(2, "仕立屋の仲間", Worker, "みんなで資金を出し合って代わりの職人を頼むか、今日の修理を手順書にまとめて他の店に引き継ぐか、協力しようよ！"),
                ],
                choices: Some([
                    Ending { label: "共同で代役を手配する", text: "仲間の応援で依頼が無事に終わり、縫い手は笑顔で船に乗り込んだ。" },
                    Ending { label: "手順を残して店に引き継ぐ", text: "丁寧な作業書のおかげで修理も滞りなく進み、後からいい便りも届いた。" },
                ]),
            },
            Story {
                title: "旗のひらく方向",
                beats: &[
                    Beat::speech(6, "市場の案内役", Villager, "建物の間の風向きが複雑で、案内旗が支柱に巻き付いてしまい、お客さんが同じ通路をぐるぐる回ってるんだ。"),
                    Beat::speech(0, "市場の世話役", Worker, "風の乱れない広い角へ旗を移すか、地面に色分けした織布の道しるべを敷くか、どっちが分かりやすいかな？"),
                ],
                choices: Some([
                    Ending { label: "風の抜ける角へ旗を移す", text: "青と白の旗がきれいに翻るようになり、通りがひと目で分かるようになった。" },
                    Ending { label: "地面に織布の標を敷く", text: "足元を見るだけで目的の店へたどり着けるようになり、迷う客がいなくなった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "運河の船大工街",
        profile: "大型船を陸に上げて直せる乾ドックを中心に栄えた街。徒弟たちも交代でドックの安全点検を受け持つ。",
        terrain: Landscape::Canal,
        entry: (156, 174),
        places: [
            Place { name: "乾ドックの船台", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "大きな船を木枠で支えて修理する中心の乾ドック。" },
            Place { name: "北の水門機関室", at: (156, 48), art: Some(Art::Sluice), ground: Landscape::Square, detail: "運河の水位と水門の開閉時間を管理する建物。" },
            Place { name: "東の徒弟工房", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Yard, detail: "若い徒弟たちが小舟の修理や道具作りを学ぶ工房。" },
            Place { name: "西の小舟船台", at: (84, 102), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "小さな漁船を引き上げて直せる予備の船台。" },
            Place { name: "運河沿いの作業歩道", at: (196, 72), art: None, ground: Landscape::Dock, detail: "ドックの壁の目地を間近で確かめられる狭い歩道。" },
            Place { name: "展示用の古船台", at: (126, 126), art: None, ground: Landscape::Dock, detail: "昔の船の竜骨を大切に展示している屋根の下。" },
            Place { name: "荷車の車輪工房", at: (218, 134), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "道具箱を運ぶ台車の幅や車輪を調整する店。" },
            Place { name: "東の係留杭", at: (96, 138), art: None, ground: Landscape::Dock, detail: "二隻の船が並んで泊まっている静かな運河の隅。" },
        ],
        stories: [
            Story {
                title: "壁の目地を読む",
                beats: &[
                    Beat::speech(2, "船大工頭", Worker, "急ぎの大型船を入れたいのに、浸水した小漁船も直さなきゃならない。水門を少しだけ開けて両方入れられないかな。"),
                    Beat::scene(4, "運河沿いの歩道から壁を見ると、北側の壁に少し歪みがあった。慎重に水位を調節すれば二組同時に通せそうだが、西の予備船台に小舟を引く手もある。"),
                    Beat::speech(1, "水門番", Worker, "壁の状態は分かったよ。時刻表に合わせて慎重に二組を入れるか、西の予備船台へ小漁船を回すか、どっちにしよう？"),
                ],
                choices: Some([
                    Ending { label: "小開きで二組を安全に入れる", text: "水門番の見事な調整で壁を傷めず、両方の船が予定どおり修理できた。" },
                    Ending { label: "予備船台へ小漁船を引く", text: "漁船はすぐ陸に上げられて修理が終わり、ドックの壁も安全に保たれた。" },
                ]),
            },
            Story {
                title: "抜けない竜骨",
                beats: &[
                    Beat::speech(5, "展示係", Villager, "古い船の部材を展示しようと外した途端、ドックの水が妙に逆巻いて騒がしくなったんだよ！"),
                    Beat::scene(5, "古い船大工たちの掛け声を思い出しながら同じ手順で部材を元の位置に収め直すと、水面のうねりがすっと鎮まった。"),
                    Beat::scene(0, "そのままの形で解説板を添えて飾ることになり、町の名所になった。"),
                ],
                choices: None,
            },
            Story {
                title: "君の最初の浸水",
                beats: &[
                    Beat::speech(2, "若い徒弟", Villager, "僕が初めて直した小舟に水が漏れてるんだ……。怒られるのが怖くて、親方に言えずにいるんだよ。"),
                    Beat::scene(3, "一緒に舟底に水が入る順番を確かめて漏れの原因を見つけ、堂々と親方へ報告しに行った。"),
                    Beat::scene(2, "親方は正直に話したことを褒め、次の再試験の日まで一緒に直してくれた。"),
                ],
                choices: None,
            },
            Story {
                title: "溺れない台車",
                beats: &[
                    Beat::speech(6, "作業員", Worker, "歩道が狭いせいで、道具箱を載せた台車が何度も運河に落ちちまうんだ。なんとかならないかな。"),
                    Beat::speech(6, "車輪職人", Worker, "手すりの幅に合わせて試作品を作ったよ。船の上でも使える軽い車輪と、歩道専用の安定した固定輪、どっちを選ぶ？"),
                ],
                choices: Some([
                    Ending { label: "船上でも使える軽量輪にする", text: "ドックと甲板の両方を楽に運べるようになり、作業がぐっとはかどった。" },
                    Ending { label: "歩道専用の固定輪にする", text: "びくともせず真っすぐ走るようになり、道具が落ちる事故はゼロになった。" },
                ]),
            },
            Story {
                title: "隣の船だけ鳴る",
                beats: &[
                    Beat::speech(7, "船番", Worker, "同じ場所に泊めてる二隻の船のうち、片方だけが夜になるとギシギシ鳴るんだよ。"),
                    Beat::scene(7, "潮が変わる時刻に調べると、片方の張り綱がきつすぎたうえ船腹に小さな擦れ傷があった。"),
                    Beat::scene(3, "傷を直して綱の結び方を変えると、その晩から波の音だけが聞こえる静かな夜に戻った。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "三潮灯台町",
        profile: "三つの潮がぶつかる難所を照らす灯台町。灯台守と水先案内人の会が信号の決まりを定めている。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "岬端の三潮灯台", at: (156, 128), art: Some(Art::Landmark), ground: Landscape::Shore, detail: "三方向からの潮を見張る背の高い石造りの灯台。" },
            Place { name: "中央の水先案内所", at: (156, 86), art: Some(Art::Inn), ground: Landscape::Square, detail: "船を安全に導く水先案内人たちが詰めている建物。" },
            Place { name: "東の救難詰所", at: (226, 108), art: Some(Art::Cottage), ground: Landscape::Square, detail: "暗礁や遭難の連絡を受ける救難隊の詰め所。" },
            Place { name: "西崖の観測小屋", at: (84, 106), art: Some(Art::Palisade), ground: Landscape::Heath, detail: "沖の暗礁と潮の流れを測る西の崖の小屋。" },
            Place { name: "岬の巡回路", at: (192, 134), art: None, ground: Landscape::Shore, detail: "夜明け前の謎の光がどこから来るか確かめられる道。" },
            Place { name: "風裏の浜", at: (98, 136), art: None, ground: Landscape::Shore, detail: "ガラス瓶が流れ着く穏やかな入江の浜。" },
            Place { name: "東崖の信号台", at: (236, 74), art: Some(Art::Lighthouse), ground: Landscape::Heath, detail: "天気が荒れたときも暗礁の位置を見極められる台。" },
            Place { name: "町の集会場", at: (122, 68), art: Some(Art::Hearth), ground: Landscape::Square, detail: "灯台守の家族たちが祝いの準備をする広場。" },
        ],
        stories: [
            Story {
                title: "いまはない信号",
                beats: &[
                    Beat::speech(1, "水先案内人", Guard, "夜明け前に、いま使われていないはずの点滅信号が沖に見えたんだ。密輸の合図じゃないかと思うんだが……。"),
                    Beat::scene(4, "岬の巡回路で光の向きを確かめ、引退した灯台守に聞くと、それは昔の潮裂きの砦で使われていた救難の合図だった。"),
                    Beat::speech(0, "灯台守", Lightkeeper, "誰かが助けを呼んでるのかもしれない。昔の救難符号で一夜だけ返事を送るか、沖の小舟向けに安全な別の灯を送ろうか？"),
                ],
                choices: Some([
                    Ending { label: "昔の救難符号で応答する", text: "数時間後、迷っていた小舟が合図を頼りに無事港へたどり着き、フィーネの捜索も一歩進んだ。" },
                    Ending { label: "安全な誘導灯を送る", text: "暗礁を避ける航路がはっきり示され、明け方にはすべての船が安全に通り抜けた。" },
                ]),
            },
            Story {
                title: "三本の流れの試験",
                beats: &[
                    Beat::speech(1, "見習い水先人", Villager, "三つの潮がぶつかる場所の流れをいつも読み違えてしまって、試験に受からないんです……。"),
                    Beat::scene(3, "西崖と岬の両方から浮標が流れていく向きを一緒に観察し、潮止まりと逆潮の境目の見分け方を教えた。"),
                    Beat::scene(1, "見習いは次の実技試験でぴったり進路を言い当て、晴れやかな顔で水先人の帽子を受け取った。"),
                ],
                choices: None,
            },
            Story {
                title: "灯守の休暇",
                beats: &[
                    Beat::speech(0, "灯台守", Lightkeeper, "交代できる人がいなくて、家族のお祝いの席にも顔を出せそうにないんだよな……。"),
                    Beat::speech(2, "救難詰所の若者", Guard, "俺たちが信号の合図を練習して夜の番を引き受けるか、昼の点検だけ別の班に代わってもらおうか？"),
                ],
                choices: Some([
                    Ending { label: "救難班に夜の見回りを任せる", text: "灯台守は久しぶりに家族とゆっくり食卓を囲み、若い衆も大きく成長した。" },
                    Ending { label: "昼の点検を別の班に頼む", text: "日中のうちに家族の祝いに参加でき、夜の大切な点灯はベテランがしっかり守った。" },
                ]),
            },
            Story {
                title: "暗礁が動いた日",
                beats: &[
                    Beat::speech(3, "観測員", Worker, "大嵐のあと、海の岩影の位置が古い海図とズレてる気がするんだ。見間違えかなあ。"),
                    Beat::scene(6, "西崖と東崖の二か所から角度を測り直すと、別の岩に付いていた古い目印が波で外れていたことが分かった。"),
                    Beat::scene(3, "暗礁の両端に新しい浮標を浮かべたおかげで、夜の航行も安心して続けられるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "瓶の中の晩鐘",
                beats: &[
                    Beat::speech(4, "夜番の住民", Villager, "夜の岬で、灯台とは別の方角から小さく澄んだ鐘の音が聞こえてくるんだ。"),
                    Beat::scene(5, "風裏の浜へ行くと、波の精霊が流れ着いたガラス瓶に心地よい潮音を閉じ込めていた。"),
                    Beat::scene(5, "瓶を壊さずに見晴らしのよい岩棚に並べたところ、夜番に時刻を知らせる優しい海の時計になった。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "網干しの村",
        profile: "帰港の合図を網の干し方で知らせる漁村。漁場や共同金庫の使い道は浜の世帯全員で話し合う。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "網干しの桟橋小屋", at: (156, 126), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "軒先に漁網を干す木枠と桟橋がつながる高床の家。" },
            Place { name: "西の網修繕屋", at: (84, 108), art: Some(Art::Cottage), ground: Landscape::Shore, detail: "破れた網を器用な手つきで繕う職人の作業場。" },
            Place { name: "中央の浜会所", at: (156, 82), art: Some(Art::Inn), ground: Landscape::Square, detail: "水揚げの相談や祝い事の支度をする集会所。" },
            Place { name: "東の産卵浜と選別場", at: (224, 122), art: Some(Art::Stalls), ground: Landscape::Shore, detail: "朝の水揚げを大きさごとに選り分ける浜辺。" },
            Place { name: "高台の網干し場", at: (116, 68), art: None, ground: Landscape::Heath, detail: "繕い終わったばかりの綺麗な網を乾かす場所。" },
            Place { name: "山側の宴会場", at: (188, 62), art: Some(Art::Hearth), ground: Landscape::Square, detail: "大漁の日に村人が集まって料理を振る舞う広場。" },
            Place { name: "西の浅瀬浜", at: (88, 136), art: None, ground: Landscape::Shore, detail: "海獣の子どもがときおり迷い込んでくる静かな浜。" },
            Place { name: "船大工の道具棚", at: (218, 86), art: None, ground: Landscape::Yard, detail: "網を固定する木枠や釣り道具が並んでいる。" },
        ],
        stories: [
            Story {
                title: "網目一つぶん",
                beats: &[
                    Beat::speech(3, "選別場の漁師", Worker, "最近、網にまだ小さい幼魚まで掛かっちまうんだ。しばらく漁場を休ませたいんだが、生活が苦しい家もあってね。"),
                    Beat::scene(3, "目の違う網で試すと、ひと回り網目の粗い網を使えば幼魚を逃がして成魚だけを獲れると分かった。"),
                    Beat::speech(2, "浜会所の世話役", Villager, "いい結果が出たね。短期だけ休漁して共同金庫で支えるか、粗い網を全戸に貸し出して漁を続けようか？"),
                ],
                choices: Some([
                    Ending { label: "短期休漁して共同金庫を使う", text: "魚たちがしっかり育つまで休み、再開した日には見事な大漁に沸いた。" },
                    Ending { label: "粗い網を貸して漁を続ける", text: "幼魚を逃がしながら毎日安定した水揚げがあり、どの家も安心して暮らせた。" },
                ]),
            },
            Story {
                title: "干し網を盗む鳥",
                beats: &[
                    Beat::speech(4, "漁師の妻", Villager, "せっかく直したばかりの網が、干しておくと端から少しずつ千切られちまうんだよ！"),
                    Beat::scene(4, "高い杭の上に海鳥の巣を見つけた。網を千切られないよう、巣作り用の短い麻縄を別の杭に結んでおいた。"),
                    Beat::scene(1, "海鳥は麻縄だけを喜んで運ぶようになり、大切な漁網は一度も傷つかなくなった。"),
                ],
                choices: None,
            },
            Story {
                title: "二軒の水揚げ祝い",
                beats: &[
                    Beat::speech(2, "浜会所の世話役", Villager, "同じ日に二軒が大漁の合図を揚げたせいで、招かれたお客さんがあちこちで迷ってるんだよ。"),
                    Beat::speech(5, "料理番", Worker, "浜と山側の両方を一か所にまとめて合同の大宴会にするか、道に分かりやすい看板を出して二か所に分かれようか？"),
                ],
                choices: Some([
                    Ending { label: "合同の大宴会を開く", text: "村じゅうの人がひとつの輪になって歌い踊り、忘れられない賑やかな夜になった。" },
                    Ending { label: "看板を立てて二か所で祝う", text: "それぞれの家の自慢の料理をゆっくり味わえ、両家とも大満足の祝いになった。" },
                ]),
            },
            Story {
                title: "網針を持つ左手",
                beats: &[
                    Beat::speech(1, "網修繕屋の職人", Worker, "右腕を怪我して、片手じゃ網をピンと張れなくてさ。せっかく頼まれた仕事も断るしかないのかな……。"),
                    Beat::scene(7, "船大工から網を固定できる木枠を借り、左手でも持ちやすい太めの網針に作り替えた。"),
                    Beat::scene(1, "小さなほころびから着実に直せるようになり、職人は自信を取り戻した。"),
                ],
                choices: None,
            },
            Story {
                title: "海獣の背の浮き玉",
                beats: &[
                    Beat::speech(6, "浜の子ども", Villager, "大変！　沖の浅瀬で仔海獣の背中に古い漁具の網が絡まって泳ぎにくそうにしてるよ！"),
                    Beat::speech(0, "漁師", Worker, "好物の小魚で岸へ誘って網をほどいてやるか、沖の救難艇を呼んで深いほうで助けてもらおうか？"),
                ],
                choices: Some([
                    Ending { label: "好物の魚で寄せてほどく", text: "仔海獣はおとなしく網を外してもらい、お礼のように水面を跳ねて海へ帰った。" },
                    Ending { label: "救難艇を呼んで助ける", text: "慣れた船員たちが手際よく網を切り離し、仔海獣は元気に親の元へ泳いでいった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "塩風の集落",
        profile: "風車で海水を汲み上げて塩を作る白壁の集落。高潮のときの堤の手入れは村全体の大切な務めだ。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "塩田と汲み上げ風車", at: (156, 106), art: Some(Art::Landmark), ground: Landscape::Square, detail: "海水を塩田へ汲み上げる白壁の塩蔵と大きな風車。" },
            Place { name: "南の白い塩田", at: (156, 140), art: None, ground: Landscape::Shore, detail: "太陽と風で白い塩の結晶が育つ平らな塩田。" },
            Place { name: "北の堤と浸水杭", at: (156, 68), art: Some(Art::Palisade), ground: Landscape::Yard, detail: "高潮の高さを示す赤い目印が打たれた堤。" },
            Place { name: "東の共同塩蔵", at: (224, 108), art: Some(Art::Inn), ground: Landscape::Square, detail: "できた塩を保管し、水を分け合う集会所。" },
            Place { name: "西の塩釜小屋", at: (84, 108), art: Some(Art::Hearth), ground: Landscape::Yard, detail: "汲み上げた濃い塩水を煮詰めて仕上げる釜小屋。" },
            Place { name: "風車裏の砂道", at: (116, 78), art: None, ground: Landscape::Shore, detail: "海辺から細かい砂が吹き上がりやすい風の通り道。" },
            Place { name: "塩蔵の木戸口", at: (206, 128), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "湿気で少し膨らんで隙間ができた古い木戸。" },
            Place { name: "南東の畦道", at: (204, 142), art: None, ground: Landscape::Yard, detail: "隣り合う塩田の持ち主が声を掛け合う畦道。" },
        ],
        stories: [
            Story {
                title: "堤の赤い印",
                beats: &[
                    Beat::speech(3, "塩田の借り手", Villager, "地主さんが『堤を全部直すから借地料を上げる』って言うんだけど、工事で塩田を休んだら払えないよ！"),
                    Beat::scene(2, "浸水杭の赤い印を見ると、高潮をかぶっていたのは堤の一部だけだった。"),
                    Beat::speech(2, "堤の世話役", Worker, "全部を一度に直す必要はないな。波をかぶる区間だけ先に直すか、工事で休む塩田の地代を一季免除してもらうか、どっちにする？"),
                ],
                choices: Some([
                    Ending { label: "危険な区間だけ先に直す", text: "予算も工期も半分で済み、ほとんどの塩田が休まずに塩を作り続けられた。" },
                    Ending { label: "休む塩田の地代を免除する", text: "借り手も安心して工事に協力でき、頑丈で美しい新しい堤が完成した。" },
                ]),
            },
            Story {
                title: "塩の結晶晩餐",
                beats: &[
                    Beat::speech(4, "塩焼き職人", Worker, "保存食屋さんから『今回の塩は苦くて使えない』って返されちまったんだ。どうしたんだろう……。"),
                    Beat::scene(1, "塩田と釜の塩を順になめて確かめると、にがりが抜けきらないうちに取り出した塩だけが苦かった。"),
                    Beat::scene(4, "料理用の甘い塩をしっかり選り分け、苦い塩は革なめし職人へ売ったところ、両方から大変喜ばれた。"),
                ],
                choices: None,
            },
            Story {
                title: "逆に回る風車",
                beats: &[
                    Beat::speech(0, "風車番", Worker, "風もない静かな夜に限って、なぜか風車がギィギィ逆向きに回ってるんだ。気味が悪くてね。"),
                    Beat::scene(5, "砂道を見張ると、夜の海辺で砂を運ぶ小さな精霊たちが羽根にいたずらして遊んでいた。"),
                    Beat::speech(0, "風車番", Worker, "可愛いいたずらだな。水を汲む時刻を昼間にずらすか、塩槽に砂が入らないよう覆いを架けておこうか？"),
                ],
                choices: Some([
                    Ending { label: "水を汲む時刻を昼にずらす", text: "夜は羽根を固定して休ませるようにし、精霊たちも別の浜で遊ぶようになった。" },
                    Ending { label: "塩槽に覆いを架ける", text: "きれいな塩を守りながら、精霊たちが回す風車の音も集落ののどかな風景になった。" },
                ]),
            },
            Story {
                title: "塩蔵の足形",
                beats: &[
                    Beat::speech(6, "塩蔵の番人", Guard, "朝来たら倉の床に白い足跡みたいな跡が残ってるんだ！　誰かが塩を盗みに入ったんじゃないか？"),
                    Beat::scene(6, "木戸の隙間を確かめると、雨で歪んだ隙間から夜風が塩の粉を吹き込み、足跡のような模様を作っていただけだった。"),
                    Beat::scene(6, "木戸の隙間を板で塞ぐと白い模様は出なくなり、番人も安心して眠れるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "畦越しの同窓会",
                beats: &[
                    Beat::speech(7, "塩田の持ち主", Villager, "昔からの友だちと隣同士の田を受け持ってるのに、お互い忙しくて全然顔を合わせられないんだよ。"),
                    Beat::scene(7, "二人の作業が終わる時刻を畦づたいに伝え合い、夕方一緒に風車を回して最後の桶を満たした。"),
                    Beat::scene(0, "夕焼けの下で久しぶりにお茶を飲みながら語り合い、また明日から頑張る力が湧いてきた。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "碧帆の町",
        profile: "碧い帆を合図に船団を守る商会連合の町。非加盟の船でも町の仲裁所を通せば公平な契約が結べる。",
        terrain: Landscape::Canal,
        entry: (156, 174),
        places: [
            Place { name: "碧帆の商会館", at: (156, 88), art: Some(Art::Landmark), ground: Landscape::Square, detail: "大きな碧い帆の天幕を掲げた連合商会の中心館。" },
            Place { name: "南の碧帆埠頭と倉", at: (156, 138), art: Some(Art::Sluice), ground: Landscape::Dock, detail: "護送船団と共同倉庫が並ぶ賑やかな埠頭。" },
            Place { name: "西の仲裁所", at: (84, 104), art: Some(Art::Inn), ground: Landscape::Square, detail: "契約の争いを公平に裁く独立した仲裁所。" },
            Place { name: "東の船具修理通り", at: (224, 104), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "船の傷や帆を修理し、記録札を残す職人の通り。" },
            Place { name: "運河の渡り橋", at: (116, 104), art: Some(Art::Bridge), ground: Landscape::Dock, detail: "埠頭から西の仲裁所へ渡る石造りの橋。" },
            Place { name: "埠頭の記念広場", at: (192, 132), art: Some(Art::Hearth), ground: Landscape::Square, detail: "船団の記念日に旗を掲げて祝う海沿いの広場。" },
            Place { name: "染料と試帆の庭", at: (96, 68), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "遠くからでも見分けやすい帆の印を試す庭。" },
            Place { name: "共同倉庫の隅", at: (196, 78), art: None, ground: Landscape::Dock, detail: "正規の取引印が押された古い茶箱が積まれた隅。" },
        ],
        stories: [
            Story {
                title: "保険帳の赤線",
                beats: &[
                    Beat::speech(2, "小舟の船長", Traveler, "海賊の被害が増えたからって、護送船団がうちの小さな港への寄港を断るようになったんだ。生活が立ち行かないよ。"),
                    Beat::scene(3, "仲裁所の保険控えと修理通りの札を突き合わせると、単なる悪天候の難破まで海賊襲撃として届け出られていた。"),
                    Beat::speech(2, "仲裁人", Villager, "不正が分かったよ。不当な寄港拒否を撤回させるか、小舟向けの負担の軽い保証率を新しく作ろうか？"),
                ],
                choices: Some([
                    Ending { label: "寄港拒否を撤回させる", text: "小さな港への航路が無事に戻り、船長たちは大喜びで荷を運び始めた。" },
                    Ending { label: "小舟専用の保証率を作る", text: "無理のない掛け金で誰もが保険に入れるようになり、安心して海へ出られるようになった。" },
                ]),
            },
            Story {
                title: "帆をあげない隊列",
                beats: &[
                    Beat::speech(5, "遺族の船主", Villager, "船団の記念日なのに、帆を上げられないうちの古い舟はパレードの列から外されちまったんだ……。"),
                    Beat::speech(1, "埠頭の船員", Worker, "そんなの寂しいよな。うちの船で後ろから曳いて一緒に進むか、倉庫の前から大きな祝いの小旗を上げて隊列を迎えようか？"),
                ],
                choices: Some([
                    Ending { label: "曳き船で一緒に隊列へ加える", text: "みんなの船に引かれて海を進む姿に、埠頭から温かい拍手が湧き起こった。" },
                    Ending { label: "倉庫前から祝いの小旗を振る", text: "色とりどりの旗が埠頭を埋め尽くし、海の仲間全員が主役の記念日になった。" },
                ]),
            },
            Story {
                title: "保証人の署名",
                beats: &[
                    Beat::speech(3, "若い舵取り", Traveler, "やっと自分の船を持って独立したんだけど、最初の荷の契約を結ぶための保証人が見つからないんだ。"),
                    Beat::scene(3, "修理通りで普段の誠実な仕事ぶりを聞き、まずは少額の初荷で信用を作るか、町の仲間数人で保証を分け合う道を見つけた。"),
                    Beat::scene(0, "保証人の署名が揃い、若い舵取りは晴れやかに最初の航海へ出発した。"),
                ],
                choices: None,
            },
            Story {
                title: "海賊が残した茶箱",
                beats: &[
                    Beat::speech(7, "倉庫番", Worker, "没収した海賊の品に混じっていた茶箱に、ちゃんとした正規の取引印が押してあるんだよ。"),
                    Beat::speech(2, "仲裁人", Villager, "海賊に奪われた荷物だな。元の荷主を探してそのまま返すか、留守中の品として市で売って代金を預かっておこうか？"),
                ],
                choices: Some([
                    Ending { label: "元の荷主を探して返す", text: "諦めていた茶箱が戻ってきた荷主は深く感謝し、町への信頼がいっそう深まった。" },
                    Ending { label: "価値を保って共同で売る", text: "お茶が傷む前に高値で売れ、戻ってきた荷主にしっかり代金が手渡された。" },
                ]),
            },
            Story {
                title: "帆に塗る碧",
                beats: &[
                    Beat::speech(6, "帆職人", Worker, "碧い染料が品薄で、いつものように帆全体を染められないんだ。遠くから仲間だと分かる印にしないと危ないのに。"),
                    Beat::scene(6, "夕暮れの海で試帆を行い、少ない染料でくっきり見える細い碧の帯のデザインと、染料の要らない旗の組み合わせを考えた。"),
                    Beat::scene(1, "夕霧の中でもひと目で味方の船だと見分けられるようになり、染料不足も乗り切った。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "潮路の宿場",
        profile: "満潮になると街道が海に沈むため、屋根付きの橋と両岸の宿を建てた宿場。通行時間は宿と橋守が掲示する。",
        terrain: Landscape::Canal,
        entry: (156, 174),
        places: [
            Place { name: "屋根付きの連絡橋宿", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "両岸の宿を結ぶ屋根付きの立派な橋。" },
            Place { name: "東岸の馬車宿と厩", at: (224, 96), art: Some(Art::Inn), ground: Landscape::Square, detail: "定期馬車と馬の世話を引き受ける東岸の宿。" },
            Place { name: "西岸の食堂宿", at: (84, 96), art: Some(Art::Inn), ground: Landscape::Square, detail: "議事の掲示板が掛かる賑やかな食堂付きの宿。" },
            Place { name: "南の旧石堤と潮目盛り", at: (156, 142), art: Some(Art::Sluice), ground: Landscape::Shore, detail: "干潮の時間だけ歩いて渡れる古い石造りの堤。" },
            Place { name: "橋の屋根の下", at: (156, 76), art: None, ground: Landscape::Dock, detail: "雨風をしのげる広い屋根付きの踊り場。" },
            Place { name: "旧石堤の浅瀬", at: (196, 134), art: None, ground: Landscape::Shore, detail: "水の精霊が迷い込んで潮の流れを乱していた場所。" },
            Place { name: "厩の預かり棚", at: (234, 122), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "客の荷物札や馬の手配控えを保管する棚。" },
            Place { name: "西岸の料理台", at: (96, 128), art: Some(Art::Hearth), ground: Landscape::Square, detail: "橋の補修を終えた職人たちをもてなす料理台。" },
        ],
        stories: [
            Story {
                title: "橋を借りる時刻",
                beats: &[
                    Beat::speech(1, "馬車宿の主人", Villager, "新しい定期馬車が『満潮の前に橋を貸し切りたい』って言ってるんだ。でも通院で歩いて渡る住民もいるしなあ。"),
                    Beat::scene(3, "潮目盛りを調べると、干潮前後なら南の旧石堤を歩いて渡れると分かった。"),
                    Beat::speech(2, "橋守", Worker, "石堤のほうへ徒歩客を案内するか、馬車の時間を一便ずつずらして共有してもらうか、どっちがいいだろう？"),
                ],
                choices: Some([
                    Ending { label: "旧石堤へ徒歩客を案内する", text: "干潮の美しい景色を眺めながら安全に渡れる道ができ、馬車も遅れず走れた。" },
                    Ending { label: "馬車の便を分けて共有する", text: "歩行者と馬車の時間がはっきり分かれ、誰でも安心して橋を渡れるようになった。" },
                ]),
            },
            Story {
                title: "橋の中の花嫁",
                beats: &[
                    Beat::speech(4, "婚礼の世話役", Villager, "対岸同士の二組が結婚するのに、潮が満ちて来て客が全員向こう岸へ渡りきれないんだよ！"),
                    Beat::speech(4, "橋守", Worker, "濡れない今のうちにみんな渡り切って向こうの宿で祝うか、この屋根付き橋の真ん中に席を作ってここで式を挙げようか？"),
                ],
                choices: Some([
                    Ending { label: "渡り切って宿で祝う", text: "少し急いだが全員が濡れずに渡り切り、温かい食堂で盛大な披露宴が開かれた。" },
                    Ending { label: "橋の屋根の下で式を挙げる", text: "両岸の家族が橋の真ん中で手を取り合い、潮風に祝福される素敵な結婚式になった。" },
                ]),
            },
            Story {
                title: "潮が忘れた砂時計",
                beats: &[
                    Beat::speech(3, "橋守", Worker, "最近、砂時計で測っている時刻よりもずっと早く満潮が来ちまうんだよ。おかしいなあ。"),
                    Beat::scene(5, "旧石堤の岩陰に、閉じ込められて困っていた水の精霊を見つけた。岩をどけて浅瀬へ逃がすと、潮のリズムが元どおりになった。"),
                    Beat::scene(3, "砂時計と潮の満ち引きが再びぴったり合うようになり、橋守もほっと胸をなで下ろした。"),
                ],
                choices: None,
            },
            Story {
                title: "濡れた宿帳",
                beats: &[
                    Beat::speech(2, "食堂宿の女将", Villager, "対岸へ届けた宿帳が波をかぶって、お客さんの予約の文字が滲んで読めなくなっちまったんだよ！"),
                    Beat::scene(6, "厩の預かり札と食堂の人数控えを突き合わせて予約をすべて復元し、水に濡れても消えない木札の予約方式に切り替えた。"),
                    Beat::scene(1, "両方の宿で手違いが起きなくなり、旅人たちも安心して泊まれるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "旅立つ石積み屋",
                beats: &[
                    Beat::speech(7, "食堂の料理番", Worker, "橋を綺麗に直してくれた石積み職人が明日発つんだ。お礼に最後の豪華な食事をみんなで囲みたくてね。"),
                    Beat::scene(3, "職人が石堤に残した目印をたどって各地にいる教え子たちにも声を掛け、橋が通れる時刻に食卓を整えた。"),
                    Beat::scene(7, "教え子たちも駆けつけた賑やかな送別会が開かれ、職人は目に涙を浮かべて喜んだ。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "小舟の里",
        profile: "廃村から引き上げた小舟を直し、色を塗り分けて助け合う漁師の里。予備の舟は急病の往診にも使われる。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "色舟の棚と修理台", at: (156, 122), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "赤・青・黄の鮮やかな小舟が並んだ舟棚と修理台。" },
            Place { name: "南の共同舟揚げ浜", at: (156, 146), art: None, ground: Landscape::Shore, detail: "潮が引くと浅瀬の斜路を歩いて渡れる浜辺。" },
            Place { name: "西の浜会小屋", at: (84, 108), art: Some(Art::Inn), ground: Landscape::Square, detail: "拾得された舟や漁の順番を話し合う小屋。" },
            Place { name: "北の旧村を望む岬", at: (156, 72), art: Some(Art::Lighthouse), ground: Landscape::Heath, detail: "沈み桟橋の廃村の方角を見渡せる見晴らしの岬。" },
            Place { name: "舟棚の塗り場", at: (218, 108), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "家の印となる青や赤のペンキが並んでいる仕事場。" },
            Place { name: "霧笛の浅瀬", at: (98, 138), art: None, ground: Landscape::Shore, detail: "貝笛を吹き合って霧の中の舟を導く場所。" },
            Place { name: "東の係留杭", at: (226, 138), art: None, ground: Landscape::Dock, detail: "波の向きに合わせて舟の結び方を工夫している杭。" },
            Place { name: "往診の控え所", at: (116, 86), art: Some(Art::Stalls), ground: Landscape::Square, detail: "冬の往診便に使う予備のオールと薬箱を置く場所。" },
        ],
        stories: [
            Story {
                title: "往診舟は誰のもの",
                beats: &[
                    Beat::speech(2, "亡き船主の親族", Traveler, "廃村から引き上げて直されたあの小舟は、うちの親族の遺品だと分かったんだ。引き取って帰りたいんだよ。"),
                    Beat::speech(0, "舟棚の番人", Worker, "お気持ちは分かりますが、あの浅瀬を渡れる舟は冬の往診に欠かせなくてね……。似た船底の代わりの舟を探すか、春まで借りる約束を結べませんか？"),
                ],
                choices: Some([
                    Ending { label: "似た船底の代船を探す", text: "別の廃材から同じ形の丈夫な舟が見つかり、遺品の舟は安心して親族のもとへ帰った。" },
                    Ending { label: "冬の往診に春まで借りる", text: "親族も里の助け合いを快く受け入れ、春に綺麗な状態で返されることになった。" },
                ]),
            },
            Story {
                title: "青い舟を塗り直す日",
                beats: &[
                    Beat::speech(4, "若い船主", Villager, "先代から受け継いだ青い舟を塗り直すんだけど、そのまま同じ青を受け継ぐか、自分の色に変えるか迷ってるんだ。"),
                    Beat::speech(0, "舟棚の番人", Worker, "水面の線にだけ先代の青を残して上を新しい色にするか、思い切ってまったく新しい色で出直そうか？"),
                ],
                choices: Some([
                    Ending { label: "水線に先代の青を残す", text: "先代への敬意と新しい決意が並んだ美しい舟が、海の上でひときわ輝いた。" },
                    Ending { label: "新しい色で塗り直す", text: "鮮やかな新しい色の舟を見て、先代を知る年寄りたちも『よく似合ってる』と喜んだ。" },
                ]),
            },
            Story {
                title: "浜通いの貝笛",
                beats: &[
                    Beat::speech(5, "浜の見張り", Worker, "浅瀬に濃い霧が立ち込めて、向こうで鳴ってる貝笛の舟がどこにいるか見えないんだ！"),
                    Beat::scene(1, "音の高低を聴き分けて迷った方角を絞り、舟揚げ浜から同じ節の貝笛を吹き返して安全な斜路へ導いた。"),
                    Beat::scene(5, "霧の中から無事に戻ってきた漁師たちは、見事な合図に何度も礼を言った。"),
                ],
                choices: None,
            },
            Story {
                title: "風にほどける舟結び",
                beats: &[
                    Beat::speech(6, "漁師", Worker, "同じ結び方でつないでおいたのに、東の杭に留めた舟だけが風で流されちまうんだ。"),
                    Beat::scene(3, "岬から波の向きを観察し、舟の低い位置に綱を通し直して予備の縄を結ぶ方法を試した。"),
                    Beat::scene(6, "強い風が吹いた夜も、一隻も流されることなくしっかり浜に留まっていた。"),
                ],
                choices: None,
            },
            Story {
                title: "波に眠る舟歌",
                beats: &[
                    Beat::speech(3, "岬の老漁師", Villager, "昔みんなで歌いながら漕いでいた舟歌のサビだけが、どうしても思い出せないんだよ……。"),
                    Beat::scene(1, "波と櫂の音に耳を澄まして古い節を聴き取り、舟を直している若い住民たちへ歌い伝えた。"),
                    Beat::scene(0, "夕暮れの浜辺に懐かしい舟歌が戻り、老漁師も若い衆も声を合わせて歌った。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "燈魚の市",
        profile: "夜漁から戻る船の目印となる琥珀灯台の下に、魚競り場と航海具屋が軒を連ねる市場町。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "琥珀灯の魚競り堂", at: (156, 122), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "大きな琥珀色の灯籠が夜の市場と港を照らす競り場。" },
            Place { name: "東の航海具横丁", at: (224, 102), art: Some(Art::Stalls), ground: Landscape::Square, detail: "海図や羅針盤、集魚灯などを売る店の並び。" },
            Place { name: "西の静かな採取浜", at: (84, 128), art: None, ground: Landscape::Shore, detail: "光る燈魚が卵を産みに立ち寄る穏やかな浜辺。" },
            Place { name: "南の琥珀灯台", at: (196, 138), art: Some(Art::Lighthouse), ground: Landscape::Shore, detail: "欠けた琥珀の覆いを直しながら海を照らす灯台。" },
            Place { name: "海図作りの岬", at: (88, 76), art: Some(Art::Palisade), ground: Landscape::Heath, detail: "海図から抜けていた隠れ岩の位置を測れる岬。" },
            Place { name: "町の料理屋台", at: (126, 86), art: Some(Art::Hearth), ground: Landscape::Square, detail: "値がつかない小魚を美味しく料理してくれる屋台。" },
            Place { name: "拾い子の露店", at: (214, 128), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "浜辺で見つけた琥珀のかけらを並べる小さな店。" },
            Place { name: "漁区控えの掲示板", at: (156, 78), art: Some(Art::Inn), ground: Landscape::Square, detail: "夜明け前に水揚げされた漁区ごとの記録が貼られる。" },
        ],
        stories: [
            Story {
                title: "灯を沈める売り手",
                beats: &[
                    Beat::speech(1, "航海具商", Worker, "燈魚が不漁だからって、旧港から出た強い集魚灯を売ろうとしてるやつがいるんだ。魚を根こそぎ獲ったら来季がいなくなるよ！"),
                    Beat::scene(7, "競り前の漁区ごとの差を調べ、採取浜に魚が集まる時期を守ることが大切だと分かった。"),
                    Beat::speech(0, "市役の係", Guard, "集魚灯を禁止してしばらく休漁するか、採取浜から遠く離れた海域だけで短期の試験をするか、どう決めよう？"),
                ],
                choices: Some([
                    Ending { label: "灯を禁じて短期休漁する", text: "浜が静けさを取り戻したおかげで、数週間後には見事な燈魚の群れが戻った。" },
                    Ending { label: "離れた海域で試験する", text: "産卵場を守りながら少しずつ漁を続けられ、漁師も道具屋も納得した。" },
                ]),
            },
            Story {
                title: "競り値のない一籠",
                beats: &[
                    Beat::speech(0, "若い漁師", Worker, "大きさの揃わない小魚ばかりで競りに値がつかなかったよ。このまま捨てるしかないのかな……。"),
                    Beat::speech(5, "料理番", Worker, "もったいない！　塩をして干物に仕立てるか、市が終わったあとここで漁師さん自身が焼き魚の一皿として売ってみないかい？"),
                ],
                choices: Some([
                    Ending { label: "干物にして保存食にする", text: "船乗りのいいおかずとして人気が出て、次の市からは最初から注文が入るようになった。" },
                    Ending { label: "閉市後に焼き魚として売る", text: "焼きたての香ばしい匂いに人が集まり、夕方の新しい名物屋台になった。" },
                ]),
            },
            Story {
                title: "海図の空白を埋める",
                beats: &[
                    Beat::speech(1, "航海具屋", Worker, "うちの海図だけ、灯台から死角になる西の岩が抜けててさ。正確な場所が分からないんだ。"),
                    Beat::scene(4, "朝の岬と採取浜を歩いて角度を測り直し、干潮で歩ける浅瀬と危険な暗礁を別々の記号で海図に描き加えた。"),
                    Beat::scene(1, "完璧になった海図のおかげで、西の浜へ入る船も座礁しなくなった。"),
                ],
                choices: None,
            },
            Story {
                title: "灯台の琥珀のかけら",
                beats: &[
                    Beat::speech(6, "拾い子の子ども", Villager, "浜で拾ったきれいな琥珀のかけらを売ってたら、灯台守さんに怒られて追い出されちゃったんだ……。"),
                    Beat::scene(3, "灯台守に聞くと、昔壊れた灯台の覆いの破片が混じっていて、修理のために探していたものだった。"),
                    Beat::scene(6, "修理に必要な破片は灯台に返し、残りの石は子どもが綺麗な飾りとして堂々と売れるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "箱を開けた魚",
                beats: &[
                    Beat::speech(0, "競り場の係", Worker, "誰もいないはずの競り場の木箱が、中から青い光でぼうっと照ってるんだよ！"),
                    Beat::scene(2, "航海具店の遮光布を掛けて光を落ち着かせ、中に入っていた若い燈魚をそっと西の採取浜の影へ放してやった。"),
                    Beat::scene(2, "数日後、放した魚に導かれるように大きな燈魚の群れが浜へ戻ってきた。"),
                ],
                choices: None,
            },
        ],
    },
];

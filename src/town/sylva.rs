use super::{Art, Beat, Ending, Landscape, Place, Plan, Story};
use crate::dialogue::Speaker::*;

pub(super) static SYLVA: [Plan; 10] = [
    Plan {
        name: "梢の集落",
        profile: "巨木の枝に板橋を掛けて暮らす樹上集落。危険な区間が出たときは橋を守る職人が道を通行止めにする。",
        terrain: Landscape::Canopy,
        entry: (156, 174),
        places: [
            Place { name: "中央樹の橋番所", at: (156, 92), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "二本の巨木をつなぐ吊り橋のそばにある橋番所。" },
            Place { name: "南の吊り橋", at: (156, 134), art: Some(Art::Bridge), ground: Landscape::Dock, detail: "地上と樹上の家々を結ぶ古い吊り橋。" },
            Place { name: "北の荷揚げ台", at: (156, 50), art: Some(Art::Palisade), ground: Landscape::Dock, detail: "滑車で森の下から荷を引き上げる木の足場。" },
            Place { name: "西の根元見張り小屋", at: (84, 112), art: Some(Art::Cottage), ground: Landscape::Wood, detail: "地上の獣道を見張る森番の小さな小屋。" },
            Place { name: "梢の屋根裏", at: (188, 78), art: None, ground: Landscape::Dock, detail: "鳥たちが木の実を集めて隠している高い屋根。" },
            Place { name: "太枝の継ぎ目", at: (126, 108), art: None, ground: Landscape::Dock, detail: "木の成長に合わせて掛け替えられた橋脚の跡。" },
            Place { name: "東の薬師小屋", at: (224, 98), art: Some(Art::Inn), ground: Landscape::Dock, detail: "夜間の急患も診てくれる樹上の薬師の小屋。" },
            Place { name: "止まり木の梯子", at: (116, 68), art: Some(Art::Lighthouse), ground: Landscape::Wood, detail: "渡り鳥が羽を休めにやって来る高い梯子。" },
        ],
        stories: [
            Story {
                title: "裂けた枝の下",
                beats: &[
                    Beat::speech(0, "橋を守る職人", Worker, "南の太枝にひびが入ってね。安全のために吊り橋を外したいんだが、それだと荷を地上へ下ろせなくなるんだよ。"),
                    Beat::scene(3, "根元の獣道を調べると、朝の時間帯だけは魔獣の巣から離れて安全に荷を下ろせることが分かった。"),
                    Beat::speech(2, "荷揚げ台の係", Worker, "安全が一番だ。朝だけ地上の道を使って荷を下ろすか、北の荷揚げ台から小分けにして下ろすか、どっちがいいだろう？"),
                ],
                choices: Some([
                    Ending { label: "朝の地上道で荷を下ろす", text: "見張りを立てて朝の間に荷を運び、南の吊り橋は安全に外された。" },
                    Ending { label: "北の荷揚げ台から下ろす", text: "滑車を二重にして荷を小分けに下ろし、木への負担をすっかり減らせた。" },
                ]),
            },
            Story {
                title: "名残の橋名",
                beats: &[
                    Beat::speech(5, "集落の長老", Villager, "昔の橋の名前が二本の新しい道にそのまま残ってて、急ぎの使いがしょっちゅう道を間違えるんだよ。"),
                    Beat::scene(5, "幹の古い傷をたどって元の橋脚の場所を見つけ、古い橋の名前を片方の広場だけに引き継ぐことにした。"),
                    Beat::scene(0, "案内板が新しく書き直され、初めての旅人でも迷わず目的地に着けるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "どんぐりの合図",
                beats: &[
                    Beat::speech(4, "樹上の子ども", Villager, "地上にいる友だちに毎日どんぐりを落として合図してたのに、最近ぜんぜん返事が来ないんだ。"),
                    Beat::scene(4, "屋根の上を見ると、鳥たちが落ちたどんぐりをそっくり巣へ運んでいた。鳥用の木の実を別の枝に置き、合図は橋の上で手を振ることにした。"),
                    Beat::scene(1, "次の朝、吊り橋越しに二人の子どもが元気よく手を振り合っていた。"),
                ],
                choices: None,
            },
            Story {
                title: "夜に荷を上げる",
                beats: &[
                    Beat::speech(6, "樹上の薬師", Villager, "具合の悪い人を夜のうちに上の家へ運びたいんだけど、灯りをつけると地上に魔獣が集まっちまうんだ。"),
                    Beat::speech(2, "荷揚げ台の係", Worker, "灯りを消して合図の音だけで荷を上げるか、見張りが増える朝まで薬師の小屋で診てもらうか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "合図の音だけで引き上げる", text: "静かに滑車を引いて無事に運び上げ、家族はほっと胸をなで下ろした。" },
                    Ending { label: "朝まで薬師の小屋で休む", text: "薬師が朝まで付き添って手当てし、日が出てから安全に家へ移せた。" },
                ]),
            },
            Story {
                title: "止まり木の柱",
                beats: &[
                    Beat::speech(7, "鳥好きの住民", Villager, "いつも梯子の上に来てた珍しい渡り鳥が、このごろぱったり姿を見せなくなったんだ。"),
                    Beat::scene(7, "梯子の継ぎ目が風でぎしぎし鳴っていたため、鳥が驚いていたのだ。木のくさびを打ち直して音を止めた。"),
                    Beat::scene(7, "翌朝にはきれいな羽根の鳥が梯子に戻り、住民は嬉しそうに渡り鳥の季節の話を聞かせてくれた。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "葉影の港村",
        profile: "入り江に根を張る巨木の間に舟を留める森の港村。根を傷める杭打ちは精霊使いが止めることになっている。",
        terrain: Landscape::Coast,
        entry: (156, 48),
        places: [
            Place { name: "根付き桟橋の舟屋", at: (156, 126), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "巨木の根に抱かれるように建つ葉屋根の舟屋。" },
            Place { name: "東の舟寄合所", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "漁師や船番が集まって航路を相談する場所。" },
            Place { name: "西の根床と苗床", at: (86, 104), art: Some(Art::Stalls), ground: Landscape::Wood, detail: "入り江を守る若い木の苗を育てる静かな苗床。" },
            Place { name: "北の潮見台", at: (156, 72), art: Some(Art::Lighthouse), ground: Landscape::Wood, detail: "入り江の水深と潮の流れを見下ろせる木の見張り台。" },
            Place { name: "水際の古杭列", at: (192, 134), art: None, ground: Landscape::Shore, detail: "干潮のときだけ顔を出す昔の桟橋の杭。" },
            Place { name: "舟大工の仕事場", at: (116, 122), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "古木の木くずのいい匂いがする舟大工の小屋。" },
            Place { name: "入り江の浅瀬", at: (82, 136), art: None, ground: Landscape::Shore, detail: "潮に運ばれた種が流れ着く穏やかな浅瀬。" },
            Place { name: "共鳴の古杭", at: (224, 134), art: None, ground: Landscape::Dock, detail: "干潮になると不思議な水音が響く古い杭。" },
        ],
        stories: [
            Story {
                title: "杭を打たない桟橋",
                beats: &[
                    Beat::speech(1, "舟大工", Worker, "旅客船を増やすには深い場所へ桟橋を伸ばさなきゃならない。でもそれには若い木の根を切るしかないんだ……。"),
                    Beat::scene(3, "潮見台から入り江を見ると、干潮のときにも十分深い水路と、使われなくなった古い杭の列が見つかった。"),
                    Beat::speech(2, "苗床の世話役", Villager, "根を切らずに済むよ。古杭を使った浮き桟橋にするか、沖の大船から小舟でお客を運ぶか、どっちにしよう？"),
                ],
                choices: Some([
                    Ending { label: "古杭で浮き桟橋を作る", text: "波に揺れる丈夫な浮き桟橋ができ、木の根を一つも切らずに船を着けられた。" },
                    Ending { label: "小舟で客を送り迎える", text: "小舟の渡し便が名物になり、森の木の根もそのまま元気に育ち続けた。" },
                ]),
            },
            Story {
                title: "根を抱く舟",
                beats: &[
                    Beat::speech(5, "舟番", Worker, "嵐で沈んだ小舟が、入り江の木の根の間に挟まって動けなくなってるんだ。無理に引くと根を傷めそうでね。"),
                    Beat::scene(2, "苗床の裏から根の動きを確かめ、潮が満ちるのを待つか、痛んだ船板を外して少しずつ取り出すか相談した。"),
                    Beat::scene(5, "舟大工が板を外して丁寧に引き上げたおかげで、木の根は無傷のまま、良質な材木も再利用できた。"),
                ],
                choices: None,
            },
            Story {
                title: "水面の種運び",
                beats: &[
                    Beat::speech(2, "苗床の世話役", Villager, "潮に流れていった森の木の種が、浅瀬で小魚に食べられそうなんだ！　集めるのを手伝ってくれない？"),
                    Beat::scene(6, "舟で入り江を回り、ぷかぷか浮かぶ種を網ですくい上げた。岸辺に根付きそうな場所には少し残しておいた。"),
                    Beat::scene(2, "苗床の棚に新しい種が並び、来年の春にはたくさんの若木が芽吹くことになった。"),
                ],
                choices: None,
            },
            Story {
                title: "潮に覚えられた歌",
                beats: &[
                    Beat::speech(7, "老船番", Worker, "干潮になると、昔の仕事歌に応えるみたいな不思議な水音が聞こえるんだ。何の音なんだろうね。"),
                    Beat::scene(7, "古杭の前で耳を澄ますと、歌の節に合わせて水が抜ける音がし、安全な航路の目印になっていたことが分かった。"),
                    Beat::scene(1, "先代の船番が残した知恵として航路図に書き留められ、若い船乗りたちにも受け継がれた。"),
                ],
                choices: None,
            },
            Story {
                title: "船のない船大工",
                beats: &[
                    Beat::speech(5, "老舟大工", Worker, "手が震えるようになって、もう自分の工房を畳むことにしたよ。道具もしまってしまおう。"),
                    Beat::speech(1, "若い船番", Worker, "まだ辞めないでよ！　手が震えても使える櫂作りの道具を工夫するか、悪い箇所を見抜く検査役をお願いしたいんだ！"),
                ],
                choices: Some([
                    Ending { label: "櫂作りの道具を工夫する", text: "固定台を使った新しい道具で再び削り始め、若い衆に自慢の櫂を届けた。" },
                    Ending { label: "修理の検査役を任せる", text: "熟練の目で水漏れを見抜き、村じゅうの舟が前よりずっと長持ちするようになった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "根渡りの里",
        profile: "川を跨ぐ巨木の根と白い歩橋で結ばれた里。根の上の道は誰もが無料で通れるのが昔からの掟だ。",
        terrain: Landscape::Valley,
        entry: (156, 174),
        places: [
            Place { name: "根渡りの白い歩橋", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "巨木の太い根に寄り添うように架けられた白い歩橋。" },
            Place { name: "東岸の渡し守小屋", at: (218, 96), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "川の水位を見張り、橋の修繕を預かる小屋。" },
            Place { name: "西岸の板材置場", at: (92, 96), art: Some(Art::Stalls), ground: Landscape::Wood, detail: "橋の掛け替えに使う丈夫な板材が積まれている。" },
            Place { name: "上流の根の見通し台", at: (156, 52), art: Some(Art::Palisade), ground: Landscape::Wood, detail: "増水のときに沈む古い道を見渡せる高台。" },
            Place { name: "東の木取り場", at: (224, 134), art: Some(Art::Mill), ground: Landscape::Wood, detail: "橋板や木工品を切り出す職人の仕事場。" },
            Place { name: "岸伝いの古道", at: (106, 132), art: None, ground: Landscape::Shore, detail: "水が引いたときにだけ顔を出す古い踏み跡。" },
            Place { name: "川辺の広場", at: (190, 126), art: Some(Art::Hearth), ground: Landscape::Square, detail: "両岸の子どもたちが待ち合わせて遊ぶ広場。" },
            Place { name: "西岸の荷車道", at: (84, 62), art: None, ground: Landscape::Yard, detail: "薬草商の荷車が通る幅の広い道。" },
        ],
        stories: [
            Story {
                title: "根の上は誰のもの",
                beats: &[
                    Beat::speech(1, "渡し守", Worker, "白い歩橋を掛け替える費用を出してくれる代わりに、薬草商が通行料を取りたいと言ってるんだ。でも畑に通う家が困るんだよ。"),
                    Beat::scene(3, "上流の根を調べると、川幅の狭い場所に短い仮橋を架けられる場所が見つかった。"),
                    Beat::speech(7, "薬草商", Traveler, "通行料を取るのに反対があるなら考えよう。上流に短い橋を架けて住み分けるか、荷車だけから通行料を取る形にするか、どうだい？"),
                ],
                choices: Some([
                    Ending { label: "上流に短い橋を架ける", text: "里の人々はいつもの道を無料で渡れるようになり、暮らしが守られた。" },
                    Ending { label: "荷車だけ通行料を取る", text: "歩いて渡る住民は無料のまま、荷車の通行料で橋はいつも綺麗に保たれた。" },
                ]),
            },
            Story {
                title: "根の向こうのおつかい",
                beats: &[
                    Beat::speech(6, "東岸の子ども", Villager, "川が増水して、西岸にいる友だちに誕生日の贈り物を渡しに行けないんだ……。"),
                    Beat::scene(1, "渡し守に相談すると、上流の根の上を通れる安全な時間帯を教えてもらい、西岸の子も岸辺まで出てきてもらった。"),
                    Beat::scene(6, "二人は浅瀬の向こうで無事に出会い、嬉しそうに贈り物を手渡し合った。"),
                ],
                choices: None,
            },
            Story {
                title: "橋板の葉模様",
                beats: &[
                    Beat::speech(2, "橋守", Worker, "寄付された古い橋板に、見慣れない葉の印が彫ってあるんだ。どこの家のものか調べてくれないか。"),
                    Beat::scene(4, "西の木取り場までたどると、昔ここで働いていた職人の大切な道具箱の蓋だったと分かった。"),
                    Beat::speech(4, "職人の家族", Villager, "お父さんの形見の板だよ！　家に持ち帰って大事に飾るか、それとも名前を刻んで新しい橋に組み込んでもらおうか？"),
                ],
                choices: Some([
                    Ending { label: "家族のもとへ返す", text: "板は職人の家に飾られ、代わりに新しい丈夫な檜の板が寄付された。" },
                    Ending { label: "橋の一部として組み込む", text: "葉の印が橋の中央に据えられ、渡るたびに職人の仕事が思い出されるようになった。" },
                ]),
            },
            Story {
                title: "古道の足跡",
                beats: &[
                    Beat::speech(5, "岸辺の老人", Villager, "水が引くたびに、沈んだ古道に同じ足跡がすっと現れて消えるんだよ。まるで誰かが向こう岸へ渡りたがってるみたいにね。"),
                    Beat::scene(5, "根から岸辺まで丈夫な縄を渡し、足跡が迷わず渡れるよう目印を結んだ。不思議と足跡は対岸まで続いて消えていった。"),
                    Beat::scene(1, "老人たちは穏やかな気持ちで川面を見つめ、昔から残る根の道を改めて大切にするようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "跳ね橋の試歩",
                beats: &[
                    Beat::speech(7, "荷獣の使い手", Traveler, "木の根の上を歩けない荷獣のために、折り畳み式の橋を作ってもらったんだ。試しに渡らせてみてくれる？"),
                    Beat::scene(2, "歩く速さや足元の板を確かめ、安全に渡れる荷の重さと水位の目安を札に書いて橋の脇に掛けた。"),
                    Beat::scene(7, "荷獣も安心して川を渡れるようになり、里へ届く品物の種類がぐっと増えた。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "月見の里",
        profile: "夜に光る水草を観測するため月映えの池のほとりに築かれた里。祭りの夜は里全体で灯りを分かち合う。",
        terrain: Landscape::Meadow,
        entry: (156, 174),
        places: [
            Place { name: "三日月の展望壇", at: (156, 78), art: Some(Art::Landmark), ground: Landscape::Square, detail: "池の面と夜空を望む三日月形の木造り展望壇。" },
            Place { name: "中央の観測役宅", at: (116, 104), art: Some(Art::Inn), ground: Landscape::Square, detail: "代々の観測記録と古い星図が並んだ木造の役宅。" },
            Place { name: "東の祭り広場", at: (218, 108), art: Some(Art::Stalls), ground: Landscape::Square, detail: "露店が並び、月見の夜に多くの旅人が集まる広場。" },
            Place { name: "南の池岸と取水樋", at: (156, 138), art: Some(Art::Sluice), ground: Landscape::Shore, detail: "夜に淡く光る水草が揺れる静かな池のほとり。" },
            Place { name: "池縁の小道", at: (196, 136), art: None, ground: Landscape::Shore, detail: "街灯の届かない静かな水辺の小道。" },
            Place { name: "裏丘の星見台", at: (92, 64), art: Some(Art::Clock), ground: Landscape::Heath, detail: "古い星図に合わせて星と月を見上げる石の台座。" },
            Place { name: "露店の料理台", at: (232, 128), art: Some(Art::Hearth), ground: Landscape::Yard, detail: "毎年訪れる客の味を覚えている名物料理の屋台。" },
            Place { name: "西の案内小屋", at: (84, 118), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "足元灯を貸し出す親切な里の案内小屋。" },
        ],
        stories: [
            Story {
                title: "池を暗くする夜",
                beats: &[
                    Beat::speech(1, "観測役", Villager, "最近、水草が光らなくなっちまってね。原因を確かめるには夜の灯りを全部消したいんだが、露店が反対しててさ。"),
                    Beat::scene(0, "展望壇から明かりを遮って池を見ると、広場の光が水に反射して水草の淡い光をかき消していた。"),
                    Beat::speech(2, "露店主", Worker, "真っ暗じゃお客が池に落ちちまうよ。灯りを低い位置に移して覆いを付けるか、案内人の手灯だけで池辺を歩いてもらうか、どっちがいい？"),
                ],
                choices: Some([
                    Ending { label: "灯りを低い位置へ移す", text: "足元だけが優しく照らされ、水面には水草の青い光がくっきり戻った。" },
                    Ending { label: "案内人の手灯で開く", text: "里人が手灯を持って案内する静かな祭りとなり、旅人たちは星と水草に見入った。" },
                ]),
            },
            Story {
                title: "月石の欠けた目盛り",
                beats: &[
                    Beat::speech(5, "若い観測役", Villager, "星見台の目盛りが、なぜか今夜の月の位置と少しずつずれてるんだよ……。"),
                    Beat::scene(0, "三日月壇の影と照らし合わせると、長い歳月で台座の石が少し傾いていたことが分かった。"),
                    Beat::scene(5, "くさびを挟んで水平を戻すと、目盛りは再びぴったり月を指し、次の満月の予報も的中した。"),
                ],
                choices: None,
            },
            Story {
                title: "毎年違う一皿",
                beats: &[
                    Beat::speech(6, "露店主", Worker, "記憶を失って去年までを覚えていないお客さんに、毎年同じ料理を出してしまっていいのかなって悩んでるんだ。"),
                    Beat::scene(1, "観測役の記録の余白から、その客が昔好んで頼んでいた別の味のメモが見つかった。"),
                    Beat::scene(6, "今年は二種類の小皿を並べて、本人に好きな味を選んでもらった。客は一口食べて懐かしい顔をした。"),
                ],
                choices: None,
            },
            Story {
                title: "池面を渡る火",
                beats: &[
                    Beat::speech(3, "水番", Worker, "夜更けに、誰も持っていないはずの明かりが池の上をすーっと動いていくんだ！"),
                    Beat::scene(4, "岸から追うと、水に落ちた祭り灯が、光に集まる精霊魚を浅瀬へ誘っていた。"),
                    Beat::speech(3, "水番", Worker, "このまま入り江の奥まで誘導して魚を休ませるか、池が汚れないうちに明かりを回収するか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "明かりを回収する", text: "水草の根に引っかかる前に引き上げ、祭りの灯籠は岸へ戻された。" },
                    Ending { label: "入り江の魚を誘導する", text: "魚たちは光について穏やかな浅瀬へ移り、里の新しい名物になった。" },
                ]),
            },
            Story {
                title: "欠席の星図",
                beats: &[
                    Beat::speech(5, "老観測役", Villager, "足を痛めて、もう丘の上の壇まで上れなくなっちまった。今年の星図は描けそうにないな……。"),
                    Beat::scene(3, "池の水面に映る星を利用すれば、低い池岸に座ったままでも正確に星の位置を測れることが分かった。"),
                    Beat::scene(1, "老観測役と若い観測役が池岸に並んで座り、二人で新しい美しい星図を描き上げた。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "雫葉の集落",
        profile: "湿地の上に高床の家と梯子を建て、葉から落ちる雨水を分け合って暮らす集落。",
        terrain: Landscape::Marsh,
        entry: (156, 174),
        places: [
            Place { name: "共用梯子塔と雨桶", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "高い柱に雨水を集める桶と梯子が組まれた塔。" },
            Place { name: "北の上段家並み", at: (156, 48), art: Some(Art::Inn), ground: Landscape::Dock, detail: "高い枝や柱の上に建てられた上段の住まい。" },
            Place { name: "南の下段菜園", at: (156, 142), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "屋根から引いた水で野菜を育てる下段の畑。" },
            Place { name: "西の屋根樋足場", at: (86, 86), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "上段から下段へ水を導く竹の樋と修理足場。" },
            Place { name: "舟着きの玄関", at: (196, 134), art: Some(Art::Bridge), ground: Landscape::Dock, detail: "増水の季節に小舟が直接着けられる板の玄関。" },
            Place { name: "染め手の水場", at: (218, 86), art: Some(Art::Well), ground: Landscape::Dock, detail: "布を染める綺麗な水を使う作業場。" },
            Place { name: "水路の曲がり角", at: (110, 128), art: None, ground: Landscape::Dock, detail: "小さな木彫りの鳥が毎朝流れ着く水路の隅。" },
            Place { name: "共用桶の置場", at: (126, 96), art: None, ground: Landscape::Dock, detail: "乾期に備えて水桶を並べておく高床の棚。" },
        ],
        stories: [
            Story {
                title: "屋根を流れる水",
                beats: &[
                    Beat::speech(2, "下段の住民", Villager, "上段の連中が新しい水籠を増やしてから、こっちの菜園に水が来なくなったんだ！　勝手にせき止めてるに違いないよ。"),
                    Beat::scene(3, "梯子を上って屋根樋をたどると、風で落ちた枯れ枝が樋の途中に詰まって水を外へそらしていた。"),
                    Beat::speech(1, "上段の班長", Villager, "誤解が解けてよかった。詰まった枯れ枝を取り除いたうえで、新しい籠にも余り水が回る分岐を作るか、まずは元どおり一本の樋に戻そうか？"),
                ],
                choices: Some([
                    Ending { label: "枯れ枝を取り除いて一本に戻す", text: "勢いよく水が下段の菜園へ流れ込み、野菜の葉が生き生きと蘇った。" },
                    Ending { label: "余り水の分岐を修理する", text: "上段にも下段にも公平に水が届くようになり、両班の仲も深まった。" },
                ]),
            },
            Story {
                title: "梯子下の忘れ物",
                beats: &[
                    Beat::speech(6, "下段の子ども", Villager, "毎朝、舟が着くところに誰のか分からない木彫りの鳥が流れ着くんだよ。不思議だね。"),
                    Beat::scene(6, "樋をつたって水を運ぶ小さな精霊が、屋根から落とした玩具を毎回ここまで運んでいたと分かった。"),
                    Beat::scene(0, "持ち主に返したあと、精霊が寂しがらないよう小さな木の実の飾りを屋根の端に置いておいた。"),
                ],
                choices: None,
            },
            Story {
                title: "水の色を染める",
                beats: &[
                    Beat::speech(5, "染め手", Worker, "染めた布の洗い水が飲み水の桶に混じってるんじゃないかって、下段の人たちに疑われてるんだ……。"),
                    Beat::scene(3, "晴れの日と雨の日の両方で樋の流れを確かめ、飲み水へつながる竹樋だけ完全に別のルートへ切り離した。"),
                    Beat::scene(5, "透き通った水が桶に溜まるのを見て、下段の住民たちも安心して染め手の仕事を応援するようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "空の雨桶",
                beats: &[
                    Beat::speech(7, "水当番", Villager, "下段の雨桶に穴が開いちまって、共同の修理順番が回ってくるまで水が溜められないんだよ。"),
                    Beat::speech(1, "上段の班長", Villager, "修理が終わるまで上段から桶を一つ貸してあげようか？　それとも丈夫な葉と樹脂で今すぐ応急修理してみようか？"),
                ],
                choices: Some([
                    Ending { label: "上段から桶を借りる", text: "乾ききる前に水が確保でき、あとから余った水をお礼に分けて返した。" },
                    Ending { label: "丈夫な葉で穴を塞ぐ", text: "驚くほどしっかり水が止まり、ほかの古い桶の補修にもその知恵が使われた。" },
                ]),
            },
            Story {
                title: "舟の玄関",
                beats: &[
                    Beat::speech(4, "高床の老人", Villager, "水かさが増えた季節は、舟から家の床までの段差が大きくて乗り降りが危ないんだよ。"),
                    Beat::scene(4, "水位に合わせて上下する浮き板の足場を試作し、手すり付きの丈夫な梯子を玄関に添えた。"),
                    Beat::scene(4, "老人は一人でも安全に舟へ乗り降りできるようになり、下の市場へ買い物に出かけられるようになった。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "精霊樹の門前村",
        profile: "精霊樹へ向かう巡礼者を案内する門前の村。根を傷めない道づくりと夜の通行を村と門守で守っている。",
        terrain: Landscape::Canopy,
        entry: (156, 174),
        places: [
            Place { name: "精霊樹の門前鳥居", at: (156, 86), art: Some(Art::Landmark), ground: Landscape::Square, detail: "古木の前へ続く灯籠道の入口に立つ立派な木門。" },
            Place { name: "北の古木と灯籠道", at: (156, 46), art: Some(Art::Palisade), ground: Landscape::Wood, detail: "精霊の灯りが揺れる静かな並木道。" },
            Place { name: "南の巡礼宿", at: (156, 138), art: Some(Art::Inn), ground: Landscape::Square, detail: "記憶や声を求めて訪れる巡礼者が泊まる宿。" },
            Place { name: "西の施療小屋", at: (84, 102), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "疲れた旅人や心の傷を癒す静かな小屋。" },
            Place { name: "根の見回り道", at: (196, 68), art: None, ground: Landscape::Wood, detail: "古木の根を踏まないよう少し離れた監視の小道。" },
            Place { name: "夜門の詰め所", at: (126, 98), art: Some(Art::Cottage), ground: Landscape::Square, detail: "夜間の通行札を貸し出す門守の詰め所。" },
            Place { name: "見回り道の曲がり", at: (224, 112), art: None, ground: Landscape::Wood, detail: "角の透けた子鹿がよく休んでいる日だまり。" },
            Place { name: "香草の道標", at: (96, 134), art: None, ground: Landscape::Yard, detail: "夜でも匂いで帰り道が分かるように植えた香草。" },
        ],
        stories: [
            Story {
                title: "封門から聞こえる声",
                beats: &[
                    Beat::speech(3, "施療小屋の世話役", Villager, "最近、古根の封門の方から声が聞こえたっていう巡礼者が増えてね。でも日によって聞こえた話が違うんだよ。"),
                    Beat::scene(4, "見回り道で音を確かめると、封印そのものではなく、聞き手の記憶が根に反響して聞こえていた。"),
                    Beat::speech(0, "門守", Guard, "むやみに騒ぎを広げないほうがいい。門守と静かに聞き取る場を作るか、時間を限って巡礼者に開くか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "門守と静かな聞き取り場を作る", text: "一人ずつ落ち着いて記憶と向き合えるようになり、無用な混乱が収まった。" },
                    Ending { label: "時間を限って巡礼者に開く", text: "静かな朝の時間だけ参拝が許され、巡礼者たちは節度を守って訪れた。" },
                ]),
            },
            Story {
                title: "灯籠に残す道",
                beats: &[
                    Beat::speech(1, "巡礼者", Traveler, "日暮れ前に灯籠の一つが消えてて、暗がりでうっかり木の根を踏みそうになっちまったよ。"),
                    Beat::scene(1, "木の幹に直接光を当てないよう、足元だけを柔らかく照らす木の覆いを灯籠に取り付けた。"),
                    Beat::scene(0, "木の精霊も落ち着きを取り戻し、夜の灯籠道が安全に歩けるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "口にしない名前",
                beats: &[
                    Beat::speech(3, "ミナの知り合い", Villager, "施療を手伝ってるミナさんから、『忘れた名前を無理に思い出させないでほしい』って頼まれたんだ。"),
                    Beat::scene(2, "巡礼宿で本人の持ち物らしい手がかりを見つけたが、無理に渡さず、本人が知りたいと望んだときに渡せるよう預かっておくことにした。"),
                    Beat::scene(3, "本人は自分の歩幅でゆっくり元気を取り戻し、いつか準備ができたときに開く日まで手元に残された。"),
                ],
                choices: None,
            },
            Story {
                title: "道を塞ぐ子鹿",
                beats: &[
                    Beat::speech(6, "見回り番", Villager, "見回り道の真ん中に、角の透けた珍しい子鹿が横になって動かないんだ。無理に追い払うわけにもいかないし……。"),
                    Beat::speech(0, "精霊使い", Villager, "休んでいる根をそっとしておくために道を少し迂回させるか、夜になってから親鹿の通り道へそっと案内しようか？"),
                ],
                choices: Some([
                    Ending { label: "道を少し迂回させる", text: "子鹿はゆっくり休んで元気を取り戻し、翌朝には森の奥へ駆けていった。" },
                    Ending { label: "夜に親の通り道へ案内する", text: "小さな灯りで誘導すると、間もなく親鹿が迎えに来て寄り添って帰っていった。" },
                ]),
            },
            Story {
                title: "夜の門の借り手",
                beats: &[
                    Beat::speech(5, "門守", Guard, "夜間の通行札が一枚まだ戻ってこないんだ。施療小屋から帰る誰かが道に迷ったのかもしれない。"),
                    Beat::scene(7, "札に結びつけた香草の匂いをたどると、帰り道が分からなくなった旅人を見つけた。足元灯を手渡し、香草の道標を整えて無事宿へ送り届けた。"),
                    Beat::scene(5, "札は無事に戻り、夜の帰り道も迷う人がいなくなった。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "苔舟の町",
        profile: "苔むした舟庫で古い丸木舟を直し続けてきた川港。水難時は舟の私用を止めて救助を優先する。",
        terrain: Landscape::River,
        entry: (156, 46),
        places: [
            Place { name: "苔舟庫と渡し札場", at: (156, 108), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "苔むした屋根の下に丸木舟が並ぶ川港の中心。" },
            Place { name: "北の泥量観測杭", at: (156, 56), art: Some(Art::Sluice), ground: Landscape::Yard, detail: "上流から流れてくる泥の量を測る古い杭。" },
            Place { name: "東の修理船台", at: (224, 108), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "傷んだ舟を引き上げて板を張り替える作業場。" },
            Place { name: "西の堤と養生畑", at: (84, 108), art: Some(Art::Stalls), ground: Landscape::Farm, detail: "苔が土をしっかり抑えている大切な川岸の堤。" },
            Place { name: "引き舟道", at: (116, 138), art: None, ground: Landscape::Shore, detail: "川底の浅瀬全体を見渡せる岸辺の作業道。" },
            Place { name: "舟子の休み場", at: (196, 134), art: Some(Art::Hearth), ground: Landscape::Yard, detail: "日除けの下で舟子たちが昼休みをとる広場。" },
            Place { name: "石堤の古舟場", at: (94, 136), art: None, ground: Landscape::Dock, detail: "引退した古い丸木舟を静かに休ませる場所。" },
            Place { name: "川岸の荷改め台", at: (218, 74), art: Some(Art::Inn), ground: Landscape::Square, detail: "荷札の取れた荷物の送り主を確かめる台。" },
        ],
        stories: [
            Story {
                title: "苔を剥がす前に",
                beats: &[
                    Beat::speech(0, "舟組の代表", Worker, "川底が浅くなって舟が通りづらいんだ。岸の苔を剥がして流れを速めたいんだが、住民は堤が崩れるって反対しててね。"),
                    Beat::scene(1, "観測杭と荷流し札を比べると、泥が溜まったのは一か所の狭い筋だけだった。"),
                    Beat::speech(3, "堤の世話役", Villager, "苔は堤を守ってるんだ。狭い流路の泥だけを掘るか、冬前だけは底の浅い小舟に荷を載せ替えるか、どっちがいい？"),
                ],
                choices: Some([
                    Ending { label: "狭い流路だけ泥を掘る", text: "堤の苔を残したまま舟の通り道が確保され、冬前の荷運びも間に合った。" },
                    Ending { label: "浅い小舟に荷を載せ替える", text: "堤を少しも傷つけずに済み、小舟の番人たちにも仕事が増えた。" },
                ]),
            },
            Story {
                title: "まっすぐ漕げない舟",
                beats: &[
                    Beat::speech(2, "修理工", Worker, "せっかく直した舟なのに、川に出すとどうしても右に曲がっちまうんだ。原因が分からなくてね。"),
                    Beat::scene(4, "岸と流れの両方で試運転すると、左右の櫂の重さがわずかに違っていた。櫂を削り直して揃えると、舟はまっすぐ進むようになった。"),
                    Beat::scene(2, "修理工はすっきりした顔で礼を言い、次の注文にも取りかかった。"),
                ],
                choices: None,
            },
            Story {
                title: "舟庫で待つ精霊",
                beats: &[
                    Beat::speech(0, "舟番", Worker, "もう使わない古い丸木舟を解体しようと決めた途端、舟庫の苔が青く光りだしたんだ！"),
                    Beat::scene(6, "耳を澄ますと、かつてこの舟を守っていた渡し守の記憶が聞こえてきた。"),
                    Beat::speech(2, "舟大工", Worker, "壊すのは忍びないな。石堤の休み場に飾って残すか、丈夫な木片だけ新しい舟に継ぎ直して海へ出してあげようか？"),
                ],
                choices: Some([
                    Ending { label: "石堤の休み場に置いて残す", text: "古い舟は花に囲まれたベンチとして生まれ変わり、青い苔も静かに落ち着いた。" },
                    Ending { label: "木片を新しい舟に継ぐ", text: "先代の舟の魂を受け継いだ新しい丸木舟が、元気よく川へ漕ぎ出していった。" },
                ]),
            },
            Story {
                title: "底に沈んだ荷札",
                beats: &[
                    Beat::speech(7, "配達人", Worker, "川の水がかかって、荷物の宛名札がみんな剥がれて流れちまったんだ！　どこへ届ければいいか分からないよ。"),
                    Beat::scene(4, "引き舟道をたどって濡れた木箱の跡を調べ、箱の焼印と重さから送り主を突き止めた。"),
                    Beat::scene(7, "すべての荷物が無事に受取人のもとへ届き、配達人は胸をなで下ろした。"),
                ],
                choices: None,
            },
            Story {
                title: "堤の上の昼休み",
                beats: &[
                    Beat::speech(5, "舟子", Worker, "堤の工事中で、いつもの休憩所が使えなくて昼飯を食べる場所がないんだよ。"),
                    Beat::scene(3, "養生中の畑を荒らさない平らな芝生を見つけて日除けを張り、代わりに夕方の見回りを手伝ってもらう約束を結んだ。"),
                    Beat::scene(5, "舟子たちは心地よい川風の中で昼休みを過ごし、堤の見回りもいっそう丁寧になった。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "山桑の里",
        profile: "美しい染め布を求めて隊商が訪れる山桑と糸場の里。桑畑組と糸場組の話し合いで薬や買い付けを決めている。",
        terrain: Landscape::Meadow,
        entry: (156, 174),
        places: [
            Place { name: "橙屋根の糸場と干し棚", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "雨雲の下でも目立つ橙色の屋根を持つ染めと糸の工房。" },
            Place { name: "北斜面の桑畑", at: (156, 48), art: None, ground: Landscape::Farm, detail: "瑞々しい桑の葉を育てる日当たりのよい畑。" },
            Place { name: "西の薬煎じ小屋", at: (84, 98), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "虫よけの薬や染料を煎じる小さな小屋。" },
            Place { name: "東の布集荷場", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "染め上がった布を隊商に引き渡す集荷場。" },
            Place { name: "風よけ林の裏道", at: (196, 62), art: None, ground: Landscape::Wood, detail: "虫の被害を逃れた桑の列が並ぶ風の通り道。" },
            Place { name: "染め液の試し場", at: (126, 122), art: Some(Art::Well), ground: Landscape::Square, detail: "季節ごとに変わる花や葉で染め色を確かめる場所。" },
            Place { name: "屋根裏の蚕棚", at: (116, 74), art: None, ground: Landscape::Yard, detail: "大切な蚕を静かに育てている暖かい棚。" },
            Place { name: "糸場の休憩台", at: (186, 128), art: Some(Art::Stalls), ground: Landscape::Square, detail: "見習いたちが糸の結び方を練習する広場。" },
        ],
        stories: [
            Story {
                title: "桑に吹く風",
                beats: &[
                    Beat::speech(1, "桑畑の世話役", Villager, "桑に虫がついて困ってるんだ。強い薬を撒きたいんだけど、蚕が弱るって糸場組が反対しててね。"),
                    Beat::scene(4, "被害のない桑の列を調べると、風通しがよくて虫が寄りつかない場所だった。"),
                    Beat::speech(0, "糸場の親方", Worker, "薬を使わずに済むならありがたい。風を通すために枝を間引くか、被害の多い畝だけみんなで手摘みするか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "風を通すよう枝を間引く", text: "心地よい山風が畑全体を抜けるようになり、虫が自然と減って蚕も元気になった。" },
                    Ending { label: "被害の畝を手摘みで守る", text: "糸場組も総出で手伝い、薬に頼らない上質な桑の葉を無事に収穫できた。" },
                ]),
            },
            Story {
                title: "同じ橙が出ない",
                beats: &[
                    Beat::speech(5, "染め手", Worker, "今年の祭りに使う布が、どうしても去年と同じ橙色に染まらないんだ。どうしよう……。"),
                    Beat::scene(5, "桑の実と乾燥棚の条件を比べ、去年と同じ鮮やかな色に戻る配合も見つけた。"),
                    Beat::speech(0, "糸場の親方", Worker, "お見事だ！　伝統の去年の色を再現して出すか、今年ならではの柔らかな新色としてお返ししようか？"),
                ],
                choices: Some([
                    Ending { label: "去年の色を再現する", text: "変わらぬ鮮やかな橙が祭りの広場を彩り、隊商たちからも高い評価を得た。" },
                    Ending { label: "今年の色として出す", text: "今年限りの柔らかな橙色が『山桑の秋色』と評判になり、新しい注文が届いた。" },
                ]),
            },
            Story {
                title: "蚕の引っ越し",
                beats: &[
                    Beat::speech(6, "蚕の番人", Villager, "屋根裏が暑くなりすぎて、このままじゃ大切な蚕たちが弱っちまうよ！"),
                    Beat::speech(1, "桑畑の世話役", Villager, "涼しい畑の近くへ蚕の箱を移すか、工房に日除けと風通し窓を付けて温度を下げるか、どっちが早いかな？"),
                ],
                choices: Some([
                    Ending { label: "涼しい畑の近くへ移す", text: "新鮮な葉をすぐ食べられる環境で、蚕たちは元気に美しい糸を吐き始めた。" },
                    Ending { label: "日除けと風窓を付ける", text: "工房の中がぐっと涼しくなり、職人も蚕も快適に夏を乗り越えられた。" },
                ]),
            },
            Story {
                title: "干し棚の手紙",
                beats: &[
                    Beat::speech(7, "見習い", Villager, "干し布の隅に、誰かが小さな糸で不思議な模様を縫い込んであるのを見つけたんだ。"),
                    Beat::scene(0, "作業場に昔から伝わる糸文字の符号だと分かり、家族へ帰郷を知らせたい見習いの手紙だった。"),
                    Beat::scene(3, "隊商にその布を託し、遠く離れた家族のもとへ無事に思いが届いた。"),
                ],
                choices: None,
            },
            Story {
                title: "帯を半分ずつ",
                beats: &[
                    Beat::speech(3, "注文主", Traveler, "仲違いしてる姉妹に同じ帯を頼んだんだけど、お互い譲らなくて仕上がりがバラバラなんだよ。"),
                    Beat::speech(0, "糸場の親方", Worker, "両方のいいところを合わせて一本に仕立て直すか、色違いの二本として両方引き取ってもらうか、話をつけてみようか？"),
                ],
                choices: Some([
                    Ending { label: "合わせて一本の帯にする", text: "二人の技が重なり合った見事な帯が完成し、姉妹も顔を合わせて微笑んだ。" },
                    Ending { label: "色違いの二本として納める", text: "それぞれの個性が活きた二本の帯が喜ばれ、次の仕事は相談しながら進めるようになった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "古枝の工房村",
        profile: "嵐で折れた古木を傷なく挽き出す木工の村。生木の伐採には森番と工房組合双方の許しが要る。",
        terrain: Landscape::Canopy,
        entry: (156, 174),
        places: [
            Place { name: "森の製材工房", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "太い丸太を美しい板材に挽く開放的な製材工房。" },
            Place { name: "西の乾燥棚", at: (86, 102), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "挽いた板材をゆっくり乾かしている広い棚。" },
            Place { name: "東の出荷道と検査所", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "出荷する材の質と印を確かめる検査所。" },
            Place { name: "北の防風林と森番小屋", at: (156, 48), art: Some(Art::Cottage), ground: Landscape::Wood, detail: "村を嵐から守る大切な防風林と森番の小屋。" },
            Place { name: "丸太引き道", at: (116, 68), art: None, ground: Landscape::Wood, detail: "森から切り株や倒木を運んでくる広い道。" },
            Place { name: "細工道具の作業台", at: (186, 122), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "徒弟たちが自分の手に合う鉋を整える作業台。" },
            Place { name: "出荷道の細橋", at: (218, 134), art: Some(Art::Bridge), ground: Landscape::Dock, detail: "大きな荷車が曲がりきれずつかえやすい橋。" },
            Place { name: "溝沿いの木屑場", at: (118, 136), art: None, ground: Landscape::Yard, detail: "製材で出た木屑が溜まりやすい水路脇。" },
        ],
        stories: [
            Story {
                title: "生木の木口",
                beats: &[
                    Beat::speech(1, "乾燥棚の係", Worker, "鉄道向けの急ぎの注文に、まだ乾ききっていない生木の板が混じってるんだよ。"),
                    Beat::scene(3, "防風林の切り株と出荷印を照らし合わせ、納期に追われた職人が切ったものだと分かった。"),
                    Beat::speech(0, "工房の親方", Worker, "すまない、徒弟たちの仕事を守りたかったんだ……。生木を抜いて分納にするか、古い古材を補って納期を守るか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "生木を抜いて分納にする", text: "誠実な説明に買い手も納得し、良質な乾いた板だけを順次納められるようになった。" },
                    Ending { label: "古材を補って納期を守る", text: "村じゅうから丈夫な古材が集まり、防風林をこれ以上傷めずに納期へ間に合わせた。" },
                ]),
            },
            Story {
                title: "嵐が置いた材料",
                beats: &[
                    Beat::speech(0, "木工職人", Worker, "雷で落ちた古枝を挽こうとしたら、刃を近づけるたびに不思議な音が響くんだ。精霊が宿ってるのかな。"),
                    Beat::speech(3, "森番", Villager, "精霊が宿った節の周りを避けて板に挽くか、そのまま森へ戻して音の出る場所をみんなで見守ろうか？"),
                ],
                choices: Some([
                    Ending { label: "節を避けて板を挽く", text: "美しい木目の楽器の板が生まれ、弾くたびに森の風が聞こえるような音色になった。" },
                    Ending { label: "森へ戻して見守る", text: "枝は古木の根元に返され、森を歩く人々に優しい音色を届ける名所になった。" },
                ]),
            },
            Story {
                title: "手に合う鉋",
                beats: &[
                    Beat::speech(5, "若い徒弟", Villager, "工房の鉋がどれも大きすぎて、僕の手には合わなくて作業が遅れちゃうんです……。"),
                    Beat::scene(5, "ちょうどいい大きさの端材を探し出し、親方と一緒に徒弟の手のひらにぴったり合う小さな鉋を削り出した。"),
                    Beat::scene(0, "徒弟は見違えるような速さで綺麗な板を削れるようになり、笑顔が戻った。"),
                ],
                choices: None,
            },
            Story {
                title: "大きすぎる荷車",
                beats: &[
                    Beat::speech(6, "荷運び人", Traveler, "大きな板を積んだ荷車が、出荷道の橋の角に引っかかって通れないんだ！"),
                    Beat::speech(2, "検査所の係", Worker, "丸太引き道に合わせて荷車を細く組み替えるか、現地で組み立てられるように工房で分けて加工しようか？"),
                ],
                choices: Some([
                    Ending { label: "荷車を細く組み替える", text: "小回りの利く荷車のおかげで、狭い橋もすいすい渡れるようになった。" },
                    Ending { label: "現地組立用に分けて加工する", text: "運びやすくなったうえ現場での組み立ても楽になり、買い手からも喜ばれた。" },
                ]),
            },
            Story {
                title: "木屑の街道",
                beats: &[
                    Beat::speech(7, "道掃除の住民", Villager, "雨が降ると製材の木屑が街道に流れ出して、足元が滑って危ないんだよ。"),
                    Beat::speech(0, "工房の親方", Worker, "溝の木屑を集めて、隣の里の苗床の保温材に届けるか、固めて冬の焚き付けにしようか？"),
                ],
                choices: Some([
                    Ending { label: "苗床の保温材に回す", text: "木屑が若芽を寒さから守り、春にはたくさんの元気な苗が育った。" },
                    Ending { label: "固めて冬の焚き付けにする", text: "よく燃える焚き付けが炉の助けになり、村の冬支度がぐっと楽になった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "若芽の里",
        profile: "廃村を離れた人々が葉屋根の家を建てて開いた里。育てた若木を他の里へ分け合うことを誇りにしている。",
        terrain: Landscape::Meadow,
        entry: (156, 174),
        places: [
            Place { name: "葉屋根の苗床堂", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "緑の灯がともる丸い葉屋根の苗床と寄合の家。" },
            Place { name: "北の共同苗床", at: (156, 48), art: Some(Art::Stalls), ground: Landscape::Farm, detail: "芽吹いたばかりの若木を大切に育てる苗床。" },
            Place { name: "東の若木畑", at: (224, 102), art: None, ground: Landscape::Farm, detail: "少し育った果樹や木々を植え替える広い畑。" },
            Place { name: "南の旧村門と接ぎ木小屋", at: (156, 142), art: Some(Art::Palisade), ground: Landscape::Wood, detail: "眠り枝の廃村へ向かう道と接ぎ木を行う小屋。" },
            Place { name: "門前の古い畦", at: (192, 134), art: None, ground: Landscape::Wood, detail: "昔は井戸があった場所へ続く静かな畦道。" },
            Place { name: "洗濯と雨見の軒先", at: (92, 98), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "屋根の葉の向きを見て天気を読み取る軒先。" },
            Place { name: "記念木の広場", at: (116, 126), art: Some(Art::Hearth), ground: Landscape::Square, detail: "移住を祝って贈られた木が植えられている広場。" },
            Place { name: "新住民の畝", at: (206, 72), art: None, ground: Landscape::Farm, detail: "新しく里に来た家族が耕している畑の区画。" },
        ],
        stories: [
            Story {
                title: "古い木の新しい枝",
                beats: &[
                    Beat::speech(3, "旧村から来た住民", Villager, "廃村に残した果樹を手入れしに戻りたい家と、森へ返したい家で意見が割れてるんだ。"),
                    Beat::scene(4, "古い果樹の接ぎ目を調べると、若い枝を分けて里の苗床へ移せば、元の木は森に返しても収穫を続けられると分かった。"),
                    Beat::speech(1, "苗床の世話役", Villager, "いい方法だ。枝を苗床へ分けて育てるか、森番と日を決めて数本だけ現地で世話するか、どっちにする？"),
                ],
                choices: Some([
                    Ending { label: "枝を苗床へ分けて育てる", text: "新しい里に懐かしの実がなり、古い村は静かな森へと還っていった。" },
                    Ending { label: "日を決めて現地で数本管理する", text: "月に一度みんなで旧村を訪ねるのが楽しみになり、思い出も受け継がれた。" },
                ]),
            },
            Story {
                title: "葉屋根の前ぶれ",
                beats: &[
                    Beat::speech(5, "洗濯当番", Villager, "風が吹く直前に、なぜか屋根の葉が裏返るんだよ。雨が来る合図なのかな。"),
                    Beat::scene(2, "若木畑を調べると、雨を呼ぶ小さな精霊が寄る木の葉が湿気を感じて反り返っていた。"),
                    Beat::scene(5, "その木の葉を見るだけで雨が予測できるようになり、洗濯も屋根直しも慌てずに済むようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "苗床にない記念木",
                beats: &[
                    Beat::speech(6, "移住した住民", Villager, "引っ越しの祝いに贈ってもらった木が枯れちまったんだ。送り主が明日訪ねてくるのに申し訳なくて……。"),
                    Beat::speech(1, "苗床の世話役", Villager, "共同苗床に残っている同じ木の新芽を探して育て直すか、枯れた枝を削って綺麗な記念札に作り替えようか？"),
                ],
                choices: Some([
                    Ending { label: "残った新芽を育て直す", text: "小さな芽を見て送り主も喜び、また大きく育つ日を一緒に楽しみにした。" },
                    Ending { label: "枝を刻んで記念札にする", text: "美しい木目の札を玄関に掛け、枯れても変わらない友情の証になった。" },
                ]),
            },
            Story {
                title: "借りた畝の秋",
                beats: &[
                    Beat::speech(7, "新住民", Villager, "新しく来た家族が増えて、水運びや畑の世話が一部の人ばかりに偏って大変そうだよ。"),
                    Beat::speech(2, "畑の代表", Villager, "世帯ごとの時間に合わせた当番表を作るか、家から近い持ち場に割り振りを変えるか、試してみようか？"),
                ],
                choices: Some([
                    Ending { label: "暮らしに合う当番表を作る", text: "無理なく交代できるようになり、水運びも楽しいお喋りの時間になった。" },
                    Ending { label: "家に近い持ち場にまとめる", text: "往復の手間が減って作業が早く終わり、夕方の食卓に笑顔が増えた。" },
                ]),
            },
            Story {
                title: "空き地の足音",
                beats: &[
                    Beat::speech(4, "門番", Villager, "夜になると、旧村行きの門の向こうから懐かしい足音が聞こえる気がするんだ……。"),
                    Beat::scene(4, "残響をたどると、今は森になった古い井戸の場所だった。元住民がそっと新しい水桶を置くと、足音は静かに消えた。"),
                    Beat::scene(0, "元住民は古い家への区切りをつけ、笑顔で若芽の里の暮らしに戻っていった。"),
                ],
                choices: None,
            },
        ],
    },
];

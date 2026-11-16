use super::{Art, Beat, Ending, Landscape, Place, Plan, Story};
use crate::dialogue::Speaker::*;

pub(super) static NOX: [Plan; 10] = [
    Plan {
        name: "灰灯りの村",
        profile: "薪と食糧を分ける炉の寄合。炉を守る人なら、名がなくても発言できる。",
        terrain: Landscape::Meadow,
        entry: (158, 174),
        places: [
            Place { name: "共同炉", at: (154, 98), art: Some(Art::Hearth), ground: Landscape::Yard, detail: "避難民が最初の家を建てた場所。薪当番が火を守る。" },
            Place { name: "ガウェイン旧宅", at: (91, 106), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "アレンが父と暮らした家。隣人は雪の日のことを覚えている。" },
            Place { name: "冬越し小屋", at: (146, 38), art: Some(Art::Inn), ground: Landscape::Yard, detail: "身元を明かさずに休める小屋。斜面を登る薪道が通じる。" },
            Place { name: "薪割り場", at: (222, 80), art: Some(Art::Cottage), ground: Landscape::Wood, detail: "東の林縁に冬の薪を集めている。" },
            Place { name: "寄合小屋", at: (173, 104), art: Some(Art::Cottage), ground: Landscape::Square, detail: "炉を使う世帯の寄合。今夜は避難民名簿を議論する。" },
            Place { name: "薪道の雪隠れ", at: (207, 40), art: None, ground: Landscape::Wood, detail: "雪の上に小さな足跡が続く。" },
            Place { name: "旧宅の繕い台", at: (75, 101), art: None, ground: Landscape::Yard, detail: "壁に古い手袋の型紙が留めてある。" },
            Place { name: "炉の炊事場", at: (132, 111), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "昔使われた大鍋に、焦げた香草がこびりついている。" },
        ],
        stories: [
            Story {
                title: "炉の名札",
                beats: &[
                    Beat::speech(0, "炉番", Hearth, "警備隊が避難してきた人の名簿を出せって言ってる。冬越し小屋の夜番に、どこまで話していいか聞いてきておくれ。"),
                    Beat::speech(2, "夜番", Worker, "名札のない薪束にも、私の刻み目があるでしょう。夜番をしてることなら話していいよ。"),
                    Beat::speech(4, "寄合の世話役", Villager, "薪札は預かるよ。配給帳に名前を書いてもいいって、ご本人から聞いた。警備隊には薪札だけ渡すか、配給係にも名前を届けるか、どうしようか。"),
                ],
                choices: Some([
                    Ending { label: "薪札だけ出す", text: "警備隊には薪札だけを渡した。夜番は名を伏せたまま当番を続け、共同炉にも席をもらった。" },
                    Ending { label: "本人の名も届ける", text: "夜番の名が配給帳に加わった。次の当番の日、夜番は自分から共同炉のそばに座った。" },
                ]),
            },
            Story {
                title: "父の手袋",
                beats: &[
                    Beat::speech(1, "旧宅の隣人", Villager, "ほら、あんたのお父さんの手袋だよ。もう片方は、うちの繕い台に置いてなかったかね。"),
                    Beat::speech(6, "旧宅の隣人", Villager, "縫い目に『あれん』ってあるだろ。雪遊びに行く前に、お父さんがあんたの手に合わせて縫い縮めたんだよ。"),
                    Beat::scene(1, "アレンは手袋に指を入れてみた。すぐにつかえた。雪の中で父の手を握っていた頃には、これでちょうどよかった。"),
                ],
                choices: None,
            },
            Story {
                title: "煙を追う",
                beats: &[
                    Beat::speech(2, "小屋の世話役", Villager, "床の隙間から煙が上がってきたんだ。寝てた人はみんな外に出したけど、この寒さじゃ長くもたないよ。"),
                    Beat::speech(3, "薪割り場の職人", Worker, "ほら、風が吹くと煙突の曲がった管が揺れるだろ。継ぎ目が割れて、煙が床下にまわってる。"),
                    Beat::speech(2, "小屋の世話役", Villager, "風が弱まったね。管を直すあいだ、小屋の人たちはどこで暖を取ろうか。"),
                ],
                choices: Some([
                    Ending { label: "炉を止めて管を直す", text: "夜明けには煙が煙突からまっすぐ上がった。外へ出ていた人たちも、寝床に戻れた。" },
                    Ending { label: "薪割り場へ炉を移す", text: "小屋の人々は林縁で温まった。職人は翌日の明るい時間に管を直す。" },
                ]),
            },
            Story {
                title: "薪山の番人",
                beats: &[
                    Beat::speech(3, "薪割り場の職人", Worker, "薪が毎晩ひと束ずつ減るんだ。獣は薪なんか持っていかないだろうし、誰が使ってるんだろうね。"),
                    Beat::scene(5, "雪の窪みで、子どもが倒れていた旅人の手を温めている。火にくべていたのは薪割り場の薪だった。日が暮れる前なら、小屋へ運べそうだ。"),
                    Beat::scene(2, "小屋の空き寝台で旅人が目を覚ました。掛けられた毛布を見て、貸してくれた子はどこかと辺りを見回した。"),
                ],
                choices: None,
            },
            Story {
                title: "灰の中の晩餐",
                beats: &[
                    Beat::speech(7, "炊事番", Worker, "先代の鍋でみんなに振る舞いたいんだ。焦げ跡は残ってるけど、何の香草を入れてたのか分からなくてね。"),
                    Beat::speech(6, "旧宅の隣人", Villager, "私が覚えてるのは苦い香草だよ。冬越し小屋の古参は甘い根だと言うけど、昔は鍋が二つあったからね。"),
                    Beat::speech(7, "炊事番", Worker, "昔は二鍋あったのか。今夜使えるのはこの鍋ひとつだ。どっちを作ろうか。"),
                ],
                choices: Some([
                    Ending { label: "苦い香草の鍋", text: "古参の薪番が集まり、戦後に初めて根菜を分けた夜の話をした。" },
                    Ending { label: "甘い根の鍋", text: "子どもたちが皿を持って集まり、明日の炊事当番に手を挙げる。" },
                ]),
            },
        ],
    },
    Plan {
        name: "霧灯の港町",
        profile: "船頭と灯台守が入港料と当番を預かる。救難では灯台守が順を変えられる。",
        terrain: Landscape::Coast,
        entry: (147, 67),
        places: [
            Place { name: "石灯台と油蔵", at: (186, 147), art: Some(Art::Lighthouse), ground: Landscape::Shore, detail: "石灯台が霧の入江を照らす。油蔵の棚はほとんど空だ。" },
            Place { name: "荷揚げ桟橋", at: (143, 142), art: None, ground: Landscape::Dock, detail: "交易船が荷を下ろす。杭には潮の高さが刻まれている。" },
            Place { name: "救難艇庫", at: (83, 122), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "救難艇がいつでも出せるよう櫂を並べている。" },
            Place { name: "崖上信号台", at: (254, 92), art: Some(Art::Palisade), ground: Landscape::Heath, detail: "東の崖から沖と入江を見渡せる。" },
            Place { name: "干潮の舟揚げ場", at: (115, 149), art: None, ground: Landscape::Shore, detail: "海が引いた跡に古い舟を曳いた筋が見える。" },
            Place { name: "霧笛の反響壁", at: (231, 112), art: None, ground: Landscape::Heath, detail: "崖に沿った石壁が霧笛を町へ返す。" },
            Place { name: "潮だまり", at: (76, 145), art: None, ground: Landscape::Shore, detail: "捨て網の下で小さな波紋が動いている。" },
            Place { name: "南の通船桟橋", at: (145, 165), art: None, ground: Landscape::Dock, detail: "島の外へ向かう船がここから出る。" },
        ],
        stories: [
            Story {
                title: "最後の一缶",
                beats: &[
                    Beat::speech(0, "灯台守", Lightkeeper, "油があと一缶しかない。貨物船のために灯をともすと、救難艇に回すぶんが残らん。信号台から沖の様子を見てきてくれ。"),
                    Beat::scene(3, "沖に貨物船が二隻いる。入江の中にはもう霧が入り始めていた。信号旗が見えるうちなら、手前の船に油を頼めそうだ。"),
                    Beat::speech(0, "灯台守", Lightkeeper, "旗を振れば、あの船にはまだ届くか。油を借りるか、いまあるぶんで夜明けまで持たせるか……どう思う？"),
                ],
                choices: Some([
                    Ending { label: "夜明けまで火を絞る", text: "貨物船は沖で夜明けを待った。救難艇は残しておいた油で、小舟を曳いて帰ってきた。" },
                    Ending { label: "沖から油を借りる", text: "貨物船が予定通り着き、艇庫にも油が届いた。船頭の当番表に貸し分を記した。" },
                ]),
            },
            Story {
                title: "折り返さない舟",
                beats: &[
                    Beat::speech(2, "艇庫の船頭", Worker, "昔、帳面に載ってない小舟を、あの浜に曳き上げたことがある。入り江の低いところに、舵が残ってるかもしれんな。"),
                    Beat::speech(4, "網を繕う漁師", Worker, "その舵なら覚えてるよ。船名が彫ってあるだろ。乗ってた人なら、浜から自分の足で丘を越えていったよ。"),
                    Beat::scene(1, "家族は舵の船名をなぞった。海で亡くなったと思っていた人が、浜から丘へ歩いていった。その話を聞くと、家族は丘へ続く道を見上げた。"),
                ],
                choices: None,
            },
            Story {
                title: "霧笛の届く家",
                beats: &[
                    Beat::speech(0, "灯台守", Lightkeeper, "霧笛を鳴らすたび、崖のそばの三軒から苦情が来る。沖の船に聞こえなくなるのも困るんだがね。"),
                    Beat::scene(5, "霧笛が鳴ると、石壁の端で跳ね返った音が家並みのほうへ飛んだ。崖の上では、沖から返る音のほうが先に聞こえた。"),
                    Beat::scene(0, "霧笛の向きはそのままに、石壁に木の覆いを掛けた。港の鐘が鳴っても、今度は家の戸がばたばたと閉まることはなかった。"),
                ],
                choices: None,
            },
            Story {
                title: "潮だまりの客",
                beats: &[
                    Beat::voice(2, "艇庫の子", "ねえ、あの潮だまり。網の下で何か動いたよ。見てくれる？"),
                    Beat::scene(6, "捨て網の下で、海獣の仔が藻を押しのけようとしていた。網を外し、海へ続く細い水路まで桶で海水を足した。"),
                    Beat::scene(2, "仔は水路を抜けて沖へ泳いでいった。艇庫の子は浜を見渡し、残っていた古い網を束ね始めた。"),
                ],
                choices: None,
            },
            Story {
                title: "まだ借りられる桟橋",
                beats: &[
                    Beat::speech(1, "桟橋の係", Worker, "見送りの船に桟橋を貸したいんだが、ちょうど同じ刻に商船が着く。小舟をつなげるところが、ほかにないかね。"),
                    Beat::scene(7, "通船用の杭なら小舟をつなげる。ただし船を留められるのは一艘だけだ。見送りの船か、商船から荷を移した小舟なら使える。"),
                    Beat::speech(1, "桟橋の係", Worker, "浜で両方とも返事を待ってる。通船の杭は、どっちに使ってもらおうか。"),
                ],
                choices: Some([
                    Ending { label: "家族の船を外桟橋へ", text: "風の当たらない外桟橋で家族は船を見送った。商船はいつもの岸壁で荷を下ろした。" },
                    Ending { label: "荷を小舟で外桟橋へ", text: "商船は荷を分けて降ろした。家族はいつもの岸壁から船を見送った。" },
                ]),
            },
        ],
    },
    Plan {
        name: "西風の砦町",
        profile: "砦長が夜の門を、商人組合が昼の市を預かる。通行料は両方の承認が必要だ。",
        terrain: Landscape::Heath,
        entry: (27, 111),
        places: [
            Place { name: "木柵門", at: (64, 102), art: Some(Art::Palisade), ground: Landscape::Heath, detail: "荒野から来る荷馬をここで調べる。" },
            Place { name: "護送受付と市", at: (153, 100), art: Some(Art::Stalls), ground: Landscape::Square, detail: "護送団の出発と、次の峠を渡る荷を募っている。" },
            Place { name: "観測櫓", at: (162, 34), art: Some(Art::Palisade), ground: Landscape::Heath, detail: "峠の崩れと西の旧荷道が見通せる。" },
            Place { name: "荷馬避難柵", at: (166, 155), art: Some(Art::Cottage), ground: Landscape::Pasture, detail: "門の外で体を休める荷馬の場所。" },
            Place { name: "旧荷道の曲がり角", at: (77, 59), art: None, ground: Landscape::Heath, detail: "風化した目印が放牧地の方を向いている。" },
            Place { name: "見張り鐘", at: (197, 49), art: Some(Art::Clock), ground: Landscape::Heath, detail: "背後にひび割れた岩壁がある。" },
            Place { name: "護送団の荷改め場", at: (237, 108), art: Some(Art::Inn), ground: Landscape::Yard, detail: "戻った荷と支払控えを記録する。" },
            Place { name: "矢羽の工房", at: (125, 117), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "引退する弓兵が羽根を選り分けている。" },
        ],
        stories: [
            Story {
                title: "二枚の地図",
                beats: &[
                    Beat::speech(1, "市の商人", Worker, "護送代がこれ以上上がると、うちは店を畳むしかない。櫓に行けば、峠の脇に別の道が見えるはずだ。"),
                    Beat::scene(2, "峠の迂回路は放牧地の柵を横切る。櫓から見ると、旧荷道は放牧地を大きく外れ、昼なら先まで見渡せる。"),
                    Beat::speech(1, "商人組合の世話役", Worker, "昼だけ旧荷道を使わせてもらうか、何軒かで護送代を出し合うか。砦長に話を持っていくなら、どっちがいい？"),
                ],
                choices: Some([
                    Ending { label: "旧荷道を昼だけ開く", text: "巡回が昼の旧荷道に移り、柵沿いの抜け道が開いた。" },
                    Ending { label: "護送料を共同払いにする", text: "店を何軒かまとめて荷車を一台にし、護送代を分け合った。旧荷道の柵は開けずに済んだ。" },
                ]),
            },
            Story {
                title: "鐘が二度鳴る",
                beats: &[
                    Beat::speech(5, "見張りの番兵", Guard, "鐘は一度しか打ってない。なのに北の荒野じゃ二度聞こえたらしい。荷運びが驚いて街道から逃げたんだ。"),
                    Beat::scene(4, "曲がり角で岩に向かって呼ぶと、鐘の方角から声が少し遅れて返ってきた。その声を聞いて、岩陰から荷運び人が顔を出した。"),
                    Beat::scene(5, "番兵は音の返る場所を札に描き、鐘の下に掛けた。荷運び人はそれを見てから、ほどけた荷縄を結び直した。"),
                ],
                choices: None,
            },
            Story {
                title: "柵の内側で育つ",
                beats: &[
                    Beat::speech(0, "門番", Guard, "その仔馬、ずっと震えてるな。病気かもしれん。先に避難柵で診てもらってくれ。"),
                    Beat::speech(3, "馬の世話役", Worker, "母馬のそばに寄せたら、震えが止まったね。この仔は長旅で冷えただけみたいだよ。"),
                    Beat::scene(0, "門番は馬を一晩休ませる札を書いた。柵の奥に母馬と仔馬の水桶が並び、乾いた藁が敷かれた。"),
                ],
                choices: None,
            },
            Story {
                title: "帳面を持った賊",
                beats: &[
                    Beat::speech(6, "荷改めの番兵", Guard, "取り返した荷に、市の印がないんだ。荷主も襲った連中も、前に運んだぶんの代金がまだだと言ってる。"),
                    Beat::speech(1, "市の帳付け", Worker, "この支払控えを見てくれ。荷主は代理人を通してる。襲った連中は、その代理人からずっと金をもらえずにいたらしい。"),
                    Beat::speech(6, "荷改めの番兵", Guard, "荷はここで返せる。未払いの話は審理に回すか、荷を返すところで両方に支払い方を決めてもらうか。どうする？"),
                ],
                choices: Some([
                    Ending { label: "公開審理に回す", text: "公開の審理に代理人も呼ばれた。ほかの運び手も、払われていない代金について申し立てられるようになった。" },
                    Ending { label: "返還と分割払いを結ぶ", text: "荷はすぐ持ち主へ戻り、運び手は市役の立会いで最初の支払いを受けた。" },
                ]),
            },
            Story {
                title: "最後の矢羽",
                beats: &[
                    Beat::speech(7, "老弓兵", Guard, "引退する前に、あの若い門兵と一度だけ射ち比べたいんだ。狙いの付け方と風を読む立ち位置、どっちを先に教えようかね。"),
                    Beat::scene(2, "櫓から見ると、柵の内と外で旗が別の向きにはためいている。足元では若い門兵が、自分の矢を一本ずつ並べていた。"),
                    Beat::speech(7, "老弓兵", Guard, "よし、始めよう。あいつに先に見せるのは、的を狙うところか、それとも風上に立つところか。"),
                ],
                choices: Some([
                    Ending { label: "柵内の的を射る", text: "若い門兵は、矢羽と弦を同じ角度に揃える手つきを教わった。" },
                    Ending { label: "外で風を読む", text: "若い門兵は遠くの旗から追い風を読む方法を教わった。翌日にはさっそく、荷道の見張りで試した。" },
                ]),
            },
        ],
    },
    Plan {
        name: "葦渡りの村",
        profile: "水路ごとの当番が板道を守る。水門は葦刈り手と漁師の立会いで開閉する。",
        terrain: Landscape::Marsh,
        entry: (61, 110),
        places: [
            Place { name: "杭上の寄合所", at: (152, 98), art: Some(Art::Inn), ground: Landscape::Dock, detail: "高床の部屋を板道がつなぐ。戸口に水路の割当表がある。" },
            Place { name: "西の渡し桟橋", at: (91, 98), art: None, ground: Landscape::Dock, detail: "底の浅い舟が板道の切れ目をつなぐ。" },
            Place { name: "北の水門", at: (138, 34), art: Some(Art::Sluice), ground: Landscape::Dock, detail: "上流の目盛りには浚渫前の基準が残っている。" },
            Place { name: "産卵棚", at: (191, 153), art: None, ground: Landscape::Dock, detail: "下流で魚の卵を数える棚。脇の杭に潮位板がある。" },
            Place { name: "古い舟大工の小屋", at: (68, 60), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "舟大工は川筋が変わる前を知っている。" },
            Place { name: "花の杭", at: (165, 118), art: None, ground: Landscape::Dock, detail: "板の下へ向けて花が供えられている。" },
            Place { name: "葦籠の作業場", at: (214, 77), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "風を通す高台で籠を編んでいる。" },
            Place { name: "舟で入る浅瀬", at: (169, 145), art: None, ground: Landscape::Dock, detail: "板道からは仔獣の鳴き声だけが聞こえる。" },
        ],
        stories: [
            Story {
                title: "二つの水位",
                beats: &[
                    Beat::speech(0, "水路の当番", Villager, "上と下で水位の話が食い違ってるんだ。水を下げたい家も魚を守りたい家も、相手が勝手に水門をいじったと思ってる。"),
                    Beat::scene(2, "水門の目盛りには、門を閉めたときの高さが刻んである。それに合わせて見ると、上流だけが普段より高い。"),
                    Beat::speech(3, "下流の漁師", Worker, "ここの水まで下げたら、産卵棚が干上がるよ。ほら、泥の筋が脇の水路に続いてる。流れがそっちへ抜けたんだ。"),
                    Beat::speech(4, "舟大工", Worker, "昔はこう流れてたんだ。浚ったときの土で、この枝水路が塞がってる。そこを掘り直せば、魚棚の水を残したまま葦を刈れるよ。"),
                    Beat::scene(0, "葦刈りの水路だけ水を落とすと、魚棚には水が残った。当番は川沿いの板をつなぎ、向こう岸まで歩けるようにした。"),
                ],
                choices: None,
            },
            Story {
                title: "渡し舟の輪番",
                beats: &[
                    Beat::speech(1, "渡し守", Worker, "櫂を握る手を痛めちまってね。今夜、舟に乗る人が何人いるか、寄合で聞いてきてくれないか。"),
                    Beat::speech(0, "寄合の世話役", Villager, "夜の舟を使うのは三人だよ。浅い舟でよければ、代わりに漁師が櫂を取ってくれるって。"),
                    Beat::speech(1, "渡し守", Worker, "三人なら浅い舟に乗れる。漁師に夜便を頼むか、舟べりに手すりを付けて片手で乗り降りできるか試すか、どっちにしよう。"),
                ],
                choices: Some([
                    Ending { label: "漁師に夜番を頼む", text: "渡し守は手を休める。漁師は夜便を受け持ち、代わりに昼の板道当番を減らしてもらった。" },
                    Ending { label: "片手用の舟を試す", text: "最初の三人を無事に運んだ。渡し守は翌日も無理のない明るい便を受け持つ。" },
                ]),
            },
            Story {
                title: "杭に咲く花",
                beats: &[
                    Beat::speech(5, "板道の職人", Worker, "この杭に花が置いてあってね。板を外す前に、誰が何のために置いたのか聞いてきてくれないか。"),
                    Beat::scene(4, "舟大工の古い図に花の位置を重ねると、水に沈んだ家の台所と戸口に当たった。"),
                    Beat::scene(5, "古い杭の位置に合わせて、新しい板に印を彫った。花を供えていた人は、印の上にそっと花を置いた。"),
                ],
                choices: None,
            },
            Story {
                title: "籠をさらう風",
                beats: &[
                    Beat::speech(6, "籠売り", Villager, "風が強い日だけ、積んだ籠が落ちるんだよ。編んだ子は、自分の並べ方が悪いと思ってるけどね。"),
                    Beat::scene(3, "産卵棚の網に籠が引っかかっていた。濡れた底を合わせてみると、口の広いほうが風を受ける形になっていた。"),
                    Beat::scene(6, "籠の底に小石を入れ、口を風下に向けて並べ直した。次に風が吹いたとき、籠は棚に残った。"),
                ],
                choices: None,
            },
            Story {
                title: "板の下の産声",
                beats: &[
                    Beat::speech(0, "寄合の見回り", Villager, "夜になると板道の下で鳴き声がするんだ。上から見ても、黒い影しか見えなくてね。"),
                    Beat::scene(7, "舟を板道の下に入れると、水が増えて取り残された仔獣がいた。下流の泥には、親らしい足跡が続いている。"),
                    Beat::speech(0, "寄合の見回り", Villager, "人が通らないようにして親を待つか、舟で仔を親のいる浅瀬まで誘うか。どっちがよさそうだい？"),
                ],
                choices: Some([
                    Ending { label: "板道を閉じて親を待つ", text: "足音が途絶えると、親が板道の下へ泳いできた。仔がそばへ寄り、二頭で水面に浮かんだ。" },
                    Ending { label: "舟で広い浅瀬へ誘う", text: "仔が舟を追い、産卵棚の脇を通らず親のいる浅瀬へ泳ぎ出した。" },
                ]),
            },
        ],
    },
    Plan {
        name: "オオ麦の里",
        profile: "耕作者が種蔵番を選ぶ。砦へ出す麦の量は里と辺境領が取り決める。",
        terrain: Landscape::Farm,
        entry: (276, 103),
        places: [
            Place { name: "風車", at: (143, 39), art: Some(Art::Mill), ground: Landscape::Farm, detail: "北尾根では風が強く、羽根の向きを読みやすい。" },
            Place { name: "共同種蔵", at: (155, 95), art: Some(Art::Inn), ground: Landscape::Square, detail: "作柄の悪い家が来年分の種を借りられる。" },
            Place { name: "南の霜害畑", at: (133, 149), art: None, ground: Landscape::Farm, detail: "斜面の下で麦が白く枯れ、石垣の脇だけ緑が残る。" },
            Place { name: "供出秤場", at: (238, 101), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "砦へ向かう荷車を量る。袋には別々の荷札がある。" },
            Place { name: "共同試験畑", at: (83, 116), art: None, ground: Landscape::Farm, detail: "力を合わせて耕す小さな畑。今季は一区画空いている。" },
            Place { name: "粉挽きの試食台", at: (181, 83), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "挽き方の違う粉が小棚に並ぶ。" },
            Place { name: "風車の制動足場", at: (159, 35), art: None, ground: Landscape::Heath, detail: "羽根を止める仕組みと手挽き石がここにある。" },
            Place { name: "畑の石垣", at: (110, 144), art: None, ground: Landscape::Heath, detail: "日が暮れると冷気が石垣の手前にたまる。" },
        ],
        stories: [
            Story {
                title: "種蔵の秤",
                beats: &[
                    Beat::speech(3, "秤場の係", Worker, "砦へ出す麦を量ったら、注文より重いんだ。風車に積み荷の控えがあるから、確かめてきてくれないか。"),
                    Beat::record(0, "風車の積み荷控え", "黄色い荷札の袋は来年まく種麦。砦への荷には入れず、共同種蔵に戻すこと。"),
                    Beat::speech(1, "砦の補給係", Worker, "黄色の札が付いた二袋は倉へ戻した。砦へは古い粉を送るか、近くの里から麦を借りるか、どっちにしよう。"),
                ],
                choices: Some([
                    Ending { label: "古い粉を送る", text: "砦は保存菓子に使う粉として引き取った。里では、各戸に来年の種を借りるための札が配られた。" },
                    Ending { label: "近隣の里から麦を借りる", text: "次の収穫で返す条件を結び、今季の粉を里の食卓にも残した。" },
                ]),
            },
            Story {
                title: "霜よけの夜",
                beats: &[
                    Beat::speech(2, "畑の世話役", Villager, "南の畑が霜で真っ白だよ。石垣が途切れてるところだけ、苗がまだ枯れてないんだ。"),
                    Beat::scene(7, "石垣の切れ目から冷たい空気が下へ流れていく。切れ目のない側では空気がよどみ、霜が白く残っていた。"),
                    Beat::speech(2, "畑の世話役", Villager, "今夜も冷えそうだよ。石垣にもう一か所隙間を作るか、苗に覆いを掛けて朝まで待つか……どっちを試そうか。"),
                ],
                choices: Some([
                    Ending { label: "石垣の一部を開く", text: "冷気が溜まらず、開いた側の苗が翌朝も緑だった。" },
                    Ending { label: "苗を藁で覆う", text: "藁に霜が積もった。下の苗が残り、列ごとの覆い方が試験畑にも伝わる。" },
                ]),
            },
            Story {
                title: "挽きたて一等賞",
                beats: &[
                    Beat::speech(5, "菓子屋", Worker, "焼き菓子を二種類作ってみたいんだ。風車で細かい粉と粗い粉を少しずつ分けてもらえる？"),
                    Beat::speech(0, "風車の粉挽き", Worker, "こっちが粗挽き。固めに焼けば日持ちするよ。細挽きのほうは、生地を柔らかくしたいときに使ってみて。"),
                    Beat::speech(5, "菓子屋", Worker, "もう試食の人が集まってる。まず出すのは、日持ちするほうと柔らかいほう、どっちにしよう？"),
                ],
                choices: Some([
                    Ending { label: "粗挽きの保存菓子を焼く", text: "護送団の注文が入った。袋に日持ちする日数の札が付いた。" },
                    Ending { label: "細挽きの菓子を焼く", text: "宿の主人が朝食用に予約した。焼き手は次の試食日を決めた。" },
                ]),
            },
            Story {
                title: "止まらない風車",
                beats: &[
                    Beat::speech(0, "風車の親方", Worker, "風がやんでも羽根の軸がきしむんだ。あの音を聞くと、誰も梯子を上りたがらなくてな。"),
                    Beat::scene(6, "羽根を止める木の部品は、端が擦り切れていた。足場の脇には、風車を止めても粉を挽ける小さな石臼がある。"),
                    Beat::scene(0, "みんなで交代しながら石臼を回した。粉を切らさずに済み、その間に親方は羽根を止める木の部品を削り直した。"),
                ],
                choices: None,
            },
            Story {
                title: "畑の貸し手",
                beats: &[
                    Beat::speech(4, "借り畑の農家", Villager, "この秋から畑を借りたんだ。ところが、まく種を貸すのは畑の持ち主だけだって言われてね。"),
                    Beat::speech(1, "種蔵番", Worker, "初めての畑で試すぶんの種は、少し余ってるよ。ただ、あの人が土をどう手入れしてるか、まだ見てないんだ。"),
                    Beat::speech(4, "種蔵番", Worker, "畝はよく手入れされてるね。畑主に保証してもらって種を借りるか、共同の試験畑を一区画使うか。どちらなら続けられそうだい？"),
                ],
                choices: Some([
                    Ending { label: "畑主の保証で借りる", text: "借りた畑に春も種をまける。保証札には、借り手自身の名前も入った。" },
                    Ending { label: "試験畑を一区画借りる", text: "共同畑で一作を始めた。翌年は自分の収穫札を種蔵に持っていける。" },
                ]),
            },
        ],
    },
    Plan {
        name: "石継ぎの町",
        profile: "石工会が建物と水路を検査する。井戸の水番は使う組が持ち回る。",
        terrain: Landscape::Stone,
        entry: (276, 121),
        places: [
            Place { name: "アーチ井戸", at: (161, 96), art: Some(Art::Well), ground: Landscape::Square, detail: "煤けた旧村の敷石が、水場の敷居に組み込まれている。" },
            Place { name: "石工場と拓本台", at: (87, 90), art: Some(Art::Inn), ground: Landscape::Stone, detail: "出張鍛冶師のドワーフ、ブロムが石工の道具を直し、旅人の武具も打つ。" },
            Place { name: "暗渠入口", at: (145, 32), art: Some(Art::Sluice), ground: Landscape::Stone, detail: "上から水が滴る。崩れた区画を避ける石目が残る。" },
            Place { name: "転用石置場", at: (160, 153), art: None, ground: Landscape::Stone, detail: "敷石のそばに、石工が水路掃除で拾った沼鉄鉱が分けてある。" },
            Place { name: "水番所", at: (172, 106), art: Some(Art::Cottage), ground: Landscape::Stone, detail: "水番が濁りと雨漏りの通報を受ける。" },
            Place { name: "段差のある家", at: (206, 77), art: Some(Art::Cottage), ground: Landscape::Stone, detail: "大きな敷石が元の家から使い継がれている。" },
            Place { name: "雨染みの裏路地", at: (199, 123), art: None, ground: Landscape::Stone, detail: "濡れているのは片方の壁だけだ。" },
            Place { name: "拓本を待つ古い壁", at: (131, 137), art: None, ground: Landscape::Stone, detail: "壁の裏に、隣家が探す戸口印がのぞく。" },
        ],
        stories: [
            Story {
                title: "井戸底の戸口印",
                beats: &[
                    Beat::speech(0, "井戸端の住民", Villager, "暗渠を閉める前に、敷石の戸口印を写してくれないか。煤けた旧村にあった、うちの敷石なんだ。"),
                    Beat::scene(2, "石目の続くほうをたどると、崩れた床を避けて奥へ進めた。水の流れが緩いところに、二軒ぶんの戸口印が並んでいた。"),
                    Beat::speech(1, "拓本師", Mason, "拓本の紙も、石を引き上げる綱も用意できる。もうすぐ暗渠を塞ぐよ。どっちを持っていく？"),
                ],
                choices: Some([
                    Ending { label: "拓本を取り水路を塞ぐ", text: "水番は予定どおり井戸を使えるようにした。二軒の遺族は、写した戸口印を石置場に掲げた。" },
                    Ending { label: "銘のある石を運び出す", text: "石は遺族の手に戻った。水番は今夜も井戸を止め、別の水を配った。" },
                ]),
            },
            Story {
                title: "玄関の一段",
                beats: &[
                    Beat::speech(5, "家の住人", Villager, "玄関の段が高くて、ひとりじゃ外へ出られないんだ。でも、あの敷石は祖父が運んできたものでね。"),
                    Beat::speech(1, "石工", Worker, "低い石と木の斜路を試してみたよ。戸はどっちでも開くし、これなら段を越えられるね。"),
                    Beat::speech(5, "家の住人", Villager, "この石を残して横に斜路を付けるか、段をなくして平らな敷石に替えるか。どっちが暮らしやすいかな。"),
                ],
                choices: Some([
                    Ending { label: "石の横に斜路を足す", text: "祖父の敷石は玄関に残った。家の人は、翌朝から斜路を通って井戸に出かけた。" },
                    Ending { label: "平らな敷石に替える", text: "敷石を替えると、路地に斜路を出さずに済んだ。古い敷石は中庭で腰掛けになった。" },
                ]),
            },
            Story {
                title: "石の雨漏り",
                beats: &[
                    Beat::speech(4, "水番", Villager, "井戸の前に、隣の壁を見てくれないか。雨が降ると、毎回あそこだけ濡れるんだ。"),
                    Beat::scene(6, "染みを上へたどると、隣家の屋根樋が途中で切れていた。そこから落ちた水が、こちらの壁の継ぎ目に入っている。"),
                    Beat::scene(4, "二軒で余った樋を継ぎ、雨水を桶に受けるようにした。次の雨の日には、たまった水で二軒の間の路地を洗えた。"),
                ],
                choices: None,
            },
            Story {
                title: "よその家の石",
                beats: &[
                    Beat::speech(1, "石工", Worker, "置場に戻した石の家印が、新しい家の壁から見えるんだ。どこから運んだ石か、確かめてくれないか。"),
                    Beat::scene(7, "壁の石に付いた運搬札には、家印を下へ向けて置いたとある。石を買った家が、印を見ないまま受け取ったのも無理はない。"),
                    Beat::scene(3, "転用石置場で両家が話した。印を刻んだ家族は、次の改築まで、その家を訪ねて印を見せてもらえることになった。"),
                ],
                choices: None,
            },
            Story {
                title: "石工の音当て",
                beats: &[
                    Beat::speech(1, "石工の弟子", Worker, "試験で、叩いた音からひびの入った石を見分けなきゃいけないんだ。でも、どう聞き分けるのか分からなくて。"),
                    Beat::scene(3, "置場にある同じ種類の石を叩き比べた。ひびに近い端は、中央を叩いたときより響きが早く消える。"),
                    Beat::scene(1, "弟子は試験でひびの場所を音で当てた。その石を捨てず、ひびの反対側から切り出せることも示した。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "遠火の宿場",
        profile: "旅籠と馬方が宿場を運営する。厩番は救急の馬を先に出せる。",
        terrain: Landscape::Crossing,
        entry: (156, 173),
        places: [
            Place { name: "灯火旅籠", at: (151, 104), art: Some(Art::Inn), ground: Landscape::Square, detail: "飛脚の目印となる火が屋根に灯る。" },
            Place { name: "替え馬厩", at: (84, 110), art: Some(Art::Cottage), ground: Landscape::Pasture, detail: "峠越えを終えた馬が休んでいる。" },
            Place { name: "郵便仕分け所", at: (221, 107), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "列車便と徒歩便に分け、宛名を確かめる。" },
            Place { name: "薬荷物置場", at: (150, 147), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "谷守へ送る小包は優先札で分けてある。" },
            Place { name: "旧飛脚道の番屋", at: (211, 35), art: Some(Art::Cottage), ground: Landscape::Heath, detail: "谷へ向かう道。古い折返し表が読める。" },
            Place { name: "峠を望む尾根", at: (129, 51), art: None, ground: Landscape::Heath, detail: "街道から宿の火が見えるかを確かめられる。" },
            Place { name: "宿の裏口", at: (132, 91), art: None, ground: Landscape::Yard, detail: "風が吹くたびに、床下で硬い音がする。" },
            Place { name: "小さな放牧地", at: (58, 135), art: None, ground: Landscape::Pasture, detail: "脚を休める馬が歩ける緩い斜面。" },
        ],
        stories: [
            Story {
                title: "夜便をつなぐ",
                beats: &[
                    Beat::speech(1, "厩番", Worker, "次の馬車を待ってたら、谷守の診療所が閉まっちまう。昔の飛脚道を使えば間に合うか、見てきてくれないか。"),
                    Beat::record(4, "番屋の運行表", "急ぎの荷は峠の手前から徒歩で運ぶ。夜の番屋には替え馬を一頭残す。借りるときは当番に声を掛けること。"),
                    Beat::speech(3, "薬荷物の係", Worker, "薬はもう包んであるよ。飛脚道を歩いて届けるか、夜の番屋で替え馬を借りるか。どっちで行こう？"),
                ],
                choices: Some([
                    Ending { label: "番屋に夜の馬を置く", text: "峠まで夜便が走り、薬と急ぎの便りを運んだ。宿は当番の交代日を増やした。" },
                    Ending { label: "薬だけ徒歩便で送る", text: "薬の小包が診療の前に着いた。馬方の夜番は増やさずに済んだ。" },
                ]),
            },
            Story {
                title: "火が見えない夜",
                beats: &[
                    Beat::speech(0, "宿主", Villager, "昨夜の飛脚が、うちの火に気づかず通り過ぎたって。峠で夜が明けるまで待ったそうだよ。"),
                    Beat::scene(5, "尾根から屋根を見下ろすと、強い風のたびに覆いが倒れて、火を隠してしまう。"),
                    Beat::scene(0, "覆いを作り替え、風を横に逃がした。次の夜、飛脚は尾根から火を見つけ、まっすぐ宿にたどり着いた。"),
                ],
                choices: None,
            },
            Story {
                title: "名前を持たない手紙",
                beats: &[
                    Beat::speech(2, "郵便係", Worker, "宛名だけがはがれた手紙があるんだ。端にこの印があるだろ。届ける人の名前を人前で呼ばないでくれっていう、差出人の頼みだよ。"),
                    Beat::speech(0, "旅籠の常連", Traveler, "この封の折り方、谷へ越した人の家で見たことがあるよ。心当たりに聞いてみる。名は人前で呼ばないと約束しておくれ。"),
                    Beat::scene(2, "宿主は受取人を確かめ、封を切らずに手紙を渡した。返書には名前の代わりに、小さな伝言札が付いていた。"),
                ],
                choices: None,
            },
            Story {
                title: "替え馬の休み",
                beats: &[
                    Beat::speech(1, "厩番", Worker, "もう遠くまで走れない馬に買い手がついた。でも、峠で何度も助けてもらった馬なんだよ。"),
                    Beat::speech(7, "放牧地の世話役", Worker, "冬のあいだだけなら、うちで世話できるよ。まだ歩けそうなら、谷の薬便に短い平道の仕事もある。"),
                    Beat::speech(1, "厩番", Worker, "冬は放牧地で休ませるか、近場の薬便を頼むか。あの馬にはどっちがいいかな。"),
                ],
                choices: Some([
                    Ending { label: "近い薬便を手伝う", text: "馬は軽い薬の小包を背負い、短い道だけ歩くようになった。馬方は毎朝、足取りを見てから送り出した。" },
                    Ending { label: "谷の放牧地に預ける", text: "忙しかった馬が群れと歩く。厩番は餌を持って会いに行く。" },
                ]),
            },
            Story {
                title: "足音のすき間",
                beats: &[
                    Beat::speech(0, "宿主", Villager, "壁の中を誰かが歩く音がするって、うちの子が言うんだ。それで、屋根の火を見に行く当番を怖がっててね。"),
                    Beat::scene(6, "風が吹くと裏口の床下で空箱が動いた。引っかかっていた荷縄を外し、穴から中をのぞくと、箱が壁をこすっていた。"),
                    Beat::scene(0, "夕食のあと、宿の子の前で空箱を動かしてみせた。同じ音がすると、子は自分から屋根の火を見に行くと言った。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "ノクス北門街",
        profile: "門衛隊が防衛を担い、街区代表が住宅と配給を審議する。",
        terrain: Landscape::Fort,
        entry: (157, 172),
        places: [
            Place { name: "紫旗門と詰所", at: (157, 35), art: Some(Art::StoneGate), ground: Landscape::Stone, detail: "門上の紫の旗は帰還できた部隊の色を受け継ぐ。" },
            Place { name: "配給所", at: (158, 101), art: Some(Art::Stalls), ground: Landscape::Square, detail: "朝の引換札と夜警の帳面が別々にある。" },
            Place { name: "西の住宅街", at: (81, 101), art: Some(Art::Inn), ground: Landscape::Stone, detail: "家の数に比べて入居待ちの人が多い。" },
            Place { name: "旧詰番板と閲覧所", at: (232, 83), art: Some(Art::Cottage), ground: Landscape::Stone, detail: "同じ徽章の下に、年の食い違う二つの日付がある。" },
            Place { name: "外濠の待合", at: (231, 142), art: None, ground: Landscape::Yard, detail: "遺族と門番の家族が腰を下ろしている。" },
            Place { name: "旗の繕い場", at: (119, 56), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "裏返した旗に色の異なる継ぎ布がある。" },
            Place { name: "水漏れの空き家", at: (67, 122), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "鍵は新しく、柱に去年の雨の跡がある。" },
            Place { name: "夕方の配給窓口", at: (178, 118), art: None, ground: Landscape::Stone, detail: "門へ戻る夜警も立ち寄れる街路に面している。" },
        ],
        stories: [
            Story {
                title: "二度帰った部隊",
                beats: &[
                    Beat::speech(3, "詰所の衛兵", Guard, "日付が違うのに、詰番板じゃ同じ徽章の部隊が戻ったことになってる。これじゃ遺族に何を話せばいいのか、俺たちにも分からない。"),
                    Beat::scene(5, "旗の継ぎ布に、戦が終わったあとの年が記されていた。古い部隊色の上に、今の部隊の胸章を縫い付けた跡がある。"),
                    Beat::speech(0, "詰所の衛兵", Guard, "引換印と旗の記録がそろったよ。遺族と衛兵を一緒に呼ぶか、まず家族にだけ話すか、どうしよう。"),
                ],
                choices: Some([
                    Ending { label: "遺族と門衛に一緒に話す", text: "両者が徽章の継承を同じ席で聞いた。死者の名と現役の隊を分けて記す。" },
                    Ending { label: "まず遺族へ経緯を知らせる", text: "家族が名札を受け取り、後日、同意した名を詰所で照合する日を決めた。" },
                ]),
            },
            Story {
                title: "空き家の鍵",
                beats: &[
                    Beat::speech(2, "住宅街の世話役", Villager, "雨漏りしてるあの家の鍵を、二組の家族が持ってるんだ。どっちの家族も、先週やっと戻ってきたばかりでね。"),
                    Beat::scene(6, "濡れているのはひと部屋だけで、壁は直せそうだ。隣の家にも長く使われていない部屋がひとつある。"),
                    Beat::speech(2, "住宅街の世話役", Villager, "両方の家族が来てくれたよ。この家を直して一緒に住むか、向かいの空き部屋も開けてもらうか、どっちから話そうか。"),
                ],
                choices: Some([
                    Ending { label: "修理して同居する", text: "二家族は乾いた部屋を先に使い、雨を止める作業を分担した。" },
                    Ending { label: "対面の空室も開ける", text: "街区会が向かいの空き部屋を開けた。一方の家族は、雨漏りが直るまでそこを使えることになった。" },
                ]),
            },
            Story {
                title: "外濠の手すり",
                beats: &[
                    Beat::speech(4, "外濠の衛兵", Guard, "ここで子どもが足を滑らせてね。でもこの近道は、年寄りの見回りにも欠かせないんだ。"),
                    Beat::scene(0, "門から回ると、待合から住宅までずいぶん遠くなる。濠の低い側なら地面が固く、手すりの杭を打てそうだ。"),
                    Beat::scene(4, "職人が外濠沿いに手すりを付けた。見回りの年寄りは途中で休めるようになり、子どもは門から帰る道を覚えた。"),
                ],
                choices: None,
            },
            Story {
                title: "旗の裏の色",
                beats: &[
                    Beat::speech(5, "旗を手入れする住民", Villager, "旗を返したら、裏が違う色だったんだ。父の隊の色だと思ってたけど、今の隊の色とも違ってね。"),
                    Beat::speech(3, "遺族", Villager, "この二色なら覚えてるよ。継ぎ目を見てごらん。ここにある印は、旗を直した日だよ。"),
                    Beat::speech(5, "旗を手入れする住民", Villager, "そろそろ旗を掲げよう。元の隊の色に戻すか、縫い足された色もそのまま見せるか、どうしようか。"),
                ],
                choices: Some([
                    Ending { label: "古い配色へ繕う", text: "元の隊の旗が家族の行列を迎えた。今の旗は見張り詰所用に別に仕立てる。" },
                    Ending { label: "継ぎ布ごと掲げる", text: "古い色と今の色が風に並んだ。門衛は掛け替えの記録を閲覧所に寄せた。" },
                ]),
            },
            Story {
                title: "配給所の朝番",
                beats: &[
                    Beat::speech(1, "配給係", Worker, "夜警明けの人や具合の悪い人は、朝の列に並べないんだ。札が残ってても、食べ物までは届かなくてね。"),
                    Beat::scene(7, "見回りが帰る路地に、夕方なら使える窓口があった。朝に来られる世話役なら、代わりに受取札を持っていけるという。"),
                    Beat::speech(1, "配給係", Worker, "朝と夕方、両方の帳面を出しておいたよ。夕方にも開けるか、朝のうちに世話役に渡すか、どっちから始めよう？"),
                ],
                choices: Some([
                    Ending { label: "受取札を代理で預ける", text: "世話役が朝の列に並んで二軒ぶんを受け取り、具合の悪い人の家まで食べ物を届けた。" },
                    Ending { label: "夕方にも配る", text: "当番を分け、見回りの帰り道に門衛が受け取れるようになった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "谷守の村",
        profile: "橋の鍵は南北の代表が一本ずつ持つ。非常時は片岸の判断で門を開けられる。",
        terrain: Landscape::Valley,
        entry: (159, 173),
        places: [
            Place { name: "吊橋と集会所", at: (159, 99), art: Some(Art::Bridge), ground: Landscape::Dock, detail: "谷の両岸に暮らす家を結ぶ。縄の交換時期が近い。" },
            Place { name: "北岸の石積み場", at: (165, 48), art: Some(Art::Stalls), ground: Landscape::Stone, detail: "斜面を固める石が荷車に積まれている。" },
            Place { name: "南岸の診療所", at: (181, 134), art: Some(Art::Inn), ground: Landscape::Yard, detail: "峠を越えた人も診る。診察票と往診札がある。" },
            Place { name: "谷底の沢道入口", at: (112, 118), art: None, ground: Landscape::Heath, detail: "軽い荷なら運べる細い沢道が下る。" },
            Place { name: "旧居の跡", at: (72, 104), art: None, ground: Landscape::Heath, detail: "谷底に炉の石と古い蜂の巣が残る。" },
            Place { name: "北岸の避難鐘", at: (191, 61), art: Some(Art::Clock), ground: Landscape::Stone, detail: "橋が通れるかを鐘の打ち方で知らせる。" },
            Place { name: "南岸の音見台", at: (220, 122), art: None, ground: Landscape::Heath, detail: "谷を渡ってきた鐘と水音を聞き分けられる。" },
            Place { name: "往診札の掲示板", at: (162, 121), art: None, ground: Landscape::Yard, detail: "急ぎの往診に使う札を橋守が読める位置にある。" },
        ],
        stories: [
            Story {
                title: "荷の重さを分ける",
                beats: &[
                    Beat::speech(0, "橋守", Worker, "重い荷車を通したら、この縄じゃ持たない。北岸には工事の荷が要るし、南には診療所へ行きたい人も待ってる。"),
                    Beat::speech(1, "石積みの職人", Worker, "量ってみたら、荷車一台に積むには重すぎるね。小分けにすれば、人の背で運べるよ。"),
                    Beat::scene(3, "水位が低い日なら、軽い荷は沢道を通せる。背負って沢を渡るか、先に橋の縄を取り替えるか、石積みの職人が二つの案を出した。"),
                    Beat::speech(0, "橋守", Worker, "橋の鍵は持ってきた。今日は荷を分けて沢を通すか、先に縄を替えて橋を開けるか、どうしよう。"),
                ],
                choices: Some([
                    Ending { label: "軽い荷を沢道へ回す", text: "吊橋を通院に空け、沢道にも荷を分けて渡せる足場が開いた。" },
                    Ending { label: "先に橋縄を取り替える", text: "石積み屋が昼までに縄を替えた。渡れなかった人には診療所から往診が出た。" },
                ]),
            },
            Story {
                title: "沢の子守歌",
                beats: &[
                    Beat::speech(2, "診療所の患者", Villager, "ここへ越す前に聞いた子守歌を、もう一度聞きたいんだ。谷底の家で聞いたような気がしてね。"),
                    Beat::scene(4, "古い炉石に手を当てると、短い歌が聞こえた。後半は沢の音にかき消されてしまう。"),
                    Beat::speech(1, "石積みの歌い手", Worker, "その節なら、仕事歌の頭によく似てるよ。続きが分かるかもしれない。診療所に連れてってくれる？"),
                    Beat::scene(2, "患者は歌い手の節を聞き、思い出した部分を小声でつないだ。隣で聞いていた子も、帰り道に続きを口ずさんだ。"),
                ],
                choices: None,
            },
            Story {
                title: "鐘を鳴らす人",
                beats: &[
                    Beat::speech(5, "鐘守", Villager, "練習の刻でもないのに、鐘が鳴ったんだ。南岸に誰かいたか、聞いてきてくれないか。"),
                    Beat::scene(6, "南の音見台で待つと、谷を吹き抜ける風で綱が引かれた。誰も引いていないのに、北岸の鐘がひとつ鳴った。"),
                    Beat::scene(5, "余った綱を留めると、風が吹いても鐘は鳴らなくなった。南岸の人が緊急時に引けるよう、別の綱を張った。"),
                ],
                choices: None,
            },
            Story {
                title: "廃屋の蜂",
                beats: &[
                    Beat::speech(3, "石積みの職人", Worker, "旧居跡の蜂が、荷を運ぶ道まで出てきてるんだ。本格的に石を積む前に、巣を移せそうな場所を探してくれないか。"),
                    Beat::scene(4, "蜂が集まる花は、沢道を外れた日だまりにも咲いていた。作業道からも離れていて、空の巣箱を置けそうだ。"),
                    Beat::scene(2, "花のそばに置いた巣箱へ蜂が移った。石運びが再開した頃、診療所にも蜂蜜がひと瓶届いた。"),
                ],
                choices: None,
            },
            Story {
                title: "対岸の診察券",
                beats: &[
                    Beat::speech(2, "診療所の受付", Villager, "橋を渡り切ってから、診察券を置いてきたって気づく人がいるんだ。次に来るときも、同じ目に遭わせたくなくてね。"),
                    Beat::scene(7, "往診札を掛ければ橋守にも見える。道中まで迎えに行ける人に、渡る前に持ち物を確かめてもらうこともできそうだ。"),
                    Beat::speech(2, "診療所の受付", Villager, "次の診察日が近いね。往診札で知らせるか、橋の手前で持ち物を一緒に確認するか、どっちから試そう？"),
                ],
                choices: Some([
                    Ending { label: "往診の印を掛ける", text: "往診札が出た日には、医者が薬を持って橋を渡った。患者は自分で歩ける日にだけ診療所へ通うことになった。" },
                    Ending { label: "付き添いの当番を頼む", text: "当番が診察券を一緒に確かめ、帰りに古い沢の話を聞いた。" },
                ]),
            },
        ],
    },
    Plan {
        name: "灰石の市",
        profile: "市の組合が露店の番地を貸す。拾得物の引渡しには市役の立会いがいる。",
        terrain: Landscape::River,
        entry: (116, 173),
        places: [
            Place { name: "時計塔広場", at: (158, 87), art: Some(Art::Clock), ground: Landscape::Square, detail: "置き時計を持たない商人に開市の時刻を知らせる。" },
            Place { name: "鑑定と拾得所", at: (78, 93), art: Some(Art::Inn), ground: Landscape::Stone, detail: "遺族の申し出と売り手の帳面を別々に扱う。" },
            Place { name: "南の河原市", at: (153, 132), art: Some(Art::Stalls), ground: Landscape::Shore, detail: "日暮れまで日用品と使い古した工具を売る。" },
            Place { name: "研ぎ直し長屋", at: (162, 37), art: Some(Art::Cottage), ground: Landscape::Stone, detail: "研ぎ屋と荷運び人が仕事を受ける長屋。" },
            Place { name: "塔の歯止め足場", at: (181, 82), art: None, ground: Landscape::Stone, detail: "開市の鐘を調節する歯止めに手が届く。" },
            Place { name: "貸し露店", at: (196, 119), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "行商が荷を取りに行く間、店を預けようとしている。" },
            Place { name: "荷運び人の休憩場", at: (100, 129), art: None, ground: Landscape::Shore, detail: "河原のぬかるみに様々な靴跡が残る。" },
            Place { name: "明かりの試し棚", at: (214, 75), art: Some(Art::Stalls), ground: Landscape::Stone, detail: "夜の露店へ貸せる灯りを並べて試す。" },
        ],
        stories: [
            Story {
                title: "刻印を研ぎ落とす前に",
                beats: &[
                    Beat::speech(1, "市の係", Worker, "研ぎ直しを待ってる工具に、遺族が捜してる印があるって話だ。研ぎ屋が手を付ける前に、確かめてきてくれないか。"),
                    Beat::speech(6, "荷運び人", Traveler, "ほら、去年の届け控え。形はこの工具と同じだね。印があるとしたら、刃じゃなくて握りのほうだよ。"),
                    Beat::speech(3, "研ぎ屋", Worker, "柄を外したよ。握りの裏に印が残ってる。欠けてるところまで、あの家族が話してたとおりだ。"),
                    Beat::scene(1, "市の係が呼んだ家族は、印に指を当ててしばらく工具を眺めた。やがて、また使ってくれるなら貸したいと申し出た。"),
                ],
                choices: Some([
                    Ending { label: "遺族のもとへ返す", text: "工具は家の台所棚に帰った。市役は古い道具を研ぐ前の照合を始めた。" },
                    Ending { label: "研ぎ屋に貸す", text: "研ぎ屋は遺族の前で工具を手入れした。以後、家の印がある修理品には、どの家から預かったか札を添えるようになった。" },
                ]),
            },
            Story {
                title: "鐘楼の遅刻",
                beats: &[
                    Beat::speech(0, "市場の時計番", Worker, "昨日と今日じゃ、市を開ける鐘も鍛冶を終える鐘も、鳴った刻が違うんだよ。塔の仕掛けを見てきてくれないか。"),
                    Beat::scene(4, "足場からのぞくと、歯止めがときどき一歯ぶん戻ってしまう。今は使っていないもう一方の留め具は、同じ形をしている。"),
                    Beat::scene(0, "留め具を替え、ひと刻待って鐘を試した。鐘が鳴ると、鍛冶屋と露店がそろって店仕舞いの支度を始めた。"),
                ],
                choices: None,
            },
            Story {
                title: "一日の店主",
                beats: &[
                    Beat::speech(5, "皿売り", Worker, "荷を取りに行く間、皿を売る店を見ててもらえる？　今日中に売りたいけど、取り置きを約束した客も来るんだ。"),
                    Beat::scene(2, "河原の買い付け人は、皿をまとめて買いたいと言う。皿を取り置いた食堂の人が来られるのは夕方だ。"),
                    Beat::scene(5, "棚に残った皿はあとわずか。まとめて売るか、食堂の人が来るまで取り置くか、店主は判断を任せていった。"),
                ],
                choices: Some([
                    Ending { label: "値を下げてまとめ売りする", text: "棚の皿がすべて売れた。店主は明るいうちに、次の仕入れへ出発できた。" },
                    Ending { label: "食堂の分を取り置く", text: "夕方、食堂の料理番が取り置いた皿を一枚ずつ確かめた。次の市にも買いに来るという。" },
                ]),
            },
            Story {
                title: "尋ね物は靴一足",
                beats: &[
                    Beat::speech(1, "落とし物係", Worker, "片方だけの靴が二つ届いたんだ。持ち主が探してる靴の説明は、二人とも全然違うんだよ。"),
                    Beat::scene(6, "ぬかるみには片方だけ深い足跡が残っていた。靴底を見ると、片方に体重がかかっても歩きやすいよう、縫い目が補強されている。"),
                    Beat::speech(3, "元靴職人", Worker, "この二つなら、私が直して一足にしたよ。二人が持って帰るときに、互いの靴を片方ずつ取り違えたんだ。"),
                ],
                choices: None,
            },
            Story {
                title: "鐘が止まった夕市",
                beats: &[
                    Beat::speech(0, "露店の店主", Worker, "塔を直してる間は、夕市を閉めるって言うんだよ。日暮れにならないと買いに来られない人もいるのにね。"),
                    Beat::scene(7, "試しにともした灯は大通りを照らすが、路地の奥までは届かない。河原の空き区画は、日が落ちても空の明るさが残る。"),
                    Beat::speech(2, "市の世話役", Worker, "河原なら夕方まで明るい。今の場所を使うなら、店先に灯が要るね。河原へ移すか、研ぎ屋から灯を借りるか、決めよう。"),
                ],
                choices: Some([
                    Ending { label: "河原に夕市を出す", text: "日が落ちるまで河原に店が並び、塔の工事は続いた。" },
                    Ending { label: "灯りを借りて続ける", text: "研ぎ屋から借りた灯を露店の前に並べ、夕市はいつもの場所で店じまいまで開いた。" },
                ]),
            },
        ],
    },
];

use super::{Art, Beat, Ending, Landscape, Place, Plan, Story};

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
                    Beat { place: 0, text: "警備隊が避難者の名前を欲しがってる。小屋の夜番に、何を届けていいか聞いてくれ。" },
                    Beat { place: 2, text: "名札のない薪束にも、夜番の刻み目がある。夜番は「働いたことなら、話していい」と言った。" },
                    Beat { place: 4, text: "寄合が薪札を受け取った。本人は配給にも名前を届けていいと付け加える。" },
                ],
                choices: Some([
                    Ending { label: "薪札だけ出す", text: "名を伏せたまま夜番を続ける人に、炉の席が用意された。" },
                    Ending { label: "本人の名も届ける", text: "夜番の名が配給帳に加わった。本人が最初の薪当番として炉に座る。" },
                ]),
            },
            Story {
                title: "父の手袋",
                beats: &[
                    Beat { place: 1, text: "これ、あんたのお父さんの手袋だよ。もう片方は、誰かに繕ってもらったはずだ。" },
                    Beat { place: 6, text: "縫い目にアレンの字がある。隣人は雪遊びへ連れ出す前、父が子どもの手に合わせて詰めたと語る。" },
                    Beat { place: 1, text: "手袋をはめると、剣を握る手よりずっと小さかった頃のことを思い出した。" },
                ],
                choices: None,
            },
            Story {
                title: "煙を追う",
                beats: &[
                    Beat { place: 2, text: "煙が床から逆に入るんだ。小屋で寝ていたみんなが、外へ出たままだよ。" },
                    Beat { place: 3, text: "薪割り場の職人と煙突を見上げる。風のたび、継ぎ目の折れた曲がり管が揺れる。" },
                    Beat { place: 2, text: "風が弱まった。煙突を直す間、暖を取る場所を決めよう。" },
                ],
                choices: Some([
                    Ending { label: "炉を止めて管を直す", text: "夜明けに煙が屋根からまっすぐ上がる。小屋の人々が寝床へ戻った。" },
                    Ending { label: "薪割り場へ炉を移す", text: "小屋の人々は林縁で温まった。職人は翌日の明るい時間に管を直す。" },
                ]),
            },
            Story {
                title: "薪山の番人",
                beats: &[
                    Beat { place: 3, text: "毎晩、束が一本ぶん消える。獣に薪なんか要るとは思えないけどね。" },
                    Beat { place: 5, text: "雪隠れの子が、小さな火で行き倒れの手を温めている。日のあるうちなら小屋まで運べる。" },
                    Beat { place: 2, text: "空き寝台に毛布を広げた。行き倒れが目を覚まし、子に返す毛布を探していた。" },
                ],
                choices: None,
            },
            Story {
                title: "灰の中の晩餐",
                beats: &[
                    Beat { place: 7, text: "先代の鍋で宴を開きたい。焦げ色だけじゃ、どっちの香草を入れたか分からないんだ。" },
                    Beat { place: 6, text: "繕い台の隣人は苦い香草を、冬越しの古参は甘い根を覚えている。以前は二つの鍋が並んだという。" },
                    Beat { place: 7, text: "今夜は鍋が一つ。炊事番が木匙を持って待っている。" },
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
                    Beat { place: 0, text: "油があと一缶だ。貨物船の灯を守るか、救難の油を残すか。信号台で沖を見てくれ。" },
                    Beat { place: 3, text: "沖の貨物船は二隻。内側に霧が寄っている。信号台の旗なら洋上の一隻から油を借りられそうだ。" },
                    Beat { place: 0, text: "灯台守が今夜の火を決める。" },
                ],
                choices: Some([
                    Ending { label: "夜明けまで火を絞る", text: "遅い貨物は沖で待った。救難艇は取り置いた油で小さな船を曳いて戻る。" },
                    Ending { label: "沖から油を借りる", text: "貨物船が予定通り着き、艇庫にも油が届いた。船頭の当番表に貸し分を記した。" },
                ]),
            },
            Story {
                title: "折り返さない舟",
                beats: &[
                    Beat { place: 2, text: "帳面にない帰り舟を、昔ここへ曳いた。浜の低い場所なら舵板が残っているかもしれん。" },
                    Beat { place: 4, text: "舵に家の船名が残る。浜の網を持つ証人は「本人は自分の足で丘を越えた」と話した。" },
                    Beat { place: 1, text: "家族が舵を受け取る。海に沈んだと思っていた人には、まだ陸の足取りがある。" },
                ],
                choices: None,
            },
            Story {
                title: "霧笛の届く家",
                beats: &[
                    Beat { place: 0, text: "霧笛を鳴らすと三軒から戸を叩かれる。沖には届いてほしいんだがね。" },
                    Beat { place: 5, text: "石壁の端だけ、音が住宅側へ跳ね返る。崖上では沖からの返りのほうが先に聞こえた。" },
                    Beat { place: 0, text: "笛は沖を向いたまま、壁に木の覆いを足した。家の戸が開き、港の鐘がふつうに聞こえた。" },
                ],
                choices: None,
            },
            Story {
                title: "潮だまりの客",
                beats: &[
                    Beat { place: 2, text: "潮が引いてる。あの潮だまりで何かが動いてるよ。艇庫の子が浜を指した。" },
                    Beat { place: 6, text: "捨て網の下で仔海獣が藻を押している。外へつながる細い澪まで、桶で水を通してやる。" },
                    Beat { place: 2, text: "仔が外海へ泳ぎ、艇庫の前で見ていた子が使い終えた網の回収を始めた。" },
                ],
                choices: None,
            },
            Story {
                title: "まだ借りられる桟橋",
                beats: &[
                    Beat { place: 1, text: "弔いの船に桟橋を貸したい。商船も同じ時刻に同じ場所を使うという。" },
                    Beat { place: 7, text: "通船用の杭は小舟を留められる。使うなら、荷揚げの小舟か家族の船の片方だ。" },
                    Beat { place: 1, text: "二組が浜でこちらの返事を待つ。" },
                ],
                choices: Some([
                    Ending { label: "家族の船を外桟橋へ", text: "風の陰で見送りが済んだ。荷揚げは元の岸壁で始まる。" },
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
                    Beat { place: 1, text: "護送代を増やすと、うちは店を畳むことになる。櫓で新しい道を見てほしい。" },
                    Beat { place: 2, text: "峠の迂回図は牧柵を横切る。旧荷道なら昼の間は見通しがよく、放牧地からも離れている。" },
                    Beat { place: 1, text: "市の組合が、砦長に出す条件を決める。" },
                ],
                choices: Some([
                    Ending { label: "旧荷道を昼だけ開く", text: "巡回が昼の旧荷道に移り、柵沿いの抜け道が開いた。" },
                    Ending { label: "護送料を共同払いにする", text: "小商人の荷を一台にまとめ、護送の出発日を揃えた。旧荷道は閉じたままだ。" },
                ]),
            },
            Story {
                title: "鐘が二度鳴る",
                beats: &[
                    Beat { place: 5, text: "見張りが一度しか打っていないのに、北の荒野で二度鳴った。荷運び人が逃げたよ。" },
                    Beat { place: 4, text: "曲がり角の岩肌へ声を投げると、少し遅れて鐘の方から音が返る。岩陰で荷運び人が息をひそめていた。" },
                    Beat { place: 5, text: "反響が返る位置を番兵が札に描いた。荷運び人は鐘の下で荷縄を結び直す。" },
                ],
                choices: None,
            },
            Story {
                title: "柵の内側で育つ",
                beats: &[
                    Beat { place: 0, text: "荷馬の仔が震えてる。病気かもしれないから、門は通せない。" },
                    Beat { place: 3, text: "柵の獣医役が、母馬を見せると仔が落ち着くのを確かめた。長旅で冷えているらしい。" },
                    Beat { place: 0, text: "門番が一晩の滞在札を発行した。親子の馬に水と乾いた藁が用意される。" },
                ],
                choices: None,
            },
            Story {
                title: "帳面を持った賊",
                beats: &[
                    Beat { place: 6, text: "取り返した荷には市の印がない。荷主も襲った者も、前に運んだ代金を求めている。" },
                    Beat { place: 1, text: "市の支払控えには荷主の代理名がある。襲った者は、この代理から支払いを待たされていた。" },
                    Beat { place: 6, text: "荷改め場で荷を返せる。代金の扱いを決めよう。" },
                ],
                choices: Some([
                    Ending { label: "公開審理に回す", text: "代理人も呼ばれ、未払いを他の運び手が申し立てられるようになった。" },
                    Ending { label: "返還と分割払いを結ぶ", text: "荷はすぐ持ち主へ戻り、運び手は市役の立会いで最初の支払いを受けた。" },
                ]),
            },
            Story {
                title: "最後の矢羽",
                beats: &[
                    Beat { place: 7, text: "引退前に一戦だけ。的に当てる腕と、風を読んで立つ位置。若い門兵にどちらを見せようか。" },
                    Beat { place: 2, text: "櫓から見ると木柵の外と中で旗の動きが違う。若い門兵が自分の矢を持って待っている。" },
                    Beat { place: 7, text: "古参の弓兵が試合を始める。" },
                ],
                choices: Some([
                    Ending { label: "柵内の的を射る", text: "若い門兵は、矢羽と弦を同じ角度に揃える手つきを教わった。" },
                    Ending { label: "外で風を読む", text: "遠い布のはためきで追い風を読む。門兵は翌日から荷道の見張りに使う。" },
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
                    Beat { place: 0, text: "上流と下流の目盛りが合わん。水を下げたい家も魚を守りたい家も、相手を疑っている。" },
                    Beat { place: 2, text: "水門の目盛りは閉めた時の高さを指す。今日は上流だけ明らかに高い。" },
                    Beat { place: 3, text: "産卵棚は水を下げると干上がる。流れが脇へ抜けた跡を下流の漁師が指した。" },
                    Beat { place: 4, text: "舟大工が古い川筋を板に描く。浚渫の土を除けば二つの水位が両立する。" },
                    Beat { place: 0, text: "葦刈りの水路を浅くし、魚棚の水を残した。川沿いに新しく渡れる板道を当番が開いた。" },
                ],
                choices: None,
            },
            Story {
                title: "渡し舟の輪番",
                beats: &[
                    Beat { place: 1, text: "櫂を握る手を痛めた。夜の乗客が何人いるか、寄合で聞いてくれ。" },
                    Beat { place: 0, text: "夜便を使うのは三人。寄合に、浅い舟なら使えるという漁師がいる。" },
                    Beat { place: 1, text: "舟の縁に片手で扱える手すりも付けられそうだ。" },
                ],
                choices: Some([
                    Ending { label: "漁師に夜番を頼む", text: "渡し守は手を休める。漁師は夜便を受け持ち、代わりに昼の板道当番を減らしてもらった。" },
                    Ending { label: "片手用の舟を試す", text: "最初の三人を無事に運んだ。渡し守は翌日も無理のない明るい便を受け持つ。" },
                ]),
            },
            Story {
                title: "杭に咲く花",
                beats: &[
                    Beat { place: 5, text: "板を抜く前に、この花が何を指すか聞いてきて。工事の人が首をかしげた。" },
                    Beat { place: 4, text: "舟大工の古い図と花の並びが重なる。水没した家の台所と戸口だけを花で示していた。" },
                    Beat { place: 5, text: "古い杭の場所が、新しい板へと写された。花を供えていた人は今度は上に花を置く。" },
                ],
                choices: None,
            },
            Story {
                title: "籠をさらう風",
                beats: &[
                    Beat { place: 6, text: "風が強い日に籠が落ちる。編んだ子は、自分の積み方のせいだと思ってる。" },
                    Beat { place: 3, text: "産卵棚の網で籠を拾う。濡れた底を重ねると、広い口が風を抱き込む向きが分かる。" },
                    Beat { place: 6, text: "底に小石を置いて並べ替える。次の突風が吹いても、籠は棚を離れなかった。" },
                ],
                choices: None,
            },
            Story {
                title: "板の下の産声",
                beats: &[
                    Beat { place: 0, text: "夜の板道で何かが鳴く。水の上からは影しか見えない。" },
                    Beat { place: 7, text: "舟を低く入れると、増水で取り残された仔獣がいる。親の足跡は下流から続いていた。" },
                    Beat { place: 0, text: "まず住民に別の板道へ回ってもらおう。" },
                ],
                choices: Some([
                    Ending { label: "板道を閉じて親を待つ", text: "足音の絶えた板道を親がくぐる。仔を連れて水面へ戻った。" },
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
                    Beat { place: 3, text: "砦へ出す麦が注文より重い。風車の計り場で控えを読んでみてくれ。" },
                    Beat { place: 0, text: "風車の帳面には黄色の荷札は来季の種とある。秤場の車にはその袋が二つ混じっていた。" },
                    Beat { place: 1, text: "種は倉へ戻した。補給係が不足する分をどう埋めるか尋ねる。" },
                ],
                choices: Some([
                    Ending { label: "古い粉を送る", text: "砦は日持ちする菓子を焼く粉として引き取った。種蔵の貸出札が全戸に配られる。" },
                    Ending { label: "近隣の里から麦を借りる", text: "次の収穫で返す条件を結び、今季の粉を里の食卓にも残した。" },
                ]),
            },
            Story {
                title: "霜よけの夜",
                beats: &[
                    Beat { place: 2, text: "南の畑は真っ白だ。でも石垣の端だけ、苗が枯れていない。" },
                    Beat { place: 7, text: "冷えた空気が石垣の切れ目から流れ出す。閉じた側に霜がたまった。" },
                    Beat { place: 2, text: "次の夜も冷え込む。農家が苗のそばで待つ。" },
                ],
                choices: Some([
                    Ending { label: "石垣の一部を開く", text: "冷気が溜まらず、開いた側の苗が翌朝も緑だった。" },
                    Ending { label: "苗を藁で覆う", text: "藁に霜が積もった。下の苗が残り、列ごとの覆い方が試験畑にも伝わる。" },
                ]),
            },
            Story {
                title: "挽きたて一等賞",
                beats: &[
                    Beat { place: 5, text: "焼き菓子を二種類試したい。挽き方の違う粉を風車から分けてもらえないかな。" },
                    Beat { place: 0, text: "挽き手が粗挽きと細挽きの粉を出す。粗い粉は保存菓子、細かい粉は柔らかな生地になる。" },
                    Beat { place: 5, text: "試食台に買い手が集まった。" },
                ],
                choices: Some([
                    Ending { label: "粗挽きの保存菓子を焼く", text: "護送団の注文が入った。袋に日持ちする日数の札が付いた。" },
                    Ending { label: "細挽きの菓子を焼く", text: "宿の主人が朝食用に予約した。焼き手は次の試食日を決めた。" },
                ]),
            },
            Story {
                title: "止まらない風車",
                beats: &[
                    Beat { place: 0, text: "風が止んでも羽根の軸が鳴る。誰も梯子を上がろうとしない。" },
                    Beat { place: 6, text: "木の制動具の端が擦り切れていた。足場の脇から手挽き石が取り出せる。" },
                    Beat { place: 0, text: "交代で手挽き石を回した。粉を出し続ける間に、親方が新しい制動具を削った。" },
                ],
                choices: None,
            },
            Story {
                title: "畑の貸し手",
                beats: &[
                    Beat { place: 4, text: "秋に借りた畑の種まで、畑主にしか貸さないと言われたんだ。" },
                    Beat { place: 1, text: "種蔵の札には一年目の試験用の余りがある。倉番は借り手が耕すところをまだ見ていない。" },
                    Beat { place: 4, text: "畝の手入れを見て、種蔵番が二つの貸し方を提案した。" },
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
            Place { name: "石工場と拓本台", at: (87, 90), art: Some(Art::Inn), ground: Landscape::Stone, detail: "石を叩いた音と木槌の音が少し違う。" },
            Place { name: "暗渠入口", at: (145, 32), art: Some(Art::Sluice), ground: Landscape::Stone, detail: "上から水が滴る。崩れた区画を避ける石目が残る。" },
            Place { name: "転用石置場", at: (160, 153), art: None, ground: Landscape::Stone, detail: "旧村の石や、次の家を待つ敷石が積まれる。" },
            Place { name: "水番所", at: (172, 106), art: Some(Art::Cottage), ground: Landscape::Stone, detail: "水番が濁りと雨漏りの通報を受ける。" },
            Place { name: "段差のある家", at: (206, 77), art: Some(Art::Cottage), ground: Landscape::Stone, detail: "大きな敷石が元の家から使い継がれている。" },
            Place { name: "雨染みの裏路地", at: (199, 123), art: None, ground: Landscape::Stone, detail: "濡れているのは片方の壁だけだ。" },
            Place { name: "拓本を待つ古い壁", at: (131, 137), art: None, ground: Landscape::Stone, detail: "壁の裏に、隣家が探す戸口印がのぞく。" },
        ],
        stories: [
            Story {
                title: "井戸底の戸口印",
                beats: &[
                    Beat { place: 0, text: "暗渠を閉める前に、敷石の戸口印を写してくれないか。煤けた旧村から移った家のだ。" },
                    Beat { place: 2, text: "足元の石目を頼りに崩れを避ける。流れの低い床に、二軒分の戸口印が並んでいた。" },
                    Beat { place: 1, text: "石工場から紙と搬出の綱が出せる。暗渠を塞ぐ時刻が迫る。" },
                ],
                choices: Some([
                    Ending { label: "拓本を取り水路を塞ぐ", text: "井戸が早く戻った。二軒の遺族は写した印を置場に掛けた。" },
                    Ending { label: "銘のある石を運び出す", text: "石は遺族の手に戻った。水番は今夜も井戸を止め、別の水を配った。" },
                ]),
            },
            Story {
                title: "玄関の一段",
                beats: &[
                    Beat { place: 5, text: "敷石の一段が高くて外へ出られない。でもこの石は祖父が運んできたんだ。" },
                    Beat { place: 1, text: "石工が低い敷石と木の斜路を持ってきた。戸の開く位置で試し、どちらでも出入りできると確かめる。" },
                    Beat { place: 5, text: "古い石を脇に残すか、通りに張り出さずに越えられる形にしよう。" },
                ],
                choices: Some([
                    Ending { label: "石の横に斜路を足す", text: "表の段と家の石が残り、家人は斜路を使って毎朝井戸まで出る。" },
                    Ending { label: "平らな敷石に替える", text: "広い路地が戻った。古い石は中庭で休む腰掛けになった。" },
                ]),
            },
            Story {
                title: "石の雨漏り",
                beats: &[
                    Beat { place: 4, text: "井戸より先に、隣の壁の染みを見て。雨ごとに同じ所だけ濡れるんだ。" },
                    Beat { place: 6, text: "壁の上をたどると、隣の屋根樋が短く、こちらの石継ぎに水を流している。" },
                    Beat { place: 4, text: "二軒が余った樋を足した。落ちる水を桶に溜めると、路地の掃除にも使えた。" },
                ],
                choices: None,
            },
            Story {
                title: "よその家の石",
                beats: &[
                    Beat { place: 1, text: "置場に戻したはずの家印が、新築の壁から見えるんだ。見に行ってくれないか。" },
                    Beat { place: 7, text: "壁の石についた運搬札は、渡された時には印が下向きだったことを示す。買った家は由来を知らなかった。" },
                    Beat { place: 3, text: "転用石置場で両家が会った。石を刻んだ家は、次の改築までは印を見せてもらえることになった。" },
                ],
                choices: None,
            },
            Story {
                title: "石工の音当て",
                beats: &[
                    Beat { place: 1, text: "石を叩いて試験するんだ。割れる音が耳で分からない。" },
                    Beat { place: 3, text: "置場で同じ材を叩くと、割れのある側だけ響きが短い。端を叩いた時と中央との違いを覚えた。" },
                    Beat { place: 1, text: "弟子は響きの短い石を除かず、割れの反対側を切り出す方法を試験で示した。" },
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
                    Beat { place: 1, text: "次の馬車では谷守の診療が終わってしまう。昔の飛脚道なら間に合うか見てきて。" },
                    Beat { place: 4, text: "番屋に残る折返し表には、峠の手前で徒歩に切り替えた便がある。夜の番屋なら替え馬も頼める。" },
                    Beat { place: 3, text: "薬包を持つ係が待っている。どちらの便を試そうか。" },
                ],
                choices: Some([
                    Ending { label: "番屋に夜の馬を置く", text: "峠まで夜便が走り、薬と急ぎの便りを運んだ。宿は当番の交代日を増やした。" },
                    Ending { label: "薬だけ徒歩便で送る", text: "薬の小包が診療の前に着いた。馬方の夜番は増やさずに済んだ。" },
                ]),
            },
            Story {
                title: "火が見えない夜",
                beats: &[
                    Beat { place: 0, text: "飛脚が火を見ずに宿を通り過ぎて、峠で夜明けを待ったそうだ。" },
                    Beat { place: 5, text: "尾根から見ると、強風のときだけ屋根の覆いが火の前に倒れている。" },
                    Beat { place: 0, text: "風を横に逃がす覆いへ替えた。次の夜、尾根を越えた飛脚がまっすぐ宿へ来た。" },
                ],
                choices: None,
            },
            Story {
                title: "名前を持たない手紙",
                beats: &[
                    Beat { place: 2, text: "送り先の名だけ取れた手紙がある。差出人は口外しないよう、端に印をつけている。" },
                    Beat { place: 0, text: "旅籠の古い客が、封の折り方を見て谷へ移った人に当たりをつける。人前で名を呼ばない約束を受ける。" },
                    Beat { place: 2, text: "受取人の了解を得て宿主が封のまま渡した。返書には名の代わりに伝言の札が付く。" },
                ],
                choices: None,
            },
            Story {
                title: "替え馬の休み",
                beats: &[
                    Beat { place: 1, text: "遠出ができなくなった馬に買い手がついた。でもこれまで峠で何度も助けてもらったんだ。" },
                    Beat { place: 7, text: "放牧地の世話役は冬場だけ見られると申し出る。谷の薬便なら短い平道にも仕事がある。" },
                    Beat { place: 1, text: "替え馬の厩で、馬の今後を決める。" },
                ],
                choices: Some([
                    Ending { label: "近い薬便を手伝う", text: "小さな荷だけを背負うようになり、馬方が毎日歩き方を確かめる。" },
                    Ending { label: "谷の放牧地に預ける", text: "忙しかった馬が群れと歩く。厩番は餌を持って会いに行く。" },
                ]),
            },
            Story {
                title: "足音のすき間",
                beats: &[
                    Beat { place: 0, text: "夜に壁の中を歩く人がいるって、宿の子が灯火当番を怖がっている。" },
                    Beat { place: 6, text: "風が吹くと裏口の床下で空箱が動く。荷縄を切って横に穴を覗くと、箱に足はない。" },
                    Beat { place: 0, text: "夕食のあとに荷箱を転がしてみせた。子どもは屋根の火を見に行くと言い出した。" },
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
                    Beat { place: 3, text: "詰番板の二つの日付で同じ徽章を使っている。これじゃ遺族も衛兵も話せない。" },
                    Beat { place: 5, text: "旗の継ぎ布に付いた補修年は戦役後。旧い色の上へ、今の部隊の胸章を縫い付けた。" },
                    Beat { place: 0, text: "詰所に引換印と旗の記録が揃う。部隊が二度戻った扱いをどう改めるか。" },
                ],
                choices: Some([
                    Ending { label: "遺族と門衛に一緒に話す", text: "両者が徽章の継承を同じ席で聞いた。死者の名と現役の隊を分けて記す。" },
                    Ending { label: "まず遺族へ経緯を知らせる", text: "家族が名札を受け取り、後日、同意した名を詰所で照合する日を決めた。" },
                ]),
            },
            Story {
                title: "空き家の鍵",
                beats: &[
                    Beat { place: 2, text: "水漏れの家の鍵を持つ家族が二組いる。どちらも先週ここへ戻ったばかりだ。" },
                    Beat { place: 6, text: "壁は補修できるが濡れた部屋は一つ。隣にも長く空いた一室がある。" },
                    Beat { place: 2, text: "住宅街の代表と二家族が待っている。" },
                ],
                choices: Some([
                    Ending { label: "修理して同居する", text: "二家族は乾いた部屋を先に使い、雨を止める作業を分担した。" },
                    Ending { label: "対面の空室も開ける", text: "街区会が第二の鍵を開けた。一家は修理中も小さい部屋を一つ持てる。" },
                ]),
            },
            Story {
                title: "外濠の手すり",
                beats: &[
                    Beat { place: 4, text: "外濠の近道で子が滑り落ちた。この道は古参の見回りにもいるんだ。" },
                    Beat { place: 0, text: "門の遠回りはできるが、待合から住宅までの帰り道が長い。濠の低い側は杭が打てる。" },
                    Beat { place: 4, text: "住宅の職人が濠沿いに手すりを入れた。子は回り道を覚え、古参は途中で休める。" },
                ],
                choices: None,
            },
            Story {
                title: "旗の裏の色",
                beats: &[
                    Beat { place: 5, text: "旗を裏返したら、父が入った隊とも今の隊とも違う色が見えた。" },
                    Beat { place: 3, text: "旧詰番板の横に、両方の部隊色を覚えている遺族がいる。継ぎ目は直した日を示す印だった。" },
                    Beat { place: 5, text: "旗を掲げ直す時が来た。" },
                ],
                choices: Some([
                    Ending { label: "古い配色へ繕う", text: "元の隊の旗が家族の行列を迎えた。今の旗は見張り詰所用に別に仕立てる。" },
                    Ending { label: "継ぎ布ごと掲げる", text: "古い色と今の色が風に並んだ。門衛は掛け替えの記録を閲覧所に寄せた。" },
                ]),
            },
            Story {
                title: "配給所の朝番",
                beats: &[
                    Beat { place: 1, text: "朝の列に夜警と具合の悪い人が来られない。受取札が余っても配給が届かないんだ。" },
                    Beat { place: 7, text: "夕方の窓口は見回りの戻る路地にある。朝に来られる世話役は、代理の札を預かれる。" },
                    Beat { place: 1, text: "配給係は二つの時間の帳面を開いて待つ。" },
                ],
                choices: Some([
                    Ending { label: "受取札を代理で預ける", text: "朝の列で世話役が二軒分を受け取った。病人のところへ配給が届く。" },
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
                    Beat { place: 0, text: "橋に重い荷を通すと縄が持たない。でも北岸の工事も南岸の通院も急いでいる。" },
                    Beat { place: 1, text: "北岸で量ると一台には重すぎる。小分けなら人が背負える重さになる。" },
                    Beat { place: 3, text: "沢道は低い水位の日だけ軽い荷が通る。石積み屋は縄を先に直す案も出した。" },
                    Beat { place: 0, text: "橋守が鍵を持って立っている。" },
                ],
                choices: Some([
                    Ending { label: "軽い荷を沢道へ回す", text: "吊橋を通院に空け、沢道にも荷を分けて渡せる足場が開いた。" },
                    Ending { label: "先に橋縄を取り替える", text: "石積み屋が昼までに縄を替えた。渡れなかった人には診療所から往診が出た。" },
                ]),
            },
            Story {
                title: "沢の子守歌",
                beats: &[
                    Beat { place: 2, text: "ここに移る前の子守歌を聞きたい。谷底で聞いた気がするんだ。" },
                    Beat { place: 4, text: "旧居の炉石に手を当てると短い節が響いた。沢の水音で後半が途切れる。" },
                    Beat { place: 1, text: "石積みの作業歌が、ちょうど欠けていた後半から始まった。歌い手に来てもらう。" },
                    Beat { place: 2, text: "診療所の患者が二つの節を合わせて小声で歌った。付き添いの子も続きを覚えた。" },
                ],
                choices: None,
            },
            Story {
                title: "鐘を鳴らす人",
                beats: &[
                    Beat { place: 5, text: "練習していない時刻に鐘が鳴る。南岸に人がいたか聞いてみてくれ。" },
                    Beat { place: 6, text: "南の音見台では、谷を抜ける風が綱を引くときだけ音が立つ。鐘守がいなくても起きる。" },
                    Beat { place: 5, text: "余った綱を留めて空鳴りを防いだ。南の使い手が緊急時に引ける補助綱もつけた。" },
                ],
                choices: None,
            },
            Story {
                title: "廃屋の蜂",
                beats: &[
                    Beat { place: 3, text: "旧居跡の蜂が作業道に出る。石積みが始まる前に、巣を移せるところを探してくれ。" },
                    Beat { place: 4, text: "谷底で蜂が集まる花は、沢道から外れた日向にも咲いている。空の巣箱なら持ってこられる。" },
                    Beat { place: 2, text: "巣箱を花の近くに置いて移るのを待った。石運びが再開し、診療所に蜜が少し届く。" },
                ],
                choices: None,
            },
            Story {
                title: "対岸の診察券",
                beats: &[
                    Beat { place: 2, text: "橋を渡ってから忘れ物を思い出す人がいる。次の診察も同じことになりそうだ。" },
                    Beat { place: 7, text: "往診札は橋守に見える。道中で迎えられる人に、持ち物を一緒に確かめてもらう手もある。" },
                    Beat { place: 2, text: "次の診察日を前に、どちらから試すか決める。" },
                ],
                choices: Some([
                    Ending { label: "往診の印を掛ける", text: "札の出た日には薬と医者が橋を渡る。患者は歩ける日にだけ来院する。" },
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
                    Beat { place: 1, text: "研ぎ直しを待つ工具に、遺族が捜している印があるらしい。削る前に確かめてくれ。" },
                    Beat { place: 6, text: "荷運び人が去年の届け控えを出す。同じ形の工具だが、元の印は握る側にあったという。" },
                    Beat { place: 3, text: "研ぎ屋が工具を外して握りの裏を見せる。印が欠けたところまで、遺族の説明と合った。" },
                    Beat { place: 1, text: "市役が遺族を呼んだ。遺族は使ってくれるなら貸してもいいと話す。" },
                ],
                choices: Some([
                    Ending { label: "遺族のもとへ返す", text: "工具は家の台所棚に帰った。市役は古い道具を研ぐ前の照合を始めた。" },
                    Ending { label: "研ぎ屋に貸す", text: "遺族が見守る前で工具を研ぎ直す。以後、市の修理品に家の印を添える。" },
                ]),
            },
            Story {
                title: "鐘楼の遅刻",
                beats: &[
                    Beat { place: 0, text: "開市と鍛冶の終業の時刻が、昨日と今日で違う。塔を見てくれ。" },
                    Beat { place: 4, text: "足場の歯止めが、ときどき一歯ぶん戻る。使われていない方の留め具は同じ形だ。" },
                    Beat { place: 0, text: "留め具を交換し、ひと刻を測って鐘を試した。鍛冶屋と露店が同じ音で身支度を始める。" },
                ],
                choices: None,
            },
            Story {
                title: "一日の店主",
                beats: &[
                    Beat { place: 5, text: "荷を取りに行く間、皿を売る店を頼めないか。売り切りたいけど取り置きの客も来る。" },
                    Beat { place: 2, text: "河原の買い付け人はまとめ買いができる。小さな食堂は夕方まで来られないと言っていた。" },
                    Beat { place: 5, text: "最後の皿を並べた。引き渡し条件はこちらに任されている。" },
                ],
                choices: Some([
                    Ending { label: "値を下げてまとめ売りする", text: "棚が片付き、行商人は次の仕入れにすぐ出かける。" },
                    Ending { label: "食堂の分を取り置く", text: "閉市時に食堂の料理番が皿を一枚ずつ確かめ、次の市でも買う約束をする。" },
                ]),
            },
            Story {
                title: "尋ね物は靴一足",
                beats: &[
                    Beat { place: 1, text: "落とし物の靴が二足とも片足だけ届いた。持ち主の話では全然違う靴なんだ。" },
                    Beat { place: 6, text: "ぬかるみに片方だけ深い足跡。靴底の縫いの向きは、歩き方の違いを補うよう作られている。" },
                    Beat { place: 3, text: "研ぎ屋の隣に住む元靴職人が、一組に作り直した靴だと覚えていた。二人が間違えて片方ずつ預かった経緯も分かった。" },
                ],
                choices: None,
            },
            Story {
                title: "鐘が止まった夕市",
                beats: &[
                    Beat { place: 0, text: "塔を直すあいだ夕方の市は閉じるって。日暮れしか来られない客もいる。" },
                    Beat { place: 7, text: "試し棚の灯は大通りなら届くが路地の奥は暗い。河原の空き区画は夕空が残る。" },
                    Beat { place: 2, text: "市の代表が残す店を決めたいという。" },
                ],
                choices: Some([
                    Ending { label: "河原に夕市を出す", text: "日が落ちるまで河原に店が並び、塔の工事は続いた。" },
                    Ending { label: "灯りを借りて続ける", text: "研ぎ屋の灯が露店の前へ増え、いつもの場所で閉市を迎えた。" },
                ]),
            },
        ],
    },
];

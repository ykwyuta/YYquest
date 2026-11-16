use super::{Art, Beat, Ending, Landscape, Place, Plan, Story};
use crate::dialogue::Speaker::*;

pub(super) static CELESTIA: [Plan; 10] = [
    Plan {
        name: "白帆港町",
        profile: "白い税関と灯台が巡礼船と穀物船を迎える港町。海難時の下船許可は灯台守が独自に出せる。",
        terrain: Landscape::Coast,
        entry: (154, 48),
        places: [
            Place { name: "白帆の税関桟橋", at: (154, 128), art: Some(Art::Landmark), ground: Landscape::Dock, detail: "青い荷揚げ機が巡礼船と貨物船の荷を降ろしている。" },
            Place { name: "東崖の石灯台", at: (238, 92), art: Some(Art::Lighthouse), ground: Landscape::Chalk, detail: "白い灯台が白い崖の上から入り江を照らしている。" },
            Place { name: "西の巡礼宿", at: (84, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "船待ちの巡礼者が身を寄せる白い石造りの宿。" },
            Place { name: "北の検札門", at: (154, 68), art: Some(Art::StoneGate), ground: Landscape::Square, detail: "聖堂街へ向かう旅人の通行証を確かめる門。" },
            Place { name: "崖下の救難階段", at: (212, 118), art: None, ground: Landscape::Chalk, detail: "浜辺から灯台へ直接上れる急な石段。" },
            Place { name: "磯の鐘縄置き場", at: (244, 128), art: None, ground: Landscape::Shore, detail: "荒天のとき沖の船に知らせる鐘の縄がつないである。" },
            Place { name: "待合の屋根裏廊下", at: (108, 86), art: None, ground: Landscape::Yard, detail: "崖下の入り江を見下ろせる静かな踊り場。" },
            Place { name: "税関裏の穀物倉", at: (178, 104), art: Some(Art::Cottage), ground: Landscape::Square, detail: "見張りの目が届く場所に小麦の袋が積まれている。" },
        ],
        stories: [
            Story {
                title: "札のない巡礼者",
                beats: &[
                    Beat::speech(2, "巡礼宿の女将", Villager, "記憶を失って名前も思い出せない旅人を保護してるんだけど、税関に『乗船控えにない密航者だ』と疑われていてね。"),
                    Beat::scene(4, "救難階段の途中で、灯台守が残した救助番号の札が見つかった。悪天候の日に保護した客の記録と人数がぴったり合う。"),
                    Beat::speech(1, "灯台守", Lightkeeper, "救助札の番号が揃ったなら密航じゃない。税関の聞き取りに付き添って証明するか、治療が終わるまで灯台の下の部屋に保護するか、どっちがいいだろう？"),
                ],
                choices: Some([
                    Ending { label: "税関の聴聞に付き添う", text: "救助札の控えが認められ、旅人は正式な滞在証を受け取った。" },
                    Ending { label: "灯台の部屋で保護する", text: "旅人は落ち着いた環境で休み、少しずつ思い出した家族の話を宿帳に記した。" },
                ]),
            },
            Story {
                title: "白崖の灯を磨く",
                beats: &[
                    Beat::speech(1, "灯台守", Lightkeeper, "灯台の反射板が潮で曇っちまった。でも板を外して磨いたら、今夜の船が入り江に入れなくなるんだよな。"),
                    Beat::scene(4, "浜辺の小屋で引退した鏡職人が、小さな予備の鏡を貸してくれた。一晩だけなら代わりを務められそうだ。"),
                    Beat::scene(1, "小鏡を仮設したあいだに、みんなで反射板を磨き上げた。夜明けの船はこれまでより遠くから灯を見つけた。"),
                ],
                choices: None,
            },
            Story {
                title: "荷札の絵葉書",
                beats: &[
                    Beat::speech(0, "税関の係", Worker, "荷物の山から宛名のない絵葉書が出たんだ。描いてある景色から場所を当てて、巡礼宿の誰かに届けてくれないか。"),
                    Beat::scene(6, "絵と同じ角度で崖の階段を見下ろすと、隠れた踊り場で家族の帰りを待つ子どもがいた。"),
                    Beat::scene(2, "子どもは絵葉書を受け取り、裏に書かれた再会の約束を読んで笑顔になった。"),
                ],
                choices: None,
            },
            Story {
                title: "海へ落ちた鐘縄",
                beats: &[
                    Beat::speech(1, "灯台守", Lightkeeper, "嵐に備えたいのに、救難鐘の縄が途中で切れて磯に絡まってしまったんだ。見てきてくれないか。"),
                    Beat::scene(5, "磯の岩に引っかかった縄をほどくと、まだ十分丈夫な長さがあった。新しい鐘縄に結んでも余りが出そうだ。"),
                    Beat::speech(0, "荷役人", Worker, "余った縄は荷役場に返しても助かるし、次の嵐に備えて救難階段のそばに予備として結んでおくのもいいね。どっちにしよう？"),
                ],
                choices: Some([
                    Ending { label: "荷役場へ返す", text: "縄は港の荷縛りに役立てられ、荷役人からお礼の油缶が届いた。" },
                    Ending { label: "階段に予備を結ぶ", text: "嵐の備えが万全になり、灯台守は安心して見張りを続けられるようになった。" },
                ]),
            },
            Story {
                title: "小麦を積む場所",
                beats: &[
                    Beat::speech(7, "穀物倉の番人", Worker, "大時化が来る前に小麦を屋根の下へ運びたいんだが、倉がもういっぱいでね。どこかに一時置きできる場所はないかな。"),
                    Beat::scene(2, "巡礼宿の一階は広くて乾いている。宿泊客に二階へ移ってもらえれば、麦袋を濡らさずに置けそうだ。"),
                    Beat::speech(7, "穀物倉の番人", Worker, "宿の一階を借りるか、見張りの届く税関の裏に積んで夜番を置くか。どっちなら安全に守れるだろう？"),
                ],
                choices: Some([
                    Ending { label: "巡礼宿の一階を借りる", text: "宿の客たちが二階へ荷物を運び上げ、麦は一粒も濡れずに嵐をやり過ごした。" },
                    Ending { label: "税関裏で夜番を立てる", text: "港の若い衆が交代で見張りを務め、嵐のあとには温かいパンが振る舞われた。" },
                ]),
            },
        ],
    },
    Plan {
        name: "クロノス聖堂街",
        profile: "双塔の大聖堂を中心に記録院と騎士団が並ぶ都。記憶の奉納は本来、市民が自ら選ぶ儀礼だった。",
        terrain: Landscape::Chalk,
        entry: (156, 174),
        places: [
            Place { name: "クロノス大聖堂", at: (156, 86), art: Some(Art::Landmark), ground: Landscape::Square, detail: "白い双塔がそびえる大聖堂。奥に記憶の奉納室がある。" },
            Place { name: "北の記録院", at: (156, 44), art: Some(Art::Inn), ground: Landscape::Square, detail: "戸籍や証文を保管する石造りの建物。" },
            Place { name: "東の騎士団詰所", at: (230, 96), art: Some(Art::Palisade), ground: Landscape::Square, detail: "街の治安を守る騎士たちが交代で詰めている。" },
            Place { name: "西の家族相談所", at: (82, 98), art: Some(Art::Cottage), ground: Landscape::Square, detail: "記憶を失った家族の相談を助祭が受け付けている。" },
            Place { name: "巡礼階段の踊り場", at: (126, 134), art: None, ground: Landscape::Chalk, detail: "大聖堂へ続く白い石段。脇に古い階段の跡がある。" },
            Place { name: "双塔の鐘楼", at: (186, 62), art: Some(Art::Clock), ground: Landscape::Square, detail: "二つの塔それぞれに大きな鐘が吊るされている。" },
            Place { name: "西の代筆机", at: (98, 122), art: Some(Art::Stalls), ground: Landscape::Square, detail: "旅人が手紙を頼める小さな机が並んでいる。" },
            Place { name: "回廊脇の花壇", at: (196, 124), art: None, ground: Landscape::Yard, detail: "静かに花を手向けられる日当たりのよい一角。" },
        ],
        stories: [
            Story {
                title: "奉納した覚えのない日",
                beats: &[
                    Beat::speech(3, "相談所の助祭", Villager, "記憶を奉納した覚えがないのに、記録上は『奉納済み』にされている家族がいるんだ。記録院の帳面を一緒に確かめてくれないか。"),
                    Beat::scene(1, "記録院の運用帳を見ると、カインが持ち出した禁書に関わる時期だけ、照合の印が別のものになっていた。"),
                    Beat::speech(1, "記録官", Worker, "印の違いは認めるよ。だが全体を一気に書き換えるには審査がいる。証人の言葉で記録を直すか、先に家族の閲覧権だけ戻そうか？"),
                ],
                choices: Some([
                    Ending { label: "証言で記録を訂正する", text: "助祭と証人たちが聴聞に出席し、家族の本来の記録が正式に戻された。" },
                    Ending { label: "先に閲覧権を取り戻す", text: "家族は自分たちの控えをすぐ読めるようになり、次の審査へ向けた準備が整った。" },
                ]),
            },
            Story {
                title: "鐘のあいだの沈黙",
                beats: &[
                    Beat::speech(0, "巡礼者", Traveler, "双塔の鐘はいつも二つで鳴るのに、今日は片方しか鳴らないんだ。鐘守に何かあったのかな。"),
                    Beat::scene(5, "鐘楼へ上がると、二人の鐘守のうち一人が病で倒れていた。相棒は一人で二つの鐘をつなぐ綱を結ぼうとしている。"),
                    Beat::scene(5, "二つの鐘の綱を滑車でつなぐ手伝いをした。夕暮れには双塔の音が揃って鳴り、病の鐘守も笑顔で耳を傾けた。"),
                ],
                choices: None,
            },
            Story {
                title: "忘れてよい感謝",
                beats: &[
                    Beat::speech(6, "巡礼者", Traveler, "記憶を奉納する前に、昔助けてくれた恩人へお礼の手紙を書きたいんだ。でも文字がうまく書けなくてね。"),
                    Beat::speech(3, "代筆人", Villager, "手紙は私が代筆するよ。ただ、恩人本人に直接届けるか、同じように誰か見知らぬ旅人を助ける品に添えるか、本人が迷ってるんだ。"),
                ],
                choices: Some([
                    Ending { label: "恩人本人へ渡す", text: "手紙を受け取った恩人は、忘れたあとも気にかけてくれたことに胸をなで下ろした。" },
                    Ending { label: "旅人を助ける品に添える", text: "温かい外套と手紙が巡礼路の宿へ預けられ、次の旅人の助けになった。" },
                ]),
            },
            Story {
                title: "交代しない衛兵",
                beats: &[
                    Beat::speech(2, "詰所の騎士", Guard, "交代時刻を過ぎても、あの兵士が持ち場を離れようとしないんだ。なぜあんなに頑ななのか聞いてきてくれ。"),
                    Beat::scene(4, "古い配置図を見ると、塞がれた石段の奥でかつて崩落事故が起きていた。兵士は誰かが迷い込まないよう見張り続けていたのだ。"),
                    Beat::scene(2, "騎士団が石段の入口に頑丈な柵と注意札を立てた。それを見た兵士はようやく肩の力を抜き、交代を代わった。"),
                ],
                choices: None,
            },
            Story {
                title: "広場にない献花台",
                beats: &[
                    Beat::speech(7, "街の住民", Villager, "奉納を選ばなかった家族が花を置ける場所が撤去されちまった。どこか静かに手を合わせられる場所を作れないかな。"),
                    Beat::speech(0, "大聖堂の世話役", Villager, "広場の隅に早朝だけ仮の台を置くか、住民と協力して回廊の脇を整えるか。どちらがみんなに喜ばれるだろう？"),
                ],
                choices: Some([
                    Ending { label: "広場に仮の献花台を置く", text: "朝一番から花を手向ける人が途絶えず、広場に穏やかな時間が戻った。" },
                    Ending { label: "回廊脇の花壇を整える", text: "住民が石を並べて花壇を作り、いつでも静かに追悼できる場所が生まれた。" },
                ]),
            },
        ],
    },
    Plan {
        name: "白壁の砦町",
        profile: "白い断崖の上から外海を見張る砦町。平時に公共水槽を転用するには砦と住民の合意が要る。",
        terrain: Landscape::Chalk,
        entry: (156, 174),
        places: [
            Place { name: "白壁の砦と水槽", at: (156, 72), art: Some(Art::Landmark), ground: Landscape::Square, detail: "白い城壁の内側に、街と軍で使う大きな水槽がある。" },
            Place { name: "西の工兵測量台", at: (84, 96), art: Some(Art::Palisade), ground: Landscape::Chalk, detail: "崖下の家並みと地下の坑道を見比べられる測量台。" },
            Place { name: "南の崖下井戸", at: (156, 136), art: Some(Art::Well), ground: Landscape::Square, detail: "崖の下に暮らす住民が毎日水を汲みに来る井戸。" },
            Place { name: "東の水平階段", at: (228, 108), art: Some(Art::Bridge), ground: Landscape::Chalk, detail: "砦と崖下の住宅街を結ぶ長い石階段。" },
            Place { name: "踊り場の見張り所", at: (224, 76), art: Some(Art::Cottage), ground: Landscape::Square, detail: "階段の途中に設けられた小さな休み場。" },
            Place { name: "崖下の砂浜", at: (236, 146), art: None, ground: Landscape::Shore, detail: "演習の標的がときおり流れ着く静かな浜。" },
            Place { name: "軍用水槽の配管口", at: (126, 86), art: Some(Art::Sluice), ground: Landscape::Square, detail: "水槽から街へ水を分ける古い石の栓。" },
            Place { name: "渇水目盛りの壁", at: (186, 88), art: None, ground: Landscape::Chalk, detail: "過去の渇水時の水位が白壁に刻み残されている。" },
        ],
        stories: [
            Story {
                title: "崖下に残す一杯",
                beats: &[
                    Beat::speech(2, "崖下の住民", Villager, "軍の水槽を増やしたせいで、うちの井戸水が枯れかけてるんだ。工兵は古い坑道を開けると言うけど、真下には家があるのに！"),
                    Beat::scene(1, "測量台の傾斜板で確かめると、坑道は家の真下を通っていたが、軍用水槽の配管から大きく水漏れしていることも分かった。"),
                    Beat::speech(6, "工兵隊長", Guard, "確かに水槽が漏れてるな。家を避けた別の水脈を掘るか、古い水槽の漏れを塞いで余分な取水を止めるか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "家を避けて新水脈を掘る", text: "住民の家を守ったまま新しい湧き水が見つかり、井戸の水位が元に戻った。" },
                    Ending { label: "水槽の漏れを修理する", text: "無駄な取水が止まり、崖下の井戸にきれいな水がたっぷりと湧き戻った。" },
                ]),
            },
            Story {
                title: "白壁のつばめ",
                beats: &[
                    Beat::speech(3, "階段の子ども", Villager, "砦の壁の穴を工兵さんが塞ぐんだって！　でもあの中にはつばめの雛がいるんだよ。確かめてきて！"),
                    Beat::scene(3, "階段の途中から壁の穴をのぞくと、もうすぐ巣立ちそうな雛たちが顔を出していた。あと数日待てば飛び立てそうだ。"),
                    Beat::scene(0, "工兵に事情を話すと、雛が巣立つまで板で外から養生して待つことになった。子どもは毎日嬉しそうに空を見上げた。"),
                ],
                choices: None,
            },
            Story {
                title: "階段番の椅子",
                beats: &[
                    Beat::speech(4, "足の悪い老兵", Guard, "階段の巡回を降りることになっちまってな。みんなに頼りにされてた相談役も続けられそうにないよ。"),
                    Beat::scene(2, "崖下の井戸の近くに、木工のうまい住民がいた。踊り場に置ける丈夫な椅子ならすぐ作れそうだ。"),
                    Beat::scene(4, "踊り場に背もたれ付きの椅子が置かれた。老兵は座ったまま旅人の道案内や住民の相談役を元気に続けられるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "波が隠した境界",
                beats: &[
                    Beat::speech(5, "崖下の住民", Villager, "海から演習の標的が流れてきて、子どもが拾っちまったんだ。危険だから何とかしてほしいよ。"),
                    Beat::speech(0, "砦の隊長", Guard, "潮流が変わったせいか。演習の境界線を沖へ引き直すか、子どもたちが浜で遊ぶ時間を昼の安全な時間帯に決めるか、どっちがいいだろう？"),
                ],
                choices: Some([
                    Ending { label: "演習の境界を引き直す", text: "標柱が沖寄りに打ち直され、浜辺に危険なものが流れ着かなくなった。" },
                    Ending { label: "安全な遊び時間を決める", text: "見張りの旗が立っていない安全な時間帯を知らせる鐘が鳴るようになった。" },
                ]),
            },
            Story {
                title: "壁に刻んだ渇き",
                beats: &[
                    Beat::speech(7, "見習い工兵", Worker, "見た目が悪いからって、白壁に刻まれた昔の水位線を削り落とせって命じられたんだ。でもこれ、大事な記録だよね。"),
                    Beat::speech(2, "古参の住民", Villager, "あれは渇水のときの節水の目安なんだよ。井戸の枠に目盛りを写すか、白壁の一部だけでも残してもらえないかね。"),
                ],
                choices: Some([
                    Ending { label: "目盛りを井戸へ移す", text: "井戸の石枠に正確な水位線が刻まれ、誰でもすぐ見て節水できるようになった。" },
                    Ending { label: "白壁の一部を残す", text: "砦長が歴史の記録として残すことを許し、隣に説明の板も添えられた。" },
                ]),
            },
        ],
    },
    Plan {
        name: "麦穂の宿場",
        profile: "巡礼者を挽きたての麦でもてなす宿場。宿組合と製粉組が交互に代表を務めている。",
        terrain: Landscape::Farm,
        entry: (156, 174),
        places: [
            Place { name: "西尾根の風車", at: (98, 72), art: Some(Art::Landmark), ground: Landscape::Farm, detail: "白い石造りの風車が、絶えず麦を挽き続けている。" },
            Place { name: "中央の施食宿", at: (156, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "旅人に安値で温かいパンを出す大きな宿。" },
            Place { name: "東の検問待機所", at: (226, 104), art: Some(Art::Palisade), ground: Landscape::Yard, detail: "聖堂街へ向かう荷車が検問を待って並んでいる。" },
            Place { name: "南の麦倉と朝市", at: (156, 144), art: Some(Art::Stalls), ground: Landscape::Square, detail: "収穫した麦と豆が並ぶにぎやかな朝市。" },
            Place { name: "畑の間の細道", at: (196, 126), art: None, ground: Landscape::Farm, detail: "検問の列を避けて朝市へ抜けられる農道。" },
            Place { name: "宿の台所", at: (136, 108), art: Some(Art::Hearth), ground: Landscape::Yard, detail: "大きな竈で毎日パンと豆の煮込みを作っている。" },
            Place { name: "布袋の洗い場", at: (116, 130), art: Some(Art::Well), ground: Landscape::Yard, detail: "返却された麦袋が山のように積まれている。" },
            Place { name: "宿帳の机", at: (174, 92), art: None, ground: Landscape::Square, detail: "宿の玄関脇にある、落書きだらけの古い帳面。" },
        ],
        stories: [
            Story {
                title: "列の外のパン",
                beats: &[
                    Beat::speech(1, "宿の主人", Villager, "粉を運ぶ荷車が検問で足止めされて、施しのパンが切れそうなんだ。朝市の倉を見てきてくれないか。"),
                    Beat::scene(3, "麦倉の在庫にはまだ余裕がある。朝市でパンと麦を直接交換するか、検問所に頼んで傷みやすい粉だけ先に通してもらうかできそうだ。"),
                    Beat::speech(2, "検問の番兵", Guard, "事情は分かった。朝市で麦をやりくりするか、生ものの粉袋だけ列の先へ通すか、どっちの手配にしよう？"),
                ],
                choices: Some([
                    Ending { label: "朝市で麦を融通し合う", text: "宿と朝市の間で現物交換の仕組みができ、パンの窯が止まらずに済んだ。" },
                    Ending { label: "粉袋だけ優先して通す", text: "検問の列が整理され、傷みやすい粉が無事に宿へ届いた。" },
                ]),
            },
            Story {
                title: "挽かれない麦",
                beats: &[
                    Beat::speech(0, "風車の弟子", Worker, "変だな……粉に挽こうとした袋から、いつもの麦と違う甘い香りがするんだよ。"),
                    Beat::scene(3, "朝市の農家に確かめると、間違えて来春まく予定だった在来種の種麦を運び込んでしまったらしい。"),
                    Beat::scene(0, "挽く直前で取り出した種麦は畑の倉へ戻り、宿場では昔ながらの香ばしい麦を絶やさずに済んだ。"),
                ],
                choices: None,
            },
            Story {
                title: "布袋の帰路",
                beats: &[
                    Beat::speech(6, "洗い場の係", Worker, "施食所に借りたままの布袋が溜まりすぎて置き場がないんだ。焼印を見れば持ち主は分かるんだけどね。"),
                    Beat::speech(3, "朝市の商人", Worker, "運び手に返して預り金を戻すか、みんなで買い付けに使い回せる共用袋にしちまうか、どっちがいいかな？"),
                ],
                choices: Some([
                    Ending { label: "運び手へ返して精算する", text: "袋が元の持ち主へ戻り、預り金も返って運び手たちは助かった。" },
                    Ending { label: "町の共用袋にする", text: "共有の印が押された袋が市を巡り、袋不足がすっかり解消した。" },
                ]),
            },
            Story {
                title: "旅の食卓",
                beats: &[
                    Beat::speech(1, "巡礼者", Traveler, "申し訳ない、体質で小麦のパンが食べられないんだ……。何か代わりに旅に持てるものはないだろうか。"),
                    Beat::scene(3, "朝市で豆と干し芋を分けてもらい、宿の台所で塩気のある焼き団子に仕立ててもらった。"),
                    Beat::scene(5, "巡礼者は香ばしい豆団子を大事そうに袋へしまい、宿場の優しいもてなしに深く礼を言った。"),
                ],
                choices: None,
            },
            Story {
                title: "追い風の墨",
                beats: &[
                    Beat::speech(7, "宿の子ども", Villager, "宿帳に落書きしたって怒られたんだ！　でもこれ、風車の羽根の回り方をメモした大事な絵なんだよ！"),
                    Beat::scene(0, "風車の親方が絵を見ると、昔の追い風の記録とぴったり一致していた。親方は子どもの観察眼に感心した。"),
                    Beat::scene(7, "宿主は子どもたちに風車の絵を描ける専用の板を壁に掛けてやった。二人は仲良く並んで空を見上げた。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "湖鏡の町",
        profile: "水没した旧書庫の写本を拾い集めて築いた学問の町。湖岸の水位に合わせて乾燥工房が動く。",
        terrain: Landscape::River,
        entry: (156, 46),
        places: [
            Place { name: "円形の書庫堂", at: (156, 86), art: Some(Art::Landmark), ground: Landscape::Square, detail: "湖面から救い出した写本を収める美しい円堂。" },
            Place { name: "東の乾燥工房", at: (224, 102), art: Some(Art::Inn), ground: Landscape::Square, detail: "濡れた紙を傷めずに乾かす風通しのよい工房。" },
            Place { name: "西の渡しと潜水具庫", at: (88, 104), art: Some(Art::Cottage), ground: Landscape::Dock, detail: "湖へ潜るための道具と小舟が並んでいる。" },
            Place { name: "南端の水位杭", at: (156, 140), art: Some(Art::Sluice), ground: Landscape::Shore, detail: "湖の水が引いた日だけ岸辺の道が現れる。" },
            Place { name: "湖岸の干し屋根", at: (228, 134), art: None, ground: Landscape::Shore, detail: "晴れた日には白い写本が一面に並べられる。" },
            Place { name: "照合会の集会机", at: (126, 98), art: Some(Art::Stalls), ground: Landscape::Square, detail: "見つかった手紙の持ち主を確かめる公開の机。" },
            Place { name: "静かな渡し場", at: (116, 136), art: Some(Art::Bridge), ground: Landscape::Dock, detail: "底に古い鐘台の屋根が透けて見える場所。" },
            Place { name: "乾燥棚の裏口", at: (206, 82), art: None, ground: Landscape::Yard, detail: "涼しい湖風が抜ける石造りの通路。" },
        ],
        stories: [
            Story {
                title: "水面が下がるあいだ",
                beats: &[
                    Beat::speech(0, "書庫会の代表", Villager, "水位が下がって、水没した旧書庫の棚が少し見えたんだ。公的な台帳を先に引き上げてもらいたいんだがね。"),
                    Beat::speech(2, "潜り手", Worker, "でも水に弱い私信の箱から助けないと、手紙が全部溶けちまうよ。次の満水まで時間がない。どっちを先に上げよう？"),
                ],
                choices: Some([
                    Ending { label: "公的な台帳から引き上げる", text: "町の歴史を証明する大切な台帳が、欠けることなく書庫堂に戻った。" },
                    Ending { label: "傷みやすい私信から助ける", text: "家族宛ての手紙が溶ける前に救い出され、持ち主たちの手へ戻った。" },
                ]),
            },
            Story {
                title: "封のまま返す",
                beats: &[
                    Beat::speech(1, "乾燥工房の職人", Worker, "拾い上げた手紙の宛名が、町の二軒の同じ名字に当てはまるんだ。中身を見ないとどっちの家か分からないかな……。"),
                    Beat::scene(5, "封を開けずに便りの控えと家の紋を照合したところ、西の通りの家族のものだと分かった。"),
                    Beat::speech(5, "受取人の家族", Villager, "ありがとう！　この手紙、みんなの前で読んで聞かせようか、それとも封をしたまま大事に持って帰ろうか？"),
                ],
                choices: Some([
                    Ending { label: "希望どおり読み上げる", text: "手紙に綴られた無事を伝える言葉に、広場の誰もがほっと息をついた。" },
                    Ending { label: "封をしたまま返す", text: "家族は秘密を守ってくれた礼を述べ、手紙を胸に抱いて帰っていった。" },
                ]),
            },
            Story {
                title: "晴れ間の屋根",
                beats: &[
                    Beat::speech(4, "水上組の船頭", Worker, "今日は雲が速いから、屋根に広げた写本が雨に降られる前にしまわないと危ないぞ！"),
                    Beat::scene(7, "乾燥工房の天井に風を通す吊り棚を追加した。強い日差しや雨を避けながら、涼しい湖風だけで紙を乾かせるようになった。"),
                    Beat::scene(1, "急なにわか雨が降っても、写本は一枚も濡れずに安全に乾き上がった。"),
                ],
                choices: None,
            },
            Story {
                title: "湖の底の鐘台",
                beats: &[
                    Beat::speech(6, "渡し客", Traveler, "さっき舟から水の下をのぞいたら、古い建物の屋根みたいなものが見えたよ。網が引っかかりそうで危ないね。"),
                    Beat::scene(2, "古い湖岸の地図と照らし合わせると、沈んだ昔の渡し場の鐘台だった。"),
                    Beat::scene(6, "鐘台の柱の位置に赤い浮標を浮かべた。漁師たちは網を破らずに済み、旅人も昔の町の面影を眺められるようになった。"),
                ],
                choices: None,
            },
            Story {
                title: "同じ名前の家",
                beats: &[
                    Beat::speech(5, "町の住民", Villager, "うちの屋号と別の家の屋号が同じだからって、渡し券がひと家族ぶんしか発行されないんだ！"),
                    Beat::scene(0, "二軒それぞれの井戸の札と近所の証言を集め、別の世帯であることを書庫堂の台帳に書き加えた。"),
                    Beat::scene(5, "両方の家族にちゃんとした渡し券が届き、対岸の畑へ一緒に通えるようになった。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "鐘草の村",
        profile: "霧の中でも方角が分かるよう青い鐘楼を建てた薬草の村。鐘の打ち方は村総出の立会いで決める。",
        terrain: Landscape::Meadow,
        entry: (156, 174),
        places: [
            Place { name: "青い小鐘楼", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "霧の中でも響く澄んだ音色の青い鐘楼。" },
            Place { name: "北の薬草畑", at: (156, 48), art: Some(Art::Stalls), ground: Landscape::Farm, detail: "霧を好む薬草を育てる段々の畑。" },
            Place { name: "西の鋳直し場", at: (84, 102), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "老鋳物師と弟子が鐘を調べている工房。" },
            Place { name: "南谷の水番小屋", at: (224, 128), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "谷の増水を見張り、鐘の合図を聞き取る小屋。" },
            Place { name: "薬草の干し棚", at: (192, 62), art: None, ground: Landscape::Yard, detail: "摘んだ薬草を風で乾かす大きな木枠。" },
            Place { name: "北の牧場柵", at: (116, 62), art: None, ground: Landscape::Pasture, detail: "鐘の合図を聞いて動く牛たちの放牧地。" },
            Place { name: "谷沿いの見通し台", at: (218, 92), art: None, ground: Landscape::Heath, detail: "鐘の反響が谷のどこまで届くか確かめられる。" },
            Place { name: "村の薬師小屋", at: (122, 122), art: Some(Art::Inn), ground: Landscape::Square, detail: "乾かした薬草を煎じて薬を作る小屋。" },
        ],
        stories: [
            Story {
                title: "谷へ届く三打",
                beats: &[
                    Beat::speech(3, "水番", Worker, "聖堂街に合わせて時報の打ち方を変えたら、危険を知らせる合図と紛らわしくて聞き逃しそうになったよ！"),
                    Beat::scene(6, "谷の三か所で響きを確かめると、時報を先にして間を空けるか、危険の合図だけ別の小さな鐘に分けられそうだ。"),
                    Beat::speech(0, "若い鐘守", Villager, "みんなの安全が一番だ。時報の打ち方をずらすか、危険合図を専用の鐘に移すか、どっちがいいだろう？"),
                ],
                choices: Some([
                    Ending { label: "時報の間隔を空ける", text: "三打の危険合図がはっきり聞き分けられるようになり、谷の作業が安全に戻った。" },
                    Ending { label: "危険合図を別の鐘に移す", text: "専用の鋭い鐘の音が谷じゅうに通り、時報と聞き間違えることがなくなった。" },
                ]),
            },
            Story {
                title: "鋳物師の耳",
                beats: &[
                    Beat::speech(2, "老鋳物師", Worker, "年を取って、新しい鐘の微妙な響きが聞き分けられなくなっちまった……。弟子に継がせたいんだが、自信が持てないようだ。"),
                    Beat::scene(1, "北の畑や水番小屋で聞いた感想を集めると、弟子の作った鐘は霧の中でも澄んでよく通るとみんなが褒めていた。"),
                    Beat::scene(2, "人々の言葉を聞いた老鋳物師は安堵し、弟子へ鐘の調整を正式に任せることにした。"),
                ],
                choices: None,
            },
            Story {
                title: "薬草を焼く朝",
                beats: &[
                    Beat::speech(4, "薬草摘み", Villager, "干し棚の枠が破れて、乾かしてた薬草が霧の中に落ちちまったんだ！　探すのを手伝ってくれないか？"),
                    Beat::scene(1, "香りを頼りに畦をたどり、無傷の袋を回収した。少し湿ってしまった分も、薬師に相談すると入浴用の薬湯に使えるそうだ。"),
                    Beat::scene(7, "無駄になった薬草はひとつもなく、村の湯小屋にいい香りの薬湯が沸いた。"),
                ],
                choices: None,
            },
            Story {
                title: "欠けた舌の帰還",
                beats: &[
                    Beat::speech(2, "鍛冶職人", Worker, "廃村から持ち帰った古い鐘の部品を溶かして、新しい鐘に作り直したいんだ。でも遺族が反対していてね。"),
                    Beat::scene(0, "鐘楼の横に小さな試聴台を組み、古い部品を鳴らしてみた。懐かしい優しい響きに、誰もが見とれた。"),
                    Beat::scene(2, "今の鐘と音色を比べたあと、古い部品は記念としてそのまま保存し、新しい鐘は別の銅で作ることになった。"),
                ],
                choices: None,
            },
            Story {
                title: "牛が覚えた合図",
                beats: &[
                    Beat::speech(5, "牛追い", Worker, "鐘の音を変えてから、牛たちが合図を勘違いして畑に入り込んじまうんだ！"),
                    Beat::speech(0, "鐘守", Villager, "試し打ちの時間を牛が小屋にいる昼間にずらすか、餌場の場所を移して新しい音を覚えてもらうか、どっちにしよう？"),
                ],
                choices: Some([
                    Ending { label: "試し打ちの時刻をずらす", text: "放牧の時間と重ならなくなり、牛たちは落ち着いて草を食むようになった。" },
                    Ending { label: "餌場を移して音を教える", text: "数日もすると牛たちが新しい合図を覚え、迷わず柵へ戻るようになった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "聖律門の市",
        profile: "教会領への積荷を調べる検印門の内外に開かれた市場。取引の揉め事は門外の仲裁人が聞き取る。",
        terrain: Landscape::Stone,
        entry: (156, 174),
        places: [
            Place { name: "聖律門の検印台", at: (156, 92), art: Some(Art::Landmark), ground: Landscape::Square, detail: "積荷の通行証に検印を押す大きな石の門。" },
            Place { name: "西の門外市と仲裁所", at: (84, 108), art: Some(Art::Stalls), ground: Landscape::Square, detail: "門の外で開かれる露店と仲裁人の机。" },
            Place { name: "東の共同秤場", at: (224, 104), art: Some(Art::Inn), ground: Landscape::Square, detail: "荷の重さを公に量る屋根付きの秤場。" },
            Place { name: "北の証文屋通り", at: (156, 48), art: Some(Art::Cottage), ground: Landscape::Square, detail: "公認の証文屋が軒を連ねる静かな通り。" },
            Place { name: "東の裏通り", at: (212, 74), art: None, ground: Landscape::Yard, detail: "再検査の荷車を静かに回せる裏道。" },
            Place { name: "門外の小机", at: (116, 128), art: None, ground: Landscape::Square, detail: "文字の読み書きを手伝う代書人の小さな机。" },
            Place { name: "秤場の水受け", at: (238, 128), art: Some(Art::Well), ground: Landscape::Yard, detail: "屋根から落ちる雨水を逃がす石の溝。" },
            Place { name: "門外の夕食広場", at: (146, 138), art: Some(Art::Hearth), ground: Landscape::Square, detail: "閉市あとも旅人が温かい食事をとれる広場。" },
        ],
        stories: [
            Story {
                title: "通らない検印",
                beats: &[
                    Beat::speech(1, "遠くから来た行商", Traveler, "ちゃんと市の証文屋で印を直してもらったのに、門衛が『無効だ』って通してくれないんだ。このままじゃ魚が傷んじまう！"),
                    Beat::scene(3, "証文屋の控えと共同秤の記録を照合すると、紛れもない正規の印だった。偽印が混じったのは別の屋号の書類だけだ。"),
                    Beat::speech(0, "門衛長", Guard, "疑いは晴れた。怪しい屋号の荷だけ再検査にするか、傷みやすい生鮮品だけ条件付きで先に通すか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "怪しい屋号だけ再検査する", text: "正しい行商たちの荷が一斉に門を通り、市場に活気が戻った。" },
                    Ending { label: "生鮮品を先に開放する", text: "魚や野菜が新鮮なまま市に並び、買い手からも感謝の声が上がった。" },
                ]),
            },
            Story {
                title: "最後の一枠",
                beats: &[
                    Beat::speech(2, "共同秤の係", Worker, "もうすぐ秤場を閉める時間なのに、魚屋と織物商が『自分が最後の計量だ』って譲らないんだよ。"),
                    Beat::speech(1, "仲裁人", Villager, "魚は朝まで待てないし、織物は今夜の契約に間に合わせたいらしい。二人で荷を分けて今日中に計るか、明朝一番の特別枠を作るか選んでもらおう。"),
                ],
                choices: Some([
                    Ending { label: "二人で枠を分け合う", text: "荷を半分ずつ計量して双方が間に合い、市が終わる頃には笑い合っていた。" },
                    Ending { label: "明朝に追加の枠を作る", text: "魚が優先され、織物商には明朝の特別枠と仮の証明書が出された。" },
                ]),
            },
            Story {
                title: "門外の小机",
                beats: &[
                    Beat::speech(5, "行商人", Traveler, "読み書きが苦手で、契約書の文字が読めずに損をしてしまったんだ……。"),
                    Beat::scene(1, "仲裁人の紹介で、門外に読み合わせを手伝う小机と当番の日が設けられた。"),
                    Beat::scene(5, "行商人は契約の内容をしっかり理解した上で取引できるようになり、堂々と門をくぐった。"),
                ],
                choices: None,
            },
            Story {
                title: "秤が沈む日",
                beats: &[
                    Beat::speech(2, "秤場の係", Worker, "雨の日だけ、新しい秤の目盛りが軽く出ちまうんだ。誰かが細工したわけでもないのに不思議でね。"),
                    Beat::scene(6, "屋根の雨漏りで受け皿の下の木台が水を含み、重みで沈んでいた。"),
                    Beat::speech(2, "秤場の係", Worker, "今日は受け皿の水を抜くだけでしのぐか、水に強い石の台へ丸ごと交換するか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "受け皿の水を抜いて調整する", text: "応急処置でその日の取引は無事に終わり、翌日に屋根も直された。" },
                    Ending { label: "石の台へ交換する", text: "どんな大雨の日でも秤が狂わなくなり、商人たちの信頼がいっそう厚くなった。" },
                ]),
            },
            Story {
                title: "売れ残った祝祭灯",
                beats: &[
                    Beat::speech(1, "露店主", Worker, "祭りが中止になって、大量の灯籠が売れ残っちまったんだ。どうにか引き取ってくれる先はないかな。"),
                    Beat::speech(7, "夕食広場の世話役", Villager, "巡礼宿の非常灯として買い取るか、ここ門外の夕食広場に吊るして夜の客を呼び込むか、どっちがいいかな？"),
                ],
                choices: Some([
                    Ending { label: "巡礼宿の非常灯にする", text: "宿の廊下が明るくなり、夜遅く着いた旅人も安心して休めるようになった。" },
                    Ending { label: "夕食広場に吊るす", text: "柔らかな灯りの下に人が集まり、門外市は夜まで賑わう名所になった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "誓約の修道村",
        profile: "街道沿いに診療所と薬草園を開いた修道村。治療の優先順は当番の施療師が判断している。",
        terrain: Landscape::Meadow,
        entry: (156, 174),
        places: [
            Place { name: "誓約の修道回廊", at: (156, 94), art: Some(Art::Landmark), ground: Landscape::Square, detail: "静かな中庭を囲む白い石柱の修道回廊。" },
            Place { name: "北の誓約聴聞室", at: (156, 48), art: Some(Art::Inn), ground: Landscape::Square, detail: "昔の避難と誓約の記録が収められた静かな部屋。" },
            Place { name: "東の薬草段畑", at: (226, 98), art: Some(Art::Stalls), ground: Landscape::Farm, detail: "日当たりのよい斜面に薬草が整然と植えられている。" },
            Place { name: "南の旅人用小屋", at: (156, 142), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "体調を崩した旅人が静かに休める隔離小屋。" },
            Place { name: "薬草の運び道", at: (204, 126), art: None, ground: Landscape::Yard, detail: "回廊を通らずに段畑と小屋を結ぶ裏道。" },
            Place { name: "中庭の井戸", at: (126, 98), art: Some(Art::Well), ground: Landscape::Square, detail: "薬を煎じるための清らかな水が湧く井戸。" },
            Place { name: "回廊の台所", at: (110, 74), art: Some(Art::Hearth), ground: Landscape::Yard, detail: "患者のための温かい食事とお菓子を作る台所。" },
            Place { name: "施療院の薬棚", at: (184, 78), art: None, ground: Landscape::Square, detail: "古い薬瓶がぎっしり並んだ木の棚。" },
        ],
        stories: [
            Story {
                title: "誰に交わした誓い",
                beats: &[
                    Beat::speech(0, "施療師", Villager, "昔の大災害のときに交わした誓いの解釈を巡って、古参の修道士と意見が割れてるんだ。当時の証人に話を聞いてきてくれないか。"),
                    Beat::scene(1, "聴聞室の古い証言から、その誓いは『立場の違いを問わず、目の前の傷ついた人を助ける』ためのものだったと分かった。"),
                    Beat::speech(0, "施療師", Villager, "今の患者に寄り添う形へ誓いを結び直すか、古い誓いを守りつつ別の手当ても募集するか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "今の患者への誓いに結び直す", text: "新しい誓約が回廊に掲げられ、村人も旅人も分け隔てなく治療を受けられるようになった。" },
                    Ending { label: "古い誓いに別の手当てを足す", text: "伝統を重んじながら協力の輪が広がり、診療所の薬も十分に行き渡るようになった。" },
                ]),
            },
            Story {
                title: "歩ける薬草園",
                beats: &[
                    Beat::speech(3, "回復した旅人", Traveler, "お礼に薬草園の草取りを手伝いたいんだけど、段畑の坂が急でまだ下まで降りられないんだよ。"),
                    Beat::scene(4, "段畑の脇にある緩やかな運び道を整えて、本人の足でも無理なく通える手すりを付けた。"),
                    Beat::scene(2, "旅人は日の当たる畝で楽しそうに草を取り、薬草番のよい話し相手になった。"),
                ],
                choices: None,
            },
            Story {
                title: "塀の外の皿",
                beats: &[
                    Beat::speech(3, "隔離小屋の旅人", Traveler, "毎日届く食事に、こっそり小さなお菓子が添えられてるんだ。誰が作ってくれてるのかお礼を言いたいよ。"),
                    Beat::scene(6, "回廊の台所で、料理番が患者を元気づけようと毎朝焼いていたことが分かった。"),
                    Beat::scene(3, "料理番は旅人から聞いた故郷のお菓子の作り方を教えてもらい、次の日の皿にさっそく乗せて届けた。"),
                ],
                choices: None,
            },
            Story {
                title: "余った寝台",
                beats: &[
                    Beat::speech(0, "退院間近の住民", Villager, "私はもうすぐ帰れるから、この寝台を遠くから来た旅人に譲りたいんだ。宿じゃゆっくり休めないだろう？"),
                    Beat::speech(3, "小屋の世話役", Villager, "空き小屋を回復室に整えるか、村の炊事当番にお願いして通いの看護を続けるか、どちらなら続けられそうかな？"),
                ],
                choices: Some([
                    Ending { label: "空き小屋を回復室にする", text: "静かに養生できる部屋が増え、旅人も安心して長旅の疲れを癒せるようになった。" },
                    Ending { label: "通いの看護を頼む", text: "住民たちが交代で温かい汁を届け、退院した人も無理なく暮らしに戻れた。" },
                ]),
            },
            Story {
                title: "薬棚の元日",
                beats: &[
                    Beat::speech(7, "若い薬師", Villager, "先代が並べ替えを忘れたままの薬棚で、必要な薬がすぐ見つからなくて困ってるんだ。"),
                    Beat::scene(2, "採取場所の札と控えを照らし合わせ、名前順と夜の暗がりでも手触りで分かる印を瓶に付けた。"),
                    Beat::scene(7, "夜の急患が来たときも迷わず薬を取り出せるようになり、薬師は胸を張って棚の前に立った。"),
                ],
                choices: None,
            },
        ],
    },
    Plan {
        name: "段丘の集落",
        profile: "崖の湧き水を段々の棚田に引き込んで暮らす集落。水の開閉には上段と下段双方の立会いが要る。",
        terrain: Landscape::Valley,
        entry: (156, 174),
        places: [
            Place { name: "段丘の石段住宅", at: (156, 108), art: Some(Art::Landmark), ground: Landscape::Square, detail: "急な斜面に石段でつながれた黄土色屋根の家並み。" },
            Place { name: "北の上段取水口", at: (156, 46), art: Some(Art::Sluice), ground: Landscape::Stone, detail: "崖から湧く清水を棚田へ分ける取水口。" },
            Place { name: "中央の測量柱と会所", at: (156, 82), art: Some(Art::Inn), ground: Landscape::Square, detail: "家の屋号が順に刻まれた背の高い測量柱がある。" },
            Place { name: "南の棚田", at: (156, 144), art: None, ground: Landscape::Farm, detail: "空と灯りを映すように水を張った美しい段々の田。" },
            Place { name: "西の排水横穴", at: (84, 96), art: None, ground: Landscape::Stone, detail: "工事の前から使われていた古い水抜きの横穴。" },
            Place { name: "最上段の掛け箱", at: (192, 54), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "水運びのついでに手紙を入れられる小さな木箱。" },
            Place { name: "中段の溜め池", at: (220, 106), art: Some(Art::Well), ground: Landscape::Yard, detail: "水量の少ない日に備えて清水を溜めておく池。" },
            Place { name: "東の畦道", at: (210, 134), art: None, ground: Landscape::Farm, detail: "段ごとの水番が行き来する細い畦道。" },
        ],
        stories: [
            Story {
                title: "泥水の出口",
                beats: &[
                    Beat::speech(3, "下段の農家", Villager, "上段で新しい屋敷の工事を始めてから、うちの棚田に泥水が流れてくるんだ！　何とか止めてくれよ。"),
                    Beat::scene(4, "西の排水横穴を調べると、工事前からあった溝ではなく、工事で掘った土がそのまま流れ込んでいた。"),
                    Beat::speech(2, "段会所の代表", Villager, "原因は工事の泥だ。工事側に泥を沈める溝を掘らせるか、水が澄むまで普請を一時止めてもらうか、どっちにしよう？"),
                ],
                choices: Some([
                    Ending { label: "泥を沈める溝を直させる", text: "翌日には澄んだ水だけが棚田に届くようになり、工事も安全に再開できた。" },
                    Ending { label: "水が澄むまで普請を止める", text: "苗が植わった大切な時期を守り切り、下段の田にも無事稲が育った。" },
                ]),
            },
            Story {
                title: "ずれた測量柱",
                beats: &[
                    Beat::speech(2, "段の子ども", Villager, "ねえ、この測量柱、石積みのあとから指一本ぶんくらい傾いた気がするんだ！"),
                    Beat::scene(3, "畦道から水面と隣の柱を見比べると、確かに小壁の根元が少し緩んでいた。"),
                    Beat::scene(2, "田植えの前に石工を呼んで小壁を補強したおかげで、崩れる前にしっかり直せた。"),
                ],
                choices: None,
            },
            Story {
                title: "上下の郵便",
                beats: &[
                    Beat::speech(0, "配達人", Worker, "膝を痛めて、一番上の段まで手紙を届ける石段がどうしても上れないんだよ……。"),
                    Beat::scene(5, "毎朝上段へ水を運ぶ住民に協力してもらい、階段の途中に雨よけのある掛け箱を取り付けた。"),
                    Beat::scene(0, "雨の日でも手紙が濡れずに一番上の家まで届くようになり、配達人も安心して仕事を続けられた。"),
                ],
                choices: None,
            },
            Story {
                title: "湧き水の分け方",
                beats: &[
                    Beat::speech(1, "上段の農家", Villager, "うちは今から用水を止めたいのに、下段は植えたばかりだから水を出せって揉めてるんだ。"),
                    Beat::speech(6, "水番", Worker, "溜め池の水ならまだ少し持つよ。時間をずらして順番に流すか、乾きやすい畝にだけ桶で運ぶか、どうしよう？"),
                ],
                choices: Some([
                    Ending { label: "時間をずらして取水する", text: "朝と夕で水の番を決めたことで、両方の棚田が必要な水を受け取れた。" },
                    Ending { label: "乾く畝へ桶で運ぶ", text: "村総出で桶リレーをして苗を守り、夕方にはみんなで冷たい湧き水を飲んだ。" },
                ]),
            },
            Story {
                title: "棚田の鏡祭り",
                beats: &[
                    Beat::speech(3, "集落の世話役", Villager, "田に灯りを映す鏡祭りの日なのに、今年は田植えが遅れて水を張れない田があるんだ。"),
                    Beat::speech(2, "石段の案内人", Villager, "空いている一番下の段に灯りをまとめるか、水抜きのあとの窪地に水を張って小さな水鏡を作るか、どっちがいいだろう？"),
                ],
                choices: Some([
                    Ending { label: "空いた一段に灯りをまとめる", text: "一段に集まった無数の灯りが夜空を映し、例年以上に幻想的な夜になった。" },
                    Ending { label: "窪地に小さな水鏡を作る", text: "子どもたちが並べた手づくりの灯籠が水面に揺れ、穏やかな祭りの夜になった。" },
                ]),
            },
        ],
    },
    Plan {
        name: "巡礼坂の宿町",
        profile: "険しい坂の中腹に大きな円形宿を構える宿町。証人を待つ旅人の宿代は坂の積立金から賄われる。",
        terrain: Landscape::Stone,
        entry: (156, 174),
        places: [
            Place { name: "坂上の円形大宿", at: (156, 74), art: Some(Art::Landmark), ground: Landscape::Square, detail: "金色の旗が揺れる大きな円形の巡礼宿。" },
            Place { name: "中腹の荷背負い所", at: (126, 114), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "急な坂道で旅人の荷を運ぶ背負い手たちの詰め所。" },
            Place { name: "坂下の待避舎", at: (156, 146), art: Some(Art::Inn), ground: Landscape::Square, detail: "坂を上り始める前に休める小さな待避小屋。" },
            Place { name: "西斜面の空き倉庫", at: (84, 96), art: Some(Art::Cottage), ground: Landscape::Yard, detail: "少し奥まった場所にある静かな石造りの倉庫。" },
            Place { name: "円形宿の食堂廊下", at: (182, 84), art: None, ground: Landscape::Square, detail: "丸い建物に沿って鐘の音が響きやすい廊下。" },
            Place { name: "途中の励まし石段", at: (172, 126), art: None, ground: Landscape::Stone, detail: "昔の旅人が励ましの言葉を刻んだ石段の踊り場。" },
            Place { name: "荷役坂の曲がり角", at: (218, 118), art: None, ground: Landscape::Yard, detail: "車輪のある荷車も通れる緩やかな坂道。" },
            Place { name: "坂下の返却棚", at: (128, 148), art: Some(Art::Stalls), ground: Landscape::Yard, detail: "雨が上がったあと旅人が靴を乾かす棚。" },
        ],
        stories: [
            Story {
                title: "証人の寝場所",
                beats: &[
                    Beat::speech(0, "宿の寮母", Villager, "聖堂街の聴聞が延期になって、今週の巡礼者と裁判の証人が重なって宿が満員なんだよ。"),
                    Beat::speech(3, "荷背負い頭", Worker, "西の空き倉庫を掃除して人目のつかない仮宿にするか、宿の棟を分けて出入りする時間をずらすか、どっちがいいかな？"),
                ],
                choices: Some([
                    Ending { label: "空き倉庫を静かな仮宿にする", text: "証人たちは誰にも邪魔されず安心して休み、荷背負い組には坂の補修費が入った。" },
                    Ending { label: "出入りを分けて宿を融通する", text: "朝と夕で玄関を分けたことで混乱もなく、全員が温かい寝台で休めた。" },
                ]),
            },
            Story {
                title: "階段がつらい巡礼者",
                beats: &[
                    Beat::speech(2, "坂下の巡礼者", Traveler, "ここまで歩いてきたけれど、この急な石段を上り切る体力がもう残っていないよ……。"),
                    Beat::speech(1, "荷背負い", Worker, "緩やかな荷役坂のほうから車椅子を押して上がるか、私が途中まで背負ってあげようか？"),
                ],
                choices: Some([
                    Ending { label: "荷役坂で車椅子を借りる", text: "緩やかな道をゆっくり進み、景色を楽しみながら円形宿まで到着できた。" },
                    Ending { label: "背負い手に協力を頼む", text: "頼もしい荷背負いの背に揺られて、あっという間に坂上の宿へたどり着いた。" },
                ]),
            },
            Story {
                title: "厨の方向音痴",
                beats: &[
                    Beat::speech(4, "宿泊客", Traveler, "丸い建物の廊下を歩いてると、食堂の鐘が反対側から聞こえて道に迷っちまうんだ。"),
                    Beat::scene(4, "丸い壁に沿って音がぐるっと回っていたためだ。食堂が開く朝だけ小さな案内灯を角ごとに置くことにした。"),
                    Beat::scene(0, "次の朝からは誰も迷わずに食堂へ着けるようになり、温かい朝食がさらに賑やかになった。"),
                ],
                choices: None,
            },
            Story {
                title: "励ましの石段",
                beats: &[
                    Beat::speech(5, "荷背負いの孫", Villager, "おじいちゃんが昔彫った『あとひと息』っていう石段が、ひび割れて崩れそうなんだ。"),
                    Beat::speech(1, "石工", Worker, "元の石を枠で固めてそのまま残すか、言葉を新しい石に写して次の踊り場へ移すか、どうしようか？"),
                ],
                choices: Some([
                    Ending { label: "元の石を補強して残す", text: "手書きの温かみが残る石段はしっかり守られ、これからも旅人を励まし続ける。" },
                    Ending { label: "言葉を次の踊り場へ移す", text: "いちばん苦しい中腹の踊り場に新しい石が据えられ、見上げた旅人の笑顔が増えた。" },
                ]),
            },
            Story {
                title: "雨靴を返す日",
                beats: &[
                    Beat::speech(0, "宿の寮母", Villager, "雨の日に貸した長靴がなかなか戻らなくてね。坂の下から返しに上ってくるのは大変だものねえ。"),
                    Beat::scene(7, "坂下の待避舎の脇に返却棚を設けた。降りきったところで靴を返せるようになり、貸出がずっと続くようになった。"),
                    Beat::scene(2, "雨が降る日でも、坂を上り下りする旅人の足元はいつも乾いたままだった。"),
                ],
                choices: None,
            },
        ],
    },
];

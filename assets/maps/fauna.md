# 島の動物と魔獣

各島に固有の動物10種・魔獣5種を割り当てています。初期配置は各島につき動物115匹と魔獣20体で、五島には合計675個体がいます。

[共通の元アトラス](fauna.png) は最初の15種の素材です。以下は組み込み ImageGen で生成した追加60種の素材と生成プロンプトです。各地域の追加アトラスは1・2段目が動物、3段目が魔獣で、表と同じ順に左から並びます。

## ノクス

[追加アトラス](fauna/nox.png)

- 動物 — 霧アナグマ、沼イタチ、灰色ヤマウズラ、灯り蛾、青背トガリネズミ、湿原ガエル、野辺ハリネズミ、夕暮れヨタカ
- 魔獣 — 霧鐘の角獣、泥殻ガメ、墓守の梟、喰影の大蜥蜴

> Transparent PNG game sprite atlas of exactly twelve distinct creatures arranged in a strict 4 columns × 3 rows even grid of equal square cells, landscape aspect ratio 4:3. All creatures are individually isolated, fully inside their own cell, with ample actual alpha margins, zero ground or background. Center subjects, occupy only 65–72% cell size. Cohesive distinct original hand-crafted 16-bit Japanese fantasy RPG pixel art with limited palettes, sharply readable silhouettes at 17 or 24 pixels in a map and 80px battle. Three-quarter overhead facing east/right, complete body and legs. Thematic palette: fogbound heath in dusk violet and tarnished amber, wet gray fur, bleak frontier wildlife. Rows 1 and 2 are small recognizable believable nonmagical animals. Row 3 is larger, dangerous fantastical monsters in the same pixel-art style. No writing, borders, drop shadows, grid lines, or effects beyond the silhouettes. Exact left-to-right across each row:
> Row 1: 1. charcoal Eurasian badger with a cream stripe and stocky digging paws; 2. slender water weasel with umber fur, pale chest and swept tapered tail; 3. round grey upland partridge with speckled chest, rust face and short wings; 4. perched nocturnal moth with slate triangular wings and small ochre eyespots.
> Row 2: 1. slate blue-gray shrew with a sharp nose and hairless thin curved tail; 2. mottled olive bog frog with a wide yellow throat, crouched with all limbs visible; 3. little hedgehog with dark coarse spines and warm brown face; 4. ground-dwelling nightjar with streaked ashy feathers and long pointed beak.
> Row 3: 1. heavy ghostly gray elk-like quadruped, hollow brass bells hanging from broken antlers, pale violet eyes; 2. monstrous broad snapping turtle armored in purple granite and swallowed bog reeds, green eyes; 3. eerie large barn owl, stone mask and skeletal ribbed slate wings, violet under-feathers; 4. coal dark six-legged monitor lizard with split violet fin-spines and glowing dusk throat.

## セレスティア

[追加アトラス](fauna/celestia.png)

- 動物 — 白尾オコジョ、金羽チョウゲンボウ、石穴ヤマネ、断崖ヒツジ、岩棚ウサギ、麦穂ヒバリ、白崖ヤモリ、斑尾ヤマバト
- 魔獣 — 聖窟の獅子、鐘楼翼蛇、大理石の雄羊、断崖グリフォン

> Transparent PNG game sprite atlas of exactly twelve distinct creatures arranged in a strict 4 columns × 3 rows even grid of equal square cells, landscape aspect ratio 4:3. All creatures are individually isolated, fully inside their own cell, with ample actual alpha margins, zero ground or background. Center subjects, occupy only 65–72% cell size. Cohesive distinct original hand-crafted 16-bit Japanese fantasy RPG pixel art with limited palettes, sharply readable silhouettes at 17 or 24 pixels in a map and 80px battle. Three-quarter overhead facing east/right, complete body and legs. Thematic palette: white limestone cliffs and sunny olive foothills, clay red markings and gold. Rows 1 and 2 are small recognizable believable nonmagical animals. Row 3 is larger, dangerous fantastical monsters in the same pixel-art style. No writing, borders, drop shadows, grid lines, or effects beyond the silhouettes. Exact left-to-right across each row:
> Row 1: 1. slender white stoat with black tail tip, raised head; 2. small perched kestrel, warm golden speckled chest and neat slate wings; 3. small warm golden dormouse with rounded ears, sitting holding one seed; 4. bighorn sheep with ivory coat and deeply curled dark horns.
> Row 2: 1. tiny gray pika, small rounded ears and short tail; 2. small tawny crested skylark with mottled wings and upright pose; 3. bone-white gecko with adhesive toes and a loose curved tail; 4. dove with gray feathers, blue neck and barred tail.
> Row 3: 1. ivory limestone lion-sphinx with golden mane and diagonal armor on its front legs, dark blue eyes; 2. winged ivory snake coiled with blue and gilt bell-like fins and long vulnerable neck; 3. imposing white marble ram with fluted golden curled horns and luminous old-blue cracks; 4. cream feathered griffin with giant bronze talons and deep slate folded wings.

背景透過の編集プロンプト

> Edit the input sprite atlas by removing ONLY the warm white paper/background behind all twelve individual creatures. Output has true alpha transparency; exterior canvas and all gaps, including between feathers/legs/wings, must be completely alpha=0 as appropriate. Critical keep the crisp pale limestone, white fur, cream wings, golden fangs, colors and original shapes of the sprites even though light-colored. Exactly the original 4 columns and 3 rows in the same order and locations. Keep all 12 distinct creatures, no new sprites, no text, no drawn grid, no grayscale checkerboard and no white or gray backdrop. Keep sprites isolated, no floor shadows, external pixels transparent. Intended for a 16-bit fantasy JRPG.

## シルヴァ

[追加アトラス](fauna/sylva.png)

- 動物 — 青背リス、苔耳ヤマネコ、森のモモンガ、赤冠キツツキ、黄紋サンショウウオ、虹羽コガネムシ、ミドリヤマバト、樹皮カメレオン
- 魔獣 — 根絡みの大蜘蛛、琥珀翅の飛竜、樹皮の巨人、棘花カマキリ

> Transparent PNG game sprite atlas of exactly twelve distinct creatures arranged in a strict 4 columns × 3 rows even grid of equal square cells, landscape aspect ratio 4:3. All creatures are individually isolated, fully inside their own cell, with ample actual alpha margins, zero ground or background. Center subjects, occupy only 65–72% cell size. Cohesive distinct original hand-crafted 16-bit Japanese fantasy RPG pixel art with limited palettes, sharply readable silhouettes at 17 or 24 pixels in a map and 80px battle. Three-quarter overhead facing east/right, complete body and legs. Thematic palette: ancient humid green forest, ochre bark, jewel green and warm russet accents. Rows 1 and 2 are small recognizable believable nonmagical animals. Row 3 is larger, dangerous fantastical monsters in the same pixel-art style. No writing, borders, drop shadows, grid lines, or effects beyond the silhouettes. Exact left-to-right across each row:
> Row 1: 1. bright rusty tree squirrel with blue-gray stripe and thick upright tail; 2. forest lynx, ochre spotted coat, short tail, long dark ear tufts; 3. small pale flying squirrel with dark eyes and folded membranes, on its feet; 4. black and cream woodpecker with bright red crest, standing.
> Row 2: 1. black salamander with golden patches and curled tail, belly low; 2. large emerald green horned beetle, six small legs and teal iridescent wing cases; 3. forest green pigeon with salmon neck stripe and perching toes; 4. brown and vivid sage chameleon with independent eyes and spiral curled tail.
> Row 3: 1. green-brown forest spider whose eight legs resemble curled live roots, glowing amber eyes; 2. slender forest wyvern with huge golden translucent moth-like wings and green leaf scales; 3. walking knotted tree-bark humanoid with a hollow glowing mint face and gnarly roots for hands, small leafy crown; 4. giant fantasy mantis with dark green claws and bright magenta thorn-flower crest.

## リベルタ

[追加アトラス](fauna/liberta.png)

- 動物 — 潮斑アザラシ、磯潜りウ、浜走りガニ、波縞イソトカゲ、砂潜りカメ、岩笛チドリ、藻拾いアライグマ、青嘴ツノメドリ
- 魔獣 — 潮砕きオオダコ、碧角の海馬獣、潮牙ウミヘビ、珊瑚殻の歩哨

> Transparent PNG game sprite atlas of exactly twelve distinct creatures arranged in a strict 4 columns × 3 rows even grid of equal square cells, landscape aspect ratio 4:3. All creatures are individually isolated, fully inside their own cell, with ample actual alpha margins, zero ground or background. Center subjects, occupy only 65–72% cell size. Cohesive distinct original hand-crafted 16-bit Japanese fantasy RPG pixel art with limited palettes, sharply readable silhouettes at 17 or 24 pixels in a map and 80px battle. Three-quarter overhead facing east/right, complete body and legs. Thematic palette: saltwater archipelago, sea teal, ochre, shell coral and bright salt-white. Rows 1 and 2 are small recognizable believable nonmagical animals. Row 3 is larger, dangerous fantastical monsters in the same pixel-art style. No writing, borders, drop shadows, grid lines, or effects beyond the silhouettes. Exact left-to-right across each row:
> Row 1: 1. slate-spotted seal lunging on front flippers, shell-white chin; 2. dark long-necked cormorant with tiny orange throat, folded wings, standing; 3. small sandy orange ghost crab with raised eye stalks and two claws; 4. deep turquoise coastal skink with vivid ivory stripe, long tail.
> Row 2: 1. juvenile olive sea turtle on four opened flippers; 2. small white coastal plover with navy chest band and orange legs; 3. coastal raccoon with ringed tail and black mask clutching seaweed strand; 4. puffin with black and cream round body and turquoise-orange multicolored beak.
> Row 3: 1. large turquoise and rust octopus on bent rocky-ground tentacles, coral plates on head; 2. pale aqua kelpie with horse head, shell back scales and large swept fluted tail; 3. coiled dark navy and teal ribbed sea serpent with long pearl fangs and coral fins; 4. massive two-legged walking coral-shell golem with pink plates and one broad crab-like forearm.

## ヴァルカン

[追加アトラス](fauna/vulcan.png)

- 動物 — 灰走りジャコウネズミ、銅羽ワタリムクドリ、石窟コウモリ、黒砂サソリ、溶岩キツネモドキ、焼石ゴミムシ、火山ヌマガメ、岩鳴きホシガラス
- 魔獣 — 火床の大蠍、溶岩ツノウシ、炉骨サラマンダー、煤翼の不死鳥

> Transparent PNG game sprite atlas of exactly twelve distinct creatures arranged in a strict 4 columns × 3 rows even grid of equal square cells, landscape aspect ratio 4:3. All creatures are individually isolated, fully inside their own cell, with ample actual alpha margins, zero ground or background. Center subjects, occupy only 65–72% cell size. Cohesive distinct original hand-crafted 16-bit Japanese fantasy RPG pixel art with limited palettes, sharply readable silhouettes at 17 or 24 pixels in a map and 80px battle. Three-quarter overhead facing east/right, complete body and legs. Thematic palette: volcanic rock fields, rusty hematite, muted terracotta, charcoal with ember-orange accents. Rows 1 and 2 are small recognizable believable nonmagical animals. Row 3 is larger, dangerous fantastical monsters in the same pixel-art style. No writing, borders, drop shadows, grid lines, or effects beyond the silhouettes. Exact left-to-right across each row:
> Row 1: 1. small ash-gray musk shrew with reddish ears and long tapered snout; 2. dark copper starling with metallic bronze wing sheen and sharp amber eye; 3. compact cave bat crouched with folded wine-colored wings, pointed ears; 4. small coal scorpion with rusty bronze pincers and a recurved tail.
> Row 2: 1. slender fennec-like volcanic mammal with large ruddy ears and soot-gray tail; 2. dark ground beetle with vivid burnt-amber narrow shell seams, no flames; 3. low land turtle with volcanic black pebbled shell and orange flecks; 4. mountain nutcracker bird with dark chocolate white-spotted feather patterns and gray beak.
> Row 3: 1. giant obsidian scorpion with brass layered claw edges and a red-hot segmented stinger; 2. powerful black basalt bull, glossy orange lava fissures and heavy crooked horns; 3. giant auburn salamander with glowing furnace ribs, thick tail and short heavy legs; 4. fantasy phoenix settled on its talons, folded smoke-charcoal wings edged in crimson and gold embers.

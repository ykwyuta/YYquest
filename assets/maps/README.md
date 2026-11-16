# マップ素材

## YYMythology

以下の PNG は [YYMythology の `assets/map/`](https://github.com/ykwyuta/yyMythology/tree/cb813a8faf33250ded914f8343aa40a141209d2f/assets/map) から取り込みました。

| 素材 | YYquest での用途 |
| --- | --- |
| `forest.png` | 樹海、林、序章の森 |
| `mountain.png` | 断崖、岩山、火口の外縁 |
| `farm.png` | 集落の畑 |
| `fort.png` | 砦の参考素材 |
| `mine.png` | 鉱山の参考素材 |

## 組み込み ImageGen

島を歩き回る動物と魔獣は [島の動物と魔獣](fauna.md) を参照してください。

森・街道・海岸の追加素材と車両の画像は [地形と交通](terrain-traffic.md) を参照してください。

各島の街・村・廃墟65点には、別途生成した [島別のアトラス](settlements/README.md) の専用画像を使用します。

`terrain.png` は地面16種、`local-landmarks.png` は透明背景の建物と名所12種の元アトラスです。ビルド時にセルを切り出し、地面は128×128のテクスチャ、建物は透過を保ったスプライトに変換します。

### 地形の生成プロンプト

> Use case: game asset sprite atlas. Generate a square single opaque 4 columns by 4 rows grid of tileable terrain material textures for a quality 16-bit top-down pixel-art JRPG, precise equal-size square cells, no gutters and no cell outlines. Every square is an overhead flat ground repeatable sample, covering the entire square uniformly, with small and medium irregular clusters, no perspective, no directional cast shadows, no characters, no objects, no letters, no symbols. All textures drawn with chunky crisp visible pixels, sophisticated muted fantasy colors, modest contrast, conceptually readable when each cell is sampled to a 32x32 raster and seen tiled, no fine-grain noise or outlines around the squares. EXACT left to right across each row: row 1: (1) yellow-green field grass with small tuft clusters, (2) dark emerald woodland grass with patches of moss, (3) warm olive coastal grass, (4) gray-lilac volcanic/bleak heath with subtle low faded tuft clusters; row 2: (1) muted dark navy teal ocean with sparse gentle wave glints, (2) lighter peacock blue freshwater with subtle ripples, (3) pale ochre sandy shore with a few small pebbles, (4) gray olive mossy marsh with small dark teal puddles and reeds; row 3: (1) compacted medium warm tan dirt trail texture, (2) subdued ash-gray wide medieval cobblestone texture, (3) medium dark walnut horizontal worn dock planks, (4) molten orange-red lava surface with dark cooled fissures and glowing channels; row 4: (1) brown garden soil with spaced leafy amber rows of crops, (2) desaturated gray-lilac cracked rubble and old stone flagstone, (3) pale gray chalk gravel with stone chips, (4) warm dark charcoal obsidian gravel with a few rusty embedded pebbles. Exact 4-by-4 grid, distinct cells. Natural and readable, leave good visibility for small character sprites atop it. No padding, no labels, no border.

### 建物・名所の生成プロンプト

> Use case: original game sprite atlas. A true-alpha transparent atlas with EXACTLY 4 columns and 3 rows of equal square cells (canvas aspect 4:3), consistent 3/4 top-down front view with the roofs visible, hand-crafted cohesive high-quality 16-bit pixel art with a muted earthy fantasy palette, contrast for readability, chunky intentional pixels at 32px. Each object is alone and centrally placed within its own cell with large empty transparent margins. Keep objects within the middle 72% of each cell and separated from neighbors. Show each subject from foundation to roof but NO extended ground pad, no drop shadow, no caption, no lettering, no numbers, no grid lines. Pixel fidelity and evocative silhouettes. EXACT left to right across each row: ROW ONE: (1) wooden pier with posts, a small blue canvas sailboat tied to one side and a square banner above the landing, (2) compact dusk-purple slate-roof rural stone cottage with an amber lit window, (3) pale limestone gothic cathedral with tall twin cream towers and dark indigo roofs, (4) whitewashed medieval limestone inn with terra-cotta roof and a blue shutter. ROW TWO: (1) monumental magical elder tree, thick twisted russet trunk and broad green canopy, softly glowing mint motes confined inside silhouette, (2) forest village timber treehouse built around a thick old trunk, moss roof, short ladder, (3) partial ring of shattered gray standing stones with a taller central memorial stone, (4) boatbuilder's open dry-dock with a clearly recognizable ochre wooden ship hull on timber frames and a narrow shed. ROW THREE: (1) busy-looking but empty clustered pair of coastal market stalls with vivid blue-white striped canvas awnings, (2) a tall narrow stone lighthouse with warmly glowing brass lamp and teal roof, (3) bulky dark iron fantasy magma foundry, brick shoulders and two exhaust stacks, glowing orange furnace doors, (4) small copper-roofed steampunk workshop with a broad round window and short exterior pipe. Individual complete and distinguishable silhouettes, no people, actual background transparency.

### ノクスの街の施設と住民

組み込みの ImageGen で生成した [ノクスの徒歩マップ用アトラス](nox-town.png)。施設12種と住民4種の透過スプライトを収録しています。ビルド時に各セルから透明部分を除いて切り出し、アルファを維持して縮小します。

> Use case: production sprite sheet for a top-down medieval JRPG. Transparent RGBA canvas. EXACT uniform 4-column by 4-row grid of 16 isolated full sprites centered in their own cells, each occupying about 75% of cell height or width, even broad transparent gutters. Consistent 3/4 overhead north-up view with roofs and front façades visible, refined painterly miniature illustration with broad readable shapes and earthy atmospheric palette. Fine natural edges, no deliberate pixelation, no thick black outlines. Region is a cold ash-gray frontier with warm amber lamps, wood, muted violet slate and gray stone. No ground square, no surrounding landscape, no text, no numbers, no template lines, truly transparent backgrounds and outer corners. Row 1 left to right: round open-air communal hearth with warm fire and ring of stones; tall gray stone lighthouse with copper lamps; timber palisade gate with two watch platforms; stout wooden marsh sluice gate with pulley. Row 2: large timber windmill with cream sails and slate roof; old stone arch over a circular covered well; substantial roadside inn with chimney and amber lantern; tall fortified ash-stone gate with rich violet banners. Row 3: little timber and rope suspension footbridge oriented vertically north-south with visible planks; gray medieval market clock tower; two blue and ochre open market stalls; modest weathered ash-stone cottage with purple roof. Row 4: four individual full-body non-chibi people facing camera in the same 3/4 overhead viewpoint with nuanced hand-painted details, first elderly woman in shawl carrying basket, second rugged bearded docks worker with cap and apron, third frontier watch soldier with muted violet cloak and simple helmet, fourth youthful traveler with ochre cloak and small bag. Each cell must contain the specified object or person ONLY.

# 画像素材

島と序章の地形タイル、建物、樹木の出典は [マップ素材](maps/README.md) を参照してください。

## フィールドのキャラクター

組み込み ImageGen で [アレンとリリスの透過スプライト](prologue-characters.png) を生成しました。行はアレン、リリスの順で、各行は南、西、東、北の4方向です。ビルド時に切り出し、色と透過を保って縮小します。

> Use case: hand-painted top-down overworld sprites for an original 1990s Japanese high-fantasy RPG. A production two-row, four-column sprite sheet, 4:2 landscape layout, exactly eight full-body figures isolated on a genuinely TRANSPARENT canvas. Grid cells identical width and height, each character fits inside the middle 68 percent of its own cell with strictly empty wide gutters and plenty of transparent head and foot margin. The perspective is 3/4 overhead as if viewed from slightly above in a classic top-down JRPG. Visual style: beautifully painted watercolor and gouache, warm muted colors, gentle shaded volumes, lively readable human silhouettes and no exaggerated chibi heads. NOT pixel art. Figures must read at a reduced size, with mature proportions visually simplified into broad distinct shapes. Top row same character repeated in four directions: Allen Weiss, 18-year-old Frontier swordsman with chestnut hair, muted cobalt tunic, practical weathered gray and bronze shoulder armor, rust-red loose scarf and tall brown boots, simple long rusty sword carried on back. Bottom row same character repeated in four directions: Lilith, young adult long-lived forest spirit medium, pale silver hair, short pointed ears, dark teal flowing cloak with pale sage hem and tiny amber clasp, slim boots. Exact directions, in both rows, from left to right: (1) facing SOUTH toward the camera, front view; (2) facing WEST to the viewer's LEFT, strict left-facing side view with nose pointing to the left; (3) facing EAST to the viewer's RIGHT, strict right-facing side view with nose pointing to the right; (4) facing NORTH away from the camera, back of head and entire back visible with no face visible. Gentle walking stance all cells. Maintain the same height, age, costume, colors and angle between views. No floors, no scenery, no captions, no border lines or guide lines, no text, no shadows outside figures. Natural painted edges rather than pixelated edges.

> Remove the hazy cream gray background completely from every cell of this exact sprite sheet. Keep exactly the existing 4 columns by 2 rows, all eight entire individual figures, their positions, costumes and the direction of each one. Make every area outside the figures 100% alpha 0 fully transparent, including between arms and legs and between sprites. Preserve drawn colors and watercolor shading inside the opaque figures, with gentle antialiasing only in their immediate silhouette edges. Leave no white/gray matte, vignette, backdrop, translucent haze, halo, floor or shadows. Output a truly transparent PNG cutout atlas. Keep the open gutters and 2:1 aspect ratio. Do not redraw the composition or alter the existing sprite orientations.

## YYDungeon から利用した素材

| YYquest | YYDungeon | ゲーム内の表示 |
| --- | --- | --- |
| `slime.png` | [02.png](https://github.com/ykwyuta/YYDungeon/blob/master/assets/monsters/02.png) | 霧のスライム |
| `bat.png` | [03.png](https://github.com/ykwyuta/YYDungeon/blob/master/assets/monsters/03.png) | 洞窟コウモリ |
| `echo.png` | [11.png](https://github.com/ykwyuta/YYDungeon/blob/master/assets/monsters/11.png) | 忘却の獣 |

## タイトル画面

アレン、リリス、霧の列島を描いた [タイトル背景](title-arcadia.png) は組み込み ImageGen で生成しました。ビルド時に 640×480 に縮小します。

> Use case: title-screen cover illustration for the original Japanese fantasy RPG Zankyo no Arcadia. Landscape 4:3 framing. Evoke the sweeping hand-painted paperback and OVA high-fantasy illustrations of 1990s Japan, the sense of weight and tragedy in classic sword-and-sorcery like Legend of Lodoss Island. Original protagonists and setting. Hand-painted gouache and watercolor over fine pencil/ink linework, delicately worn details, high contrast with sophisticated deep midnight ultramarine, moss blue-green, violet, old bronze, and pale gold; deliberately dramatic, mature and elegant. Composition: RIGHT half holds the strong pyramidal portrait grouping. Allen Weiss, a young male chestnut-haired frontier swordsman with cobalt travel tunic under weathered practical bronze-and-steel armour and a windswept rust-red scarf, holding a tall old broadsword; Lilith, a silver-haired pointed-eared spirit medium in dark teal traveling cloak with amber clasp, slightly behind him in complementary profile. Expressive beautiful hand-drawn faces, human proportions. Their faces at around x=72–86%, y=30–44%. In the upper right distance, jagged ring-islands, a ruined Gothic cliff fortress and a pale sun illuminating wisps of sea fog; tiny soaring silhouette in the far sky. The left 53% is a calm DARK and low-detail cloud, black-blue coastline and dark fog with room for a large ivory Japanese game title and two menu lines to be added separately. Keep that area dark all the way down the left and the lower center. A glint of ember on the blade; collectible vintage fantasy cover quality. Full bleed original painting. No graphics, borders, signatures, letters, logos, faux lettering, HUD, or watermarks.

## 案内精霊

組み込み ImageGen で [案内精霊の透過画像](guide-spirit.png) を生成しました。

> Single isolated sprite for a Japanese hand-painted fantasy RPG, true transparent background. A tiny friendly guiding spirit made of luminous mint and pale gold flame, its form like a rounded little floating forest wisp with two dark eyes, two swept-back leaflike lobes, a bright readable solid jade core and three warm golden droplets curling into a short tail. Magical and ethereal yet high contrast and recognizable at 18 pixels, a clean iconic silhouette, a softly selective outline and subtle restrained glow confined close to the creature. Hand-painted finish matching watercolor and gouache fantasy book art. Front-facing three-quarter view, centered compact shape filling the canvas. No text, floor, props, border or other characters.

## 歩行差分

組み込み ImageGen で、既存の歩行アトラスを基に、[踏み出す足を切り替えた差分](prologue-walk-opposite.png)と[片足を持ち上げる差分](prologue-walk-passing.png)を作成しました。

> Match the existing transparent four-direction atlas for Allen and Lilith. For the second contact pose, reverse the lead boot and arm swing. For the passing pose, keep the planted foot under the hips and visibly lift the other boot. Keep the same costumes, facing directions, scale and watercolor style in both two-by-four sheets.

組み込み ImageGen で [歩行差分](prologue-walk.png) を生成しました。既存の4方向アトラスと組み合わせ、移動に合わせて姿勢を切り替えます。

> Create the complementary walking frame for a hand-painted RPG sprite cycle. The attached two-row by four-column transparent sprite atlas is the exact reference for costume, framing, posture, orientation, existing palette, and identity. Output a NEW 2:1 true-alpha PNG with all eight figures in the same positions and directions. Top row Allen in cobalt brown armor with red scarf, bottom row Lilith with silver hair in green cloak. Each row from left to right matches the reference: south/front, west/left profile, east/right profile, north/back. In EVERY image make a clearly different walking pose: exchange which leg leads and which trails, swing the corresponding arms the other way, move scarf and cloak subtly to match the shift in weight. Front and back must visibly shift weight onto the opposite foot, avoid static standing. Preserve face and clothes, body proportions, exact orientation, 3/4-overhead perspective, soft watercolor-gouache edges and scale. Keep large transparent gutters, eight independent silhouettes, no backdrop or drop shadows at all, fully alpha zero around and between each figure. No letters, no extra figures, no border or guides. Alternate step of the SAME walker, frame-compatible with the reference at small game size.

# 画像素材

島と序章の地形タイル、建物、樹木の出典は [マップ素材](maps/README.md) を参照してください。

アレンとリリスのフィールド用スプライトを、組み込み ImageGen で `prologue-characters.png` に生成しました。ビルド時に各キャラクターを切り出して縮小します。

## 生成プロンプト

> Transparent cutout raster sprite atlas for an original Japanese late-1980s Famicom role-playing game. Two separate full-body top-down three-quarter south-facing chibi overworld sprites in one horizontal two-cell atlas, standing near center of respective left and right half, wide empty transparent separation. Left: Allen Weiss, 18-year-old young frontier swordsman with chestnut hair, muted cobalt travel tunic, rust-red scarf, brown boots, with large rusty sword carried on back. Right: Lilith, young long-lived forest spirit medium, silver hair, short elven ears, dark teal hooded traveling cloak, pale sage hem and tiny amber clasp. Each sprite designed on a tiny 24x24 pixel grid, large hard square pixels, flat palette with deep plum outlines and at most six colors per sprite, direct pixel placement, no gradients, no lighting effects, no shadows, no ground, no props beyond clothing and sword, no text, no labels. Center both figures vertically, same height, each occupies roughly 70 percent of image height. Genuine transparent background.

## YYDungeon から利用した素材

| YYquest | YYDungeon | ゲーム内の表示 |
| --- | --- | --- |
| `slime.png` | [02.png](https://github.com/ykwyuta/YYDungeon/blob/master/assets/monsters/02.png) | 霧のスライム |
| `bat.png` | [03.png](https://github.com/ykwyuta/YYDungeon/blob/master/assets/monsters/03.png) | 洞窟コウモリ |
| `echo.png` | [11.png](https://github.com/ykwyuta/YYDungeon/blob/master/assets/monsters/11.png) | 忘却の獣 |

## タイトル画面

アレン、リリス、霧の列島を描いた [タイトル背景](title-arcadia.png) は組み込み ImageGen で生成しました。ビルド時に 640×480 に縮小します。

> Use case: title-screen cover illustration for the original Japanese fantasy RPG Zankyo no Arcadia. Landscape 4:3 framing. Evoke the sweeping hand-painted paperback and OVA high-fantasy illustrations of 1990s Japan, the sense of weight and tragedy in classic sword-and-sorcery like Legend of Lodoss Island. Original protagonists and setting. Hand-painted gouache and watercolor over fine pencil/ink linework, delicately worn details, high contrast with sophisticated deep midnight ultramarine, moss blue-green, violet, old bronze, and pale gold; deliberately dramatic, mature and elegant. Composition: RIGHT half holds the strong pyramidal portrait grouping. Allen Weiss, a young male chestnut-haired frontier swordsman with cobalt travel tunic under weathered practical bronze-and-steel armour and a windswept rust-red scarf, holding a tall old broadsword; Lilith, a silver-haired pointed-eared spirit medium in dark teal traveling cloak with amber clasp, slightly behind him in complementary profile. Expressive beautiful hand-drawn faces, human proportions. Their faces at around x=72–86%, y=30–44%. In the upper right distance, jagged ring-islands, a ruined Gothic cliff fortress and a pale sun illuminating wisps of sea fog; tiny soaring silhouette in the far sky. The left 53% is a calm DARK and low-detail cloud, black-blue coastline and dark fog with room for a large ivory Japanese game title and two menu lines to be added separately. Keep that area dark all the way down the left and the lower center. A glint of ember on the blade; collectible vintage fantasy cover quality. Full bleed original painting. No graphics, borders, signatures, letters, logos, faux lettering, HUD, or watermarks.

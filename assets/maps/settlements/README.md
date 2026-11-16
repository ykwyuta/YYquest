# 街・村・廃墟の画像

組み込み ImageGen で島ごとに透過アトラスを生成しました。各アトラスは左上から行順に、街・村10点と廃村・廃墟3点を収録しています。各地に元画像から縮小したスプライトを1枚割り当て、ゲーム内では村48、町56、主要都市68、廃墟42ピクセル四方の、複数タイルにまたがるアイコンとして描きます。画像の前にある入口で名称と説明が表示されます。

## Nox

[nox.png](nox.png)

1. 灰灯りの村
2. 霧灯の港町
3. 西風の砦町
4. 葦渡りの村
5. オオ麦の里
6. 石継ぎの町
7. 遠火の宿場
8. ノクス北門街
9. 谷守の村
10. 灰石の市
11. 名のない慰霊碑
12. 煤けた旧村
13. 天墜ち砦跡

### 生成プロンプト

> Original professional 16-bit JRPG game sprite atlas on genuinely transparent alpha, perfectly regular 4 by 4 equally sized square cells, square canvas, fixed 3/4 top-down angle and matching central position, no terrain squares, no ground plane beyond a narrow shared foundation footprint, no grid or margins drawn, no humans, no lettering. Every populated cell is ONE isolated settlement cluster or ruined complex treated as a single playable map icon, no repeated architecture between cells, readable silhouettes and front doors, clear textured but chunky pixels, designed to remain recognizable when sampled to 48-64px. Keep each sprite strictly in the central 74% of its cell with wide empty transparent space on all four sides. Buildings in one cell should read as one connected cluster. Draw exactly the following in this order. The bottom row's final three cells must be blank transparent.
> Regional art direction for Nox: fogbound violet heath, gray fieldstone, slate and weathered timber, quiet amber hearths of a frontier realm.
> Row 1: cell 1 three slate-roof farm cottages encircling a large lit communal hearth; cell 2 small stone harbor town with an amber signal tower and low slate warehouses; cell 3 frontier trading town behind a timber palisade, a squat gray watchtower; cell 4 reed-thatch village with plank walkways, a distinctive round communal house.
> Row 2: cell 1 frontier hamlet with one windmill and its walled grain store; cell 2 gray stone houses around a crooked stone arch and a roadside well; cell 3 roadside village dominated by a broad two-story inn and covered stable; cell 4 densely packed slate buildings framing an imposing gatehouse with a faded violet flag.
> Row 3: cell 1 fortified river valley settlement built around a long timber hall with pitched roofs; cell 2 market town with a round slate clock tower and burgundy market canopies; cell 3 ruined village square with a tall worn memorial amid collapsed homes, no lettering; cell 4 abandoned hamlet with charred frames, empty hearths and a broken stone chimney.
> Row 4: cell 1 collapsed circular cliffside fort, broken battlements around an open courtyard; cells 2, 3 and 4 completely empty transparent.

## Celestia

[celestia.png](celestia.png)

1. 白帆港町
2. クロノス聖堂街
3. 白壁の砦町
4. 麦穂の宿場
5. 湖鏡の町
6. 鐘草の村
7. 聖律門の市
8. 誓約の修道村
9. 段丘の集落
10. 巡礼坂の宿町
11. 湖畔の旧記録庫
12. 欠け鐘の廃村
13. 白階段の聖堂跡

### 生成プロンプト

> Original professional 16-bit JRPG game sprite atlas on genuinely transparent alpha, perfectly regular 4 by 4 equally sized square cells, square canvas, fixed 3/4 top-down angle and matching central position, no terrain squares, no ground plane beyond a narrow shared foundation footprint, no grid or margins drawn, no humans, no lettering. Every populated cell is ONE isolated settlement cluster or ruined complex treated as a single playable map icon, no repeated architecture between cells, readable silhouettes and front doors, clear textured but chunky pixels, designed to remain recognizable when sampled to 48-64px. Keep each sprite strictly in the central 74% of its cell with wide empty transparent space on all four sides. Buildings in one cell should read as one connected cluster. Draw exactly the following in this order. The bottom row's final three cells must be blank transparent.
> Regional art direction for Celestia: white limestone and ivory walls, deep blue slate, muted gold pinnacles and terracotta on modest homes, a cliffside ecclesiastical realm.
> Row 1: cell 1 white limestone port town, blue-roofed customs house and stacked harbor warehouses; cell 2 cathedral city with one enormous ivory twin-spired Gothic cathedral above clustered blue-roof homes; cell 3 pale cliff fortress with stepped curtain wall and a compact row of carmine roofs; cell 4 whitewashed farming village dominated by a tall working stone windmill, wheat-colored roofs.
> Row 2: cell 1 elegant scholarly lakeside town with an archive rotunda and arched colonnade; cell 2 tiny white cottages gathered around one blue onion-shaped chapel bell turret; cell 3 cliff trading town built around monumental roofed white city gate and blue stalls; cell 4 quadrangular limestone cloister with a tiny central garden, flanked by roofs.
> Row 3: cell 1 three stepped white stone cottages with ochre tile roofs and an exterior stair; cell 2 cluster of half-timber inns around a central circular pilgrim hall with yellow banners; cell 3 derelict ivory archive, broken round dome and fallen stone shelves visible; cell 4 broken pale homes and a cracked roofless bell tower with fallen bell.
> Row 4: cell 1 large roofless church with tall white arches, broken pews and crumbling stairs; cells 2, 3 and 4 completely empty transparent.

## Sylva

[sylva.png](sylva.png)

1. 梢の集落
2. 葉影の港村
3. 根渡りの里
4. 月見の里
5. 雫葉の集落
6. 精霊樹の門前村
7. 苔舟の町
8. 山桑の里
9. 古枝の工房村
10. 若芽の里
11. 古根の封門
12. 眠り枝の廃村
13. 月石庭園跡

### 生成プロンプト

> Original professional 16-bit JRPG game sprite atlas on genuinely transparent alpha, perfectly regular 4 by 4 equally sized square cells, square canvas, fixed 3/4 top-down angle and matching central position, no terrain squares, no ground plane beyond a narrow shared foundation footprint, no grid or margins drawn, no humans, no lettering. Every populated cell is ONE isolated settlement cluster or ruined complex treated as a single playable map icon, no repeated architecture between cells, readable silhouettes and front doors, clear textured but chunky pixels, designed to remain recognizable when sampled to 48-64px. Keep each sprite strictly in the central 74% of its cell with wide empty transparent space on all four sides. Buildings in one cell should read as one connected cluster. Draw exactly the following in this order. The bottom row's final three cells must be blank transparent.
> Regional art direction for Sylva: ancient green forest civilization, ochre timber houses, sage roofs, russet bark and warm lanterns; platforms and ladders within silhouettes.
> Row 1: cell 1 twin gigantic tree trunks carrying a connected village of three timber treehouses; cell 2 wooden plank forest harbor village with leafy boat-house and waterside homes; cell 3 timber homes tucked inside arched exposed tree roots, a pale stone footbridge; cell 4 woodland village with a crescent-shaped observation platform above mossy low roofs.
> Row 2: cell 1 small cluster of green shingle houses set on raised stilts and a ladder; cell 2 tiny elven houses forming an arc beside a tall standing lantern tree; cell 3 timber ferry village with one high curved boathouse and two moss cottages; cell 4 rustic cluster of gingerbread timber homes with light amber roof gables and a communal grove.
> Row 3: cell 1 woodland woodworking village with an open saw shed, suspended timber walkways and watch perch; cell 2 three round elven huts with overlapping leaf roofs and green lanterns; cell 3 abandoned monumental stone gate wrapped by thick mossy roots, damaged pylons; cell 4 abandoned tree village with rotten collapsed walkways and empty exposed roof frames.
> Row 4: cell 1 collapsed ancient moon observatory, pale concentric standing stones and roots; cells 2, 3 and 4 completely empty transparent.

## Liberta

[liberta.png](liberta.png)

1. 潮待ちの港町
2. 帆布の市場街
3. 運河の船大工街
4. 三潮灯台町
5. 網干しの村
6. 塩風の集落
7. 碧帆の町
8. 潮路の宿場
9. 小舟の里
10. 燈魚の市
11. 帆隠しの旧港
12. 沈み桟橋の廃村
13. 潮裂きの砦跡

### 生成プロンプト

> Original professional 16-bit JRPG game sprite atlas on genuinely transparent alpha, perfectly regular 4 by 4 equally sized square cells, square canvas, fixed 3/4 top-down angle and matching central position, no terrain squares, no ground plane beyond a narrow shared foundation footprint, no grid or margins drawn, no humans, no lettering. Every populated cell is ONE isolated settlement cluster or ruined complex treated as a single playable map icon, no repeated architecture between cells, readable silhouettes and front doors, clear textured but chunky pixels, designed to remain recognizable when sampled to 48-64px. Keep each sprite strictly in the central 74% of its cell with wide empty transparent space on all four sides. Buildings in one cell should read as one connected cluster. Draw exactly the following in this order. The bottom row's final three cells must be blank transparent.
> Regional art direction for Liberta: seafaring archipelago, muted turquoise and salt-white walls, dark naval timber, copper, ochre shingles, distinct sails and fishing props.
> Row 1: cell 1 port town cluster with a blue crane, attached warehouse and moored fishing boat; cell 2 busy market village with three striped blue-white canvas stalls and adjoining town houses; cell 3 boatbuilding quarter centered on a dry dock with a large unfinished ochre hull and surrounding sheds; cell 4 lighthouse town with a large stone teal-roofed lighthouse and cluster of fisher cottages.
> Row 2: cell 1 three simple coastal cottages connected by drying fishing nets and stilt decks; cell 2 salt-worker village of low white buildings with teal shutters and a wind-powered salt mill; cell 3 harbor district centered on an imposing merchant guild hall with a single turquoise sail canopy; cell 4 two tile-roof hostels framing a covered footbridge and timber market platform.
> Row 3: cell 1 compact fishermen's village on one pier, with a few colorful roof cabins and boat racks; cell 2 compact merchant town clustered around a bulbous amber beacon, ochre and red roof contrast; cell 3 abandoned inlet village and broken pier surrounding dark mystery crates; cell 4 collapsed stilt cottages leaning over a wrecked wooden boardwalk.
> Row 4: cell 1 abandoned battered coastal bastion with empty watch post and shattered blue flagstone; cells 2, 3 and 4 completely empty transparent.

## Vulcan

[vulcan.png](vulcan.png)

1. 黒鉄港町
2. 歯車街
3. 赤炉工区
4. 火口の見張り町
5. 煤坂の村
6. 蒸気弁の集落
7. 鎚音の町
8. 炉守の宿場
9. 黄銅の工房村
10. 黒曜の市場町
11. 旧魔鉱坑
12. 灰埋もれの廃村
13. 折れ炉の工場跡

### 生成プロンプト

> Original professional 16-bit JRPG game sprite atlas on genuinely transparent alpha, perfectly regular 4 by 4 equally sized square cells, square canvas, fixed 3/4 top-down angle and matching central position, no terrain squares, no ground plane beyond a narrow shared foundation footprint, no grid or margins drawn, no humans, no lettering. Every populated cell is ONE isolated settlement cluster or ruined complex treated as a single playable map icon, no repeated architecture between cells, readable silhouettes and front doors, clear textured but chunky pixels, designed to remain recognizable when sampled to 48-64px. Keep each sprite strictly in the central 74% of its cell with wide empty transparent space on all four sides. Buildings in one cell should read as one connected cluster. Draw exactly the following in this order. The bottom row's final three cells must be blank transparent.
> Regional art direction for Vulcan: volcanic industrial realm, charcoal basalt and warm copper, terracotta brick, tarnished brass, warm amber furnaces, intricate readable silhouettes.
> Row 1: cell 1 heavy steampunk port cluster with a steam crane and dark metal warehouses; cell 2 dense industrial town dominated by a large copper-roof factory, round gear window and pipes; cell 3 foundry city with two prominent chimneys, bright roaring brick furnaces and attached workshops; cell 4 hillside company town with a tall riveted observation tower, fenced gantry and soot-red roofs.
> Row 2: cell 1 stone workers village with four low steep red roofs staggered down masonry stairs; cell 2 three domed homes grouped around a visible brass steam valve and pipes; cell 3 forge town centered on a large exterior trip-hammer and paired copper-roof sheds; cell 4 roadside workers inn with massive brick chimney and clustered dark shelters.
> Row 3: cell 1 copper artisans' village of three workshops and circular amber clock tower; cell 2 black masonry market district with angular red awnings and a heated brass fountain; cell 3 abandoned mining settlement with collapsed headframe, boarded shaft and derelict rail carts; cell 4 surviving roofless brick houses amid partially buried burnt iron framework.
> Row 4: cell 1 factory ruins with two cracked, cold extinguished brick furnaces and crumpled vent pipes; cells 2, 3 and 4 completely empty transparent.

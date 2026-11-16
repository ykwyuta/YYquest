use super::{Habitat::*, Species, SpeciesData, SPECIES_PER_ISLAND};
use crate::world_map::Region;

pub(super) fn get(species: Species) -> &'static SpeciesData {
    let regional = match species.region {
        Region::Nox => &NOX,
        Region::Celestia => &CELESTIA,
        Region::Sylva => &SYLVA,
        Region::Liberta => &LIBERTA,
        Region::Vulcan => &VULCAN,
    };
    &regional[species.index]
}

const NOX: [SpeciesData; SPECIES_PER_ISLAND] = [
    SpeciesData::animal("ノウサギ", Wetland),
    SpeciesData::animal("ハイイロガラス", Open),
    SpeciesData::animal("霧アナグマ", Open),
    SpeciesData::animal("沼イタチ", Wetland),
    SpeciesData::animal("灰色ヤマウズラ", Open),
    SpeciesData::animal("灯り蛾", Farmland),
    SpeciesData::animal("青背トガリネズミ", Open),
    SpeciesData::animal("湿原ガエル", Wetland),
    SpeciesData::animal("野辺ハリネズミ", Woodland),
    SpeciesData::animal("夕暮れヨタカ", Open),
    SpeciesData::beast("霧牙狼", Wetland, 18, 3),
    SpeciesData::beast("霧鐘の角獣", Open, 24, 4),
    SpeciesData::beast("泥殻ガメ", Wetland, 28, 3),
    SpeciesData::beast("墓守の梟", Stone, 20, 5),
    SpeciesData::beast("喰影の大蜥蜴", Wetland, 24, 4),
];

const CELESTIA: [SpeciesData; SPECIES_PER_ISLAND] = [
    SpeciesData::animal("岩ヤギ", Stone),
    SpeciesData::animal("アカシカ", Farmland),
    SpeciesData::animal("白尾オコジョ", Stone),
    SpeciesData::animal("金羽チョウゲンボウ", Open),
    SpeciesData::animal("石穴ヤマネ", Woodland),
    SpeciesData::animal("断崖ヒツジ", Stone),
    SpeciesData::animal("岩棚ウサギ", Stone),
    SpeciesData::animal("麦穂ヒバリ", Farmland),
    SpeciesData::animal("白崖ヤモリ", Stone),
    SpeciesData::animal("斑尾ヤマバト", Woodland),
    SpeciesData::beast("白岩のガーゴイル", Stone, 24, 4),
    SpeciesData::beast("聖窟の獅子", Stone, 30, 5),
    SpeciesData::beast("鐘楼翼蛇", Stone, 24, 5),
    SpeciesData::beast("大理石の雄羊", Stone, 32, 4),
    SpeciesData::beast("断崖グリフォン", Open, 32, 6),
];

const SYLVA: [SpeciesData; SPECIES_PER_ISLAND] = [
    SpeciesData::animal("アカギツネ", Woodland),
    SpeciesData::animal("イノシシ", Woodland),
    SpeciesData::animal("青背リス", Woodland),
    SpeciesData::animal("苔耳ヤマネコ", Woodland),
    SpeciesData::animal("森のモモンガ", Woodland),
    SpeciesData::animal("赤冠キツツキ", Woodland),
    SpeciesData::animal("黄紋サンショウウオ", Wetland),
    SpeciesData::animal("虹羽コガネムシ", Woodland),
    SpeciesData::animal("ミドリヤマバト", Woodland),
    SpeciesData::animal("樹皮カメレオン", Woodland),
    SpeciesData::beast("棘背の大猪", Woodland, 22, 4),
    SpeciesData::beast("根絡みの大蜘蛛", Woodland, 24, 5),
    SpeciesData::beast("琥珀翅の飛竜", Woodland, 28, 5),
    SpeciesData::beast("樹皮の巨人", Woodland, 34, 4),
    SpeciesData::beast("棘花カマキリ", Wetland, 24, 6),
];

const LIBERTA: [SpeciesData; SPECIES_PER_ISLAND] = [
    SpeciesData::animal("カモメ", Shore),
    SpeciesData::animal("ラッコ", Shore),
    SpeciesData::animal("潮斑アザラシ", Shore),
    SpeciesData::animal("磯潜りウ", Shore),
    SpeciesData::animal("浜走りガニ", Shore),
    SpeciesData::animal("波縞イソトカゲ", Stone),
    SpeciesData::animal("砂潜りカメ", Shore),
    SpeciesData::animal("岩笛チドリ", Shore),
    SpeciesData::animal("藻拾いアライグマ", Woodland),
    SpeciesData::animal("青嘴ツノメドリ", Shore),
    SpeciesData::beast("渦潮ヤドカリ", Shore, 24, 4),
    SpeciesData::beast("潮砕きオオダコ", Shore, 28, 5),
    SpeciesData::beast("碧角の海馬獣", Shore, 24, 5),
    SpeciesData::beast("潮牙ウミヘビ", Shore, 28, 6),
    SpeciesData::beast("珊瑚殻の歩哨", Stone, 34, 4),
];

const VULCAN: [SpeciesData; SPECIES_PER_ISLAND] = [
    SpeciesData::animal("黒イグアナ", Stone),
    SpeciesData::animal("岩アルマジロ", Stone),
    SpeciesData::animal("灰走りジャコウネズミ", Open),
    SpeciesData::animal("銅羽ワタリムクドリ", Open),
    SpeciesData::animal("石窟コウモリ", Stone),
    SpeciesData::animal("黒砂サソリ", Stone),
    SpeciesData::animal("溶岩キツネモドキ", Open),
    SpeciesData::animal("焼石ゴミムシ", Stone),
    SpeciesData::animal("火山ヌマガメ", Wetland),
    SpeciesData::animal("岩鳴きホシガラス", Stone),
    SpeciesData::beast("熔火の竜", Stone, 32, 6),
    SpeciesData::beast("火床の大蠍", Stone, 28, 5),
    SpeciesData::beast("溶岩ツノウシ", Stone, 34, 6),
    SpeciesData::beast("炉骨サラマンダー", Stone, 30, 5),
    SpeciesData::beast("煤翼の不死鳥", Open, 36, 5),
];

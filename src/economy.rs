use crate::{game::MAX_HP, world_map::Region};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) enum Item {
    RustySword,
    AshHerb,
    BitterRoot,
    BogIron,
    AshWood,
    BeastHide,
    EchoShard,
    Poultice,
    Tonic,
    SparkTube,
    IronSword,
    EchoBlade,
    LeatherCoat,
    RivetedCoat,
    MistCharm,
}

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    Material,
    Heal(u8),
    Spell(u8),
    Weapon(u8),
    Armor(u8),
    Charm(u8),
}

impl Item {
    pub(crate) const ALL: [Self; 15] = {
        use Item::*;
        [
            RustySword,
            AshHerb,
            BitterRoot,
            BogIron,
            AshWood,
            BeastHide,
            EchoShard,
            Poultice,
            Tonic,
            SparkTube,
            IronSword,
            EchoBlade,
            LeatherCoat,
            RivetedCoat,
            MistCharm,
        ]
    };

    pub(crate) fn data(self) -> (&'static str, Kind, u32) {
        use Item::*;
        use Kind::*;
        match self {
            RustySword => ("父の錆びた大剣", Weapon(0), 0),
            AshHerb => ("灰葉草", Material, 4),
            BitterRoot => ("苦霧根", Material, 8),
            BogIron => ("沼鉄鉱", Material, 10),
            AshWood => ("灰樹の枝", Material, 6),
            BeastHide => ("魔獣のなめし革", Material, 12),
            EchoShard => ("残響の欠片", Material, 18),
            Poultice => ("灰葉の傷薬", Heal(8), 10),
            Tonic => ("苦霧の煎じ薬", Heal(16), 22),
            SparkTube => ("火花筒", Spell(12), 24),
            IronSword => ("沼鉄の片手剣", Weapon(2), 0),
            EchoBlade => ("残響鋼の剣", Weapon(4), 0),
            LeatherCoat => ("革の旅装", Armor(1), 0),
            RivetedCoat => ("鋲留めの胴鎧", Armor(2), 0),
            MistCharm => ("霧灯の護符", Charm(1), 0),
        }
    }

    pub(crate) fn name(self) -> &'static str {
        self.data().0
    }

    pub(crate) fn description(self) -> String {
        match self.data().1 {
            Kind::Material => "調薬や鍛冶の素材".into(),
            Kind::Heal(hp) => format!("アレンのHPを {hp} 回復"),
            Kind::Spell(power) => format!("戦闘用：記憶を使わずに {power} ダメージ"),
            Kind::Weapon(power) => format!("通常攻撃力 {}", 6 + power),
            Kind::Armor(guard) => format!("受けるダメージを {guard} 軽減"),
            Kind::Charm(guard) => format!("受けるダメージをさらに {guard} 軽減"),
        }
    }
}

pub(crate) struct Recipe {
    pub(crate) output: Item,
    pub(crate) fee: u32,
    pub(crate) needs: &'static [(Item, u8)],
}

use Item::*;

pub(crate) const BREW: &[Recipe] = &[
    Recipe {
        output: Poultice,
        fee: 0,
        needs: &[(AshHerb, 2)],
    },
    Recipe {
        output: Tonic,
        fee: 4,
        needs: &[(AshHerb, 1), (BitterRoot, 2)],
    },
];

pub(crate) const FORGE: &[Recipe] = &[
    Recipe {
        output: IronSword,
        fee: 18,
        needs: &[(BogIron, 2), (AshWood, 1)],
    },
    Recipe {
        output: EchoBlade,
        fee: 42,
        needs: &[(BogIron, 3), (EchoShard, 2)],
    },
    Recipe {
        output: LeatherCoat,
        fee: 12,
        needs: &[(BeastHide, 2), (AshWood, 1)],
    },
    Recipe {
        output: RivetedCoat,
        fee: 26,
        needs: &[(BogIron, 2), (BeastHide, 2)],
    },
    Recipe {
        output: MistCharm,
        fee: 16,
        needs: &[(AshWood, 1), (EchoShard, 1)],
    },
    Recipe {
        output: SparkTube,
        fee: 6,
        needs: &[(BogIron, 1), (EchoShard, 1)],
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Service {
    Shop,
    Brew,
    Forge,
    Rest(u32),
}

impl Service {
    pub(crate) fn at(town: usize, place: usize) -> Option<Self> {
        Some(match (town, place) {
            (0, 0) | (8, 2) => Self::Rest(0),
            (6, 0) => Self::Rest(8),
            (1, 1) | (2, 1) | (9, 2) => Self::Shop,
            (0, 7) => Self::Brew,
            (5, 1) => Self::Forge,
            _ => return None,
        })
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Shop => "よろず屋で買い物",
            Self::Brew => "傷薬を調合する",
            Self::Forge => "鍛冶屋で装備を作る",
            Self::Rest(0) => "無料で手当てを受ける",
            Self::Rest(_) => "旅籠で休む",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Bag,
    Battle,
    Shop,
    Buy,
    Sell,
    Brew,
    Forge,
    Rest(u32),
}

#[derive(Clone, Copy)]
pub(crate) enum OptionRow {
    BuyList,
    SellList,
    Buy(Item),
    Sell(Item),
    Item(Item),
    Craft(&'static Recipe),
    Rest(u32),
}

impl OptionRow {
    pub(crate) fn item(self) -> Option<Item> {
        match self {
            Self::Buy(item) | Self::Sell(item) | Self::Item(item) => Some(item),
            Self::Craft(recipe) => Some(recipe.output),
            _ => None,
        }
    }
}

pub(crate) struct Menu {
    pub(crate) page: Page,
    pub(crate) selected: usize,
    pub(crate) note: String,
    pub(crate) return_to: Option<Page>,
}

impl Menu {
    pub(crate) fn new(page: Page) -> Self {
        Self {
            page,
            selected: 0,
            note: String::new(),
            return_to: None,
        }
    }

    pub(crate) fn service(service: Service) -> Self {
        Self::new(match service {
            Service::Shop => Page::Shop,
            Service::Brew => Page::Brew,
            Service::Forge => Page::Forge,
            Service::Rest(cost) => Page::Rest(cost),
        })
    }

    pub(crate) fn rows(&self, goods: &Goods) -> Vec<OptionRow> {
        use OptionRow::*;
        let held = || {
            crate::economy::Item::ALL
                .into_iter()
                .filter(|&item| goods.count(item) > 0)
        };
        match self.page {
            Page::Bag => held().map(Item).collect(),
            Page::Battle => held()
                .filter(|i| matches!(i.data().1, Kind::Heal(_) | Kind::Spell(_)))
                .map(Item)
                .collect(),
            Page::Shop => vec![BuyList, SellList],
            Page::Buy => [
                Poultice, Tonic, SparkTube, AshHerb, BitterRoot, AshWood, BogIron, BeastHide,
            ]
            .into_iter()
            .map(Buy)
            .collect(),
            Page::Sell => held()
                .filter(|i| matches!(i.data().1, Kind::Material))
                .map(Sell)
                .collect(),
            Page::Brew => BREW.iter().map(Craft).collect(),
            Page::Forge => FORGE.iter().map(Craft).collect(),
            Page::Rest(cost) => vec![Rest(cost)],
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Goods {
    pub(crate) coins: u32,
    pub(crate) hp: u8,
    stock: [u8; Item::ALL.len()],
    weapon: Item,
    armor: Option<Item>,
    charm: Option<Item>,
    steps: u64,
    picked_at: [Option<u64>; Item::ALL.len()],
    rewarded: HashSet<(Region, usize)>,
}

impl Goods {
    pub(crate) fn valid(&self) -> bool {
        let equipped = |item, kind: fn(Kind) -> bool| self.count(item) == 1 && kind(item.data().1);
        self.hp > 0
            && self.hp <= MAX_HP
            && self.count(RustySword) == 1
            && Item::ALL.into_iter().all(|item| {
                let limit = match item.data().1 {
                    Kind::Weapon(_) | Kind::Armor(_) | Kind::Charm(_) => 1,
                    _ => 99,
                };
                self.count(item) <= limit
            })
            && equipped(self.weapon, |kind| matches!(kind, Kind::Weapon(_)))
            && self.armor.is_none_or(|item| equipped(item, |kind| matches!(kind, Kind::Armor(_))))
            && self.charm.is_none_or(|item| equipped(item, |kind| matches!(kind, Kind::Charm(_))))
            && self.picked_at.iter().flatten().all(|&step| step <= self.steps)
            && self.rewarded.iter().all(|&(_, index)| index < 135)
    }

    pub(crate) fn new() -> Self {
        let mut goods = Self {
            coins: 24,
            hp: MAX_HP,
            stock: Default::default(),
            weapon: RustySword,
            armor: None,
            charm: None,
            steps: 0,
            picked_at: Default::default(),
            rewarded: HashSet::new(),
        };
        goods.stock[RustySword as usize] = 1;
        goods.stock[Poultice as usize] = 2;
        goods
    }

    pub(crate) fn count(&self, item: Item) -> u8 {
        self.stock[item as usize]
    }

    pub(crate) fn equipped(&self, item: Item) -> bool {
        self.weapon == item || self.armor == Some(item) || self.charm == Some(item)
    }

    pub(crate) fn attack(&self) -> u8 {
        match self.weapon.data().1 {
            Kind::Weapon(bonus) => 6 + bonus,
            _ => unreachable!(),
        }
    }

    pub(crate) fn defense(&self) -> u8 {
        [self.armor, self.charm]
            .into_iter()
            .flatten()
            .map(|item| match item.data().1 {
                Kind::Armor(value) | Kind::Charm(value) => value,
                _ => unreachable!(),
            })
            .sum()
    }

    pub(crate) fn earn(&mut self, coins: u32) {
        self.coins = self.coins.saturating_add(coins);
    }

    pub(crate) fn battle_reward(&mut self, region: Region, id: usize, enemy_hp: u8) -> String {
        if !self.rewarded.insert((region, id)) {
            return "魔獣を退けた。".into();
        }
        let (coins, item) = if enemy_hp >= 20 {
            (14, EchoShard)
        } else {
            (9, BeastHide)
        };
        self.earn(coins);
        if self.add(item, 1).is_ok() {
            format!("{coins}銀環と{}を得た。", item.name())
        } else {
            format!("{coins}銀環を得た。素材袋は満杯。")
        }
    }

    pub(crate) fn walked(&mut self) {
        self.steps = self.steps.saturating_add(1);
    }

    pub(crate) fn gather_wait(&self, item: Item) -> u64 {
        self.picked_at[item as usize].map_or(0, |step| 96_u64.saturating_sub(self.steps - step))
    }

    pub(crate) fn gather(&mut self, item: Item, quantity: u8) -> Result<(), &'static str> {
        if self.gather_wait(item) != 0 {
            return Err("まだ採集できない。");
        }
        self.add(item, quantity)?;
        self.picked_at[item as usize] = Some(self.steps);
        Ok(())
    }

    fn add(&mut self, item: Item, quantity: u8) -> Result<(), &'static str> {
        let count = &mut self.stock[item as usize];
        if *count + quantity > 99 {
            return Err("これ以上は持てない。");
        }
        *count += quantity;
        Ok(())
    }

    fn pay(&self, cost: u32) -> Result<(), &'static str> {
        if self.coins < cost {
            Err("銀環が足りない。")
        } else {
            Ok(())
        }
    }

    pub(crate) fn act(&mut self, row: OptionRow) -> Result<String, &'static str> {
        use OptionRow::*;
        Ok(match row {
            Buy(item) => {
                let cost = item.data().2;
                self.pay(cost)?;
                self.add(item, 1)?;
                self.coins -= cost;
                format!("{}を買った。", item.name())
            }
            Sell(item) => {
                if self.count(item) == 0 || !matches!(item.data().1, Kind::Material) {
                    return Err("売れる素材がない。");
                }
                self.stock[item as usize] -= 1;
                let price = item.data().2 / 2;
                self.earn(price);
                format!("{}を売り、{price}銀環を得た。", item.name())
            }
            Craft(recipe) => {
                let item = recipe.output;
                if matches!(
                    item.data().1,
                    Kind::Weapon(_) | Kind::Armor(_) | Kind::Charm(_)
                ) && self.count(item) > 0
                {
                    return Err("この装備はすでに持っている。");
                }
                self.pay(recipe.fee)?;
                if recipe
                    .needs
                    .iter()
                    .any(|&(material, quantity)| self.count(material) < quantity)
                {
                    return Err("必要な素材が足りない。");
                }
                self.add(item, 1)?;
                self.coins -= recipe.fee;
                for &(material, quantity) in recipe.needs {
                    self.stock[material as usize] -= quantity;
                }
                format!("{}ができた。", item.name())
            }
            Rest(cost) => {
                if self.hp == MAX_HP {
                    return Err("体力は満タンだ。");
                }
                self.pay(cost)?;
                self.coins -= cost;
                self.hp = MAX_HP;
                "体力が全回復した。".into()
            }
            Item(item) => {
                match item.data().1 {
                    Kind::Weapon(_) => self.weapon = item,
                    Kind::Armor(_) => self.armor = Some(item),
                    Kind::Charm(_) => self.charm = Some(item),
                    Kind::Heal(_) => {
                        let mut hp = self.hp;
                        self.use_item(item, &mut hp)?;
                        self.hp = hp;
                        return Ok(format!("{}を使った。", item.name()));
                    }
                    Kind::Spell(_) => return Err("戦闘中に使う魔道具だ。"),
                    Kind::Material => return Err("素材は店や工房で使える。"),
                }
                format!("{}を装備した。", item.name())
            }
            BuyList | SellList => unreachable!(),
        })
    }

    pub(crate) fn use_item(&mut self, item: Item, hp: &mut u8) -> Result<u8, &'static str> {
        if self.count(item) == 0 {
            return Err("道具を持っていない。");
        }
        let damage = match item.data().1 {
            Kind::Heal(_) if *hp == MAX_HP => return Err("体力は満タンだ。"),
            Kind::Heal(amount) => {
                *hp = hp.saturating_add(amount).min(MAX_HP);
                0
            }
            Kind::Spell(amount) => amount,
            _ => return Err("戦闘で使える道具を選んでください。"),
        };
        self.stock[item as usize] -= 1;
        Ok(damage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harvesting_replenishes_after_walking_and_keeps_full_bag_plants() {
        let mut goods = Goods::new();
        goods.stock[AshHerb as usize] = 98;
        assert!(goods.gather(AshHerb, 3).is_err());
        assert_eq!((goods.count(AshHerb), goods.gather_wait(AshHerb)), (98, 0));

        goods.stock[AshHerb as usize] = 96;
        assert!(goods.gather(AshHerb, 3).is_ok());
        assert_eq!((goods.count(AshHerb), goods.gather_wait(AshHerb)), (99, 96));
        assert!(goods.gather(AshHerb, 3).is_err());
        for _ in 0..96 {
            goods.walked();
        }
        assert_eq!(goods.gather_wait(AshHerb), 0);
    }
}

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "build:8")]
    pub build: String,
    #[serde(rename = "format:8")]
    pub format: String,
    #[serde(rename = "questDatabase:9")]
    pub quest_database: HashMap<String, Quest>,
    #[serde(rename = "questLines:9")]
    pub quest_lines: HashMap<String, QuestLine>,
    #[serde(rename = "questSettings:10")]
    pub quest_settings: QuestSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quest {
    #[serde(rename = "questID:3")]
    pub quest_id: i64,
    #[serde(rename = "preRequisites:11")]
    pub pre_requisites: Vec<i64>,
    #[serde(rename = "properties:10")]
    pub properties: QuestProperties,
    #[serde(rename = "tasks:9")]
    pub tasks: ::serde_json::Value,
    #[serde(rename = "rewards:9")]
    pub rewards: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestProperties {
    #[serde(rename = "betterquesting:10")]
    pub betterquesting: BetterquestingQuestProperties,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetterquestingQuestProperties {
    #[serde(rename = "snd_complete:8")]
    pub snd_complete: String,
    #[serde(rename = "taskLogic:8")]
    pub task_logic: String,
    #[serde(rename = "partySingleReward:1")]
    pub party_single_reward: Option<i64>,
    #[serde(rename = "visibility:8")]
    pub visibility: String,
    #[serde(rename = "isMain:1")]
    pub is_main: i64,
    #[serde(rename = "simultaneous:1")]
    pub simultaneous: i64,
    #[serde(rename = "icon:10")]
    pub icon: Icon,
    #[serde(rename = "snd_update:8")]
    pub snd_update: String,
    #[serde(rename = "repeatTime:3")]
    pub repeat_time: i64,
    #[serde(rename = "globalShare:1")]
    pub global_share: i64,
    #[serde(rename = "questLogic:8")]
    pub quest_logic: String,
    #[serde(rename = "repeat_relative:1")]
    pub repeat_relative: i64,
    #[serde(rename = "name:8")]
    pub name: String,
    #[serde(rename = "isGlobal:1")]
    pub is_global: Option<i64>,
    #[serde(rename = "lockedProgress:1")]
    pub locked_progress: i64,
    #[serde(rename = "autoClaim:1")]
    pub auto_claim: i64,
    #[serde(rename = "isSilent:1")]
    pub is_silent: i64,
    #[serde(rename = "desc:8")]
    pub desc: String,
    // #[serde(rename = "ignoreNBT")]
    // pub ignore_nbt: Option<i64>,
    // #[serde(rename = "")]
    // pub field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "id:8")]
    pub id: String,
    #[serde(rename = "Count:3")]
    pub count: i64,
    #[serde(rename = "registryName:8")]
    pub registry_name: String,
    #[serde(rename = "Damage:2")]
    pub damage: i64,
    #[serde(rename = "OreDict:8")]
    pub ore_dict: String,
    // pub tag: Option<Tag>,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Tag {
//     #[serde(rename = "Fluid")]
//     pub fluid: Option<Fluid>,
//     #[serde(rename = "GT.ToolStats")]
//     pub gt_tool_stats: Option<GtToolStats>,
//     pub color: Option<i64>,
//     #[serde(rename = "Plasmid")]
//     pub plasmid: Option<Plasmid>,
//     #[serde(rename = "DNA")]
//     pub dna: Option<Dna>,
//     #[serde(rename = "Breedable")]
//     pub breedable: Option<i64>,
//     #[serde(rename = "Color")]
//     pub color2: ::serde_json::Value,
//     #[serde(rename = "Rarety")]
//     pub rarety: Option<i64>,
//     #[serde(rename = "Name")]
//     pub name: Option<String>,
//     #[serde(rename = "Durability")]
//     pub durability: Option<i64>,
//     #[serde(rename = "GT.ItemCharge")]
//     pub gt_item_charge: Option<f64>,
//     pub cap: Option<String>,
//     pub rod: Option<String>,
//     #[serde(default)]
//     pub ench: Vec<Ench>,
//     #[serde(rename = "internalCurrentPower")]
//     pub internal_current_power: Option<i64>,
//     pub power: Option<i64>,
//     #[serde(rename = "type")]
//     pub type_field: ::serde_json::Value,
//     pub modules: Option<i64>,
//     #[serde(rename = "internalMaxPower")]
//     pub internal_max_power: Option<i64>,
//     pub upgrades: Option<Upgrades>,
//     pub config: Option<Config>,
//     pub sacrifice: Option<i64>,
//     #[serde(rename = "BrewName")]
//     pub brew_name: Option<String>,
//     #[serde(rename = "RemainingCapacity")]
//     pub remaining_capacity: Option<i64>,
//     #[serde(rename = "Splash")]
//     pub splash: Option<i64>,
//     #[serde(rename = "Items")]
//     pub items: ::serde_json::Value,
//     #[serde(rename = "UsedCapacity")]
//     pub used_capacity: Option<i64>,
//     #[serde(rename = "BrewDrinkSpeed")]
//     pub brew_drink_speed: Option<i64>,
//     #[serde(rename = "BrewInfo")]
//     pub brew_info: Option<String>,
//     #[serde(rename = "Power")]
//     pub power2: Option<i64>,
//     #[serde(rename = "EffectCount")]
//     pub effect_count: Option<i64>,
//     #[serde(rename = "MaxH")]
//     pub max_h: Option<i64>,
//     #[serde(rename = "Health")]
//     pub health: Option<i64>,
//     #[serde(rename = "IsAnalyzed")]
//     pub is_analyzed: Option<i64>,
//     #[serde(rename = "Genome")]
//     pub genome: Option<Genome>,
//     #[serde(rename = "HEE_enhancements")]
//     #[serde(default)]
//     pub hee_enhancements: Vec<String>,
//     #[serde(rename = "hasPotionChanged")]
//     pub has_potion_changed: Option<i64>,
//     #[serde(rename = "pouchCharms")]
//     pub pouch_charms: Option<Vec<PouchCharm>>,
//     #[serde(rename = "pouchID")]
//     pub pouch_id: Option<i64>,
//     #[serde(rename = "engRgnTim")]
//     pub eng_rgn_tim: Option<i64>,
//     pub cluster: Option<Cluster>,
//     #[serde(rename = "prevLoc")]
//     pub prev_loc: Option<i64>,
//     #[serde(rename = "prevDim")]
//     pub prev_dim: Option<i64>,
//     pub seed2: Option<i64>,
//     pub seed1: Option<i64>,
//     #[serde(rename = "curBiome")]
//     pub cur_biome: Option<i64>,
//     pub charge: Option<i64>,
//     pub energy: Option<i64>,
//     #[serde(rename = "wearableData")]
//     pub wearable_data: Option<WearableData>,
//     pub entity: Option<String>,
//     pub stats: Option<Stats>,
//     pub saved: Option<Vec<String>>,
//     pub meta: Option<i64>,
//     #[serde(rename = "StoredEnchantments")]
//     #[serde(default)]
//     pub stored_enchantments: Vec<StoredEnchantment>,
//     #[serde(rename = "InfiTool")]
//     pub infi_tool: Option<InfiTool>,
//     pub display: Option<Display>,
//     #[serde(rename = "Energy")]
//     pub energy2: Option<i64>,
//     #[serde(rename = "TinkerArmor")]
//     pub tinker_armor: Option<TinkerArmor>,
//     #[serde(rename = "TinkerAccessory")]
//     pub tinker_accessory: Option<TinkerAccessory>,
//     pub electricity: Option<i64>,
//     pub uid: Option<i64>,
//     pub material: Option<String>,
//     #[serde(rename = "DualMat")]
//     pub dual_mat: Option<DualMat>,
//     #[serde(rename = "Aspects")]
//     #[serde(default)]
//     pub aspects: Vec<Aspect>,
//     pub upgrade: Option<Vec<Upgrade>>,
//     #[serde(rename = "Mate")]
//     pub mate: Option<Mate>,
//     pub warp: Option<i64>,
//     #[serde(rename = "Data")]
//     pub data: Option<i64>,
//     #[serde(rename = "AttributeModifiers")]
//     #[serde(default)]
//     pub attribute_modifiers: Vec<AttributeModifier>,
//     pub track: Option<String>,
//     pub model: Option<String>,
//     pub stable: Option<i64>,
//     pub damage: Option<i64>,
//     #[serde(rename = "enderio.darksteel.upgrade.energyUpgrade")]
//     pub enderio_darksteel_upgrade_energy_upgrade: Option<EnderioDarksteelUpgradeEnergyUpgrade>,
//     #[serde(rename = "enderio.darksteel.upgrade.glide")]
//     pub enderio_darksteel_upgrade_glide: Option<EnderioDarksteelUpgradeGlide>,
//     #[serde(rename = "enderio.darksteel.upgrade.apiaristArmor")]
//     pub enderio_darksteel_upgrade_apiarist_armor: Option<EnderioDarksteelUpgradeApiaristArmor>,
//     #[serde(rename = "NA")]
//     pub na: Option<i64>,
//     #[serde(rename = "storedEnergyRF")]
//     pub stored_energy_rf: Option<i64>,
//     pub sceptre: Option<i64>,
//     pub owner: Option<String>,
//     #[serde(rename = "name")]
//     pub name2: Option<String>,
//     pub scan: Option<i64>,
//     pub growth: Option<i64>,
//     pub resistance: Option<i64>,
//     pub gain: Option<i64>,
//     #[serde(rename = "reagentTanks")]
//     #[serde(default)]
//     pub reagent_tanks: Vec<ReagentTank>,
//     #[serde(rename = "fuelTank")]
//     pub fuel_tank: Option<FuelTank>,
//     #[serde(rename = "tickCounter")]
//     pub tick_counter: Option<i64>,
//     pub status: Option<i64>,
//     #[serde(rename = "jetpackData")]
//     pub jetpack_data: Option<JetpackData>,
//     #[serde(rename = "T")]
//     pub t: Option<i64>,
//     #[serde(rename = "FarmBlock")]
//     pub farm_block: Option<i64>,
//     #[serde(rename = "fluidID")]
//     pub fluid_id: Option<String>,
//     pub species: Option<String>,
//     pub allele: Option<String>,
//     pub chromosome: Option<i64>,
//     #[serde(rename = "UUID")]
//     pub uuid: Option<String>,
//     #[serde(rename = "Inventory")]
//     pub inventory: Option<Inventory2>,
//     #[serde(rename = "Open")]
//     pub open: Option<i64>,
//     pub aqua: Option<i64>,
//     pub terra: Option<i64>,
//     pub ignis: Option<i64>,
//     pub ordo: Option<i64>,
//     pub perditio: Option<i64>,
//     pub aer: Option<i64>,
//     #[serde(default)]
//     pub pages: Vec<String>,
//     #[serde(rename = "WITCDamage")]
//     pub witcdamage: Option<i64>,
//     pub nodetype: Option<i64>,
//     pub nodemod: Option<i64>,
//     pub nodeid: Option<String>,
//     #[serde(rename = "mFluidDisplayAmount")]
//     pub m_fluid_display_amount: Option<i64>,
//     #[serde(rename = "mFluidDisplayHeat")]
//     pub m_fluid_display_heat: Option<i64>,
//     #[serde(rename = "mFluidState")]
//     pub m_fluid_state: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Fluid {
//     #[serde(rename = "FluidName")]
//     pub fluid_name: String,
//     #[serde(rename = "Amount")]
//     pub amount: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct GtToolStats {
//     #[serde(rename = "PrimaryMaterial")]
//     pub primary_material: String,
//     #[serde(rename = "MaxDamage")]
//     pub max_damage: i64,
//     #[serde(rename = "SecondaryMaterial")]
//     pub secondary_material: String,
//     #[serde(rename = "Damage")]
//     pub damage: Option<i64>,
//     #[serde(rename = "SpecialData")]
//     pub special_data: Option<i64>,
//     #[serde(rename = "Tier")]
//     pub tier: Option<i64>,
//     #[serde(rename = "Voltage")]
//     pub voltage: Option<i64>,
//     #[serde(rename = "MaxCharge")]
//     pub max_charge: Option<i64>,
//     #[serde(rename = "Electric")]
//     pub electric: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Plasmid {
//     #[serde(rename = "Rarity")]
//     pub rarity: i64,
//     #[serde(rename = "Tier")]
//     pub tier: i64,
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Chance")]
//     pub chance: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Dna {
//     #[serde(rename = "Rarity")]
//     pub rarity: i64,
//     #[serde(rename = "Tier")]
//     pub tier: i64,
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Chance")]
//     pub chance: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Ench {
//     pub lvl: i64,
//     pub id: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Upgrades {
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Config {
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Genome {
//     #[serde(rename = "Chromosomes")]
//     pub chromosomes: Vec<Chromosome>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Chromosome {
//     #[serde(rename = "UID1")]
//     pub uid1: String,
//     #[serde(rename = "UID0")]
//     pub uid0: String,
//     #[serde(rename = "Slot")]
//     pub slot: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct PouchCharm {
//     pub null: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Cluster {
//     pub col: Vec<i64>,
//     pub loc: i64,
//     pub lvl: f64,
//     pub max: f64,
//     pub regen: i64,
//     pub drain: i64,
//     pub status: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct WearableData {
//     #[serde(rename = "type")]
//     pub type_field: i64,
//     pub inventory: Option<Inventory>,
//     #[serde(rename = "leftTank")]
//     pub left_tank: Option<LeftTank>,
//     #[serde(rename = "rightTank")]
//     pub right_tank: Option<RightTank>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Inventory {
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct LeftTank {
//     #[serde(rename = "Empty")]
//     pub empty: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct RightTank {
//     #[serde(rename = "Empty")]
//     pub empty: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Stats {
//     #[serde(rename = "mMeta")]
//     pub m_meta: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct StoredEnchantment {
//     pub lvl: i64,
//     pub id: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct InfiTool {
//     #[serde(rename = "RenderExtra")]
//     pub render_extra: Option<i64>,
//     #[serde(rename = "BaseDurability")]
//     pub base_durability: i64,
//     #[serde(rename = "BaseAttack")]
//     pub base_attack: i64,
//     #[serde(rename = "ToolEXP")]
//     pub tool_exp: i64,
//     #[serde(rename = "Built")]
//     pub built: Option<i64>,
//     #[serde(rename = "HarvestLevel")]
//     pub harvest_level: i64,
//     #[serde(rename = "HarvestLevelExtra")]
//     pub harvest_level_extra: Option<i64>,
//     #[serde(rename = "RenderHead")]
//     pub render_head: i64,
//     #[serde(rename = "ModDurability")]
//     pub mod_durability: i64,
//     #[serde(rename = "MiningSpeedExtra")]
//     pub mining_speed_extra: Option<i64>,
//     #[serde(rename = "Shoddy")]
//     pub shoddy: i64,
//     #[serde(rename = "RenderHandle")]
//     pub render_handle: i64,
//     #[serde(rename = "Accessory")]
//     pub accessory: Option<i64>,
//     #[serde(rename = "MiningSpeed")]
//     pub mining_speed: i64,
//     #[serde(rename = "Unbreaking")]
//     pub unbreaking: i64,
//     #[serde(rename = "HarvestLevel2")]
//     pub harvest_level2: Option<i64>,
//     #[serde(rename = "Damage")]
//     pub damage: i64,
//     #[serde(rename = "BonusDurability")]
//     pub bonus_durability: i64,
//     #[serde(rename = "TotalDurability")]
//     pub total_durability: i64,
//     #[serde(rename = "MiningSpeed2")]
//     pub mining_speed2: Option<i64>,
//     #[serde(rename = "HeadEXP")]
//     pub head_exp: Option<i64>,
//     #[serde(rename = "Head")]
//     pub head: i64,
//     #[serde(rename = "Attack")]
//     pub attack: i64,
//     #[serde(rename = "Handle")]
//     pub handle: i64,
//     #[serde(rename = "Broken")]
//     pub broken: i64,
//     #[serde(rename = "Extra")]
//     pub extra: Option<i64>,
//     #[serde(rename = "RenderAccessory")]
//     pub render_accessory: Option<i64>,
//     #[serde(rename = "ToolLevel")]
//     pub tool_level: i64,
//     #[serde(rename = "HarvestLevelModified")]
//     pub harvest_level_modified: Option<i64>,
//     #[serde(rename = "Modifiers")]
//     pub modifiers: i64,
//     #[serde(rename = "BreakChance")]
//     pub break_chance: Option<f64>,
//     #[serde(rename = "Accuracy")]
//     pub accuracy: Option<f64>,
//     #[serde(rename = "Mass")]
//     pub mass: Option<f64>,
//     #[serde(rename = "Ammo")]
//     pub ammo: Option<i64>,
//     #[serde(rename = "DrawSpeed")]
//     pub draw_speed: Option<i64>,
//     #[serde(rename = "BaseDrawSpeed")]
//     pub base_draw_speed: Option<i64>,
//     #[serde(rename = "FlightSpeed")]
//     pub flight_speed: Option<f64>,
//     #[serde(rename = "ExtraAttack")]
//     pub extra_attack: Option<i64>,
//     #[serde(rename = "ExtraSmite")]
//     pub extra_smite: Option<i64>,
//     #[serde(rename = "ExtraLuckLooting")]
//     pub extra_luck_looting: Option<i64>,
//     #[serde(rename = "RepairCount")]
//     pub repair_count: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Display {
//     #[serde(rename = "Name")]
//     pub name: Option<String>,
//     #[serde(rename = "Lore")]
//     #[serde(default)]
//     pub lore: Vec<String>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct TinkerArmor {
//     #[serde(rename = "BaseDurability")]
//     pub base_durability: i64,
//     #[serde(rename = "BaseDefense")]
//     pub base_defense: i64,
//     #[serde(rename = "Built")]
//     pub built: i64,
//     #[serde(rename = "Moss")]
//     pub moss: Option<i64>,
//     #[serde(rename = "Tooltip1")]
//     pub tooltip1: Option<String>,
//     #[serde(rename = "Tooltip2")]
//     pub tooltip2: Option<String>,
//     #[serde(rename = "ModifierTip3")]
//     pub modifier_tip3: Option<String>,
//     #[serde(rename = "Effect2")]
//     pub effect2: Option<i64>,
//     #[serde(rename = "ModifierTip2")]
//     pub modifier_tip2: Option<String>,
//     #[serde(rename = "Effect1")]
//     pub effect1: Option<i64>,
//     #[serde(rename = "ModifierTip1")]
//     pub modifier_tip1: Option<String>,
//     #[serde(rename = "Effect3")]
//     pub effect3: Option<i64>,
//     #[serde(rename = "ModDurability")]
//     pub mod_durability: i64,
//     #[serde(rename = "DamageReduction")]
//     pub damage_reduction: i64,
//     #[serde(rename = "WaterWalk")]
//     pub water_walk: Option<i64>,
//     #[serde(rename = "Broken")]
//     pub broken: i64,
//     #[serde(rename = "Tooltip3")]
//     pub tooltip3: Option<String>,
//     #[serde(rename = "MaxDefense")]
//     pub max_defense: i64,
//     #[serde(rename = "Damage")]
//     pub damage: i64,
//     #[serde(rename = "Slimy Soles")]
//     pub slimy_soles: Option<i64>,
//     #[serde(rename = "BonusDurability")]
//     pub bonus_durability: i64,
//     #[serde(rename = "Modifiers")]
//     pub modifiers: i64,
//     #[serde(rename = "TotalDurability")]
//     pub total_durability: i64,
//     #[serde(rename = "Double-Jump")]
//     pub double_jump: Option<i64>,
//     #[serde(rename = "Feather Fall")]
//     pub feather_fall: Option<i64>,
//     #[serde(rename = "Perfect Dodge")]
//     pub perfect_dodge: Option<i64>,
//     #[serde(rename = "Stealth")]
//     pub stealth: Option<i64>,
//     #[serde(rename = "Night Vision")]
//     pub night_vision: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct TinkerAccessory {
//     #[serde(rename = "BaseDurability")]
//     pub base_durability: i64,
//     #[serde(rename = "Built")]
//     pub built: i64,
//     #[serde(rename = "Tooltip1")]
//     pub tooltip1: String,
//     #[serde(rename = "Effect1")]
//     pub effect1: i64,
//     #[serde(rename = "ModifierTip1")]
//     pub modifier_tip1: String,
//     #[serde(rename = "ModDurability")]
//     pub mod_durability: i64,
//     #[serde(rename = "Broken")]
//     pub broken: i64,
//     #[serde(rename = "Redstone")]
//     pub redstone: Vec<i64>,
//     #[serde(rename = "MiningSpeed")]
//     pub mining_speed: i64,
//     #[serde(rename = "Damage")]
//     pub damage: i64,
//     #[serde(rename = "BonusDurability")]
//     pub bonus_durability: i64,
//     #[serde(rename = "Modifiers")]
//     pub modifiers: i64,
//     #[serde(rename = "TotalDurability")]
//     pub total_durability: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct DualMat {
//     #[serde(rename = "Material2")]
//     pub material2: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Aspect {
//     pub amount: i64,
//     pub key: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Upgrade {
//     pub id: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Mate {
//     #[serde(rename = "Chromosomes")]
//     pub chromosomes: Vec<Chromosome2>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Chromosome2 {
//     #[serde(rename = "UID1")]
//     pub uid1: String,
//     #[serde(rename = "UID0")]
//     pub uid0: String,
//     #[serde(rename = "Slot")]
//     pub slot: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct AttributeModifier {
//     #[serde(rename = "UUIDMost")]
//     pub uuidmost: i64,
//     #[serde(rename = "UUIDLeast")]
//     pub uuidleast: i64,
//     #[serde(rename = "Amount")]
//     pub amount: i64,
//     #[serde(rename = "AttributeName")]
//     pub attribute_name: String,
//     #[serde(rename = "Operation")]
//     pub operation: i64,
//     #[serde(rename = "Name")]
//     pub name: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct EnderioDarksteelUpgradeEnergyUpgrade {
//     #[serde(rename = "upgradeItem")]
//     pub upgrade_item: UpgradeItem,
//     #[serde(rename = "maxOuput")]
//     pub max_ouput: i64,
//     pub level_cost: i64,
//     pub unlocalized_name: String,
//     #[serde(rename = "maxInput")]
//     pub max_input: i64,
//     pub capacity: i64,
//     pub energy: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct UpgradeItem {
//     #[serde(rename = "Count")]
//     pub count: i64,
//     #[serde(rename = "Damage")]
//     pub damage: i64,
//     pub id: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct EnderioDarksteelUpgradeGlide {
//     #[serde(rename = "upgradeItem")]
//     pub upgrade_item: UpgradeItem2,
//     pub level_cost: i64,
//     pub unlocalized_name: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct UpgradeItem2 {
//     pub id: i64,
//     #[serde(rename = "Count")]
//     pub count: i64,
//     #[serde(rename = "Damage")]
//     pub damage: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct EnderioDarksteelUpgradeApiaristArmor {
//     #[serde(rename = "upgradeItem")]
//     pub upgrade_item: UpgradeItem3,
//     pub level_cost: i64,
//     pub unlocalized_name: String,
//     pub slot: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct UpgradeItem3 {
//     pub id: i64,
//     #[serde(rename = "Count")]
//     pub count: i64,
//     #[serde(rename = "Damage")]
//     pub damage: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct ReagentTank {
//     pub amount: i64,
//     #[serde(rename = "Reagent")]
//     pub reagent: String,
//     pub capacity: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct FuelTank {
//     #[serde(rename = "Empty")]
//     pub empty: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct JetpackData {
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Inventory2 {
//     #[serde(rename = "Items")]
//     pub items: Items,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Items {
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestLine {
    #[serde(rename = "quests:9")]
    pub quests: HashMap<String, QuestPosition>,
    #[serde(rename = "lineID:3")]
    pub line_id: i64,
    #[serde(rename = "properties:10")]
    pub properties: QuestLineProperties,
    #[serde(rename = "order:3")]
    pub order: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestPosition {
    #[serde(rename = "sizeX:3")]
    pub size_x: i64,
    #[serde(rename = "x:3")]
    pub x: i64,
    #[serde(rename = "y:3")]
    pub y: i64,
    #[serde(rename = "id:3")]
    pub id: i64,
    #[serde(rename = "sizeY:3")]
    pub size_y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestLineProperties {
    #[serde(rename = "betterquesting:10")]
    pub betterquesting: BetterquestingQuestLineProperties,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetterquestingQuestLineProperties {
    #[serde(rename = "visibility:8")]
    pub visibility: String,
    #[serde(rename = "name:8")]
    pub name: String,
    #[serde(rename = "icon:10")]
    pub icon: Icon2,
    #[serde(rename = "bg_image:8")]
    pub bg_image: String,
    #[serde(rename = "bg_size:3")]
    pub bg_size: i64,
    #[serde(rename = "desc:8")]
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Icon2 {
    #[serde(rename = "id:8")]
    pub id: String,
    #[serde(rename = "Count:3")]
    pub count: i64,
    #[serde(rename = "registryName:8")]
    pub registry_name: String,
    #[serde(rename = "Damage:2")]
    pub damage: i64,
    #[serde(rename = "OreDict:8")]
    pub ore_dict: String,
    // #[serde(rename = "tag:10")]
    // pub tag: Option<Tag2>,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Tag2 {
//     #[serde(default)]
//     pub ench: Vec<Ench2>,
//     #[serde(rename = "GT.ItemCharge")]
//     pub gt_item_charge: Option<i64>,
//     pub cap: Option<String>,
//     pub rod: Option<String>,
//     #[serde(rename = "AttributeModifiers")]
//     #[serde(default)]
//     pub attribute_modifiers: Vec<AttributeModifier2>,
//     pub aqua: Option<i64>,
//     pub terra: Option<i64>,
//     pub ignis: Option<i64>,
//     pub ordo: Option<i64>,
//     pub perditio: Option<i64>,
//     pub aer: Option<i64>,
//     #[serde(rename = "MaxH")]
//     pub max_h: Option<i64>,
//     #[serde(rename = "Mate")]
//     pub mate: Option<Mate2>,
//     #[serde(rename = "Health")]
//     pub health: Option<i64>,
//     #[serde(rename = "IsAnalyzed")]
//     pub is_analyzed: Option<i64>,
//     #[serde(rename = "Genome")]
//     pub genome: Option<Genome2>,
//     pub charge: Option<i64>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Ench2 {
//     pub lvl: i64,
//     pub id: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct AttributeModifier2 {
//     #[serde(rename = "UUIDMost")]
//     pub uuidmost: i64,
//     #[serde(rename = "UUIDLeast")]
//     pub uuidleast: i64,
//     #[serde(rename = "Amount")]
//     pub amount: i64,
//     #[serde(rename = "AttributeName")]
//     pub attribute_name: String,
//     #[serde(rename = "Operation")]
//     pub operation: i64,
//     #[serde(rename = "Name")]
//     pub name: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Mate2 {
//     #[serde(rename = "Chromosomes")]
//     pub chromosomes: Vec<Chromosome3>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Chromosome3 {
//     #[serde(rename = "UID1")]
//     pub uid1: String,
//     #[serde(rename = "UID0")]
//     pub uid0: String,
//     #[serde(rename = "Slot")]
//     pub slot: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Genome2 {
//     #[serde(rename = "Chromosomes")]
//     pub chromosomes: Vec<Chromosome4>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Chromosome4 {
//     #[serde(rename = "UID1")]
//     pub uid1: String,
//     #[serde(rename = "UID0")]
//     pub uid0: String,
//     #[serde(rename = "Slot")]
//     pub slot: i64,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestSettings {
    #[serde(rename = "betterquesting:10")]
    pub betterquesting: BetterquestingQuestSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetterquestingQuestSettings {
    #[serde(rename = "livesMax:3")]
    pub lives_max: i64,
    #[serde(rename = "pack_name:8")]
    pub pack_name: String,
    #[serde(rename = "home_anchor_y:5")]
    pub home_anchor_y: f64,
    #[serde(rename = "livesDef:3")]
    pub lives_def: i64,
    #[serde(rename = "home_anchor_x:5")]
    pub home_anchor_x: f64,
    #[serde(rename = "hardcore:1")]
    pub hardcore: i64,
    #[serde(rename = "home_image:8")]
    pub home_image: String,
    #[serde(rename = "party_enable:1")]
    pub party_enable: i64,
    #[serde(rename = "pack_version:3")]
    pub pack_version: i64,
    #[serde(rename = "editMode:1")]
    pub edit_mode: i64,
    #[serde(rename = "home_offset_x:3")]
    pub home_offset_x: i64,
    #[serde(rename = "home_offset_y:3")]
    pub home_offset_y: i64,
}

#[cfg(test)]
mod tests {
    use super::Root;
    use std::fs;

    #[test]
    fn desrialize_sample() {
        let text = fs::read_to_string("./sample/1/QuestDatabase.json").unwrap();
        let result = serde_json::from_str::<Root>(&text).unwrap();
        for (_id, q) in result.quest_database {
            println!("{}", q.properties.betterquesting.name);
        }
    }
}

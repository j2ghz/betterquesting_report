// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "build:8")]
    build_8: String,
    #[serde(rename = "format:8")]
    format_8: String,
    //#[serde(rename = "questDatabase:9")]
    //quest_database_9: HashMap<String, QuestDatabase9>,
    //#[serde(rename = "questLines:9")]
    //quest_lines_9: QuestLines9,
    #[serde(rename = "questSettings:10")]
    quest_settings_10: QuestSettings10,
}

#[derive(Serialize, Deserialize)]
pub struct QuestDatabase9 {
    #[serde(rename = "questID:3")]
    quest_id_3: i64,
    #[serde(rename = "preRequisites:11")]
    pre_requisites_11: Vec<i64>,
    #[serde(rename = "properties:10")]
    properties_10: QuestDatabase9_Properties10,
    #[serde(rename = "tasks:9")]
    tasks_9: Tasks9,
    #[serde(rename = "rewards:9")]
    rewards_9: QuestDatabase9_Rewards9,
}

#[derive(Serialize, Deserialize)]
pub struct QuestDatabase9_Properties10 {
    #[serde(rename = "betterquesting:10")]
    betterquesting_10: PurpleBetterquesting10,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleBetterquesting10 {
    #[serde(rename = "snd_complete:8")]
    snd_complete_8: SndTe8,
    #[serde(rename = "taskLogic:8")]
    task_logic_8: Logic8,
    #[serde(rename = "partySingleReward:1")]
    party_single_reward_1: Option<i64>,
    #[serde(rename = "visibility:8")]
    visibility_8: Visibility8,
    #[serde(rename = "isMain:1")]
    is_main_1: i64,
    #[serde(rename = "simultaneous:1")]
    simultaneous_1: i64,
    #[serde(rename = "icon:10")]
    icon_10: The1210_Value,
    #[serde(rename = "snd_update:8")]
    snd_update_8: SndTe8,
    #[serde(rename = "repeatTime:3")]
    repeat_time_3: i64,
    #[serde(rename = "globalShare:1")]
    global_share_1: i64,
    #[serde(rename = "questLogic:8")]
    quest_logic_8: Logic8,
    #[serde(rename = "repeat_relative:1")]
    repeat_relative_1: i64,
    #[serde(rename = "name:8")]
    name_8: String,
    #[serde(rename = "isGlobal:1")]
    is_global_1: Option<i64>,
    #[serde(rename = "lockedProgress:1")]
    locked_progress_1: i64,
    #[serde(rename = "autoClaim:1")]
    auto_claim_1: i64,
    #[serde(rename = "isSilent:1")]
    is_silent_1: i64,
    #[serde(rename = "desc:8")]
    desc_8: String,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: Option<i64>,
    #[serde(rename = ":8")]
    the_8: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct The1210_Value {
    #[serde(rename = "id:8")]
    id_8: String,
    #[serde(rename = "Count:3")]
    count_3: i64,
    #[serde(rename = "registryName:8")]
    registry_name_8: String,
    #[serde(rename = "Damage:2")]
    damage_2: i64,
    #[serde(rename = "OreDict:8")]
    ore_dict_8: String,
    #[serde(rename = "tag:10")]
    tag_10: Option<The1210_Tag10>,
}

#[derive(Serialize, Deserialize)]
pub struct The1210_Tag10 {
    #[serde(rename = "GT.ToolStats:10")]
    gt_tool_stats_10: Option<PurpleGtToolStats10>,
    #[serde(rename = "InfiTool:10")]
    infi_tool_10: Option<InfiTool10>,
    #[serde(rename = "display:10")]
    display_10: Option<Display10>,
    #[serde(rename = "Durability:4")]
    durability_4: Option<i64>,
    #[serde(rename = "mFluidDisplayAmount:4")]
    m_fluid_display_amount_4: Option<i64>,
    #[serde(rename = "mFluidDisplayHeat:4")]
    m_fluid_display_heat_4: Option<i64>,
    #[serde(rename = "mFluidState:4")]
    m_fluid_state_4: Option<i64>,
    #[serde(rename = "aqua:4")]
    aqua_4: Option<i64>,
    #[serde(rename = "terra:4")]
    terra_4: Option<i64>,
    #[serde(rename = "ignis:4")]
    ignis_4: Option<i64>,
    #[serde(rename = "ordo:4")]
    ordo_4: Option<i64>,
    #[serde(rename = "perditio:4")]
    perditio_4: Option<i64>,
    #[serde(rename = "aer:4")]
    aer_4: Option<i64>,
    #[serde(rename = "Data:4")]
    data_4: Option<i64>,
    #[serde(rename = "sacrifice:4")]
    sacrifice_4: Option<i64>,
    #[serde(rename = "cap:8")]
    cap_8: Option<String>,
    #[serde(rename = "rod:8")]
    rod_8: Option<String>,
    #[serde(rename = "nodetype:4")]
    nodetype_4: Option<i64>,
    #[serde(rename = "Aspects:9")]
    aspects_9: Option<Aspects9>,
    #[serde(rename = "nodemod:4")]
    nodemod_4: Option<i64>,
    #[serde(rename = "nodeid:8")]
    nodeid_8: Option<Nodeid8>,
    #[serde(rename = "GT.ItemCharge:4")]
    gt_item_charge_4: Option<f64>,
    #[serde(rename = "WITCDamage:3")]
    witc_damage_3: Option<i64>,
    #[serde(rename = "pages:9")]
    pages_9: Option<Pages9>,
    #[serde(rename = "entity:8")]
    entity_8: Option<String>,
    #[serde(rename = "UUID:8")]
    uuid_8: Option<String>,
    #[serde(rename = "Inventory:10")]
    inventory_10: Option<Inventory10>,
    #[serde(rename = "Open:4")]
    open_4: Option<i64>,
    #[serde(rename = "charge:4")]
    charge_4: Option<i64>,
    #[serde(rename = "MaxH:4")]
    max_h_4: Option<i64>,
    #[serde(rename = "NA:4")]
    na_4: Option<i64>,
    #[serde(rename = "Health:4")]
    health_4: Option<i64>,
    #[serde(rename = "IsAnalyzed:4")]
    is_analyzed_4: Option<i64>,
    #[serde(rename = "Genome:10")]
    genome_10: Option<E10>,
    #[serde(rename = "species:8")]
    species_8: Option<Species8>,
    #[serde(rename = "allele:8")]
    allele_8: Option<Allele8>,
    #[serde(rename = "chromosome:4")]
    chromosome_4: Option<i64>,
    #[serde(rename = "Fluid:10")]
    fluid_10: Option<Fluid10>,
    #[serde(rename = "fluidID:8")]
    fluid_id_8: Option<String>,
    #[serde(rename = "FarmBlock:4")]
    farm_block_4: Option<i64>,
    #[serde(rename = "T:4")]
    t_4: Option<i64>,
    #[serde(rename = "type:8")]
    type_8: Option<String>,
    #[serde(rename = "storedEnergyRF:4")]
    stored_energy_rf_4: Option<i64>,
    #[serde(rename = "charge:6")]
    charge_6: Option<i64>,
    #[serde(rename = "jetpackData:10")]
    jetpack_data_10: Option<ForgeData10>,
    #[serde(rename = "fuelTank:10")]
    fuel_tank_10: Option<Tank10>,
    #[serde(rename = "tickCounter:4")]
    tick_counter_4: Option<i64>,
    #[serde(rename = "status:4")]
    status_4: Option<i64>,
    #[serde(rename = "track:8")]
    track_8: Option<String>,
    #[serde(rename = "reagentTanks:9")]
    reagent_tanks_9: Option<ReagentTanks9>,
    #[serde(rename = "owner:8")]
    owner_8: Option<Owner8>,
    #[serde(rename = "name:8")]
    tag_10__name_8: Option<String>,
    #[serde(rename = "scan:4")]
    scan_4: Option<i64>,
    #[serde(rename = "growth:4")]
    growth_4: Option<i64>,
    #[serde(rename = "resistance:4")]
    resistance_4: Option<i64>,
    #[serde(rename = "gain:4")]
    gain_4: Option<i64>,
    #[serde(rename = "sceptre:4")]
    sceptre_4: Option<i64>,
    #[serde(rename = "AttributeModifiers:9")]
    attribute_modifiers_9: Option<Modifiers9>,
    #[serde(rename = "wearableData:10")]
    wearable_data_10: Option<PurpleWearableData10>,
    #[serde(rename = "enderio.darksteel.upgrade.energyUpgrade:10")]
    enderio_darksteel_upgrade_energy_upgrade_10: Option<PurpleEnderioDarksteelUpgradeEnergyUpgrade10>,
    #[serde(rename = "enderio.darksteel.upgrade.glide:10")]
    enderio_darksteel_upgrade_glide_10: Option<EnderioDarksteelUpgradeSoundDetector10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.apiaristArmor:10")]
    enderio_darksteel_upgrade_apiarist_armor_10: Option<EnderioDarksteelUpgradeSoundDetector10_Class>,
    #[serde(rename = "ench:9")]
    ench_9: Option<HashMap<String, HashMap<String, i64>>>,
    #[serde(rename = "damage:4")]
    damage_4: Option<i64>,
    #[serde(rename = "stable:4")]
    stable_4: Option<i64>,
    #[serde(rename = "Mate:10")]
    mate_10: Option<E10>,
    #[serde(rename = "model:8")]
    model_8: Option<String>,
    #[serde(rename = "internalMaxPower:6")]
    internal_max_power_6: Option<i64>,
    #[serde(rename = "internalCurrentPower:6")]
    internal_current_power_6: Option<i64>,
    #[serde(rename = "warp:3")]
    warp_3: Option<i64>,
    #[serde(rename = "upgrade:9")]
    upgrade_9: Option<HashMap<String, Upgrade9>>,
    #[serde(rename = "material:8")]
    material_8: Option<String>,
    #[serde(rename = "Energy:4")]
    energy_4: Option<i64>,
    #[serde(rename = "DualMat:10")]
    dual_mat_10: Option<DualMat10>,
    #[serde(rename = "uid:4")]
    uid_4: Option<i64>,
    #[serde(rename = "Items:9")]
    items_9: Option<HashMap<String, HashMap<String, i64>>>,
    #[serde(rename = "electricity:6")]
    electricity_6: Option<i64>,
    #[serde(rename = "TinkerArmor:10")]
    tinker_armor_10: Option<TinkerArmor10>,
    #[serde(rename = "TinkerAccessory:10")]
    tinker_accessory_10: Option<TinkerAccessory10>,
    #[serde(rename = "StoredEnchantments:9")]
    stored_enchantments_9: Option<StoredEnchantments9>,
    #[serde(rename = "meta:4")]
    meta_4: Option<i64>,
    #[serde(rename = "saved:9")]
    saved_9: Option<Saved9>,
    #[serde(rename = "stats:10")]
    stats_10: Option<Stats10>,
    #[serde(rename = "IsAnalyzed:1")]
    is_analyzed_1: Option<i64>,
    #[serde(rename = "energy:4")]
    tag_10__energy_4: Option<i64>,
    #[serde(rename = "seed2:2")]
    seed2_2: Option<i64>,
    #[serde(rename = "seed1:4")]
    seed1_4: Option<f64>,
    #[serde(rename = "curBiome:1")]
    cur_biome_1: Option<i64>,
    #[serde(rename = "cluster:10")]
    cluster_10: Option<Cluster10>,
    #[serde(rename = "prevLoc:4")]
    prev_loc_4: Option<i64>,
    #[serde(rename = "prevDim:2")]
    prev_dim_2: Option<i64>,
    #[serde(rename = "engRgnTim:2")]
    eng_rgn_tim_2: Option<i64>,
    #[serde(rename = "HEE_enhancements:9")]
    hee_enhancements_9: Option<HeeEnhancements9>,
    #[serde(rename = "pouchCharms:9")]
    pouch_charms_9: Option<HashMap<String, PouchCharms9>>,
    #[serde(rename = "pouchID:4")]
    pouch_id_4: Option<f64>,
    #[serde(rename = "hasPotionChanged:1")]
    has_potion_changed_1: Option<i64>,
    #[serde(rename = "MaxH:3")]
    max_h_3: Option<i64>,
    #[serde(rename = "Health:3")]
    health_3: Option<i64>,
    #[serde(rename = "BrewName:8")]
    brew_name_8: Option<String>,
    #[serde(rename = "RemainingCapacity:3")]
    remaining_capacity_3: Option<i64>,
    #[serde(rename = "Color:3")]
    color_3: Option<i64>,
    #[serde(rename = "Splash:1")]
    splash_1: Option<i64>,
    #[serde(rename = "UsedCapacity:3")]
    used_capacity_3: Option<i64>,
    #[serde(rename = "BrewDrinkSpeed:3")]
    brew_drink_speed_3: Option<i64>,
    #[serde(rename = "BrewInfo:8")]
    brew_info_8: Option<String>,
    #[serde(rename = "Power:3")]
    power_3: Option<i64>,
    #[serde(rename = "EffectCount:3")]
    effect_count_3: Option<i64>,
    #[serde(rename = "sacrifice:1")]
    sacrifice_1: Option<i64>,
    #[serde(rename = "upgrades:10")]
    upgrades_10: Option<ForgeData10>,
    #[serde(rename = "config:10")]
    config_10: Option<ForgeData10>,
    #[serde(rename = "power:6")]
    power_6: Option<i64>,
    #[serde(rename = "type:1")]
    type_1: Option<i64>,
    #[serde(rename = "modules:1")]
    modules_1: Option<i64>,
    #[serde(rename = "Durability:3")]
    durability_3: Option<i64>,
    #[serde(rename = "Plasmid:10")]
    plasmid_10: Option<The10>,
    #[serde(rename = "DNA:10")]
    dna_10: Option<The10>,
    #[serde(rename = "Breedable:1")]
    breedable_1: Option<i64>,
    #[serde(rename = "Color:11")]
    color_11: Option<Vec<i64>>,
    #[serde(rename = "Rarety:1")]
    rarety_1: Option<i64>,
    #[serde(rename = "Name:8")]
    name_8: Option<String>,
    #[serde(rename = "color:1")]
    color_1: Option<i64>,
    #[serde(rename = "ForgeData:10")]
    forge_data_10: Option<PurpleForgeData10>,
    #[serde(rename = "Attributes:9")]
    attributes_9: Option<Attributes9>,
    #[serde(rename = "Invulnerable:4")]
    invulnerable_4: Option<i64>,
    #[serde(rename = "PortalCooldown:4")]
    portal_cooldown_4: Option<i64>,
    #[serde(rename = "AbsorptionAmount:6")]
    absorption_amount_6: Option<i64>,
    #[serde(rename = "FallDistance:6")]
    fall_distance_6: Option<i64>,
    #[serde(rename = "DeathTime:4")]
    death_time_4: Option<i64>,
    #[serde(rename = "DropChances:9")]
    drop_chances_9: Option<HashMap<String, i64>>,
    #[serde(rename = "PersistenceRequired:4")]
    persistence_required_4: Option<i64>,
    #[serde(rename = "HealF:6")]
    heal_f_6: Option<i64>,
    #[serde(rename = "id:8")]
    id_8: Option<String>,
    #[serde(rename = "Motion:9")]
    motion_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Leashed:4")]
    leashed_4: Option<i64>,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: Option<f64>,
    #[serde(rename = "Air:4")]
    air_4: Option<i64>,
    #[serde(rename = "OnGround:4")]
    on_ground_4: Option<i64>,
    #[serde(rename = "Dimension:4")]
    dimension_4: Option<i64>,
    #[serde(rename = "Rotation:9")]
    rotation_9: Option<HashMap<String, i64>>,
    #[serde(rename = "CreatureInfusion:10")]
    creature_infusion_10: Option<Tag10_CreatureInfusion10>,
    #[serde(rename = "Equipment:9")]
    equipment_9: Option<HashMap<String, ForgeData10>>,
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: Option<i64>,
    #[serde(rename = "CustomName:8")]
    custom_name_8: Option<String>,
    #[serde(rename = "Pos:9")]
    pos_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Fire:4")]
    fire_4: Option<i64>,
    #[serde(rename = "CanPickUpLoot:4")]
    can_pick_up_loot_4: Option<i64>,
    #[serde(rename = "HurtTime:4")]
    hurt_time_4: Option<i64>,
    #[serde(rename = "AttackTime:4")]
    attack_time_4: Option<i64>,
    #[serde(rename = "CustomNameVisible:4")]
    custom_name_visible_4: Option<i64>,
    #[serde(rename = "progress:4")]
    progress_4: Option<i64>,
    #[serde(rename = "CustomFlaskEffects:9")]
    custom_flask_effects_9: Option<CustomFlaskEffects9>,
    #[serde(rename = "oc:data:10")]
    oc_data_10: Option<OcData10>,
    #[serde(rename = "oc:color:4")]
    oc_color_4: Option<i64>,
    #[serde(rename = "oc:lootFactory:8")]
    oc_loot_factory_8: Option<String>,
    #[serde(rename = "Wilt:4")]
    wilt_4: Option<i64>,
    #[serde(rename = "Flowered:4")]
    flowered_4: Option<i64>,
    #[serde(rename = "Age:4")]
    age_4: Option<i64>,
    #[serde(rename = "LavaFilter:10")]
    lava_filter_10: Option<Filter10>,
    #[serde(rename = "RepairCost:4")]
    repair_cost_4: Option<i64>,
    #[serde(rename = "pts:1")]
    pts_1: Option<i64>,
    #[serde(rename = "RepairCost:3")]
    repair_cost_3: Option<i64>,
    #[serde(rename = "toolXP:6")]
    tool_xp_6: Option<i64>,
    #[serde(rename = "charge:3")]
    charge_3: Option<i64>,
    #[serde(rename = "toolMode:3")]
    tool_mode_3: Option<i64>,
    #[serde(rename = "Capacity:4")]
    capacity_4: Option<i64>,
    #[serde(rename = "toolMode:4")]
    tool_mode_4: Option<i64>,
    #[serde(rename = "enderio.darksteel.upgrade.travel:10")]
    enderio_darksteel_upgrade_travel_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "id:2")]
    id_2: Option<i64>,
    #[serde(rename = "Count:1")]
    count_1: Option<i64>,
    #[serde(rename = "registryName:8")]
    registry_name_8: Option<String>,
    #[serde(rename = "Damage:2")]
    damage_2: Option<i64>,
    #[serde(rename = "enderio.darksteel.upgrade.soundDetector:10")]
    enderio_darksteel_upgrade_sound_detector_10: Option<EnderioDarksteelUpgradeSoundDetector10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.naturalistEye:10")]
    enderio_darksteel_upgrade_naturalist_eye_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.nightVision:10")]
    enderio_darksteel_upgrade_night_vision_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.gogglesRevealing:10")]
    enderio_darksteel_upgrade_goggles_revealing_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.speedBoost:10")]
    enderio_darksteel_upgrade_speed_boost_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
}

#[derive(Serialize, Deserialize)]
pub struct Aspects9 {
    #[serde(rename = "0:10")]
    the_010: Aspects9_010,
}

#[derive(Serialize, Deserialize)]
pub struct Aspects9_010 {
    #[serde(rename = "amount:4")]
    amount_4: i64,
    #[serde(rename = "key:8")]
    key_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct Modifiers9 {
    #[serde(rename = "0:10")]
    the_010: AttributeModifiers9_010,
}

#[derive(Serialize, Deserialize)]
pub struct AttributeModifiers9_010 {
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: f64,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: f64,
    #[serde(rename = "Amount:6")]
    amount_6: f64,
    #[serde(rename = "AttributeName:8")]
    attribute_name_8: Option<Name8>,
    #[serde(rename = "Operation:4")]
    operation_4: Option<i64>,
    #[serde(rename = "Name:8")]
    name_8: PurpleName8,
    #[serde(rename = "Operation:3")]
    operation_3: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes9 {
    #[serde(rename = "0:10")]
    the_010: Attributes9_010,
    #[serde(rename = "1:10")]
    the_110: Attributes9_Value,
    #[serde(rename = "2:10")]
    the_210: Attributes9_Value,
    #[serde(rename = "3:10")]
    the_310: Attributes9_Value,
    #[serde(rename = "4:10")]
    the_410: Attributes9_010,
    #[serde(rename = "5:10")]
    the_510: Attributes9_Value,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes9_010 {
    #[serde(rename = "Base:6")]
    base_6: i64,
    #[serde(rename = "Modifiers:9")]
    modifiers_9: Modifiers9,
    #[serde(rename = "Name:8")]
    name_8: Name8,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes9_Value {
    #[serde(rename = "Base:6")]
    base_6: f64,
    #[serde(rename = "Name:8")]
    name_8: Name8,
}

#[derive(Serialize, Deserialize)]
pub struct Cluster10 {
    #[serde(rename = "col:7")]
    col_7: Vec<i64>,
    #[serde(rename = "loc:4")]
    loc_4: i64,
    #[serde(rename = "lvl:5")]
    lvl_5: f64,
    #[serde(rename = "max:5")]
    max_5: f64,
    #[serde(rename = "regen:1")]
    regen_1: i64,
    #[serde(rename = "drain:1")]
    drain_1: i64,
    #[serde(rename = "status:1")]
    status_1: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ForgeData10 {
}

#[derive(Serialize, Deserialize)]
pub struct Tag10_CreatureInfusion10 {
    #[serde(rename = "PlayerInfusions:9")]
    player_infusions_9: HashMap<String, i64>,
    #[serde(rename = "tumorWarpTemp:4")]
    tumor_warp_temp_4: i64,
    #[serde(rename = "InfusionCosts:10")]
    infusion_costs_10: PurpleInfusionCosts10,
    #[serde(rename = "Infusions:9")]
    infusions_9: HashMap<String, i64>,
    #[serde(rename = "toggleClimb:4")]
    toggle_climb_4: i64,
    #[serde(rename = "toggleInvisible:4")]
    toggle_invisible_4: i64,
    #[serde(rename = "tumorWarp:4")]
    tumor_warp_4: i64,
    #[serde(rename = "tumorWarpPermanent:4")]
    tumor_warp_permanent_4: i64,
    #[serde(rename = "sitting:4")]
    sitting_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleInfusionCosts10 {
    #[serde(rename = "Aspects:8")]
    aspects_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomFlaskEffects9 {
    #[serde(rename = "0:10")]
    the_010: CustomFlaskEffects9_010,
}

#[derive(Serialize, Deserialize)]
pub struct CustomFlaskEffects9_010 {
    #[serde(rename = "durationFactor:4")]
    duration_factor_4: i64,
    #[serde(rename = "concentration:4")]
    concentration_4: i64,
    #[serde(rename = "potionID:4")]
    potion_id_4: i64,
    #[serde(rename = "tickDuration:4")]
    tick_duration_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Display10 {
    #[serde(rename = "Name:8")]
    name_8: Option<String>,
    #[serde(rename = "Lore:9")]
    lore_9: Option<Lore9>,
}

#[derive(Serialize, Deserialize)]
pub struct Lore9 {
    #[serde(rename = "0:8")]
    the_08: String,
    #[serde(rename = "1:8")]
    the_18: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct The10 {
    #[serde(rename = "Rarity:1")]
    rarity_1: i64,
    #[serde(rename = "Tier:3")]
    tier_3: i64,
    #[serde(rename = "Name:8")]
    name_8: String,
    #[serde(rename = "Chance:3")]
    chance_3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DualMat10 {
    #[serde(rename = "Material2:4")]
    material2_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct EnderioDarksteelUpgradeSoundDetector10_Class {
    #[serde(rename = "upgradeItem:10")]
    upgrade_item_10: HashMap<String, i64>,
    #[serde(rename = "level_cost:4")]
    level_cost_4: i64,
    #[serde(rename = "unlocalized_name:8")]
    unlocalized_name_8: String,
    #[serde(rename = "slot:4")]
    slot_4: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleEnderioDarksteelUpgradeEnergyUpgrade10 {
    #[serde(rename = "upgradeItem:10")]
    upgrade_item_10: HashMap<String, i64>,
    #[serde(rename = "maxOuput:4")]
    max_ouput_4: i64,
    #[serde(rename = "level_cost:4")]
    level_cost_4: i64,
    #[serde(rename = "unlocalized_name:8")]
    unlocalized_name_8: UnlocalizedName8,
    #[serde(rename = "maxInput:4")]
    max_input_4: i64,
    #[serde(rename = "capacity:4")]
    capacity_4: i64,
    #[serde(rename = "energy:4")]
    energy_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct EnderioDarksteelUpgradeGogglesRevealing10_Class {
    #[serde(rename = "upgradeItem:10")]
    upgrade_item_10: UpgradeItem10,
    #[serde(rename = "level_cost:4")]
    level_cost_4: i64,
    #[serde(rename = "unlocalized_name:8")]
    unlocalized_name_8: String,
    #[serde(rename = "slot:4")]
    slot_4: Option<i64>,
    #[serde(rename = "level:4")]
    level_4: Option<i64>,
    #[serde(rename = "multiplier:6")]
    multiplier_6: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct UpgradeItem10 {
    #[serde(rename = "Count:4")]
    count_4: i64,
    #[serde(rename = "Damage:4")]
    damage_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Fluid10 {
    #[serde(rename = "FluidName:8")]
    fluid_name_8: String,
    #[serde(rename = "Amount:4")]
    amount_4: Option<i64>,
    #[serde(rename = "Amount:3")]
    amount_3: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleForgeData10 {
    #[serde(rename = "WITCInitialWidth:6")]
    witc_initial_width_6: f64,
    #[serde(rename = "ezModsApp:4")]
    ez_mods_app_4: i64,
    #[serde(rename = "smi:4")]
    smi_4: i64,
    #[serde(rename = "WITCInitialHeight:6")]
    witc_initial_height_6: f64,
    #[serde(rename = "HungerOverhaulCheck:4")]
    hunger_overhaul_check_4: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Tank10 {
    #[serde(rename = "Empty:8")]
    empty_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct E10 {
    #[serde(rename = "Chromosomes:9")]
    chromosomes_9: HashMap<String, Chromosomes9>,
}

#[derive(Serialize, Deserialize)]
pub struct Chromosomes9 {
    #[serde(rename = "UID1:8")]
    uid1_8: String,
    #[serde(rename = "UID0:8")]
    uid0_8: String,
    #[serde(rename = "Slot:4")]
    slot_4: Option<i64>,
    #[serde(rename = "Slot:1")]
    slot_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleGtToolStats10 {
    #[serde(rename = "PrimaryMaterial:8")]
    primary_material_8: String,
    #[serde(rename = "MaxDamage:4")]
    max_damage_4: i64,
    #[serde(rename = "SecondaryMaterial:8")]
    secondary_material_8: String,
    #[serde(rename = "SpecialData:4")]
    special_data_4: Option<i64>,
    #[serde(rename = "Tier:4")]
    tier_4: Option<i64>,
    #[serde(rename = "Voltage:4")]
    voltage_4: Option<i64>,
    #[serde(rename = "MaxCharge:4")]
    max_charge_4: Option<i64>,
    #[serde(rename = "Electric:4")]
    electric_4: Option<i64>,
    #[serde(rename = "Damage:4")]
    damage_4: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct HeeEnhancements9 {
    #[serde(rename = "0:8")]
    the_08: String,
    #[serde(rename = "1:8")]
    the_18: Option<String>,
    #[serde(rename = "2:8")]
    the_28: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InfiTool10 {
    #[serde(rename = "BaseDurability:4")]
    base_durability_4: Option<i64>,
    #[serde(rename = "Head:4")]
    head_4: Option<i64>,
    #[serde(rename = "HeadEXP:4")]
    head_exp_4: Option<i64>,
    #[serde(rename = "BaseAttack:4")]
    base_attack_4: Option<i64>,
    #[serde(rename = "Built:4")]
    built_4: Option<i64>,
    #[serde(rename = "ToolEXP:4")]
    tool_exp_4: i64,
    #[serde(rename = "HarvestLevel:4")]
    harvest_level_4: Option<i64>,
    #[serde(rename = "Attack:4")]
    attack_4: Option<i64>,
    #[serde(rename = "RenderHead:4")]
    render_head_4: Option<i64>,
    #[serde(rename = "ModDurability:6")]
    mod_durability_6: Option<i64>,
    #[serde(rename = "Handle:4")]
    handle_4: Option<i64>,
    #[serde(rename = "Broken:4")]
    broken_4: Option<i64>,
    #[serde(rename = "Shoddy:6")]
    shoddy_6: Option<i64>,
    #[serde(rename = "RenderHandle:4")]
    render_handle_4: Option<i64>,
    #[serde(rename = "Accessory:4")]
    accessory_4: Option<i64>,
    #[serde(rename = "MiningSpeed:4")]
    mining_speed_4: Option<i64>,
    #[serde(rename = "RenderAccessory:4")]
    render_accessory_4: Option<i64>,
    #[serde(rename = "ToolLevel:4")]
    tool_level_4: Option<i64>,
    #[serde(rename = "HarvestLevelModified:4")]
    harvest_level_modified_4: Option<i64>,
    #[serde(rename = "Unbreaking:4")]
    unbreaking_4: Option<i64>,
    #[serde(rename = "Damage:4")]
    damage_4: Option<i64>,
    #[serde(rename = "BonusDurability:4")]
    bonus_durability_4: Option<i64>,
    #[serde(rename = "Modifiers:4")]
    modifiers_4: Option<i64>,
    #[serde(rename = "TotalDurability:4")]
    total_durability_4: Option<i64>,
    #[serde(rename = "RenderExtra:4")]
    render_extra_4: Option<i64>,
    #[serde(rename = "HarvestLevelExtra:4")]
    harvest_level_extra_4: Option<i64>,
    #[serde(rename = "MiningSpeedExtra:4")]
    mining_speed_extra_4: Option<i64>,
    #[serde(rename = "HarvestLevel2:4")]
    harvest_level2_4: Option<i64>,
    #[serde(rename = "MiningSpeed2:4")]
    mining_speed2_4: Option<i64>,
    #[serde(rename = "Extra:4")]
    extra_4: Option<i64>,
    #[serde(rename = "DrawSpeed:4")]
    draw_speed_4: Option<i64>,
    #[serde(rename = "BaseDrawSpeed:4")]
    base_draw_speed_4: Option<i64>,
    #[serde(rename = "FlightSpeed:6")]
    flight_speed_6: Option<f64>,
    #[serde(rename = "ExtraAttack:4")]
    extra_attack_4: Option<i64>,
    #[serde(rename = "ExtraSmite:4")]
    extra_smite_4: Option<i64>,
    #[serde(rename = "ExtraLuckLooting:4")]
    extra_luck_looting_4: Option<i64>,
    #[serde(rename = "Ammo:4")]
    ammo_4: Option<i64>,
    #[serde(rename = "RepairCount:4")]
    repair_count_4: Option<i64>,
    #[serde(rename = "BreakChance:6")]
    break_chance_6: Option<f64>,
    #[serde(rename = "Accuracy:6")]
    accuracy_6: Option<f64>,
    #[serde(rename = "Mass:6")]
    mass_6: Option<f64>,
    #[serde(rename = "Moss:4")]
    moss_4: Option<i64>,
    #[serde(rename = "Effect1:4")]
    effect1_4: Option<i64>,
    #[serde(rename = "ModifierTip1:8")]
    modifier_tip1_8: Option<String>,
    #[serde(rename = "Tooltip1:8")]
    tooltip1_8: Option<String>,
    #[serde(rename = "BaseDurability:3")]
    base_durability_3: Option<i64>,
    #[serde(rename = "Head:3")]
    head_3: Option<i64>,
    #[serde(rename = "BaseAttack:3")]
    base_attack_3: Option<i64>,
    #[serde(rename = "Built:1")]
    built_1: Option<i64>,
    #[serde(rename = "HarvestLevel:3")]
    harvest_level_3: Option<i64>,
    #[serde(rename = "Attack:3")]
    attack_3: Option<i64>,
    #[serde(rename = "RenderHead:3")]
    render_head_3: Option<i64>,
    #[serde(rename = "ModDurability:5")]
    mod_durability_5: Option<i64>,
    #[serde(rename = "Handle:3")]
    handle_3: Option<i64>,
    #[serde(rename = "Broken:1")]
    broken_1: Option<i64>,
    #[serde(rename = "Shoddy:5")]
    shoddy_5: Option<i64>,
    #[serde(rename = "RenderHandle:3")]
    render_handle_3: Option<i64>,
    #[serde(rename = "MiningSpeed:3")]
    mining_speed_3: Option<i64>,
    #[serde(rename = "ToolLevel:3")]
    tool_level_3: Option<i64>,
    #[serde(rename = "Unbreaking:3")]
    unbreaking_3: Option<i64>,
    #[serde(rename = "Damage:3")]
    damage_3: Option<i64>,
    #[serde(rename = "BonusDurability:3")]
    bonus_durability_3: Option<i64>,
    #[serde(rename = "TotalDurability:3")]
    total_durability_3: Option<i64>,
    #[serde(rename = "Modifiers:3")]
    modifiers_3: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Inventory10 {
    #[serde(rename = "Items:9")]
    items_9: ForgeData10,
}

#[derive(Serialize, Deserialize)]
pub struct Filter10 {
    #[serde(rename = "Damage:4")]
    damage_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct OcData10 {
    #[serde(rename = "oc:fs.label:8")]
    oc_fs_label_8: Option<String>,
    #[serde(rename = "oc:readonly:4")]
    oc_readonly_4: Option<i64>,
    #[serde(rename = "oc:eeprom:9")]
    oc_eeprom_9: Option<HashMap<String, i64>>,
    #[serde(rename = "oc:label:8")]
    oc_label_8: Option<String>,
    #[serde(rename = "node:10")]
    node_10: Option<Node10>,
    #[serde(rename = "owners:9")]
    owners_9: Option<Owners9>,
    #[serde(rename = "fs:10")]
    fs_10: Option<Fs10>,
}

#[derive(Serialize, Deserialize)]
pub struct Fs10 {
    #[serde(rename = "output:9")]
    output_9: ForgeData10,
    #[serde(rename = "input:9")]
    input_9: ForgeData10,
    #[serde(rename = "capacity.used:4")]
    capacity_used_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Node10 {
    #[serde(rename = "address:8")]
    address_8: String,
    #[serde(rename = "visibility:4")]
    visibility_4: i64,
    #[serde(rename = "buffer:6")]
    buffer_6: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Owners9 {
    #[serde(rename = "0:10")]
    the_010: Owners9_010,
}

#[derive(Serialize, Deserialize)]
pub struct Owners9_010 {
    #[serde(rename = "handles:9")]
    handles_9: ForgeData10,
    #[serde(rename = "address:8")]
    address_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct Pages9 {
    #[serde(rename = "0:8")]
    the_08: String,
}

#[derive(Serialize, Deserialize)]
pub struct PouchCharms9 {
    #[serde(rename = "null:1")]
    null_1: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ReagentTanks9 {
    #[serde(rename = "0:10")]
    the_010: ReagentTanks9_010,
}

#[derive(Serialize, Deserialize)]
pub struct ReagentTanks9_010 {
    #[serde(rename = "amount:4")]
    amount_4: i64,
    #[serde(rename = "Reagent:8")]
    reagent_8: String,
    #[serde(rename = "capacity:4")]
    capacity_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Saved9 {
    #[serde(rename = "0:8")]
    the_08: String,
    #[serde(rename = "1:8")]
    the_18: String,
    #[serde(rename = "2:8")]
    the_28: String,
    #[serde(rename = "3:8")]
    the_38: String,
    #[serde(rename = "4:8")]
    the_48: String,
    #[serde(rename = "5:8")]
    the_58: String,
}

#[derive(Serialize, Deserialize)]
pub struct Stats10 {
    #[serde(rename = "mMeta:4")]
    m_meta_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct StoredEnchantments9 {
    #[serde(rename = "0:10")]
    the_010: HashMap<String, i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TinkerAccessory10 {
    #[serde(rename = "BaseDurability:4")]
    base_durability_4: i64,
    #[serde(rename = "Built:4")]
    built_4: i64,
    #[serde(rename = "Tooltip1:8")]
    tooltip1_8: Option<String>,
    #[serde(rename = "Effect1:4")]
    effect1_4: Option<i64>,
    #[serde(rename = "ModifierTip1:8")]
    modifier_tip1_8: Option<String>,
    #[serde(rename = "ModDurability:6")]
    mod_durability_6: i64,
    #[serde(rename = "Broken:4")]
    broken_4: i64,
    #[serde(rename = "Redstone:9")]
    redstone_9: Option<HashMap<String, i64>>,
    #[serde(rename = "MiningSpeed:4")]
    mining_speed_4: Option<i64>,
    #[serde(rename = "Damage:4")]
    damage_4: i64,
    #[serde(rename = "BonusDurability:4")]
    bonus_durability_4: i64,
    #[serde(rename = "Modifiers:4")]
    modifiers_4: i64,
    #[serde(rename = "TotalDurability:4")]
    total_durability_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TinkerArmor10 {
    #[serde(rename = "BaseDurability:4")]
    base_durability_4: i64,
    #[serde(rename = "BaseDefense:6")]
    base_defense_6: i64,
    #[serde(rename = "Built:4")]
    built_4: i64,
    #[serde(rename = "MaxDefense:6")]
    max_defense_6: i64,
    #[serde(rename = "Damage:4")]
    damage_4: i64,
    #[serde(rename = "BonusDurability:4")]
    bonus_durability_4: i64,
    #[serde(rename = "Modifiers:4")]
    modifiers_4: i64,
    #[serde(rename = "TotalDurability:4")]
    total_durability_4: i64,
    #[serde(rename = "ModDurability:6")]
    mod_durability_6: i64,
    #[serde(rename = "DamageReduction:6")]
    damage_reduction_6: i64,
    #[serde(rename = "Broken:4")]
    broken_4: i64,
    #[serde(rename = "Night Vision:4")]
    night_vision_4: Option<i64>,
    #[serde(rename = "Moss:4")]
    moss_4: Option<i64>,
    #[serde(rename = "Tooltip1:8")]
    tooltip1_8: Option<String>,
    #[serde(rename = "Tooltip2:8")]
    tooltip2_8: Option<String>,
    #[serde(rename = "Effect2:4")]
    effect2_4: Option<i64>,
    #[serde(rename = "ModifierTip2:8")]
    modifier_tip2_8: Option<String>,
    #[serde(rename = "Effect1:4")]
    effect1_4: Option<i64>,
    #[serde(rename = "ModifierTip1:8")]
    modifier_tip1_8: Option<String>,
    #[serde(rename = "ModifierTip3:8")]
    modifier_tip3_8: Option<String>,
    #[serde(rename = "Effect3:4")]
    effect3_4: Option<i64>,
    #[serde(rename = "Perfect Dodge:4")]
    perfect_dodge_4: Option<i64>,
    #[serde(rename = "Stealth:4")]
    stealth_4: Option<i64>,
    #[serde(rename = "Tooltip3:8")]
    tooltip3_8: Option<String>,
    #[serde(rename = "Double-Jump:4")]
    double_jump_4: Option<i64>,
    #[serde(rename = "Feather Fall:4")]
    feather_fall_4: Option<i64>,
    #[serde(rename = "WaterWalk:4")]
    water_walk_4: Option<i64>,
    #[serde(rename = "Slimy Soles:4")]
    slimy_soles_4: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Upgrade9 {
    #[serde(rename = "id:4")]
    id_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleWearableData10 {
    #[serde(rename = "inventory:9")]
    inventory_9: Option<ForgeData10>,
    #[serde(rename = "type:1")]
    type_1: Option<i64>,
    #[serde(rename = "leftTank:10")]
    left_tank_10: Option<Tank10>,
    #[serde(rename = "rightTank:10")]
    right_tank_10: Option<Tank10>,
    #[serde(rename = "type:4")]
    type_4: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestDatabase9_Rewards9 {
    #[serde(rename = "0:10")]
    the_010: Option<Rewards9_010>,
    #[serde(rename = "1:10")]
    the_110: Option<Rewards9_110>,
    #[serde(rename = "2:10")]
    the_210: Option<Rewards9_210>,
    #[serde(rename = "3:10")]
    the_310: Option<Rewards9_310>,
}

#[derive(Serialize, Deserialize)]
pub struct Rewards9_010 {
    #[serde(rename = "rewardID:8")]
    reward_id_8: RewardId8,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "rewards:9")]
    rewards_9: Option<The010_Rewards9>,
    #[serde(rename = "choices:9")]
    choices_9: Option<The010_Choices9>,
    #[serde(rename = "amount:3")]
    amount_3: Option<i64>,
    #[serde(rename = "isLevels:1")]
    is_levels_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The010_Choices9 {
    #[serde(rename = "0:10")]
    the_010: Choices9_Value,
    #[serde(rename = "1:10")]
    the_110: Option<The1210_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<The1210_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<The1210_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<The1210_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<The1210_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<The1210_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<Choices9_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<Choices9_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<Choices9_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1210_Value>,
    #[serde(rename = "11:10")]
    the_1110: Option<The1210_Value>,
    #[serde(rename = "12:10")]
    the_1210: Option<The1210_Value>,
    #[serde(rename = "13:10")]
    the_1310: Option<The1210_Value>,
    #[serde(rename = "14:10")]
    the_1410: Option<The1210_Value>,
    #[serde(rename = "15:10")]
    the_1510: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Choices9_Value {
    #[serde(rename = "id:8")]
    id_8: String,
    #[serde(rename = "Count:3")]
    count_3: i64,
    #[serde(rename = "registryName:8")]
    registry_name_8: String,
    #[serde(rename = "Damage:2")]
    damage_2: i64,
    #[serde(rename = "OreDict:8")]
    ore_dict_8: String,
    #[serde(rename = "tag:10")]
    tag_10: Option<Choices9_Tag10>,
}

#[derive(Serialize, Deserialize)]
pub struct Choices9_Tag10 {
    #[serde(rename = "ForgeData:10")]
    forge_data_10: Option<PurpleForgeData10>,
    #[serde(rename = "Attributes:9")]
    attributes_9: Option<Attributes9>,
    #[serde(rename = "Invulnerable:4")]
    invulnerable_4: Option<i64>,
    #[serde(rename = "PortalCooldown:4")]
    portal_cooldown_4: Option<i64>,
    #[serde(rename = "AbsorptionAmount:6")]
    absorption_amount_6: Option<i64>,
    #[serde(rename = "FallDistance:6")]
    fall_distance_6: Option<i64>,
    #[serde(rename = "DeathTime:4")]
    death_time_4: Option<i64>,
    #[serde(rename = "DropChances:9")]
    drop_chances_9: Option<HashMap<String, i64>>,
    #[serde(rename = "PersistenceRequired:4")]
    persistence_required_4: Option<i64>,
    #[serde(rename = "HealF:6")]
    heal_f_6: Option<i64>,
    #[serde(rename = "id:8")]
    id_8: Option<String>,
    #[serde(rename = "Fuse:4")]
    fuse_4: Option<i64>,
    #[serde(rename = "Motion:9")]
    motion_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Leashed:4")]
    leashed_4: Option<i64>,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: Option<f64>,
    #[serde(rename = "Health:4")]
    health_4: Option<i64>,
    #[serde(rename = "Air:4")]
    air_4: Option<i64>,
    #[serde(rename = "OnGround:4")]
    on_ground_4: Option<i64>,
    #[serde(rename = "Dimension:4")]
    dimension_4: Option<i64>,
    #[serde(rename = "Rotation:9")]
    rotation_9: Option<HashMap<String, i64>>,
    #[serde(rename = "CreatureInfusion:10")]
    creature_infusion_10: Option<Tag10_CreatureInfusion10>,
    #[serde(rename = "Equipment:9")]
    equipment_9: Option<HashMap<String, ForgeData10>>,
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: Option<i64>,
    #[serde(rename = "CustomName:8")]
    custom_name_8: Option<String>,
    #[serde(rename = "Pos:9")]
    pos_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Fire:4")]
    fire_4: Option<i64>,
    #[serde(rename = "CanPickUpLoot:4")]
    can_pick_up_loot_4: Option<i64>,
    #[serde(rename = "HurtTime:4")]
    hurt_time_4: Option<i64>,
    #[serde(rename = "ExplosionRadius:4")]
    explosion_radius_4: Option<i64>,
    #[serde(rename = "AttackTime:4")]
    attack_time_4: Option<i64>,
    #[serde(rename = "CustomNameVisible:4")]
    custom_name_visible_4: Option<i64>,
    #[serde(rename = "StoredEnchantments:9")]
    stored_enchantments_9: Option<StoredEnchantments9>,
    #[serde(rename = "gadomancy:10")]
    gadomancy_10: Option<ForgeData10>,
    #[serde(rename = "Aspects:9")]
    aspects_9: Option<Aspects9>,
    #[serde(rename = "ench:9")]
    ench_9: Option<HashMap<String, HashMap<String, i64>>>,
    #[serde(rename = "GT.ToolStats:10")]
    gt_tool_stats_10: Option<FluffyGtToolStats10>,
    #[serde(rename = "track:8")]
    track_8: Option<String>,
    #[serde(rename = "owner:8")]
    owner_8: Option<Owner8>,
    #[serde(rename = "name:8")]
    tag_10__name_8: Option<String>,
    #[serde(rename = "scan:4")]
    scan_4: Option<i64>,
    #[serde(rename = "growth:4")]
    growth_4: Option<i64>,
    #[serde(rename = "resistance:4")]
    resistance_4: Option<i64>,
    #[serde(rename = "gain:4")]
    gain_4: Option<i64>,
    #[serde(rename = "nodetype:4")]
    nodetype_4: Option<i64>,
    #[serde(rename = "display:10")]
    display_10: Option<Display10>,
    #[serde(rename = "nodemod:4")]
    nodemod_4: Option<i64>,
    #[serde(rename = "nodeid:8")]
    nodeid_8: Option<Nodeid8>,
    #[serde(rename = "MaxH:4")]
    max_h_4: Option<i64>,
    #[serde(rename = "Mate:10")]
    mate_10: Option<E10>,
    #[serde(rename = "IsAnalyzed:4")]
    is_analyzed_4: Option<i64>,
    #[serde(rename = "Genome:10")]
    genome_10: Option<E10>,
    #[serde(rename = "Fluid:10")]
    fluid_10: Option<Fluid10>,
    #[serde(rename = "progress:4")]
    progress_4: Option<i64>,
    #[serde(rename = "AirFilter:10")]
    air_filter_10: Option<Filter10>,
    #[serde(rename = "GT.ItemCharge:4")]
    gt_item_charge_4: Option<f64>,
    #[serde(rename = "electricity:6")]
    electricity_6: Option<i64>,
    #[serde(rename = "material:8")]
    material_8: Option<String>,
    #[serde(rename = "meta:4")]
    meta_4: Option<i64>,
    #[serde(rename = "enabled:4")]
    enabled_4: Option<i64>,
    #[serde(rename = "color:4")]
    color_4: Option<i64>,
    #[serde(rename = "TickableItem:10")]
    tickable_item_10: Option<TickableItem10>,
    #[serde(rename = "metadata:4")]
    metadata_4: Option<i64>,
    #[serde(rename = "oreDict:4")]
    ore_dict_4: Option<i64>,
    #[serde(rename = "Wilt:4")]
    wilt_4: Option<i64>,
    #[serde(rename = "Flowered:4")]
    flowered_4: Option<i64>,
    #[serde(rename = "Age:4")]
    age_4: Option<i64>,
    #[serde(rename = "progress:3")]
    progress_3: Option<i64>,
    #[serde(rename = "RepairCost:4")]
    repair_cost_4: Option<i64>,
    #[serde(rename = "pts:1")]
    pts_1: Option<i64>,
    #[serde(rename = "Data:4")]
    data_4: Option<i64>,
    #[serde(rename = "sacrifice:4")]
    sacrifice_4: Option<i64>,
    #[serde(rename = "cap:8")]
    cap_8: Option<String>,
    #[serde(rename = "rod:8")]
    rod_8: Option<String>,
    #[serde(rename = "aqua:4")]
    aqua_4: Option<i64>,
    #[serde(rename = "terra:4")]
    terra_4: Option<i64>,
    #[serde(rename = "ignis:4")]
    ignis_4: Option<i64>,
    #[serde(rename = "ordo:4")]
    ordo_4: Option<i64>,
    #[serde(rename = "perditio:4")]
    perditio_4: Option<i64>,
    #[serde(rename = "aer:4")]
    aer_4: Option<i64>,
    #[serde(rename = "Inventory:8")]
    inventory_8: Option<String>,
    #[serde(rename = "WITCDamage:4")]
    witc_damage_4: Option<i64>,
    #[serde(rename = "UUID:8")]
    uuid_8: Option<String>,
    #[serde(rename = "Inventory:10")]
    inventory_10: Option<Inventory10>,
    #[serde(rename = "Open:4")]
    open_4: Option<i64>,
    #[serde(rename = "species:8")]
    species_8: Option<Species8>,
    #[serde(rename = "allele:8")]
    allele_8: Option<String>,
    #[serde(rename = "chromosome:4")]
    chromosome_4: Option<i64>,
    #[serde(rename = "FarmBlock:4")]
    farm_block_4: Option<i64>,
    #[serde(rename = "T:4")]
    t_4: Option<i64>,
    #[serde(rename = "toolXP:6")]
    tool_xp_6: Option<i64>,
    #[serde(rename = "charge:3")]
    charge_3: Option<i64>,
    #[serde(rename = "toolMode:3")]
    tool_mode_3: Option<i64>,
    #[serde(rename = "reagentTanks:9")]
    reagent_tanks_9: Option<ReagentTanks9>,
    #[serde(rename = "model:8")]
    model_8: Option<String>,
    #[serde(rename = "warp:3")]
    warp_3: Option<i64>,
    #[serde(rename = "upgrade:9")]
    upgrade_9: Option<HashMap<String, Upgrade9>>,
    #[serde(rename = "Energy:4")]
    energy_4: Option<i64>,
    #[serde(rename = "TinkerArmor:10")]
    tinker_armor_10: Option<HashMap<String, i64>>,
    #[serde(rename = "TinkerAccessory:10")]
    tinker_accessory_10: Option<TinkerAccessory10>,
    #[serde(rename = "mat:8")]
    mat_8: Option<String>,
    #[serde(rename = "saved:9")]
    saved_9: Option<Saved9>,
    #[serde(rename = "IsAnalyzed:1")]
    is_analyzed_1: Option<i64>,
    #[serde(rename = "wearableData:10")]
    wearable_data_10: Option<FluffyWearableData10>,
    #[serde(rename = "engRgnTim:2")]
    eng_rgn_tim_2: Option<i64>,
    #[serde(rename = "scan:1")]
    scan_1: Option<i64>,
    #[serde(rename = "sacrifice:1")]
    sacrifice_1: Option<i64>,
    #[serde(rename = "upgrades:10")]
    upgrades_10: Option<ForgeData10>,
    #[serde(rename = "config:10")]
    config_10: Option<ForgeData10>,
    #[serde(rename = "Capacity:3")]
    capacity_3: Option<i64>,
    #[serde(rename = "catalyst:10")]
    catalyst_10: Option<Catalyst10>,
    #[serde(rename = "freq:4")]
    freq_4: Option<i64>,
    #[serde(rename = "InfiTool:10")]
    infi_tool_10: Option<HashMap<String, f64>>,
    #[serde(rename = "Slots:10")]
    slots_10: Option<Slots10>,
    #[serde(rename = "UID:4")]
    uid_4: Option<i64>,
    #[serde(rename = "type:8")]
    type_8: Option<String>,
    #[serde(rename = "storedEnergyRF:4")]
    stored_energy_rf_4: Option<i64>,
    #[serde(rename = "jetpackData:10")]
    jetpack_data_10: Option<ForgeData10>,
    #[serde(rename = "current:4")]
    current_4: Option<i64>,
    #[serde(rename = "sceptre:4")]
    sceptre_4: Option<i64>,
    #[serde(rename = "AttributeModifiers:9")]
    attribute_modifiers_9: Option<Modifiers9>,
    #[serde(rename = "charge:6")]
    charge_6: Option<i64>,
    #[serde(rename = "hasPotionChanged:1")]
    has_potion_changed_1: Option<i64>,
    #[serde(rename = "power:6")]
    power_6: Option<i64>,
    #[serde(rename = "type:1")]
    type_1: Option<i64>,
    #[serde(rename = "key:8")]
    key_8: Option<String>,
    #[serde(rename = "modules:1")]
    modules_1: Option<i64>,
    #[serde(rename = "id:2")]
    id_2: Option<i64>,
    #[serde(rename = "Count:1")]
    count_1: Option<i64>,
    #[serde(rename = "registryName:8")]
    registry_name_8: Option<String>,
    #[serde(rename = "Damage:2")]
    damage_2: Option<i64>,
    #[serde(rename = "Plasmid:10")]
    plasmid_10: Option<The10>,
    #[serde(rename = "DNA:10")]
    dna_10: Option<The10>,
    #[serde(rename = "Breedable:1")]
    breedable_1: Option<i64>,
    #[serde(rename = "Color:11")]
    color_11: Option<Vec<i64>>,
    #[serde(rename = "Fluid:8")]
    fluid_8: Option<String>,
    #[serde(rename = "Rarety:1")]
    rarety_1: Option<i64>,
    #[serde(rename = "Name:8")]
    name_8: Option<String>,
    #[serde(rename = "rocket_tier:4")]
    rocket_tier_4: Option<i64>,
    #[serde(rename = "enderio.darksteel.upgrade.energyUpgrade:10")]
    enderio_darksteel_upgrade_energy_upgrade_10: Option<FluffyEnderioDarksteelUpgradeEnergyUpgrade10>,
    #[serde(rename = "damage:4")]
    damage_4: Option<i64>,
    #[serde(rename = "DualMat:10")]
    dual_mat_10: Option<DualMat10>,
    #[serde(rename = "mFluid:8")]
    m_fluid_8: Option<String>,
    #[serde(rename = "mCapacity:3")]
    m_capacity_3: Option<i64>,
    #[serde(rename = "mMeta:3")]
    m_meta_3: Option<i64>,
    #[serde(rename = "mFluidAmount:3")]
    m_fluid_amount_3: Option<i64>,
    #[serde(rename = "mInit:1")]
    m_init_1: Option<i64>,
    #[serde(rename = "enderio.darksteel.upgrade.travel:10")]
    enderio_darksteel_upgrade_travel_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.spoon:10")]
    enderio_darksteel_upgrade_spoon_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "fuelTank:10")]
    fuel_tank_10: Option<Tank10>,
    #[serde(rename = "tickCounter:4")]
    tick_counter_4: Option<i64>,
    #[serde(rename = "status:4")]
    status_4: Option<i64>,
    #[serde(rename = "enderio.darksteel.upgrade.glide:10")]
    enderio_darksteel_upgrade_glide_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.apiaristArmor:10")]
    enderio_darksteel_upgrade_apiarist_armor_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.swim:10")]
    enderio_darksteel_upgrade_swim_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "enderio.darksteel.upgrade.jumpBoost:10")]
    enderio_darksteel_upgrade_jump_boost_10: Option<EnderioDarksteelUpgradeGogglesRevealing10_Class>,
    #[serde(rename = "aqua:3")]
    aqua_3: Option<i64>,
    #[serde(rename = "terra:3")]
    terra_3: Option<i64>,
    #[serde(rename = "ignis:3")]
    ignis_3: Option<i64>,
    #[serde(rename = "ordo:3")]
    ordo_3: Option<i64>,
    #[serde(rename = "perditio:3")]
    perditio_3: Option<i64>,
    #[serde(rename = "aer:3")]
    aer_3: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Catalyst10 {
    #[serde(rename = "MaxDamage:4")]
    max_damage_4: i64,
    #[serde(rename = "Damage:4")]
    damage_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyEnderioDarksteelUpgradeEnergyUpgrade10 {
    #[serde(rename = "upgradeItem:10")]
    upgrade_item_10: UpgradeItem10,
    #[serde(rename = "maxOuput:4")]
    max_ouput_4: i64,
    #[serde(rename = "level_cost:4")]
    level_cost_4: i64,
    #[serde(rename = "unlocalized_name:8")]
    unlocalized_name_8: UnlocalizedName8,
    #[serde(rename = "maxInput:4")]
    max_input_4: i64,
    #[serde(rename = "capacity:4")]
    capacity_4: i64,
    #[serde(rename = "energy:4")]
    energy_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyGtToolStats10 {
    #[serde(rename = "PrimaryMaterial:8")]
    primary_material_8: Option<String>,
    #[serde(rename = "MaxDamage:4")]
    max_damage_4: Option<i64>,
    #[serde(rename = "SecondaryMaterial:8")]
    secondary_material_8: Option<String>,
    #[serde(rename = "Damage:4")]
    damage_4: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Slots10 {
    #[serde(rename = "1:8")]
    the_18: String,
}

#[derive(Serialize, Deserialize)]
pub struct TickableItem10 {
    #[serde(rename = "maxTick:4")]
    max_tick_4: i64,
    #[serde(rename = "CreationDate:4")]
    creation_date_4: i64,
    #[serde(rename = "Tick:4")]
    tick_4: i64,
    #[serde(rename = "isActive:4")]
    is_active_4: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyWearableData10 {
    #[serde(rename = "type:4")]
    type_4: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The010_Rewards9 {
    #[serde(rename = "0:10")]
    the_010: The1210_Value,
    #[serde(rename = "1:10")]
    the_110: Option<Choices9_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<The1210_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<Choices9_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<The1210_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<The1210_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<The1210_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<The1210_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Rewards9_110 {
    #[serde(rename = "rewardID:8")]
    reward_id_8: RewardId8,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "rewards:9")]
    rewards_9: Option<The110_Rewards9>,
    #[serde(rename = "choices:9")]
    choices_9: Option<The110_Choices9>,
}

#[derive(Serialize, Deserialize)]
pub struct The110_Choices9 {
    #[serde(rename = "0:10")]
    the_010: Choices9_Value,
    #[serde(rename = "1:10")]
    the_110: The1210_Value,
    #[serde(rename = "2:10")]
    the_210: Option<Choices9_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<Choices9_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<Choices9_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<Choices9_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<Choices9_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<Choices9_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<Choices9_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<Choices9_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1210_Value>,
    #[serde(rename = "11:10")]
    the_1110: Option<The1210_Value>,
    #[serde(rename = "12:10")]
    the_1210: Option<The1210_Value>,
    #[serde(rename = "13:10")]
    the_1310: Option<The1210_Value>,
    #[serde(rename = "14:10")]
    the_1410: Option<The1210_Value>,
    #[serde(rename = "15:10")]
    the_1510: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The110_Rewards9 {
    #[serde(rename = "0:10")]
    the_010: Option<The1210_Value>,
    #[serde(rename = "1:10")]
    the_110: Option<Choices9_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<The1210_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<Choices9_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Rewards9_210 {
    #[serde(rename = "rewardID:8")]
    reward_id_8: RewardId8,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "rewards:9")]
    rewards_9: Option<The210_Rewards9>,
    #[serde(rename = "choices:9")]
    choices_9: Option<HashMap<String, Choices9_Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct The210_Rewards9 {
    #[serde(rename = "0:10")]
    the_010: The1210_Value,
    #[serde(rename = "1:10")]
    the_110: Option<Choices9_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<Choices9_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Rewards9_310 {
    #[serde(rename = "rewardID:8")]
    reward_id_8: RewardId8,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "rewards:9")]
    rewards_9: S9,
}

#[derive(Serialize, Deserialize)]
pub struct S9 {
    #[serde(rename = "0:10")]
    the_010: PurpleIcon10,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleIcon10 {
    #[serde(rename = "id:8")]
    id_8: String,
    #[serde(rename = "Count:3")]
    count_3: i64,
    #[serde(rename = "registryName:8")]
    registry_name_8: String,
    #[serde(rename = "Damage:2")]
    damage_2: i64,
    #[serde(rename = "OreDict:8")]
    ore_dict_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks9 {
    #[serde(rename = "0:10")]
    the_010: Option<Tasks9_010>,
    #[serde(rename = "1:10")]
    the_110: Option<Tasks9_110>,
    #[serde(rename = "2:10")]
    the_210: Option<Tasks9_210>,
    #[serde(rename = "3:10")]
    the_310: Option<Tasks9_310>,
    #[serde(rename = "4:10")]
    the_410: Option<The410>,
    #[serde(rename = "5:10")]
    the_510: Option<The510>,
    #[serde(rename = "6:10")]
    the_610: Option<The610>,
    #[serde(rename = "7:10")]
    the_710: Option<The710>,
    #[serde(rename = "8:10")]
    the_810: Option<The810>,
    #[serde(rename = "9:10")]
    the_910: Option<The910>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1010>,
    #[serde(rename = "11:10")]
    the_1110: Option<The1110>,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks9_010 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: Option<i64>,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The010_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
    #[serde(rename = "visible:1")]
    visible_1: Option<i64>,
    #[serde(rename = "invert:1")]
    invert_1: Option<i64>,
    #[serde(rename = "range:3")]
    range_3: Option<i64>,
    #[serde(rename = "structure:8")]
    structure_8: Option<String>,
    #[serde(rename = "hideInfo:1")]
    hide_info_1: Option<i64>,
    #[serde(rename = "posX:3")]
    pos_x_3: Option<i64>,
    #[serde(rename = "posY:3")]
    pos_y_3: Option<i64>,
    #[serde(rename = "posZ:3")]
    pos_z_3: Option<i64>,
    #[serde(rename = "biome:3")]
    biome_3: Option<i64>,
    #[serde(rename = "name:8")]
    name_8: Option<String>,
    #[serde(rename = "dimension:3")]
    dimension_3: Option<i64>,
    #[serde(rename = "taxiCabDist:1")]
    taxi_cab_dist_1: Option<i64>,
    #[serde(rename = "requiredFluids:9")]
    required_fluids_9: Option<RequiredFluids9>,
}

#[derive(Serialize, Deserialize)]
pub struct RequiredFluids9 {
    #[serde(rename = "0:10")]
    the_010: RequiredFluids9_010,
}

#[derive(Serialize, Deserialize)]
pub struct RequiredFluids9_010 {
    #[serde(rename = "FluidName:8")]
    fluid_name_8: String,
    #[serde(rename = "Amount:3")]
    amount_3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct The010_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: Choices9_Value,
    #[serde(rename = "1:10")]
    the_110: Option<The1210_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<Choices9_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<The1210_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<Choices9_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<The1210_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<The1210_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<Choices9_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<Choices9_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<The1210_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1210_Value>,
    #[serde(rename = "11:10")]
    the_1110: Option<The1210_Value>,
    #[serde(rename = "12:10")]
    the_1210: Option<The1210_Value>,
    #[serde(rename = "13:10")]
    the_1310: Option<The1210_Value>,
    #[serde(rename = "14:10")]
    the_1410: Option<The1210_Value>,
    #[serde(rename = "15:10")]
    the_1510: Option<The1210_Value>,
    #[serde(rename = "16:10")]
    the_1610: Option<The1210_Value>,
    #[serde(rename = "17:10")]
    the_1710: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The010_TargetNbt10 {
    #[serde(rename = "Attributes:9")]
    attributes_9: Option<HashMap<String, Attributes9_Value>>,
    #[serde(rename = "Invulnerable:4")]
    invulnerable_4: Option<i64>,
    #[serde(rename = "PortalCooldown:4")]
    portal_cooldown_4: Option<i64>,
    #[serde(rename = "AbsorptionAmount:6")]
    absorption_amount_6: Option<i64>,
    #[serde(rename = "Saddle:4")]
    saddle_4: Option<i64>,
    #[serde(rename = "FallDistance:6")]
    fall_distance_6: Option<i64>,
    #[serde(rename = "InLove:4")]
    in_love_4: Option<i64>,
    #[serde(rename = "DeathTime:4")]
    death_time_4: Option<i64>,
    #[serde(rename = "DropChances:9")]
    drop_chances_9: Option<HashMap<String, f64>>,
    #[serde(rename = "PersistenceRequired:4")]
    persistence_required_4: Option<i64>,
    #[serde(rename = "HealF:6")]
    heal_f_6: Option<i64>,
    #[serde(rename = "id:8")]
    id_8: Option<String>,
    #[serde(rename = "Age:4")]
    age_4: Option<i64>,
    #[serde(rename = "Motion:9")]
    motion_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Leashed:4")]
    leashed_4: Option<i64>,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: Option<f64>,
    #[serde(rename = "Health:4")]
    health_4: Option<i64>,
    #[serde(rename = "Air:4")]
    air_4: Option<i64>,
    #[serde(rename = "OnGround:4")]
    on_ground_4: Option<i64>,
    #[serde(rename = "Dimension:4")]
    dimension_4: Option<i64>,
    #[serde(rename = "Rotation:9")]
    rotation_9: Option<HashMap<String, f64>>,
    #[serde(rename = "CreatureInfusion:10")]
    creature_infusion_10: Option<PurpleCreatureInfusion10>,
    #[serde(rename = "Equipment:9")]
    equipment_9: Option<HashMap<String, ForgeData10>>,
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: Option<i64>,
    #[serde(rename = "CustomName:8")]
    custom_name_8: Option<String>,
    #[serde(rename = "Pos:9")]
    pos_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Fire:4")]
    fire_4: Option<i64>,
    #[serde(rename = "CanPickUpLoot:4")]
    can_pick_up_loot_4: Option<i64>,
    #[serde(rename = "HurtTime:4")]
    hurt_time_4: Option<i64>,
    #[serde(rename = "AttackTime:4")]
    attack_time_4: Option<i64>,
    #[serde(rename = "CustomNameVisible:4")]
    custom_name_visible_4: Option<i64>,
    #[serde(rename = "Invul:4")]
    invul_4: Option<i64>,
    #[serde(rename = "ForgeData:10")]
    forge_data_10: Option<ForgeData10>,
    #[serde(rename = "Initialized:1")]
    initialized_1: Option<i64>,
    #[serde(rename = "Invulnerable:1")]
    invulnerable_1: Option<i64>,
    #[serde(rename = "PortalCooldown:3")]
    portal_cooldown_3: Option<i64>,
    #[serde(rename = "AbsorptionAmount:5")]
    absorption_amount_5: Option<i64>,
    #[serde(rename = "FallDistance:5")]
    fall_distance_5: Option<i64>,
    #[serde(rename = "DeathTime:2")]
    death_time_2: Option<i64>,
    #[serde(rename = "PersistenceRequired:1")]
    persistence_required_1: Option<i64>,
    #[serde(rename = "HealF:5")]
    heal_f_5: Option<i64>,
    #[serde(rename = "Leashed:1")]
    leashed_1: Option<i64>,
    #[serde(rename = "AttackDamage:5")]
    attack_damage_5: Option<i64>,
    #[serde(rename = "Health:2")]
    health_2: Option<i64>,
    #[serde(rename = "IsUber:1")]
    is_uber_1: Option<i64>,
    #[serde(rename = "Dimension:3")]
    dimension_3: Option<i64>,
    #[serde(rename = "OnGround:1")]
    on_ground_1: Option<i64>,
    #[serde(rename = "Air:2")]
    air_2: Option<i64>,
    #[serde(rename = "Fire:2")]
    fire_2: Option<i64>,
    #[serde(rename = "CanPickUpLoot:1")]
    can_pick_up_loot_1: Option<i64>,
    #[serde(rename = "HurtTime:2")]
    hurt_time_2: Option<i64>,
    #[serde(rename = "oiltweak.inOil:1")]
    oiltweak_in_oil_1: Option<i64>,
    #[serde(rename = "AttackTime:2")]
    attack_time_2: Option<i64>,
    #[serde(rename = "CustomNameVisible:1")]
    custom_name_visible_1: Option<i64>,
    #[serde(rename = "PlayerCreated:1")]
    player_created_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleCreatureInfusion10 {
    #[serde(rename = "PlayerInfusions:9")]
    player_infusions_9: Option<HashMap<String, i64>>,
    #[serde(rename = "tumorWarpTemp:4")]
    tumor_warp_temp_4: Option<i64>,
    #[serde(rename = "InfusionCosts:10")]
    infusion_costs_10: FluffyInfusionCosts10,
    #[serde(rename = "Infusions:9")]
    infusions_9: Option<HashMap<String, i64>>,
    #[serde(rename = "toggleClimb:4")]
    toggle_climb_4: Option<i64>,
    #[serde(rename = "toggleInvisible:4")]
    toggle_invisible_4: Option<i64>,
    #[serde(rename = "tumorWarp:4")]
    tumor_warp_4: Option<i64>,
    #[serde(rename = "tumorWarpPermanent:4")]
    tumor_warp_permanent_4: Option<i64>,
    #[serde(rename = "sitting:4")]
    sitting_4: Option<i64>,
    #[serde(rename = "PlayerInfusions:11")]
    player_infusions_11: Option<Vec<i64>>,
    #[serde(rename = "tumorWarpTemp:3")]
    tumor_warp_temp_3: Option<i64>,
    #[serde(rename = "Infusions:11")]
    infusions_11: Option<Vec<i64>>,
    #[serde(rename = "toggleClimb:1")]
    toggle_climb_1: Option<i64>,
    #[serde(rename = "toggleInvisible:1")]
    toggle_invisible_1: Option<i64>,
    #[serde(rename = "tumorWarp:3")]
    tumor_warp_3: Option<i64>,
    #[serde(rename = "tumorWarpPermanent:3")]
    tumor_warp_permanent_3: Option<i64>,
    #[serde(rename = "sitting:1")]
    sitting_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyInfusionCosts10 {
    #[serde(rename = "Aspects:8")]
    aspects_8: Option<String>,
    #[serde(rename = "Aspects:9")]
    aspects_9: Option<ForgeData10>,
}

#[derive(Serialize, Deserialize)]
pub struct The1010 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<S9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The1010_TargetNbt10 {
    #[serde(rename = "ForgeData:10")]
    forge_data_10: ForgeData10,
    #[serde(rename = "Attributes:9")]
    attributes_9: HashMap<String, Attributes9_Value>,
    #[serde(rename = "Invulnerable:1")]
    invulnerable_1: i64,
    #[serde(rename = "PortalCooldown:3")]
    portal_cooldown_3: i64,
    #[serde(rename = "AbsorptionAmount:5")]
    absorption_amount_5: i64,
    #[serde(rename = "FallDistance:5")]
    fall_distance_5: i64,
    #[serde(rename = "DeathTime:2")]
    death_time_2: i64,
    #[serde(rename = "DropChances:9")]
    drop_chances_9: HashMap<String, f64>,
    #[serde(rename = "PersistenceRequired:1")]
    persistence_required_1: i64,
    #[serde(rename = "id:8")]
    id_8: String,
    #[serde(rename = "HealF:5")]
    heal_f_5: i64,
    #[serde(rename = "Motion:9")]
    motion_9: HashMap<String, i64>,
    #[serde(rename = "Leashed:1")]
    leashed_1: i64,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: f64,
    #[serde(rename = "Health:2")]
    health_2: i64,
    #[serde(rename = "Air:2")]
    air_2: i64,
    #[serde(rename = "OnGround:1")]
    on_ground_1: i64,
    #[serde(rename = "Dimension:3")]
    dimension_3: i64,
    #[serde(rename = "Rotation:9")]
    rotation_9: HashMap<String, f64>,
    #[serde(rename = "CreatureInfusion:10")]
    creature_infusion_10: FluffyCreatureInfusion10,
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: i64,
    #[serde(rename = "Equipment:9")]
    equipment_9: HashMap<String, ForgeData10>,
    #[serde(rename = "CustomName:8")]
    custom_name_8: String,
    #[serde(rename = "Pos:9")]
    pos_9: HashMap<String, i64>,
    #[serde(rename = "Fire:2")]
    fire_2: i64,
    #[serde(rename = "CanPickUpLoot:1")]
    can_pick_up_loot_1: i64,
    #[serde(rename = "HurtTime:2")]
    hurt_time_2: i64,
    #[serde(rename = "oiltweak.inOil:1")]
    oiltweak_in_oil_1: i64,
    #[serde(rename = "AttackTime:2")]
    attack_time_2: i64,
    #[serde(rename = "CustomNameVisible:1")]
    custom_name_visible_1: i64,
    #[serde(rename = "canDespawn:1")]
    can_despawn_1: Option<i64>,
    #[serde(rename = "carriedData:2")]
    carried_data_2: Option<i64>,
    #[serde(rename = "carried:2")]
    carried_2: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyCreatureInfusion10 {
    #[serde(rename = "PlayerInfusions:11")]
    player_infusions_11: Vec<i64>,
    #[serde(rename = "InfusionCosts:10")]
    infusion_costs_10: TentacledInfusionCosts10,
    #[serde(rename = "tumorWarpTemp:3")]
    tumor_warp_temp_3: i64,
    #[serde(rename = "Infusions:11")]
    infusions_11: Vec<i64>,
    #[serde(rename = "toggleClimb:1")]
    toggle_climb_1: i64,
    #[serde(rename = "toggleInvisible:1")]
    toggle_invisible_1: i64,
    #[serde(rename = "tumorWarp:3")]
    tumor_warp_3: i64,
    #[serde(rename = "sitting:1")]
    sitting_1: i64,
    #[serde(rename = "tumorWarpPermanent:3")]
    tumor_warp_permanent_3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledInfusionCosts10 {
    #[serde(rename = "Aspects:9")]
    aspects_9: ForgeData10,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks9_110 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: Option<i64>,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The110_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "visible:1")]
    visible_1: Option<i64>,
    #[serde(rename = "invert:1")]
    invert_1: Option<i64>,
    #[serde(rename = "range:3")]
    range_3: Option<i64>,
    #[serde(rename = "structure:8")]
    structure_8: Option<String>,
    #[serde(rename = "hideInfo:1")]
    hide_info_1: Option<i64>,
    #[serde(rename = "posX:3")]
    pos_x_3: Option<i64>,
    #[serde(rename = "posY:3")]
    pos_y_3: Option<i64>,
    #[serde(rename = "posZ:3")]
    pos_z_3: Option<i64>,
    #[serde(rename = "biome:3")]
    biome_3: Option<i64>,
    #[serde(rename = "name:8")]
    name_8: Option<String>,
    #[serde(rename = "dimension:3")]
    dimension_3: Option<i64>,
    #[serde(rename = "taxiCabDist:1")]
    taxi_cab_dist_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The110_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
    #[serde(rename = "requiredFluids:9")]
    required_fluids_9: Option<RequiredFluids9>,
}

#[derive(Serialize, Deserialize)]
pub struct The110_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: Option<Choices9_Value>,
    #[serde(rename = "1:10")]
    the_110: Option<The1210_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<Choices9_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<The1210_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<Choices9_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<The1210_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<Choices9_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<The1210_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<The1210_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<The1210_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The110_TargetNbt10 {
    #[serde(rename = "Attributes:9")]
    attributes_9: Option<HashMap<String, Attributes9_Value>>,
    #[serde(rename = "Invulnerable:4")]
    invulnerable_4: Option<i64>,
    #[serde(rename = "PortalCooldown:4")]
    portal_cooldown_4: Option<i64>,
    #[serde(rename = "AbsorptionAmount:6")]
    absorption_amount_6: Option<i64>,
    #[serde(rename = "FallDistance:6")]
    fall_distance_6: Option<i64>,
    #[serde(rename = "DeathTime:4")]
    death_time_4: Option<i64>,
    #[serde(rename = "DropChances:9")]
    drop_chances_9: Option<HashMap<String, f64>>,
    #[serde(rename = "roomCoordsY:6")]
    room_coords_y_6: Option<i64>,
    #[serde(rename = "roomCoordsZ:6")]
    room_coords_z_6: Option<i64>,
    #[serde(rename = "PersistenceRequired:4")]
    persistence_required_4: Option<i64>,
    #[serde(rename = "HealF:6")]
    heal_f_6: Option<i64>,
    #[serde(rename = "id:8")]
    id_8: Option<String>,
    #[serde(rename = "Motion:9")]
    motion_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Leashed:4")]
    leashed_4: Option<i64>,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: Option<f64>,
    #[serde(rename = "Health:4")]
    health_4: Option<i64>,
    #[serde(rename = "roomCoordsX:6")]
    room_coords_x_6: Option<i64>,
    #[serde(rename = "Air:4")]
    air_4: Option<i64>,
    #[serde(rename = "OnGround:4")]
    on_ground_4: Option<i64>,
    #[serde(rename = "Dimension:4")]
    dimension_4: Option<i64>,
    #[serde(rename = "roomSizeZ:6")]
    room_size_z_6: Option<i64>,
    #[serde(rename = "Rotation:9")]
    rotation_9: Option<HashMap<String, f64>>,
    #[serde(rename = "roomSizeY:6")]
    room_size_y_6: Option<i64>,
    #[serde(rename = "roomSizeX:6")]
    room_size_x_6: Option<i64>,
    #[serde(rename = "CreatureInfusion:10")]
    creature_infusion_10: Option<PurpleCreatureInfusion10>,
    #[serde(rename = "Equipment:9")]
    equipment_9: Option<HashMap<String, ForgeData10>>,
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: Option<i64>,
    #[serde(rename = "CustomName:8")]
    custom_name_8: Option<CustomName8>,
    #[serde(rename = "Pos:9")]
    pos_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Fire:4")]
    fire_4: Option<i64>,
    #[serde(rename = "CanPickUpLoot:4")]
    can_pick_up_loot_4: Option<i64>,
    #[serde(rename = "HurtTime:4")]
    hurt_time_4: Option<i64>,
    #[serde(rename = "AttackTime:4")]
    attack_time_4: Option<i64>,
    #[serde(rename = "CustomNameVisible:4")]
    custom_name_visible_4: Option<i64>,
    #[serde(rename = "Size:3")]
    size_3: Option<i64>,
    #[serde(rename = "Invulnerable:1")]
    invulnerable_1: Option<i64>,
    #[serde(rename = "PortalCooldown:3")]
    portal_cooldown_3: Option<i64>,
    #[serde(rename = "AbsorptionAmount:5")]
    absorption_amount_5: Option<i64>,
    #[serde(rename = "FallDistance:5")]
    fall_distance_5: Option<i64>,
    #[serde(rename = "DeathTime:2")]
    death_time_2: Option<i64>,
    #[serde(rename = "PersistenceRequired:1")]
    persistence_required_1: Option<i64>,
    #[serde(rename = "HealF:5")]
    heal_f_5: Option<i64>,
    #[serde(rename = "Fuse:2")]
    fuse_2: Option<i64>,
    #[serde(rename = "Leashed:1")]
    leashed_1: Option<i64>,
    #[serde(rename = "Health:2")]
    health_2: Option<i64>,
    #[serde(rename = "Dimension:3")]
    dimension_3: Option<i64>,
    #[serde(rename = "OnGround:1")]
    on_ground_1: Option<i64>,
    #[serde(rename = "ignited:1")]
    ignited_1: Option<i64>,
    #[serde(rename = "Air:2")]
    air_2: Option<i64>,
    #[serde(rename = "Fire:2")]
    fire_2: Option<i64>,
    #[serde(rename = "CanPickUpLoot:1")]
    can_pick_up_loot_1: Option<i64>,
    #[serde(rename = "HurtTime:2")]
    hurt_time_2: Option<i64>,
    #[serde(rename = "AttackTime:2")]
    attack_time_2: Option<i64>,
    #[serde(rename = "ExplosionRadius:1")]
    explosion_radius_1: Option<i64>,
    #[serde(rename = "CustomNameVisible:1")]
    custom_name_visible_1: Option<i64>,
    #[serde(rename = "ForgeData:10")]
    forge_data_10: Option<ForgeData10>,
    #[serde(rename = "SkeletonType:1")]
    skeleton_type_1: Option<i64>,
    #[serde(rename = "carriedData:2")]
    carried_data_2: Option<i64>,
    #[serde(rename = "carried:2")]
    carried_2: Option<i64>,
    #[serde(rename = "ConversionTime:3")]
    conversion_time_3: Option<i64>,
    #[serde(rename = "CanBreakDoors:1")]
    can_break_doors_1: Option<i64>,
    #[serde(rename = "Size:4")]
    size_4: Option<i64>,
    #[serde(rename = "Victim:8")]
    victim_8: Option<String>,
    #[serde(rename = "oiltweak.inOil:1")]
    oiltweak_in_oil_1: Option<i64>,
    #[serde(rename = "isAngry:1")]
    is_angry_1: Option<i64>,
    #[serde(rename = "Sitting:1")]
    sitting_1: Option<i64>,
    #[serde(rename = "InLove:3")]
    in_love_3: Option<i64>,
    #[serde(rename = "SkinColor:1")]
    skin_color_1: Option<i64>,
    #[serde(rename = "Age:3")]
    age_3: Option<i64>,
    #[serde(rename = "Poisonous:1")]
    poisonous_1: Option<i64>,
    #[serde(rename = "Familiar:1")]
    familiar_1: Option<i64>,
    #[serde(rename = "OwnerUUID:8")]
    owner_uuid_8: Option<String>,
    #[serde(rename = "SuicideIn:3")]
    suicide_in_3: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The1110 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<S9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1110_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The1110_TargetNbt10 {
    #[serde(rename = "ForgeData:10")]
    forge_data_10: ForgeData10,
    #[serde(rename = "Attributes:9")]
    attributes_9: HashMap<String, Attributes9_Value>,
    #[serde(rename = "Invulnerable:1")]
    invulnerable_1: i64,
    #[serde(rename = "PortalCooldown:3")]
    portal_cooldown_3: i64,
    #[serde(rename = "AbsorptionAmount:5")]
    absorption_amount_5: i64,
    #[serde(rename = "FallDistance:5")]
    fall_distance_5: i64,
    #[serde(rename = "DeathTime:2")]
    death_time_2: i64,
    #[serde(rename = "DropChances:9")]
    drop_chances_9: HashMap<String, f64>,
    #[serde(rename = "PersistenceRequired:1")]
    persistence_required_1: i64,
    #[serde(rename = "id:8")]
    id_8: String,
    #[serde(rename = "HealF:5")]
    heal_f_5: i64,
    #[serde(rename = "Motion:9")]
    motion_9: HashMap<String, i64>,
    #[serde(rename = "Leashed:1")]
    leashed_1: i64,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: f64,
    #[serde(rename = "Health:2")]
    health_2: i64,
    #[serde(rename = "Air:2")]
    air_2: i64,
    #[serde(rename = "OnGround:1")]
    on_ground_1: i64,
    #[serde(rename = "Dimension:3")]
    dimension_3: i64,
    #[serde(rename = "Rotation:9")]
    rotation_9: HashMap<String, f64>,
    #[serde(rename = "CreatureInfusion:10")]
    creature_infusion_10: FluffyCreatureInfusion10,
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: i64,
    #[serde(rename = "Equipment:9")]
    equipment_9: HashMap<String, ForgeData10>,
    #[serde(rename = "CustomName:8")]
    custom_name_8: String,
    #[serde(rename = "Pos:9")]
    pos_9: HashMap<String, i64>,
    #[serde(rename = "Fire:2")]
    fire_2: i64,
    #[serde(rename = "CanPickUpLoot:1")]
    can_pick_up_loot_1: i64,
    #[serde(rename = "HurtTime:2")]
    hurt_time_2: i64,
    #[serde(rename = "oiltweak.inOil:1")]
    oiltweak_in_oil_1: i64,
    #[serde(rename = "AttackTime:2")]
    attack_time_2: i64,
    #[serde(rename = "CustomNameVisible:1")]
    custom_name_visible_1: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks9_210 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: Option<i64>,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The210_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The210_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
    #[serde(rename = "visible:1")]
    visible_1: Option<i64>,
    #[serde(rename = "invert:1")]
    invert_1: Option<i64>,
    #[serde(rename = "range:3")]
    range_3: Option<i64>,
    #[serde(rename = "structure:8")]
    structure_8: Option<String>,
    #[serde(rename = "hideInfo:1")]
    hide_info_1: Option<i64>,
    #[serde(rename = "posX:3")]
    pos_x_3: Option<i64>,
    #[serde(rename = "posY:3")]
    pos_y_3: Option<i64>,
    #[serde(rename = "posZ:3")]
    pos_z_3: Option<i64>,
    #[serde(rename = "biome:3")]
    biome_3: Option<i64>,
    #[serde(rename = "name:8")]
    name_8: Option<String>,
    #[serde(rename = "dimension:3")]
    dimension_3: Option<i64>,
    #[serde(rename = "taxiCabDist:1")]
    taxi_cab_dist_1: Option<i64>,
    #[serde(rename = "requiredFluids:9")]
    required_fluids_9: Option<RequiredFluids9>,
}

#[derive(Serialize, Deserialize)]
pub struct The210_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: Choices9_Value,
    #[serde(rename = "1:10")]
    the_110: Option<The1210_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<Choices9_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<The1210_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<Choices9_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<Choices9_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<Choices9_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<Choices9_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<Choices9_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<Choices9_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1210_Value>,
    #[serde(rename = "11:10")]
    the_1110: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The210_TargetNbt10 {
    #[serde(rename = "ForgeData:10")]
    forge_data_10: Option<ForgeData10>,
    #[serde(rename = "Attributes:9")]
    attributes_9: Option<HashMap<String, Attributes9_Value>>,
    #[serde(rename = "Invulnerable:1")]
    invulnerable_1: Option<i64>,
    #[serde(rename = "PortalCooldown:3")]
    portal_cooldown_3: Option<i64>,
    #[serde(rename = "AbsorptionAmount:5")]
    absorption_amount_5: Option<i64>,
    #[serde(rename = "FallDistance:5")]
    fall_distance_5: Option<i64>,
    #[serde(rename = "DeathTime:2")]
    death_time_2: Option<i64>,
    #[serde(rename = "DropChances:9")]
    drop_chances_9: Option<HashMap<String, f64>>,
    #[serde(rename = "PersistenceRequired:1")]
    persistence_required_1: Option<i64>,
    #[serde(rename = "id:8")]
    id_8: Option<String>,
    #[serde(rename = "HealF:5")]
    heal_f_5: Option<i64>,
    #[serde(rename = "Motion:9")]
    motion_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Leashed:1")]
    leashed_1: Option<i64>,
    #[serde(rename = "UUIDLeast:4")]
    uuid_least_4: Option<f64>,
    #[serde(rename = "Health:2")]
    health_2: Option<i64>,
    #[serde(rename = "PlayerCreated:1")]
    player_created_1: Option<i64>,
    #[serde(rename = "Air:2")]
    air_2: Option<i64>,
    #[serde(rename = "OnGround:1")]
    on_ground_1: Option<i64>,
    #[serde(rename = "Dimension:3")]
    dimension_3: Option<i64>,
    #[serde(rename = "Rotation:9")]
    rotation_9: Option<HashMap<String, f64>>,
    #[serde(rename = "CreatureInfusion:10")]
    creature_infusion_10: Option<FluffyCreatureInfusion10>,
    #[serde(rename = "UUIDMost:4")]
    uuid_most_4: Option<i64>,
    #[serde(rename = "Equipment:9")]
    equipment_9: Option<HashMap<String, ForgeData10>>,
    #[serde(rename = "CustomName:8")]
    custom_name_8: Option<String>,
    #[serde(rename = "Pos:9")]
    pos_9: Option<HashMap<String, i64>>,
    #[serde(rename = "Fire:2")]
    fire_2: Option<i64>,
    #[serde(rename = "CanPickUpLoot:1")]
    can_pick_up_loot_1: Option<i64>,
    #[serde(rename = "HurtTime:2")]
    hurt_time_2: Option<i64>,
    #[serde(rename = "oiltweak.inOil:1")]
    oiltweak_in_oil_1: Option<i64>,
    #[serde(rename = "AttackTime:2")]
    attack_time_2: Option<i64>,
    #[serde(rename = "CustomNameVisible:1")]
    custom_name_visible_1: Option<i64>,
    #[serde(rename = "FeatherColor:1")]
    feather_color_1: Option<i64>,
    #[serde(rename = "Sitting:1")]
    sitting_1: Option<i64>,
    #[serde(rename = "HomeY:6")]
    home_y_6: Option<i64>,
    #[serde(rename = "HomeZ:6")]
    home_z_6: Option<i64>,
    #[serde(rename = "InLove:3")]
    in_love_3: Option<i64>,
    #[serde(rename = "HomeX:6")]
    home_x_6: Option<i64>,
    #[serde(rename = "Age:3")]
    age_3: Option<i64>,
    #[serde(rename = "Familiar:1")]
    familiar_1: Option<i64>,
    #[serde(rename = "OwnerUUID:8")]
    owner_uuid_8: Option<String>,
    #[serde(rename = "SuicideIn:3")]
    suicide_in_3: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks9_310 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The310_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The310_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: The1210_Value,
    #[serde(rename = "1:10")]
    the_110: Option<Choices9_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<The1210_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<Choices9_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<Choices9_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<Choices9_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<Choices9_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<Choices9_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<Choices9_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<Choices9_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<Choices9_Value>,
    #[serde(rename = "11:10")]
    the_1110: Option<Choices9_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The410 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The410_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The410_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: Choices9_Value,
    #[serde(rename = "1:10")]
    the_110: Option<The1210_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<Choices9_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<Choices9_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<Choices9_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<Choices9_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<Choices9_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<Choices9_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<Choices9_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<Choices9_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1210_Value>,
    #[serde(rename = "11:10")]
    the_1110: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The510 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The510_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The510_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: Choices9_Value,
    #[serde(rename = "1:10")]
    the_110: Option<The1210_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<The1210_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<The1210_Value>,
    #[serde(rename = "4:10")]
    the_410: Option<The1210_Value>,
    #[serde(rename = "5:10")]
    the_510: Option<The1210_Value>,
    #[serde(rename = "6:10")]
    the_610: Option<The1210_Value>,
    #[serde(rename = "7:10")]
    the_710: Option<The1210_Value>,
    #[serde(rename = "8:10")]
    the_810: Option<The1210_Value>,
    #[serde(rename = "9:10")]
    the_910: Option<The1210_Value>,
    #[serde(rename = "10:10")]
    the_1010: Option<The1210_Value>,
    #[serde(rename = "11:10")]
    the_1110: Option<The1210_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The610 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<HashMap<String, The1210_Value>>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The710 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: Option<i64>,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The710_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "visible:1")]
    visible_1: Option<i64>,
    #[serde(rename = "invert:1")]
    invert_1: Option<i64>,
    #[serde(rename = "range:3")]
    range_3: Option<i64>,
    #[serde(rename = "structure:8")]
    structure_8: Option<String>,
    #[serde(rename = "hideInfo:1")]
    hide_info_1: Option<i64>,
    #[serde(rename = "posX:3")]
    pos_x_3: Option<i64>,
    #[serde(rename = "posY:3")]
    pos_y_3: Option<i64>,
    #[serde(rename = "posZ:3")]
    pos_z_3: Option<i64>,
    #[serde(rename = "biome:3")]
    biome_3: Option<i64>,
    #[serde(rename = "name:8")]
    name_8: Option<String>,
    #[serde(rename = "dimension:3")]
    dimension_3: Option<i64>,
    #[serde(rename = "taxiCabDist:1")]
    taxi_cab_dist_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The710_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: The1210_Value,
    #[serde(rename = "1:10")]
    the_110: Option<Choices9_Value>,
    #[serde(rename = "2:10")]
    the_210: Option<The1210_Value>,
    #[serde(rename = "3:10")]
    the_310: Option<Choices9_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The810 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<The810_RequiredItems9>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct The810_RequiredItems9 {
    #[serde(rename = "0:10")]
    the_010: The1210_Value,
    #[serde(rename = "1:10")]
    the_110: Option<Choices9_Value>,
}

#[derive(Serialize, Deserialize)]
pub struct The910 {
    #[serde(rename = "partialMatch:1")]
    partial_match_1: Option<i64>,
    #[serde(rename = "allowSmelt:1")]
    allow_smelt_1: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    ignore_nbt_1: i64,
    #[serde(rename = "index:3")]
    index_3: i64,
    #[serde(rename = "allowCraft:1")]
    allow_craft_1: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    allow_anvil_1: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    required_items_9: Option<HashMap<String, Choices9_Value>>,
    #[serde(rename = "taskID:8")]
    task_id_8: TaskId8,
    #[serde(rename = "autoConsume:1")]
    auto_consume_1: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    group_detect_1: Option<i64>,
    #[serde(rename = "consume:1")]
    consume_1: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    target_nbt_10: Option<The1010_TargetNbt10>,
    #[serde(rename = "damageType:8")]
    damage_type_8: Option<String>,
    #[serde(rename = "required:3")]
    required_3: Option<i64>,
    #[serde(rename = "target:8")]
    target_8: Option<String>,
    #[serde(rename = "subtypes:1")]
    subtypes_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestLines9 {
    #[serde(rename = "0:10")]
    the_010: The1110_Class,
    #[serde(rename = "1:10")]
    the_110: The1110_Class,
    #[serde(rename = "2:10")]
    the_210: The1110_Class,
    #[serde(rename = "3:10")]
    the_310: The1110_Class,
    #[serde(rename = "4:10")]
    the_410: The1110_Class,
    #[serde(rename = "5:10")]
    the_510: The1110_Class,
    #[serde(rename = "6:10")]
    the_610: The1110_Class,
    #[serde(rename = "7:10")]
    the_710: The1110_Class,
    #[serde(rename = "8:10")]
    the_810: The1110_Class,
    #[serde(rename = "9:10")]
    the_910: The1110_Class,
    #[serde(rename = "10:10")]
    the_1010: The1010_Class,
    #[serde(rename = "11:10")]
    the_1110: The1110_Class,
    #[serde(rename = "12:10")]
    the_1210: The1110_Class,
    #[serde(rename = "13:10")]
    the_1310: The1110_Class,
    #[serde(rename = "14:10")]
    the_1410: The1110_Class,
    #[serde(rename = "15:10")]
    the_1510: The1110_Class,
    #[serde(rename = "16:10")]
    the_1610: The1110_Class,
    #[serde(rename = "17:10")]
    the_1710: The1110_Class,
    #[serde(rename = "18:10")]
    the_1810: The1010_Class,
    #[serde(rename = "19:10")]
    the_1910: The1110_Class,
    #[serde(rename = "20:10")]
    the_2010: The1110_Class,
    #[serde(rename = "21:10")]
    the_2110: The2110_Class,
    #[serde(rename = "22:10")]
    the_2210: The1010_Class,
    #[serde(rename = "23:10")]
    the_2310: The1110_Class,
    #[serde(rename = "24:10")]
    the_2410: The1110_Class,
    #[serde(rename = "25:10")]
    the_2510: The1110_Class,
    #[serde(rename = "26:10")]
    the_2610: The1110_Class,
    #[serde(rename = "27:10")]
    the_2710: The1110_Class,
    #[serde(rename = "28:10")]
    the_2810: The1110_Class,
    #[serde(rename = "29:10")]
    the_2910: The2110_Class,
    #[serde(rename = "30:10")]
    the_3010: The1110_Class,
    #[serde(rename = "31:10")]
    the_3110: The1110_Class,
    #[serde(rename = "32:10")]
    the_3210: The1110_Class,
    #[serde(rename = "33:10")]
    the_3310: The1110_Class,
    #[serde(rename = "34:10")]
    the_3410: The2110_Class,
    #[serde(rename = "35:10")]
    the_3510: The1110_Class,
    #[serde(rename = "36:10")]
    the_3610: The1110_Class,
    #[serde(rename = "37:10")]
    the_3710: The1110_Class,
    #[serde(rename = "38:10")]
    the_3810: The1110_Class,
    #[serde(rename = "39:10")]
    the_3910: The1110_Class,
    #[serde(rename = "40:10")]
    the_4010: The2110_Class,
    #[serde(rename = "41:10")]
    the_4110: The1110_Class,
    #[serde(rename = "42:10")]
    the_4210: The1110_Class,
    #[serde(rename = "43:10")]
    the_4310: The1110_Class,
    #[serde(rename = "44:10")]
    the_4410: The1110_Class,
}

#[derive(Serialize, Deserialize)]
pub struct The1110_Class {
    #[serde(rename = "quests:9")]
    quests_9: HashMap<String, HashMap<String, i64>>,
    #[serde(rename = "lineID:3")]
    line_id_3: i64,
    #[serde(rename = "properties:10")]
    properties_10: The010_Properties10,
    #[serde(rename = "order:3")]
    order_3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct The010_Properties10 {
    #[serde(rename = "betterquesting:10")]
    betterquesting_10: FluffyBetterquesting10,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyBetterquesting10 {
    #[serde(rename = "visibility:8")]
    visibility_8: Visibility8,
    #[serde(rename = "name:8")]
    name_8: String,
    #[serde(rename = "icon:10")]
    icon_10: PurpleIcon10,
    #[serde(rename = "bg_image:8")]
    bg_image_8: String,
    #[serde(rename = "bg_size:3")]
    bg_size_3: i64,
    #[serde(rename = "desc:8")]
    desc_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct The1010_Class {
    #[serde(rename = "quests:9")]
    quests_9: HashMap<String, HashMap<String, i64>>,
    #[serde(rename = "lineID:3")]
    line_id_3: i64,
    #[serde(rename = "properties:10")]
    properties_10: The1010_Properties10,
    #[serde(rename = "order:3")]
    order_3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct The1010_Properties10 {
    #[serde(rename = "betterquesting:10")]
    betterquesting_10: TentacledBetterquesting10,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledBetterquesting10 {
    #[serde(rename = "visibility:8")]
    visibility_8: Visibility8,
    #[serde(rename = "name:8")]
    name_8: String,
    #[serde(rename = "icon:10")]
    icon_10: The1210_Value,
    #[serde(rename = "bg_image:8")]
    bg_image_8: String,
    #[serde(rename = "bg_size:3")]
    bg_size_3: i64,
    #[serde(rename = "desc:8")]
    desc_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct The2110_Class {
    #[serde(rename = "quests:9")]
    quests_9: HashMap<String, HashMap<String, i64>>,
    #[serde(rename = "lineID:3")]
    line_id_3: i64,
    #[serde(rename = "properties:10")]
    properties_10: The2110_Properties10,
    #[serde(rename = "order:3")]
    order_3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct The2110_Properties10 {
    #[serde(rename = "betterquesting:10")]
    betterquesting_10: StickyBetterquesting10,
}

#[derive(Serialize, Deserialize)]
pub struct StickyBetterquesting10 {
    #[serde(rename = "visibility:8")]
    visibility_8: Visibility8,
    #[serde(rename = "name:8")]
    name_8: String,
    #[serde(rename = "icon:10")]
    icon_10: Choices9_Value,
    #[serde(rename = "bg_image:8")]
    bg_image_8: String,
    #[serde(rename = "bg_size:3")]
    bg_size_3: i64,
    #[serde(rename = "desc:8")]
    desc_8: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestSettings10 {
    #[serde(rename = "betterquesting:10")]
    betterquesting_10: QuestSettings10_Betterquesting10,
}

#[derive(Serialize, Deserialize)]
pub struct QuestSettings10_Betterquesting10 {
    #[serde(rename = "livesMax:3")]
    lives_max_3: i64,
    #[serde(rename = "pack_name:8")]
    pack_name_8: String,
    #[serde(rename = "home_anchor_y:5")]
    home_anchor_y_5: f64,
    #[serde(rename = "livesDef:3")]
    lives_def_3: i64,
    #[serde(rename = "home_anchor_x:5")]
    home_anchor_x_5: f64,
    #[serde(rename = "hardcore:1")]
    hardcore_1: i64,
    #[serde(rename = "home_image:8")]
    home_image_8: String,
    #[serde(rename = "party_enable:1")]
    party_enable_1: i64,
    #[serde(rename = "pack_version:3")]
    pack_version_3: i64,
    #[serde(rename = "editMode:1")]
    edit_mode_1: i64,
    #[serde(rename = "home_offset_x:3")]
    home_offset_x_3: i64,
    #[serde(rename = "home_offset_y:3")]
    home_offset_y_3: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Allele8 {
    #[serde(rename = "forestry.boolTrue")]
    ForestryBoolTrue,
    #[serde(rename = "forestry.toleranceBoth3")]
    ForestryToleranceBoth3,
    #[serde(rename = "magicbees.speciesDoctoral")]
    MagicbeesSpeciesDoctoral,
}

#[derive(Serialize, Deserialize)]
pub enum Name8 {
    #[serde(rename = "generic.attackDamage")]
    GenericAttackDamage,
    #[serde(rename = "generic.followRange")]
    GenericFollowRange,
    #[serde(rename = "generic.knockbackResistance")]
    GenericKnockbackResistance,
    #[serde(rename = "generic.maxHealth")]
    GenericMaxHealth,
    #[serde(rename = "generic.movementSpeed")]
    GenericMovementSpeed,
    #[serde(rename = "tc.mobmod")]
    TcMobmod,
    #[serde(rename = "weapon.attackSpeed")]
    WeaponAttackSpeed,
    #[serde(rename = "weapon.daze")]
    WeaponDaze,
    #[serde(rename = "weapon.mountedBonus")]
    WeaponMountedBonus,
    #[serde(rename = "weapon.penetrateArmor")]
    WeaponPenetrateArmor,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleName8 {
    #[serde(rename = "Random spawn bonus")]
    RandomSpawnBonus,
    #[serde(rename = "UNLOCKED")]
    Unlocked,
    #[serde(rename = "Weapon modifier")]
    WeaponModifier,
}

#[derive(Serialize, Deserialize)]
pub enum UnlocalizedName8 {
    #[serde(rename = "enderio.darksteel.upgrade.empowered_four")]
    EnderioDarksteelUpgradeEmpoweredFour,
    #[serde(rename = "enderio.darksteel.upgrade.empowered_one")]
    EnderioDarksteelUpgradeEmpoweredOne,
}

#[derive(Serialize, Deserialize)]
pub enum Nodeid8 {
    #[serde(rename = "0:-312:64:54")]
    The03126454,
}

#[derive(Serialize, Deserialize)]
pub enum Owner8 {
    #[serde(rename = "berriespp")]
    Berriespp,
    #[serde(rename = "gregtech")]
    Gregtech,
    #[serde(rename = "IC2")]
    Ic2,
}

#[derive(Serialize, Deserialize)]
pub enum Species8 {
    #[serde(rename = "rootBees")]
    RootBees,
}

#[derive(Serialize, Deserialize)]
pub enum Logic8 {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "XOR")]
    Xor,
}

#[derive(Serialize, Deserialize)]
pub enum SndTe8 {
    #[serde(rename = "minecraft:entity.player.levelup")]
    MinecraftEntityPlayerLevelup,
    #[serde(rename = "random.levelup")]
    RandomLevelup,
}

#[derive(Serialize, Deserialize)]
pub enum Visibility8 {
    #[serde(rename = "HIDDEN")]
    Hidden,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "UNLOCKED")]
    Unlocked,
}

#[derive(Serialize, Deserialize)]
pub enum RewardId8 {
    #[serde(rename = "bq_standard:choice")]
    BqStandardChoice,
    #[serde(rename = "bq_standard:item")]
    BqStandardItem,
    #[serde(rename = "bq_standard:xp")]
    BqStandardXp,
}

#[derive(Serialize, Deserialize)]
pub enum TaskId8 {
    #[serde(rename = "bq_standard:checkbox")]
    BqStandardCheckbox,
    #[serde(rename = "bq_standard:crafting")]
    BqStandardCrafting,
    #[serde(rename = "bq_standard:fluid")]
    BqStandardFluid,
    #[serde(rename = "bq_standard:hunt")]
    BqStandardHunt,
    #[serde(rename = "bq_standard:location")]
    BqStandardLocation,
    #[serde(rename = "bq_standard:retrieval")]
    BqStandardRetrieval,
}

#[derive(Serialize, Deserialize)]
pub enum CustomName8 {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "Tuzdemaku, the Fire Fiend")]
    TuzdemakuTheFireFiend,
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::Root;

    #[test]
    fn desrialize_sample() {
        let text = fs::read_to_string("./sample/1/QuestDatabase.json").unwrap();
        let result : Root = serde_json::from_str(&text).unwrap();
    }
}
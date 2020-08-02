use serde::Deserialize;
use serde::Serialize;
use std::{collections::HashMap, error::Error, fs, path::Path};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Quest {
    #[serde(rename = "questID:3")]
    pub quest_id: i64,
    #[serde(rename = "preRequisites:11")]
    pub pre_requisites: Vec<i64>,
    #[serde(rename = "properties:10")]
    pub properties: QuestProperties,
    #[serde(rename = "tasks:9")]
    pub tasks: HashMap<String, Task>,
    #[serde(rename = "rewards:9")]
    pub rewards: HashMap<String, RewardType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "rewardID:8")]
pub enum RewardType {
    #[serde(rename = "bq_standard:choice")]
    Choice {
        #[serde(rename = "index:3")]
        index: i64,
        #[serde(rename = "choices:9")]
        choices: HashMap<String, Item>,
    },
    #[serde(rename = "bq_standard:item")]
    Item {
        #[serde(rename = "index:3")]
        index: i64,
        #[serde(rename = "rewards:9")]
        rewards: HashMap<String, Item>,
    },
    #[serde(rename = "bq_standard:xp")]
    Xp {
        #[serde(rename = "index:3")]
        index: i64,
        #[serde(rename = "amount:3")]
        amount: i64,
        #[serde(rename = "isLevels:1")]
        is_levels: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BqTaskType {
    #[serde(rename = "bq_standard:checkbox")]
    StandardCheckbox,
    #[serde(rename = "bq_standard:crafting")]
    StandardCrafting,
    #[serde(rename = "bq_standard:fluid")]
    StandardFluid,
    #[serde(rename = "bq_standard:hunt")]
    StandardHunt,
    #[serde(rename = "bq_standard:location")]
    StandardLocation,
    #[serde(rename = "bq_standard:retrieval")]
    StandardRetrieval,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Item {
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
    #[serde(rename = "tag:10")]
    pub nbt: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Task {
    #[serde(rename = "partialMatch:1")]
    pub partial_match: Option<i64>,
    #[serde(rename = "autoConsume:1")]
    pub auto_consume: Option<i64>,
    #[serde(rename = "groupDetect:1")]
    pub group_detect: Option<i64>,
    #[serde(rename = "ignoreNBT:1")]
    pub ignore_nbt: Option<i64>,
    #[serde(rename = "index:3")]
    pub index: i64,
    #[serde(rename = "consume:1")]
    pub consume: Option<i64>,
    #[serde(rename = "requiredItems:9")]
    pub required_items: Option<HashMap<String, Item>>,
    #[serde(rename = "taskID:8")]
    pub task_id: BqTaskType,
    #[serde(rename = "allowSmelt:1")]
    pub allow_smelt: Option<i64>,
    #[serde(rename = "allowCraft:1")]
    pub allow_craft: Option<i64>,
    #[serde(rename = "allowAnvil:1")]
    pub allow_anvil: Option<i64>,
    #[serde(rename = "targetNBT:10")]
    pub target_nbt: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "damageType:8")]
    pub damage_type: Option<String>,
    #[serde(rename = "required:3")]
    pub required: Option<i64>,
    #[serde(rename = "target:8")]
    pub target: Option<String>,
    #[serde(rename = "subtypes:1")]
    pub subtypes: Option<i64>,
    #[serde(rename = "visible:1")]
    pub visible: Option<i64>,
    #[serde(rename = "invert:1")]
    pub invert: Option<i64>,
    #[serde(rename = "range:3")]
    pub range: Option<i64>,
    #[serde(rename = "structure:8")]
    pub structure: Option<String>,
    #[serde(rename = "hideInfo:1")]
    pub hide_info: Option<i64>,
    #[serde(rename = "posX:3")]
    pub pos_x: Option<i64>,
    #[serde(rename = "posY:3")]
    pub pos_y: Option<i64>,
    #[serde(rename = "posZ:3")]
    pub pos_z: Option<i64>,
    #[serde(rename = "biome:3")]
    pub biome: Option<i64>,
    #[serde(rename = "name:8")]
    pub name: Option<String>,
    #[serde(rename = "dimension:3")]
    pub dimension: Option<i64>,
    #[serde(rename = "taxiCabDist:1")]
    pub taxi_cab_dist: Option<i64>,
    #[serde(rename = "requiredFluids:9")]
    pub required_fluids: Option<HashMap<String, RequiredFluids>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct RequiredFluids {
    #[serde(rename = "FluidName:8")]
    pub fluid_name: String,
    #[serde(rename = "Amount:3")]
    pub amount: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct QuestProperties {
    #[serde(rename = "betterquesting:10")]
    pub betterquesting: BetterquestingQuestProperties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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
    #[serde(rename = "tag:10")]
    pub nbt: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct QuestLineProperties {
    #[serde(rename = "betterquesting:10")]
    pub betterquesting: BetterquestingQuestLineProperties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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
    #[serde(rename = "tag:10")]
    pub nbt: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct QuestSettings {
    #[serde(rename = "betterquesting:10")]
    pub betterquesting: BetterquestingQuestSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

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

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Root, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let result = serde_json::from_str::<Root>(&text)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::Root;
    use std::fs;

    #[test]
    fn desrialize_sample() {
        let text = fs::read_to_string("./sample/1/QuestDatabase.json").unwrap();
        let result = serde_json::from_str::<Root>(&text).unwrap();
        for q in result.quest_database.values() {
            println!("{}", q.properties.betterquesting.name);
        }
    }
}

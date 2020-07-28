use serde::Deserialize;
use serde::Serialize;
use std::{collections::HashMap, error::Error, fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "questProgress:9")]
    pub quest_progress: HashMap<String, QuestProgress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestProgress {
    #[serde(rename = "questID:3")]
    pub quest_id: i64,
    #[serde(rename = "completed:9")]
    pub completed: HashMap<String, Completed>,
    #[serde(rename = "tasks:9")]
    pub tasks: HashMap<String, Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Completed {
    #[serde(rename = "claimed:1")]
    pub claimed: i64,
    #[serde(rename = "uuid:8")]
    pub uuid: String,
    #[serde(rename = "timestamp:4")]
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TasksClass {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "completeUsers:9")]
    pub complete_users: HashMap<String, String>,
    #[serde(rename = "userProgress:9")]
    pub user_progress: Option<HashMap<String, UserProgress>>,
    #[serde(rename = "index:3")]
    pub index: i64,
    #[serde(rename = "taskID:8")]
    pub task_id: TaskType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProgress {
    #[serde(rename = "data:9")]
    pub data: Option<HashMap<String, i64>>,
    #[serde(rename = "uuid:8")]
    pub uuid: String,
    #[serde(rename = "value:3")]
    pub value: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
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

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Root, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let result = serde_json::from_str::<Root>(&text)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::Root;
    use chrono::prelude::*;
    use std::{collections::BinaryHeap, fs};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct QuestCompletion {
        timestamp: DateTime<Utc>,
        user: String,
        id: i64,
    }

    #[test]
    fn desrialize_sample() {
        let text = fs::read_to_string("./sample/1/QuestProgress.json").unwrap();
        let result = serde_json::from_str::<Root>(&text).unwrap();
        assert_ne!(0, result.quest_progress.len());

        let mut items = result
            .quest_progress
            .values()
            .flat_map(|q| {
                let id = q.quest_id;
                q.completed.values().map(move |c| QuestCompletion {
                    id,
                    user: c.uuid.clone(),
                    timestamp: Utc.timestamp_millis(c.timestamp),
                })
            })
            .collect::<BinaryHeap<_>>();
        let mut last_date = Utc.timestamp_millis(0).date();
        while items.len() > 0 {
            let item = items.pop().unwrap();
            if item.timestamp.date() != last_date {
                last_date = item.timestamp.date();
                println!("{:?}", last_date)
            }
            println!("{:?}", item)
        }
    }
}

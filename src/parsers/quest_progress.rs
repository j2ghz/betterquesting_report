use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "questProgress:9")]
    pub quest_progress: HashMap<String,QuestProgress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestProgress {
    #[serde(rename = "questID:3")]
    pub quest_id: i64,
    #[serde(rename = "completed:9")]
    pub completed: HashMap<String,Completed>,
    #[serde(rename = "tasks:9")]
    pub tasks: HashMap<String,Task>,
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
pub struct TasksClass {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "completeUsers:9")]
    pub complete_users: HashMap<String,String>,
    #[serde(rename = "userProgress:9")]
    pub user_progress: Option<HashMap<String,UserProgress>>,
    #[serde(rename = "index:3")]
    pub index: i64,
    #[serde(rename = "taskID:8")]
    pub task_id: TaskType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProgress {
    
    #[serde(rename = "data:9")]
    pub data: Option<HashMap<String,i64>>,
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


#[cfg(test)]
mod tests {
    use super::Root;
    use std::fs;

    #[test]
    fn desrialize_sample() {
        let text = fs::read_to_string("./sample/1/QuestProgress.json").unwrap();
        let result = serde_json::from_str::<Root>(&text).unwrap();      
    }
}

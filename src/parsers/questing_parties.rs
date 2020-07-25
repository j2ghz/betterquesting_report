use serde::Deserialize;
use serde::Serialize;
use std::{collections::HashMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "parties:9")]
    pub parties: HashMap<String, Parties>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parties {
    #[serde(rename = "members:9")]
    pub members: HashMap<String, Members>,
    #[serde(rename = "partyID:3")]
    pub party_id: i64,
    #[serde(rename = "properties:10")]
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Members {
    #[serde(rename = "uuid:8")]
    pub uuid: String,
    #[serde(rename = "status:8")]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "betterquesting:10")]
    pub betterquesting: Betterquesting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Betterquesting {
    #[serde(rename = "name:8")]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::Root;
    use std::fs;

    #[test]
    fn desrialize_sample() {
        let text = fs::read_to_string("./sample/1/QuestingParties.json").unwrap();
        let result = serde_json::from_str::<Root>(&text).unwrap();
        for q in result.parties {
            println!("{:?}", q);
        }
    }
}

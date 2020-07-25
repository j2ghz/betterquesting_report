use chrono::{DateTime, TimeZone, Utc};
use quest_database::Quest;
use serde::Serialize;
use std::{collections::HashMap, path::Path};

pub mod name_cache;
pub mod quest_database;
pub mod quest_progress;
pub mod questing_parties;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
pub struct QuestCompletion {
    timestamp: DateTime<Utc>,
    user: String,
    id: i64,
}

#[derive(Debug, Serialize)]
pub struct Data {
    pub quests: HashMap<i64, Quest>,
    pub users: HashMap<String, String>,
    pub quest_ql: HashMap<i64, String>,
    pub quest_unlocks: HashMap<i64, Vec<i64>>,
    pub completions: quest_progress::Root,
}

pub fn load_data<P: AsRef<Path>>(dir: P) -> Data {
    let db = quest_database::parse(dir.as_ref().join("QuestDatabase.json")).unwrap();
    let quests = db
        .quest_database
        .iter()
        .map(|(_id, q)| (q.quest_id, q.clone()))
        .collect::<HashMap<i64, quest_database::Quest>>();
    let users = name_cache::parse(dir.as_ref().join("NameCache.json"))
        .unwrap()
        .name_cache
        .iter()
        .map(|(_id, i)| (i.uuid.clone(), i.name.clone()))
        .collect::<HashMap<String, String>>();

    let quest_ql: HashMap<i64, String> = db
        .quest_lines
        .iter()
        .flat_map(|(_id, ql)| {
            let name = ql.properties.betterquesting.name.clone();
            ql.quests
                .iter()
                .map(|(_id, q)| (q.id.clone(), name.clone()))
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect();

    let mut quest_unlocks: HashMap<i64, Vec<i64>> = HashMap::new();

    for (_id, q) in &quests {
        for prereq_id in &q.pre_requisites {
            if quest_unlocks.contains_key(&prereq_id) {
                let old = quest_unlocks.get(&prereq_id).unwrap().to_vec();
                quest_unlocks
                    .insert(prereq_id.clone(), {
                        let mut new: Vec<_> = old.as_slice().to_vec();
                        new.push(q.quest_id);
                        new
                    })
                    .unwrap();
            } else {
                quest_unlocks.insert(prereq_id.clone(), vec![q.quest_id]);
            }
        }
    }

    let completions = quest_progress::parse(dir.as_ref().join("QuestProgress.json")).unwrap();

    Data {
        quests,
        users,
        quest_ql,
        quest_unlocks,
        completions,
    }
}

#[cfg(test)]
mod tests {
    use super::load_data;
    use chrono::prelude::*;
    use std::collections::{BinaryHeap, HashMap};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct QuestCompletion {
        timestamp: DateTime<Utc>,
        user: String,
        id: i64,
    }

    #[test]
    fn desrialize_sample_all() {
        load_data("./sample/1");
    }

    #[test]
    fn desrialize_sample() {
        let quests = crate::parsers::quest_database::parse("./sample/1/QuestDatabase.json")
            .unwrap()
            .quest_database;
        let quests = quests
            .iter()
            .map(|(_id, q)| (q.quest_id, q.clone()))
            .collect::<HashMap<i64, super::quest_database::Quest>>();
        let completions =
            crate::parsers::quest_progress::parse("./sample/1/QuestProgress.json").unwrap();

        let mut items = completions
            .quest_progress
            .iter()
            .flat_map(|(_id, q)| {
                let id = q.quest_id;
                q.completed.iter().map(move |(_idc, c)| QuestCompletion {
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
            let quest = quests.get(&item.id).unwrap();
            println!(
                "{:?} {} {}",
                item.timestamp, item.user, quest.properties.betterquesting.name
            );
        }
    }
}

use chrono::{DateTime, TimeZone, Utc};
use serde::Serialize;
use std::{
    collections::{BinaryHeap, HashMap},
    path::Path,
};

pub mod name_cache;
pub mod quest_database;
pub mod quest_progress;
pub mod questing_parties;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Clone)]
pub struct QuestCompletion {
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub quest: QuestRef,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Clone)]
pub struct QuestRef {
    pub questline: Option<QuestLineRef>,
    pub id: i64,
    pub name: String,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
pub struct QuestTask {
    pub task_type: String,
    pub subtasks: Vec<String>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
pub struct QuestDetails {
    pub id: i64,
    pub name: String,
    pub questline: Option<QuestLineRef>,
    pub desc: Vec<String>,
    pub unlocks: Vec<QuestRef>,
    pub requires: Vec<QuestRef>,
    pub tasks: Vec<QuestTask>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Clone)]
pub struct QuestLineRef {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct Data {
    pub quests: HashMap<i64, QuestDetails>,
    pub completions: Vec<QuestCompletion>,
}

pub fn fix_formatting(str: &str) -> String {
    let mut result = String::new();
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        if c == '§' {
            chars.next().unwrap();
        } else {
            result.push(c);
        }
    }
    result
}

pub fn load_data<P: AsRef<Path>>(dir: P) -> Data {
    let db = quest_database::parse(dir.as_ref().join("QuestDatabase.json")).unwrap();
    let quests = db
        .quest_database
        .values()
        .map(|q| (q.quest_id, q.clone()))
        .collect::<HashMap<i64, quest_database::Quest>>();

    let quest_ql: HashMap<i64, QuestLineRef> = db
        .quest_lines
        .values()
        .flat_map(|ql| {
            let name = fix_formatting(&ql.properties.betterquesting.name);
            let id = ql.line_id;
            let desc = fix_formatting(&ql.properties.betterquesting.desc);
            ql.quests
                .values()
                .map(|q| {
                    (
                        q.id,
                        QuestLineRef {
                            id,
                            name: name.clone(),
                        },
                    )
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect();

    let mut quest_unlocks: HashMap<i64, Vec<i64>> = HashMap::new();

    for q in quests.values() {
        for prereq_id in &q.pre_requisites {
            if quest_unlocks.contains_key(&prereq_id) {
                let old = quest_unlocks.get(&prereq_id).unwrap().to_vec();
                quest_unlocks
                    .insert(*prereq_id, {
                        let mut new: Vec<_> = old.as_slice().to_vec();
                        new.push(q.quest_id);
                        new
                    })
                    .unwrap();
            } else {
                quest_unlocks.insert(*prereq_id, vec![q.quest_id]);
            }
        }
    }
    let quest_unlocks = quest_unlocks;

    let quests: HashMap<i64, QuestDetails> = quests
        .values()
        .map(|q| QuestDetails {
            id: q.quest_id,
            name: fix_formatting(&q.properties.betterquesting.name),
            questline: quest_ql.get(&q.quest_id).map(QuestLineRef::to_owned),
            desc: q
                .properties
                .betterquesting
                .desc
                .lines()
                .map(fix_formatting)
                .collect(),
            unlocks: quest_unlocks
                .get(&q.quest_id)
                .iter()
                .flat_map(|x| x.iter())
                .map(|ref_quest_id| {
                    let quest = quests.get(&ref_quest_id).unwrap();
                    QuestRef {
                        id: *ref_quest_id,
                        name: fix_formatting(&quest.properties.betterquesting.name),
                        questline: quest_ql.get(ref_quest_id).map(QuestLineRef::to_owned),
                    }
                })
                .collect::<BinaryHeap<_>>()
                .into_sorted_vec(),
            requires: q
                .pre_requisites
                .iter()
                .map(|id| {
                    let ref_q = quests.get(id).unwrap();
                    QuestRef {
                        id: ref_q.quest_id,
                        name: fix_formatting(&ref_q.properties.betterquesting.name),
                        questline: quest_ql.get(&ref_q.quest_id).map(QuestLineRef::to_owned),
                    }
                })
                .collect(),
            tasks: q
                .tasks
                .values()
                .map(|t| match t.task_id {
                    quest_database::BqTaskType::StandardCheckbox => QuestTask {
                        task_type: "Checkbox".to_string(),
                        subtasks: Vec::default(),
                    },
                    quest_database::BqTaskType::StandardCrafting => QuestTask {
                        task_type: "Craft".to_string(),
                        subtasks: t
                            .required_items
                            .clone()
                            .unwrap()
                            .values()
                            .map(|i| {
                                format!(
                                    "{}x {} (dmg: {} nbt: {:?})",
                                    i.count, i.id, i.damage, i.nbt
                                )
                            })
                            .collect::<Vec<_>>(),
                    },
                    quest_database::BqTaskType::StandardFluid => QuestTask {
                        task_type: "Craft".to_string(),
                        subtasks: t
                            .required_fluids
                            .clone()
                            .unwrap()
                            .values()
                            .map(|i| format!("{}L of {}", i.amount, i.fluid_name))
                            .collect::<Vec<_>>(),
                    },
                    quest_database::BqTaskType::StandardHunt => QuestTask {
                        task_type: "Unsupported".to_string(),
                        subtasks: Vec::default(),
                    },
                    quest_database::BqTaskType::StandardLocation => QuestTask {
                        task_type: "Unsupported".to_string(),
                        subtasks: Vec::default(),
                    },
                    quest_database::BqTaskType::StandardRetrieval => QuestTask {
                        task_type: "Retrieve".to_string(),
                        subtasks: t
                            .required_items
                            .clone()
                            .unwrap()
                            .values()
                            .map(|i| {
                                format!(
                                    "{}x {} (dmg: {} nbt: {:?})",
                                    i.count, i.id, i.damage, i.nbt
                                )
                            })
                            .collect::<Vec<_>>(),
                    },
                })
                .collect(),
        })
        .map(|q| (q.id, q))
        .collect();
    let users = name_cache::parse(dir.as_ref().join("NameCache.json"))
        .unwrap()
        .name_cache
        .values()
        .map(|i| (i.uuid.clone(), i.name.clone()))
        .collect::<HashMap<String, String>>();

    let completions = quest_progress::parse(dir.as_ref().join("QuestProgress.json"))
        .unwrap()
        .quest_progress
        .values()
        .flat_map(|qp| {
            let q = QuestRef {
                id: qp.quest_id,
                name: quests.get(&qp.quest_id).unwrap().name.clone(),
                questline: quest_ql.get(&qp.quest_id).map(QuestLineRef::to_owned),
            };
            qp.completed
                .values()
                .map(|compl| QuestCompletion {
                    quest: q.clone(),
                    timestamp: Utc.timestamp_millis(compl.timestamp),
                    user: users.get(&compl.uuid).unwrap().to_owned(),
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<BinaryHeap<QuestCompletion>>()
        .into_sorted_vec();

    Data {
        quests,
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
            .values()
            .map(|q| (q.quest_id, q.clone()))
            .collect::<HashMap<i64, super::quest_database::Quest>>();
        let completions =
            crate::parsers::quest_progress::parse("./sample/1/QuestProgress.json").unwrap();

        let mut items = completions
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
            let quest = quests.get(&item.id).unwrap();
            println!(
                "{:?} {} {}",
                item.timestamp, item.user, quest.properties.betterquesting.name
            );
        }
    }
}

#![feature(proc_macro_hygiene, decl_macro)]
use ::betterquesting_report::parsers;
use chrono::TimeZone;
use chrono::{DateTime, Utc};
use rocket::State;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::{
    collections::{BinaryHeap, HashMap},
    path::PathBuf,
};

#[macro_use]
extern crate rocket;

struct FileLocation {
    betterquesting: PathBuf,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct QuestCompletion {
    timestamp: DateTime<Utc>,
    user: String,
    id: i64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct QuestDetails {
    name: String,
    desc: Vec<String>,
    user: String,
    time: String,
    unlocks: Vec<QuestRef>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct DateQuests {
    date: String,
    quests: Vec<QuestDetails>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct QuestRef {
    id: i64,
    name: String,
    questline: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct Ctx {
    quests: Vec<DateQuests>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn strip_formatting(str: &str) -> String {
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

fn clone_and_push<T: Clone>(v: &[T], i: T) -> Vec<T> {
    let mut v = v.to_vec();
    v.push(i);
    v
}

#[get("/quests/history")]
fn quests_history(file_loc: State<FileLocation>) -> Template {
    let db =
        parsers::quest_database::parse(file_loc.betterquesting.join("QuestDatabase.json")).unwrap();
    let quests = db
        .quest_database
        .iter()
        .map(|(_id, q)| (q.quest_id, q.clone()))
        .collect::<HashMap<i64, parsers::quest_database::Quest>>();
    let users = parsers::name_cache::parse(file_loc.betterquesting.join("NameCache.json"))
        .unwrap()
        .name_cache
        .iter()
        .map(|(_id, i)| (i.uuid.clone(), i.name.clone()))
        .collect::<HashMap<String, String>>();
    let completions =
        crate::parsers::quest_progress::parse(file_loc.betterquesting.join("QuestProgress.json"))
            .unwrap();

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
                    .insert(prereq_id.clone(), clone_and_push(&old, q.quest_id))
                    .unwrap();
            } else {
                quest_unlocks.insert(prereq_id.clone(), vec![q.quest_id]);
            }
        }
    }

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
    let mut date_quests = Vec::new();
    let mut day = Vec::new();
    while !items.is_empty() {
        let item = items.pop().unwrap();
        if item.timestamp.date() != last_date {
            if !day.is_empty() {
                date_quests.push(DateQuests {
                    date: last_date.format("%v").to_string(),
                    quests: day,
                });
            }
            last_date = item.timestamp.date();
            day = Vec::new();
        }
        let quest = quests.get(&item.id).unwrap();
        day.push(QuestDetails {
            name: strip_formatting(&quest.properties.betterquesting.name),
            desc: quest
                .properties
                .betterquesting
                .desc
                .lines()
                .map(|s| s.to_string())
                .collect(),
            user: users.get(&item.user).unwrap().clone(),
            time: item.timestamp.format("%T").to_string(),
            unlocks: quest_unlocks
                .get(&item.id)
                .iter()
                .flat_map(|x| x.iter())
                .map(|ref_quest_id| {
                    let quest = quests.get(&ref_quest_id).unwrap();
                    QuestRef {
                        id: ref_quest_id.clone(),
                        name: strip_formatting(&quest.properties.betterquesting.name),
                        questline: quest_ql.get(ref_quest_id).unwrap_or(&"Questline not found".to_string()).clone(),
                    }
                })
                .collect(),
        });
    }

    Template::render(
        "quests_history",
        &Ctx {
            quests: date_quests,
        },
    )
}

fn main() {
    rocket::ignite()
        .manage(FileLocation {
            betterquesting: "/mnt/data/old/services/GTNH/world/betterquesting/".into(),
        })
        .mount("/", routes![index, quests_history])
        .attach(Template::fairing())
        .launch();
}

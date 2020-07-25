#![feature(proc_macro_hygiene, decl_macro)]
use ::betterquesting_report::parsers;
use betterquesting_report::{self, strip_formatting};
use chrono::TimeZone;
use chrono::{DateTime, Local, Utc};
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
    id: i64,
    name: String,
    questline: String,
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
    questline: String,
    id: i64,
    name: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct Ctx {
    quests: Vec<DateQuests>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/quests/history")]
fn quests_history(file_loc: State<FileLocation>) -> Template {
    let data = parsers::load_data(&file_loc.betterquesting);

    let mut items = data
        .completions
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
    let mut last_date = Utc.timestamp_millis(0).with_timezone(&Local).date();
    let mut date_quests = Vec::new();
    let mut day = Vec::new();
    while !items.is_empty() {
        let item = items.pop().unwrap();
        if item.timestamp.with_timezone(&Local).date() != last_date {
            if !day.is_empty() {
                date_quests.push(DateQuests {
                    date: last_date.format("%v").to_string(),
                    quests: day,
                });
            }
            last_date = item.timestamp.with_timezone(&Local).date();
            day = Vec::new();
        }
        let quest = data.quests.get(&item.id).unwrap();
        day.push(QuestDetails {
            id: item.id,
            name: strip_formatting(&quest.properties.betterquesting.name),
            questline: data
                .quest_ql
                .get(&item.id)
                .unwrap_or(&"Questline not found".to_string())
                .clone(),
            desc: quest
                .properties
                .betterquesting
                .desc
                .lines()
                .map(|s| s.to_string())
                .collect(),
            user: data.users.get(&item.user).unwrap().clone(),
            time: item
                .timestamp
                .with_timezone(&Local)
                .format("%T")
                .to_string(),
            unlocks: data
                .quest_unlocks
                .get(&item.id)
                .iter()
                .flat_map(|x| x.iter())
                .map(|ref_quest_id| {
                    let quest = data.quests.get(&ref_quest_id).unwrap();
                    QuestRef {
                        id: ref_quest_id.clone(),
                        name: strip_formatting(&quest.properties.betterquesting.name),
                        questline: data
                            .quest_ql
                            .get(ref_quest_id)
                            .unwrap_or(&"Questline not found".to_string())
                            .clone(),
                    }
                })
                .collect::<BinaryHeap<_>>()
                .into_sorted_vec(),
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

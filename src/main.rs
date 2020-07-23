#![feature(proc_macro_hygiene, decl_macro)]
use ::betterquesting_report::parsers;
use rocket::State;
use std::{
    collections::{BinaryHeap, HashMap},
    path::{PathBuf},
};
use rocket_contrib::templates::Template;
use chrono::{Utc, DateTime};
use chrono::{Date, TimeZone};
use serde::Serialize;

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
    desc: String,
    user: String,
    time: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct DateQuests {
    date: String,
    quests: Vec<QuestDetails>
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct Ctx {
    quests: Vec<DateQuests>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/quests/history")]
fn quests_history(file_loc: State<FileLocation>) -> Template {
    let quests = parsers::quest_database::parse(
        file_loc.betterquesting.join("QuestDatabase.json"),
    )
    .unwrap()
    .quest_database
    .iter()
    .map(|(_id, q)| (q.quest_id, q.clone()))
    .collect::<HashMap<i64, parsers::quest_database::Quest>>();
    let completions =
        crate::parsers::quest_progress::parse(file_loc.betterquesting.join("QuestProgress.json")).unwrap();

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
        let mut day= Vec::new();
        while items.len() > 0 {
            let item = items.pop().unwrap();
            if item.timestamp.date() != last_date {                
                date_quests.push(DateQuests{date:last_date.format("%v").to_string(),quests:day});
                last_date = item.timestamp.date();
                day = Vec::new();
            }
            let quest = quests.get(&item.id).unwrap();
            day.push(QuestDetails{name:quest.properties.betterquesting.name.clone(), desc:quest.properties.betterquesting.desc.clone(),user:item.user, time:item.timestamp.format("%T").to_string()});
        }
        
    Template::render("quests_history", &Ctx{quests:date_quests})
}

fn main() {
    rocket::ignite()
        .manage(FileLocation {
            betterquesting: "./sample/1/".into(),
        })
        .mount("/", routes![index, quests_history]).attach(Template::fairing())
        .launch();
}

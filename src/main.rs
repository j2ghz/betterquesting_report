#![feature(proc_macro_hygiene, decl_macro)]
use ::betterquesting_report::parsers;
use parsers::{QuestCompletion, QuestDetails};
use rocket::State;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::{collections::BinaryHeap, path::PathBuf};

#[macro_use]
extern crate rocket;

struct FileLocation {
    betterquesting: PathBuf,
}

#[derive(Debug, Serialize)]
struct QuestHistoryCtx {
    quests: Vec<QuestCompletion>,
}

#[derive(Debug, Serialize)]
struct QuestDetailsCtx<'a> {
    quest: &'a QuestDetails,
}

#[derive(Debug, Serialize)]
struct IndexCtx {
    quests: Vec<QuestDetails>,
}

#[get("/")]
fn index(file_loc: State<FileLocation>) -> Template {
    let data = parsers::load_data(&file_loc.betterquesting);
    Template::render(
        "index",
        &IndexCtx {
            quests: data
                .quests
                .values()
                .map(|q| q.to_owned())
                .collect::<BinaryHeap<QuestDetails>>()
                .into_sorted_vec(),
        },
    )
}

#[get("/quests/history")]
fn quests_history(file_loc: State<FileLocation>) -> Template {
    let data = parsers::load_data(&file_loc.betterquesting);
    Template::render(
        "quests_history",
        &QuestHistoryCtx {
            quests: data.completions,
        },
    )
}

#[get("/quest/<id>")]
fn quest(id: i64, file_loc: State<FileLocation>) -> Template {
    let data = parsers::load_data(&file_loc.betterquesting);
    let quest = data.quests.get(&id).unwrap();
    Template::render("quest", &QuestDetailsCtx { quest: quest })
}

fn main() {
    rocket::ignite()
        .manage(FileLocation {
            betterquesting: "/mnt/data/old/services/GTNH/world/betterquesting/".into(),
        })
        .mount("/", routes![index, quests_history, quest])
        .attach(Template::fairing())
        .launch();
}

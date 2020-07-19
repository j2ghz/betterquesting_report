mod quest_database;
mod quest_progress;

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use std::{cmp::Ordering, collections::{HashMap, BinaryHeap}, fs};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct QuestCompletion {
        timestamp: DateTime<Utc>,
        user: String,
        id: i64,
    }

    #[test]
    fn desrialize_sample() {
        let quests = crate::parsers::quest_database::parse("./sample/1/QuestDatabase.json").unwrap().quest_database;
        let quests = quests.iter().map(|(_id,q)| (q.quest_id,q.clone())).collect::<HashMap<i64,super::quest_database::Quest>>();
        let completions = crate::parsers::quest_progress::parse("./sample/1/QuestProgress.json").unwrap();    

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
            println!("{:?} {} {}", item.timestamp,item.user, quest.properties.betterquesting.name);
        }
    }
}

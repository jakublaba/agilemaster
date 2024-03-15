use std::error::Error;
use std::fs::File;
use std::io::Write;

use chrono::{Duration, Local};

use crate::model::export::Export;
use crate::model::history_entry::HistoryEntry;
use crate::model::issue::Issue;
use crate::model::item::Item;
use crate::model::jira_date::JiraDate;
use crate::model::project::Project;
use crate::model::user::User;

mod model;

fn main() -> Result<(), Box<dyn Error>> {
    let export = Export::new(
        vec![
            User::new(
                "jakublaba",
                vec![],
                true,
                "jakub.maciej.laba@gmail.com",
                "Jakub Łaba",
            ),
        ],
        vec![
            Project::new(
                "kanban-test",
                "KT",
                vec![
                    Issue::new(
                        "DONE",
                        "606affea126db9006ff5c5de",
                        "Story",
                        JiraDate::Absolute(Local::now() - Duration::days(30)),
                        JiraDate::Absolute(Local::now()),
                        "Task 1",
                        vec![
                            HistoryEntry::new(
                                "606affea126db9006ff5c5de",
                                JiraDate::Absolute(Local::now() - Duration::days(55)),
                                vec![
                                    Item::new(
                                        "status",
                                        "TO DO",
                                        "IN PROGRESS",
                                    )
                                ],
                            ),
                            HistoryEntry::new(
                                "606affea126db9006ff5c5de",
                                JiraDate::Absolute(Local::now() - Duration::days(23)),
                                vec![
                                    Item::new(
                                        "status",
                                        "IN PROGRESS",
                                        "DONE",
                                    )
                                ],
                            ),
                        ],
                    ),
                    Issue::new(
                        "DONE",
                        "606affea126db9006ff5c5de",
                        "Story",
                        JiraDate::Absolute(Local::now() - Duration::days(30)),
                        JiraDate::Absolute(Local::now()),
                        "Task 2",
                        vec![
                            HistoryEntry::new(
                                "606affea126db9006ff5c5de",
                                JiraDate::Absolute(Local::now() - Duration::days(28)),
                                vec![
                                    Item::new(
                                        "status",
                                        "TO DO",
                                        "IN PROGRESS",
                                    )
                                ],
                            ),
                            HistoryEntry::new(
                                "606affea126db9006ff5c5de",
                                JiraDate::Absolute(Local::now() - Duration::days(2)),
                                vec![
                                    Item::new(
                                        "status",
                                        "IN PROGRESS",
                                        "DONE",
                                    )
                                ],
                            ),
                        ],
                    ),
                ],
            )
        ],
    );

    let json = serde_json::to_string_pretty(&export)?;
    let mut file = File::create("project.json")?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
mod prompt;
mod ui;
mod storage;

use serde::{Serialize, Deserialize};
use crate::storage::create_file_if_not_exists;

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const BLUE: &str = "\x1b[34m";
pub const RESET: &str = "\x1b[0m";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Stage {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
enum Priority {
    High = 1,
    Medium = 2,
    Low = 3,
}

impl Priority {
    fn from_u8(priority: u8) -> Option<Priority> {
        match priority {
            1 => Some(Priority::High),
            2 => Some(Priority::Medium),
            3 => Some(Priority::Low),
            _ => None,
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Medium => write!(f, "Medium"),
            Priority::Low => write!(f, "Low"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    stage: Stage,
    priority: Priority,
}

fn main() {
    eprintln!("Welcome to the sample todo list!");
    if let Err(err) = create_file_if_not_exists() {
        eprintln!("{}Error creating config file: {}{}", RED, err, RESET);
    }

    loop {
        let option = prompt::prompt_for_options();
        match option {
            Some(1) => ui::render_todo_list(),
            Some(2) => add_task(),
            Some(3) => break,
            Some(_) => eprintln!("Invalid option"),
            None => return,
        }
    }

}

fn add_task() {
    let task_name = prompt::prompt_for_task_name();
    if task_name.is_none() {
        eprintln!("task name cannot be empty");
        return;
    }

    let task_name: String = task_name.unwrap();

    let priority = prompt::prompt_for_priority();
    if priority.is_none() {
        eprintln!("priority cannot be empty");
        return;
    }


    let priority: u8 = priority.unwrap();

    let priority = match Priority::from_u8(priority) {
        Some(p) => p,
        None => {
            eprintln!("Priority must be 1, 2, or 3");
            return;
        }
    };

    let task = Task {
        name: task_name,
        priority,
        stage: Stage::Todo
    };

    storage::add_task(task);
}
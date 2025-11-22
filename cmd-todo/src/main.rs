mod prompt;
mod ui;
mod storage;

use serde::{Serialize, Deserialize};

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const BLUE: &str = "\x1b[34m";
pub const RESET: &str = "\x1b[0m";

#[derive(Serialize, Deserialize, Debug)]
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
    fn from_u8(priority: u8) -> Priority {
        match priority {
            1 => Priority::High,
            2 => Priority::Medium,
            _ => Priority::Low,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    completed: bool,
    stage: Stage,
    priority: Priority,
}

fn main() {
    eprintln!("Welcome to the sample todo list!");
    storage::create_file_if_not_exists();
    loop {
        let option = prompt::prompt_for_options();
        if option.is_none() {
            return;
        }

        match option.unwrap() {
            1 => ui::render_todo_list(),
            2 => add_task(),
            3 => break,
            _ => eprintln!("Invalid option")
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

    let task = Task {
        name: task_name,
        priority: Priority::from_u8(priority),
        completed: false,
        stage: Stage::Todo
    };

    storage::add_task(task);
}
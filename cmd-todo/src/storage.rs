use std::fs;
use std::path::PathBuf;
use std::io::Error;
use crate::{Task, GREEN, RESET};

const TASKS_FILE_NAME: &str = "tasks.json";

pub fn create_file_if_not_exists() -> Result<(), Error> {
    let tasks_file_path = get_tasks_file_path();

    if let Some(parent) = tasks_file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !tasks_file_path.exists() {
        fs::write(&tasks_file_path, "[]")?;
    }

    Ok(())
}


fn get_tasks_file_path() -> PathBuf {
    let home_dir = std::env::var("HOME").expect("HOME env variable not set");
    let data_dir = PathBuf::from(home_dir).join(".local/share/cmd-todo");
    data_dir.join(TASKS_FILE_NAME)
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let tasks_to_save = serde_json::to_string(tasks).expect("Failed to save tasks");
    let tasks_file_path = get_tasks_file_path();
    fs::write(tasks_file_path, tasks_to_save).expect("Failed to save tasks");
    eprintln!("{}Tasks saved successfully!{}", GREEN, RESET);
}

pub fn get_saved_tasks() -> Vec<Task> {
    let tasks_file_path = get_tasks_file_path();
    let data = fs::read_to_string(tasks_file_path);

    match data {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        },
        Err(_) => Vec::new(),
    }
}

pub fn add_task(task: Task) {
    let mut existing_tasks = get_saved_tasks();
    existing_tasks.push(task);
    save_tasks(&existing_tasks);

}
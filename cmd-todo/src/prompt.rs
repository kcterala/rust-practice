use std::io;
use crate::{ui, BLUE, RESET};
pub fn prompt_for_task_name() -> Option<String> {
    let mut task_name = String::new();
    eprintln!("{}Enter task name:{}", BLUE, RESET);
    io::stdin().read_line(&mut task_name).ok()?;
    let task_name = task_name.trim().to_string();
    Some(task_name)
}

pub fn prompt_for_priority() -> Option<u8> {
    let mut priority = String::new();
    eprintln!("{}Enter priority (1-3):{}", BLUE, RESET);
    io::stdin().read_line(&mut priority).ok()?;
    let priority = priority.trim().parse::<u8>().ok()?;
    Some(priority)
}

pub fn prompt_for_options() -> Option<u8> {
    ui::show_options();
    let mut option = String::new();
    io::stdin().read_line(&mut  option).ok();
    let option = option.trim().parse::<u8>().ok();
    option
}
use crate::{storage, Stage, BLUE, GREEN, RESET};


pub fn show_options() {
    eprintln!("1. Show current tasks");
    eprintln!("2. Add task");
    eprintln!("3. Exit");
}

pub fn render_todo_list() {
    let todo_list = storage::get_saved_tasks();
    eprintln!("\n{}========= Todo List ========={}", BLUE, RESET);

    if todo_list.is_empty() {
        eprintln!("{}No tasks yet!{}", GREEN, RESET);
        return;
    }

    for (index, task) in todo_list.iter().enumerate() {
        let status = if Stage::Done == task.stage {
            format!("{}[✓]{}", GREEN, RESET)
        } else {
            "[ ]".to_string()
        };

        eprintln!(
            "{}. {} {} (Priority: {})",
            index + 1,
            status,
            task.name,
            task.priority
        );
    }
    eprintln!();
}
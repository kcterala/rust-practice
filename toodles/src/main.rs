
use color_eyre::eyre::Result;
use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyEvent}, layout::{Constraint, Layout}, style::{Color, Style, Stylize}, text::ToSpan, widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget}};


#[derive(Debug, Default)]
struct TodoItem {
	is_done: bool,
	description: String,
}

#[derive(Debug, Default)]
struct AppState {
	items: Vec<TodoItem>,
	list_state: ListState,
	is_add_new: bool,
	input_value: String,
}

enum FormAction {
	None,
	Submit,
	Escape
}

fn main() -> Result<()> {
	let mut state = AppState::default();
	state.items.push(TodoItem { is_done: false, description: String::from("Finish application - desc 1") });
	state.items.push(TodoItem { is_done: false, description: String::from("Finish application - desc 2") });
	state.items.push(TodoItem { is_done: false, description: String::from("Finish application - desc 3") });
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
	loop {
		terminal.draw(|f| render(f, app_state))?;
		if let Event::Key(key) = event::read()? {
			if app_state.is_add_new {
				match handle_add_new(key, app_state) {
					FormAction::Escape => {
						app_state.is_add_new = true;
						app_state.input_value.clear();
					},
					FormAction::None => {},
					FormAction::Submit => {
						app_state.is_add_new = false;
						app_state.items.push(TodoItem { is_done: false, description: app_state.input_value.clone() });
						app_state.input_value.clear();
					}

				}

			} else {
				if handle_key(key, app_state) {
					break;
				}
			}

		}
	}
	Ok(())
}

fn handle_key(event: KeyEvent, app_state: &mut AppState) -> bool {
	match event.code {
		event::KeyCode::Esc => {
		 	return true
		}
		event::KeyCode::Char(char) => match char {
			'Q' => {
				return true
			}
			'A' => {
				app_state.is_add_new = true;
			},
			'k' => {
				app_state.list_state.select_previous();
			},
			'j' => {
				app_state.list_state.select_next();
			},
			'D' => {
				if let Some(index) = app_state.list_state.selected() {
					app_state.items.remove(index);
				}
			},
			_ => {}
		}
		_ => {}
	}

	false
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
	match key.code {
		event::KeyCode::Char(c) => {
			app_state.input_value.push(c);
		}
		event::KeyCode::Backspace => {
			app_state.input_value.pop();
		}
		event::KeyCode::Enter => {
			return FormAction::Submit;
		}
		event::KeyCode::Esc => {
			return FormAction::Escape;
		}
    	_ => {}
	}

	FormAction::None
}

fn render(f: &mut Frame, app_state: &mut AppState) {
	let constraints = vec![Constraint::Fill(1)];
	let [border_area] = Layout::vertical(constraints.clone())
		.margin(1)
		.areas(f.area());

	if app_state.is_add_new {
		Paragraph::new(app_state.input_value.as_str())
    		.block(Block::bordered()
      				.fg(Color::Green)
          			.padding(Padding::uniform(1))
             		.title(" Input Description ".to_span().into_centered_line())
          .border_type(BorderType::Rounded))
			.render(border_area, f.buffer_mut());
	} else {
		let [inner_area] = Layout::vertical(constraints.clone())
			.margin(2)
			.areas(border_area);


		Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(Color::Yellow)
			.render(border_area, f.buffer_mut());

		let list = List::new(
			app_state.items
			.iter()
			.map(|x| ListItem::from(x.description.clone()))
		).highlight_symbol(">")
		.highlight_style(Style::default().fg(Color::Green));

		f.render_stateful_widget(list, inner_area, &mut app_state.list_state);
	}

}

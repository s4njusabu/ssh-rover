use crossterm::event::{self, Event, KeyCode};

mod state;
mod ui;

use state::State;
use ui::{border, home};

use crate::state::Selected;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::new();

    loop {
        terminal.draw(|frame| {
            let inner = border::draw(frame, &state);

            match state.selected {
                Selected::Discovery => home::draw(frame, inner, &state),
                Selected::Dependencies => home::draw(frame, inner, &state),
                Selected::Themes => home::draw(frame, inner, &state),
                Selected::About => home::draw(frame, inner, &state),
                Selected::Exit => {}
            }
        })?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if state.hovered > 0 {
                        state.hovered -= 1;
                    }
                }

                KeyCode::Down => {
                    if state.hovered < home::ITEM_COUNT - 1 {
                        state.hovered += 1;
                    }
                }

                KeyCode::Enter => match state.hovered {
                    0 => state.selected = Selected::Dependencies,
                    1 => state.selected = Selected::Dependencies,
                    2 => state.selected = Selected::Themes,
                    3 => state.selected = Selected::About,
                    4 => state.selected = Selected::Exit,
                    _ => {}
                },

                KeyCode::Esc => break,
                _ => (),
            }

            if state.selected == Selected::Exit {
                break;
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

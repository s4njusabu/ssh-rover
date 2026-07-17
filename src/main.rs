use crossterm::event::{self, Event, KeyCode};

mod app;
mod ui;

use app::App;
use ui::{border, home};

use crate::app::Selected;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let inner = border::draw(frame, &app);

            match app.selected {
                Selected::Discovery => home::draw(frame, inner, &app),
                Selected::Dependencies => home::draw(frame, inner, &app),
                Selected::Themes => home::draw(frame, inner, &app),
                Selected::About => home::draw(frame, inner, &app),
                Selected::Exit => {}
            }
        })?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if app.hovered > 0 {
                        app.hovered -= 1;
                    }
                }

                KeyCode::Down => {
                    if app.hovered < home::ITEM_COUNT - 1 {
                        app.hovered += 1;
                    }
                }

                KeyCode::Enter => match app.hovered {
                    0 => app.selected = Selected::Dependencies,
                    1 => app.selected = Selected::Dependencies,
                    2 => app.selected = Selected::Themes,
                    3 => app.selected = Selected::About,
                    4 => app.selected = Selected::Exit,
                    _ => {}
                },

                KeyCode::Esc => break,
                _ => (),
            }

            if app.selected == Selected::Exit {
                break;
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

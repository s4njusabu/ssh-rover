use crossterm::event::{self, Event, KeyCode};

mod app;
mod ui;

use app::App;
use ui::{border, home};

use crate::app::View;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let inner = border::draw(frame, &app);

            match app.view {
                View::Home => home::draw(frame, inner, &app),
                View::QuickConnect => home::draw(frame, inner, &app),
                View::SavedHosts => home::draw(frame, inner, &app),
                View::Dependencies => home::draw(frame, inner, &app),
                View::About => home::draw(frame, inner, &app),
                View::Exit => {}
            }
        })?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }

                KeyCode::Down => {
                    if app.selected < home::MENU_ITEMS.len() - 1 {
                        app.selected += 1;
                    }
                }

                KeyCode::Enter => match app.selected {
                    0 => app.view = View::Home,
                    1 => app.view = View::QuickConnect,
                    2 => app.view = View::SavedHosts,
                    3 => app.view = View::Dependencies,
                    4 => app.view = View::About,
                    5 => app.view = View::Exit,
                    _ => {}
                },

                KeyCode::Esc => break,
                _ => (),
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

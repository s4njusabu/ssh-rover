use crossterm::event::{self, Event, KeyCode};

mod app;
mod ui;

use app::App;
use ui::{border, home};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            border::draw(frame);
            home::draw(frame, &app);
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

                KeyCode::Esc => break,
                _ => (),
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

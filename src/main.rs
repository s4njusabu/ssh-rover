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
                View::Discovery => home::draw(frame, inner, &app),
                View::Dependencies => home::draw(frame, inner, &app),
                View::Themes => home::draw(frame, inner, &app),
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
                    if app.selected < home::ITEM_COUNT - 1 {
                        app.selected += 1;
                    }
                }

                KeyCode::Enter => match app.selected {
                    0 => app.view = View::Dependencies,
                    1 => app.view = View::Dependencies,
                    2 => app.view = View::Themes,
                    3 => app.view = View::About,
                    4 => app.view = View::Exit,
                    _ => {}
                },

                KeyCode::Esc => break,
                _ => (),
            }

            if app.view == View::Exit {
                break;
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

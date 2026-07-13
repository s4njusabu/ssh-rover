use crossterm::event::{self, Event, KeyCode};

mod ui;
use ui::{border, home};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| {
            border::draw(frame);
            home::draw(frame);
        })?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Esc => break,
                _ => (),
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

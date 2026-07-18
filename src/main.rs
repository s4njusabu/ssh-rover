use crossterm::event::{self, Event, KeyCode};

mod state;
mod ui;

use state::State;
use ui::{border, home};

use crate::{state::Selected, ui::panes::pane2};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::new();

    loop {
        terminal.draw(|frame| {
            let inner = border::draw(frame, &state);

            match state.selected {
                Selected::Discovery(_) => home::draw(frame, inner, &state),
                Selected::Dependencies(_) => home::draw(frame, inner, &state),
                Selected::Themes(_) => home::draw(frame, inner, &state),
                Selected::About(_) => home::draw(frame, inner, &state),
                _ => {}
            }
        })?;

        if state.in_pane1 {
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
                        0 => {
                            state.selected = Selected::Discovery(pane2::discovery::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;

                            state.pane2_hovered = Some(0);
                        }
                        1 => {
                            state.selected =
                                Selected::Dependencies(pane2::dependencies::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        2 => {
                            state.selected = Selected::Themes(pane2::themes::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        3 => {
                            state.selected = Selected::About(pane2::about::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        4 => {
                            state.selected = Selected::Exit;
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        _ => {}
                    },
                    KeyCode::Esc => break,
                    _ => {}
                }

                if state.selected == Selected::Exit {
                    break;
                }
            }
        } else {
            if let Event::Key(key_event) = event::read()? {

                match state.selected {
                    Selected::Discovery(index)
                    | Selected::Dependencies(index)
                    | Selected::Themes(index)
                    | Selected::About(index) => match key_event.code {
                        KeyCode::Up => {
                            let t1 = state.pane2_hovered.unwrap();
                            if t1 > 0 {
                                state.pane2_hovered = Some(t1 - 1);
                            }
                        }
                        KeyCode::Down => {
                            let t1 = state.pane2_hovered.unwrap();

                            if t1 < index - 1 {
                                let t2 = t1 + 1;
                                state.pane2_hovered = Some(t2);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

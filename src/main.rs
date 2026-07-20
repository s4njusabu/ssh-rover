use crossterm::event::{self, Event, KeyCode};

mod services;
mod state;
mod ui;

use ratatui::{style::Style, widgets::Block};
use state::State;
use ui::{border, home};

use crate::{
    state::Pane1,
    ui::{panes::pane2, themes::Theme},
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::new();

    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                Block::default().style(Style::default().bg(state.theme.colors().background)),
                frame.area(),
            );
            let inner = border::draw(frame, &state);

            match state.pane1_selected {
                Pane1::Discovery(_) => home::draw(frame, inner, &state),
                Pane1::Dependencies(_) => home::draw(frame, inner, &state),
                Pane1::Themes(_) => home::draw(frame, inner, &state),
                Pane1::About(_) => home::draw(frame, inner, &state),
                _ => {}
            }
        })?;

        if state.in_pane1 {
            // In pane 1
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
                            state.pane1_selected = Pane1::Discovery(pane2::discovery::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;

                            state.pane2_hovered = Some(0);
                        }
                        1 => {
                            state.pane1_selected =
                                Pane1::Dependencies(pane2::dependencies::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        2 => {
                            state.pane1_selected = Pane1::Themes(pane2::themes::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        3 => {
                            state.pane1_selected = Pane1::About(pane2::about::ITEM_COUNT);
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        4 => {
                            state.pane1_selected = Pane1::Exit;
                            state.in_pane1 = false;
                            state.in_pane2 = true;
                            state.pane2_hovered = Some(0);
                        }
                        _ => {}
                    },
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    _ => {}
                }

                if state.pane1_selected == Pane1::Exit {
                    break;
                }
            }
        } else {
            // In pane 2
            if let Event::Key(key_event) = event::read()? {
                match state.pane1_selected {
                    Pane1::Discovery(index)
                    | Pane1::Dependencies(index)
                    | Pane1::Themes(index)
                    | Pane1::About(index) => match key_event.code {
                        KeyCode::Up => {
                            let t1 = state.pane2_hovered.unwrap();
                            if t1 > 0 {
                                state.pane2_hovered = Some(t1 - 1);
                            }

                            state.pane2_selected = usize::MAX;
                        }
                        KeyCode::Down => {
                            let t1 = state.pane2_hovered.unwrap();

                            if t1 < index - 1 {
                                let t2 = t1 + 1;
                                state.pane2_hovered = Some(t2);
                            }
                            state.pane2_selected = usize::MAX;
                        }
                        KeyCode::Enter => {
                            state.pane2_selected = state.pane2_hovered.unwrap();
                            if state.pane2_selected == index - 1 {
                                state.in_pane1 = true;
                                state.in_pane2 = false;
                                state.pane2_hovered = None;
                                state.pane2_selected = usize::MAX;
                            }
                        }
                        KeyCode::Esc | KeyCode::Char('q') if !state.in_pane3 => {
                            break;
                        }
                        _ => {}
                    },

                    _ => {}
                }

                if state.in_pane2 {
                    match state.pane1_selected {
                        Pane1::Discovery(_) => {}
                        Pane1::Dependencies(_) => match state.pane2_selected {
                            1 => state.pane3_selected = 1,
                            2 => state.pane3_selected = 2,
                            3 => state.pane3_selected = 3,
                            _ => {}
                        },
                        Pane1::Themes(_) => match state.pane2_selected {
                            0 => state.theme = Theme::Default,
                            1 => state.theme = Theme::Red,
                            2 => state.theme = Theme::Blue,
                            3 => state.theme = Theme::Green,
                            4 => state.theme = Theme::Yellow,
                            5 => state.theme = Theme::Magenta,
                            6 => state.theme = Theme::Gray,
                            _ => {}
                        },
                        Pane1::About(_) => {}
                        Pane1::Exit => {}
                    }
                }
            }
        }
    }

    ratatui::restore();
    println!("Bye from MuxSSH!");
    Ok(())
}

use crossterm::event::{self, Event, KeyCode};

mod services;
mod state;
mod ui;

use ratatui::{style::Style, widgets::Block};
use state::State;
use ui::{border, home};

use crate::{
    state::Pane1,
    ui::{
        home_for_discovery::{self, draw_pane_3},
        panes::{pane2, pane3::discovery},
        themes::Theme,
    },
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
                Pane1::Discovery(_) => {
                    let output_area = home_for_discovery::draw(frame, inner, &state);

                    let rect_of_draw_pane_3 = draw_pane_3(frame, output_area, &state);

                    match state.pane2_hovered {
                        Some(0) => {
                            discovery::scan_current_network::draw(
                                frame,
                                rect_of_draw_pane_3,
                                &state,
                            );
                        }

                        Some(1) => {
                            discovery::scan_cidr_range::draw(frame, rect_of_draw_pane_3, &state);
                        }

                        Some(2) => {
                            discovery::manual_connect::draw(frame, rect_of_draw_pane_3, &state);
                        }

                        _ => {}
                    }
                }

                Pane1::Dependencies(_) => home::draw(frame, inner, &state),
                Pane1::Themes(_) => home::draw(frame, inner, &state),
                Pane1::Project(_) => home::draw(frame, inner, &state),
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
                            state.pane1_selected = Pane1::Project(pane2::project::ITEM_COUNT);
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
                    KeyCode::Right => match state.hovered {
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
                            state.pane1_selected = Pane1::Project(pane2::project::ITEM_COUNT);
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
        } else if state.in_pane3 {
            if let Event::Key(key_event) = event::read()? {
                if state.pane2_selected == 0 {
                    if state.entering_username {
                        match key_event.code {
                            KeyCode::Char(c) => {
                                state.username.push(c);
                            }
                            KeyCode::Backspace => {
                                state.username.pop();
                            }
                            KeyCode::Enter => {
                                ratatui::restore();

                                println!();
                                println!("SSH Rover");
                                println!(
                                    "Connecting to {}@{}...",
                                    state.username, state.scanned_ips[state.selected_ip]
                                );
                                println!("Enter the password of {}", state.username);
                                std::process::Command::new("ssh")
                                    .arg(format!(
                                        "{}@{}",
                                        state.username, state.scanned_ips[state.selected_ip]
                                    ))
                                    .status()
                                    .unwrap();

                                println!("Bye from SSH Rover!");

                                return Ok(());
                            }
                            KeyCode::Esc => {
                                state.entering_username = false;
                                state.username.clear();
                            }
                            _ => {}
                        }
                    } else {
                        match key_event.code {
                            KeyCode::Up => {
                                if state.selected_ip > 0 {
                                    state.selected_ip -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if state.selected_ip < state.scanned_ips.len() - 1 {
                                    state.selected_ip += 1;
                                }
                            }
                            KeyCode::Enter => {
                                state.entering_username = true;
                            }
                            KeyCode::Esc | KeyCode::Left => {
                                state.selected_ip = 0;
                                state.username.clear();
                                state.in_pane3 = false;
                                state.in_pane2 = true;
                            }
                            _ => {}
                        }
                    }
                } else if state.pane2_selected == 1 {
                    if state.entering_cidr {
                        match key_event.code {
                            KeyCode::Char(c) => {
                                state.cidr_range.push(c);
                            }
                            KeyCode::Backspace => {
                                state.cidr_range.pop();
                            }
                            KeyCode::Enter => {
                                state.scanned_ips =
                                    crate::services::discovery::scan_cidr_range::scan_cidr_range(
                                        &state.cidr_range,
                                    );
                                state.entering_cidr = false;
                                state.selected_ip = 0;
                            }
                            KeyCode::Esc => {
                                state.cidr_range.clear();
                                state.entering_cidr = false;
                                state.in_pane3 = false;
                                state.in_pane2 = true;
                            }
                            _ => {}
                        }
                    } else if state.entering_username {
                        match key_event.code {
                            KeyCode::Char(c) => {
                                state.username.push(c);
                            }
                            KeyCode::Backspace => {
                                state.username.pop();
                            }
                            KeyCode::Enter => {
                                ratatui::restore();

                                println!();
                                println!("SSH Rover");
                                println!(
                                    "Connecting to {}@{}...",
                                    state.username, state.scanned_ips[state.selected_ip]
                                );
                                println!("Enter the password of {}", state.username);
                                std::process::Command::new("ssh")
                                    .arg(format!(
                                        "{}@{}",
                                        state.username, state.scanned_ips[state.selected_ip]
                                    ))
                                    .status()
                                    .unwrap();

                                println!("Bye from SSH Rover!");

                                return Ok(());
                            }
                            KeyCode::Esc => {
                                state.entering_username = false;
                                state.username.clear();
                            }
                            _ => {}
                        }
                    } else if state.scanned_ips.is_empty() {
                        match key_event.code {
                            KeyCode::Esc | KeyCode::Left => {
                                state.cidr_range.clear();
                                state.in_pane3 = false;
                                state.in_pane2 = true;
                            }
                            _ => {}
                        }
                    } else {
                        match key_event.code {
                            KeyCode::Up => {
                                if state.selected_ip > 0 {
                                    state.selected_ip -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if state.selected_ip < state.scanned_ips.len() - 1 {
                                    state.selected_ip += 1;
                                }
                            }
                            KeyCode::Enter => {
                                state.entering_username = true;
                            }
                            KeyCode::Esc | KeyCode::Left => {
                                state.cidr_range.clear();
                                state.selected_ip = 0;
                                state.username.clear();
                                state.in_pane3 = false;
                                state.in_pane2 = true;
                            }
                            _ => {}
                        }
                    }
                } else if state.pane2_selected == 2 {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            if state.entering_manual_username {
                                state.manual_username.push(c);
                            } else {
                                state.manual_ip.push(c);
                            }
                        }

                        KeyCode::Backspace => {
                            if state.entering_manual_username {
                                state.manual_username.pop();
                            } else {
                                state.manual_ip.pop();
                            }
                        }
                        KeyCode::Enter => {
                            if state.entering_manual_username {
                                ratatui::restore();

                                println!();
                                println!("SSH Rover");
                                println!(
                                    "Connecting to {}@{}...",
                                    state.manual_username, state.manual_ip
                                );
                                println!("Enter the password of {}", state.manual_username);
                                std::process::Command::new("ssh")
                                    .arg(format!("{}@{}", state.manual_username, state.manual_ip))
                                    .status()
                                    .unwrap();

                                println!("Bye from SSH Rover!");

                                return Ok(());
                            } else {
                                state.entering_manual_username = true;
                            }
                        }

                        KeyCode::Esc | KeyCode::Left => {
                            state.manual_ip.clear();
                            state.manual_username.clear();
                            state.entering_manual_username = false;

                            state.in_pane3 = false;
                            state.in_pane2 = true;
                        }

                        _ => {}
                    }
                }
            }
        } else {
            // In pane 2
            if let Event::Key(key_event) = event::read()? {
                match state.pane1_selected {
                    Pane1::Discovery(index) => match key_event.code {
                        KeyCode::Up => {
                            let t1 = state.pane2_hovered.unwrap();
                            if t1 > 0 {
                                state.pane2_hovered = Some(t1 - 1);
                                state.scanned_ips.clear();
                                state.selected_ip = 0;
                                state.username.clear();
                                state.entering_username = false;

                                state.cidr_range.clear();
                                state.entering_cidr = true;

                                state.manual_ip.clear();
                                state.manual_username.clear();
                                state.entering_manual_username = false;
                            }

                            state.pane2_selected = usize::MAX;
                        }
                        KeyCode::Down => {
                            let t1 = state.pane2_hovered.unwrap();

                            if t1 < index - 1 {
                                let t2 = t1 + 1;
                                state.pane2_hovered = Some(t2);

                                state.scanned_ips.clear();
                                state.selected_ip = 0;
                                state.username.clear();
                                state.entering_username = false;

                                state.cidr_range.clear();
                                state.entering_cidr = true;

                                state.manual_ip.clear();
                                state.manual_username.clear();
                                state.entering_manual_username = false;
                            }
                            state.pane2_selected = usize::MAX;
                        }
                        KeyCode::Enter | KeyCode::Right => {
                            state.pane2_selected = state.pane2_hovered.unwrap();
                            state.pane3_selected = state.pane2_selected;

                            match state.pane2_selected {
                                0 => {
                                    state.scanned_ips =
            crate::services::discovery::scan_current_network::scan_current_network();

                                    if !state.scanned_ips.is_empty() {
                                        state.selected_ip = 0;
                                        state.in_pane2 = false;
                                        state.in_pane3 = true;
                                    }
                                }

                                1 => {
                                    state.cidr_range.clear();
                                    state.scanned_ips.clear();
                                    state.entering_cidr = true;

                                    state.in_pane2 = false;
                                    state.in_pane3 = true;
                                }

                                2 => {
                                    state.manual_ip.clear();
                                    state.manual_username.clear();
                                    state.entering_manual_username = false;

                                    state.in_pane2 = false;
                                    state.in_pane3 = true;
                                }

                                _ => {}
                            }

                            if state.pane2_selected == index - 1 {
                                state.in_pane1 = true;
                                state.in_pane2 = false;
                                state.pane2_hovered = None;
                                state.pane2_selected = usize::MAX;
                            }
                        }
                        KeyCode::Left => {
                            state.in_pane1 = true;
                            state.in_pane2 = false;
                            state.pane2_hovered = None;
                            state.pane2_selected = usize::MAX;
                        }
                        KeyCode::Esc | KeyCode::Char('q') if !state.in_pane3 => {
                            break;
                        }
                        _ => {}
                    },
                    Pane1::Dependencies(index) | Pane1::Themes(index) | Pane1::Project(index) => {
                        match key_event.code {
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
                            KeyCode::Enter | KeyCode::Right => {
                                state.pane2_selected = state.pane2_hovered.unwrap();
                                if state.pane2_selected == index - 1 {
                                    state.in_pane1 = true;
                                    state.in_pane2 = false;
                                    state.pane2_hovered = None;
                                    state.pane2_selected = usize::MAX;
                                }

                                state.pane3_selected = state.pane2_selected;
                            }
                            KeyCode::Left => {
                                state.in_pane1 = true;
                                state.in_pane2 = false;
                                state.pane2_hovered = None;
                                state.pane2_selected = usize::MAX;
                            }
                            KeyCode::Esc | KeyCode::Char('q') if !state.in_pane3 => {
                                break;
                            }
                            _ => {}
                        }
                    }

                    _ => {}
                }

                if state.in_pane2 {
                    match state.pane1_selected {
                        Pane1::Dependencies(_) | Pane1::Project(_) | Pane1::Exit => {}
                        Pane1::Discovery(_) => {}
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
                    }
                }
            }
        }
    }

    ratatui::restore();
    println!("Bye from SSH Rover!");
    Ok(())
}

use eframe::egui;
use crate::core::board;
use crate::core::rule;

type GameRule = rule::OmokRule;

#[derive(PartialEq, Debug, Clone, Copy)] // Added Clone, Copy for later reset example
enum AppMode {
    Game, About,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum GameStatus {
    Ongoing, Ended,
}

pub struct MyApp {
    current_mode: AppMode,
    board: board::Board,
    game_status: GameStatus,
}

// Manually implement the Default trait for MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_mode: AppMode::Game, // Start in View mode by default
            board: board::Board::blank(),
            game_status: GameStatus::Ongoing,
        }
    }
}
// Replace the previous impl eframe::App for MyApp block with this enhanced version
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // --- Top Panel for Mode Switching (Example) ---
        egui::TopBottomPanel::top("mode_switcher").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Menu:");
                // Use radio buttons to switch the app mode state (we'll use self.current_mode)
                ui.radio_value(&mut self.current_mode, AppMode::Game, "Game");
                ui.radio_value(&mut self.current_mode, AppMode::About, "About");
            });
        });

        // --- Central Panel for Content based on Mode ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Current Mode: {:?}", self.current_mode)); // Show the mode
            ui.separator();

            // `match` allows us to render different UI based on the current mode
            match self.current_mode {
                AppMode::Game => {
                    let mut text = format!("Turn: {:?}", self.board.turn);
                    if self.game_status == GameStatus::Ended {
                        text = format!("{:?} wins.", self.board.turn.next());
                    }
                    ui.label(text);

                    let (resp, painter) =
                        ui.allocate_painter(egui::Vec2::splat(360.0), egui::Sense::click());

                    let rect = resp.rect;
                    let cell = rect.width() / 15.0;

                    // input handling
                    if resp.clicked() && self.game_status == GameStatus::Ongoing {
                        if let Some(pos) = resp.interact_pointer_pos() {
                            let local = (pos - resp.rect.min)
                                .clamp(eframe::emath::Vec2::ZERO, rect.size());
                            let coord = (local / cell).floor();

                            if let Some(mv) = board::Move::new(coord.x as usize, coord.y as usize) {
                                let player = self.board.turn;
                                let put = <GameRule as rule::Rule>::put(
                                    &mut self.board, mv, player,
                                );
                                match put {
                                    Ok(rule::PutOutcome::Continue) => {
                                        println!("successfully put {:?}", coord);
                                    },
                                    Ok(rule::PutOutcome::Win(p)) => {
                                        println!("{:?} wins!", p);
                                        self.game_status = GameStatus::Ended;
                                    },
                                    Ok(rule::PutOutcome::Draw) => {
                                        println!("Draw!");
                                        self.game_status = GameStatus::Ended;
                                    },
                                    Err(rule::PutError::Occupied) => {
                                        println!("This position is already occupied");
                                    }
                                }
                            }
                        }
                    }

                    // drawing grid
                    for i in 0..15 {
                        let x = rect.left() + cell * (i as f32 + 0.5);
                        painter.line_segment(
                            [
                                egui::pos2(x, rect.top()),
                                egui::pos2(x, rect.bottom()),
                            ],
                            egui::Stroke::new(1.0, egui::Color32::GRAY),
                        );
                        let y = rect.top() + cell * (i as f32 + 0.5);
                        painter.line_segment(
                            [
                                egui::pos2(rect.left(), y),
                                egui::pos2(rect.right(), y),
                            ], 
                            egui::Stroke::new(1.0, egui::Color32::GRAY),
                        );
                    }

                    // drawing stones
                    for i in 0..15 {
                        for j in 0..15 {
                            let mv = board::Move::new(i, j).unwrap();
                            let stone = self.board.get(mv);
                            if stone != board::Stone::None {
                                let x = rect.left() + cell * (i as f32 + 0.5);
                                let y = rect.top() + cell * (j as f32 + 0.5);
                                let fill_color = match stone {
                                    board::Stone::Black => egui::Color32::BLACK,
                                    board::Stone::White => egui::Color32::WHITE,
                                    board::Stone::None => unreachable!(),
                                };
                                let center = egui::Pos2::new(x, y);
                                painter.circle(center, 10.0, fill_color, egui::Stroke::new(2.0, egui::Color32::DARK_GRAY));
                                // painter.circle_filled(center, 8.0, fill_color);
                            }
                        }
                    }
                }

                AppMode::About => {
                    ui.label("Legend omok game amado");
                    ui.label("made by mark-2008");
                }
            }
        });
    }
}

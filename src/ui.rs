use std::thread;
use std::sync::{Arc, mpsc};
use eframe::egui;
use crate::core::board;
use crate::core::rule::{self, Rule};
use crate::bot::{
    eval,
    model::{self, Model},
    prune,
};

#[derive(PartialEq, Debug, Clone, Copy)] // Added Clone, Copy for later reset example
enum AppMode {
    PvpGame, BotGame, About,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum GameStatus {
    Ongoing, Ended,
}

struct GameData {
    board: board::Board,
    game_status: GameStatus,
    rule: Arc<dyn Rule>,
}

pub struct MyApp {
    current_mode: AppMode,
    pvp_data: GameData,
    bot_data: GameData,
    bot_context: Option<BotContext>,
}

impl GameData {
    fn new() -> Self {
        Self {
            board: board::Board::blank(),
            game_status: GameStatus::Ongoing,
            rule: Arc::new(rule::OmokRule),
        }
    }
}

// Manually implement the Default trait for MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_mode: AppMode::BotGame, // Start in View mode by default
            pvp_data: GameData::new(),
            bot_data: GameData::new(),
            bot_context: None,
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
                ui.radio_value(&mut self.current_mode, AppMode::PvpGame, "PvP");
                ui.radio_value(&mut self.current_mode, AppMode::BotGame, "Bot");
                ui.radio_value(&mut self.current_mode, AppMode::About, "About");
            });
        });

        // --- Central Panel for Content based on Mode ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Current Mode: {:?}", self.current_mode)); // Show the mode
            ui.separator();

            // `match` allows us to render different UI based on the current mode
            match self.current_mode {
                AppMode::PvpGame => {
                    omok_template(&mut self.pvp_data, ui, self.current_mode, &mut self.bot_context);
                }

                AppMode::BotGame => {
                    omok_template(&mut self.bot_data, ui, self.current_mode, &mut self.bot_context);
                }

                AppMode::About => {
                    ui.label("Legend omok game amado");
                    ui.label("made by mark-2008");
                }
            }
        });
    }
}

struct BotContext {
    rx: mpsc::Receiver<Option<board::Move>>,
}

fn omok_template(data: &mut GameData, ui: &mut egui::Ui, current_mode: AppMode, bot_context: &mut Option<BotContext>) {
    let mut text = format!("Turn: {:?}", data.board.turn());
    if data.game_status == GameStatus::Ended {
        text = format!("{:?} wins.", data.board.turn().next());
    }
    ui.label(text);

    let (resp, painter) =
        ui.allocate_painter(egui::Vec2::splat(360.0), egui::Sense::click());

    let rect = resp.rect;
    let cell = rect.width() / 15.0;

    // input handling
    if current_mode == AppMode::PvpGame || (current_mode == AppMode::BotGame && bot_context.is_none()) {
        if resp.clicked() && data.game_status == GameStatus::Ongoing {
            if let Some(pos) = resp.interact_pointer_pos() {
                let local = (pos - resp.rect.min)
                    .clamp(eframe::emath::Vec2::ZERO, rect.size());
                let coord = (local / cell).floor();

                if let Some(mv) = board::Move::new(coord.x as usize, coord.y as usize) {
                    let player = data.board.turn();
                    let put = data.rule.put(
                        &mut data.board, mv, player,
                    );
                    match put {
                        Ok(rule::PutOutcome::Continue) => {
                            tracing::debug!("successfully put {:?}", coord);
                            let new_board = data.board.clone();
                            let rule = data.rule.clone();
                            // legend spagetti code warning!!!!!!!!!!!!!!!!!!!!!!!
                            // todo: fix to reference

                            if current_mode == AppMode::BotGame {
                                let (tx, rx) = mpsc::channel();
                                
                                let _ = thread::spawn(move || {
                                    let model = model::NegamaxModel {
                                        depth: 4,
                                        eval: eval::BaboEval { rule: rule },
                                        prune: prune::NeighborPrune {},
                                        rule: rule::OmokRule {},
                                    };
                                    let selection = model.next_move(&new_board, mv);
                                    tx.send(selection).unwrap();
                                });
                                // might be end of the spagetti

                                *bot_context = Some(BotContext {
                                    rx: rx
                                });
                            }
                        },
                        Ok(rule::PutOutcome::Win) => {
                            tracing::debug!("someone wins!");
                            data.game_status = GameStatus::Ended;
                        },
                        Ok(rule::PutOutcome::Draw) => {
                            tracing::debug!("Draw!");
                            data.game_status = GameStatus::Ended;
                        },
                        Err(rule::PutError::Occupied) => {
                            tracing::debug!("This position is already occupied");
                        },
                        Err(rule::PutError::Invalid) => {
                            tracing::debug!("This position is invalid");
                        }
                    }
                }
            }
        }
    } else {    // when bot running
        if let Some(ctx) = bot_context.as_mut() {
            if let Ok(received) = ctx.rx.try_recv() {
                if let Some(mv) = received {
                    let put = data.rule.put(&mut data.board, mv, board::Turn::White);
                    match put {
                        Ok(rule::PutOutcome::Continue) => {
                            tracing::debug!("successfully put {:?}", mv);
                            *bot_context = None;
                        },
                        Ok(rule::PutOutcome::Win) => {
                            tracing::debug!("someone wins!");
                            data.game_status = GameStatus::Ended;
                        },
                        Ok(rule::PutOutcome::Draw) => {
                            tracing::debug!("Draw!");
                            data.game_status = GameStatus::Ended;
                        },
                        Err(rule::PutError::Occupied) => {
                            tracing::debug!("aaaaaaaaaa This position is already occupied");
                        },
                        Err(rule::PutError::Invalid) => {
                            tracing::debug!("aaaaaaaaaa This position is invalid");
                        }
                    }
                } else {
                    tracing::debug!("bot resigned!!");
                    data.game_status = GameStatus::Ended;
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
            let stone = data.board.get(mv);
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
use std::sync::{Arc, mpsc, RwLock};
use std::thread;
use std::time::Duration;
use eframe::egui;
use crate::core::board::{Board, Move, Player, Stone, Turn};
use crate::core::rule::{self, Rule, PutOutcome, PutError};
use crate::bot::model::{self, Model};
use crate::bot::eval;
use crate::bot::prune;


// The app is consisted of independent games.

pub struct MyApp {
    mode: AppMode,
    games: GameGroup,
    ui_setting: UiSetting,
    input_manager: Arc<RwLock<InputManager>>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum AppMode {
    Pvp, Bot, TwoBot, About,
}

impl Default for MyApp {
    fn default() -> Self {
        let rule = rule::OmokRule;
        let input_manager = Arc::new(RwLock::new(InputManager::default()));

        let mut result = Self {
            mode: AppMode::Bot,     // default mode
            games: GameGroup {
                pvp: GameData::new(
                    HumanPlayer { input_manager: Arc::clone(&input_manager) }, 
                    HumanPlayer { input_manager: Arc::clone(&input_manager) }, 
                    rule.clone()
                ),
                bot: GameData::new(
                    HumanPlayer { input_manager: Arc::clone(&input_manager) }, 
                    BotPlayer::new(
                        model::NegamaxModel {
                            depth: 5,
                            eval: eval::PatternEval { rule: rule.clone() },
                            prune: prune::NeighborPrune,
                            rule: rule.clone(),
                        }
                    ),
                    rule.clone(),
                ),
                twobot: GameData::new(
                    BotPlayer::new(
                        model::NegamaxModel {
                            depth: 5,
                            eval: eval::PatternEval { rule: rule.clone() },
                            prune: prune::NeighborPrune,
                            rule: rule.clone(),
                        }
                    ),
                    BotPlayer::new(
                        model::NegamaxModel {
                            depth: 5,
                            eval: eval::PatternEval { rule: rule.clone() },
                            prune: prune::NeighborPrune,
                            rule: rule.clone(),
                        }
                    ),
                    rule.clone(),
                )
            },
            ui_setting: UiSetting {
                board_size: 360.0,
                grid_stroke: egui::Stroke::new(
                    1.0, egui::Color32::GRAY
                ),
                stone_size: 10.0,
                stone_color_black: egui::Color32::BLACK,
                stone_color_white: egui::Color32::WHITE,
                stone_outline: egui::Stroke::new(
                    2.0, egui::Color32::DARK_GRAY
                ),
            },
            input_manager: Arc::clone(&input_manager),
        };

        result.games.pvp.trigger_start();
        result.games.bot.trigger_start();
        result.games.twobot.trigger_start();

        result
    }
}


// Player trait represents human or bot that can make placement.
// poll_move method is called every frame when on that player's turn.

enum PlayerAction {
    Move(Move),
    Thinking,
}

trait GamePlayer {
    fn turn_start(&mut self, board: &Board, last_mv: Move) { }

    fn poll_move(&mut self) -> PlayerAction;

    fn rejected(&mut self, board: &Board, reason: PutError);
}

struct HumanPlayer {
    input_manager: Arc<RwLock<InputManager>>,
}

struct BotPlayer {
    model: Arc<dyn Model + Send>,
    rx: Option<mpsc::Receiver<Option<Move>>>,
}

impl BotPlayer {
    fn new<M>(model: M) -> Self 
    where
        M: Model + Send + 'static
    {
        Self {
            model: Arc::new(model),
            rx: None,
        }
    }
}

impl GamePlayer for BotPlayer {
    fn turn_start(&mut self, board: &Board, last_mv: Move) {
        let (tx, rx) = mpsc::channel();

        self.rx = Some(rx);

        let board = board.clone();
        let model = Arc::clone(&self.model);

        
        let _ = thread::spawn(move || {
            let mv = model.next_move(&board, last_mv);

            // todo sdasdfsdfsdfsfd
            thread::sleep(Duration::from_millis(500));
            tx.send(mv).unwrap();
        });

    }

    fn poll_move(&mut self) -> PlayerAction {
        if let Ok(recv) = self.rx.as_ref().unwrap().try_recv() {
            PlayerAction::Move(recv.unwrap())
        } else {
            PlayerAction::Thinking
        }
    }

    fn rejected(&mut self, board: &Board, reason: PutError) {
        // todo!();
    }
}

impl GamePlayer for HumanPlayer {
    fn poll_move(&mut self) -> PlayerAction {
        
        let input_manager = self.input_manager.read().unwrap();
        
        let click = input_manager.get_click();
        if let Some(mv) = click {
            PlayerAction::Move(mv)
        } else {
            PlayerAction::Thinking
        }
    }

    fn rejected(&mut self, board: &Board, reason: PutError) {
        // todo!();
    }
}



// GameData struct represents the single game.

struct GameGroup {
    pvp: GameData,
    bot: GameData,
    twobot: GameData,
}

struct GameData {
    board: Board,
    black: Box<dyn GamePlayer>,
    white: Box<dyn GamePlayer>,
    status: GameStatus,
    rule: Box<dyn Rule>,
}

enum GameStatus {
    Ongo, Win(Player), Draw,
}

impl GameData {
    fn new<P1, P2, R>(black: P1, white: P2, rule: R) -> Self 
    where
        P1: GamePlayer + 'static,
        P2: GamePlayer + 'static,
        R: Rule + 'static,
    {
        Self {
            board: Board::blank(),
            black: Box::new(black),
            white: Box::new(white),
            status: GameStatus::Ongo,
            rule: Box::new(rule),
        }
    }

    fn trigger_start(&mut self) {
        // black starts first
        // (7, 7) is dummy data, TODO: fix it to Option<Move>
        self.black.turn_start(&self.board, Move { x: 7, y: 7 });
    }
}

struct UiSetting {
    board_size: f32,
    grid_stroke: egui::Stroke,
    stone_size: f32,
    stone_color_black: egui::Color32,
    stone_color_white: egui::Color32,
    stone_outline: egui::Stroke,
}


// handle input events
// currently this handles mouse event

struct InputManager {
    mouse: Option<Move>
}

impl Default for InputManager {
    fn default() -> Self {
        Self {
            mouse: None,
        }
    }
}

impl InputManager {
    fn update(&mut self, ui_setting: &UiSetting, resp: &egui::Response) {
        if resp.clicked() {
            let cell = ui_setting.board_size / 15.0;

            let pos = resp.interact_pointer_pos().unwrap();
            let local = (pos - resp.rect.min)
                .clamp(eframe::emath::Vec2::ZERO, resp.rect.size());
            let coord = (local / cell).floor();
            let mv = Move::new(coord.x as usize, coord.y as usize);

            self.mouse = mv;
            
        } else {
            self.mouse = None;
        }
    }

    fn get_click(&self) -> Option<Move> {
        
        self.mouse
    }
}

// main ui logic
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- top panel ---
        egui::TopBottomPanel::top("mode_switcher").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Menu: ");
                
                // Use radio buttons to switch the app mode state
                ui.radio_value(&mut self.mode, AppMode::Pvp, "PvP");
                ui.radio_value(&mut self.mode, AppMode::Bot, "Bot");
                ui.radio_value(&mut self.mode, AppMode::TwoBot, "Bot vs Bot");
                ui.radio_value(&mut self.mode, AppMode::About, "About");
            })
        });

        // --- Central Panel ---
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.mode == AppMode::About {
                ui.heading("Legend omok game amado");
                ui.label("made by mark-2008");

            } else {
                let mut game = match self.mode {
                    AppMode::Pvp => &mut self.games.pvp,
                    AppMode::Bot => &mut self.games.bot,
                    AppMode::TwoBot => &mut self.games.twobot,
                    _ => unreachable!(),
                };

                let (resp, painter) = game_template(
                    &self.ui_setting, &game, ui,
                );

                {
                    let mut input = self.input_manager.write().unwrap();
                    input.update(&self.ui_setting, &resp);
                }
                
                game_logic(game);

                // draw current mode
                let rect = resp.rect;
                draw_board(&self.ui_setting, &game, painter, &rect);
            }
        });
    }
}

/// template for game 
fn game_template(
    setting: &UiSetting,
    game: &GameData,
    ui: &mut egui::Ui,
) -> (egui::Response, egui::Painter) {
    ui.horizontal(|ui| {
        // game status message
        if matches!(game.status, GameStatus::Draw) {
            ui.label("Draw.");
        }
        if let GameStatus::Win(winner) = game.status {
            let winner_text = match winner {
                Turn::Black => "Black wins.",
                Turn::White => "White wins.",
            };
            ui.label(winner_text);
        }
        
        // turn, ply text
        let turn_text = match game.board.turn() {
            Turn::Black => "Black",
            Turn::White => "White",
        };

        let mut label_text = format!(
            "turn: {}, ply: {}",
            turn_text,
            game.board.ply(),
        );

        ui.label(label_text);
    });
    ui.separator();
    let (resp, painter) = ui.allocate_painter(
        egui::Vec2::splat(setting.board_size), 
        egui::Sense::click()
    );

    (resp, painter)
}

fn game_logic(game: &mut GameData) {
    if matches!(game.status, GameStatus::Win(_) | GameStatus::Draw) {
        return;
    }

    let turn = game.board.turn();
    let game_player = match turn {
        Turn::Black => &mut game.black,
        Turn::White => &mut game.white,
    };

    let action = game_player.poll_move();
    
    match action {
        PlayerAction::Move(mv) => {
            let result = game.rule.put(&mut game.board, mv, turn);
            match result {
                Ok(outcome) => {
                    game.status = match outcome {
                        PutOutcome::Continue => {
                            let next_player = match turn.next() {
                                Turn::Black => &mut game.black,
                                Turn::White => &mut game.white,
                            };
                            next_player.turn_start(&game.board, mv);
                            GameStatus::Ongo
                        },
                        PutOutcome::Win => GameStatus::Win(turn),
                        PutOutcome::Draw => GameStatus::Draw,
                    };
                },
                Err(error) => game_player.rejected(&game.board, error),
            }
        },
        PlayerAction::Thinking => (),
    }
}

fn draw_board(
    setting: &UiSetting,
    game: &GameData,
    painter: egui::Painter,
    rect: &egui::Rect,
 ) {
    let cell = setting.board_size / 15.0;

    // draw vertical lines
    for i in 0..15 {
        let x = rect.left() + cell * (i as f32 + 0.5);
        painter.line_segment(
            [
                egui::pos2(x, rect.top()),
                egui::pos2(x, rect.bottom()),
            ], 
            setting.grid_stroke,
        );
    }

    // draw horizontal lines
    for i in 0..15 {
        let y = rect.top() + cell * (i as f32 + 0.5);
        painter.line_segment(
            [
                egui::pos2(rect.left(), y),
                egui::pos2(rect.right(), y),
            ],
            setting.grid_stroke,
        );
    }

    // draw stones
    for i in 0..15 {
        for j in 0..15 {
            let stone = game.board.get(Move { x: i, y: j });
            if stone != Stone::None {
                let center = egui::Pos2::new(
                    rect.left() + cell * (i as f32 + 0.5),
                    rect.top() + cell * (j as f32 + 0.5),
                );
                let fill_color = match stone {
                    Stone::Black => setting.stone_color_black,
                    Stone::White => setting.stone_color_white,
                    Stone::None => unreachable!(),
                };

                painter.circle(
                    center, 
                    setting.stone_size, 
                    fill_color, 
                    setting.stone_outline,
                );
            }
        }
    }
}
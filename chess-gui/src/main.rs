#![allow(dead_code)]
/**
 * Chess GUI template.
 * Author: Eskil Queseth <eskilq@kth.se>, Viola Söderlund <violaso@kth.se>
 * Last updated: 2020-09-20
 */
use ggez::event;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, apply_transformations, Color, DrawMode, DrawParam, origin};
use ggez::{Context, GameResult};
use rustchessian::*;
use std::path;

/// A chess board is 8x8 tiles.
const GRID_SIZE: (i16, i16) = (8, 8);
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = ((45f32*1.5)as i16, (45f32*1.5)as i16);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: Color = Color::new(80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0);
const WHITE: Color = Color::new(125.0 / 255.0, 125.0 / 255.0, 125.0 / 255.0, 1.0);
const HIGHLIGHT_COLOR: Color = Color::new(255.0 / 255.0, 131.0 / 255.0, 21.0 / 255.0, 0.5);

/// GUI logic and event implementation structure.
struct AppState {
    sprites: Vec<((Team, Rank), graphics::Image)>,
    game: Game,
    possible_moves: Option<Vec<Action>>,
    // Save piece positions, which tiles has been clicked, current colour, etc...
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let sprites = AppState::load_sprites();
        let mut game = Game::new();
        game.player = Team::Black; //Bad workaround but fuck this library
        game.start_round();

        let state = AppState {
            sprites: sprites
                .iter()
                .map(|_sprite| {
                    (
                        _sprite.0,
                        graphics::Image::new(ctx, _sprite.1.clone()).unwrap(),
                    )
                })
                .collect::<Vec<((Team, Rank), graphics::Image)>>(),
            game: game,
            possible_moves: None,
        };

        Ok(state)
    }

    /// Loads chess piese images into vector.
    fn load_sprites() -> Vec<((Team, Rank), String)> {
        let mut sprites = Vec::new();
        sprites.push(((Team::Black, Rank::King), "/black_king.png".to_string()));
        sprites.push(((Team::Black, Rank::Queen), "/black_queen.png".to_string()));
        sprites.push(((Team::Black, Rank::Rook), "/black_rook.png".to_string()));
        sprites.push(((Team::Black, Rank::Pawn), "/black_pawn.png".to_string()));
        sprites.push(((Team::Black, Rank::Bishop), "/black_bishop.png".to_string()));
        sprites.push(((Team::Black, Rank::Knight), "/black_knight.png".to_string()));
        sprites.push(((Team::White, Rank::King), "/white_king.png".to_string()));
        sprites.push(((Team::White, Rank::Queen), "/white_queen.png".to_string()));
        sprites.push(((Team::White, Rank::Rook), "/white_rook.png".to_string()));
        sprites.push(((Team::White, Rank::Pawn), "/white_pawn.png".to_string()));
        sprites.push(((Team::White, Rank::Bishop), "/white_bishop.png".to_string()));
        sprites.push(((Team::White, Rank::Knight), "/white_knight.png".to_string()));
        sprites
    }
}

/// Implement each stage of the application event loop.
impl event::EventHandler for AppState {
    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        /*
        let param = DrawParam::default().offset(ggez::mint::Point2 { x: 1.0, y: 1.0 });
        let transform = param.to_matrix(); graphics::set_transform(ctx, transform);
        graphics::origin(ctx);
        apply_transformations(ctx)?;
        */

        // create text representation
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_gamestate()))
                .scale(graphics::Scale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions.0 as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions.1 as f32) / 2f32 as f32,
                text_dimensions.0 as f32 + 16.0,
                text_dimensions.1 as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // draw background
        graphics::draw(ctx, &background_box, DrawParam::default());

        // draw tiles
        for i in 0..64 {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    i % 8 * GRID_CELL_SIZE.0 as i32,
                    i / 8 * GRID_CELL_SIZE.1 as i32,
                    GRID_CELL_SIZE.0 as i32,
                    GRID_CELL_SIZE.1 as i32,
                ),
                match i % 2 {
                    0 => match i / 8 {
                        _row if _row % 2 == 0 => WHITE,
                        _ => BLACK,
                    },
                    _ => match i / 8 {
                        _row if _row % 2 == 0 => BLACK,
                        _ => WHITE,
                    },
                },
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }

        // draw text with dark gray colouring and center position
        if self.game.get_gamestate() != Gamestate::InProgress {
            graphics::draw(
                ctx,
                &state_text,
                DrawParam::default()
                    .color([0.0, 0.0, 0.0, 1.0].into())
                    .dest(ggez::mint::Point2 {
                        x: (SCREEN_SIZE.0 - text_dimensions.0 as f32) / 2f32 as f32,
                        y: (SCREEN_SIZE.0 - text_dimensions.1 as f32) / 2f32 as f32,
                    }),
            );
        }

        if self.possible_moves.is_some() {
            let available_moves = self.possible_moves.clone().unwrap();
            for action in available_moves.iter() {
                let x = action.to.coordinate.0 as i32;
                let y = action.to.coordinate.1 as i32;

                let highlight_rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        x * GRID_CELL_SIZE.0 as i32,
                        y * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    HIGHLIGHT_COLOR,
                )?;

                graphics::draw(
                    ctx,
                    &highlight_rectangle,
                    (ggez::mint::Point2 { x: 0.0, y: 0.0 },),
                );
            }
        }

        for x in 0..8 {
            for y in 0..8 {
                if let Some(Piece) = self.game.grid[x][y].piece {
                    let sprite = &self.sprites[0..12]
                        .iter()
                        .find(|&p| p.0 == (Piece.team, Piece.rank))
                        .unwrap()
                        .1;

                    graphics::draw(
                        ctx,
                        sprite,
                        DrawParam::default()
                            .scale(ggez::mint::Point2 {
                                x: GRID_CELL_SIZE.0 as f32 / sprite.width() as f32,
                                y: GRID_CELL_SIZE.1 as f32 / sprite.height() as f32,
                            })
                            .dest(ggez::mint::Point2 {
                                x: (GRID_CELL_SIZE.0 as f32) * (x as f32),
                                y: (GRID_CELL_SIZE.1 as f32) * (y as f32),
                            }),
                    )
                    .expect("Drawing tiles");
                };
            }
        }
        
        // render updated graphics
        graphics::present(ctx)?;

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            if self.possible_moves.is_some() {
                let x = (x as isize) / (GRID_CELL_SIZE.0 as isize);
                let y = (y as isize) / (GRID_CELL_SIZE.1 as isize);
                let coordinates = (x, y);

                let available_moves = self.possible_moves.clone().unwrap();
                for action in available_moves.iter() {
                    if action.to.coordinate == coordinates {
                        self.game.make_move(*action);
                        self.possible_moves = None;
                        self.game.start_round();
                        return;
                    }
                }

                self.possible_moves = None;
            } else {
                let x = (x as usize) / (GRID_CELL_SIZE.0 as usize);
                let y = (y as usize) / (GRID_CELL_SIZE.1 as usize);
                let coordinates = (x, y);

                let moves = match self.game.gen_moves_from_coordinate(coordinates) {
                    Ok(i) => i,
                    Err(i) => {
                        println!("{}", i);
                        self.possible_moves = None;
                        return;
                    }
                };

                self.possible_moves = Some(moves);
            }

            /* check click position and update board accordingly */
        };
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::R => {
                self.game = Game::new();
                self.possible_moves = None;
                self.game.start_round();
            }
            _ => (),
        };
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./chess_png");

    let context_builder = ggez::ContextBuilder::new("schack", "vem vet inte")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Schack") // Set window title "Schack"
                .icon("/icon.ico"), // Set application icon
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimenstions
                .resizable(false), // Fixate window size
        );
    let (context, event_loop) = &mut context_builder.build()?;
    let state = &mut AppState::new(context)?;
    event::run(context, event_loop, state) // Run window event loop
}

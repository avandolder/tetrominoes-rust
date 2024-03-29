use ggez::{
    graphics::{self, DrawParam, Text},
    input::keyboard::{self, KeyCode},
    mint::Point2,
    timer, Context, GameResult,
};
use ggez_goodies::scene::{Scene, SceneSwitch};

use crate::board::{Board, BOARD_HEIGHT, BOARD_WIDTH};
use crate::pausescene::PauseScene;
use crate::piece::{generate_pieces, make_ghost, Piece, PIECE_SIZE};
use crate::world::World;

const KEY_WAIT: f64 = 0.2;
const MOVE_WAIT: f64 = 1.;
const FAST_MOVE_WAIT: f64 = 0.05;

pub struct MainScene {
    board: Board,
    piece: Piece,
    piece_bag: Vec<Piece>,
    ghost: Piece,
    move_dt: f64,
    key_dt: f64,
}

impl MainScene {
    pub fn new(ctx: &mut Context) -> GameResult<MainScene> {
        let mut piece_bag = generate_pieces();
        let piece = piece_bag.pop().unwrap().prepare();
        let ghost = make_ghost(&piece);

        Ok(MainScene {
            board: Board::new(ctx, BOARD_WIDTH, BOARD_HEIGHT)?,
            piece,
            piece_bag,
            ghost,
            move_dt: 0.,
            key_dt: KEY_WAIT,
        })
    }

    fn rotate_piece(&mut self) {
        let mut new_piece = self.piece.clone();
        new_piece.rotate(1);
        for _ in 0..PIECE_SIZE {
            if !self.board.collides(&new_piece) {
                self.piece = new_piece;
                break;
            }
            new_piece = new_piece.shift(-1);
        }
    }

    fn drop_piece(&self, mut p: Piece) -> Piece {
        while !self.board.collides(&p) {
            p.row += 1;
        }
        p.row -= 1;
        p
    }

    fn update_ghost(&mut self) {
        self.ghost = self.drop_piece(make_ghost(&self.piece));
    }

    fn set_piece(&mut self, world: &mut World) {
        if self.piece.row < 0 {
            println!("Game over!");
            std::process::exit(0);
        }

        self.board.set_piece(&self.piece);
        world.score += self.board.clear_rows().pow(2);
        self.piece = self.piece_bag.pop().unwrap().prepare();
        if self.piece_bag.is_empty() {
            self.piece_bag = generate_pieces();
        }
    }
}

impl Scene<World, ()> for MainScene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> SceneSwitch<World, ()> {
        let dt = timer::duration_to_f64(timer::delta(ctx));
        self.move_dt += dt;
        if self.move_dt >= MOVE_WAIT
            || (keyboard::is_key_pressed(ctx, KeyCode::Down) && self.move_dt >= FAST_MOVE_WAIT)
        {
            self.piece.row += 1;

            if self.board.collides(&self.piece) {
                self.piece.row -= 1;
                self.set_piece(world);
            }

            self.move_dt = 0.;
        }

        self.key_dt += dt;
        if self.key_dt >= KEY_WAIT {
            if keyboard::is_key_pressed(ctx, KeyCode::Left)
                && !self.board.collides(&self.piece.clone().shift(-1))
            {
                self.piece.column -= 1;
                self.key_dt = 0.;
            } else if keyboard::is_key_pressed(ctx, KeyCode::Right)
                && !self.board.collides(&self.piece.clone().shift(1))
            {
                self.piece.column += 1;
                self.key_dt = 0.;
            }

            if keyboard::is_key_pressed(ctx, KeyCode::Up) {
                self.rotate_piece();
                self.key_dt = 0.;
            }

            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.piece = self.drop_piece(self.piece.clone());
                self.set_piece(world);
                self.key_dt = 0.;
            }

            if keyboard::is_key_pressed(ctx, KeyCode::M) {
                world.paused = true;
                return SceneSwitch::push(PauseScene::new(ctx).unwrap());
            }
        }

        self.update_ghost();

        SceneSwitch::None
    }

    fn draw(&mut self, world: &mut World, ctx: &mut Context) -> GameResult {
        let title = Text::new(("Tetrominoes", world.font, 12.));
        let next = Text::new(("Next Piece", world.font, 12.));
        let next_dp = DrawParam::new().dest(Point2 { x: 0., y: 100. });
        let score = Text::new((format!("Score: {}", world.score), world.font, 12.));
        let score_dp = DrawParam::new().dest(Point2 { x: 0., y: 50. });
        let next_piece_dp = DrawParam::new().dest(Point2 { x: 0., y: 120. });
        let board_dp = DrawParam::new().dest(Point2 { x: 100., y: 100. });

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &title, DrawParam::default())?;
        graphics::draw(ctx, &score, score_dp)?;
        graphics::draw(ctx, &next, next_dp)?;
        graphics::draw(ctx, self.piece_bag.last().unwrap(), next_piece_dp)?;
        graphics::draw(ctx, &self.board, board_dp)?;
        graphics::draw(ctx, &self.ghost, board_dp)?;
        graphics::draw(ctx, &self.piece, board_dp)?;

        if !world.paused {
            graphics::present(ctx)?;
        }

        Ok(())
    }

    fn input(&mut self, _world: &mut World, _event: (), _started: bool) {}

    fn name(&self) -> &str {
        "Main"
    }
}

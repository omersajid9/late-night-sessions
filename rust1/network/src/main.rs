use ggez::{self, graphics::{self, Color}, mint::IntoMint, Context, GameResult};
use ggez::event;
use nalgebra as na;
use ggez::mint::Point2;
use ggez::input::keyboard::{self, KeyCode};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: u8 = 100;

struct MainState {
    player_1_pos: Point2<f32>,
    player_2_pos: Point2<f32>,
    ball_pos: Point2<f32>
}

impl MainState {
    pub fn new(ctx: & Context) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        MainState {
            player_1_pos: Point2{x:RACKET_WIDTH_HALF, y:screen_h_half},
            player_2_pos: Point2{x:screen_w - RACKET_WIDTH_HALF, y:screen_h_half},
            ball_pos: Point2{x:screen_w_half, y:screen_h_half},
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let dt = ctx.time.delta().as_secs_f32();
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.player_1_pos.y -= PLAYER_SPEED * dt;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.player_1_pos.y += PLAYER_SPEED * dt;
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::CYAN);
        
        let rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;
        
        let ball = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball, Color::WHITE)?;
        
        let mut draw_params = graphics::DrawParam::default();
        draw_params = draw_params.dest(self.ball_pos.to_owned());
        canvas.draw(&ball_mesh, draw_params);

        draw_params = draw_params.dest(self.player_1_pos.to_owned());
        canvas.draw(&rect_mesh, draw_params);

        draw_params = draw_params.dest(self.player_2_pos.to_owned());
        canvas.draw(&rect_mesh, draw_params);

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut ggez::Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), ggez::GameError> {
        println!("Button {:?}, X {}, Y {}", _button, _x, _y);
        Ok(())
    }

    fn mouse_motion_event(
            &mut self,
            _ctx: &mut ggez::Context,
            _x: f32,
            _y: f32,
            _dx: f32,
            _dy: f32,
        ) -> Result<(), ggez::GameError> {
            // println!("X {}, Y {}, dX {}, dY {}", _x, _y, _dx, _dy);
            Ok(())
    }
}

fn main() -> GameResult {
    let game_name = "pong";
    let cb = ggez::ContextBuilder::new(game_name, "omersajid");
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title(game_name);

    let mut state = MainState::new(&ctx);
    event::run(ctx, event_loop, state);
    Ok(())
}
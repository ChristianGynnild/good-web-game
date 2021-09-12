/// Based on the bunnymark example from [`tetra`](https://crates.io/crates/tetra)
/// which is based on https://github.com/openfl/openfl-samples/tree/master/demos/BunnyMark
/// Original BunnyMark (and sprite) by Iain Lobb
extern crate good_web_game as ggez;
use quad_rand as qrand;

use std::env;
use std::path;

use ggez::graphics::{spritebatch::SpriteBatch, Color, Image};
use ggez::Context;
use ggez::*;

use glam::*;

// NOTE: Using a high number here yields worse performance than adding more bunnies over
// time - I think this is due to all of the RNG being run on the same tick...
const INITIAL_BUNNIES: usize = 1000;
const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;
const GRAVITY: f32 = 0.5;

struct Bunny {
    position: Vec2,
    velocity: Vec2,
}

impl Bunny {
    fn new() -> Bunny {
        let x_vel = qrand::gen_range(0.0, 5.0);
        let y_vel = qrand::gen_range(0.0, 5.0) - 2.5;

        Bunny {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(x_vel, y_vel),
        }
    }
}

struct GameState {
    texture: Image,
    bunnies: Vec<Bunny>,
    max_x: f32,
    max_y: f32,

    click_timer: i32,
    bunnybatch: SpriteBatch,
    batched_drawing: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> ggez::GameResult<GameState> {
        // We just use the same RNG seed every time.
        qrand::srand(12345);
        let texture = Image::new(ctx, "/wabbit_alpha.png")?;
        let mut bunnies = Vec::with_capacity(INITIAL_BUNNIES);
        let max_x = (WIDTH - texture.width()) as f32;
        let max_y = (HEIGHT - texture.height()) as f32;

        for _ in 0..INITIAL_BUNNIES {
            bunnies.push(Bunny::new());
        }

        let bunnybatch = SpriteBatch::new(texture.clone());

        Ok(GameState {
            texture,
            bunnies,
            max_x,
            max_y,

            click_timer: 0,
            bunnybatch,
            batched_drawing: true,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.click_timer > 0 {
            self.click_timer -= 1;
        }

        for bunny in &mut self.bunnies {
            bunny.position += bunny.velocity;
            bunny.velocity += Vec2::new(0.0, GRAVITY);

            if bunny.position.x > self.max_x {
                bunny.velocity *= Vec2::new(-1.0, 0.);
                bunny.position.x = self.max_x;
            } else if bunny.position.x < 0.0 {
                bunny.velocity *= Vec2::new(-1.0, 0.0);
                bunny.position.x = 0.0;
            }

            if bunny.position.y > self.max_y {
                bunny.velocity.y *= -0.8;
                bunny.position.y = self.max_y;

                // Flip a coin
                if qrand::gen_range(0, 2) > 0 {
                    bunny.velocity -= Vec2::new(0.0, 3.0 + (qrand::gen_range(0.0, 4.0)));
                }
            } else if bunny.position.y < 0.0 {
                bunny.velocity.y = 0.0;
                bunny.position.y = 0.0;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from((0.392, 0.584, 0.929)));

        if self.batched_drawing {
            self.bunnybatch.clear();
            for bunny in &self.bunnies {
                self.bunnybatch.add((bunny.position,));
            }
            graphics::draw(ctx, &self.bunnybatch, (Vec2::new(0.0, 0.0),))?;
        } else {
            for bunny in &self.bunnies {
                graphics::draw(ctx, &self.texture, (bunny.position,))?;
            }
        }

        /*
        graphics::set_window_title(
            ctx,
            &format!(
                "BunnyMark - {} bunnies - {:.0} FPS - batched drawing: {}",
                self.bunnies.len(),
                timer::fps(ctx),
                self.batched_drawing
            ),
        );
        */
        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if button == input::mouse::MouseButton::Left && self.click_timer == 0 {
            for _ in 0..INITIAL_BUNNIES {
                self.bunnies.push(Bunny::new());
            }
            self.click_timer = 10;
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == event::KeyCode::Space {
            self.batched_drawing = !self.batched_drawing;
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    ggez::start(
        ggez::conf::Conf::default()
            .cache(miniquad::conf::Cache::Tar(include_bytes!("resources.tar")))
            .physical_root_dir(Some(resource_dir)),
        |context| Box::new(GameState::new(context).unwrap()),
    )
}

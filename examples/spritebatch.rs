//! An example of how to use a `SpriteBatch`.
//!

extern crate cgmath;
extern crate good_web_game as ggez;

use ggez::event;
use ggez::graphics;

use ggez::timer;
use ggez::{Context, GameResult};

use cgmath::{Point2, Vector2};

struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "tile.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let s = MainState { spritebatch: batch };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let time = (timer::duration_to_f64(timer::time_since_start(ctx)) * 1000.0) as u32;
        let cycle = 10_000;
        for x in 0..150 {
            for y in 0..150 {
                let x = x as f32;
                let y = y as f32;
                let p = graphics::DrawParam::new()
                    .dest(Point2::new(x * 10.0, y * 10.0))
                    .scale(Vector2::new(
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                            .cos()
                            .abs()
                            * 0.0625,
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                            .cos()
                            .abs()
                            * 0.0625,
                    ))
                    .rotation(-2.0 * ((time % cycle) as f32 / cycle as f32 * 6.28));
                self.spritebatch.add(p);
            }
        }
        /*
        let param = graphics::DrawParam::new()
            .dest(Point2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).cos() * 50.0 - 150.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin() * 50.0 - 150.0,
            ))
            .scale(Vector2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
            ))
            .rotation((time % cycle) as f32 / cycle as f32 * 6.28)
            .offset(Point2::new(750.0, 750.0));
         */
        let param = graphics::DrawParam::new()
            .dest(Point2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).cos() * 50.0 + 250.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin() * 50.0 + 150.0,
            ))
            .scale(Vector2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
            ))
            // applying a src parameter to a spritebatch globally has no effect
            //.src([0.25,0.25,0.5,0.5].into())
            .rotation((time % cycle) as f32 / cycle as f32 * 6.28);
        // WARNING: Using an offset != (0.,0.) on a spritebatch may come with a significant performance cost.
        // This is due to the fact that the total dimensions of everything drawn by it have to be calculated.
        // See SpriteBatch::draw and SpriteBatch::dimensions for more information.
        //.offset(Point2::new(0.5, 0.5));
        graphics::draw(ctx, &self.spritebatch, param)?;
        self.spritebatch.clear();

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    ggez::start(
        ggez::conf::Conf::default()
            .cache(Some(include_bytes!("resources.tar"))),
        |mut context| Box::new(MainState::new(&mut context).unwrap()),
    )
}

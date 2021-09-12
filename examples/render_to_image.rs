//! An example of how to draw to `Image`'s using the `Canvas` type.

extern crate good_web_game as ggez;

use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, GameResult};

type Point2 = glam::Vec2;
type Vector2 = glam::Vec2;

struct MainState {
    canvas: graphics::Canvas,
    text: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        let font = graphics::Font::default();
        let text = graphics::Text::new(("Hello world!", font, 24.0));
        Ok(MainState { canvas, text })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // first lets render to our canvas
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::draw(ctx, &self.text, (Point2::new(400.0, 300.0), Color::WHITE))?;

        // now lets render our scene once in the top left and in the bottom
        // right
        let window_size = graphics::drawable_size(ctx);
        let scale = Vector2::new(
            0.5 * window_size.0 as f32 / self.canvas.width() as f32,
            0.5 * window_size.1 as f32 / self.canvas.height() as f32,
        );
        // let scale = Vector2::new(1.0, 1.0);
        graphics::set_canvas(ctx, None);
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        graphics::draw(
            ctx,
            &self.canvas,
            DrawParam::default()
                .dest(Point2::new(0.0, 0.0))
                .scale(scale),
        )?;
        graphics::draw(
            ctx,
            &self.canvas,
            DrawParam::default()
                .dest(Point2::new(400.0, 300.0))
                .scale(scale),
        )?;
        graphics::present(ctx)?;

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("drawable size: {:?}", graphics::drawable_size(ctx));
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

pub fn main() -> GameResult {
    ggez::start(
        ggez::conf::Conf::default()
            .cache(miniquad::conf::Cache::Tar(include_bytes!("resources.tar"))),
        |mut context| Box::new(MainState::new(&mut context).unwrap()),
    )
}

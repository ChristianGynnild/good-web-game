//! https://github.com/ggez/ggez/blob/master/examples/03_drawing.rs
//! A collection of semi-random shape and image drawing examples.

extern crate good_web_game as ggez;
extern crate nalgebra;

use ggez::event;
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::timer;
use ggez::{Context, GameResult};

type Point2 = nalgebra::Point2<f32>;

struct MainState {
    image1: graphics::Image,
    // image2_linear: graphics::Image,
    // image2_nearest: graphics::Image,
    meshes: Vec<graphics::Mesh>,
    zoomlevel: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::new(ctx, "dragon1.png")?;
        //let image2_linear = graphics::Image::new(ctx, "shot.png")?;
        // let mut image2_nearest = graphics::Image::new(ctx, "shot.png")?;
        // image2_nearest.set_filter(graphics::FilterMode::Nearest);

        let meshes = vec![build_mesh(ctx)?, build_textured_triangle(ctx)?];
        let s = MainState {
            image1,
            // image2_linear,
            // image2_nearest,
            meshes,
            zoomlevel: 1.0,
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            Point2::new(200.0, 200.0),
            Point2::new(400.0, 200.0),
            Point2::new(400.0, 400.0),
            Point2::new(200.0, 400.0),
            Point2::new(200.0, 300.0),
        ],
        4.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    )?;

    mb.ellipse(
        DrawMode::fill(),
        Point2::new(600.0, 200.0),
        50.0,
        120.0,
        1.0,
        Color::new(1.0, 1.0, 0.0, 1.0),
    );

    mb.circle(
        DrawMode::fill(),
        Point2::new(600.0, 380.0),
        40.0,
        1.0,
        Color::new(1.0, 0.0, 1.0, 1.0),
    );

    mb.build(ctx)
}

fn build_textured_triangle(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();
    let triangle_verts = vec![
        graphics::Vertex {
            pos: [100.0, 100.0],
            uv: [1.0, 1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        graphics::Vertex {
            pos: [0.0, 100.0],
            uv: [0.0, 1.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        graphics::Vertex {
            pos: [0.0, 0.0],
            uv: [0.0, 0.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];

    let triangle_indices = vec![0, 1, 2];

    let i = graphics::Image::new(ctx, "rock.png")?;
    mb.from_raw(&triangle_verts, &triangle_indices, Some(i));
    mb.build(ctx)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.zoomlevel += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        // let src = graphics::Rect::new(0.25, 0.25, 0.5, 0.5);
        // let src = graphics::Rect::one();
        let dst = cgmath::Point2::new(20.0, 20.0);
        graphics::draw(ctx, &self.image1, (dst,))?;
        /*
        let dst = cgmath::Point2::new(200.0, 100.0);
        let dst2 = cgmath::Point2::new(400.0, 400.0);
        let scale = cgmath::Vector2::new(10.0, 10.0);
        // let shear = graphics::Point::new(self.zoomlevel, self.zoomlevel);
        // graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0));
        graphics::draw(
            ctx,
            &self.image2_linear,
            graphics::DrawParam::new()
                // src: src,
                .dest(dst)
                .rotation(self.zoomlevel)
                // offset: Point2::new(-16.0, 0.0),
                .scale(scale)
                // shear: shear,
        )?;
        graphics::draw(
            ctx,
            &self.image2_nearest,
            graphics::DrawParam::new()
                // src: src,
                .dest(dst2)
                .rotation(self.zoomlevel)
                .offset(Point2::new(0.5, 0.5))
                .scale(scale)
                // shear: shear,
        )?;
        */

        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        let r1 =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        let r2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            rect,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &r2, DrawParam::default())?;
        // graphics::rectangle(ctx, graphics::WHITE, graphics::DrawMode::fill(), rect)?;

        // let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        // graphics::rectangle(
        //     ctx,
        //     graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        //     graphics::DrawMode::stroke(1.0),
        //     rect,
        // )?;

        for m in &self.meshes {
            graphics::draw(ctx, m, DrawParam::new())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    ggez::start(
        ggez::conf::Conf {
            cache: ggez::conf::Cache::Tar(include_bytes!("resources.tar").to_vec()),
            loading: ggez::conf::Loading::Embedded,
            ..Default::default()
        },
        |mut context| Box::new(MainState::new(&mut context).unwrap()),
    )
}

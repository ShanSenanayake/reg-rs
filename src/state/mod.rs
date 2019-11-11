mod drawable_image;
use drawable_image::DrawableImage;

use ggez::{
    conf,
    event::{self, EventHandler},
    graphics::{self, DrawParam, Image},
    Context, ContextBuilder, GameResult,
};
use nalgebra as na;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

struct MovingBackground {
    position: na::Vector2<f32>,
    background: (DrawableImage, DrawableImage),
}

impl MovingBackground {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = DrawableImage::new(
            ctx,
            &format!("/background-overlay_{}_{}.png", SCREEN_WIDTH, SCREEN_HEIGHT),
        )?;
        let image_tuple = (image.clone(), image);

        let moving_background = MovingBackground {
            position: na::Vector2::new(0., 0.),
            background: image_tuple,
        };
        Ok(moving_background)
    }
}

pub struct State {
    static_background: DrawableImage,
    moving_background: MovingBackground,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        let state = State {
            static_background: DrawableImage::new(
                ctx,
                &format!("/background_{}_{}.png", SCREEN_WIDTH, SCREEN_HEIGHT),
            )?,
            moving_background: MovingBackground::new(ctx)?,
        };
        Ok(state)
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...
        graphics::draw(
            ctx,
            &self.static_background.image,
            self.static_background.draw_param,
        )?;
        graphics::draw(
            ctx,
            &self.moving_background.background.0.image,
            self.moving_background.background.0.draw_param,
        )?;

        graphics::present(ctx)
    }
}

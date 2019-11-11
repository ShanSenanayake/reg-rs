pub mod state;

use ggez::{
    conf,
    event::{self, EventHandler},
    graphics::{self, DrawParam, Image},
    Context, ContextBuilder, GameResult,
};

use crate::{
    SCREEN_HEIGHT,
    SCREEN_WIDTH
};

pub struct DrawableImage {
    image: Image,
    draw_param: DrawParam,
}

pub struct State {
    static_background: DrawableImage,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        let state = State {
            static_background: DrawableImage {
                image: Image::new(
                    ctx,
                    format!("/background_{}_{}.png", SCREEN_WIDTH, SCREEN_HEIGHT),
                )?,
                draw_param: DrawParam::default(),
            },
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

        graphics::present(ctx)
    }
}

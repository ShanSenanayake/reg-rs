use ggez::{
  graphics::{self, DrawParam, Image},
  Context, ContextBuilder, GameResult,
};

#[derive(Clone)]
pub struct DrawableImage {
  pub image: Image,
  pub draw_param: DrawParam,
}

impl DrawableImage {
  pub fn new(ctx: &mut Context, path: &str) -> GameResult<Self> {
    let drawable_image = DrawableImage {
      image: Image::new(ctx, path)?,
      draw_param: DrawParam::default(),
    };
    Ok(drawable_image)
  }
}

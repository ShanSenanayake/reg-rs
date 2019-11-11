pub mod state;

use ggez::{
    conf,
    event::{self, EventHandler},
    graphics::{self, DrawParam, Image},
    Context, ContextBuilder, GameResult,
};

use std::path::PathBuf;

use state::State;

pub const SCREEN_WIDTH: f32 = 600f32;
pub const SCREEN_HEIGHT: f32 = 800f32;

fn main() -> GameResult {
    // Make a Context and an EventLoop.
    let (ref mut ctx, ref mut event_loop) =
        ContextBuilder::new("spacing all them invaders", "a author")
            .window_setup(conf::WindowSetup::default().title("Spac Invader"))
            .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
            .add_resource_path(PathBuf::from("./assets"))
            .build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut state = State::new(ctx)?;

    // Run!
    match event::run(ctx, event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
    Ok(())
}



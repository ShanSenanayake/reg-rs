use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};

use std::path;

const SCREEN_WIDTH: f32 = 800f32;
const SCREEN_HEIGHT: f32 = 600f32;

fn main() -> GameResult {
    // Make a Context and an EventLoop.
    let (ref mut ctx, ref mut event_loop) =
        ContextBuilder::new("spacing all them invaders", "a author")
            .window_setup(conf::WindowSetup::default().title("Spac Invader"))
            .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
            .add_resource_path(path::PathBuf::from("./assets"))
            .build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(ctx);

    // Run!
    match event::run(ctx, event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
    Ok(())
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...

        graphics::present(ctx)
    }
}

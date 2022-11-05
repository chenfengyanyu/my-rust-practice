extern crate skeptic;
#[test] fn readme_sect_what_is_this_line_117() {
    let s = &format!(r####"
{}"####, r####"use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}
"####);
    skeptic::rt::compile_test(r#"/Users/jartto/.cargo/registry/src/rsproxy.cn-8f6827c7555bfaf8/ggez-0.7.0"#, r#"/Users/jartto/Documents/Project/my-rust-practice/my_box_game/target/debug/build/ggez-c2a2d8e3fd38edaf/out"#, r#"aarch64-apple-darwin"#, s);
}


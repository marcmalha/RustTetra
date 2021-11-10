use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.78, 0.58, 0.92));
        Ok(())
    }
}



fn main() -> tetra::Result {
    // println!("Hello, world!");
    ContextBuilder::new("PoC_WS", 800, 500)
        .build()?
        .run(|_| Ok(GameState))
}

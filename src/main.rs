use tetra::graphics::{self, Color};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

// 2D vector used for textBox position configuration
const TEXT_OFFSET: Vec2<f32> = Vec2::new(50.0, 50.0);

// In step2, GameState struct now contains variable vector_text of type Text
struct GameState = {
    vector_text: Text,
}

impl GameState {
    // overriding the ContextBuilder new function (Rust constructor)
    fn new(&mut self, ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState{
            // building the vector_text with the "Text" constructor
            vector_text: Text::new(
                "Hello, World!\n",
                // loading Font from assets folder
                Font::vector(ctx, "../assets/font/DejaVuSansMono.ttf", 50.0)?,
            ),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.78, 0.58, 0.92));

        // drawing vector_text using Text::draw
        self.vector_text.draw(ctx, TEXT_OFFSET);
        Ok(())
    }
}



fn main() -> tetra::Result {
    // println!("Hello, world!");
    ContextBuilder::new("Hello, World!", 1280, 800)
        .quit_on_escape(true)
        .build()?
        // running our newly implemented GameState::new function
        .run(GameState::new))
}

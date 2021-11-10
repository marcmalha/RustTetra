use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};


const IMG_POS: Vec2<f32> = Vec2::new(120.0, 120.0);
const IMG_SCALE: Vec2<f32> = Vec2::new(0.5, 0.5);
const IMG_ORIGIN: Vec2<f32> = Vec2::new(50.0, 50.0);
const TEXT_OFFSET: Vec2<f32> = Vec2::new(50.0, 50.0);

struct GameState {
    img: Texture,
    vector_text: Text,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState{
            img: Texture::new(ctx, "./assets/images/PoC.png")?,
            vector_text: Text::new("Hello PoC",
                                Font::vector(ctx, "./assets/font/DejaVuSansMono.ttf", 45.0)?,
            ),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.78, 0.58, 0.92));

        self.img.draw(ctx, DrawParams::new()
                    .position(IMG_POS)
                    .scale(IMG_SCALE)
                    .origin(IMG_ORIGIN),
        );
        self.vector_text.draw(ctx, TEXT_OFFSET);
        Ok(())
    }
}



fn main() -> tetra::Result {
    ContextBuilder::new("Step3", 1280, 800)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

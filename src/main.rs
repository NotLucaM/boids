use ggez::{GameResult, event, Context, graphics};
use ggez::graphics::Color;
use ggez::mint::Vector2;
use glam::*;

struct StartState {
    pos_x: f32,
}

impl StartState {
    fn new() -> GameResult<StartState> {
        Ok( StartState {pos_x: 0.0} )
    }
}

impl event::EventHandler for StartState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (Vec2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build().unwrap();
    let state = StartState::new().unwrap();
    event::run(ctx, event_loop, state)
}

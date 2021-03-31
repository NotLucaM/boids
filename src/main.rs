mod boid;

use ggez::{GameResult, event, Context, graphics};
use ggez::graphics::Color;
use ggez::mint::Vector2;
use glam::*;
use legion::{World, Schedule, Resources, Read, IntoQuery};

use crate::boid::{update_positions_system, Position, Boid, Velocity};
use std::time::{Instant, Duration};

#[derive(Copy, Clone)]
pub struct Time {
    start: Instant,
}

impl Time {
    pub fn elapsed_seconds(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    pub fn reset(&mut self) {
        self.start = Instant::now()
    }
}

impl Default for Time {
    fn default() -> Self {
        Time { start: Instant::now() }
    }
}

struct StartState {
    world: World,
    resources: Resources,
    update_schedule: Schedule,
}

impl StartState {
    fn new() -> GameResult<StartState> {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(Time::default());
        let mut update_schedule = Schedule::builder()
            .add_system(update_positions_system())
            .build();

        world.push((Position {x: 0.0, y: 0.0}, Velocity { dx: 1.0, dy: 0.1 }, Boid ));
        Ok( StartState { world, resources, update_schedule } )
    }
}

impl event::EventHandler for StartState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update_schedule.execute(&mut self.world, &mut self.resources);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let mut draw_boid = |_boid: &Boid, position: &Position| {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                25.0,
                2.0,
                Color::WHITE,
            ).unwrap();
            graphics::draw(ctx, &circle, (Vec2::new(position.x, position.y),)).unwrap();
        };

        let mut query = Read::<(&Position, &Boid)>::query();

        for (pos, boid) in query.iter(&self.world) {
            draw_boid(boid, pos);
        }

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

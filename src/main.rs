mod boid;

use ggez::{GameResult, event, Context, graphics};
use ggez::graphics::Color;
use ggez::mint::Vector2;
use glam::*;
use legion::{World, Schedule, Resources, Read, IntoQuery, Entity};

use crate::boid::{update_positions_system, update_velocities_system, Position, Boid, Velocity, update_velocities, Acceleration};
use std::time::{Instant, Duration};
use rand::rngs::ThreadRng;
use rand::Rng;

pub const WINDOW_WIDTH: usize = 1000;
pub const WINDOW_HEIGHT: usize = 1000;
pub const BOIDS: usize = 400;

#[derive(Copy, Clone)]
pub struct Time {
    start: Instant,
}

impl Time {
    pub fn elapsed_seconds(&self) -> f32 {
        self.start.elapsed().as_secs() as f32 / 60.0
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
    fn new(ctx: &mut Context) -> GameResult<StartState> {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(Time::default());
        let mut update_schedule = Schedule::builder()
            .add_system(update_velocities_system())
            .add_system(update_positions_system())
            .build();

        let mut rand = ThreadRng::default();
        for i in 0..BOIDS {
            let pos = Position { x: rand.gen_range(0.0..400.0), y: rand.gen_range(0.0..400.0) };
            let vel = Velocity { dx: 0.0, dy: 0.0 };
            let acc = Acceleration { dx: 1.0, dy: 1.0 };
            world.push((pos, vel, acc, Boid::new(ctx) ));
        }
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
        let mut draw_boid = |boid: &Boid, position: &Position| {
            graphics::draw(ctx, &boid.mesh, (Vec2::new(position.x, position.y),)).unwrap();
        };

        let mut query = <(&Position, &Boid)>::query();

        for (pos, boid) in query.iter(&self.world) {
            draw_boid(boid, pos);
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("boids", "luca");
    let (mut ctx, event_loop) = cb.build().unwrap();
    let state = StartState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state)
}

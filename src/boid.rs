use legion::{system, Query};
use crate::Time;
use ggez::graphics::{Mesh, Color};
use ggez::{Context, graphics};
use glam::Vec2;
use legion::world::SubWorld;

#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    pub mesh: Mesh,
}

impl Boid {
    pub fn new(ctx: &mut Context) -> Boid {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            10.0,
            2.0,
            Color::WHITE,
        ).unwrap();
        Boid { mesh: circle }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub(crate) dx: f32,
    pub(crate) dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acceleration {
    pub(crate) dx: f32,
    pub(crate) dy: f32,
}

#[system(for_each)]
pub fn update_velocities(vel: &mut Velocity, acc: &Acceleration, #[resource] time: &Time) {
    vel.dx += acc.dx * time.elapsed_seconds() as f32;
    vel.dy += acc.dy * time.elapsed_seconds() as f32;
}


#[system(for_each)]
pub fn update_positions(pos: &mut Position, vel: &Velocity, #[resource] time: &Time) {
    pos.x += vel.dx * time.elapsed_seconds() as f32;
    pos.y += vel.dy * time.elapsed_seconds() as f32;
}

#[system]
pub fn update_boids(world: &mut SubWorld, query: &mut Query<(&mut Velocity, &mut Acceleration, &Position, &Boid)>,) {
    for (_vel, acc, _pos, _boid) in &query.iter(world) {

    }
}
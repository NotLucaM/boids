use legion::system;
use crate::Time;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boid;

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

#[system(for_each)]
pub fn update_positions(pos: &mut Position, vel: &Velocity, #[resource] time: &Time) {
    pos.x += vel.dx * time.elapsed_seconds() as f32;
    pos.y += vel.dy * time.elapsed_seconds() as f32;
}
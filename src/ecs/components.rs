use bevy::prelude::*;

#[derive(Component)]
pub struct Boid {
    pub perception_radius: f32,
}

#[derive(Component)]
pub struct Kinematic2D {
    pub move_speed: f32,
    pub acceleration: Vec2,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct DesiredAcceleration2D(pub Vec2);

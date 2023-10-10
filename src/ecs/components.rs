use bevy::prelude::*;

#[derive(Component)]
pub struct Boid {
    pub perception_radius: f32,
    pub seperate_distance: f32,
}

#[derive(Component)]
pub struct Kinematic2D {
    pub max_speed: f32,
    pub acceleration: Vec2,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct DesiredVelocity2D(pub Vec2);

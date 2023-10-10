use bevy::prelude::*;

#[derive(Resource)]
pub struct BoidCount(pub i32);

#[derive(Resource)]
pub struct BoidScale(pub f32);

#[derive(Resource)]
pub struct BoidMaxSpeed(pub f32);
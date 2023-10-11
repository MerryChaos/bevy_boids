use bevy::prelude::*;

#[derive(Resource)]
pub struct BoidCount(pub u32);

#[derive(Resource)]
pub struct BoidScale(pub f32);

#[derive(Resource)]
pub struct BoidMaxSpeed(pub f32);

#[derive(Resource)]
pub struct BoidPerceptionRadius(pub f32);
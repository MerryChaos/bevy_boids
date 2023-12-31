use bevy::prelude::*;
mod ecs;
mod utils;
use ecs::resources::{BoidCount, BoidMaxSpeed, BoidScale, BoidPerceptionRadius};
use ecs::systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BoidCount(3000))
        .insert_resource(BoidScale(20.))
        .insert_resource(BoidMaxSpeed(100.))
        .insert_resource(BoidPerceptionRadius(80.))
        .add_systems(Startup, systems::setup_camera)
        .add_systems(Startup, systems::spawn_boids)
        .add_systems(Update, systems::calculate_boid_velocity)
        .add_systems(Update, systems::move_boids)
        .add_systems(Update, systems::wrap_boids)
        .add_systems(Update, systems::resize_camera)
        .run();
}

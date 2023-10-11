use bevy::prelude::{Transform, Vec2};

use crate::ecs::components::{Boid, Kinematic2D};

use super::vec3_to_vec2;

const ALIGNMENT_FORCE: f32 = 5.6;
const COHESION_FORCE: f32 = 0.6;
const SEPERATION_FORCE: f32 = 0.6;
const STEER_FORCE: f32 = 5.0;

pub fn alignment(
    boid: (&Transform, &Kinematic2D),
    neighbours: &Vec<(&Transform, &Kinematic2D)>,
) -> Vec2 {
    if neighbours.is_empty() {
        return Vec2::ZERO;
    }

    let mut vector = Vec2::ZERO;
    for (_, kinematic) in neighbours.iter() {
        vector += kinematic.velocity;
    }
    vector /= neighbours.len() as f32;
    vector = vector.normalize_or_zero() * boid.1.move_speed;

    steer(vector, boid.1.velocity) * ALIGNMENT_FORCE
}

pub fn cohesion(
    boid: (&Transform, &Kinematic2D),
    neighbours: &Vec<(&Transform, &Kinematic2D)>,
) -> Vec2 {
    if neighbours.is_empty() {
        return Vec2::ZERO;
    }

    let mut vector = Vec2::ZERO;
    for (transform, _) in neighbours.iter() {
        vector += vec3_to_vec2(transform.translation);
    }
    vector /= neighbours.len() as f32;
    vector = (vector - vec3_to_vec2(boid.0.translation)).normalize_or_zero() * boid.1.move_speed;
    
    steer(vector, boid.1.velocity) * COHESION_FORCE
}

pub fn separation(
    boid: (&Transform, &Kinematic2D, &Boid),
    neighbours: &Vec<(&Transform, &Kinematic2D)>,
) -> Vec2 {
    if neighbours.is_empty() {
        return Vec2::ZERO;
    }
    let pos = vec3_to_vec2(boid.0.translation);

    let mut vector = Vec2::ZERO;
    let mut total = 0;
    for (transform, _) in neighbours.iter() {
        let neighbour_pos = vec3_to_vec2(transform.translation);
        let dist = pos.distance(neighbour_pos);
        if dist < boid.2.perception_radius / 2. {
            let diff = pos - neighbour_pos;
            vector += diff / dist;
            total += 1;
        }
    }
        
    vector /= total as f32;
    vector = vector.normalize_or_zero() * boid.1.move_speed;
    
    steer(vector, boid.1.velocity) * SEPERATION_FORCE
}

fn steer(target: Vec2, velocity: Vec2) -> Vec2 {
    (target - velocity).normalize_or_zero() * STEER_FORCE
}
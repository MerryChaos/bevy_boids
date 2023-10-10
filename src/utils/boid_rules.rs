use bevy::prelude::{Transform, Vec2};

use crate::ecs::components::{Boid, Kinematic2D};

use super::vec3_to_vec2;

const ALIGNMENT_COEF: f32 = 0.5;
const COHERE_COEF: f32 = 0.7;
const SEPERATE_COEF: f32 = 1.0;

pub fn alignment(
    _boid: (&Transform, &Kinematic2D),
    other_boids: &Vec<(&Transform, &Kinematic2D)>,
) -> Vec2 {
    let mut velocity = Vec2::ZERO;

    if !other_boids.is_empty() {
        for (_, kinematic) in other_boids.iter() {
            velocity += kinematic.velocity
        }
        velocity /= other_boids.len() as f32;
    }

    velocity * ALIGNMENT_COEF
}

pub fn cohere(
    boid: (&Transform, &Kinematic2D),
    other_boids: &Vec<(&Transform, &Kinematic2D)>,
) -> Vec2 {
    let mut velocity = Vec2::ZERO;

    if !other_boids.is_empty() {
        let mut center_of_mass = Vec2::ZERO;
        for (transform, _) in other_boids.iter() {
            center_of_mass += vec3_to_vec2(transform.translation)
        }
        center_of_mass /= other_boids.len() as f32;
        velocity = center_of_mass - vec3_to_vec2(boid.0.translation)
    }

    velocity * COHERE_COEF
}

pub fn seperate(
    boid: (&Transform, &Kinematic2D, &Boid),
    other_boids: &Vec<(&Transform, &Kinematic2D)>,
) -> Vec2 {
    let mut velocity = Vec2::ZERO;

    let translation = vec3_to_vec2(boid.0.translation);
    if !other_boids.is_empty() {
        for (transform, _) in other_boids.iter() {
            if boid.0.translation.distance(transform.translation) <= boid.2.seperate_distance {
                velocity -= vec3_to_vec2(transform.translation) - translation;
            }
        }
        velocity /= other_boids.len() as f32;
    }

    velocity * SEPERATE_COEF
}

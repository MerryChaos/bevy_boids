use bevy::prelude::*;

pub mod flocking_rules;
pub mod triangle_mesh;

pub fn vec3_to_vec2(vector: Vec3) -> Vec2 {
    Vec2::new(vector.x, vector.y)
}

pub fn vec2_to_vec3(vector: Vec2, z: Option<f32>) -> Vec3 {
    Vec3::new(vector.x, vector.y, z.unwrap_or(0.0))
}

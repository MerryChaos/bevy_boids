use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::window::{PrimaryWindow, WindowResized};
use rand::prelude::*;

use crate::utils::boid_rules::{cohere, alignment, seperate};
use crate::utils::triangle_mesh;
use crate::utils::vec2_to_vec3;

use super::resources::{BoidCount, BoidScale, BoidMaxSpeed};
use super::components::{Boid, Kinematic2D, DesiredVelocity2D};


pub fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut clear_color: ResMut<ClearColor>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
    *clear_color = ClearColor(Color::rgb(0.0, 0.0, 0.1));
}

pub fn resize_camera(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut resized_events: EventReader<WindowResized>,
) {
    let window = window_query.get_single().unwrap();

    // Check if there were any WindowResized events
    for _event in resized_events.iter() {
        for (_camera, mut transform) in camera_query.iter_mut() {
            // Reset the position of the camera
            transform.translation = Vec3::new(window.width() / 2., window.height() / 2., 0.);
        }
    }
}

pub fn spawn_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    boid_count: Res<BoidCount>,
    boid_scale: Res<BoidScale>,
    boid_max_speed: Res<BoidMaxSpeed>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();
    let n = boid_count.0;

    for i in 0..n {
        let color = Color::rgb(0., 1., rng.gen());
    
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(triangle_mesh::create_2d()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: Vec3 {
                        x: rng.gen::<f32>() * window.width(),
                        y: rng.gen::<f32>() * window.height(),
                        z: i as f32
                    },
                    rotation: Quat::from_xyzw(0., 0., rng.gen::<f32>(), 1.),
                    scale: Vec3::ONE * boid_scale.0
                },
                // transform: Transform::from_xyz(0., 0., x as f32),
                ..default()
            },
            Kinematic2D {
                max_speed: boid_max_speed.0,
                acceleration: Vec2::ZERO,
                velocity: Vec2::new(
                    rng.gen::<f32>() * 2. - 1.,
                    rng.gen::<f32>() * 2. - 1.,
                ).normalize() * boid_max_speed.0,
            },
            Boid {
                perception_radius: boid_scale.0 + 100.,
                seperate_distance: boid_scale.0 + 100.,
            }
        ));
    }
}

pub fn calculate_boid_velocity(
    par_commands: ParallelCommands,
    query: Query<(Entity, &Transform, &Kinematic2D, &Boid)>,
    kinematic_query: Query<(Entity, &Transform, &Kinematic2D), With<Boid>>,
) {
    query.par_iter().for_each(|(entity, transform, kinematic, boid)| {
        // Step #1: Get percepted boids
        let mut percepted_boids: Vec<(&Transform, &Kinematic2D)> = vec![];
        
        for (other_entity, other_transform, other_kinematic) in kinematic_query.iter() {

            if entity.index() == other_entity.index() {
                continue;
            }

            let dist = transform.translation.distance(other_transform.translation);
            if dist <= boid.perception_radius {
                percepted_boids.push((other_transform, other_kinematic));
            }
        }

        // Step #2: Calculate velocity
        let v_align = alignment((&transform, &kinematic), &percepted_boids);
        let v_cohere = cohere((&transform, &kinematic), &percepted_boids);
        let v_seperate = seperate((&transform, &kinematic, &boid), &percepted_boids);

        let desired_velocity = v_align + v_cohere + v_seperate;

        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert(DesiredVelocity2D(desired_velocity));
        });
    });
}

pub fn move_boids(
    mut query: Query<(&mut Transform, &mut Kinematic2D, &DesiredVelocity2D), With<Boid>>,
    time: Res<Time>,
) {
    query.par_iter_mut().for_each_mut(|(mut transform, mut kinematic, desired_velocity)| {
        kinematic.acceleration = kinematic.acceleration + kinematic.acceleration.lerp(desired_velocity.0, 0.2);
        kinematic.velocity = kinematic.velocity + kinematic.acceleration;
        if kinematic.velocity.length() > kinematic.max_speed {
            kinematic.acceleration = Vec2::ZERO;
            kinematic.velocity = kinematic.velocity.normalize() * kinematic.max_speed;
        }

        transform.translation += vec2_to_vec3(kinematic.velocity, None) * time.delta_seconds();
        
        let angle = kinematic.velocity.y.atan2(kinematic.velocity.x);
        transform.rotation = Quat::from_rotation_z(angle);
    });
}

pub fn wrap_boids(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Boid>>,
) {
    let window = window_query.get_single().unwrap();

    query.par_iter_mut().for_each_mut(|mut transform| {
        let x_min = 0. - transform.scale.x / 2.;
        let x_max = window.width() + transform.scale.x / 2.;
        let y_min = 0. - transform.scale.y / 2.;
        let y_max = window.height() + transform.scale.y / 2.;

        if transform.translation.x < x_min {
            transform.translation.x += window.width() + transform.scale.x
        } else if transform.translation.x > x_max {
            transform.translation.x -= window.width() + transform.scale.x
        }
        if transform.translation.y < y_min {
            transform.translation.y += window.height() + transform.scale.y
        } else if transform.translation.y > y_max {
            transform.translation.y -= window.height() + transform.scale.y
        }
    });
}
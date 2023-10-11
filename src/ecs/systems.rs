use bevy::window::{PrimaryWindow, WindowResized};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::utils::flocking_rules::{alignment, cohesion, separation};
use crate::utils::triangle_mesh;
use crate::utils::vec2_to_vec3;

use super::components::{Boid, DesiredAcceleration2D, Kinematic2D};
use super::resources::{BoidCount, BoidMaxSpeed, BoidScale, BoidPerceptionRadius};

pub fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut clear_color: ResMut<ClearColor>,
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
    boid_perception_radius: Res<BoidPerceptionRadius>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();
    
    for i in 0..boid_count.0 {
        let mut color = Color::rgb(0., 1., rng.gen());
        if i == boid_count.0 - 1 {
            color = Color::hex("FFBB00").unwrap();
        }

        let angle_deg: f32 = rng.gen_range(0.0..360.0);
        let angle_rad: f32 = angle_deg.to_radians();
        let velocity = Vec2::new(
            boid_max_speed.0 * angle_rad.cos(),
            boid_max_speed.0 * angle_rad.sin(),
        );

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(triangle_mesh::create_2d()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: Vec3 {
                        x: rng.gen::<f32>() * window.width(),
                        y: rng.gen::<f32>() * window.height(),
                        z: i as f32,
                    },
                    rotation: Quat::default(),
                    scale: Vec3::ONE * boid_scale.0,
                },
                ..default()
            },
            Kinematic2D {
                move_speed: boid_max_speed.0,
                acceleration: Vec2::ZERO,
                velocity,
            },
            Boid {
                perception_radius: boid_perception_radius.0,
            },
        ));
    }
}

pub fn calculate_boid_velocity(
    par_commands: ParallelCommands,
    query: Query<(Entity, &Transform, &Kinematic2D, &Boid)>,
    kinematic_query: Query<(Entity, &Transform, &Kinematic2D), With<Boid>>,
) {
    query
        .par_iter()
        .for_each(|(entity, transform, kinematic, boid)| {
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
            let mut desired_acceleration = Vec2::ZERO;
            desired_acceleration += alignment((&transform, &kinematic), &percepted_boids);
            desired_acceleration += cohesion((&transform, &kinematic), &percepted_boids);
            desired_acceleration += separation((&transform, &kinematic, &boid), &percepted_boids);

            par_commands.command_scope(|mut commands| {
                commands
                    .entity(entity)
                    .insert(DesiredAcceleration2D(desired_acceleration));
            });
        });
}

pub fn move_boids(
    mut query: Query<(&mut Transform, &mut Kinematic2D, &DesiredAcceleration2D), With<Boid>>,
    time: Res<Time>,
) {
    query
        .par_iter_mut()
        .for_each_mut(|(mut transform, mut kinematic, desired_acceleration)| {
            kinematic.acceleration += desired_acceleration.0;
            kinematic.velocity = kinematic.velocity + kinematic.acceleration * time.delta_seconds();
            if kinematic.velocity.length_squared() > kinematic.move_speed * kinematic.move_speed {
                kinematic.velocity = kinematic.velocity.normalize_or_zero() * kinematic.move_speed;
            }

            transform.translation += vec2_to_vec3(kinematic.velocity, None) * time.delta_seconds();

            // Rotate
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

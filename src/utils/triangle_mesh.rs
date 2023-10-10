use bevy::{render::{render_resource::PrimitiveTopology, mesh::Indices}, prelude::{Mesh, Vec3}};

pub fn create_2d() -> Mesh {
    let vertices = vec![
        Vec3::new(-0.5, -0.3, 0.),
        Vec3::new(0.5, 0., 0.),
        Vec3::new(-0.5, 0.3, 0.),
        Vec3::new(-0.4, 0., 0.),
    ];

    let indices = vec![
        0, 1, 3,
        1, 2, 3,
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    mesh
}

pub fn create_3d() -> Mesh {
    let vertices = vec![
        Vec3::new(-0.5, -0.3, 0.),
        Vec3::new(0.5, 0., 0.),
        Vec3::new(-0.5, 0.3, 0.),
        Vec3::new(-0.4, 0., 0.2),
        Vec3::new(-0.4, 0., -0.2),
    ];

    let indices = vec![
        0, 1, 3,
        1, 2, 3,
        0, 1, 4,
        1, 2, 4,
        0, 3, 4,
        1, 3, 4,
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    mesh
}
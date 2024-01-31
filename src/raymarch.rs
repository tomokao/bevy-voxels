use bevy::{
    prelude::*,
    render::{
        camera::CameraProjection,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

use crate::{camera::PlayerCamera, utils::AsVec2};

pub struct RaymarchPlugin;

impl Plugin for RaymarchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<RaymarchMaterial>::default())
            .add_systems(Update, update_screen_quad);
    }
}

#[derive(Default)]
struct ScreenQuad {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<RaymarchMaterial>,
}

fn update_screen_quad(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut screen_quad: Local<ScreenQuad>,
    mut windows: Query<&mut Window>,
    mut materials: ResMut<Assets<RaymarchMaterial>>,
    events: EventReader<WindowResized>,
    player_camera: Query<(&Projection, &Transform), With<PlayerCamera>>,
) {
    if let Some(material) = materials.get_mut(&screen_quad.material_handle) {
        let window = windows.single_mut();
        let (projection, transform) = player_camera.single();

        material.pos = transform.translation;
        material.rotation = transform.rotation.into();
        material.inverse_camera = projection.get_projection_matrix().inverse();
        material.resolution = window.resolution.as_vec2();
    }

    match meshes.get_mut(&screen_quad.mesh_handle) {
        None => {
            let window = windows.single_mut();
            let (projection, transform) = player_camera.single();

            let mesh = Mesh::from(shape::Quad::new(window.resolution.as_vec2()));
            let material = RaymarchMaterial {
                pos: transform.translation,
                rotation: transform.rotation.into(),
                inverse_camera: projection.get_projection_matrix().inverse(),
                resolution: window.resolution.as_vec2(),
            };

            let mesh_handle = meshes.add(mesh);
            let material_handle = materials.add(material);

            screen_quad.mesh_handle = mesh_handle.clone();
            screen_quad.material_handle = material_handle.clone();

            commands.spawn(Camera2dBundle::default());
            commands.spawn(MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            });
        }
        Some(mesh) if !events.is_empty() => {
            let window = windows.single_mut();

            let mut new_mesh = Mesh::from(shape::Quad::new(window.resolution.as_vec2()));

            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                new_mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap(),
            );
        }
        _ => {}
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct RaymarchMaterial {
    #[uniform(0)]
    pos: Vec3,
    #[uniform(1)]
    rotation: Vec4,
    #[uniform(2)]
    inverse_camera: Mat4,
    #[uniform(3)]
    resolution: Vec2, // TODO: use this
}

impl Material2d for RaymarchMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/raymarch.wgsl".into()
    }
}

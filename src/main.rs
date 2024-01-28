use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<ScreenQuadMaterial>::default(),
        ))
        .add_systems(Update, update_screen_quad)
        .run();
}

#[derive(Default)]
struct ScreenQuad {
    mesh_handle: Handle<Mesh>,
}

fn update_screen_quad(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut screen_quad: Local<ScreenQuad>,
    mut windows: Query<&mut Window>,
    mut materials: ResMut<Assets<ScreenQuadMaterial>>,
    events: EventReader<WindowResized>,
) {
    match meshes.get_mut(&screen_quad.mesh_handle) {
        None => {
            let window = windows.single_mut();
            let mesh = Mesh::from(shape::Quad::new(Vec2::new(
                window.resolution.width(),
                window.resolution.height(),
            )));
            let mesh_handle = meshes.add(mesh);
            screen_quad.mesh_handle = mesh_handle.clone();
            commands.spawn(Camera2dBundle::default());
            commands.spawn(MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(ScreenQuadMaterial {}),
                ..default()
            });
        }
        Some(mesh) if !events.is_empty() => {
            let window = windows.single_mut();
            let mut new_mesh = Mesh::from(shape::Quad::new(Vec2::new(
                window.resolution.width(),
                window.resolution.height(),
            )));
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                new_mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap(),
            );
        }
        _ => {}
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ScreenQuadMaterial {}

impl Material2d for ScreenQuadMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screen_quad.wgsl".into()
    }
}

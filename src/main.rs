use bevy::{
    prelude::*,
    render::mesh::VertexAttributeValues,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResized,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<WindowResized>,
) {
    match meshes.get_mut(&screen_quad.mesh_handle) {
        None => {
            let window = windows.single_mut();
            let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(
                window.resolution.width(),
                window.resolution.height(),
            )));
            let vertex_colors = vec![
                Color::RED.as_rgba_f32(),
                Color::GREEN.as_rgba_f32(),
                Color::BLUE.as_rgba_f32(),
                Color::WHITE.as_rgba_f32(),
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
            let mesh_handle = meshes.add(mesh);
            screen_quad.mesh_handle = mesh_handle.clone();
            commands.spawn(Camera2dBundle::default());
            commands.spawn(MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: color_materials.add(ColorMaterial::default()),
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

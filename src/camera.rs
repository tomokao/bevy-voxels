use bevy::prelude::*;
use bevy_flycam::prelude::*;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NoCameraPlayerPlugin)
            .insert_resource(KeyBindings {
                move_forward: KeyCode::Comma,
                move_backward: KeyCode::O,
                move_left: KeyCode::A,
                move_right: KeyCode::E,
                move_ascend: KeyCode::Space,
                move_descend: KeyCode::ShiftLeft,
                toggle_grab_cursor: KeyCode::Escape,
            })
            .add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct PlayerCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                is_active: false,
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: f32::to_radians(90.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCamera,
        FlyCam,
    ));
}

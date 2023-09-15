use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use big_space::{FloatingOriginPlugin, GridCell};

mod camera;
mod speed;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<TransformPlugin>(),
            FloatingOriginPlugin::<i64>::new(100.0, 10.0),
            camera::CameraPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, (spawn_floor, spawn_light))
        .add_systems(Update, toggle_vsync)
        .run();
}

fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        GridCell::<i64>::new(0, 0, 0),
        PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
    );

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        GridCell::<i64>::new(0, 0, 0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
    );

    commands.spawn(floor);
}

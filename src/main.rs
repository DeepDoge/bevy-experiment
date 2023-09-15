use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use big_space::{FloatingOriginPlugin, GridCell};

mod camera;
mod speed;

const GRID_EDGE_LENGTH: f32 = 100_000.0 * 2.0;
const SWITCHING_THRESHOLD: f32 = GRID_EDGE_LENGTH * 0.5;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<TransformPlugin>(),
            FloatingOriginPlugin::<i64>::new(GRID_EDGE_LENGTH, SWITCHING_THRESHOLD),
            camera::CameraPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            #[cfg(debug_assertions)]
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup, spawn_floor, test))
        .add_systems(Update, toggle_vsync)
        .run();
}

fn setup(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 1.0;
}

fn test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&GridCell<i64>, With<Camera>>,
) {
    let player_cell = *player_query.single();

    let material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

    for x in -5..5 {
        for y in -5..5 {
            for z in -5..5 {
                let sphere = (
                    GridCell::<i64>::from(player_cell + GridCell::new(x, y, z)),
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())),
                        material: material.clone(),
                        transform: Transform::from_xyz(50_000.0, 50_000.0, 50_000.0)
                            .with_scale(Vec3::splat(50_000.0)),
                        ..default()
                    },
                );

                commands.spawn(sphere);
            }
        }
    }
}

fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = match window.present_mode {
            PresentMode::AutoNoVsync => PresentMode::AutoVsync,
            _ => PresentMode::AutoNoVsync,
        };

        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
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

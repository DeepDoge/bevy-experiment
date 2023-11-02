use std::time::Duration;

use bevy::{
    asset::ChangeWatcher,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec2,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RawRenderPipelineDescriptor, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
use big_space::{FloatingOriginPlugin, GridCell};

mod camera;
mod speed;

const GRID_EDGE_LENGTH: f32 = 100_000.0 * 2.0;
const SWITCHING_THRESHOLD: f32 = GRID_EDGE_LENGTH * 0.5;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<TransformPlugin>()
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
            ShaderUtilsPlugin,
            MaterialPlugin::<CustomMaterial>::default(),
            FloatingOriginPlugin::<i64>::new(GRID_EDGE_LENGTH, SWITCHING_THRESHOLD),
            camera::CameraPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_vsync)
        .run();
}

fn setup(
    mut ambient_light: ResMut<AmbientLight>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 1.0;

    let quad_mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.0,
        subdivisions: 0,
        ..default()
    }));

    // Shell Texturing Test
    let shell_count = 64;
    let size = 10_000.0;
    for i in 0..shell_count {
        let local_height = i as f32 / shell_count as f32;
        let quad = (
            GridCell::<i64>::ZERO,
            MaterialMeshBundle {
                mesh: quad_mesh.clone(),
                transform: Transform {
                    translation: Vec3::ZERO,
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(size, 1.0, size),
                },
                material: custom_materials.add(CustomMaterial {
                    height: local_height,
                    size,
                }),
                ..default()
            },
        );

        commands.spawn(quad);
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct CustomMaterial {
    #[uniform(0)]
    height: f32,
    #[uniform(1)]
    size: f32,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grass_test.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/grass_test.wgsl".into()
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

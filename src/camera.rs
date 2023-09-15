use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::CursorGrabMode,
};
use big_space::{FloatingOrigin, GridCell};

use crate::speed::Speed;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (update_camera_speed, rotate_camera, move_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Speed { value: 5.0 },
        FloatingOrigin,
        GridCell::<i64>::new(0, 0, 0),
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    );
    commands.spawn(camera);
}

// change camera speed on mouse scroll
fn update_camera_speed(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Speed, With<Camera>>,
) {
    for event in mouse_wheel_events.iter() {
        for mut speed in query.iter_mut() {
            if event.y > 0.0 {
                speed.value *= 2.0;
            } else if event.y < 0.0 {
                speed.value *= 0.5;
            }
        }
    }
}

fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Camera>>,
) {
    let (mut camera_transform, speed) = query.single_mut();

    let mut translation = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        translation += camera_transform.forward();
    }
    if keyboard_input.pressed(KeyCode::S) {
        translation += camera_transform.back();
    }
    if keyboard_input.pressed(KeyCode::A) {
        translation += camera_transform.left();
    }
    if keyboard_input.pressed(KeyCode::D) {
        translation += camera_transform.right();
    }

    camera_transform.translation +=
        translation.normalize_or_zero() * speed.value * time.delta_seconds();
}

fn cursor_visible(window: &mut Window, visible: bool) {
    if window.cursor.visible == visible {
        return;
    }

    if visible {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    } else {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    let mut camera_transform = query.single_mut();

    let rotation_speed = 1.5;

    let mut mouse_delta = Vec2::ZERO;
    if buttons.pressed(MouseButton::Right) {
        cursor_visible(&mut window, false);
        for event in mouse_motion_events.iter() {
            mouse_delta += event.delta;
        }
    } else {
        cursor_visible(&mut window, true);
    }

    let rotate_x = -mouse_delta.y * 0.003;
    let rotate_y = -mouse_delta.x * 0.003;
    let rotate_z = ((keyboard_input.pressed(KeyCode::Q) as i32
        - keyboard_input.pressed(KeyCode::E) as i32) as f32)
        * time.delta_seconds();

    camera_transform.rotation *= Quat::from_axis_angle(Vec3::Y, rotate_y * rotation_speed);
    camera_transform.rotation *= Quat::from_axis_angle(Vec3::X, rotate_x * rotation_speed);
    camera_transform.rotation *= Quat::from_axis_angle(Vec3::Z, rotate_z * rotation_speed);
}

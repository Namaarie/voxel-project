use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};

pub struct OrbitCameraPlugin {
    mouse_settings: MouseSettings,
    camera_settings: CameraSettings,
}

impl Default for OrbitCameraPlugin {
    fn default() -> Self {
        Self {
            mouse_settings: MouseSettings::default(),
            camera_settings: CameraSettings::default(),
        }
    }
}

#[derive(Resource, Clone, Copy)]
struct MouseSettings {
    sensitivity: f32,
    scroll_sensitivity_line: f32,
    scroll_sensitivity_pixel: f32
}

#[derive(Resource, Clone, Copy)]
struct CameraSettings {
    close: f32,
    far: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            close: 1.0,
            far: 20.0,
        }
    }
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.012,
            scroll_sensitivity_line: 1.0,
            scroll_sensitivity_pixel: 0.01,
        }
    }
}

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.camera_settings)
            .insert_resource(self.mouse_settings)
            .add_systems(Startup, setup)
            .add_systems(Update, mouse_input);
    }
}

#[derive(Component)]
pub struct OrbitCamera;

fn setup(
    mut commands: Commands,
) {
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
        OrbitCamera,
    ));
}

fn mouse_input(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<OrbitCamera>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mouse_settings: Res<MouseSettings>,
    mut mouse_wheel: EventReader<MouseWheel>,
    camera_settings: Res<CameraSettings>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        for mm in mouse_motion.read() {
            if buttons.pressed(MouseButton::Left) {
                // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
                let rotation_horizontal = Quat::from_rotation_y(-(mm.delta.x * mouse_settings.sensitivity));
                let rotation_vertical = Quat::from_axis_angle(camera_transform.local_x().as_vec3(), -(mm.delta.y * mouse_settings.sensitivity));
                let rotation = rotation_horizontal * rotation_vertical;
                camera_transform.rotate_around(Vec3::from([0.0; 3]), rotation);
            }
        }

        use bevy::input::mouse::MouseScrollUnit;

        // println!("{}", camera_transform.translation.length());

        let prev_transform= camera_transform.clone();

        for ev in mouse_wheel.read() {
            let forward_vec = camera_transform.forward();
            // info!("scrolling {} {:?}", ev.y, ev.unit);
            match ev.unit {
                MouseScrollUnit::Line => {
                    camera_transform.translation += forward_vec * mouse_settings.scroll_sensitivity_line * ev.y;
                }
                MouseScrollUnit::Pixel => {
                    camera_transform.translation += forward_vec * mouse_settings.scroll_sensitivity_pixel * ev.y;
                }
            }

            if camera_transform.translation.length() <= camera_settings.close {
                camera_transform.clone_from(&prev_transform)
            }

            if camera_transform.translation.length() >= camera_settings.far {
                camera_transform.clone_from(&prev_transform)
            }
        }
    }
}
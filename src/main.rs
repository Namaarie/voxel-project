use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, window::PrimaryWindow};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use std::collections::VecDeque;
use log::Level;
use log::info;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
        ))
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            keyboard_input,
            mouse_input,
        ))
        .init_resource::<Octree>()
        .init_resource::<MouseSettings>()
        .init_resource::<CameraSettings>()
        .run();
}
#[derive(Resource)]
struct MouseSettings {
    sensitivity: f32,
    scroll_sensitivity_line: f32,
    scroll_sensitivity_pixel: f32
}

#[derive(Resource)]
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

struct Octant {
    children: Option<Vec<Octant>>,
    color: Color,
    position: Vec3,
    depth: u32
}

impl Default for Octree {
    fn default() -> Self {
        Octree::new()
    }
}

impl Octant {
    fn new(pos: Vec3, depth: u32, color: Color) -> Self {
        Self {
            children: None,
            color,
            position: pos,
            depth
        }
    }

    pub fn get_size(&self) -> f32 {
        return Self::get_size_from_depth(self.depth);
    }

    fn get_size_from_depth(depth: u32) -> f32 {
        1.0 / (1 << depth) as f32
    }

    fn create_children(&mut self) {
        let mut children: Vec<Octant> = Vec::with_capacity(8);
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    let child_size = Self::get_size_from_depth(self.depth+1);
                    let pos = self.position + Vec3::from((x as f32, y as f32, z as f32)) * Vec3::from([child_size; 3]) - Vec3::from([child_size / 2.0; 3]);
                    children.push(Octant::new(pos, self.depth+1, self.color));
                }
            }
        }
        self.children = Some(children);
    }
}

#[derive(Resource)]
struct Octree {
    root: Octant
}

impl Octree {
    fn new() -> Self {
        Self {
            root: Octant::new(Vec3::new(0.0, 0.0, 0.0), 0, Color::linear_rgba(0.0, 0.5, 0.0, 0.25))
        }
    }
}

#[derive(Component)]
pub struct OrbitCamera;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut octree: ResMut<Octree>,
) {
    console_log::init_with_level(Level::Debug);

    info!("It works!");

    let mut deq: VecDeque<&Octant> = VecDeque::new();
    let root = &mut octree.root;

    root.create_children();
    deq.push_back(&root);

    while !deq.is_empty() {
        let octant = deq.pop_front().unwrap();

        if let Some(children) = octant.children.as_ref() {
            for i in children {
                deq.push_back(i);
            }
        }

        let size = octant.get_size();

        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(size, size, size)),
            material: materials.add(octant.color),
            transform: Transform::from_translation(octant.position),
            ..default()
        });
    }

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
        OrbitCamera,
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
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
            info!("scrolling {} {:?}", ev.y, ev.unit);
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

fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {

}
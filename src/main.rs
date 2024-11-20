use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, window::PrimaryWindow};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use octree::{Octant, Octree};
use orbit_camera::OrbitCameraPlugin;
use std::collections::VecDeque;
use log::Level;
use log::info;

mod orbit_camera;
mod octree;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            OrbitCameraPlugin::default(),
        ))
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin
        ))
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            keyboard_input,
            mouse_input,
        ))
        .init_resource::<Octree>()
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut octree: ResMut<Octree>,
) {
    let _ = console_log::init_with_level(Level::Debug);

    let mut deq: VecDeque<&Octant> = VecDeque::new();
    let root = &mut octree.root;

    let size = root.get_size();
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(size, size, size)),
        material: materials.add(root.color),
        transform: Transform::from_translation(root.position),
        ..default()
    });

    // root.create_children();
    // deq.push_back(&root);

    // while !deq.is_empty() {
    //     let octant = deq.pop_front().unwrap();

    //     if let Some(children) = octant.children.as_ref() {
    //         for i in children {
    //             deq.push_back(i);
    //         }
    //     }

    //     let size = octant.get_size();

    //     commands.spawn(PbrBundle {
    //         mesh: meshes.add(Cuboid::new(size, size, size)),
    //         material: materials.add(octant.color),
    //         transform: Transform::from_translation(octant.position),
    //         ..default()
    //     });
    // }

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
    buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel: EventReader<MouseWheel>,
) {

}

fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {

}
use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowMode};
use playable::PlayablePlugin;

mod controls;
mod playable;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Index(1)),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PlayablePlugin,
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.))
        .add_systems(Startup, (spawn_camera, spawn_floor))
        .run();
}

fn spawn_camera(mut commands: Commands) -> () {
    commands.spawn(Camera2d);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let dimensions = (3000., 30.);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(dimensions.0, dimensions.1))),
        MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
        Transform::from_xyz(0., -400., 0.),
        RigidBody::Static,
        Collider::rectangle(dimensions.0, dimensions.1),
    ));
}

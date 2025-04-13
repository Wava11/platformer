use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;

pub struct PlayablePlugin;
impl Plugin for PlayablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementEvent>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (keyboard_input, move_playable, apply_movement_damping).chain(),
            );
    }
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementEvent>,
    mut query: Query<&mut LinearVelocity, With<Playable>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        for mut velocity in query.iter_mut() {
            let next_velocity = velocity.x + direction * MOVE_SPEED * time.delta_secs();
            if velocity.x.abs() <= MAX_X_SPEED {
                velocity.x = next_velocity.clamp(-MAX_X_SPEED, MAX_X_SPEED);
            }
        }
    }

    if keyboard_input.any_just_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        for mut velocity in query.iter_mut() {
            velocity.y += JUMP_SPEED * time.delta_secs();
        }
        // movement_event_writer.send(MovementEvent::Jump);
    }
}
// fn keyboard_input(
//     mut movement_event_writer: EventWriter<MovementEvent>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {
//     let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
//     let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

//     let horizontal = right as i8 - left as i8;
//     let direction = horizontal as Scalar;

//     if direction != 0.0 {
//         movement_event_writer.send(MovementEvent::Move(direction));
//     }

//     if keyboard_input.any_just_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
//         movement_event_writer.send(MovementEvent::Jump);
//     }
// }

fn move_playable(
    mut movement_event_reader: EventReader<MovementEvent>,
    mut query: Query<&mut LinearVelocity, With<Playable>>,
    time: Res<Time>,
) {
    for event in movement_event_reader.read() {
        for mut velocity in query.iter_mut() {
            match event {
                MovementEvent::Move(direction) => {
                    let next_velocity = velocity.x + direction * MOVE_SPEED * time.delta_secs();
                    if velocity.x.abs() <= MAX_X_SPEED {
                        velocity.x = next_velocity.clamp(-MAX_X_SPEED, MAX_X_SPEED);
                    }
                }
                MovementEvent::Jump => {
                    velocity.y += JUMP_SPEED * time.delta_secs();
                }
            }
        }
    }
}
const MOVE_SPEED: f32 = 30000.;
const MAX_X_SPEED: f32 = 200.;
const JUMP_SPEED: f32 = 10000.;
const FRICTION: f32 = 10.;

#[derive(Event)]
enum MovementEvent {
    Move(Scalar),
    Jump,
}

pub fn spawn_player(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        Playable,
        // Mesh2d(meshes.add(Rectangle::new(30., 30.))),
        // MeshMaterial2d(materials.add(Color::srgb(0., 0., 1.))),
        Sprite::from_image(assets_server.load("player.png")),
        AngularInertia(f32::MAX),
        Friction::new(0.)
            .with_dynamic_coefficient(0.)
            .with_combine_rule(CoefficientCombine::Min),
        MovementDampingFactor(0.92),
        Transform::from_xyz(0., 0., 0.),
        RigidBody::Dynamic,
        Collider::rectangle(22., 34.),
    ));
}

#[derive(Component)]
pub struct Playable;

#[derive(Component)]
struct MovementDampingFactor(f32);

fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= if linear_velocity.x.abs() > 100. {
            damping_factor.0
        } else {
            0.
        };
    }
}

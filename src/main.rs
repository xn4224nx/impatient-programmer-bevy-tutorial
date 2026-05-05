/*
 * The Impatient Programmer's Guide to Bevy and Rust
 * https://dev.to/jamesfebin/
 */

use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);

    cmds.spawn((
        Text2d::new("*"),
        TextFont {
            font_size: 200.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        Player,
    ));
}

/// Based on the keyboard input, move the player sprite.
fn move_player(
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut direct = Vec2::ZERO;

    if key_input.pressed(KeyCode::KeyA) {
        direct.x -= 1.0;
    }
    if key_input.pressed(KeyCode::KeyD) {
        direct.x += 1.0;
    }
    if key_input.pressed(KeyCode::KeyW) {
        direct.y += 1.0;
    }
    if key_input.pressed(KeyCode::KeyS) {
        direct.y -= 1.0;
    }

    if direct != Vec2::ZERO {
        let change = direct.normalize() * 300.0 * time.delta_secs();
        player_transform.translation.x += change.x;
        player_transform.translation.y += change.y;
    }
}

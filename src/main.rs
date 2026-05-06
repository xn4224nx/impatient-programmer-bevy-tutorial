/*
 * The Impatient Programmer's Guide to Bevy and Rust
 * https://dev.to/jamesfebin/
 */

use bevy::prelude::*;

const TILE_SZ: u32 = 64; // 64 x 64 tiles
const WALK_FRAMES: usize = 9; // 9 cols per walking row
const MOVE_SPEED: f32 = 140.0; // pixels per second
const ANIM_DT: f32 = 0.1; // seconds per frame

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "./assets".into(),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(Update, (move_player, animate_player))
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);


}

/// Based on the keyboard input, move the player sprite.
fn move_player(
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    /* There should be exactly one player. */
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

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

    /* Has the direction changed? */
    if direct != Vec2::ZERO {
        let change = direct.normalize() * 300.0 * time.delta_secs();
        transform.translation.x += change.x;
        transform.translation.y += change.y;
        anim.moving = true;

        /* Change the sprite direction. */
        anim.facing = if direct.x.abs() > direct.y.abs() {
            if direct.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else {
            if direct.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            }
        };
    } else {
        anim.moving = false;
    }
}

fn spawn_player(
    mut cmds: Commands,
    assert_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    /* Load the character spritesheet and build the grid. */
    let texture = assert_server.load("efnysien_spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SZ),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    /* Start facing away from the user and stationary. */
    let start_direct = Facing::Up;

    cmds.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: atlas_index_for(start_direct, 0),
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Player,
        AnimationState {
            facing: start_direct,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    /* There should be exactly one player. */
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    /* Compute the targets posotion in the 9-col-row */
    let target_row = row_zero_based(anim.facing);
    let mut current_col = atlas.index % WALK_FRAMES;
    let mut current_row = atlas.index / WALK_FRAMES;

    /* Snap to the first row if the facing has changed. */
    if current_row != target_row {
        atlas.index = row_start_index(anim.facing);
        current_col = 0;
        current_row = target_row;
        timer.reset();
    }


    if anim.moving {
        if !anim.was_moving {
            let row_start = row_start_index(anim.facing);
            let next_col = (current_col + 1) % WALK_FRAMES;
            atlas.index = row_start + next_col;
            timer.reset()

        /* Has the sprite just started moving? */
        } else {
            timer.tick(time.delta());
            if timer.just_finished() {
                let row_start = row_start_index(anim.facing);
                let next_col = (current_col + 1) % WALK_FRAMES;
                atlas.index = row_start + next_col;
            }
        }

    /* If the sprite has just stopped. */
    } else if !anim.moving && anim.was_moving {
        timer.reset()
    }

    anim.was_moving = anim.moving;
}

fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}

fn atlas_index_for(facing: Facing, frame_in_row: usize) -> usize {
    row_start_index(facing) + frame_in_row.min(WALK_FRAMES - 1)
}

fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 8,
        Facing::Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}

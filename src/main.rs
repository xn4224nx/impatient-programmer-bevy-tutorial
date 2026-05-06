/*
 * The Impatient Programmer's Guide to Bevy and Rust
 * https://dev.to/jamesfebin/
 */

use bevy::prelude::*;

mod avatar;

use crate::avatar::AvatarPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "./assets".into(),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_plugins(AvatarPlugin)
        .run();
}

fn setup_camera(mut cmds: Commands) {
    cmds.spawn(Camera2d);
}

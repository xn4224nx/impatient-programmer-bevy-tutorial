/*
 * WORLD/GENERATE
 * ==============
 */

use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::*;

use crate::world::{
    assets::{load_assets, prepare_tilemap_handles},
    rules::build_world,
};

/* Define the size of the map size. */
pub const GRID_X: u32 = 25;
pub const GRID_Y: u32 = 18;
const GRID_Z: u32 = 2;

const ASSETS_PATH: &str = "tilemaps";
const TILEMAP_FILE: &str = "tilemap_grassland.png";

/* What is the size of a block in world units. */
pub const TILE_SIZE: f32 = 32.0;

/* What is the size of a grid node in world units. */
const NODE_SIZE: Vec3 = Vec3::new(TILE_SIZE, TILE_SIZE, 1.0);

const ASSETS_SCALE: Vec3 = Vec3::ONE;

pub fn map_pixel_dimensions() -> Vec2 {
    Vec2::new(TILE_SIZE * GRID_X as f32, TILE_SIZE * GRID_Y as f32)
}

pub fn setup_generator(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    /* Rule Initalisation */
    let (assets_definitions, models, socket_collection) = build_world();
    let rules = RulesBuilder::new_cartesian_3d(models, socket_collection)
        .with_rotation_axis(Direction::ZForward)
        .build()
        .unwrap();

    /* Create the 3D world, ie the Grid. */
    let grid = CartesianGrid::new_cartesian_3d(GRID_X, GRID_Y, GRID_Z, false, false, false);

    /* Create the WFC algorithm. */
    let gen_builder = GeneratorBuilder::new()
        .with_rules(rules)
        .with_grid(grid.clone())
        .with_rng(RngMode::RandomSeed)
        .with_node_heuristic(NodeSelectionHeuristic::MinimumRemainingValue)
        .with_model_heuristic(ModelSelectionHeuristic::WeightedProbability);
    let generator = gen_builder.build().unwrap();

    /* Load the sprites and convert them to renderable assets. */
    let tilemap_handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);
    let models_assets = load_assets(&tilemap_handles, assets_definitions);

    /* Spawn the Generator. */
    cmds.spawn((
        Transform::from_translation(Vec3 {
            x: -TILE_SIZE * grid.size_x() as f32 / 2.0,
            y: -TILE_SIZE * grid.size_y() as f32 / 2.0,
            z: 0.0,
        }),
        grid,
        generator,
        NodesSpawner::new(models_assets, NODE_SIZE, ASSETS_SCALE).with_z_offset_from_y(true),
    ));
}

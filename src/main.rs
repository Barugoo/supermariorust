use crate::components::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use systems::*;
use systems::{
    BLOCK, MAX_WALK_SPEED, METER, MIN_WALK_SPEED, PIXELS_PER_METER, SKIDD_DECELER, SSS_PIXEL,
    SS_PIXEL, S_PIXEL,
};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER as f32),
        ))
        .insert_resource(RapierConfiguration {
            ..Default::default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::Index(0))
        .add_systems(Startup, systems::setup)
        .add_systems(Update, systems::spawn_wall_collision)
        .add_systems(Update, systems::movement)
        .add_systems(Update, systems::camera_fit_inside_current_level)
        .add_systems(Update, systems::update_on_ground)
        .add_systems(Update, spawn_ground_sensor)
        .add_systems(Update, ground_detection)
        .add_systems(Update, update_level_selection)
        .add_systems(Update, patrol)
        .add_systems(Update, restart_level)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::MobBundle>("Enemy")
        .run();
}

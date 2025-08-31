use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod bird;
mod feeder;
mod camera;
mod ui;
mod audio;
mod resources;
mod bird_ai;
mod animation;
mod photo_mode;
mod journal;
mod user_interface;
mod despawn;

use user_interface::UserInterfacePlugin;
use bird::BirdPlugin;
use feeder::FeederPlugin;
use camera::CameraPlugin;
use ui::UiPlugin;
use audio::AudioPlugin as GameAudioPlugin;
use resources::GameConfig;
use bird_ai::BirdAiPlugin;
use animation::AnimationPlugin;
use photo_mode::PhotoModePlugin;
use journal::JournalPlugin;
use despawn::{robust_despawn_system};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Playing,
    Journal,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .init_state::<AppState>()
        .init_resource::<GameConfig>()
        .add_plugins((
            BirdPlugin,
            FeederPlugin,
            CameraPlugin,
            UiPlugin,
            GameAudioPlugin,
            BirdAiPlugin,
            AnimationPlugin,
            UserInterfacePlugin,
            PhotoModePlugin,
            JournalPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, robust_despawn_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, photo_mode::components::PhotoTarget));
}
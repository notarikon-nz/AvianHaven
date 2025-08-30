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

use bird::BirdPlugin;
use feeder::FeederPlugin;
use camera::CameraPlugin;
use ui::UiPlugin;
use audio::AudioPlugin as GameAudioPlugin;
use resources::GameConfig;
use bird_ai::BirdAiPlugin;
use animation::AnimationPlugin;
use photo_mode::PhotoModePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Playing,
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
            PhotoModePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, photo_mode::components::PhotoTarget));
}
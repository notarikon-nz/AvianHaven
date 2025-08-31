use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod bird;
mod feeder;
mod camera;
mod ui;
mod resources;
mod bird_ai;
mod animation;
mod photo_mode;
mod journal;
mod user_interface;
mod despawn;
mod audio;
mod achievements;
mod notifications;
mod environment;
mod steam;
mod performance;
mod flocking;
mod weather_effects;
mod bird_data;

use user_interface::UserInterfacePlugin;
use bird::BirdPlugin;
use feeder::FeederPlugin;
use camera::CameraPlugin;
use ui::UiPlugin;
use resources::GameConfig;
use bird_ai::BirdAiPlugin;
use animation::AnimationPlugin;
use photo_mode::PhotoModePlugin;
use journal::JournalPlugin;
use despawn::{robust_despawn_system};
use audio::AudioPlugin;
use achievements::AchievementPlugin;
use notifications::NotificationPlugin;
use environment::EnvironmentPlugin;
use steam::SteamPlugin;
use performance::PerformancePlugin;
use flocking::FlockingPlugin;
use weather_effects::WeatherEffectsPlugin;
use bird_data::BirdDataPlugin;

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
            AudioPlugin,
            BirdAiPlugin,
            AnimationPlugin,
            UserInterfacePlugin,
            PhotoModePlugin,
            JournalPlugin,
            AchievementPlugin,
            NotificationPlugin,
            EnvironmentPlugin,
            SteamPlugin,
            PerformancePlugin,
        ))
        .add_plugins(FlockingPlugin)
        .add_plugins(WeatherEffectsPlugin)
        .add_plugins(BirdDataPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, robust_despawn_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, photo_mode::components::PhotoTarget));
}
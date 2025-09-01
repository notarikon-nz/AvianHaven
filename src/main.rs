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
mod smart_objects;
mod aesthetic_objects;
mod catalog;
mod save_load;
mod menu;
mod tutorial;

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
use smart_objects::SmartObjectsPlugin;
use aesthetic_objects::AestheticObjectsPlugin;
use catalog::CatalogPlugin;
use save_load::SaveLoadPlugin;
use menu::MenuPlugin;
use tutorial::TutorialPlugin;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Playing,
    Journal,
    Settings,
    LoadGame,
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
        .add_plugins(SmartObjectsPlugin)
        .add_plugins(AestheticObjectsPlugin)
        .add_plugins(CatalogPlugin)
        .add_plugins(SaveLoadPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(TutorialPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, robust_despawn_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d, 
        Transform::from_xyz(0.0, 0.0, 1000.0)
            .with_rotation(Quat::from_rotation_x(-0.5)), // 2.5D angled view like Neko Atsume
        photo_mode::components::PhotoTarget,
        photo_mode::components::CameraControls {
            zoom_level: 1.0,
            min_zoom: 0.5,
            max_zoom: 5.0,
            zoom_speed: 2.0,
            focus_distance: 0.0,
            aperture: 5.6,
            exposure: 0.0,
            iso: 400.0,
        },
    ));
}
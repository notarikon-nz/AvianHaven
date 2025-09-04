use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
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
mod ui_widgets;
mod tooltip;
mod keybindings;
mod lunex_ui;
mod nocturnal_behaviors;
mod advanced_weather; // Advanced weather effects and storm behaviors
mod foraging_ecology; // Advanced foraging patterns and mixed flocks
mod social_features; // Phase 4: Community features
mod sanctuary_management; // Phase 4: Advanced sanctuary management
mod hanabi_effects; // Phase 4: GPU-accelerated particle effects
mod predator_prey; // Phase 4: Predator-prey dynamics
mod ui_diagnostic; // UI diagnostic and testing tools

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
use nocturnal_behaviors::NocturnalBehaviorPlugin;
use advanced_weather::AdvancedWeatherPlugin;
use foraging_ecology::ForagingEcologyPlugin;
use social_features::SocialFeaturesPlugin;
use sanctuary_management::SanctuaryManagementPlugin;
use hanabi_effects::HanabiEffectsPlugin;
use predator_prey::PredatorPreyPlugin;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Playing,
    Journal,
    Catalog,
    Settings,
    LoadGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            FrameTimeDiagnosticsPlugin::default(),
        ))
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
        .add_plugins(keybindings::KeyBindingsPlugin)
        .add_plugins(lunex_ui::LunexUiPlugin)
        .add_plugins(TutorialPlugin)
        .add_plugins(tooltip::TooltipPlugin)
        .add_plugins(NocturnalBehaviorPlugin)
        .add_plugins(AdvancedWeatherPlugin)
        .add_plugins(ForagingEcologyPlugin)
        .add_plugins(PredatorPreyPlugin)
        .add_plugins(SocialFeaturesPlugin)
        .add_plugins(SanctuaryManagementPlugin)
        .add_plugins(HanabiEffectsPlugin)
        .add_plugins(ui_diagnostic::UiDiagnosticPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, robust_despawn_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0)
            // .with_rotation(Quat::from_rotation_x(-0.5)), // 2.5D angled view like Neko Atsume
            .with_rotation(Quat::from_rotation_x(0.0)), // 2.5D angled view like Neko Atsume
        // Ensure orthographic projection uses full window
        Projection::Orthographic(OrthographicProjection::default_2d()),
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
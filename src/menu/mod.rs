use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use components::*;
use resources::*;
use systems::*;
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuState>()
            .init_resource::<GameSettings>()
            .add_event::<MenuNavigationEvent>()
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(AppState::Settings), setup_settings_menu)
            .add_systems(OnEnter(AppState::LoadGame), setup_load_game_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu_ui)
            .add_systems(OnExit(AppState::Settings), cleanup_menu_ui)
            .add_systems(OnExit(AppState::LoadGame), cleanup_menu_ui)
            .add_systems(Update, (
                main_menu_button_system,
                settings_button_system,
                load_game_button_system,
                menu_navigation_system,
                escape_key_system,
            ));
    }
}
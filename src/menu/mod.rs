use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

// use components::*;
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
            .add_event::<crate::ui_widgets::SliderValueChanged>()
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu) // Re-enabled as fallback
            .add_systems(OnEnter(AppState::Settings), setup_settings_menu)
            .add_systems(OnEnter(AppState::LoadGame), setup_load_game_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu_ui)
            .add_systems(OnExit(AppState::Settings), cleanup_menu_ui)
            .add_systems(OnExit(AppState::LoadGame), cleanup_menu_ui)
            .add_systems(Update, (
                main_menu_button_system,
                menu_navigation_system,
            ).run_if(in_state(AppState::MainMenu))) // Re-enabled as fallback
            .add_systems(Update, (
                settings_button_system,
                crate::ui_widgets::slider_interaction_system,
                volume_slider_update_system,
                graphics_toggle_system,
                handle_controls_menu,
                // New simplified widget systems
                resolution_dropdown_system,
                graphics_quality_dropdown_system,
                settings_toggle_system,
            ).run_if(in_state(AppState::Settings)))
            .add_systems(Update, load_game_button_system.run_if(in_state(AppState::LoadGame)))
            .add_systems(Update, escape_key_system.run_if(
                in_state(AppState::MainMenu)
                    .or(in_state(AppState::Settings))
                    .or(in_state(AppState::LoadGame))
            ));
    }
}
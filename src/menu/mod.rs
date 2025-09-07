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
        info!("Registering MenuPlugin with resolution dropdown setup");
        app
            .init_resource::<MenuState>()
            .init_resource::<crate::ui_widgets::CursorPosition>()
            .add_event::<MenuNavigationEvent>()
            .add_event::<crate::user_interface::slider::SliderValueChangedEvent>()
            .add_event::<crate::user_interface::dropdown::DropdownChangedEvent>()
            .add_systems(Startup, load_settings_on_startup)
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu) // Re-enabled as fallback
            .add_systems(OnEnter(AppState::Settings), (setup_settings_menu, setup_audio_sliders_system, setup_resolution_dropdown_system).chain())
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
                menu_navigation_system,
                volume_slider_update_system,
                graphics_toggle_system,
                handle_controls_menu,
                // New simplified widget systems
                resolution_dropdown_system,
                graphics_quality_dropdown_system,
                settings_toggle_system,
                // StateScoped toggle widget system
                fullscreen_toggle_system,
            ).run_if(in_state(AppState::Settings)))
            .add_systems(Update, (tab_test_system, tab_test_escape_system).run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, load_game_button_system.run_if(in_state(AppState::LoadGame)))
            .add_systems(Update, escape_key_system.run_if(
                in_state(AppState::MainMenu)
                    .or(in_state(AppState::Settings))
                    .or(in_state(AppState::LoadGame))
            ));
    }
}
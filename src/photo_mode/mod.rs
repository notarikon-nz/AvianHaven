use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
pub mod advanced_systems;
pub mod advanced_photo; // Phase 4: Advanced Photography Features

use components::*;
use resources::*;
use systems::*;
use advanced_systems::*;

pub struct PhotoModePlugin;

impl Plugin for PhotoModePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PhotoModeSettings>()
            .init_resource::<CurrencyResource>()
            .init_resource::<DiscoveredSpecies>()
            .init_resource::<PhotoCollection>()
            .add_event::<PhotoTakenEvent>()
            .add_systems(Update, (
                toggle_photo_mode_system.run_if(crate::debug_console::console_is_not_visible),
                capture_photo_system.run_if(crate::debug_console::console_is_not_visible),
                photo_reward_system,
                photo_ui_system,
                camera_controls_system.run_if(crate::debug_console::console_is_not_visible),
                composition_grid_system,
                camera_settings_panel_system,
                photo_mode_input_system.run_if(crate::debug_console::console_is_not_visible),
            ))
            .add_systems(Startup, (setup_photo_ui, setup_advanced_photo_ui));
    }
}

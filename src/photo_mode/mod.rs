use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use components::*;
use resources::*;
use systems::*;

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
                toggle_photo_mode_system,
                capture_photo_system,
                photo_reward_system,
                photo_ui_system,
            ))
            .add_systems(Startup, setup_photo_ui);
    }
}

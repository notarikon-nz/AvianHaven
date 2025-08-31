use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
pub mod ui_builder;

use resources::*;
use systems::*;

pub struct JournalPlugin;

impl Plugin for JournalPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DiscoveredSpecies>()
            .init_resource::<JournalData>()
            .add_systems(Update, toggle_journal_system)
            .add_systems(OnEnter(crate::AppState::Journal), setup_journal_menu_system)
            .add_systems(OnExit(crate::AppState::Journal), teardown_journal_menu_system)
            .add_systems(Update, (
                update_journal_on_discovery_system,
                journal_interaction_system,
            ).run_if(in_state(crate::AppState::Journal)));
    }
}


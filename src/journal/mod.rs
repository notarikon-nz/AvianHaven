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
            .init_resource::<JournalState>()
            .init_resource::<BirdEducationData>()
            .init_resource::<ResearchMissionManager>()
            .add_systems(Startup, (load_education_data, setup_research_missions))
            .add_systems(Update, toggle_journal_system.run_if(crate::debug_console::console_is_not_visible))
            .add_systems(OnEnter(crate::AppState::Journal), setup_journal_menu_system) // Re-enabled - using new journal implementation
            .add_systems(OnExit(crate::AppState::Journal), teardown_journal_menu_system) // Re-enabled - using new journal implementation
            .add_systems(Update, (
                update_journal_on_discovery_system,
                journal_interaction_system.run_if(crate::debug_console::console_is_not_visible), // Re-enabled - using new journal implementation
                journal_tab_system.run_if(crate::debug_console::console_is_not_visible), // Re-enabled - using new journal implementation  
                journal_species_detail_system.run_if(crate::debug_console::console_is_not_visible), // Re-enabled - using new journal implementation
                journal_state_monitor_system, // Monitor for state changes and update content
            ).run_if(in_state(crate::AppState::Journal)));
    }
}


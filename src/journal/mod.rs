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
            .add_systems(Update, toggle_journal_system)
            // .add_systems(OnEnter(crate::AppState::Journal), setup_journal_menu_system) // Disabled - using Lunex UI
            // .add_systems(OnExit(crate::AppState::Journal), teardown_journal_menu_system) // Disabled - using Lunex UI
            .add_systems(Update, (
                update_journal_on_discovery_system,
                // journal_interaction_system, // Disabled - using Lunex UI
                // journal_tab_system, // Disabled - using Lunex UI  
                // journal_species_detail_system, // Disabled - using Lunex UI
            ).run_if(in_state(crate::AppState::Journal)));
    }
}


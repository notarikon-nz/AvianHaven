use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;
use crate::AppState;

pub struct CatalogPlugin;

impl Plugin for CatalogPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CatalogState>()
            .init_resource::<PlayerInventory>()
            .init_resource::<PlacedObjects>()
            .add_event::<PurchaseItemEvent>()
            .add_event::<PlaceObjectEvent>()
            .add_systems(OnEnter(AppState::Playing), setup_catalog_items)
            .add_systems(OnExit(AppState::Playing), cleanup_catalog_ui)

            .add_systems(Update, (
                handle_catalog_input, // Re-enabled for keyboard shortcut
                // handle_category_buttons, // Disabled - using Lunex UI
                // handle_purchase_buttons, // Disabled - using Lunex UI
                // handle_place_buttons, // Disabled - using Lunex UI
                handle_purchase_events, // Keep for functionality
                handle_place_object_events, // Keep for functionality
                handle_object_placement, // Keep for functionality
                start_placement_mode, // Keep for functionality
                // update_catalog_visibility, // Disabled - using Lunex UI
            ).run_if(in_state(AppState::Playing)))
           ;
    }
}
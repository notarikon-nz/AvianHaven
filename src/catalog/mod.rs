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
                handle_catalog_input,
                handle_category_buttons,
                handle_purchase_buttons,
                handle_place_buttons,
                handle_purchase_events,
                handle_place_object_events,
                handle_object_placement,
                start_placement_mode,
                update_catalog_visibility,
            ).run_if(in_state(AppState::Playing)))
           ;
    }
}
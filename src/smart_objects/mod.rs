use bevy::prelude::*;

pub mod config;
pub mod systems;
pub mod workshop;
pub mod creation_kit;

use config::*;
use systems::*;
use creation_kit::*;
use workshop::*;

pub struct ConfigurableSmartObjectsPlugin;

impl Plugin for ConfigurableSmartObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<SmartObjectCatalog>()
            .init_resource::<SmartObjectRegistry>()
            .init_resource::<CreationKitState>()
            .init_resource::<CreationKitTemplates>()
            .init_resource::<SteamWorkshopClient>()
            .init_resource::<WorkshopBrowserState>()
            .add_event::<SpawnSmartObjectEvent>()
            .add_event::<RemoveSmartObjectEvent>()
            .add_event::<ModifySmartObjectEvent>()
            .add_event::<WorkshopUploadEvent>()
            .add_event::<WorkshopDownloadEvent>()
            .add_event::<SubscribeToItemEvent>()
            .add_event::<UnsubscribeFromItemEvent>()
            .add_event::<PublishItemEvent>()
            .add_event::<UpdatePublishedItemEvent>()
            .add_event::<RateItemEvent>()
            .add_event::<ReportItemEvent>()
            .add_systems(Startup, (
                setup_smart_object_registry,
                load_catalog_from_config,
                initialize_steam_workshop,
            ))
            .add_systems(Update, (
                handle_spawn_smart_object_events,
                handle_remove_smart_object_events,
                handle_modify_smart_object_events,
                update_smart_object_durability,
                process_smart_object_interactions,
                handle_workshop_events,
                process_workshop_downloads,
                process_workshop_uploads,
                handle_workshop_subscriptions,
                update_workshop_browser,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// Events for the new system
#[derive(Event)]
pub struct SpawnSmartObjectEvent {
    pub definition_id: String,
    pub position: Vec3,
    pub custom_modifications: Option<std::collections::HashMap<String, String>>,
}

#[derive(Event)]
pub struct RemoveSmartObjectEvent {
    pub entity: Entity,
    pub refund_player: bool,
}

#[derive(Event)]
pub struct ModifySmartObjectEvent {
    pub entity: Entity,
    pub modifications: std::collections::HashMap<String, String>,
}

#[derive(Event)]
pub struct WorkshopUploadEvent {
    pub object_id: String,
    pub workshop_metadata: WorkshopItemMetadata,
}

#[derive(Event)]
pub struct WorkshopDownloadEvent {
    pub workshop_item_id: String,
    pub install_path: String,
}

// Resource to maintain registry of all smart object definitions
#[derive(Resource, Default)]
pub struct SmartObjectRegistry {
    pub catalog: Option<SmartObjectCatalog>,
    pub workshop_items: std::collections::HashMap<String, WorkshopSmartObject>,
    pub active_objects: std::collections::HashMap<Entity, ConfigurableSmartObject>,
}

#[derive(Debug, Clone)]
pub struct WorkshopSmartObject {
    pub base_definition: SmartObjectDefinition,
    pub workshop_metadata: WorkshopItemMetadata,
    pub local_path: String,
    pub is_subscribed: bool,
}

#[derive(Debug, Clone)]
pub struct WorkshopItemMetadata {
    pub title: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub tags: Vec<String>,
    pub thumbnail_path: String,
    pub rating: f32,
    pub download_count: u32,
    pub file_size: u64,
    pub dependencies: Vec<String>,
}

impl SmartObjectRegistry {
    pub fn get_definition(&self, id: &str) -> Option<&SmartObjectDefinition> {
        // First check workshop items
        if let Some(workshop_item) = self.workshop_items.get(id) {
            return Some(&workshop_item.base_definition);
        }
        
        // Then check catalog
        self.catalog.as_ref()?.get_definition(id)
    }
    
    pub fn get_all_available_definitions(&self) -> Vec<&SmartObjectDefinition> {
        let mut definitions = Vec::new();
        
        // Add catalog items
        if let Some(catalog) = &self.catalog {
            definitions.extend(catalog.items.iter());
        }
        
        // Add workshop items
        for workshop_item in self.workshop_items.values() {
            definitions.push(&workshop_item.base_definition);
        }
        
        definitions
    }
    
    pub fn register_workshop_item(&mut self, id: String, item: WorkshopSmartObject) {
        self.workshop_items.insert(id, item);
    }
    
    pub fn get_workshop_items_by_author(&self, author: &str) -> Vec<&WorkshopSmartObject> {
        self.workshop_items.values()
            .filter(|item| item.workshop_metadata.author == author)
            .collect()
    }
    
    pub fn search_workshop_items(&self, query: &str, tags: &[String]) -> Vec<&WorkshopSmartObject> {
        self.workshop_items.values()
            .filter(|item| {
                // Text search in title and description
                let text_match = item.workshop_metadata.title.to_lowercase().contains(&query.to_lowercase()) ||
                    item.workshop_metadata.description.to_lowercase().contains(&query.to_lowercase());
                
                // Tag filtering
                let tag_match = tags.is_empty() || 
                    tags.iter().any(|tag| item.workshop_metadata.tags.contains(tag));
                
                text_match && tag_match
            })
            .collect()
    }
}
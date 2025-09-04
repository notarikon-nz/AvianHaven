use bevy::prelude::*;
// use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::save_load::resources::*;
use crate::catalog::resources::{PlayerInventory, PlacedObjects};
use crate::journal::resources::DiscoveredSpecies;
use crate::environment::resources::{TimeState, WeatherState};
use crate::achievements::{AchievementProgress};
use crate::catalog::components::{PlaceableObject};
use crate::save_load::components::PersistentObject;
use crate::despawn::SafeDespawn;

pub fn save_game_system(
    mut save_events: EventReader<SaveGameEvent>,
    mut save_complete_events: EventWriter<SaveCompleteEvent>,
    save_manager: Res<SaveManager>,
    
    // Resources to save
    player_inventory: Res<PlayerInventory>,
    discovered_species: Res<DiscoveredSpecies>,
    achievement_progress: Res<AchievementProgress>,
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    placed_objects: Res<PlacedObjects>,
    
    // Queries for world objects
    placed_object_query: Query<(&Transform, &PlaceableObject, Option<&PersistentObject>)>,
) {
    for save_event in save_events.read() {
        let result = perform_save(
            &save_manager,
            save_event.slot,
            &player_inventory,
            &discovered_species,
            &achievement_progress,
            &time_state,
            &weather_state,
            &placed_objects,
            &placed_object_query,
        );
        
        let (success, error_message) = match result {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };
        
        save_complete_events.write(SaveCompleteEvent {
            slot: save_event.slot,
            success,
            error_message: error_message.clone(),
        });
        
        if success {
            info!("Game saved successfully to slot {}", save_event.slot);
        } else {
            error!("Failed to save game to slot {}: {:?}", save_event.slot, error_message);
        }
    }
}

pub fn load_game_system(
    mut commands: Commands,
    mut load_events: EventReader<LoadGameEvent>,
    mut load_complete_events: EventWriter<LoadCompleteEvent>,
    save_manager: Res<SaveManager>,
    asset_server: Res<AssetServer>,
    
    // Resources to update
    mut player_inventory: ResMut<PlayerInventory>,
    mut discovered_species: ResMut<DiscoveredSpecies>,
    mut achievement_progress: ResMut<AchievementProgress>,
    mut time_state: ResMut<TimeState>,
    mut weather_state: ResMut<WeatherState>,
    mut placed_objects: ResMut<PlacedObjects>,
    
    // Clear existing placed objects
    placed_object_query: Query<Entity, With<PlaceableObject>>,
) {
    for load_event in load_events.read() {
        let result = perform_load(
            &mut commands,
            &save_manager,
            load_event.slot,
            &asset_server,
            &mut player_inventory,
            &mut discovered_species,
            &mut achievement_progress,
            &mut time_state,
            &mut weather_state,
            &mut placed_objects,
            &placed_object_query,
        );
        
        let (success, error_message) = match result {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };
        
        load_complete_events.write(LoadCompleteEvent {
            slot: load_event.slot,
            success,
            error_message: error_message.clone(),
        });
        
        if success {
            info!("Game loaded successfully from slot {}", load_event.slot);
        } else {
            error!("Failed to load game from slot {}: {:?}", load_event.slot, error_message);
        }
    }
}

pub fn auto_save_system(
    mut save_manager: ResMut<SaveManager>,
    mut save_events: EventWriter<SaveGameEvent>,
    time: Res<Time>,
) {
    if save_manager.auto_save_enabled {
        save_manager.auto_save_timer.tick(time.delta());
        
        if save_manager.auto_save_timer.just_finished() {
            if let Some(slot) = save_manager.current_save_slot {
                save_events.write(SaveGameEvent {
                    slot,
                    save_name: Some("Auto Save".to_string()),
                });
                info!("Auto-saving to slot {}", slot);
            }
        }
    }
}

fn perform_save(
    save_manager: &SaveManager,
    slot: u32,
    player_inventory: &PlayerInventory,
    discovered_species: &DiscoveredSpecies,
    achievement_progress: &AchievementProgress,
    time_state: &TimeState,
    weather_state: &WeatherState,
    placed_objects: &PlacedObjects,
    placed_object_query: &Query<(&Transform, &PlaceableObject, Option<&PersistentObject>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    save_manager.ensure_save_directory()?;
    
    // Collect placed objects data
    let mut placed_objects_data = Vec::new();
    for (transform, placeable_object, persistent_object) in placed_object_query.iter() {
        let save_id = if let Some(persistent) = persistent_object {
            persistent.save_id.clone()
        } else {
            format!("{}_{}", placeable_object.item_type.name(), 
                    placed_objects_data.len())
        };
        
        placed_objects_data.push(PlacedObjectSaveData {
            item_type: placeable_object.item_type.clone(),
            position: [
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ],
            save_id,
        });
    }
    
    // Create save data structure
    let save_data = GameSaveData {
        version: env!("CARGO_PKG_VERSION").to_string(),
        save_timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
            
        player_inventory: InventorySaveData {
            currency: player_inventory.currency,
            owned_items: player_inventory.owned_items.clone(),
        },
        
        discovered_species: discovered_species.0.clone(),
        
        achievements: achievement_progress.unlocked.clone(),
        
        achievement_progress: AchievementProgressSaveData {
            photos_taken: achievement_progress.photos_taken,
            currency_earned: 0, // We'll track this separately
            species_discovered: achievement_progress.species_discovered,
            feeders_upgraded: 0, // We'll track this separately
        },
        
        environment_state: EnvironmentSaveData {
            current_hour: time_state.hour,
            day_of_year: time_state.day_of_year,
            current_weather: weather_state.current_weather,
            temperature: weather_state.temperature,
        },
        
        placed_objects: placed_objects_data,
        
        total_photos_taken: achievement_progress.photos_taken,
        total_playtime_seconds: 0.0, // TODO: Track playtime
        birds_observed: discovered_species.0.len() as u32,
    };
    
    // Serialize and write to file
    let serialized = ron::to_string(&save_data)?;
    let save_path = save_manager.get_save_path(slot);
    fs::write(save_path, serialized)?;
    
    Ok(())
}

fn perform_load(
    commands: &mut Commands,
    save_manager: &SaveManager,
    slot: u32,
    asset_server: &AssetServer,
    player_inventory: &mut PlayerInventory,
    discovered_species: &mut DiscoveredSpecies,
    achievement_progress: &mut AchievementProgress,
    time_state: &mut TimeState,
    weather_state: &mut WeatherState,
    placed_objects: &mut PlacedObjects,
    placed_object_query: &Query<Entity, With<PlaceableObject>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let save_path = save_manager.get_save_path(slot);
    
    if !save_path.exists() {
        return Err(format!("Save file for slot {} does not exist", slot).into());
    }
    
    // Read and deserialize save file
    let file_content = fs::read_to_string(save_path)?;
    let save_data: GameSaveData = ron::from_str(&file_content)?;
    
    // Clear existing placed objects
    for entity in placed_object_query.iter() {
        commands.entity(entity).safe_despawn();
    }
    placed_objects.objects.clear();
    
    // Restore player inventory
    player_inventory.currency = save_data.player_inventory.currency;
    player_inventory.owned_items = save_data.player_inventory.owned_items;
    
    // Restore discovered species
    discovered_species.0 = save_data.discovered_species;
    
    // Restore achievements
    achievement_progress.unlocked = save_data.achievements;
    achievement_progress.photos_taken = save_data.achievement_progress.photos_taken;
    achievement_progress.species_discovered = save_data.achievement_progress.species_discovered;
    
    // Restore environment state
    time_state.hour = save_data.environment_state.current_hour;
    time_state.day_of_year = save_data.environment_state.day_of_year;
    weather_state.current_weather = save_data.environment_state.current_weather;
    weather_state.temperature = save_data.environment_state.temperature;
    
    // Restore placed objects
    use crate::bird_ai::components::{SmartObject, ProvidesUtility};
    use bevy_rapier2d::prelude::*;
    
    let placed_objects_len = save_data.placed_objects.len();
    
    for object_data in save_data.placed_objects {
        let position = Vec3::new(
            object_data.position[0],
            object_data.position[1], 
            object_data.position[2],
        );
        
        let item_size = object_data.item_type.physical_size();
        let actions = object_data.item_type.provides_actions();
        
        // Recreate the placed object with all components
        let mut entity_commands = commands.spawn((
            Sprite {
                image: asset_server.load(&format!("objects/{}.png", 
                    crate::catalog::systems::object_filename(&object_data.item_type))),
                ..default()
            },
            Transform::from_translation(position),
            RigidBody::Fixed,
            Collider::cuboid(item_size.x / 2.0, item_size.y / 2.0),
            Sensor,
            crate::catalog::components::PlaceableObject {
                item_type: object_data.item_type.clone(),
                placement_cost: object_data.item_type.price(),
            },
            SmartObject,
            PersistentObject {
                save_id: object_data.save_id,
            },
        ));
        
        // Add utility components
        let base_utility = object_data.item_type.base_utility();
        let interaction_range = object_data.item_type.interaction_range();
        
        if let Some(primary_action) = actions.first() {
            entity_commands.insert(ProvidesUtility {
                action: *primary_action,
                base_utility,
                range: interaction_range,
            });
        }
        
        let entity = entity_commands.id();
        
        // Add secondary utility providers for multi-action items
        for action in actions.iter().skip(1) {
            commands.spawn((
                Transform::from_translation(position),
                SmartObject,
                ProvidesUtility {
                    action: *action,
                    base_utility: base_utility * 0.8,
                    range: interaction_range,
                },
            ));
        }
        
        placed_objects.objects.insert(entity, object_data.item_type);
    }
    
    info!("Loaded {} placed objects", placed_objects_len);
    Ok(())
}
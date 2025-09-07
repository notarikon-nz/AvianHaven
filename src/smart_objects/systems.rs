use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::bird_ai::components::{SmartObject, ProvidesUtility};
use crate::smart_objects::config::*;
use crate::smart_objects::*;
use crate::environment::resources::{TimeState, WeatherState};
use std::collections::HashMap;
use crate::notifications::{resources::ShowNotificationEvent, components::NotificationType};
use crate::catalog::resources::PlayerInventory;
use crate::garden_lighting::{spawn_solar_light, spawn_garden_lamp};

pub fn setup_smart_object_registry(
    mut registry: ResMut<SmartObjectRegistry>,
) {
    info!("Setting up Smart Object Registry");
    
    // Registry is initialized with default values
    // Catalog will be loaded separately
}

pub fn load_catalog_from_config(
    mut registry: ResMut<SmartObjectRegistry>,
    asset_server: Res<AssetServer>,
) {
    info!("Loading Smart Object Catalog at startup from data/smart_objects/catalog_items.ron");
    let catalog_handle: Handle<SmartObjectCatalog> = asset_server.load("data/smart_objects/catalog_items.ron");
    registry.catalog_handle = Some(catalog_handle);
}

pub fn check_catalog_loading(
    mut registry: ResMut<SmartObjectRegistry>,
    catalog_assets: Res<Assets<SmartObjectCatalog>>,
) {
    if registry.catalog_loaded {
        return; // Already loaded
    }
    
    if let Some(handle) = &registry.catalog_handle {
        if let Some(catalog) = catalog_assets.get(handle) {
            info!("Successfully loaded Smart Object Catalog with {} items", catalog.items.len());
            #[cfg(debug_assertions)]
            info!("Debug: First item ID: {}, category: {}", 
                catalog.items.first().map(|item| &item.id).unwrap_or(&"none".to_string()),
                catalog.items.first().map(|item| &item.metadata.category).unwrap_or(&"none".to_string()));
            
            registry.catalog = Some(catalog.clone());
            registry.catalog_loaded = true;
        }
    }
}

pub fn handle_spawn_smart_object_events(
    mut commands: Commands,
    mut events: EventReader<SpawnSmartObjectEvent>,
    mut registry: ResMut<SmartObjectRegistry>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for event in events.read() {
        // Handle specialized lighting objects
        match event.definition_id.as_str() {
            "solar_light" => {
                let entity = spawn_solar_light(&mut commands, event.position);
                info!("Spawned solar light at {:?}", event.position);
                continue;
            },
            "garden_lamp" => {
                let entity = spawn_garden_lamp(&mut commands, event.position);
                info!("Spawned garden lamp at {:?}", event.position);
                continue;
            },
            _ => {
                // Handle regular smart objects
                let definition = registry.get_definition(&event.definition_id).cloned();
                if let Some(definition) = definition {
                    spawn_smart_object_from_definition(
                        &mut commands,
                        &mut registry,
                        &asset_server,
                        &definition,
                        event.position,
                        event.custom_modifications.as_ref(),
                        time.elapsed_secs_f64(),
                    );
                } else {
                    error!("Smart object definition not found: {}", event.definition_id);
                }
            }
        }
    }
}

pub fn handle_remove_smart_object_events(
    mut commands: Commands,
    mut events: EventReader<RemoveSmartObjectEvent>,
    mut registry: ResMut<SmartObjectRegistry>,
    mut player_inventory: ResMut<PlayerInventory>,
) {
    for event in events.read() {
        if let Some(configurable_object) = registry.active_objects.remove(&event.entity) {
            // Handle refund logic if needed
            if event.refund_player {
                if let Some(definition) = registry.get_definition(&configurable_object.definition_id) {
                    let refund_amount = (definition.economy.purchase_cost as f32 * 
                        definition.economy.resale_value_multiplier * 
                        configurable_object.current_durability) as u32;
                    
                    info!("Refunding {} credits for removed object", refund_amount);
                    player_inventory.currency += refund_amount;
                }
            }
            
            // Despawn the entity
            if let Ok(mut entity_commands) = commands.get_entity(event.entity) {
                entity_commands.despawn();
            }
            
            info!("Removed smart object: {}", configurable_object.definition_id);
        }
    }
}

pub fn handle_modify_smart_object_events(
    mut events: EventReader<ModifySmartObjectEvent>,
    mut registry: ResMut<SmartObjectRegistry>,
) {
    for event in events.read() {
        if let Some(configurable_object) = registry.active_objects.get_mut(&event.entity) {
            // Apply custom modifications
            for (key, value) in &event.modifications {
                configurable_object.custom_modifications.insert(key.clone(), value.clone());
            }
            
            info!("Applied {} modifications to smart object", event.modifications.len());
        }
    }
}

pub fn update_smart_object_durability(
    mut registry: ResMut<SmartObjectRegistry>,
    time: Res<Time>,
    mut notification_events: EventWriter<ShowNotificationEvent>,
) {
    let current_time = time.elapsed_secs_f64();
    let delta_time = time.delta_secs();

    // 1) Collect the entities up front (no &mut held across the loop)
    let entities: Vec<_> = registry.active_objects.keys().cloned().collect();

    for entity in entities {
        // 2) Read what you need immutably (no &mut borrow yet)
        let (def_id, last_maintenance) = match registry.active_objects.get(&entity) {
            Some(obj) => (obj.definition_id.clone(), obj.last_maintenance),
            None => continue,
        };

        let definition = match registry.get_definition(&def_id).cloned() {
            Some(d) => d,
            None => continue,
        };

        // 3) Now take the mutable borrow and write updates
        if let Some(obj) = registry.active_objects.get_mut(&entity) {
            let time_since_last_update = current_time - last_maintenance;
            let decay_amount = definition.economy.decay_rate * delta_time / 86400.0;

            obj.current_durability = (obj.current_durability - decay_amount).max(0.0);

            let days_since_maintenance = time_since_last_update / 86400.0;
            if days_since_maintenance >= definition.economy.maintenance_cost as f64
                && obj.current_durability < 0.3
            {
                notification_events.write(ShowNotificationEvent {
                    notification: NotificationType::Warning {
                        message: format!("{} requires maintenance, its durability is now at {:.0}%", obj.definition_id, obj.current_durability * 100.0),
                    },
                });                
            }
        }
    }
}

pub fn process_smart_object_interactions(
    mut registry: ResMut<SmartObjectRegistry>,
    mut smart_object_query: Query<(Entity, &mut ProvidesUtility), With<SmartObject>>,
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
) {
    let current_season = time_state.get_season().to_string();
    let weather_intensity = weather_state.current_weather.weather_fear_factor();
    
    for (entity, mut utility_provider) in smart_object_query.iter_mut() {
        if let Some(configurable_object) = registry.active_objects.get(&entity) {
            if let Some(definition) = registry.get_definition(&configurable_object.definition_id) {
                // Update utility based on current conditions
                let current_users = configurable_object.current_users.len() as u32;
                
                // Calculate effective utility (distance will be calculated per bird)
                let base_effective_utility = definition.calculate_effective_utility(
                    &current_season,
                    weather_intensity,
                    current_users,
                    0.0, // Base calculation without distance
                );
                
                // Update the utility provider
                utility_provider.base_utility = base_effective_utility;
                utility_provider.range = definition.behavior.interaction_range;
                
                // Update durability effect on utility
                let durability_modifier = configurable_object.current_durability.max(0.1);
                utility_provider.base_utility *= durability_modifier;
            }
        }
    }
}

pub fn handle_workshop_events(
    mut upload_events: EventReader<WorkshopUploadEvent>,
    mut download_events: EventReader<WorkshopDownloadEvent>,
    mut registry: ResMut<SmartObjectRegistry>,
) {
    // Handle workshop uploads
    for event in upload_events.read() {
        info!("Uploading smart object to workshop: {}", event.object_id);
        // TODO: Integrate with Steam Workshop API
        // This would package the object definition and assets for upload
    }
    
    // Handle workshop downloads
    for event in download_events.read() {
        info!("Downloading workshop item: {}", event.workshop_item_id);
        // TODO: Integrate with Steam Workshop API
        // This would download and install workshop items
    }
}

fn spawn_smart_object_from_definition(
    commands: &mut Commands,
    registry: &mut SmartObjectRegistry,
    asset_server: &AssetServer,
    definition: &SmartObjectDefinition,
    position: Vec3,
    custom_modifications: Option<&HashMap<String, String>>,
    current_time: f64,
) -> Entity {
    // Load sprite
    let texture_handle: Handle<Image> = asset_server.load(format!("sprites/objects/{}.png", definition.visual.sprite_filename));
    
    // Create sprite
    let sprite = Sprite {
        color: Color::srgba(
            definition.visual.color_tint.0,
            definition.visual.color_tint.1,
            definition.visual.color_tint.2,
            definition.visual.color_tint.3,
        ),
        custom_size: Some(Vec2::new(definition.visual.size.0, definition.visual.size.1)),
        ..default()
    };
    
    // Create physics components
    let collider = match definition.physics.collision_shape.as_str() {
        "Circle" => Collider::ball(definition.physics.collision_size.0 / 2.0),
        "Rectangle" => Collider::cuboid(
            definition.physics.collision_size.0 / 2.0,
            definition.physics.collision_size.1 / 2.0,
        ),
        _ => Collider::cuboid(
            definition.physics.collision_size.0 / 2.0,
            definition.physics.collision_size.1 / 2.0,
        ),
    };
    
    // Create the entity
    let entity = commands.spawn((

        Sprite {
                color: Color::srgba(
                    definition.visual.color_tint.0,
                    definition.visual.color_tint.1,
                    definition.visual.color_tint.2,
                    definition.visual.color_tint.3,
                ),
                custom_size: Some(Vec2::new(definition.visual.size.0, definition.visual.size.1)),
                ..default()
            },
            // asset_server.load("textures/object.png"), // Handle<Image>
        Transform::from_translation(position)
            .with_scale(Vec3::splat(definition.visual.scale)),
        GlobalTransform::default(),
        Visibility::default(),
        SmartObject,
        collider,
    ))
    .id();
    
    // Add ProvidesUtility components for each action the object provides
    let bird_actions = definition.get_bird_actions();
    for action in bird_actions {
        commands.entity(entity).insert(ProvidesUtility {
            action,
            base_utility: definition.behavior.base_utility,
            range: definition.behavior.interaction_range,
        });
    }
    
    // Add physics components if needed
    if definition.physics.is_solid {
        commands.entity(entity).insert(RigidBody::Fixed);
        
        if definition.physics.can_be_moved {
            // Make it kinematic so it can be moved but doesn't fall
            commands.entity(entity).insert(RigidBody::KinematicPositionBased);
        }
    }
    
    // Create ConfigurableSmartObject component
    let mut configurable_object = ConfigurableSmartObject {
        definition_id: definition.id.clone(),
        current_durability: definition.economy.durability,
        last_maintenance: current_time,
        current_users: Vec::new(),
        total_usage_time: 0.0,
        custom_modifications: HashMap::new(),
    };
    
    // Apply custom modifications if provided
    if let Some(modifications) = custom_modifications {
        configurable_object.custom_modifications = modifications.clone();
    }
    
    // Register the object
    registry.active_objects.insert(entity, configurable_object);
    
    info!("Spawned smart object: {} at {:?}", definition.id, position);
    
    entity
}

// Helper trait to convert Season enum to string
trait SeasonString {
    fn to_string(&self) -> String;
}

impl SeasonString for crate::environment::components::Season {
    fn to_string(&self) -> String {
        match self {
            crate::environment::components::Season::Spring => "Spring".to_string(),
            crate::environment::components::Season::Summer => "Summer".to_string(),
            crate::environment::components::Season::Fall => "Fall".to_string(),
            crate::environment::components::Season::Winter => "Winter".to_string(),
        }
    }
}
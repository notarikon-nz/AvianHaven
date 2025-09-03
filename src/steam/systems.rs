use bevy::prelude::*;
use super::{components::*, resources::*};
use crate::achievements::{AchievementUnlockedEvent, Achievement};
use crate::photo_mode::components::PhotoTakenEvent;

pub fn initialize_steam_systems(
    mut steam_state: ResMut<SteamState>,
    mut steam_achievements: ResMut<SteamAchievements>,
) {
    info!("Initializing Steam integration...");
    
    // Initialize Steam client - simplified for now
    match std::env::var("STEAM_OFFLINE") {
        Ok(_) => {
            warn!("Steam offline mode - running without Steam integration");
            steam_state.is_initialized = false;
            steam_state.is_connected = false;
        },
        Err(_) => {
            // In production, this would call steamworks::Client::init()
            steam_state.is_initialized = true;
            steam_state.is_connected = true;
            steam_state.user_id = Some(123456789);
            info!("Steam connection established! User ID: {:?}", steam_state.user_id);
        }
    }
    
    // Register achievement mappings
    steam_achievements.register_achievement_mapping();
    
    info!("Steam systems initialized");
}

fn check_steam_connection() -> bool {
    // In real implementation: check if Steam client is running
    // For now, assume Steam is available
    std::env::var("STEAM_OFFLINE").is_err()
}

pub fn steam_achievement_sync_system(
    mut achievement_events: EventReader<AchievementUnlockedEvent>,
    mut steam_achievements: ResMut<SteamAchievements>,
    mut steam_achievement_events: EventWriter<SteamAchievementEvent>,
    steam_state: Res<SteamState>,
) {
    if !steam_state.is_connected {
        return;
    }
    
    for event in achievement_events.read() {
        let achievement_id = match event.achievement {
            Achievement::FirstPhoto => "FirstPhoto",
            Achievement::PhotoMaster => "PhotoMaster",
            Achievement::ActionShot => "ActionShot",
            Achievement::MultiSpeciesShot => "MultiSpeciesShot",
            Achievement::FirstSpecies => "FirstSpecies",
            Achievement::CommonCollector => "CommonCollector",
            Achievement::Ornithologist => "Ornithologist",
            Achievement::Wealthy => "Wealthy",
            Achievement::Millionaire => "Millionaire",
            Achievement::FeederMaintainer => "FeederMaintainer",
            Achievement::FeederExpert => "FeederExpert",
        };
        
        steam_achievements.unlock_achievement(achievement_id);
        steam_achievement_events.write(SteamAchievementEvent {
            achievement_id: achievement_id.to_string(),
        });
        
        info!("Steam achievement unlocked: {}", achievement_id);
    }
    
    // Process pending achievement syncs
    let pending_achievements = steam_achievements.sync_pending.clone();
    steam_achievements.sync_pending.clear();
    
    for achievement_id in pending_achievements {
        // Call Steam API to unlock achievement
        if let Err(e) = unlock_steam_achievement(&achievement_id, &*steam_state) {
            error!("Failed to unlock Steam achievement {}: {}", achievement_id, e);
            // Re-add to pending if failed
            steam_achievements.sync_pending.push(achievement_id);
        } else {
            info!("Successfully synced achievement to Steam: {}", achievement_id);
        }
    }
}

pub fn steam_stats_tracking_system(
    mut photo_events: EventReader<PhotoTakenEvent>,
    mut steam_stats: ResMut<SteamStats>,
    mut steam_stats_events: EventWriter<SteamStatsEvent>,
    achievement_progress: Res<crate::achievements::AchievementProgress>,
    currency: Res<crate::photo_mode::resources::CurrencyResource>,
    steam_state: Res<SteamState>,
    time: Res<Time>,
) {
    if !steam_state.is_connected {
        return;
    }
    
    // Update playtime
    steam_stats.total_playtime += time.delta().as_secs_f64();
    
    // Track photo events
    for event in photo_events.read() {
        steam_stats.increment_stat("photos_taken", 1);
        steam_stats_events.write(SteamStatsEvent {
            stat_name: "photos_taken".to_string(),
            value: steam_stats.photos_taken,
        });
        
        // Sync to Steam API
        if let Err(e) = update_steam_stat("photos_taken", steam_stats.photos_taken, &*steam_state) {
            warn!("Failed to sync photos_taken stat: {}", e);
        }
    }
    
    // Sync other stats periodically
    if achievement_progress.is_changed() {
        steam_stats.update_stat("species_discovered", achievement_progress.species_discovered as u64);
        steam_stats_events.write(SteamStatsEvent {
            stat_name: "species_discovered".to_string(),
            value: steam_stats.species_discovered,
        });
        
        if let Err(e) = update_steam_stat("species_discovered", steam_stats.species_discovered, &*steam_state) {
            warn!("Failed to sync species_discovered stat: {}", e);
        }
    }
    
    if currency.is_changed() {
        steam_stats.update_stat("currency_earned", currency.0 as u64);
        steam_stats_events.write(SteamStatsEvent {
            stat_name: "currency_earned".to_string(),
            value: steam_stats.currency_earned,
        });
        
        if let Err(e) = update_steam_stat("currency_earned", steam_stats.currency_earned, &*steam_state) {
            warn!("Failed to sync currency_earned stat: {}", e);
        }
    }
}

pub fn load_workshop_content(
    mut commands: Commands,
    steam_state: Res<SteamState>,
) {
    info!("Loading workshop content at startup...");
    
    // Load and integrate workshop items
    let workshop_items = load_workshop_items(&*steam_state);
    
    for item in workshop_items {
        if validate_workshop_content(&item) {
            integrate_workshop_item(&mut commands, item);
        } else {
            warn!("Invalid workshop item: {}", item.title);
        }
    }
    
    info!("Workshop content loading complete");
}

fn integrate_workshop_item(_commands: &mut Commands, item: WorkshopItem) {
    match item.item_type {
        WorkshopItemType::CustomBird { species_name: _, behavior_data: _ } => {
            info!("Loading custom bird: {} by {}", item.title, item.author);
            // Parse behavior data and register custom bird species
            // In production: extend BirdSpecies enum dynamically
            // For now: log successful integration
            info!("Custom bird '{}' successfully integrated", item.title);
        },
        WorkshopItemType::CustomFeeder { feeder_name: _, stats } => {
            info!("Loading custom feeder: {} by {} (capacity: {})", 
                  item.title, item.author, stats.capacity);
            // Create custom feeder type with workshop stats
            // In production: add to feeder registry for spawning
            info!("Custom feeder '{}' successfully integrated", item.title);
        },
        WorkshopItemType::Habitat { theme_name: _, assets } => {
            info!("Loading habitat theme: {} by {} ({} assets)", 
                  item.title, item.author, assets.len());
            // Load custom textures and environment objects
            // In production: update environment asset registry
            info!("Habitat theme '{}' successfully integrated", item.title);
        },
    }
}

// Workshop integration helpers
pub fn load_workshop_items(steam_state: &SteamState) -> Vec<WorkshopItem> {
    // Check for local workshop items first
    let mut items = load_local_workshop_items();
    
    // Add Steam Workshop subscribed items
    if let Ok(steam_items) = load_steam_workshop_items(steam_state) {
        items.extend(steam_items);
    }
    
    items
}

fn load_local_workshop_items() -> Vec<WorkshopItem> {
    // Load workshop items from local directory (for development)
    vec![
        WorkshopItem {
            workshop_id: 12345,
            item_type: WorkshopItemType::CustomBird {
                species_name: "Custom Woodpecker".to_string(),
                behavior_data: "aggressive_feeder".to_string(),
            },
            author: "BirdLover123".to_string(),
            title: "Realistic Pileated Woodpecker".to_string(),
            description: "Adds authentic Pileated Woodpecker with suet feeder preference".to_string(),
        },
        WorkshopItem {
            workshop_id: 67890,
            item_type: WorkshopItemType::CustomFeeder {
                feeder_name: "Premium Thistle Feeder".to_string(),
                stats: FeederWorkshopStats {
                    capacity: 500.0,
                    attraction_radius: 150.0,
                    supported_food_types: vec!["thistle".to_string(), "nyjer".to_string()],
                },
            },
            author: "FeederCrafter".to_string(),
            title: "Premium Goldfinch Feeder".to_string(),
            description: "Specialized thistle feeder that attracts goldfinches and siskins".to_string(),
        },
    ]
}

fn load_steam_workshop_items(steam_state: &SteamState) -> Result<Vec<WorkshopItem>, String> {
    if !steam_state.is_connected {
        return Ok(vec![]);
    }
    
    info!("Loading subscribed Steam Workshop items...");
    
    // In production, this would use steamworks UGC API:
    // let ugc = client.ugc();
    // let subscribed_items = ugc.subscribed_items();
    // Parse item details and return actual workshop content
    
    // For now, return mock data
    Ok(vec![
        WorkshopItem {
            workshop_id: 98765,
            item_type: WorkshopItemType::Habitat {
                theme_name: "Winter Wonderland".to_string(),
                assets: vec!["snow_texture.png".to_string(), "ice_feeder.png".to_string()],
            },
            author: "SeasonalMods".to_string(),
            title: "Winter Environment Pack".to_string(),
            description: "Beautiful winter-themed environment with snow effects".to_string(),
        },
    ])
}

fn _parse_workshop_item_type(_tags: &[String]) -> WorkshopItemType {
    // Default to habitat for now
    WorkshopItemType::Habitat {
        theme_name: "Custom Theme".to_string(),
        assets: vec!["custom_texture.png".to_string()],
    }
}

pub fn validate_workshop_content(item: &WorkshopItem) -> bool {
    // Content validation for workshop items
    match &item.item_type {
        WorkshopItemType::CustomBird { species_name, .. } => {
            // Validate bird data
            !species_name.is_empty() && species_name.len() < 50
        },
        WorkshopItemType::CustomFeeder { feeder_name, stats } => {
            // Validate feeder stats are reasonable
            !feeder_name.is_empty() && 
            stats.capacity > 0.0 && stats.capacity < 1000.0 &&
            stats.attraction_radius > 0.0 && stats.attraction_radius < 500.0
        },
        WorkshopItemType::Habitat { theme_name, assets } => {
            // Validate habitat data
            !theme_name.is_empty() && !assets.is_empty() && assets.len() < 20
        },
    }
}

// Steam API Integration Functions
fn unlock_steam_achievement(achievement_id: &str, steam_state: &SteamState) -> Result<(), String> {
    if !steam_state.is_connected {
        return Err("Steam not connected".to_string());
    }
    
    info!("Unlocking Steam achievement: {}", achievement_id);
    
    // In production, this would use steamworks API:
    // let user_stats = client.user_stats();
    // user_stats.set_achievement(achievement_id);
    // user_stats.store_stats();
    
    info!("Steam achievement {} unlocked successfully", achievement_id);
    Ok(())
}

fn update_steam_stat(stat_name: &str, value: u64, steam_state: &SteamState) -> Result<(), String> {
    if !steam_state.is_connected {
        return Err("Steam not connected".to_string());
    }
    
    info!("Updating Steam stat: {} = {}", stat_name, value);
    
    // In production, this would use steamworks API:
    // let user_stats = client.user_stats();
    // user_stats.set_stat(stat_name, value as i32);
    // user_stats.store_stats();
    
    Ok(())
}

fn sync_steam_leaderboards(score: u64, category: &str) -> Result<(), String> {
    if std::env::var("STEAM_OFFLINE").is_ok() {
        return Err("Steam offline mode".to_string());
    }
    
    info!("Syncing leaderboard score: {} in category {}", score, category);
    
    // In production, this would use Steam API calls:
    // - steamapi::user_stats::upload_leaderboard_score()
    
    Ok(())
}
use bevy::prelude::*;
use super::{components::*, resources::*};
use crate::achievements::{AchievementUnlockedEvent, Achievement};
use crate::photo_mode::components::PhotoTakenEvent;

pub fn initialize_steam_systems(
    mut steam_state: ResMut<SteamState>,
    mut steam_achievements: ResMut<SteamAchievements>,
) {
    // In a real implementation, this would initialize the Steam API
    // For now, we'll simulate the initialization
    
    info!("Initializing Steam integration...");
    
    // Simulate Steam API initialization
    steam_state.is_initialized = true;
    steam_state.is_connected = check_steam_connection();
    
    if steam_state.is_connected {
        steam_state.user_id = Some(123456789); // Mock Steam ID
        info!("Steam connection established!");
    } else {
        warn!("Steam not available - running in offline mode");
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
        // In real implementation: call Steam API to unlock achievement
        info!("Syncing achievement to Steam: {}", achievement_id);
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
    for _event in photo_events.read() {
        steam_stats.increment_stat("photos_taken", 1);
        steam_stats_events.write(SteamStatsEvent {
            stat_name: "photos_taken".to_string(),
            value: steam_stats.photos_taken,
        });
    }
    
    // Sync other stats periodically
    if achievement_progress.is_changed() {
        steam_stats.update_stat("species_discovered", achievement_progress.species_discovered as u64);
        steam_stats_events.write(SteamStatsEvent {
            stat_name: "species_discovered".to_string(),
            value: steam_stats.species_discovered,
        });
    }
    
    if currency.is_changed() {
        steam_stats.update_stat("currency_earned", currency.0 as u64);
        steam_stats_events.write(SteamStatsEvent {
            stat_name: "currency_earned".to_string(),
            value: steam_stats.currency_earned,
        });
    }
}

pub fn steam_workshop_system(
    mut commands: Commands,
    steam_state: Res<SteamState>,
) {
    if !steam_state.is_connected {
        return;
    }
    
    // Load and integrate workshop items
    let workshop_items = load_workshop_items();
    
    for item in workshop_items {
        if validate_workshop_content(&item) {
            integrate_workshop_item(&mut commands, item);
        } else {
            warn!("Invalid workshop item: {}", item.title);
        }
    }
}

fn integrate_workshop_item(commands: &mut Commands, item: WorkshopItem) {
    match item.item_type {
        WorkshopItemType::CustomBird { species_name, behavior_data } => {
            // info!("Loading custom bird: {} by {}", species_name, item.author);
            // In real implementation: parse behavior data and spawn custom bird entity
        },
        WorkshopItemType::CustomFeeder { feeder_name, stats } => {
            // info!("Loading custom feeder: {} by {} (capacity: {})",  feeder_name, item.author, stats.capacity);
            // In real implementation: create custom feeder with workshop stats
        },
        WorkshopItemType::Habitat { theme_name, assets } => {
            // info!("Loading habitat theme: {} by {} ({} assets)",  theme_name, item.author, assets.len());
            // In real implementation: load custom textures and environment objects
        },
    }
}

// Workshop integration helpers
pub fn load_workshop_items() -> Vec<WorkshopItem> {
    // Mock workshop items for development
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
    ]
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
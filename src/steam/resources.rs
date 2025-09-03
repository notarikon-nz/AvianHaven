use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Steam client wrapper that's thread-safe
pub type SteamClientWrapper = Arc<Mutex<Option<steamworks::SingleClient>>>;

#[derive(Resource)]
pub struct SteamState {
    pub is_initialized: bool,
    pub is_connected: bool,
    pub app_id: u32,
    pub user_id: Option<u64>,
    pub client: SteamClientWrapper,
}

impl Default for SteamState {
    fn default() -> Self {
        Self {
            is_initialized: false,
            is_connected: false,
            app_id: 12345678, // Placeholder - will be assigned by Steam
            user_id: None,
            client: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Resource, Default)]
pub struct SteamAchievements {
    pub local_achievements: HashMap<String, bool>,
    pub steam_achievements: HashMap<String, bool>,
    pub sync_pending: Vec<String>,
}

impl SteamAchievements {
    pub fn register_achievement_mapping(&mut self) {
        // Map our achievements to Steam achievement IDs
        let mappings = vec![
            ("FirstPhoto", "FIRST_SNAPSHOT"),
            ("PhotoMaster", "PHOTO_MASTER"),
            ("ActionShot", "ACTION_PHOTOGRAPHER"),
            ("MultiSpeciesShot", "FLOCK_PHOTOGRAPHER"),
            ("FirstSpecies", "FIRST_DISCOVERY"),
            ("CommonCollector", "COMMON_COLLECTOR"),
            ("Ornithologist", "ORNITHOLOGIST"),
            ("Wealthy", "WEALTHY_BIRDER"),
            ("Millionaire", "MILLIONAIRE_BIRDER"),
            ("FeederMaintainer", "FEEDER_MAINTAINER"),
            ("FeederExpert", "FEEDER_EXPERT"),
        ];
        
        for (local_id, _steam_id) in mappings {
            self.local_achievements.insert(local_id.to_string(), false);
            self.steam_achievements.insert(local_id.to_string(), false);
        }
    }
    
    pub fn unlock_achievement(&mut self, achievement_id: &str) {
        if let Some(unlocked) = self.local_achievements.get_mut(achievement_id) {
            if !*unlocked {
                *unlocked = true;
                self.sync_pending.push(achievement_id.to_string());
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct SteamStats {
    pub photos_taken: u64,
    pub species_discovered: u64,
    pub total_playtime: f64,
    pub feeders_upgraded: u64,
    pub currency_earned: u64,
}

impl SteamStats {
    pub fn update_stat(&mut self, stat_name: &str, value: u64) {
        match stat_name {
            "photos_taken" => self.photos_taken = value,
            "species_discovered" => self.species_discovered = value,
            "feeders_upgraded" => self.feeders_upgraded = value,
            "currency_earned" => self.currency_earned = value,
            _ => warn!("Unknown stat: {}", stat_name),
        }
    }
    
    pub fn increment_stat(&mut self, stat_name: &str, amount: u64) {
        match stat_name {
            "photos_taken" => self.photos_taken += amount,
            "species_discovered" => self.species_discovered += amount,
            "feeders_upgraded" => self.feeders_upgraded += amount,
            "currency_earned" => self.currency_earned += amount,
            _ => warn!("Unknown stat: {}", stat_name),
        }
    }
}

#[derive(Event)]
pub struct SteamAchievementEvent {
    pub achievement_id: String,
}

#[derive(Event)]
pub struct SteamStatsEvent {
    pub stat_name: String,
    pub value: u64,
}
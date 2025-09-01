use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::bird::BirdSpecies;
use crate::catalog::components::PlaceableItemType;
use crate::environment::components::{Weather, Season};
use crate::achievements::Achievement;

#[derive(Resource)]
pub struct SaveManager {
    pub save_directory: PathBuf,
    pub current_save_slot: Option<u32>,
    pub auto_save_enabled: bool,
    pub auto_save_timer: Timer,
}

impl Default for SaveManager {
    fn default() -> Self {
        let save_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("AvianHaven")
            .join("saves");
            
        Self {
            save_directory: save_dir,
            current_save_slot: None,
            auto_save_enabled: true,
            auto_save_timer: Timer::from_seconds(300.0, TimerMode::Repeating), // Auto-save every 5 minutes
        }
    }
}

impl SaveManager {
    pub fn ensure_save_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.save_directory)?;
        Ok(())
    }
    
    pub fn get_save_path(&self, slot: u32) -> PathBuf {
        self.save_directory.join(format!("save_{}.ron", slot))
    }
    
    pub fn list_save_files(&self) -> Vec<SaveFileInfo> {
        let mut saves = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.save_directory) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("save_") && filename.ends_with(".ron") {
                        if let Some(slot_str) = filename.strip_prefix("save_").and_then(|s| s.strip_suffix(".ron")) {
                            if let Ok(slot) = slot_str.parse::<u32>() {
                                if let Ok(metadata) = entry.metadata() {
                                    if let Ok(modified) = metadata.modified() {
                                        saves.push(SaveFileInfo {
                                            slot,
                                            filename: filename.to_string(),
                                            last_modified: modified,
                                            exists: true,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        saves.sort_by_key(|s| s.slot);
        saves
    }
}

#[derive(Debug, Clone)]
pub struct SaveFileInfo {
    pub slot: u32,
    pub filename: String,
    pub last_modified: std::time::SystemTime,
    pub exists: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameSaveData {
    pub version: String,
    pub save_timestamp: u64,
    
    // Player progress
    pub player_inventory: InventorySaveData,
    pub discovered_species: HashSet<BirdSpecies>,
    pub achievements: HashSet<Achievement>,
    pub achievement_progress: AchievementProgressSaveData,
    
    // World state
    pub environment_state: EnvironmentSaveData,
    pub placed_objects: Vec<PlacedObjectSaveData>,
    
    // Game statistics
    pub total_photos_taken: u32,
    pub total_playtime_seconds: f64,
    pub birds_observed: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InventorySaveData {
    pub currency: u32,
    pub owned_items: HashMap<PlaceableItemType, u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AchievementProgressSaveData {
    pub photos_taken: u32,
    pub currency_earned: u32,
    pub species_discovered: u32,
    pub feeders_upgraded: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvironmentSaveData {
    pub current_hour: f32,
    pub day_of_year: u32,
    pub current_weather: Weather,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlacedObjectSaveData {
    pub item_type: PlaceableItemType,
    pub position: [f32; 3],
    pub save_id: String,
}

// Events
#[derive(Event)]
pub struct SaveGameEvent {
    pub slot: u32,
    pub save_name: Option<String>,
}

#[derive(Event)]
pub struct LoadGameEvent {
    pub slot: u32,
}

#[derive(Event)]
pub struct SaveCompleteEvent {
    pub slot: u32,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Event)]
pub struct LoadCompleteEvent {
    pub slot: u32,
    pub success: bool,
    pub error_message: Option<String>,
}
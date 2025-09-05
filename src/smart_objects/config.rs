use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::bird_ai::components::BirdAction;
use crate::bird::BirdSpecies;

#[derive(Asset, TypePath, Resource, Debug, Clone, Deserialize, Serialize)]
pub struct SmartObjectCatalog {
    pub items: Vec<SmartObjectDefinition>,
    pub global_settings: CatalogGlobalSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmartObjectDefinition {
    pub id: String,
    pub metadata: SmartObjectMetadata,
    pub visual: VisualData,
    pub physics: PhysicsData,
    pub behavior: BehaviorData,
    pub attraction: AttractionData,
    pub economy: EconomyData,
    pub workshop: WorkshopData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmartObjectMetadata {
    pub name: String,
    pub description: String,
    pub category: String,
    pub price: u32,
    pub rarity: String,
    pub unlock_level: u32,
    pub workshop_compatible: bool,
    pub version: String,
    pub author: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisualData {
    pub sprite_filename: String,
    pub size: (f32, f32),
    pub color_tint: (f32, f32, f32, f32),
    pub scale: f32,
    pub z_order: f32,
    pub animation_set: Option<String>,
    pub particle_effects: Vec<String>,
    pub sound_effects: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PhysicsData {
    pub collision_shape: String, // "Rectangle", "Circle", "Custom"
    pub collision_size: (f32, f32),
    pub is_solid: bool,
    pub weight: f32,
    pub can_be_moved: bool,
    pub stability: f32, // Resistance to being knocked over
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BehaviorData {
    pub provides_actions: Vec<String>, // String versions of BirdAction
    pub base_utility: f32,
    pub interaction_range: f32,
    pub max_simultaneous_users: u32,
    pub usage_duration_range: (f32, f32), // min, max seconds
    pub cooldown_after_use: f32,
    pub seasonal_modifiers: Vec<(String, String, f32)>, // season, property, multiplier
    pub weather_resistance: f32, // 0.0-1.0, resistance to weather effects
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttractionData {
    pub attracts_species: Vec<String>, // String versions of BirdSpecies
    pub species_preferences: HashMap<String, f32>, // species -> preference multiplier
    pub size_preferences: Vec<u32>, // Bird size categories this attracts
    pub time_of_day_bonus: Vec<(String, f32)>, // time period, multiplier
    pub seasonal_attraction: HashMap<String, f32>, // season -> multiplier
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EconomyData {
    pub purchase_cost: u32,
    pub maintenance_cost: u32, // Cost per maintenance cycle
    pub durability: f32, // 0.0-1.0, how long it lasts
    pub decay_rate: f32, // How fast it degrades per day
    pub repair_cost_multiplier: f32, // Cost to repair as fraction of purchase cost
    pub resale_value_multiplier: f32, // Resale value as fraction of purchase cost
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkshopData {
    pub is_moddable: bool,
    pub custom_script_support: bool,
    pub texture_replaceable: bool,
    pub model_replaceable: bool,
    pub behavior_scriptable: bool,
    pub required_dependencies: Vec<String>, // Other workshop items required
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogGlobalSettings {
    pub base_interaction_multiplier: f32,
    pub seasonal_effect_strength: f32,
    pub weather_effect_strength: f32,
    pub species_preference_weight: f32,
    pub distance_falloff_rate: f32,
    pub simultaneous_user_penalty: f32,
    pub maintenance_frequency_days: f32,
    pub decay_calculation_interval: f32,
}

// Component that gets attached to spawned smart objects
#[derive(Component, Debug, Clone)]
pub struct ConfigurableSmartObject {
    pub definition_id: String,
    pub current_durability: f32,
    pub last_maintenance: f64, // Game time of last maintenance
    pub current_users: Vec<Entity>, // Birds currently using this object
    pub total_usage_time: f32,
    pub custom_modifications: HashMap<String, String>, // Workshop customizations
}

impl SmartObjectCatalog {
    pub fn get_definition(&self, id: &str) -> Option<&SmartObjectDefinition> {
        self.items.iter().find(|item| item.id == id)
    }
    
    pub fn get_definitions_by_category(&self, category: &str) -> Vec<&SmartObjectDefinition> {
        self.items.iter().filter(|item| item.metadata.category == category).collect()
    }
    
    pub fn get_unlocked_items(&self, player_level: u32) -> Vec<&SmartObjectDefinition> {
        self.items.iter().filter(|item| item.metadata.unlock_level <= player_level).collect()
    }
}

impl SmartObjectDefinition {
    /// Convert string action names to BirdAction enum values
    pub fn get_bird_actions(&self) -> Vec<BirdAction> {
        self.behavior.provides_actions.iter()
            .filter_map(|action_str| self.parse_bird_action(action_str))
            .collect()
    }
    
    fn parse_bird_action(&self, action: &str) -> Option<BirdAction> {
        match action {
            "Eat" => Some(BirdAction::Eat),
            "Drink" => Some(BirdAction::Drink),
            "Bathe" => Some(BirdAction::Bathe),
            "Perch" => Some(BirdAction::Perch),
            "Play" => Some(BirdAction::Play),
            "Explore" => Some(BirdAction::Explore),
            "Nest" => Some(BirdAction::Nest),
            "Roost" => Some(BirdAction::Roost),
            "Shelter" => Some(BirdAction::Shelter),
            "Court" => Some(BirdAction::Court),
            "Follow" => Some(BirdAction::Follow),
            "Challenge" => Some(BirdAction::Challenge),
            "Flock" => Some(BirdAction::Flock),
            "Forage" => Some(BirdAction::Forage),
            "Cache" => Some(BirdAction::Cache),
            "Retrieve" => Some(BirdAction::Retrieve),
            "HoverFeed" => Some(BirdAction::HoverFeed),
            _ => None,
        }
    }
    
    /// Convert string species names to BirdSpecies enum values
    pub fn get_attracted_species(&self) -> Vec<BirdSpecies> {
        self.attraction.attracts_species.iter()
            .filter_map(|species_str| self.parse_bird_species(species_str))
            .collect()
    }
    
    fn parse_bird_species(&self, species: &str) -> Option<BirdSpecies> {
        match species {
            "Cardinal" => Some(BirdSpecies::Cardinal),
            "BlueJay" => Some(BirdSpecies::BlueJay),
            "Robin" => Some(BirdSpecies::Robin),
            "Sparrow" => Some(BirdSpecies::Sparrow),
            "Chickadee" => Some(BirdSpecies::Chickadee),
            "RubyThroatedHummingbird" => Some(BirdSpecies::RubyThroatedHummingbird),
            "DownyWoodpecker" => Some(BirdSpecies::DownyWoodpecker),
            "HairyWoodpecker" => Some(BirdSpecies::HairyWoodpecker),
            "WhiteBreastedNuthatch" => Some(BirdSpecies::WhiteBreastedNuthatch),
            "BrownThrasher" => Some(BirdSpecies::BrownThrasher),
            "ScarletTanager" => Some(BirdSpecies::ScarletTanager),
            "BaltimoreOriole" => Some(BirdSpecies::BaltimoreOriole),
            "NorthernMockingbird" => Some(BirdSpecies::NorthernMockingbird),
            _ => None,
        }
    }
    
    /// Calculate effective utility based on various modifiers
    pub fn calculate_effective_utility(&self, 
        season: &str, 
        weather_intensity: f32,
        current_users: u32,
        distance_from_bird: f32) -> f32 {
        
        let mut utility = self.behavior.base_utility;
        
        // Apply seasonal modifiers
        if let Some(seasonal_multiplier) = self.attraction.seasonal_attraction.get(season) {
            utility *= seasonal_multiplier;
        }
        
        // Apply weather resistance (lower resistance = more affected by bad weather)
        let weather_penalty = (1.0 - self.behavior.weather_resistance) * weather_intensity;
        utility *= 1.0 - weather_penalty;
        
        // Apply simultaneous user penalty
        if current_users > 0 {
            let user_penalty = (current_users as f32) * 0.2; // 20% penalty per additional user
            utility *= (1.0 - user_penalty).max(0.1); // Minimum 10% utility
        }
        
        // Apply distance falloff
        let distance_factor = (self.behavior.interaction_range - distance_from_bird) / self.behavior.interaction_range;
        utility *= distance_factor.max(0.0);
        
        utility.max(0.0)
    }
    
    /// Get species preference multiplier for a specific species
    pub fn get_species_preference(&self, species: &str) -> f32 {
        self.attraction.species_preferences.get(species).copied().unwrap_or(1.0)
    }
    
    /// Check if this object can accommodate another user
    pub fn can_accommodate_user(&self, current_users: u32) -> bool {
        current_users < self.behavior.max_simultaneous_users
    }
}

impl Default for CatalogGlobalSettings {
    fn default() -> Self {
        Self {
            base_interaction_multiplier: 1.0,
            seasonal_effect_strength: 0.2,
            weather_effect_strength: 0.15,
            species_preference_weight: 0.3,
            distance_falloff_rate: 0.8,
            simultaneous_user_penalty: 0.2,
            maintenance_frequency_days: 7.0,
            decay_calculation_interval: 3600.0,
        }
    }
}
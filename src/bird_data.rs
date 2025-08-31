use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::bird::BirdSpecies;
use crate::environment::components::Season;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdSpeciesConfig {
    pub species: Vec<BirdData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdData {
    pub name: String,
    pub tier: u8,
    pub size_category: u8,
    pub max_flock_size: usize,
    pub territory_radius: f32,
    pub aggression_level: f32,
    pub spawn_probability: f32,
    pub feeding_preferences: HashMap<String, f32>,
    pub behavioral_traits: HashMap<String, f32>,
    pub audio_config: HashMap<String, f32>,
    pub seasonal_availability: HashMap<String, f32>,
}

#[derive(Resource, Default)]
pub struct BirdDataRegistry {
    pub species_data: HashMap<String, BirdData>,
    pub loaded_files: Vec<String>,
}

impl BirdDataRegistry {
    pub fn load_from_files(&mut self, asset_server: &AssetServer) {
        let data_files = vec![
            "data/birds/common_species.ron",
            "data/birds/rare_species.ron",
        ];
        
        for file_path in data_files {
            if let Err(e) = self.load_species_file(file_path) {
                error!("Failed to load bird data from {}: {}", file_path, e);
            } else {
                info!("Successfully loaded bird data from {}", file_path);
                self.loaded_files.push(file_path.to_string());
            }
        }
        
        info!("Bird data registry initialized with {} species", self.species_data.len());
    }
    
    fn load_species_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = format!("assets/{}", file_path);
        let content = std::fs::read_to_string(&full_path)?;
        let config: BirdSpeciesConfig = ron::from_str(&content)?;
        
        for bird_data in config.species {
            self.species_data.insert(bird_data.name.clone(), bird_data);
        }
        
        Ok(())
    }
    
    pub fn get_species_data(&self, species_name: &str) -> Option<&BirdData> {
        self.species_data.get(species_name)
    }
    
    pub fn get_feeding_preference(&self, species: &BirdSpecies, food_type: &str) -> f32 {
        if let Some(data) = self.get_species_data(&format!("{:?}", species)) {
            data.feeding_preferences.get(food_type).copied().unwrap_or(0.0)
        } else {
            // Fallback to hardcoded values if data not found
            species.feeding_preference_fallback(food_type)
        }
    }
    
    pub fn get_spawn_probability(&self, species: &BirdSpecies, season: Season) -> f32 {
        if let Some(data) = self.get_species_data(&format!("{:?}", species)) {
            let base_prob = data.spawn_probability;
            let seasonal_modifier = data.seasonal_availability
                .get(&format!("{:?}", season))
                .copied()
                .unwrap_or(1.0);
            base_prob * seasonal_modifier
        } else {
            species.spawn_probability_fallback()
        }
    }
    
    pub fn get_behavioral_trait(&self, species: &BirdSpecies, trait_name: &str) -> f32 {
        if let Some(data) = self.get_species_data(&format!("{:?}", species)) {
            data.behavioral_traits.get(trait_name).copied().unwrap_or(0.0)
        } else {
            species.behavioral_trait_fallback(trait_name)
        }
    }
    
    pub fn get_size_category(&self, species: &BirdSpecies) -> u8 {
        if let Some(data) = self.get_species_data(&format!("{:?}", species)) {
            data.size_category
        } else {
            species.size_category_fallback()
        }
    }
}

// Extension trait for BirdSpecies fallback methods
impl BirdSpecies {
    fn feeding_preference_fallback(&self, food_type: &str) -> f32 {
        // Fallback to original hardcoded preferences
        match (self, food_type) {
            (Self::Cardinal, "Seed") => 0.9,
            (Self::Cardinal, "Suet") => 0.3,
            (Self::BlueJay, "Seed") => 0.7,
            (Self::BlueJay, "Suet") => 0.8,
            (Self::Robin, "Fruit") => 0.8,
            (Self::Robin, "Seed") => 0.4,
            (Self::Chickadee, "Seed") => 0.9,
            (Self::Chickadee, "Suet") => 0.7,
            (Self::HouseFinch, "Seed") => 1.0,
            (Self::RubyThroatedHummingbird, "Nectar") => 1.0,
            _ => 0.1,
        }
    }
    
    fn spawn_probability_fallback(&self) -> f32 {
        match self {
            Self::Chickadee => 0.20,
            Self::Robin => 0.18,
            Self::Cardinal => 0.15,
            Self::HouseFinch => 0.16,
            Self::BlueJay => 0.12,
            Self::RubyThroatedHummingbird => 0.08,
            Self::RedTailedHawk => 0.02,
            Self::GreatHornedOwl => 0.01,
            Self::BaldEagle => 0.005,
            _ => 0.10,
        }
    }
    
    fn behavioral_trait_fallback(&self, trait_name: &str) -> f32 {
        match (self, trait_name) {
            (Self::Cardinal, "territorial_aggression") => 0.6,
            (Self::BlueJay, "territorial_aggression") => 0.9,
            (Self::Chickadee, "social_compatibility_bonus") => 0.8,
            (Self::RedTailedHawk, "territorial_aggression") => 1.0,
            (Self::BaldEagle, "flight_speed") => 350.0,
            _ => match trait_name {
                "flight_speed" => 150.0,
                "feeding_duration" => 8.0,
                "territorial_aggression" => 0.4,
                "social_compatibility_bonus" => 0.2,
                _ => 0.0,
            }
        }
    }
    
    fn size_category_fallback(&self) -> u8 {
        match self {
            // Tier 1 - Common birds (size 2-5)
            Self::Chickadee | Self::WhiteBreastedNuthatch | Self::TuftedTitmouse => 2,
            Self::Goldfinch | Self::HouseFinch | Self::PurpleFinch | Self::CarolinaWren => 3,
            Self::Cardinal | Self::Robin | Self::Sparrow => 4,
            Self::BlueJay | Self::NorthernMockingbird | Self::RedWingedBlackbird => 5,
            
            // Tier 2 - Uncommon birds (size 2-6)
            Self::DownyWoodpecker => 3,
            Self::HairyWoodpecker | Self::BaltimoreOriole | Self::IndianaBunting => 4,
            Self::CommonGrackle | Self::BrownThrasher | Self::EasternBluebird => 5,
            Self::RoseBreastedGrosbeak | Self::ScarletTanager => 5,
            
            // Tier 3 - Rare birds (size 1-8)
            Self::RubyThroatedHummingbird => 1,
            Self::RedHeadedWoodpecker | Self::YellowBelledSapsucker => 4,
            Self::PileatedWoodpecker => 7,
            Self::RedTailedHawk | Self::CoopersHawk => 8,
            
            // Tier 4 - Legendary birds (size 6-8)
            Self::BaldEagle => 8,
            Self::PeregrineFalcon => 6,
            Self::ProthonotaryWarbler | Self::KentuckyWarbler => 3,
            Self::CommonCrow => 6,
            
            _ => 4, // Default medium size
        }
    }
}

pub struct BirdDataPlugin;

impl Plugin for BirdDataPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BirdDataRegistry>()
            .add_systems(Startup, load_bird_data_system);
    }
}

fn load_bird_data_system(
    mut registry: ResMut<BirdDataRegistry>,
    asset_server: Res<AssetServer>,
) {
    registry.load_from_files(&asset_server);
}
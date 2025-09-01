use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::bird::BirdSpecies;
use crate::journal::components::JournalTab;
use serde::{Serialize, Deserialize};

#[derive(Resource, Default)]
pub struct DiscoveredSpecies(pub HashSet<BirdSpecies>);

#[derive(Resource)]
pub struct JournalState {
    pub is_open: bool,
    pub current_tab: JournalTab,
    pub selected_species: Option<BirdSpecies>,
}

impl Default for JournalState {
    fn default() -> Self {
        Self {
            is_open: false,
            current_tab: JournalTab::Species,
            selected_species: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdFacts {
    pub common_name: String,
    pub scientific_name: String,
    pub habitat: String,
    pub diet: String,
    pub nesting: String,
    pub behavior: String,
    pub identification_tips: String,
    pub fun_fact: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConservationStatus {
    LeastConcern,
    NearThreatened,
    Vulnerable,
    Endangered,
    CriticallyEndangered,
    ExtinctInWild,
    Extinct,
}

impl ConservationStatus {
    pub fn color(&self) -> Color {
        match self {
            ConservationStatus::LeastConcern => Color::srgb(0.2, 0.8, 0.2),
            ConservationStatus::NearThreatened => Color::srgb(0.9, 0.9, 0.2),
            ConservationStatus::Vulnerable => Color::srgb(0.9, 0.6, 0.1),
            ConservationStatus::Endangered => Color::srgb(0.9, 0.3, 0.1),
            ConservationStatus::CriticallyEndangered => Color::srgb(0.9, 0.1, 0.1),
            ConservationStatus::ExtinctInWild => Color::srgb(0.5, 0.1, 0.1),
            ConservationStatus::Extinct => Color::srgb(0.3, 0.3, 0.3),
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            ConservationStatus::LeastConcern => "Least Concern",
            ConservationStatus::NearThreatened => "Near Threatened",
            ConservationStatus::Vulnerable => "Vulnerable",
            ConservationStatus::Endangered => "Endangered",
            ConservationStatus::CriticallyEndangered => "Critically Endangered",
            ConservationStatus::ExtinctInWild => "Extinct in Wild",
            ConservationStatus::Extinct => "Extinct",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationData {
    pub is_migratory: bool,
    pub breeding_range: String,
    pub wintering_range: String,
    pub migration_timing: String,
    pub migration_distance: Option<f32>, // km
    pub interesting_fact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationDataConfig {
    pub species: Vec<BirdEducationEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdEducationEntry {
    pub species: BirdSpecies,
    pub facts: BirdFacts,
    pub conservation_status: ConservationStatus,
    pub migration_data: MigrationData,
}

#[derive(Resource, Default)]
pub struct BirdEducationData {
    pub species_facts: HashMap<BirdSpecies, BirdFacts>,
    pub conservation_status: HashMap<BirdSpecies, ConservationStatus>,
    pub migration_data: HashMap<BirdSpecies, MigrationData>,
    pub loaded_files: Vec<String>,
}

impl BirdEducationData {
    pub fn load_from_files(&mut self) {
        let data_files = vec![
            "data/education/common_birds.ron",
            "data/education/uncommon_birds.ron", 
            "data/education/rare_birds.ron",
            "data/education/legendary_birds.ron",
        ];
        
        for file_path in data_files {
            if let Err(e) = self.load_education_file(file_path) {
                error!("Failed to load education data from {}: {}", file_path, e);
            } else {
                info!("Successfully loaded education data from {}", file_path);
                self.loaded_files.push(file_path.to_string());
            }
        }
        
        info!("Education data registry initialized with {} species", self.species_facts.len());
    }
    
    fn load_education_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        
        let content = fs::read_to_string(file_path)?;
        let config: EducationDataConfig = ron::from_str(&content)?;
        
        for entry in config.species {
            self.species_facts.insert(entry.species, entry.facts);
            self.conservation_status.insert(entry.species, entry.conservation_status);
            self.migration_data.insert(entry.species, entry.migration_data);
        }
        
        Ok(())
    }
}


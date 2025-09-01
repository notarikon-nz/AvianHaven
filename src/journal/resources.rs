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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Resource)]
pub struct BirdEducationData {
    pub species_facts: HashMap<BirdSpecies, BirdFacts>,
    pub conservation_status: HashMap<BirdSpecies, ConservationStatus>,
    pub migration_data: HashMap<BirdSpecies, MigrationData>,
}

impl Default for BirdEducationData {
    fn default() -> Self {
        let mut education_data = Self {
            species_facts: HashMap::new(),
            conservation_status: HashMap::new(),
            migration_data: HashMap::new(),
        };
        
        education_data.populate_default_data();
        education_data
    }
}

impl BirdEducationData {
    fn populate_default_data(&mut self) {
        // Sample data for a few species - in production this would be loaded from files
        
        // American Robin
        self.species_facts.insert(BirdSpecies::AmericanRobin, BirdFacts {
            common_name: "American Robin".to_string(),
            scientific_name: "Turdus migratorius".to_string(),
            habitat: "Open woodlands, parks, gardens, lawns".to_string(),
            diet: "Earthworms, insects, berries, fruits".to_string(),
            nesting: "Cup-shaped nests in trees or shrubs, 3-5 blue eggs".to_string(),
            behavior: "Often seen hopping on lawns searching for worms. Forms large flocks in winter.".to_string(),
            identification_tips: "Orange-red breast, dark head, yellow bill. Males darker than females.".to_string(),
            fun_fact: "Robins can live up to 13 years and are often the first birds to sing at dawn!".to_string(),
        });
        
        self.conservation_status.insert(BirdSpecies::AmericanRobin, ConservationStatus::LeastConcern);
        
        self.migration_data.insert(BirdSpecies::AmericanRobin, MigrationData {
            is_migratory: true,
            breeding_range: "Alaska and Canada south to Mexico".to_string(),
            wintering_range: "Southern United States to Central America".to_string(),
            migration_timing: "Spring: March-May, Fall: September-November".to_string(),
            migration_distance: Some(2500.0),
            interesting_fact: "Some robins don't migrate at all if food sources remain available!".to_string(),
        });
        
        // Northern Cardinal  
        self.species_facts.insert(BirdSpecies::NorthernCardinal, BirdFacts {
            common_name: "Northern Cardinal".to_string(),
            scientific_name: "Cardinalis cardinalis".to_string(),
            habitat: "Woodland edges, overgrown fields, parks, gardens".to_string(),
            diet: "Seeds, fruits, insects, snails".to_string(),
            nesting: "Dense shrub nests, 2-5 eggs, multiple broods per year".to_string(),
            behavior: "Non-migratory. Males sing year-round and are highly territorial.".to_string(),
            identification_tips: "Male: Bright red with black face. Female: Brown with red tinges.".to_string(),
            fun_fact: "Cardinals can live up to 15 years and mate for life!".to_string(),
        });
        
        self.conservation_status.insert(BirdSpecies::NorthernCardinal, ConservationStatus::LeastConcern);
        
        self.migration_data.insert(BirdSpecies::NorthernCardinal, MigrationData {
            is_migratory: false,
            breeding_range: "Eastern United States, extending west and north".to_string(),
            wintering_range: "Same as breeding range - non-migratory".to_string(),
            migration_timing: "N/A - Resident year-round".to_string(),
            migration_distance: None,
            interesting_fact: "Cardinals have expanded their range northward over the past century due to bird feeders!".to_string(),
        });
        
        // Add similar data for more species...
        // In production, this would be loaded from comprehensive JSON/RON files
    }
}


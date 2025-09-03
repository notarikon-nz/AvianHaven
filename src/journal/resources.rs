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

// Research Mission System
#[derive(Resource, Default)]
pub struct ResearchMissionManager {
    pub active_missions: Vec<ResearchMission>,
    pub completed_missions: Vec<ResearchMission>,
    pub collected_data: HashMap<DataType, u32>,
    pub research_points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMission {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub mission_type: MissionType,
    pub difficulty: MissionDifficulty,
    pub objectives: Vec<ResearchObjective>,
    pub rewards: ResearchRewards,
    pub progress: MissionProgress,
    pub citizen_science_partner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissionType {
    BehaviorStudy {
        target_species: BirdSpecies,
        target_behavior: String,
        required_observations: u32,
    },
    MigrationTracking {
        species_group: Vec<BirdSpecies>,
        tracking_duration: f32, // in game days
        data_points_needed: u32,
    },
    FeedingEcology {
        ecosystem_type: String,
        species_interactions: u32,
        food_preference_data: u32,
    },
    PopulationCount {
        target_area: String,
        species_census: HashMap<BirdSpecies, u32>,
        accuracy_requirement: f32,
    },
    ConservationStudy {
        threatened_species: BirdSpecies,
        habitat_assessment: bool,
        threat_documentation: u32,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MissionDifficulty {
    Citizen,      // 1-2 weeks, basic observation
    Student,      // 2-4 weeks, detailed data collection
    Researcher,   // 1-2 months, complex study design
    Expert,       // 2+ months, publication-quality research
}

impl MissionDifficulty {
    pub fn duration_days(&self) -> u32 {
        match self {
            Self::Citizen => 7,
            Self::Student => 21,
            Self::Researcher => 45,
            Self::Expert => 90,
        }
    }
    
    pub fn research_points(&self) -> u32 {
        match self {
            Self::Citizen => 50,
            Self::Student => 150,
            Self::Researcher => 400,
            Self::Expert => 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchObjective {
    pub id: String,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub completed: bool,
    pub progress: f32, // 0.0-1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    ObserveBehavior { species: BirdSpecies, behavior: String, count: u32 },
    CollectPhotos { species: BirdSpecies, min_score: u32, count: u32 },
    DocumentInteraction { species_a: BirdSpecies, species_b: BirdSpecies, count: u32 },
    TrackMovement { species: BirdSpecies, duration_hours: f32 },
    AnalyzeFeeding { location: String, species_count: u32, duration_hours: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchRewards {
    pub research_points: u32,
    pub currency: u32,
    pub unlocked_content: Vec<String>,
    pub badge: Option<String>,
    pub citizen_science_credit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionProgress {
    pub started_date: String,
    pub days_active: u32,
    pub completion_percentage: f32,
    pub data_quality_score: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    BehaviorObservations,
    SpeciesInteractions,
    FeedingPatterns,
    MigrationData,
    HabitatUse,
    VocalizationRecords,
    BreedingBehavior,
    FlockDynamics,
}

impl ResearchMissionManager {
    pub fn generate_starter_missions() -> Vec<ResearchMission> {
        vec![
            ResearchMission {
                id: 1,
                title: "Dawn Chorus Study".to_string(),
                description: "Document morning bird activity patterns to contribute to citizen science databases".to_string(),
                mission_type: MissionType::BehaviorStudy {
                    target_species: BirdSpecies::Robin,
                    target_behavior: "Dawn singing".to_string(),
                    required_observations: 10,
                },
                difficulty: MissionDifficulty::Citizen,
                objectives: vec![
                    ResearchObjective {
                        id: "dawn_photos".to_string(),
                        description: "Capture 5 photos of birds singing during dawn chorus (6-8 AM)".to_string(),
                        objective_type: ObjectiveType::CollectPhotos {
                            species: BirdSpecies::Robin,
                            min_score: 400,
                            count: 5,
                        },
                        completed: false,
                        progress: 0.0,
                    },
                    ResearchObjective {
                        id: "dawn_observations".to_string(),
                        description: "Record 10 instances of dawn singing behavior".to_string(),
                        objective_type: ObjectiveType::ObserveBehavior {
                            species: BirdSpecies::Robin,
                            behavior: "Singing".to_string(),
                            count: 10,
                        },
                        completed: false,
                        progress: 0.0,
                    },
                ],
                rewards: ResearchRewards {
                    research_points: 50,
                    currency: 200,
                    unlocked_content: vec!["Dawn Chorus Audio Guide".to_string()],
                    badge: Some("Early Bird Researcher".to_string()),
                    citizen_science_credit: true,
                },
                progress: MissionProgress {
                    started_date: "2025-01-01".to_string(),
                    days_active: 0,
                    completion_percentage: 0.0,
                    data_quality_score: 0.0,
                },
                citizen_science_partner: Some("eBird/Cornell Lab".to_string()),
            },
            ResearchMission {
                id: 2,
                title: "Feeder Interaction Study".to_string(),
                description: "Analyze feeding behavior and species interactions at different feeder types".to_string(),
                mission_type: MissionType::FeedingEcology {
                    ecosystem_type: "Backyard habitat".to_string(),
                    species_interactions: 20,
                    food_preference_data: 50,
                },
                difficulty: MissionDifficulty::Student,
                objectives: vec![
                    ResearchObjective {
                        id: "feeder_photos".to_string(),
                        description: "Document 15 different species at various feeder types".to_string(),
                        objective_type: ObjectiveType::CollectPhotos {
                            species: BirdSpecies::Chickadee, // Placeholder - any species
                            min_score: 300,
                            count: 15,
                        },
                        completed: false,
                        progress: 0.0,
                    },
                    ResearchObjective {
                        id: "interaction_data".to_string(),
                        description: "Record 20 interspecies interactions at feeding stations".to_string(),
                        objective_type: ObjectiveType::DocumentInteraction {
                            species_a: BirdSpecies::Cardinal,
                            species_b: BirdSpecies::BlueJay,
                            count: 20,
                        },
                        completed: false,
                        progress: 0.0,
                    },
                ],
                rewards: ResearchRewards {
                    research_points: 150,
                    currency: 500,
                    unlocked_content: vec!["Feeder Placement Guide".to_string(), "Species Preference Chart".to_string()],
                    badge: Some("Feeding Ecology Specialist".to_string()),
                    citizen_science_credit: true,
                },
                progress: MissionProgress {
                    started_date: "2025-01-01".to_string(),
                    days_active: 0,
                    completion_percentage: 0.0,
                    data_quality_score: 0.0,
                },
                citizen_science_partner: Some("Project FeederWatch".to_string()),
            },
            ResearchMission {
                id: 3,
                title: "Climate Change Impact Assessment".to_string(),
                description: "Study how changing weather patterns affect bird behavior and migration timing".to_string(),
                mission_type: MissionType::MigrationTracking {
                    species_group: vec![
                        BirdSpecies::Robin, BirdSpecies::RedWingedBlackbird,
                        BirdSpecies::BaltimoreOriole, BirdSpecies::RosebrestedGrosbeak,
                    ],
                    tracking_duration: 30.0,
                    data_points_needed: 100,
                },
                difficulty: MissionDifficulty::Researcher,
                objectives: vec![
                    ResearchObjective {
                        id: "migration_timing".to_string(),
                        description: "Track arrival and departure dates for 4 migratory species".to_string(),
                        objective_type: ObjectiveType::TrackMovement {
                            species: BirdSpecies::Robin,
                            duration_hours: 720.0, // 30 days
                        },
                        completed: false,
                        progress: 0.0,
                    },
                ],
                rewards: ResearchRewards {
                    research_points: 400,
                    currency: 1000,
                    unlocked_content: vec!["Climate Data Overlay".to_string(), "Migration Prediction Tool".to_string()],
                    badge: Some("Climate Research Contributor".to_string()),
                    citizen_science_credit: true,
                },
                progress: MissionProgress {
                    started_date: "2025-01-01".to_string(),
                    days_active: 0,
                    completion_percentage: 0.0,
                    data_quality_score: 0.0,
                },
                citizen_science_partner: Some("Audubon Climate Watch".to_string()),
            },
        ]
    }
}


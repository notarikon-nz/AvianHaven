// Advanced Sanctuary Management - Phase 4
use bevy::prelude::*;
use crate::bird::{BirdSpecies};
use crate::bird_ai::components::{SmartObject, ProvidesUtility};
use crate::environment::components::{Season};
use crate::advanced_weather::{WeatherShelter, ShelterType};

// Additional sanctuary management extensions to ShelterType
impl ShelterType {
    pub fn cost(&self) -> u32 {
        match self {
            Self::DenseVegetation => 300,
            Self::Building => 1200,
            Self::TreeHollow => 600,
            Self::RockFormation => 800,
            Self::Feeder => 500,
        }
    }
    
    pub fn maintenance_needs(&self) -> f32 {
        match self {
            Self::DenseVegetation => 0.1,  // Natural, low maintenance
            Self::Building => 0.3,         // Requires upkeep
            Self::TreeHollow => 0.05,      // Natural, minimal maintenance
            Self::RockFormation => 0.0,    // No maintenance needed
            Self::Feeder => 0.2,           // Regular cleaning needed
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::DenseVegetation => "Dense Vegetation",
            Self::Building => "Building Shelter",
            Self::TreeHollow => "Tree Hollow",
            Self::RockFormation => "Rock Formation",
            Self::Feeder => "Covered Feeder",
        }
    }
}

// Nesting Box System
#[derive(Component, Debug, Clone)]
pub struct NestingBox {
    pub box_type: NestingBoxType,
    pub target_species: Vec<BirdSpecies>,
    pub occupancy_status: NestingStatus,
    pub breeding_season: Vec<Season>,
    pub success_rate: f32,
    pub maintenance_required: bool,
    pub maintenance_level: f32,
    pub eggs_laid: u32,
    pub fledglings_raised: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NestingBoxType {
    SmallCavity,      // Chickadees, Wrens
    MediumCavity,     // Bluebirds, Swallows
    LargeCavity,      // Woodpeckers
    OpenPlatform,     // Robins, Cardinals
    SpecializedBox,   // Species-specific designs
}

impl NestingBoxType {
    pub fn suitable_species(&self) -> Vec<BirdSpecies> {
        match self {
            Self::SmallCavity => vec![BirdSpecies::Chickadee, BirdSpecies::CarolinaWren],
            Self::MediumCavity => vec![BirdSpecies::BlueGrayGnatcatcher],
            Self::LargeCavity => vec![BirdSpecies::DownyWoodpecker],
            Self::OpenPlatform => vec![BirdSpecies::Robin, BirdSpecies::Cardinal],
            Self::SpecializedBox => vec![BirdSpecies::BlueJay, BirdSpecies::NorthernMockingbird],
        }
    }
    
    pub fn cost(&self) -> u32 {
        match self {
            Self::SmallCavity => 150,
            Self::MediumCavity => 200,
            Self::LargeCavity => 300,
            Self::OpenPlatform => 250,
            Self::SpecializedBox => 400,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NestingStatus {
    Empty,
    UnderConstruction,
    EggsLaid(u32),
    Incubating,
    Hatched(u32), // Number of chicks
    Fledged,
    Abandoned,
}

// Predator Management System
#[derive(Resource, Default)]
pub struct PredatorManagement {
    pub active_deterrents: Vec<PredatorDeterrent>,
    pub predator_activity: PredatorActivity,
    pub protection_zones: Vec<ProtectionZone>,
}

#[derive(Component, Debug, Clone)]
pub struct PredatorDeterrent {
    pub deterrent_type: DeterrentType,
    pub position: Vec3,
    pub effectiveness: f32,
    pub range: f32,
    pub maintenance_timer: Timer,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeterrentType {
    MotionActivatedSprinkler, // General deterrent
    UltrasonicDevice,        // Cats, small predators
    ReflectiveTape,          // Birds of prey
    ScareOwl,               // Small mammals
    ProtectiveMesh,         // Physical barrier
    NaturalBarrier,         // Thorny bushes, etc.
}

impl DeterrentType {
    pub fn effectiveness_against(&self, predator: PredatorType) -> f32 {
        match (self, predator) {
            (Self::MotionActivatedSprinkler, PredatorType::Cat) => 0.8,
            (Self::UltrasonicDevice, PredatorType::Cat) => 0.6,
            (Self::ReflectiveTape, PredatorType::Hawk) => 0.7,
            (Self::ScareOwl, PredatorType::Rodent) => 0.5,
            (Self::ProtectiveMesh, _) => 0.9,
            (Self::NaturalBarrier, PredatorType::Cat) => 0.4,
            _ => 0.1,
        }
    }
    
    pub fn cost(&self) -> u32 {
        match self {
            Self::MotionActivatedSprinkler => 300,
            Self::UltrasonicDevice => 200,
            Self::ReflectiveTape => 50,
            Self::ScareOwl => 100,
            Self::ProtectiveMesh => 250,
            Self::NaturalBarrier => 150,
        }
    }
    
    pub fn maintenance_interval(&self) -> f32 {
        match self {
            Self::MotionActivatedSprinkler => 30.0, // days
            Self::UltrasonicDevice => 60.0,
            Self::ReflectiveTape => 14.0,
            Self::ScareOwl => 90.0,
            Self::ProtectiveMesh => 120.0,
            Self::NaturalBarrier => 180.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PredatorType {
    Cat,
    Hawk,
    Snake,
    Rodent,
    Other,
}

#[derive(Debug, Clone, Default)]
pub struct PredatorActivity {
    pub recent_sightings: Vec<PredatorSighting>,
    pub threat_level: ThreatLevel,
    pub protected_birds: u32,
    pub successful_attacks_prevented: u32,
}

#[derive(Debug, Clone)]
pub struct PredatorSighting {
    pub predator_type: PredatorType,
    pub location: Vec3,
    pub timestamp: f32,
    pub deterred: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ThreatLevel {
    #[default]
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ProtectionZone {
    pub center: Vec3,
    pub radius: f32,
    pub protection_level: f32,
    pub active_deterrents: Vec<DeterrentType>,
}

// Habitat Enhancement
#[derive(Component, Debug, Clone)]
pub struct HabitatEnhancement {
    pub enhancement_type: EnhancementType,
    pub species_attraction: Vec<(BirdSpecies, f32)>, // Species and attraction bonus
    pub seasonal_effectiveness: Vec<(Season, f32)>,
    pub maintenance_cost: u32,
    pub ecological_impact: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnhancementType {
    NativePlanting,       // Native plants for food/shelter
    ButterflyGarden,      // Attracts insects, then insectivorous birds
    SeedBearing,          // Plants with seeds birds eat
    FruitBearing,         // Berry-producing plants
    CoverPlanting,        // Dense shrubs for protection
    GroundCover,          // Low plants for ground-feeding birds
    FlowerMeadow,         // Pollinator-friendly space
}

impl EnhancementType {
    pub fn attracted_species(&self) -> Vec<(BirdSpecies, f32)> {
        match self {
            Self::NativePlanting => vec![
                (BirdSpecies::Cardinal, 0.3),
                (BirdSpecies::Robin, 0.2),
                (BirdSpecies::Sparrow, 0.4),
            ],
            Self::ButterflyGarden => vec![
                (BirdSpecies::YellowWarbler, 0.5),
                (BirdSpecies::Chickadee, 0.3),
            ],
            Self::SeedBearing => vec![
                (BirdSpecies::Goldfinch, 0.6),
                (BirdSpecies::Sparrow, 0.4),
            ],
            Self::FruitBearing => vec![
                (BirdSpecies::Robin, 0.5),
                (BirdSpecies::CedarWaxwing, 0.7),
            ],
            Self::CoverPlanting => vec![
                (BirdSpecies::CarolinaWren, 0.4),
                (BirdSpecies::BrownThrasher, 0.5),
            ],
            Self::GroundCover => vec![
                (BirdSpecies::Sparrow, 0.5),
                (BirdSpecies::CommonGrackle, 0.3),
            ],
            Self::FlowerMeadow => vec![
                (BirdSpecies::Goldfinch, 0.4),
                (BirdSpecies::YellowWarbler, 0.3),
            ],
        }
    }
    
    pub fn cost(&self) -> u32 {
        match self {
            Self::NativePlanting => 400,
            Self::ButterflyGarden => 350,
            Self::SeedBearing => 300,
            Self::FruitBearing => 450,
            Self::CoverPlanting => 500,
            Self::GroundCover => 250,
            Self::FlowerMeadow => 300,
        }
    }
}


// Note: WeatherShelter and NestingBox are components, not trait implementations
// They integrate with the SmartObject system through the existing BirdAction mechanism

// Events
#[derive(Event)]
pub struct PredatorSpottedEvent {
    pub predator_type: PredatorType,
    pub location: Vec3,
}

#[derive(Event)]
pub struct NestingEvent {
    pub box_id: Entity,
    pub species: BirdSpecies,
    pub event_type: NestingEventType,
}

#[derive(Debug, Clone)]
pub enum NestingEventType {
    EggsLaid(u32),
    ChicksHatched(u32),
    Fledglings(u32),
    NestAbandoned,
}

#[derive(Event)]
pub struct ShelterOccupancyEvent {
    pub shelter_id: Entity,
    pub occupancy_change: i32, // +1 for entry, -1 for exit
}

// Sanctuary Management Plugin
pub struct SanctuaryManagementPlugin;

impl Plugin for SanctuaryManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PredatorSpottedEvent>()
            .add_event::<NestingEvent>()
            .add_event::<ShelterOccupancyEvent>()
            .add_systems(OnEnter(crate::AppState::Playing), setup_sanctuary_objects)
            .add_systems(Update, (
                nesting_box_system,
                predator_management_system,
                shelter_maintenance_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// Setup sanctuary objects
fn setup_sanctuary_objects(mut commands: Commands) {
    // Spawn some predator deterrents
    commands.spawn((
        Transform::from_xyz(-200.0, 100.0, 0.8),
        Sprite::from_color(Color::srgb(0.7, 0.7, 0.7), Vec2::new(30.0, 80.0)),
        PredatorDeterrent {
            deterrent_type: DeterrentType::ReflectiveTape,
            position: Vec3::new(-200.0, 100.0, 0.8),
            effectiveness: 0.7,
            range: 150.0,
            maintenance_timer: Timer::from_seconds(300.0, TimerMode::Repeating),
            active: true,
        },
    ));
    
    // Spawn nesting boxes
    commands.spawn((
        Transform::from_xyz(150.0, 120.0, 0.6),
        Sprite::from_color(Color::srgb(0.6, 0.4, 0.2), Vec2::new(25.0, 30.0)),
        NestingBox {
            box_type: NestingBoxType::SmallCavity,
            target_species: vec![BirdSpecies::Chickadee, BirdSpecies::CarolinaWren],
            occupancy_status: NestingStatus::Empty,
            breeding_season: vec![Season::Spring, Season::Summer],
            success_rate: 0.75,
            maintenance_required: false,
            maintenance_level: 1.0,
            eggs_laid: 0,
            fledglings_raised: 0,
        },
        SmartObject,
        ProvidesUtility {
            action: crate::bird_ai::components::BirdAction::Nest,
            base_utility: 0.8,
            range: 100.0,
        },
    ));
}

// Basic systems for sanctuary management
fn nesting_box_system(
    mut nesting_query: Query<&mut NestingBox>,
    time: Res<Time>,
) {
    for mut nesting_box in nesting_query.iter_mut() {
        // Simple nesting box decay
        nesting_box.maintenance_level -= time.delta_secs() * 0.001; // Slow decay
        nesting_box.maintenance_level = nesting_box.maintenance_level.max(0.0);
    }
}

fn predator_management_system(
    deterrent_query: Query<&PredatorDeterrent>,
    mut predator_events: EventWriter<PredatorSpottedEvent>,
) {
    // Simple predator deterrent system - would be expanded later
    for _deterrent in deterrent_query.iter() {
        // Deterrents reduce predator spawn chances
    }
}

fn shelter_maintenance_system(
    mut shelter_query: Query<&mut WeatherShelter>,
    time: Res<Time>,
) {
    for mut shelter in shelter_query.iter_mut() {
        // Maintenance system would be implemented here
        // For now, just ensure occupancy doesn't exceed capacity
        if shelter.current_occupancy > shelter.capacity {
            shelter.current_occupancy = shelter.capacity;
        }
    }
}
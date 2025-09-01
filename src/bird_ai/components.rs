use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct BirdAI;

#[derive(Component, Default)]
pub struct Blackboard {
    pub internal: InternalState,
    pub world_knowledge: WorldKnowledge,
    pub current_target: Option<Entity>,
}

#[derive(Default)]
pub struct InternalState {
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
    pub fear: f32,
    pub social_need: f32,      // Desire for social interaction
    pub territorial_stress: f32, // Stress from territorial pressure
}

#[derive(Default)]
pub struct WorldKnowledge {
    pub perceived_threat: Option<Vec2>,
    pub available_actions: HashMap<BirdAction, UtilityEntry>,
    pub nearby_birds: Vec<SocialBirdInfo>,      // Information about nearby birds
    pub potential_mates: Vec<Entity>,           // Compatible birds for mating
    pub territory_challengers: Vec<Entity>,     // Birds challenging territory
}

pub struct UtilityEntry {
    pub entity: Entity,
    pub score: f32,
}

#[derive(Clone)]
pub struct SocialBirdInfo {
    pub entity: Entity,
    pub species: crate::bird::BirdSpecies,
    pub position: Vec2,
    pub distance: f32,
    pub is_same_species: bool,
    pub dominance_level: f32,
    pub social_compatibility: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BirdState {
    #[default]
    Wandering,
    MovingToTarget,
    Eating,
    Drinking,
    Bathing,
    Fleeing,
    Resting,
    Playing,        // For interaction with toys
    Exploring,      // For investigating decorative items
    Nesting,        // For using nesting boxes and bird houses
    Roosting,       // For evening gathering behavior
    Sheltering,     // For weather protection behavior
    Courting,       // Mate attraction and courtship display
    Following,      // Following potential mate or social partner
    Territorial,    // Defending territory from rivals
    Flocking,       // Mixed species flocking behavior
    Foraging,       // Ground foraging with search patterns
    Caching,        // Storing food in hidden locations
    Retrieving,     // Recovering cached food items
    HoverFeeding,   // Hover feeding behavior for nectar species
}

#[derive(Component)]
pub struct SmartObject;

#[derive(Component, Clone)]
pub struct ProvidesUtility {
    pub action: BirdAction,
    pub base_utility: f32,
    pub range: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirdAction {
    Eat,
    Drink,
    Bathe,
    Perch,
    Play,         // For toys and interactive objects
    Explore,      // For decorative items that encourage investigation
    Nest,         // For nesting boxes and houses
    Roost,        // For evening roosting and gathering behavior
    Shelter,      // For weather protection and storm sheltering
    Court,        // Courtship display toward potential mate
    Follow,       // Follow another bird for social interaction
    Challenge,    // Challenge territorial rival
    Flock,        // Join mixed species flock
    Forage,       // Ground foraging with search patterns
    Cache,        // Store food in hiding spots
    Retrieve,     // Retrieve cached food items
    HoverFeed,    // Hover feeding for nectar species
}

// Social behavior tracking components
#[derive(Component, Default)]
pub struct SocialBirdTraits {
    pub dominance_level: f32,      // 0.0 = submissive, 1.0 = highly dominant
    pub territorial_aggression: f32, // How aggressively this bird defends territory
    pub social_tolerance: f32,     // Tolerance for other birds nearby
    pub mating_receptivity: f32,   // Current willingness to mate (seasonal)
    pub flock_tendency: f32,       // Likelihood to join mixed flocks
}

#[derive(Component, Default)]
pub struct SocialRelationships {
    pub mate: Option<Entity>,              // Current mate if paired
    pub rivals: Vec<Entity>,               // Territorial rivals
    pub flock_members: Vec<Entity>,        // Current flock companions
    pub courtship_targets: Vec<Entity>,    // Birds being courted
    pub territory_center: Option<Vec2>,    // Center of claimed territory
    pub territory_radius: f32,             // Size of claimed territory
}

// Foraging behavior components
#[derive(Component, Default)]
pub struct ForagingTraits {
    pub foraging_style: ForagingStyle,
    pub ground_preference: f32,    // 0.0 = never on ground, 1.0 = primarily ground feeder
    pub cache_tendency: f32,       // Likelihood to cache food for later
    pub search_pattern: SearchPattern,
    pub hover_ability: f32,        // Ability to hover feed (0.0 = cannot hover)
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ForagingStyle {
    #[default]
    Opportunistic,    // General feeding, adapts to available food
    Specialist,       // Focused on specific food types
    Scatter,          // Scatters to find food, moves frequently
    Methodical,       // Systematic search patterns
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SearchPattern {
    #[default]
    Random,           // Random movement while foraging
    Grid,             // Systematic grid-like search
    Spiral,           // Spiral outward from center
    Linear,           // Back and forth linear searches
}

#[derive(Component, Default)]
pub struct CacheData {
    pub cached_locations: Vec<CacheSpot>,
    pub retrieval_memory: f32,     // How well bird remembers cache locations
    pub current_cache_count: u32,
    pub max_cache_capacity: u32,
}

#[derive(Clone)]
pub struct CacheSpot {
    pub location: Vec2,
    pub food_amount: f32,
    pub cache_time: f64,          // When the cache was created
    pub decay_rate: f32,          // How fast cached food spoils
}

#[derive(Component, Default)]
pub struct ForagingState {
    pub search_center: Vec2,      // Center point of current foraging area
    pub search_radius: f32,       // How far to search from center
    pub search_progress: f32,     // Progress through current search pattern
    pub items_found: u32,         // Items found in current foraging session
    pub energy_spent: f32,        // Energy used in current session
}
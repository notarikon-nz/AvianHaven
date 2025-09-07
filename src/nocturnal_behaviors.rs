// Nocturnal Bird Behaviors - Night Activity Patterns
use bevy::prelude::*;
use crate::bird::{BirdSpecies, Bird};
use crate::bird_ai::components::{BirdAI, BirdState};
use crate::environment::resources::TimeState;
use crate::environment::components::Season;

pub struct NocturnalBehaviorPlugin;

impl Plugin for NocturnalBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NocturnalManager>()
            .add_event::<RoostEvent>()
            .add_event::<HuntingEvent>()
            .add_event::<MigrationEvent>()
            .add_systems(Startup, setup_nocturnal_sites)
            .add_systems(Update, (
                initialize_nocturnal_behavior,
                nocturnal_activity_cycle_system,
                owl_hunting_system,
                roost_selection_system,
                dawn_departure_system,
                night_migration_system,
                update_nocturnal_states,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// Nocturnal behavior traits for bird species
#[derive(Debug, Clone, PartialEq)]
pub enum ActivityPattern {
    Diurnal,        // Active during day
    Nocturnal,      // Active during night
    Crepuscular,    // Active during dawn/dusk
    Cathemeral,     // Active both day and night
}

#[derive(Debug, Clone)]
pub struct NocturnalTraits {
    pub activity_pattern: ActivityPattern,
    pub night_vision_quality: f32,       // 0.0-1.0, affects hunting success
    pub hunting_efficiency: f32,         // 0.0-1.0, predatory effectiveness
    pub roost_preference: RoostType,
    pub forms_communal_roosts: bool,
    pub migration_behavior: MigrationPattern,
    pub dawn_departure_time: f32,        // Hours after sunrise (0.0-2.0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoostType {
    TreeHollow,        // Owls, woodpeckers
    DenseBranch,       // Most songbirds
    ConiferousTree,    // Winter roosting
    CommunalSite,      // Crows, starlings
    Ground,            // Some game birds
    CliffLedge,        // Raptors
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MigrationPattern {
    NonMigratory,      // Stays year-round
    ShortDistance,     // Regional movement
    NightMigrant,      // Migrates at night
    DayMigrant,        // Migrates during day
}

#[derive(Component)]
pub struct NocturnalBehavior {
    pub traits: NocturnalTraits,
    pub current_roost: Option<Entity>,
    pub hunting_success_rate: f32,
    pub energy_level: f32,              // 0.0-1.0, affects activity
    pub last_hunt_time: f32,
    pub roost_arrival_time: Option<f32>,
}

#[derive(Component)]
pub struct RoostSite {
    pub roost_type: RoostType,
    pub capacity: u32,
    pub current_occupancy: u32,
    pub quality: f32,                   // 0.0-1.0, affects selection preference
    pub safety_rating: f32,             // 0.0-1.0, predator protection
}

#[derive(Component)]
pub struct HuntingTerritory {
    pub center: Vec3,
    pub radius: f32,
    pub prey_density: f32,              // Affects hunting success
    pub competition_level: f32,         // Other predators present
}

#[derive(Resource, Default)]
pub struct NocturnalManager {
    pub active_roost_sites: Vec<Entity>,
    pub hunting_territories: Vec<Entity>,
    pub migration_active: bool,
    pub dawn_chorus_started: bool,
    pub dusk_activity_peak: bool,
}

// Events
#[derive(Event)]
pub struct RoostEvent {
    pub bird_entity: Entity,
    pub roost_site: Entity,
    pub event_type: RoostEventType,
}

#[derive(Debug, Clone)]
pub enum RoostEventType {
    Arrival,
    Departure,
    SiteSelection,
    Disturbance,
}

#[derive(Event)]
pub struct HuntingEvent {
    pub hunter: Entity,
    pub territory: Entity,
    pub success: bool,
    pub prey_type: PreyType,
}

#[derive(Debug, Clone, Copy)]
pub enum PreyType {
    SmallMammal,
    Bird,
    Insect,
    Fish,
    Reptile,
}

#[derive(Event)]
pub struct MigrationEvent {
    pub bird_entity: Entity,
    pub migration_type: MigrationPattern,
    pub direction: Vec3,
    pub distance: f32,
}

impl BirdSpecies {
    pub fn nocturnal_traits(&self) -> NocturnalTraits {
        match self {
            // Owls - Primary nocturnal predators
            BirdSpecies::GreatHornedOwl => NocturnalTraits {
                activity_pattern: ActivityPattern::Nocturnal,
                night_vision_quality: 0.95,
                hunting_efficiency: 0.9,
                roost_preference: RoostType::TreeHollow,
                forms_communal_roosts: false,
                migration_behavior: MigrationPattern::NonMigratory,
                dawn_departure_time: 1.0,
            },
            BirdSpecies::BarredOwl => NocturnalTraits {
                activity_pattern: ActivityPattern::Nocturnal,
                night_vision_quality: 0.9,
                hunting_efficiency: 0.85,
                roost_preference: RoostType::TreeHollow,
                forms_communal_roosts: false,
                migration_behavior: MigrationPattern::NonMigratory,
                dawn_departure_time: 0.8,
            },
            
            // Hawks - Diurnal but roost at night
            BirdSpecies::RedTailedHawk | BirdSpecies::CoopersHawk => NocturnalTraits {
                activity_pattern: ActivityPattern::Diurnal,
                night_vision_quality: 0.3,
                hunting_efficiency: 0.1,
                roost_preference: RoostType::DenseBranch,
                forms_communal_roosts: false,
                migration_behavior: MigrationPattern::DayMigrant,
                dawn_departure_time: 0.5,
            },
            
            // Crows - Intelligent communal roosters
            BirdSpecies::CommonCrow => NocturnalTraits {
                activity_pattern: ActivityPattern::Diurnal,
                night_vision_quality: 0.4,
                hunting_efficiency: 0.2,
                roost_preference: RoostType::CommunalSite,
                forms_communal_roosts: true,
                migration_behavior: MigrationPattern::ShortDistance,
                dawn_departure_time: 0.3,
            },
            
            // Starlings - Major communal roosters
            BirdSpecies::EuropeanStarling => NocturnalTraits {
                activity_pattern: ActivityPattern::Diurnal,
                night_vision_quality: 0.3,
                hunting_efficiency: 0.1,
                roost_preference: RoostType::CommunalSite,
                forms_communal_roosts: true,
                migration_behavior: MigrationPattern::ShortDistance,
                dawn_departure_time: 0.4,
            },
            
            // Night migrants - Many songbirds
            BirdSpecies::YellowWarbler | BirdSpecies::ScarletTanager | 
            BirdSpecies::BaltimoreOriole => NocturnalTraits {
                activity_pattern: ActivityPattern::Crepuscular,
                night_vision_quality: 0.6,
                hunting_efficiency: 0.05,
                roost_preference: RoostType::DenseBranch,
                forms_communal_roosts: false,
                migration_behavior: MigrationPattern::NightMigrant,
                dawn_departure_time: 0.2,
            },
            
            // Woodpeckers - Tree hollow roosters
            BirdSpecies::DownyWoodpecker | BirdSpecies::HairyWoodpecker | 
            BirdSpecies::PileatedWoodpecker => NocturnalTraits {
                activity_pattern: ActivityPattern::Diurnal,
                night_vision_quality: 0.2,
                hunting_efficiency: 0.05,
                roost_preference: RoostType::TreeHollow,
                forms_communal_roosts: false,
                migration_behavior: MigrationPattern::NonMigratory,
                dawn_departure_time: 0.6,
            },
            
            // Default for other species - typical songbird pattern
            _ => NocturnalTraits {
                activity_pattern: ActivityPattern::Diurnal,
                night_vision_quality: 0.2,
                hunting_efficiency: 0.02,
                roost_preference: RoostType::DenseBranch,
                forms_communal_roosts: false,
                migration_behavior: MigrationPattern::NonMigratory,
                dawn_departure_time: 0.4,
            },
        }
    }
}

// Nocturnal Activity Cycle System
fn nocturnal_activity_cycle_system(
    time_state: Res<TimeState>,
    mut bird_query: Query<(&mut BirdState, &mut NocturnalBehavior, &Bird), With<BirdAI>>,
    mut nocturnal_manager: ResMut<NocturnalManager>,
) {
    let current_hour = time_state.hour as f32;
    let is_night = current_hour >= 20.0 || current_hour <= 6.0;
    let is_dusk = current_hour >= 18.0 && current_hour <= 20.0;
    let is_dawn = current_hour >= 5.0 && current_hour <= 7.0;
    
    // Update global nocturnal states
    nocturnal_manager.dusk_activity_peak = is_dusk;
    nocturnal_manager.dawn_chorus_started = is_dawn;
    
    for (mut bird_state, mut nocturnal, bird) in bird_query.iter_mut() {
        let traits = bird.species.nocturnal_traits();
        
        // Calculate activity level based on species pattern and time
        let activity_modifier = match traits.activity_pattern {
            ActivityPattern::Nocturnal => {
                if is_night { 1.0 } else if is_dusk || is_dawn { 0.7 } else { 0.1 }
            },
            ActivityPattern::Diurnal => {
                if is_night { 0.1 } else if is_dusk || is_dawn { 0.6 } else { 1.0 }
            },
            ActivityPattern::Crepuscular => {
                if is_dusk || is_dawn { 1.0 } else if is_night { 0.3 } else { 0.5 }
            },
            ActivityPattern::Cathemeral => 0.8, // Always moderately active
        };
        
        // Update energy levels based on activity pattern
        nocturnal.energy_level = (nocturnal.energy_level * 0.99 + activity_modifier * 0.01).clamp(0.1, 1.0);
        
        // Modify bird AI behavior based on nocturnal state
        if activity_modifier < 0.3 {
            // Low activity - diurnal birds should rest during deep night
            // Only allow emergency behaviors to override rest
            match *bird_state {
                BirdState::Fleeing => {
                    // Allow fleeing to continue (emergency)
                },
                _ => {
                    *bird_state = BirdState::Resting;
                }
            }
        } else if activity_modifier > 0.8 {
            // High activity - normal or hunting behavior
            if matches!(traits.activity_pattern, ActivityPattern::Nocturnal) && is_night {
                // Nocturnal hunters become more active
                if *bird_state == BirdState::Resting {
                    *bird_state = BirdState::Wandering;
                }
            }
        }
    }
}

// Owl Hunting System
fn owl_hunting_system(
    mut commands: Commands,
    time_state: Res<TimeState>,
    mut owl_query: Query<(Entity, &mut NocturnalBehavior, &Transform, &Bird), 
                        (With<BirdAI>, With<NocturnalBehavior>)>,
    territory_query: Query<(Entity, &HuntingTerritory, &Transform), With<HuntingTerritory>>,
    mut hunting_events: EventWriter<HuntingEvent>,
) {
    let current_hour = time_state.hour as f32;
    let is_prime_hunting = (current_hour >= 20.0 && current_hour <= 23.0) || 
                          (current_hour >= 3.0 && current_hour <= 6.0);
    
    if !is_prime_hunting {
        return;
    }
    
    for (entity, mut nocturnal, transform, bird) in owl_query.iter_mut() {
        let traits = bird.species.nocturnal_traits();
        
        // Only process nocturnal hunters
        if traits.activity_pattern != ActivityPattern::Nocturnal || traits.hunting_efficiency < 0.5 {
            continue;
        }
        
        // Check if enough time has passed since last hunt
        if time_state.hour as f32 - nocturnal.last_hunt_time < 2.0 {
            continue;
        }
        
        // Find suitable hunting territory
        let mut best_territory = None;
        let mut best_score = 0.0;
        
        for (territory_entity, territory, territory_transform) in territory_query.iter() {
            let distance = transform.translation.distance(territory_transform.translation);
            if distance <= territory.radius {
                let score = territory.prey_density * (1.0 - territory.competition_level) * 
                           (territory.radius - distance) / territory.radius;
                
                if score > best_score {
                    best_score = score;
                    best_territory = Some(territory_entity);
                }
            }
        }
        
        if let Some(territory) = best_territory {
            // Calculate hunting success
            let success_chance = traits.hunting_efficiency * 
                                traits.night_vision_quality * 
                                nocturnal.energy_level * 
                                best_score;
            
            let success = rand::random::<f32>() < success_chance;
            nocturnal.last_hunt_time = time_state.hour as f32;
            
            if success {
                nocturnal.hunting_success_rate = (nocturnal.hunting_success_rate * 0.9 + 0.1).min(1.0);
                nocturnal.energy_level = (nocturnal.energy_level + 0.3).min(1.0);
            } else {
                nocturnal.energy_level = (nocturnal.energy_level - 0.1).max(0.1);
            }
            
            // Emit hunting event
            hunting_events.write(HuntingEvent {
                hunter: entity,
                territory,
                success,
                prey_type: PreyType::SmallMammal, // Default for owls
            });
        }
    }
}

// Roost Site Selection System
fn roost_selection_system(
    mut commands: Commands,
    time_state: Res<TimeState>,
    mut bird_query: Query<(Entity, &mut NocturnalBehavior, &Transform, &Bird), With<BirdAI>>,
    roost_query: Query<(Entity, &mut RoostSite, &Transform), With<RoostSite>>,
    mut roost_events: EventWriter<RoostEvent>,
    mut nocturnal_manager: ResMut<NocturnalManager>,
) {
    let current_hour = time_state.hour as f32;
    let seeking_roost_time = current_hour >= 18.0 || current_hour <= 7.0;
    
    if !seeking_roost_time {
        return;
    }
    
    for (entity, mut nocturnal, transform, bird) in bird_query.iter_mut() {
        let traits = bird.species.nocturnal_traits();
        
        // Skip if already at roost or nocturnal species during active time
        if nocturnal.current_roost.is_some() || 
           (matches!(traits.activity_pattern, ActivityPattern::Nocturnal) && current_hour >= 20.0) {
            continue;
        }
        
        // Find suitable roost site
        let mut best_roost = None;
        let mut best_score = 0.0;
        
        for (roost_entity, roost_site, roost_transform) in roost_query.iter() {
            // Check roost type compatibility
            if roost_site.roost_type != traits.roost_preference {
                continue;
            }
            
            // Check capacity for communal roosts
            if traits.forms_communal_roosts && roost_site.current_occupancy >= roost_site.capacity {
                continue;
            }
            
            let distance = transform.translation.distance(roost_transform.translation);
            if distance > 200.0 { // Max roost seeking distance
                continue;
            }
            
            let score = roost_site.quality * roost_site.safety_rating * (200.0 - distance) / 200.0;
            
            if score > best_score {
                best_score = score;
                best_roost = Some(roost_entity);
            }
        }
        
        // Select roost site
        if let Some(roost_entity) = best_roost {
            nocturnal.current_roost = Some(roost_entity);
            nocturnal.roost_arrival_time = Some(current_hour);
            
            roost_events.write(RoostEvent {
                bird_entity: entity,
                roost_site: roost_entity,
                event_type: RoostEventType::Arrival,
            });
            
            // Update roost occupancy for communal roosts
            if traits.forms_communal_roosts {
                // This would need to be handled in a separate system due to query restrictions
                nocturnal_manager.active_roost_sites.push(roost_entity);
            }
        }
    }
}

// Dawn Departure System
fn dawn_departure_system(
    time_state: Res<TimeState>,
    mut bird_query: Query<(Entity, &mut NocturnalBehavior, &mut BirdState, &Bird), With<BirdAI>>,
    mut roost_events: EventWriter<RoostEvent>,
) {
    let current_hour = time_state.hour as f32;
    let sunrise_time = 6.0; // Simplified - could be seasonal
    
    // Dawn departure window
    if current_hour < sunrise_time || current_hour > sunrise_time + 2.0 {
        return;
    }
    
    for (entity, mut nocturnal, mut bird_state, bird) in bird_query.iter_mut() {
        let traits = bird.species.nocturnal_traits();
        
        // Skip nocturnal species that roost during day
        if matches!(traits.activity_pattern, ActivityPattern::Nocturnal) {
            continue;
        }
        
        // Check if bird should depart roost
        if let Some(roost_site) = nocturnal.current_roost {
            let departure_time = sunrise_time + traits.dawn_departure_time;
            
            if current_hour >= departure_time {
                // Depart from roost
                nocturnal.current_roost = None;
                nocturnal.roost_arrival_time = None;
                *bird_state = BirdState::Wandering;
                
                roost_events.write(RoostEvent {
                    bird_entity: entity,
                    roost_site,
                    event_type: RoostEventType::Departure,
                });
            }
        }
    }
}

// Night Migration System
fn night_migration_system(
    time_state: Res<TimeState>,
    season: Res<crate::environment::resources::SeasonalState>,
    mut bird_query: Query<(Entity, &mut NocturnalBehavior, &mut Transform, &Bird), With<BirdAI>>,
    mut migration_events: EventWriter<MigrationEvent>,
    mut nocturnal_manager: ResMut<NocturnalManager>,
) {
    let current_hour = time_state.hour as f32;
    let is_night = current_hour >= 21.0 || current_hour <= 5.0;
    let current_season = time_state.get_season();
    let is_migration_season = matches!(current_season, Season::Spring | Season::Fall);
    
    if !is_night || !is_migration_season {
        nocturnal_manager.migration_active = false;
        return;
    }
    
    nocturnal_manager.migration_active = true;
    
    for (entity, nocturnal, mut transform, bird) in bird_query.iter_mut() {
        let traits = bird.species.nocturnal_traits();
        
        // Only night migrants participate
        if traits.migration_behavior != MigrationPattern::NightMigrant {
            continue;
        }
        
        // Migration probability based on conditions
        let migration_probability = match current_season {
            Season::Spring => 0.05, // 5% chance per night during spring
            Season::Fall => 0.08,   // 8% chance per night during fall
            _ => 0.0,
        };
        
        if rand::random::<f32>() < migration_probability {
            // Determine migration direction based on season
            let direction = match current_season {
                Season::Spring => Vec3::new(0.0, 1.0, 0.0), // North
                Season::Fall => Vec3::new(0.0, -1.0, 0.0),  // South
                _ => Vec3::ZERO,
            };
            
            let migration_distance = 100.0 + rand::random::<f32>() * 200.0; // 100-300 units
            
            // Update bird position for migration
            transform.translation += direction * migration_distance;
            
            migration_events.write(MigrationEvent {
                bird_entity: entity,
                migration_type: traits.migration_behavior,
                direction,
                distance: migration_distance,
            });
        }
    }
}

// System to add NocturnalBehavior component to birds that don't have it
fn initialize_nocturnal_behavior(
    mut commands: Commands,
    bird_query: Query<(Entity, &Bird), (With<BirdAI>, Without<NocturnalBehavior>)>,
) {
    for (entity, bird) in bird_query.iter() {
        let traits = bird.species.nocturnal_traits();
        
        commands.entity(entity).insert(NocturnalBehavior {
            traits,
            current_roost: None,
            hunting_success_rate: 0.5,
            energy_level: 0.8,
            last_hunt_time: 0.0,
            roost_arrival_time: None,
        });
    }
}

// Update Nocturnal States System
fn update_nocturnal_states(
    mut roost_query: Query<&mut RoostSite, With<RoostSite>>,
    mut roost_events: EventReader<RoostEvent>,
) {
    for event in roost_events.read() {
        if let Ok(mut roost_site) = roost_query.get_mut(event.roost_site) {
            match event.event_type {
                RoostEventType::Arrival => {
                    roost_site.current_occupancy += 1;
                },
                RoostEventType::Departure => {
                    roost_site.current_occupancy = roost_site.current_occupancy.saturating_sub(1);
                },
                _ => {},
            }
        }
    }
}

// Setup initial roost sites and hunting territories
fn setup_nocturnal_sites(
    mut commands: Commands,
    mut nocturnal_manager: ResMut<NocturnalManager>,
) {
    use rand::Rng;
    let mut rng = rand::rng();
    
    // Create roost sites across the map
    let roost_positions = [
        (-300.0, 200.0), (300.0, 200.0),   // Tree hollows for owls
        (-200.0, 150.0), (200.0, 150.0),   // Dense branches
        (-100.0, 100.0), (100.0, 100.0),   // More dense branches
        (0.0, 180.0),                       // Central communal site
        (-350.0, -100.0), (350.0, -100.0), // Coniferous trees
    ];
    
    let roost_types = [
        RoostType::TreeHollow,
        RoostType::TreeHollow,
        RoostType::DenseBranch,
        RoostType::DenseBranch,
        RoostType::DenseBranch,
        RoostType::DenseBranch,
        RoostType::CommunalSite,
        RoostType::ConiferousTree,
        RoostType::ConiferousTree,
    ];
    
    for (i, &(x, y)) in roost_positions.iter().enumerate() {
        let roost_entity = commands.spawn((
            Transform::from_xyz(x, y, 1.0),
            RoostSite {
                roost_type: roost_types[i],
                capacity: match roost_types[i] {
                    RoostType::TreeHollow => 2,
                    RoostType::CommunalSite => 20,
                    RoostType::ConiferousTree => 8,
                    _ => 5,
                },
                current_occupancy: 0,
                quality: rng.random_range(0.6..1.0),
                safety_rating: rng.random_range(0.7..1.0),
            },
        )).id();
        
        nocturnal_manager.active_roost_sites.push(roost_entity);
    }
    
    // Create hunting territories for owls and hawks
    let hunting_positions = [
        (-250.0, 100.0, 80.0),  // Left territory
        (250.0, 100.0, 80.0),   // Right territory  
        (0.0, -150.0, 100.0),   // Bottom territory
        (-150.0, -50.0, 60.0),  // Small territory
        (150.0, -50.0, 60.0),   // Small territory
    ];
    
    for &(x, y, radius) in hunting_positions.iter() {
        let territory_entity = commands.spawn((
            Transform::from_xyz(x, y, 1.0),
            HuntingTerritory {
                center: Vec3::new(x, y, 1.0),
                radius,
                prey_density: rng.random_range(0.4..0.8),
                competition_level: rng.random_range(0.1..0.4),
            },
        )).id();
        
        nocturnal_manager.hunting_territories.push(territory_entity);
    }
}
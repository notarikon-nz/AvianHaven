// Advanced Foraging Ecology System
use bevy::prelude::*;
use crate::bird::{BirdSpecies, Bird};
use crate::bird_ai::components::{BirdAI, BirdState, Blackboard, ForagingTraits};
use crate::environment::resources::{TimeState, WeatherState, SeasonalState};
use crate::environment::components::{Weather, Season};
use std::collections::HashMap;
use rand::Rng;

pub struct ForagingEcologyPlugin;

impl Plugin for ForagingEcologyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InsectManager>()
            .init_resource::<FruitManager>()
            .init_resource::<ForagingFlockManager>()
            .init_resource::<OpportunisticFoodManager>()
            .add_event::<InsectEmergenceEvent>()
            .add_event::<FruitRipeningEvent>()
            .add_event::<MixedFlockFormationEvent>()
            .add_event::<OpportunisticFoodEvent>()
            .add_systems(Startup, setup_foraging_ecology)
            .add_systems(Update, (
                insect_emergence_system,
                fruit_ripening_system,
                mixed_flock_formation_system,
                mixed_flock_behavior_system,
                opportunistic_feeding_system,
                seasonal_food_availability_system,
                leader_follower_dynamics_system,
                insectivore_behavior_system,
                frugivore_behavior_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// Food source components and types
#[derive(Component)]
pub struct InsectColony {
    pub insect_type: InsectType,
    pub population: u32,
    pub emergence_cycle: EmergenceCycle,
    pub activity_pattern: InsectActivityPattern,
    pub last_emergence: f32,
    pub peak_activity_hour: f32,
}

#[derive(Component)]
pub struct FruitTree {
    pub fruit_type: FruitType,
    pub ripening_stage: RipeningStage,
    pub fruit_count: u32,
    pub max_fruit: u32,
    pub ripening_progress: f32,
    pub seasonal_cycle: SeasonalFruit,
}

#[derive(Component)]
pub struct OpportunisticFood {
    pub food_type: OpportunisticFoodType,
    pub quantity: f32,
    pub discovery_time: f32,
    pub decay_rate: f32,
    pub attracts_species: Vec<BirdSpecies>,
}

#[derive(Component)]
pub struct MixedForagingFlock {
    pub leader: Entity,
    pub core_members: Vec<Entity>,
    pub followers: Vec<Entity>,
    pub flock_type: FlockType,
    pub formation_time: f32,
    pub cohesion_strength: f32,
    pub foraging_efficiency: f32,
}

#[derive(Component)]
pub struct FlockLeader {
    pub leadership_strength: f32,
    pub experience: f32,
    pub territory_knowledge: f32,
    pub followers: Vec<Entity>,
}

#[derive(Component)]
pub struct FlockFollower {
    pub following_tendency: f32,
    pub current_leader: Option<Entity>,
    pub benefit_received: f32,
}

// Enums for different food types and patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsectType {
    Aphids,        // Early spring, attracted by new growth
    Caterpillars,  // Late spring/summer, on leaves
    Flying,        // Summer evening swarms
    GroundDwelling, // Year-round soil insects
    TreeBark,      // Year-round under bark
    Aquatic,       // Near water sources
}

#[derive(Debug, Clone, Copy)]
pub enum EmergenceCycle {
    Daily,         // Emerge every day at specific times
    Weekly,        // Weekly cycles
    Seasonal,      // Once per season
    TemperatureBased, // Based on temperature thresholds
    WeatherTriggered, // After rain or specific weather
}

#[derive(Debug, Clone, Copy)]
pub enum InsectActivityPattern {
    Dawn,          // Most active at dawn
    Dusk,          // Most active at dusk  
    Midday,        // Peak activity midday
    AllDay,        // Active throughout day
    Night,         // Nocturnal insects
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FruitType {
    Berries,       // Small fruits, attract many species
    LargeFruit,    // Apples, pears - fewer species
    Nuts,          // Acorns, nuts - cached by some species
    Seeds,         // Sunflower seeds, pine seeds
    Nectar,        // Flowering plants
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RipeningStage {
    Unripe,
    HalfRipe,
    Ripe,
    Overripe,
    Rotting,
}

#[derive(Debug, Clone, Copy)]
pub enum SeasonalFruit {
    EarlySpring,   // March-April
    LateSpring,    // May-June  
    Summer,        // July-August
    EarlyFall,     // September
    LateFall,      // October-November
    Winter,        // December-February (limited)
}

#[derive(Debug, Clone, Copy)]
pub enum OpportunisticFoodType {
    SpilledSeed,   // Spilled from feeders
    Roadkill,      // Scavenging opportunity
    HumanFood,     // Dropped human food
    InsectSwarm,   // Temporary insect swarm
    FlowerNectar,  // Seasonal flower blooming
    SapFlow,       // Tree sap from woodpecker holes
}

#[derive(Debug, Clone, Copy)]
pub enum FlockType {
    MixedSpecies,     // Multiple species foraging together
    FeedingFlock,     // Focused on feeding
    SafetyFlock,      // Safety in numbers while foraging
    LeaderFollower,   // Experienced bird leading others
    Opportunistic,    // Formed around rare food source
}

// Resources for managing ecological systems
#[derive(Resource, Default)]
pub struct InsectManager {
    pub colonies: Vec<Entity>,
    pub emergence_schedule: HashMap<InsectType, f32>, // Next emergence time
    pub weather_effects: HashMap<Weather, f32>,       // Weather impact on insects
    pub temperature_thresholds: HashMap<InsectType, f32>, // Min temp for activity
}

#[derive(Resource, Default)]
pub struct FruitManager {
    pub fruit_trees: Vec<Entity>,
    pub ripening_schedule: HashMap<FruitType, f32>,
    pub seasonal_availability: HashMap<Season, Vec<FruitType>>,
    pub weather_ripening_effects: HashMap<Weather, f32>,
}

#[derive(Resource, Default)]
pub struct ForagingFlockManager {
    pub active_flocks: Vec<Entity>,
    pub formation_cooldown: f32,
    pub species_compatibility: HashMap<(BirdSpecies, BirdSpecies), f32>,
    pub leadership_hierarchy: HashMap<BirdSpecies, f32>,
}

#[derive(Resource, Default)]
pub struct OpportunisticFoodManager {
    pub active_opportunities: Vec<Entity>,
    pub spawn_probability: f32,
    pub discovery_radius: f32,
}

// Events
#[derive(Event)]
pub struct InsectEmergenceEvent {
    pub colony: Entity,
    pub insect_type: InsectType,
    pub population_size: u32,
    pub duration: f32,
}

#[derive(Event)]
pub struct FruitRipeningEvent {
    pub tree: Entity,
    pub fruit_type: FruitType,
    pub new_stage: RipeningStage,
    pub availability: f32,
}

#[derive(Event)]
pub struct MixedFlockFormationEvent {
    pub leader: Entity,
    pub members: Vec<Entity>,
    pub flock_type: FlockType,
}

#[derive(Event)]
pub struct OpportunisticFoodEvent {
    pub food_entity: Entity,
    pub food_type: OpportunisticFoodType,
    pub location: Vec3,
    pub discovery_bird: Option<Entity>,
}

impl BirdSpecies {
    pub fn insect_preference(&self) -> Vec<(InsectType, f32)> {
        match self {
            // Specialized insectivores
            Self::DownyWoodpecker | Self::HairyWoodpecker | Self::PileatedWoodpecker => {
                vec![(InsectType::TreeBark, 0.9), (InsectType::GroundDwelling, 0.3)]
            },
            Self::YellowWarbler | Self::BaltimoreOriole => {
                vec![(InsectType::Caterpillars, 0.8), (InsectType::Flying, 0.6), (InsectType::Aphids, 0.7)]
            },
            Self::RubyThroatedHummingbird => {
                vec![(InsectType::Flying, 0.5), (InsectType::Aphids, 0.3)] // Also nectar
            },
            // Generalist feeders
            Self::Robin | Self::BrownThrasher => {
                vec![(InsectType::GroundDwelling, 0.7), (InsectType::Caterpillars, 0.5)]
            },
            Self::BlueJay | Self::CommonCrow => {
                vec![(InsectType::GroundDwelling, 0.4), (InsectType::TreeBark, 0.3)]
            },
            // Less insectivorous species
            _ => vec![(InsectType::Flying, 0.3), (InsectType::GroundDwelling, 0.2)],
        }
    }

    pub fn fruit_preference(&self) -> Vec<(FruitType, f32)> {
        match self {
            // Fruit specialists
            Self::CedarWaxwing | Self::EuropeanStarling => {
                vec![(FruitType::Berries, 0.9), (FruitType::LargeFruit, 0.6)]
            },
            Self::BaltimoreOriole | Self::ScarletTanager => {
                vec![(FruitType::Berries, 0.7), (FruitType::Nectar, 0.5)]
            },
            Self::RubyThroatedHummingbird => {
                vec![(FruitType::Nectar, 0.95)]
            },
            // Nut and seed specialists
            Self::BlueJay | Self::CommonCrow => {
                vec![(FruitType::Nuts, 0.8), (FruitType::Seeds, 0.7), (FruitType::Berries, 0.4)]
            },
            Self::Goldfinch | Self::HouseFinch => {
                vec![(FruitType::Seeds, 0.9)]
            },
            // Generalists
            Self::Robin | Self::CommonGrackle => {
                vec![(FruitType::Berries, 0.6), (FruitType::Seeds, 0.5)]
            },
            _ => vec![(FruitType::Seeds, 0.4), (FruitType::Berries, 0.3)],
        }
    }

    pub fn flocking_tendency(&self) -> f32 {
        match self {
            // Highly social species
            Self::EuropeanStarling | Self::CommonGrackle | Self::RedWingedBlackbird => 0.9,
            Self::CommonCrow | Self::BlueJay => 0.8,
            Self::Goldfinch | Self::HouseFinch => 0.7,
            // Moderately social
            Self::Chickadee | Self::TuftedTitmouse | Self::WhiteBreastedNuthatch => 0.6,
            Self::Robin | Self::CedarWaxwing => 0.5,
            // Less social
            Self::Cardinal | Self::BrownThrasher => 0.3,
            // Solitary species
            Self::RubyThroatedHummingbird | Self::DownyWoodpecker => 0.1,
            _ => 0.4,
        }
    }

    pub fn leadership_potential(&self) -> f32 {
        match self {
            // Natural leaders
            Self::CommonCrow | Self::BlueJay => 0.9,
            Self::RedWingedBlackbird | Self::CommonGrackle => 0.7,
            Self::Cardinal | Self::Robin => 0.6,
            // Followers
            Self::Chickadee | Self::Goldfinch => 0.3,
            Self::HouseFinch | Self::Sparrow => 0.2,
            // Solitary
            Self::RubyThroatedHummingbird => 0.1,
            _ => 0.4,
        }
    }
}

// System implementations
fn setup_foraging_ecology(
    mut commands: Commands,
    mut insect_manager: ResMut<InsectManager>,
    mut fruit_manager: ResMut<FruitManager>,
    mut flock_manager: ResMut<ForagingFlockManager>,
) {
    // Setup insect colonies around the map
    let insect_locations = [
        (-350.0, 200.0, InsectType::TreeBark),
        (350.0, 200.0, InsectType::TreeBark),
        (-200.0, 300.0, InsectType::Aphids),
        (200.0, 300.0, InsectType::Caterpillars),
        (-100.0, -300.0, InsectType::GroundDwelling),
        (100.0, -300.0, InsectType::Flying),
        (0.0, 400.0, InsectType::Aquatic),
    ];

    for &(x, y, insect_type) in &insect_locations {
        let colony_entity = commands.spawn((
            Transform::from_xyz(x, y, 0.5),
            InsectColony {
                insect_type,
                population: rand::random::<u32>() % 500 + 100,
                emergence_cycle: match insect_type {
                    InsectType::Flying => EmergenceCycle::Daily,
                    InsectType::Aphids => EmergenceCycle::WeatherTriggered,
                    InsectType::Caterpillars => EmergenceCycle::Seasonal,
                    _ => EmergenceCycle::TemperatureBased,
                },
                activity_pattern: match insect_type {
                    InsectType::Flying => InsectActivityPattern::Dusk,
                    InsectType::Aphids => InsectActivityPattern::AllDay,
                    InsectType::TreeBark => InsectActivityPattern::Dawn,
                    _ => InsectActivityPattern::Midday,
                },
                last_emergence: 0.0,
                peak_activity_hour: match insect_type {
                    InsectType::Flying => 18.0,
                    InsectType::TreeBark => 6.0,
                    _ => 12.0,
                },
            },
        )).id();
        
        insect_manager.colonies.push(colony_entity);
    }

    // Setup fruit trees
    let fruit_locations = [
        (-300.0, 150.0, FruitType::Berries, SeasonalFruit::Summer),
        (300.0, 150.0, FruitType::Berries, SeasonalFruit::EarlyFall),
        (-150.0, 250.0, FruitType::Seeds, SeasonalFruit::LateFall),
        (150.0, 250.0, FruitType::Nuts, SeasonalFruit::EarlyFall),
        (0.0, 300.0, FruitType::Nectar, SeasonalFruit::LateSpring),
        (-250.0, -150.0, FruitType::LargeFruit, SeasonalFruit::Summer),
        (250.0, -150.0, FruitType::Berries, SeasonalFruit::LateSpring),
    ];

    for &(x, y, fruit_type, seasonal_cycle) in &fruit_locations {
        let tree_entity = commands.spawn((
            Transform::from_xyz(x, y, 1.0),
            FruitTree {
                fruit_type,
                ripening_stage: RipeningStage::Unripe,
                fruit_count: 0,
                max_fruit: match fruit_type {
                    FruitType::Berries => rand::random::<u32>() % 200 + 100,
                    FruitType::Nuts => rand::random::<u32>() % 50 + 25,
                    FruitType::LargeFruit => rand::random::<u32>() % 30 + 10,
                    FruitType::Seeds => rand::random::<u32>() % 500 + 200,
                    FruitType::Nectar => rand::random::<u32>() % 100 + 50,
                },
                ripening_progress: 0.0,
                seasonal_cycle,
            },
        )).id();
        
        fruit_manager.fruit_trees.push(tree_entity);
    }

    // Initialize species compatibility for mixed flocks
    let high_compatibility_pairs = [
        (BirdSpecies::Chickadee, BirdSpecies::TuftedTitmouse),
        (BirdSpecies::Chickadee, BirdSpecies::WhiteBreastedNuthatch),
        (BirdSpecies::Goldfinch, BirdSpecies::HouseFinch),
        (BirdSpecies::CommonGrackle, BirdSpecies::RedWingedBlackbird),
        (BirdSpecies::BlueJay, BirdSpecies::CommonCrow),
    ];

    for &(species1, species2) in &high_compatibility_pairs {
        flock_manager.species_compatibility.insert((species1, species2), 0.8);
        flock_manager.species_compatibility.insert((species2, species1), 0.8);
    }
}

fn insect_emergence_system(
    mut insect_query: Query<(Entity, &mut InsectColony, &Transform)>,
    weather_state: Res<WeatherState>,
    time_state: Res<TimeState>,
    mut insect_manager: ResMut<InsectManager>,
    mut emergence_events: EventWriter<InsectEmergenceEvent>,
    time: Res<Time>,
) {
    let current_hour = time_state.hour as f32;
    
    for (entity, mut colony, transform) in insect_query.iter_mut() {
        let should_emerge = match colony.emergence_cycle {
            EmergenceCycle::Daily => {
                (current_hour - colony.peak_activity_hour).abs() < 1.0 &&
                (time_state.hour as f32 - colony.last_emergence) > 22.0
            },
            EmergenceCycle::WeatherTriggered => {
                matches!(weather_state.current_weather, Weather::Clear | Weather::Cloudy) &&
                weather_state.temperature > 10.0
            },
            EmergenceCycle::TemperatureBased => {
                weather_state.temperature > insect_manager.temperature_thresholds
                    .get(&colony.insect_type).copied().unwrap_or(15.0)
            },
            EmergenceCycle::Seasonal => {
                // Simplified seasonal emergence
                matches!(time_state.get_season(), Season::Spring | Season::Summer)
            },
            _ => rand::random::<f32>() < 0.1, // 10% chance per update cycle
        };

        if should_emerge && colony.population > 10 {
            let emergence_size = (colony.population as f32 * rand::random::<f32>() * 0.3).round() as u32;
            colony.population = colony.population.saturating_sub(emergence_size);
            colony.last_emergence = time_state.hour as f32;

            emergence_events.write(InsectEmergenceEvent {
                colony: entity,
                insect_type: colony.insect_type,
                population_size: emergence_size,
                duration: match colony.insect_type {
                    InsectType::Flying => 120.0, // 2 hours
                    InsectType::Aphids => 480.0,  // 8 hours  
                    _ => 240.0, // 4 hours
                },
            });
        }

        // Slowly regenerate insect populations
        if colony.population < 500 {
            colony.population += (rand::random::<f32>() * 5.0) as u32;
        }
    }
}

fn fruit_ripening_system(
    mut fruit_query: Query<(Entity, &mut FruitTree, &Transform)>,
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    mut fruit_events: EventWriter<FruitRipeningEvent>,
    time: Res<Time>,
) {
    let current_season = time_state.get_season();
    
    for (entity, mut tree, _transform) in fruit_query.iter_mut() {
        // Check if it's the right season for this fruit
        let is_fruit_season = match (current_season, tree.seasonal_cycle) {
            (Season::Spring, SeasonalFruit::EarlySpring | SeasonalFruit::LateSpring) => true,
            (Season::Summer, SeasonalFruit::LateSpring | SeasonalFruit::Summer) => true,
            (Season::Fall, SeasonalFruit::Summer | SeasonalFruit::EarlyFall | SeasonalFruit::LateFall) => true,
            (Season::Winter, SeasonalFruit::Winter) => true,
            _ => false,
        };

        if !is_fruit_season {
            tree.ripening_stage = RipeningStage::Unripe;
            tree.fruit_count = 0;
            continue;
        }

        // Progress ripening based on weather and time
        let ripening_speed = match weather_state.current_weather {
            Weather::Clear => 1.0,
            Weather::Cloudy => 0.8,
            Weather::Rainy => 1.2, // Rain helps ripening
            Weather::Windy => 0.9,
            Weather::Snowy => 0.3,
        } * time.delta_secs();

        tree.ripening_progress += ripening_speed;

        let old_stage = tree.ripening_stage;
        
        // Update ripening stage
        tree.ripening_stage = match tree.ripening_progress {
            p if p < 100.0 => RipeningStage::Unripe,
            p if p < 200.0 => RipeningStage::HalfRipe,
            p if p < 300.0 => RipeningStage::Ripe,
            p if p < 400.0 => RipeningStage::Overripe,
            _ => RipeningStage::Rotting,
        };

        // Update fruit count based on stage
        tree.fruit_count = match tree.ripening_stage {
            RipeningStage::Unripe => (tree.max_fruit as f32 * 0.2) as u32,
            RipeningStage::HalfRipe => (tree.max_fruit as f32 * 0.6) as u32,
            RipeningStage::Ripe => tree.max_fruit,
            RipeningStage::Overripe => (tree.max_fruit as f32 * 0.8) as u32,
            RipeningStage::Rotting => (tree.max_fruit as f32 * 0.3) as u32,
        };

        // Emit event if stage changed
        if old_stage != tree.ripening_stage {
            fruit_events.write(FruitRipeningEvent {
                tree: entity,
                fruit_type: tree.fruit_type,
                new_stage: tree.ripening_stage,
                availability: tree.fruit_count as f32 / tree.max_fruit as f32,
            });
        }
    }
}

fn mixed_flock_formation_system(
    mut commands: Commands,
    bird_query: Query<(Entity, &Transform, &Bird, &Blackboard), With<BirdAI>>,
    mut flock_manager: ResMut<ForagingFlockManager>,
    mut flock_events: EventWriter<MixedFlockFormationEvent>,
    time: Res<Time>,
) {
    flock_manager.formation_cooldown -= time.delta_secs();
    
    if flock_manager.formation_cooldown > 0.0 || flock_manager.active_flocks.len() >= 5 {
        return;
    }

    // Find potential flock leaders
    let potential_leaders: Vec<_> = bird_query
        .iter()
        .filter(|(_, _, bird, blackboard)| {
            bird.species.leadership_potential() > 0.6 &&
            bird.species.flocking_tendency() > 0.5 &&
            blackboard.internal.fear < 0.3 // Not stressed
        })
        .collect();

    for (leader_entity, leader_transform, leader_bird, _) in potential_leaders.iter().take(2) {
        let mut flock_members = vec![*leader_entity];
        
        // Find compatible birds nearby
        for (other_entity, other_transform, other_bird, other_blackboard) in bird_query.iter() {
            if other_entity == *leader_entity || flock_members.len() >= 8 {
                continue;
            }

            let distance = leader_transform.translation.distance(other_transform.translation);
            if distance > 200.0 {
                continue;
            }

            // Check species compatibility
            let compatibility = flock_manager.species_compatibility
                .get(&(leader_bird.species, other_bird.species))
                .copied()
                .unwrap_or(0.3);

            let flocking_willingness = other_bird.species.flocking_tendency() * 
                                     (1.0 - other_blackboard.internal.fear) *
                                     compatibility;

            if rand::random::<f32>() < flocking_willingness * 0.3 {
                flock_members.push(other_entity);
            }
        }

        // Create flock if we have enough members
        if flock_members.len() >= 3 {
            let flock_entity = commands.spawn(MixedForagingFlock {
                leader: *leader_entity,
                core_members: flock_members[..2].to_vec(),
                followers: flock_members[2..].to_vec(),
                flock_type: FlockType::MixedSpecies,
                formation_time: 0.0,
                cohesion_strength: 0.8,
                foraging_efficiency: 1.0 + (flock_members.len() as f32 * 0.1),
            }).id();

            // Add FlockLeader component to leader
            commands.entity(*leader_entity).insert(FlockLeader {
                leadership_strength: leader_bird.species.leadership_potential(),
                experience: rand::random::<f32>(),
                territory_knowledge: rand::random::<f32>(),
                followers: flock_members[1..].to_vec(),
            });

            // Add FlockFollower components to followers
            for &follower in &flock_members[1..] {
                commands.entity(follower).insert(FlockFollower {
                    following_tendency: 0.8,
                    current_leader: Some(*leader_entity),
                    benefit_received: 0.0,
                });
            }

            flock_manager.active_flocks.push(flock_entity);
            
            flock_events.write(MixedFlockFormationEvent {
                leader: *leader_entity,
                members: flock_members,
                flock_type: FlockType::MixedSpecies,
            });
        }
    }

    flock_manager.formation_cooldown = 30.0; // 30 second cooldown between formations
}

fn mixed_flock_behavior_system(
    mut bird_query: Query<(&mut Transform, &mut BirdState, &mut Blackboard), With<BirdAI>>,
    leader_query: Query<&FlockLeader>,
    follower_query: Query<&FlockFollower>,
    mut flock_query: Query<&mut MixedForagingFlock>,
    time: Res<Time>,
) {
    for mut flock in flock_query.iter_mut() {
        flock.formation_time += time.delta_secs();
        
        // Update cohesion based on flock age and success
        if flock.formation_time > 300.0 { // 5 minutes
            flock.cohesion_strength *= 0.99; // Slowly decrease over time
        }

        // Process leader behavior
        if let Ok((mut leader_transform, mut leader_state, mut leader_blackboard)) = bird_query.get_mut(flock.leader) {
            if let Ok(_leader_component) = leader_query.get(flock.leader) {
                // Leaders are more confident and explore more
                leader_blackboard.internal.fear *= 0.8;
                
                // Leader foraging behavior - more systematic
                if matches!(*leader_state, BirdState::Wandering | BirdState::Foraging) {
                    *leader_state = BirdState::Foraging;
                }
            }
        }

        // Process follower behavior
        let leader_position = bird_query.get(flock.leader)
            .map(|(transform, _, _)| transform.translation)
            .unwrap_or(Vec3::ZERO);

        for &follower in flock.followers.iter().chain(flock.core_members.iter()) {
            if follower == flock.leader {
                continue;
            }

            if let Ok((mut follower_transform, mut follower_state, mut follower_blackboard)) = bird_query.get_mut(follower) {
                let distance_to_leader = follower_transform.translation.distance(leader_position);
                
                // Followers stay close to leader
                if distance_to_leader > 100.0 {
                    let direction = (leader_position - follower_transform.translation).normalize();
                    let move_speed = 80.0 * time.delta_secs();
                    follower_transform.translation += direction * move_speed;
                }

                // Followers benefit from reduced fear and increased foraging efficiency
                follower_blackboard.internal.fear *= 0.9;
                if matches!(*follower_state, BirdState::Wandering) {
                    *follower_state = BirdState::Following;
                }

                // Followers learn from leader's foraging success
                if let Ok(follower_component) = follower_query.get(follower) {
                    // Increase follower's benefit over time
                    // This would be used to make followers more likely to join future flocks
                }
            }
        }

        // Disperse flock if cohesion gets too low
        if flock.cohesion_strength < 0.3 {
            // Mark flock for removal (would need a despawn system)
            for &member in flock.core_members.iter().chain(flock.followers.iter()) {
                // Remove flock components from members
                // This would be implemented with proper entity commands
            }
        }
    }
}

fn leader_follower_dynamics_system(
    mut bird_query: Query<(Entity, &Transform, &Bird, &mut Blackboard), With<BirdAI>>,
    leader_query: Query<&FlockLeader>,
    mut follower_query: Query<&mut FlockFollower>,
    time: Res<Time>,
) {
    // Collect leader info first to avoid borrowing conflicts
    let mut leader_info: Vec<(Entity, Vec3, bool, f32)> = Vec::new();
    
    for (leader_entity, leader_transform, _leader_bird, leader_blackboard) in bird_query.iter() {
        if leader_query.get(leader_entity).is_ok() {
            leader_info.push((
                leader_entity,
                leader_transform.translation,
                leader_blackboard.internal.fear < 0.3,
                leader_blackboard.internal.hunger,
            ));
        }
    }
    
    // Now process followers
    for (leader_entity, leader_pos, leader_confident, leader_hunger) in leader_info {
        if let Ok(leader_component) = leader_query.get(leader_entity) {
            for &follower_entity in &leader_component.followers {
                if let Ok(mut follower_component) = follower_query.get_mut(follower_entity) {
                    if let Ok((_follower_entity, follower_transform, _follower_bird, mut follower_blackboard)) = bird_query.get_mut(follower_entity) {
                        let distance = leader_pos.distance(follower_transform.translation);
                        
                        // Followers benefit from being near experienced leaders
                        if distance < 80.0 {
                            follower_component.benefit_received += time.delta_secs() * 0.1;
                            
                            // Transfer some of leader's confidence
                            if leader_confident {
                                follower_blackboard.internal.fear *= 0.95;
                            }

                            // Followers learn foraging locations from leaders
                            if leader_hunger < 0.3 { // Leader recently fed successfully
                                follower_blackboard.internal.hunger *= 0.98; // Slight hunger reduction from learning
                            }
                        }
                        
                        // Followers may become more independent over time
                        if follower_component.benefit_received > 100.0 {
                            follower_component.following_tendency *= 0.99;
                        }
                    }
                }
            }
        }
    }
}

fn insectivore_behavior_system(
    mut bird_query: Query<(&mut Transform, &mut BirdState, &mut Blackboard, &Bird), With<BirdAI>>,
    insect_query: Query<(Entity, &Transform, &InsectColony)>,
    mut emergence_events: EventReader<InsectEmergenceEvent>,
    time: Res<Time>,
) {
    // Process insect emergence events and attract insectivores
    for event in emergence_events.read() {
        if let Ok((_, colony_transform, _)) = insect_query.get(event.colony) {
            let insect_attraction_radius = 200.0;
            
            for (mut bird_transform, mut bird_state, mut blackboard, bird) in bird_query.iter_mut() {
                let distance = bird_transform.translation.distance(colony_transform.translation);
                
                if distance > insect_attraction_radius {
                    continue;
                }

                // Check if this species is interested in this insect type
                let preferences = bird.species.insect_preference();
                let interest_level = preferences.iter()
                    .find(|(insect_type, _)| *insect_type == event.insect_type)
                    .map(|(_, preference)| *preference)
                    .unwrap_or(0.1);

                if interest_level > 0.4 && blackboard.internal.hunger > 0.3 {
                    // Bird is attracted to the insect emergence
                    let direction = (colony_transform.translation - bird_transform.translation).normalize();
                    let move_speed = 60.0 * interest_level * time.delta_secs();
                    
                    bird_transform.translation += direction * move_speed;
                    *bird_state = BirdState::Foraging;
                    
                    // Reduce hunger if close enough to "feed"
                    if distance < 50.0 {
                        blackboard.internal.hunger = (blackboard.internal.hunger - interest_level * time.delta_secs() * 0.5).max(0.0);
                    }
                }
            }
        }
    }
}

fn frugivore_behavior_system(
    mut bird_query: Query<(&mut Transform, &mut BirdState, &mut Blackboard, &Bird), With<BirdAI>>,
    fruit_query: Query<(Entity, &Transform, &FruitTree)>,
    mut ripening_events: EventReader<FruitRipeningEvent>,
    time: Res<Time>,
) {
    // Process fruit ripening events and attract frugivores
    for event in ripening_events.read() {
        // Only attract birds to ripe and overripe fruit
        if !matches!(event.new_stage, RipeningStage::Ripe | RipeningStage::Overripe) {
            continue;
        }

        if let Ok((_, fruit_transform, fruit_tree)) = fruit_query.get(event.tree) {
            let fruit_attraction_radius = 150.0;
            
            for (mut bird_transform, mut bird_state, mut blackboard, bird) in bird_query.iter_mut() {
                let distance = bird_transform.translation.distance(fruit_transform.translation);
                
                if distance > fruit_attraction_radius || fruit_tree.fruit_count == 0 {
                    continue;
                }

                // Check if this species is interested in this fruit type
                let preferences = bird.species.fruit_preference();
                let interest_level = preferences.iter()
                    .find(|(fruit_type, _)| *fruit_type == event.fruit_type)
                    .map(|(_, preference)| *preference)
                    .unwrap_or(0.1);

                if interest_level > 0.4 && blackboard.internal.hunger > 0.2 {
                    // Bird is attracted to the ripe fruit
                    let direction = (fruit_transform.translation - bird_transform.translation).normalize();
                    let move_speed = 50.0 * interest_level * time.delta_secs();
                    
                    bird_transform.translation += direction * move_speed;
                    *bird_state = BirdState::Foraging;
                    
                    // Reduce hunger if close enough to "feed"
                    if distance < 40.0 {
                        blackboard.internal.hunger = (blackboard.internal.hunger - interest_level * time.delta_secs() * 0.3).max(0.0);
                    }
                }
            }
        }
    }
}

fn opportunistic_feeding_system(
    mut commands: Commands,
    bird_query: Query<(Entity, &Transform, &Bird), With<BirdAI>>,
    mut opp_food_query: Query<(Entity, &mut OpportunisticFood, &Transform)>,
    mut opp_manager: ResMut<OpportunisticFoodManager>,
    mut opp_events: EventWriter<OpportunisticFoodEvent>,
    time: Res<Time>,
) {
    // Spawn new opportunistic food sources randomly
    if rand::random::<f32>() < 0.005 { // 0.5% chance per update
        let food_types = [
            OpportunisticFoodType::SpilledSeed,
            OpportunisticFoodType::HumanFood,
            OpportunisticFoodType::InsectSwarm,
            OpportunisticFoodType::FlowerNectar,
            OpportunisticFoodType::SapFlow,
        ];

        let food_type = food_types[(rand::random::<f32>() * food_types.len() as f32) as usize];
        let location = Vec3::new(
            (rand::random::<f32>() - 0.5) * 800.0,
            (rand::random::<f32>() - 0.5) * 600.0,
            0.5,
        );

        let attracted_species = match food_type {
            OpportunisticFoodType::SpilledSeed => vec![
                BirdSpecies::Sparrow, BirdSpecies::Goldfinch, BirdSpecies::HouseFinch,
                BirdSpecies::Cardinal, BirdSpecies::CommonGrackle
            ],
            OpportunisticFoodType::HumanFood => vec![
                BirdSpecies::CommonCrow, BirdSpecies::BlueJay, BirdSpecies::EuropeanStarling
            ],
            OpportunisticFoodType::InsectSwarm => vec![
                BirdSpecies::YellowWarbler, BirdSpecies::Robin, BirdSpecies::BrownThrasher
            ],
            OpportunisticFoodType::FlowerNectar => vec![
                BirdSpecies::RubyThroatedHummingbird, BirdSpecies::BaltimoreOriole
            ],
            OpportunisticFoodType::SapFlow => vec![
                BirdSpecies::DownyWoodpecker, BirdSpecies::YellowBelledSapsucker
            ],
            _ => vec![],
        };

        let food_entity = commands.spawn((
            Transform::from_translation(location),
            OpportunisticFood {
                food_type,
                quantity: rand::random::<f32>() * 100.0 + 50.0,
                discovery_time: 0.0,
                decay_rate: match food_type {
                    OpportunisticFoodType::InsectSwarm => 0.1,  // Decays quickly
                    OpportunisticFoodType::FlowerNectar => 0.05, // Decays slowly
                    _ => 0.02,
                },
                attracts_species: attracted_species,
            },
        )).id();

        opp_manager.active_opportunities.push(food_entity);
        
        opp_events.write(OpportunisticFoodEvent {
            food_entity,
            food_type,
            location,
            discovery_bird: None,
        });
    }

    // Process existing opportunistic food sources
    for (food_entity, mut opp_food, food_transform) in opp_food_query.iter_mut() {
        opp_food.discovery_time += time.delta_secs();
        opp_food.quantity -= opp_food.decay_rate * time.delta_secs();

        if opp_food.quantity <= 0.0 {
            commands.entity(food_entity).despawn();
            opp_manager.active_opportunities.retain(|&e| e != food_entity);
            continue;
        }

        // Check for birds discovering the food source
        for (bird_entity, bird_transform, bird_species) in bird_query.iter() {
            let distance = bird_transform.translation.distance(food_transform.translation);
            
            if distance < opp_manager.discovery_radius &&
               opp_food.attracts_species.contains(&bird_species.species) {
                
                opp_events.write(OpportunisticFoodEvent {
                    food_entity,
                    food_type: opp_food.food_type,
                    location: food_transform.translation,
                    discovery_bird: Some(bird_entity),
                });
            }
        }
    }
}

fn seasonal_food_availability_system(
    time_state: Res<TimeState>,
    mut fruit_query: Query<&mut FruitTree>,
    mut insect_query: Query<&mut InsectColony>,
) {
    let current_season = time_state.get_season();
    
    // Adjust insect populations based on season
    for mut colony in insect_query.iter_mut() {
        let seasonal_modifier = match (current_season, colony.insect_type) {
            (Season::Spring, InsectType::Aphids) => 2.0,
            (Season::Summer, InsectType::Flying | InsectType::Caterpillars) => 1.5,
            (Season::Fall, InsectType::GroundDwelling) => 1.2,
            (Season::Winter, _) => 0.5,
            _ => 1.0,
        };

        if colony.population < 1000 {
            colony.population = ((colony.population as f32 * seasonal_modifier) as u32).min(1000);
        }
    }
    
    // Seasonal fruit availability is handled in the fruit_ripening_system
}
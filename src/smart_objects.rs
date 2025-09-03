use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::bird_ai::components::{SmartObject, ProvidesUtility, BirdAction, BirdState};
use crate::environment::components::Season;
use crate::bird::BirdSpecies;
use crate::sanctuary_management::{NestingBox, NestingStatus, PredatorDeterrent, DeterrentType};
use crate::tooltip::Hoverable;

pub struct SmartObjectsPlugin;

impl Plugin for SmartObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_smart_objects)
            .add_systems(Update, (
                perch_interaction_system,
                shelter_usage_system,
                water_feature_system,
                seasonal_object_system,
                sanctuary_interaction_system,
                predator_deterrent_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// === PERCHING STRUCTURES ===

#[derive(Component)]
pub struct PerchingSpot {
    pub perch_type: PerchType,
    pub comfort_level: f32,     // How appealing this perch is
    pub occupant: Option<Entity>, // Current bird using this perch
    pub species_preference: Vec<BirdSpecies>, // Which species prefer this perch
}

#[derive(Debug, Clone, Copy)]
pub enum PerchType {
    Branch,      // Natural tree branch
    Post,        // Artificial perching post
    Wire,        // Utility wire or fence
    Roof,        // Building edge or roof
    Rock,        // Natural stone perch
}

impl PerchType {
    pub fn base_utility(&self) -> f32 {
        match self {
            Self::Branch => 0.8,  // Most natural and preferred
            Self::Post => 0.6,    // Functional but artificial
            Self::Wire => 0.4,    // Some species love wires, others avoid
            Self::Roof => 0.5,    // Good vantage point
            Self::Rock => 0.7,    // Natural and stable
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Self::Branch => Color::srgb(0.4, 0.3, 0.2),
            Self::Post => Color::srgb(0.6, 0.5, 0.4),
            Self::Wire => Color::srgb(0.2, 0.2, 0.2),
            Self::Roof => Color::srgb(0.5, 0.4, 0.4),
            Self::Rock => Color::srgb(0.4, 0.4, 0.4),
        }
    }
}

// === SHELTER STRUCTURES ===

#[derive(Component)]
pub struct Shelter {
    pub shelter_type: ShelterType,
    pub capacity: u32,           // How many birds can shelter here
    pub current_occupants: Vec<Entity>,
    pub weather_protection: f32, // 0.0-1.0, how much weather protection
    pub concealment: f32,        // 0.0-1.0, protection from predators
}

#[derive(Debug, Clone, Copy)]
pub enum ShelterType {
    BushCluster,    // Dense bushes for hiding
    TreeCanopy,     // Overhead coverage
    BirdHouse,      // Artificial shelter
    RockCrevice,    // Natural rock formation
    Undergrowth,    // Low vegetation cover
}

impl ShelterType {
    pub fn base_capacity(&self) -> u32 {
        match self {
            Self::BushCluster => 4,
            Self::TreeCanopy => 8,
            Self::BirdHouse => 2,
            Self::RockCrevice => 3,
            Self::Undergrowth => 6,
        }
    }
    
    pub fn weather_protection(&self) -> f32 {
        match self {
            Self::BushCluster => 0.7,
            Self::TreeCanopy => 0.9,
            Self::BirdHouse => 0.95,
            Self::RockCrevice => 0.8,
            Self::Undergrowth => 0.6,
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Self::BushCluster => Color::srgb(0.2, 0.5, 0.2),
            Self::TreeCanopy => Color::srgb(0.1, 0.4, 0.1),
            Self::BirdHouse => Color::srgb(0.6, 0.4, 0.2),
            Self::RockCrevice => Color::srgb(0.4, 0.4, 0.4),
            Self::Undergrowth => Color::srgb(0.3, 0.4, 0.2),
        }
    }
}

// === WATER FEATURES ===

#[derive(Component)]
pub struct WaterFeature {
    pub water_type: WaterType,
    pub water_level: f32,        // Current water level (0.0-1.0)
    pub cleanliness: f32,        // How clean the water is
    pub temperature: f32,        // Water temperature (affects appeal)
    pub flow_rate: f32,          // For moving water features
}

#[derive(Debug, Clone, Copy)]
pub enum WaterType {
    BirdBath,      // Shallow bathing dish
    Fountain,      // Moving water feature
    Pond,          // Large still water
    Creek,         // Natural flowing water
    DrippingTap,   // Artificial drip water source
}

impl WaterType {
    pub fn base_utility(&self) -> f32 {
        match self {
            Self::BirdBath => 0.9,     // Perfect for birds
            Self::Fountain => 0.8,     // Moving water is attractive
            Self::Pond => 0.7,         // Good but might be too deep for some
            Self::Creek => 0.85,       // Natural flowing water
            Self::DrippingTap => 0.6,  // Artificial but provides fresh water
        }
    }
    
    pub fn provides_actions(&self) -> Vec<BirdAction> {
        match self {
            Self::BirdBath => vec![BirdAction::Drink, BirdAction::Bathe],
            Self::Fountain => vec![BirdAction::Drink, BirdAction::Bathe],
            Self::Pond => vec![BirdAction::Drink], // Too deep for bathing
            Self::Creek => vec![BirdAction::Drink, BirdAction::Bathe],
            Self::DrippingTap => vec![BirdAction::Drink],
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Self::BirdBath => Color::srgb(0.7, 0.8, 0.9),
            Self::Fountain => Color::srgb(0.6, 0.7, 0.9),
            Self::Pond => Color::srgb(0.4, 0.6, 0.8),
            Self::Creek => Color::srgb(0.5, 0.7, 0.8),
            Self::DrippingTap => Color::srgb(0.6, 0.6, 0.7),
        }
    }
}

// === ENHANCED BIRD ACTIONS ===

// Extend BirdAction enum for new smart object interactions
impl BirdAction {
    pub fn utility_decay_rate(&self) -> f32 {
        match self {
            Self::Eat => 0.1,     // Eating utility decreases as hunger decreases
            Self::Drink => 0.08,  // Thirst satisfaction
            Self::Bathe => 0.05,  // Bathing need decreases slowly
            Self::Perch => 0.02,  // Resting need decreases very slowly
            Self::Play => 0.03,   // Play behavior has moderate decay
            Self::Explore => 0.04, // Curiosity satisfaction
            Self::Nest => 0.01,   // Nesting instinct persists longest
            Self::Roost => 0.015, // Roosting instinct persists during evening hours
            Self::Shelter => 0.08, // Shelter need decays moderately based on weather
            Self::Court => 0.06,  // Courtship behavior moderate decay
            Self::Follow => 0.05, // Following behavior moderate decay
            Self::Challenge => 0.12, // Territorial challenge high decay (aggressive behavior)
            Self::Flock => 0.03,  // Flocking behavior low decay (social need)
            Self::Forage => 0.08, // Foraging behavior moderate decay
            Self::Cache => 0.04,  // Caching behavior low decay (important survival behavior)
            Self::Retrieve => 0.06, // Retrieval behavior moderate decay
            Self::HoverFeed => 0.1, // Hover feeding high energy cost, higher decay
        }
    }
    
    pub fn duration_range(&self) -> (f32, f32) {
        match self {
            Self::Eat => (3.0, 8.0),     // 3-8 seconds feeding
            Self::Drink => (2.0, 5.0),   // 2-5 seconds drinking
            Self::Bathe => (8.0, 15.0),  // 8-15 seconds bathing
            Self::Perch => (10.0, 30.0), // 10-30 seconds resting
            Self::Play => (5.0, 12.0),   // 5-12 seconds playing
            Self::Explore => (4.0, 10.0), // 4-10 seconds investigating
            Self::Nest => (15.0, 45.0),  // 15-45 seconds in nesting area
            Self::Roost => (30.0, 90.0), // 30-90 seconds roosting/gathering
            Self::Shelter => (60.0, 180.0), // 1-3 minutes sheltering from weather
            Self::Court => (10.0, 25.0), // 10-25 seconds courtship display
            Self::Follow => (5.0, 20.0), // 5-20 seconds following behavior
            Self::Challenge => (3.0, 15.0), // 3-15 seconds territorial display
            Self::Flock => (20.0, 60.0), // 20-60 seconds flocking behavior
            Self::Forage => (15.0, 45.0), // 15-45 seconds ground foraging
            Self::Cache => (5.0, 12.0), // 5-12 seconds caching food
            Self::Retrieve => (3.0, 8.0), // 3-8 seconds retrieving cached food
            Self::HoverFeed => (8.0, 20.0), // 8-20 seconds hover feeding
        }
    }
}

// === SETUP SYSTEMS ===

pub fn setup_smart_objects(mut commands: Commands) {
    // Spawn various perching spots
    spawn_perching_spots(&mut commands);
    
    // Spawn shelter areas
    spawn_shelter_areas(&mut commands);
    
    // Spawn water features
    spawn_water_features(&mut commands);
    
    // Phase 4 sanctuary management objects (TODO: implement properly)
}

fn spawn_perching_spots(commands: &mut Commands) {
    // Tree branch - high appeal for most species
    let branch_perch = PerchType::Branch;
    commands.spawn((
        Sprite::from_color(branch_perch.color(), Vec2::new(80.0, 15.0)),
        Transform::from_xyz(-200.0, 150.0, 0.3),
        RigidBody::Fixed,
        Collider::cuboid(40.0, 7.5),
        Sensor,
        PerchingSpot {
            perch_type: branch_perch,
            comfort_level: 0.8,
            occupant: None,
            species_preference: vec![
                BirdSpecies::Cardinal, BirdSpecies::BlueJay, BirdSpecies::Robin,
                BirdSpecies::NorthernMockingbird, BirdSpecies::BrownThrasher,
            ],
        },
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Perch,
            base_utility: branch_perch.base_utility(),
            range: 80.0,
        },
        Hoverable::new("Natural tree branch - favored by cardinals, jays, and songbirds"),
    ));
    
    // Utility wire - some species love this
    let wire_perch = PerchType::Wire;
    commands.spawn((
        Sprite::from_color(wire_perch.color(), Vec2::new(150.0, 3.0)),
        Transform::from_xyz(50.0, 200.0, 0.3),
        RigidBody::Fixed,
        Collider::cuboid(75.0, 1.5),
        Sensor,
        PerchingSpot {
            perch_type: wire_perch,
            comfort_level: 0.6,
            occupant: None,
            species_preference: vec![
                BirdSpecies::EuropeanStarling, BirdSpecies::CommonGrackle,
                BirdSpecies::RedWingedBlackbird, BirdSpecies::BaltimoreOriole,
            ],
        },
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Perch,
            base_utility: wire_perch.base_utility(),
            range: 60.0,
        },
        Hoverable::new("Utility wire - preferred by starlings, grackles, and blackbirds"),
    ));
    
    // Natural rock perch
    let rock_perch = PerchType::Rock;
    commands.spawn((
        Sprite::from_color(rock_perch.color(), Vec2::new(60.0, 30.0)),
        Transform::from_xyz(180.0, -100.0, 0.3),
        RigidBody::Fixed,
        Collider::cuboid(30.0, 15.0),
        Sensor,
        PerchingSpot {
            perch_type: rock_perch,
            comfort_level: 0.7,
            occupant: None,
            species_preference: vec![
                BirdSpecies::Robin, BirdSpecies::BrownThrasher,
                BirdSpecies::NorthernMockingbird, BirdSpecies::RedTailedHawk,
            ],
        },
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Perch,
            base_utility: rock_perch.base_utility(),
            range: 70.0,
        },
        Hoverable::new("Natural rock outcrop - good vantage point for ground feeders and raptors"),
    ));
}

fn spawn_shelter_areas(commands: &mut Commands) {
    // Dense bush cluster
    let bush_shelter = ShelterType::BushCluster;
    commands.spawn((
        Sprite::from_color(bush_shelter.color(), Vec2::new(120.0, 80.0)),
        Transform::from_xyz(-150.0, -120.0, 0.2),
        RigidBody::Fixed,
        Collider::cuboid(60.0, 40.0),
        Sensor,
        Shelter {
            shelter_type: bush_shelter,
            capacity: bush_shelter.base_capacity(),
            current_occupants: Vec::new(),
            weather_protection: bush_shelter.weather_protection(),
            concealment: 0.8,
        },
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Perch, // Shelter provides resting/hiding
            base_utility: 0.6,
            range: 100.0,
        },
    ));
    
    // Tree canopy for overhead shelter
    let canopy_shelter = ShelterType::TreeCanopy;
    commands.spawn((
        Sprite::from_color(canopy_shelter.color(), Vec2::new(200.0, 150.0)),
        Transform::from_xyz(0.0, 180.0, 0.1),
        RigidBody::Fixed,
        Collider::cuboid(100.0, 75.0),
        Sensor,
        Shelter {
            shelter_type: canopy_shelter,
            capacity: canopy_shelter.base_capacity(),
            current_occupants: Vec::new(),
            weather_protection: canopy_shelter.weather_protection(),
            concealment: 0.6,
        },
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Perch,
            base_utility: 0.7,
            range: 150.0,
        },
    ));
    
    // Bird house for cavity nesters
    let house_shelter = ShelterType::BirdHouse;
    commands.spawn((
        Sprite::from_color(house_shelter.color(), Vec2::new(25.0, 35.0)),
        Transform::from_xyz(120.0, 100.0, 0.4),
        RigidBody::Fixed,
        Collider::cuboid(12.5, 17.5),
        Sensor,
        Shelter {
            shelter_type: house_shelter,
            capacity: house_shelter.base_capacity(),
            current_occupants: Vec::new(),
            weather_protection: house_shelter.weather_protection(),
            concealment: 0.9,
        },
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Perch,
            base_utility: 0.75,
            range: 50.0,
        },
    ));
}

fn spawn_water_features(commands: &mut Commands) {
    // Classic bird bath
    let bird_bath = WaterType::BirdBath;
    for action in bird_bath.provides_actions() {
        commands.spawn((
            Sprite::from_color(bird_bath.color(), Vec2::new(50.0, 50.0)),
            Transform::from_xyz(-80.0, -50.0, 0.3),
            RigidBody::Fixed,
            Collider::cuboid(25.0, 25.0),
            Sensor,
            WaterFeature {
                water_type: bird_bath,
                water_level: 0.8,
                cleanliness: 1.0,
                temperature: 20.0, // Celsius
                flow_rate: 0.0,    // Still water
            },
            SmartObject,
            ProvidesUtility {
                action,
                base_utility: bird_bath.base_utility(),
                range: 80.0,
            },
            Hoverable::new("Bird bath - essential for drinking and bathing. Attracts all species"),
        ));
    }
    
    // Fountain with moving water
    let fountain = WaterType::Fountain;
    for action in fountain.provides_actions() {
        commands.spawn((
            Sprite::from_color(fountain.color(), Vec2::new(60.0, 60.0)),
            Transform::from_xyz(200.0, 50.0, 0.3),
            RigidBody::Fixed,
            Collider::cuboid(30.0, 30.0),
            Sensor,
            WaterFeature {
                water_type: fountain,
                water_level: 1.0,
                cleanliness: 0.95,
                temperature: 18.0,
                flow_rate: 0.3, // Gentle flow
            },
            SmartObject,
            ProvidesUtility {
                action,
                base_utility: fountain.base_utility(),
                range: 100.0,
            },
            Hoverable::new("Decorative fountain - moving water attracts birds from greater distances"),
        ));
    }
    
    // Natural creek
    let creek = WaterType::Creek;
    for action in creek.provides_actions() {
        commands.spawn((
            Sprite::from_color(creek.color(), Vec2::new(180.0, 40.0)),
            Transform::from_xyz(-250.0, 0.0, 0.2),
            RigidBody::Fixed,
            Collider::cuboid(90.0, 20.0),
            Sensor,
            WaterFeature {
                water_type: creek,
                water_level: 0.9,
                cleanliness: 0.9,
                temperature: 15.0,
                flow_rate: 0.5, // Natural flow
            },
            SmartObject,
            ProvidesUtility {
                action,
                base_utility: creek.base_utility(),
                range: 120.0,
            },
        ));
    }
}

// === INTERACTION SYSTEMS ===

pub fn perch_interaction_system(
    mut perch_query: Query<(&Transform, &mut PerchingSpot)>,
    bird_query: Query<(Entity, &Transform, &BirdState), With<crate::bird_ai::components::BirdAI>>,
) {
    // Update perch occupancy
    for (perch_transform, mut perch) in &mut perch_query {
        let mut current_occupant = None;
        
        // Find birds currently perching at this spot
        for (bird_entity, bird_transform, bird_state) in &bird_query {
            if matches!(bird_state, BirdState::Resting) {
                let distance = perch_transform.translation.distance(bird_transform.translation);
                if distance < 30.0 { // Bird is close enough to be "on" the perch
                    current_occupant = Some(bird_entity);
                    break;
                }
            }
        }
        
        perch.occupant = current_occupant;
    }
}

pub fn shelter_usage_system(
    mut shelter_query: Query<(&Transform, &mut Shelter)>,
    bird_query: Query<(Entity, &Transform, &BirdState), With<crate::bird_ai::components::BirdAI>>,
    weather_state: Res<crate::environment::resources::WeatherState>,
) {
    for (shelter_transform, mut shelter) in &mut shelter_query {
        shelter.current_occupants.clear();
        
        // Find birds taking shelter
        for (bird_entity, bird_transform, bird_state) in &bird_query {
            // Birds seek shelter when fleeing or during bad weather
            let seeking_shelter = matches!(bird_state, BirdState::Fleeing | BirdState::Resting) ||
                                 weather_state.current_weather.bird_activity_modifier() < 0.6;
                                 
            if seeking_shelter {
                let distance = shelter_transform.translation.distance(bird_transform.translation);
                if distance < 80.0 && shelter.current_occupants.len() < shelter.capacity as usize {
                    shelter.current_occupants.push(bird_entity);
                }
            }
        }
    }
}

pub fn water_feature_system(
    mut water_query: Query<(&Transform, &mut WaterFeature)>,
    time: Res<Time>,
    weather_state: Res<crate::environment::resources::WeatherState>,
) {
    for (_water_transform, mut water) in &mut water_query {
        // Water evaporation and refilling from rain
        match weather_state.current_weather {
            crate::environment::components::Weather::Rainy => {
                water.water_level = (water.water_level + 0.1 * time.delta().as_secs_f32()).min(1.0);
                water.cleanliness = (water.cleanliness + 0.05 * time.delta().as_secs_f32()).min(1.0);
            },
            crate::environment::components::Weather::Clear => {
                water.water_level = (water.water_level - 0.02 * time.delta().as_secs_f32()).max(0.2);
            },
            _ => {}
        }
        
        // Water temperature follows seasonal patterns (simplified)
        // This would ideally be integrated with the environment system
        water.temperature = 15.0 + (water.temperature - 15.0) * 0.95; // Gradual temperature normalization
    }
}

// === PHASE 4 SANCTUARY MANAGEMENT === (TEMPORARILY DISABLED)
/*
fn spawn_sanctuary_objects(commands: &mut Commands) {
    // Weather shelter - insulated hut for cold protection
    let weather_shelter = WeatherShelter {
        shelter_type: SanctuaryShelterType::InsulatedHut,
        capacity: 8,
        current_occupancy: 0,
        weather_protection: vec![
            crate::environment::components::Weather::Snowy,
            crate::environment::components::Weather::Rainy,
            crate::environment::components::Weather::Windy,
        ],
        comfort_level: 0.9,
        maintenance_level: 1.0,
    };
    
    commands.spawn((
        Sprite::from_color(Color::srgb(0.5, 0.3, 0.2), Vec2::new(100.0, 80.0)),
        Transform::from_xyz(100.0, -150.0, 0.4),
        RigidBody::Fixed,
        Collider::cuboid(50.0, 40.0),
        Sensor,
        weather_shelter,
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Shelter,
            base_utility: 0.8,
            range: 90.0,
        },
    ));
    
    // Nesting boxes for different species
    let small_nesting_box = NestingBox {
        box_type: NestingBoxType::SmallCavity,
        target_species: vec![BirdSpecies::Chickadee, BirdSpecies::CarolinaWren],
        occupancy_status: NestingStatus::Empty,
        breeding_season: vec![Season::Spring, Season::Summer],
        success_rate: 0.7,
        maintenance_required: false,
        eggs_laid: 0,
        fledglings_raised: 0,
    };
    
    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.4, 0.2), Vec2::new(20.0, 25.0)),
        Transform::from_xyz(-100.0, 80.0, 0.4),
        RigidBody::Fixed,
        Collider::cuboid(10.0, 12.5),
        Sensor,
        small_nesting_box,
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Nest,
            base_utility: 0.9,
            range: 40.0,
        },
    ));
    
    // Open platform nesting box for Cardinals/Robins
    let platform_nesting_box = NestingBox {
        box_type: NestingBoxType::OpenPlatform,
        target_species: vec![BirdSpecies::Cardinal, BirdSpecies::Robin],
        occupancy_status: NestingStatus::Empty,
        breeding_season: vec![Season::Spring, Season::Summer],
        success_rate: 0.6,
        maintenance_required: false,
        eggs_laid: 0,
        fledglings_raised: 0,
    };
    
    commands.spawn((
        Sprite::from_color(Color::srgb(0.7, 0.5, 0.3), Vec2::new(30.0, 15.0)),
        Transform::from_xyz(150.0, 120.0, 0.4),
        RigidBody::Fixed,
        Collider::cuboid(15.0, 7.5),
        Sensor,
        platform_nesting_box,
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Nest,
            base_utility: 0.85,
            range: 50.0,
        },
    ));
    
    // Predator deterrent - motion-activated sprinkler
    let motion_sprinkler = PredatorDeterrent {
        deterrent_type: DeterrentType::MotionActivatedSprinkler,
        position: Vec3::new(-50.0, -200.0, 0.0),
        effectiveness: 0.8,
        range: 100.0,
        maintenance_timer: Timer::from_seconds(30.0 * 24.0 * 3600.0, TimerMode::Repeating), // 30 days
        active: true,
    };
    
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.5, 0.7), Vec2::new(25.0, 30.0)),
        Transform::from_xyz(-50.0, -200.0, 0.3),
        RigidBody::Fixed,
        Collider::cuboid(12.5, 15.0),
        Sensor,
        motion_sprinkler,
    ));
    
    // Reflective tape deterrent for hawks
    let reflective_tape = PredatorDeterrent {
        deterrent_type: DeterrentType::ReflectiveTape,
        position: Vec3::new(250.0, 180.0, 0.0),
        effectiveness: 0.7,
        range: 80.0,
        maintenance_timer: Timer::from_seconds(14.0 * 24.0 * 3600.0, TimerMode::Repeating), // 14 days
        active: true,
    };
    
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.9, 0.9), Vec2::new(60.0, 8.0)),
        Transform::from_xyz(250.0, 180.0, 0.3),
        RigidBody::Fixed,
        Collider::cuboid(30.0, 4.0),
        Sensor,
        reflective_tape,
    ));
}
*/

pub fn seasonal_object_system(
    time_state: Res<crate::environment::resources::TimeState>,
    mut perch_query: Query<&mut PerchingSpot>,
    mut shelter_query: Query<&mut Shelter>,
    mut water_query: Query<&mut WaterFeature>,
) {
    let season = time_state.get_season();
    
    // Adjust perch appeal based on season
    for mut perch in &mut perch_query {
        perch.comfort_level = match (season, perch.perch_type) {
            (Season::Winter, PerchType::Wire) => 0.3,    // Cold metal wire
            (Season::Winter, PerchType::Branch) => 0.9,  // Natural perches still good
            (Season::Summer, PerchType::Rock) => 0.5,    // Hot rocks less appealing
            (Season::Spring, _) => 0.9,                  // All perches appealing in spring
            _ => perch.perch_type.base_utility(),
        };
    }
    
    // Seasonal shelter appeal changes
    for mut shelter in &mut shelter_query {
        let seasonal_modifier = match season {
            Season::Winter => 1.3, // More important in winter
            Season::Fall => 1.2,   // Migration season, need shelter
            Season::Summer => 0.8, // Less critical in good weather
            Season::Spring => 1.0, // Baseline appeal
        };
        
        shelter.weather_protection = shelter.shelter_type.weather_protection() * seasonal_modifier;
    }
    
    // Water feature seasonal changes
    for mut water in &mut water_query {
        match season {
            Season::Winter => {
                water.temperature = 5.0; // Cold water
                if matches!(water.water_type, WaterType::Pond | WaterType::Creek) {
                    water.water_level *= 0.8; // Some freezing
                }
            },
            Season::Summer => {
                water.temperature = 25.0; // Warm water
                // Faster evaporation in summer (handled in water_feature_system)
            },
            Season::Spring => {
                water.temperature = 15.0;
                water.water_level = 1.0; // Spring rains fill everything
            },
            Season::Fall => {
                water.temperature = 10.0;
                water.water_level = 0.9; // Fall rains
            },
        }
    }
}

// === SANCTUARY MANAGEMENT SYSTEMS ===

pub fn sanctuary_interaction_system(
    mut nesting_box_query: Query<(&Transform, &mut NestingBox)>,
    bird_query: Query<(Entity, &Transform, &BirdState, &crate::bird::Bird), With<crate::bird_ai::components::BirdAI>>,
    time_state: Res<crate::environment::resources::TimeState>,
) {
    let current_season = time_state.get_season();
    
    // Update nesting box usage during breeding season
    for (_nesting_transform, mut nesting_box) in &mut nesting_box_query {
        if nesting_box.breeding_season.contains(&current_season) {
            // Simulate breeding activity during appropriate seasons
            if matches!(nesting_box.occupancy_status, NestingStatus::Empty) {
                // Check for birds that might want to nest
                for (_bird_entity, _bird_transform, bird_state, bird) in &bird_query {
                    if nesting_box.target_species.contains(&bird.species) && 
                       matches!(bird_state, BirdState::Resting) {
                        // Small chance to start nesting
                        if rand::random::<f32>() < 0.001 { // Very low chance per frame
                            nesting_box.occupancy_status = NestingStatus::UnderConstruction;
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn predator_deterrent_system(
    mut deterrent_query: Query<&mut PredatorDeterrent>,
    bird_query: Query<&Transform, (With<crate::bird::Bird>, With<crate::bird_ai::components::BirdAI>)>,
    time: Res<Time>,
) {
    for mut deterrent in &mut deterrent_query {
        // Update maintenance timer
        deterrent.maintenance_timer.tick(time.delta());
        
        // Check if maintenance is needed
        if deterrent.maintenance_timer.finished() {
            deterrent.active = false; // Requires maintenance
        }
        
        // If active, provide protection to nearby birds
        if deterrent.active {
            let protected_birds = bird_query.iter()
                .filter(|bird_transform| {
                    bird_transform.translation.distance(deterrent.position) < deterrent.range
                })
                .count();
                
            // Deterrent effectiveness may vary based on number of protected birds
            deterrent.effectiveness = match deterrent.deterrent_type {
                DeterrentType::MotionActivatedSprinkler => 0.8,
                DeterrentType::ReflectiveTape => 0.7,
                DeterrentType::UltrasonicDevice => 0.6,
                DeterrentType::ScareOwl => 0.5,
                DeterrentType::ProtectiveMesh => 0.9,
                DeterrentType::NaturalBarrier => 0.4,
            };
        }
    }
}
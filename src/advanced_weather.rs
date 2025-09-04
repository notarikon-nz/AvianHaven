// Advanced Weather Effects System
use bevy::prelude::*;
use crate::bird::{BirdSpecies, Bird};
use crate::bird_ai::components::{BirdAI, BirdState, Blackboard};
use crate::environment::resources::{WeatherState, TimeState};
use crate::environment::components::{Weather};
// use crate::flocking::components::Flock;

pub struct AdvancedWeatherPlugin;

impl Plugin for AdvancedWeatherPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StormManager>()
            .init_resource::<TemperatureManager>()
            .init_resource::<WindManager>()
            .add_event::<StormEvent>()
            .add_event::<EmergencyFlockingEvent>()
            .add_event::<TemperatureStressEvent>()
            .add_systems(Startup, setup_weather_shelters)
            .add_systems(Update, (
                storm_detection_system,
                emergency_flocking_system,
                storm_sheltering_system,
                temperature_feeding_urgency_system,
                wind_flight_effects_system,
                weather_stress_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// Enhanced Weather Types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StormSeverity {
    #[default]
    Light,      // Light rain/snow, mild wind
    Moderate,   // Heavy rain/snow, strong wind
    Severe,     // Storm conditions, very dangerous
    Extreme,    // Hurricane/blizzard conditions
}

impl StormSeverity {
    pub fn emergency_flocking_chance(&self) -> f32 {
        match self {
            Self::Light => 0.1,
            Self::Moderate => 0.3,
            Self::Severe => 0.7,
            Self::Extreme => 0.9,
        }
    }

    pub fn shelter_urgency_multiplier(&self) -> f32 {
        match self {
            Self::Light => 1.0,
            Self::Moderate => 1.5,
            Self::Severe => 2.0,
            Self::Extreme => 3.0,
        }
    }
}

// Temperature ranges and effects
#[derive(Debug, Clone, Copy)]
pub struct TemperatureRange {
    pub celsius: f32,
    pub comfort_level: f32,    // 0.0 = very uncomfortable, 1.0 = perfect
    pub feeding_urgency: f32,  // Multiplier for hunger rate
    pub activity_modifier: f32, // Effect on general activity
}

impl TemperatureRange {
    pub fn from_celsius(temp: f32) -> Self {
        if temp < -10.0 {
            // Extreme cold
            Self {
                celsius: temp,
                comfort_level: 0.0,
                feeding_urgency: 2.5,
                activity_modifier: 0.3,
            }
        } else if temp < 0.0 {
            // Cold
            Self {
                celsius: temp,
                comfort_level: 0.2,
                feeding_urgency: 2.0,
                activity_modifier: 0.5,
            }
        } else if temp < 10.0 {
            // Cool
            Self {
                celsius: temp,
                comfort_level: 0.7,
                feeding_urgency: 1.3,
                activity_modifier: 0.8,
            }
        } else if temp < 25.0 {
            // Comfortable
            Self {
                celsius: temp,
                comfort_level: 1.0,
                feeding_urgency: 1.0,
                activity_modifier: 1.0,
            }
        } else if temp < 35.0 {
            // Warm
            Self {
                celsius: temp,
                comfort_level: 0.8,
                feeding_urgency: 0.8,
                activity_modifier: 0.9,
            }
        } else {
            // Hot
            Self {
                celsius: temp,
                comfort_level: 0.3,
                feeding_urgency: 0.6,
                activity_modifier: 0.4,
            }
        }
    }
}

// Wind effects on different flight behaviors
#[derive(Debug, Clone, Copy)]
pub struct WindEffects {
    pub speed_kmh: f32,
    pub direction: Vec2,       // Normalized wind direction
    pub turbulence: f32,       // 0.0 = smooth, 1.0 = very turbulent
    pub hover_difficulty: f32, // Multiplier for hover feeding difficulty
}

impl Default for WindEffects {
    fn default() -> Self {
        Self {
            speed_kmh: 10.0,
            direction: Vec2::new(1.0, 0.0),
            turbulence: 0.1,
            hover_difficulty: 1.0,
        }
    }
}

impl WindEffects {
    pub fn from_weather(weather: Weather, base_speed: f32) -> Self {
        let speed = match weather {
            Weather::Clear => base_speed * 0.5,
            Weather::Cloudy => base_speed * 0.7,
            Weather::Rainy => base_speed * 1.2,
            Weather::Snowy => base_speed * 0.8,
            Weather::Windy => base_speed * 2.5,
        };

        Self {
            speed_kmh: speed,
            direction: Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize(),
            turbulence: (speed / 50.0).clamp(0.0, 1.0),
            hover_difficulty: (speed / 30.0).clamp(1.0, 3.0),
        }
    }

    pub fn flight_speed_modifier(&self) -> f32 {
        // Strong headwinds slow down flight, tailwinds speed it up
        let base_modifier = (50.0 - self.speed_kmh) / 50.0;
        base_modifier.clamp(0.3, 1.5)
    }
}

// Components
#[derive(Component)]
pub struct WeatherShelter {
    pub shelter_type: ShelterType,
    pub capacity: u32,
    pub current_occupancy: u32,
    pub protection_level: f32, // 0.0-1.0, how much protection it offers
    pub wind_resistance: f32,  // How well it blocks wind
}

#[derive(Debug, Clone, Copy)]
pub enum ShelterType {
    DenseVegetation,  // Thick bushes, evergreen trees
    Building,         // Houses, sheds, overhangs
    TreeHollow,       // Natural cavities
    RockFormation,    // Cliffs, rocky areas
    Feeder,           // Covered feeders with protection
}

#[derive(Component)]
pub struct EmergencyFlock {
    pub leader: Entity,
    pub members: Vec<Entity>,
    pub formation_time: f32,
    pub shelter_target: Option<Entity>,
    pub urgency_level: f32,
}

#[derive(Component)]
pub struct WindResistance {
    pub base_resistance: f32,    // Species-based wind tolerance
    pub current_fatigue: f32,    // Accumulated wind fatigue
    pub max_wind_speed: f32,     // Maximum wind speed this bird can handle
}

// Resources
#[derive(Resource, Default)]
pub struct StormManager {
    pub current_severity: StormSeverity,
    pub storm_duration: f32,
    pub time_remaining: f32,
    pub is_storm_warning: bool,
    pub warning_time: f32,
}

#[derive(Resource)]
pub struct TemperatureManager {
    pub current_temp: f32,
    pub daily_variation: f32,    // How much temperature varies during day
    pub seasonal_base: f32,      // Base temperature for current season
}

impl Default for TemperatureManager {
    fn default() -> Self {
        Self {
            current_temp: 20.0,
            daily_variation: 8.0,
            seasonal_base: 20.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct WindManager {
    pub current_effects: WindEffects,
    pub gust_timer: f32,
    pub base_wind_speed: f32,
}

// Events
#[derive(Event)]
pub struct StormEvent {
    pub severity: StormSeverity,
    pub duration: f32,
    pub warning_time: f32,
}

#[derive(Event)]
pub struct EmergencyFlockingEvent {
    pub bird_entities: Vec<Entity>,
    pub leader: Entity,
    pub urgency: f32,
}

#[derive(Event)]
pub struct TemperatureStressEvent {
    pub bird: Entity,
    pub temperature: f32,
    pub stress_level: f32,
}

// Systems implementation
fn setup_weather_shelters(mut commands: Commands) {
    // Create various shelter locations around the map
    let shelter_locations = [
        (-400.0, 300.0, ShelterType::DenseVegetation),
        (400.0, 300.0, ShelterType::DenseVegetation), 
        (-200.0, -400.0, ShelterType::Building),
        (200.0, -400.0, ShelterType::Building),
        (-300.0, 0.0, ShelterType::TreeHollow),
        (300.0, 0.0, ShelterType::TreeHollow),
        (0.0, 200.0, ShelterType::RockFormation),
        (-100.0, -200.0, ShelterType::Feeder),
        (100.0, -200.0, ShelterType::Feeder),
    ];

    for &(x, y, shelter_type) in &shelter_locations {
        commands.spawn((
            Transform::from_xyz(x, y, 1.0),
            WeatherShelter {
                shelter_type,
                capacity: match shelter_type {
                    ShelterType::DenseVegetation => 15,
                    ShelterType::Building => 25,
                    ShelterType::TreeHollow => 8,
                    ShelterType::RockFormation => 12,
                    ShelterType::Feeder => 6,
                },
                current_occupancy: 0,
                protection_level: match shelter_type {
                    ShelterType::Building => 0.95,
                    ShelterType::TreeHollow => 0.85,
                    ShelterType::RockFormation => 0.80,
                    ShelterType::DenseVegetation => 0.70,
                    ShelterType::Feeder => 0.60,
                },
                wind_resistance: match shelter_type {
                    ShelterType::Building => 0.90,
                    ShelterType::RockFormation => 0.85,
                    ShelterType::TreeHollow => 0.80,
                    ShelterType::DenseVegetation => 0.65,
                    ShelterType::Feeder => 0.40,
                },
            },
        ));
    }
}

fn storm_detection_system(
    mut storm_manager: ResMut<StormManager>,
    weather_state: Res<WeatherState>,
    time: Res<Time>,
    mut storm_events: EventWriter<StormEvent>,
) {
    storm_manager.time_remaining -= time.delta_secs();
    storm_manager.warning_time -= time.delta_secs();

    // Detect storm conditions from weather
    let new_severity = match weather_state.current_weather {
        Weather::Clear => StormSeverity::Light,
        Weather::Cloudy => StormSeverity::Light,
        Weather::Rainy => {
            if weather_state.temperature < 5.0 {
                StormSeverity::Moderate // Cold rain is more severe
            } else {
                StormSeverity::Light
            }
        },
        Weather::Snowy => StormSeverity::Moderate,
        Weather::Windy => StormSeverity::Severe,
    };

    // Check if storm severity has increased
    if matches!(new_severity, StormSeverity::Severe | StormSeverity::Extreme) &&
       !matches!(storm_manager.current_severity, StormSeverity::Severe | StormSeverity::Extreme) {
        // New severe storm detected
        storm_manager.current_severity = new_severity;
        storm_manager.storm_duration = rand::random::<f32>() * 600.0 + 300.0; // 5-15 minutes
        storm_manager.time_remaining = storm_manager.storm_duration;
        storm_manager.warning_time = 60.0; // 1 minute warning

        storm_events.write(StormEvent {
            severity: new_severity,
            duration: storm_manager.storm_duration,
            warning_time: 60.0,
        });
    }

    // Issue warning
    if storm_manager.warning_time > 0.0 && !storm_manager.is_storm_warning {
        storm_manager.is_storm_warning = true;
    }

    // Clear storm when time expires
    if storm_manager.time_remaining <= 0.0 {
        storm_manager.current_severity = StormSeverity::Light;
        storm_manager.is_storm_warning = false;
    }
}

fn emergency_flocking_system(
    mut commands: Commands,
    storm_manager: Res<StormManager>,
    mut bird_query: Query<(Entity, &Transform, &Bird, &mut Blackboard), With<BirdAI>>,
    shelter_query: Query<(Entity, &Transform, &WeatherShelter), With<WeatherShelter>>,
    mut emergency_events: EventWriter<EmergencyFlockingEvent>,
) {
    if !storm_manager.is_storm_warning {
        return;
    }

    let flocking_chance = storm_manager.current_severity.emergency_flocking_chance();
    
    // Find birds that should form emergency flocks
    let mut potential_flockers: Vec<(Entity, Vec3, BirdSpecies)> = Vec::new();
    
    for (entity, transform, bird, mut blackboard) in bird_query.iter_mut() {
        // Increase fear due to storm
        blackboard.internal.fear += 0.1;
        
        if rand::random::<f32>() < flocking_chance {
            potential_flockers.push((entity, transform.translation, bird.species));
        }
    }

    // Group nearby birds into emergency flocks
    let mut flocks_formed = 0;
    let mut processed_birds = std::collections::HashSet::new();

    for &(leader_entity, leader_pos, leader_species) in &potential_flockers {
        if processed_birds.contains(&leader_entity) || flocks_formed >= 3 {
            continue;
        }

        let mut flock_members = vec![leader_entity];
        processed_birds.insert(leader_entity);

        // Find nearby birds to join the flock
        for &(other_entity, other_pos, other_species) in &potential_flockers {
            if processed_birds.contains(&other_entity) || flock_members.len() >= 12 {
                continue;
            }

            let distance = leader_pos.distance(other_pos);
            let species_compatibility = if leader_species == other_species { 1.0 } else { 0.6 };
            
            if distance < 150.0 && rand::random::<f32>() < species_compatibility {
                flock_members.push(other_entity);
                processed_birds.insert(other_entity);
            }
        }

        // Create emergency flock if we have enough members
        if flock_members.len() >= 3 {
            // Find nearest suitable shelter
            let nearest_shelter = shelter_query
                .iter()
                .filter(|(_, _, shelter)| shelter.current_occupancy < shelter.capacity)
                .min_by(|(_, shelter_transform, _), (_, other_transform, _)| {
                    let dist1 = leader_pos.distance(shelter_transform.translation);
                    let dist2 = leader_pos.distance(other_transform.translation);
                    dist1.partial_cmp(&dist2).unwrap()
                })
                .map(|(entity, _, _)| entity);

            commands.spawn(EmergencyFlock {
                leader: leader_entity,
                members: flock_members.clone(),
                formation_time: 0.0,
                shelter_target: nearest_shelter,
                urgency_level: flocking_chance,
            });

            emergency_events.write(EmergencyFlockingEvent {
                bird_entities: flock_members,
                leader: leader_entity,
                urgency: flocking_chance,
            });

            flocks_formed += 1;
        }
    }
}

fn storm_sheltering_system(
    mut bird_query: Query<(&mut Transform, &mut BirdState, &mut Blackboard, &Bird), With<BirdAI>>,
    shelter_query: Query<(Entity, &Transform, &WeatherShelter), (With<WeatherShelter>, Without<BirdAI>)>,
    storm_manager: Res<StormManager>,
    weather_state: Res<WeatherState>,
    time: Res<Time>,
) {
    let shelter_urgency = weather_state.current_weather.shelter_urgency() * 
                         storm_manager.current_severity.shelter_urgency_multiplier();

    if shelter_urgency < 0.3 {
        return; // No significant need for shelter
    }

    for (mut transform, mut bird_state, mut blackboard, bird) in bird_query.iter_mut() {
        // Increase fear and urgency based on weather
        blackboard.internal.fear = (blackboard.internal.fear + shelter_urgency * time.delta_secs()).clamp(0.0, 1.0);

        // If bird is already sheltering, continue
        if matches!(*bird_state, BirdState::Sheltering) {
            continue;
        }

        // Check if bird should seek shelter
        let species_shelter_tolerance = match bird.species {
            // Hardy species that can tolerate more weather
            crate::bird::BirdSpecies::CommonCrow |
            crate::bird::BirdSpecies::BlueJay |
            crate::bird::BirdSpecies::MourningDove => 0.8,
            // Average tolerance
            _ => 0.5,
        };

        if blackboard.internal.fear > species_shelter_tolerance {
            // Find nearest suitable shelter
            if let Some((shelter_entity, shelter_transform, _)) = shelter_query
                .iter()
                .filter(|(_, _, shelter)| shelter.current_occupancy < shelter.capacity)
                .min_by(|(_, shelter_transform, _), (_, other_transform, _)| {
                    let dist1 = transform.translation.distance(shelter_transform.translation);
                    let dist2 = transform.translation.distance(other_transform.translation);
                    dist1.partial_cmp(&dist2).unwrap()
                }) {
                
                // Move toward shelter
                let direction = (shelter_transform.translation - transform.translation).normalize();
                let move_speed = 100.0 * time.delta_secs();
                transform.translation += direction * move_speed;

                // If close enough to shelter, enter sheltering state
                if transform.translation.distance(shelter_transform.translation) < 30.0 {
                    *bird_state = BirdState::Sheltering;
                    blackboard.current_target = Some(shelter_entity);
                    blackboard.internal.fear *= 0.5; // Reduce fear when sheltered
                }
            }
        }
    }
}

fn temperature_feeding_urgency_system(
    mut bird_query: Query<(Entity, &Bird, &mut Blackboard), With<BirdAI>>,
    temp_manager: Res<TemperatureManager>,
    time_state: Res<TimeState>,
    time: Res<Time>,
    mut temp_events: EventWriter<TemperatureStressEvent>,
) {
    // Calculate current temperature with daily variation
    let hour_factor = (time_state.hour as f32 / 24.0 * 2.0 * std::f32::consts::PI).sin();
    let current_temp = temp_manager.seasonal_base + (temp_manager.daily_variation * hour_factor * 0.5);
    
    let temp_range = TemperatureRange::from_celsius(current_temp);

    for (entity, bird, mut blackboard) in bird_query.iter_mut() {
        let species_temp_tolerance = match bird.species {
            // Cold-adapted species
            crate::bird::BirdSpecies::Chickadee |
            crate::bird::BirdSpecies::WhiteBreastedNuthatch |
            crate::bird::BirdSpecies::CommonCrow => 0.3,
            // Heat-adapted species  
            crate::bird::BirdSpecies::MourningDove |
            crate::bird::BirdSpecies::RedWingedBlackbird => -0.3,
            // Average adaptation
            _ => 0.0,
        };

        let adjusted_comfort = (temp_range.comfort_level + species_temp_tolerance).clamp(0.0, 1.0);
        let stress_level = 1.0 - adjusted_comfort;

        // Increase hunger rate based on temperature stress
        let hunger_increase = temp_range.feeding_urgency * stress_level * time.delta_secs() * 0.01;
        blackboard.internal.hunger = (blackboard.internal.hunger + hunger_increase).clamp(0.0, 1.0);

        // Add temperature-based stress
        if stress_level > 0.5 {
            blackboard.internal.fear = (blackboard.internal.fear + stress_level * time.delta_secs() * 0.1).clamp(0.0, 1.0);
            
            temp_events.write(TemperatureStressEvent {
                bird: entity,
                temperature: current_temp,
                stress_level,
            });
        }
    }
}

fn wind_flight_effects_system(
    mut bird_query: Query<(&mut Transform, &Bird, &BirdState), With<BirdAI>>,
    wind_manager: Res<WindManager>,
    time: Res<Time>,
) {
    let wind_effects = &wind_manager.current_effects;
    
    for (mut transform, bird, state) in bird_query.iter_mut() {
        // Only apply wind effects to flying/moving birds
        if !matches!(state, BirdState::MovingToTarget | BirdState::Wandering | BirdState::HoverFeeding) {
            continue;
        }

        // Calculate wind resistance based on species size and adaptations
        let wind_resistance = match bird.species {
            // Large, strong fliers
            crate::bird::BirdSpecies::CommonCrow |
            crate::bird::BirdSpecies::BlueJay => 0.8,
            // Small, agile fliers  
            crate::bird::BirdSpecies::Chickadee |
            crate::bird::BirdSpecies::RubyThroatedHummingbird => 0.4,
            // Medium birds
            _ => 0.6,
        };

        // Apply wind drift
        let wind_force = wind_effects.direction * wind_effects.speed_kmh * (1.0 - wind_resistance) * 0.1;
        transform.translation.x += wind_force.x * time.delta_secs();
        transform.translation.y += wind_force.y * time.delta_secs();

        // Special hover feeding wind effects
        if matches!(state, BirdState::HoverFeeding) {
            let hover_difficulty = wind_effects.hover_difficulty;
            // Add turbulence to hover feeding birds
            if hover_difficulty > 1.5 {
                let turbulence = Vec2::new(
                    (rand::random::<f32>() - 0.5) * wind_effects.turbulence,
                    (rand::random::<f32>() - 0.5) * wind_effects.turbulence,
                ) * 20.0 * time.delta_secs();
                
                transform.translation.x += turbulence.x;
                transform.translation.y += turbulence.y;
            }
        }
    }
}

fn weather_stress_system(
    mut bird_query: Query<(Entity, &Bird, &mut Blackboard), With<BirdAI>>,
    weather_state: Res<WeatherState>,
    storm_manager: Res<StormManager>,
    time: Res<Time>,
) {
    let base_stress = weather_state.current_weather.weather_fear_factor();
    let storm_stress = match storm_manager.current_severity {
        StormSeverity::Light => 0.0,
        StormSeverity::Moderate => 0.2,
        StormSeverity::Severe => 0.4,
        StormSeverity::Extreme => 0.8,
    };

    let total_stress = (base_stress + storm_stress).min(1.0);

    if total_stress > 0.1 {
        for (_entity, bird, mut blackboard) in bird_query.iter_mut() {
            let species_stress_resistance = match bird.species {
                // Hardy species
                crate::bird::BirdSpecies::CommonCrow |
                crate::bird::BirdSpecies::BlueJay => 0.7,
                // Sensitive species
                crate::bird::BirdSpecies::RubyThroatedHummingbird |
                crate::bird::BirdSpecies::Goldfinch => 0.3,
                // Average
                _ => 0.5,
            };

            let effective_stress = total_stress * (1.0 - species_stress_resistance);
            blackboard.internal.fear = (blackboard.internal.fear + effective_stress * time.delta_secs() * 0.2).clamp(0.0, 1.0);
        }
    }
}
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{AppState, resources::{GameConfig, BirdCount, SpawnBirdEvent}};
use crate::bird_ai::components::{BirdAI, BirdState, Blackboard, InternalState};
use crate::animation::components::AnimatedBird;
use crate::feeder::FeederType;
use crate::environment::resources::{TimeState, WeatherState, SeasonalState};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BirdCount>()
            .add_event::<SpawnBirdEvent>()
            .add_systems(Startup, spawn_initial_birds)
            .add_systems(
                Update,
                (
                    handle_spawn_events,
                    bird_movement,
                    update_wander_timer,
                    environmental_bird_spawning_system,
                ).run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirdSpecies {
    // Tier 1 - Common backyard birds (20 species total)
    Cardinal,
    BlueJay,
    Robin,
    Sparrow,
    Chickadee,
    Goldfinch,
    NorthernMockingbird,
    RedWingedBlackbird,
    CommonGrackle,
    BrownThrasher,
    CedarWaxwing,
    WhiteBreastedNuthatch,
    TuftedTitmouse,
    CarolinaWren,
    HouseFinch,
    EuropeanStarling,
    MourningDove,
    CommonCrow,
    BlueGrayGnatcatcher,
    YellowWarbler,
}

impl BirdSpecies {
    fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..20) {
            0 => Self::Cardinal,
            1 => Self::BlueJay,
            2 => Self::Robin,
            3 => Self::Sparrow,
            4 => Self::Chickadee,
            5 => Self::Goldfinch,
            6 => Self::NorthernMockingbird,
            7 => Self::RedWingedBlackbird,
            8 => Self::CommonGrackle,
            9 => Self::BrownThrasher,
            10 => Self::CedarWaxwing,
            11 => Self::WhiteBreastedNuthatch,
            12 => Self::TuftedTitmouse,
            13 => Self::CarolinaWren,
            14 => Self::HouseFinch,
            15 => Self::EuropeanStarling,
            16 => Self::MourningDove,
            17 => Self::CommonCrow,
            18 => Self::BlueGrayGnatcatcher,
            _ => Self::YellowWarbler,
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Cardinal => Color::srgb(0.8, 0.2, 0.2),
            Self::BlueJay => Color::srgb(0.2, 0.4, 0.8),
            Self::Robin => Color::srgb(0.6, 0.3, 0.1),
            Self::Sparrow => Color::srgb(0.5, 0.4, 0.3),
            Self::Chickadee => Color::srgb(0.3, 0.3, 0.3),
            Self::Goldfinch => Color::srgb(0.9, 0.8, 0.2),
            Self::NorthernMockingbird => Color::srgb(0.7, 0.7, 0.7),
            Self::RedWingedBlackbird => Color::srgb(0.1, 0.1, 0.1),
            Self::CommonGrackle => Color::srgb(0.2, 0.1, 0.3),
            Self::BrownThrasher => Color::srgb(0.6, 0.4, 0.2),
            Self::CedarWaxwing => Color::srgb(0.7, 0.6, 0.4),
            Self::WhiteBreastedNuthatch => Color::srgb(0.8, 0.8, 0.9),
            Self::TuftedTitmouse => Color::srgb(0.6, 0.6, 0.6),
            Self::CarolinaWren => Color::srgb(0.5, 0.3, 0.2),
            Self::HouseFinch => Color::srgb(0.7, 0.4, 0.4),
            Self::EuropeanStarling => Color::srgb(0.3, 0.3, 0.4),
            Self::MourningDove => Color::srgb(0.5, 0.4, 0.4),
            Self::CommonCrow => Color::srgb(0.1, 0.1, 0.1),
            Self::BlueGrayGnatcatcher => Color::srgb(0.4, 0.5, 0.6),
            Self::YellowWarbler => Color::srgb(0.8, 0.8, 0.3),
        }
    }
    
    /// Returns preferred feeder types for this species, in order of preference
    pub fn preferred_feeders(&self) -> Vec<FeederType> {
        match self {
            // Seed lovers
            Self::Cardinal | Self::BlueJay | Self::Chickadee | Self::TuftedTitmouse |
            Self::WhiteBreastedNuthatch => vec![FeederType::Seed, FeederType::Suet],
            
            // Ground feeders
            Self::Sparrow | Self::MourningDove | Self::CommonGrackle | Self::RedWingedBlackbird =>
                vec![FeederType::Ground, FeederType::Seed],
            
            // Nectar and fruit lovers
            Self::CedarWaxwing | Self::BrownThrasher => vec![FeederType::Fruit, FeederType::Seed],
            
            // Finches (small seeds)
            Self::Goldfinch | Self::HouseFinch => vec![FeederType::Seed],
            
            // Versatile feeders
            Self::Robin | Self::NorthernMockingbird => vec![FeederType::Ground, FeederType::Fruit, FeederType::Seed],
            
            // Insect eaters with occasional seeds
            Self::CarolinaWren | Self::BlueGrayGnatcatcher | Self::YellowWarbler =>
                vec![FeederType::Suet, FeederType::Seed],
            
            // Large omnivores
            Self::CommonCrow | Self::EuropeanStarling => vec![FeederType::Ground, FeederType::Suet, FeederType::Seed],
        }
    }
    
    /// Returns utility multiplier for a given feeder type (0.0-2.0)
    pub fn feeder_utility_modifier(&self, feeder_type: FeederType) -> f32 {
        let preferences = self.preferred_feeders();
        if let Some(index) = preferences.iter().position(|&ft| ft as u8 == feeder_type as u8) {
            match index {
                0 => 1.5,  // Most preferred
                1 => 1.2,  // Second preference
                2 => 1.0,  // Third preference
                _ => 0.8,  // Lower preference
            }
        } else {
            0.3  // Not preferred, but will still use if desperate
        }
    }
}

#[derive(Component)]
pub struct Bird {
    pub species: BirdSpecies,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
struct WanderTimer(Timer);

fn handle_spawn_events(
    mut commands: Commands,
    mut events: EventReader<SpawnBirdEvent>,
    mut bird_count: ResMut<BirdCount>,
) {
    for _ in events.read() {
        spawn_bird(&mut commands);
        bird_count.0 += 1;
    }
}

fn spawn_initial_birds(mut commands: Commands) {
    // Spawn 3-5 initial birds to populate the world
    for _ in 0..4 {
        spawn_bird(&mut commands);
    }
}

fn spawn_bird(commands: &mut Commands) {
    let species = BirdSpecies::random();
    let mut rng = rand::rng();
    
    let x = rng.random_range(-400.0..400.0);
    let y = rng.random_range(-300.0..300.0);
    
    commands.spawn((
        Sprite::from_color(species.color(), Vec2::new(20.0, 20.0)),
        Transform::from_xyz(x, y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(10.0),
        Bird { species },
        Velocity(Vec2::ZERO),
        WanderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        GravityScale(0.0),
        Damping { linear_damping: 2.0, angular_damping: 10.0 },
        // AI Components
        BirdAI,
        BirdState::Wandering,
        Blackboard {
            internal: InternalState {
                hunger: rng.random_range(0.2..0.8),
                thirst: rng.random_range(0.2..0.8),
                energy: rng.random_range(0.5..1.0),
                fear: 0.0,
            },
            ..default()
        },
        // Animation component (using available animated species for now)
        AnimatedBird {
            species: match species {
                BirdSpecies::Cardinal => crate::animation::components::BirdSpecies::Cardinal,
                BirdSpecies::BlueJay => crate::animation::components::BirdSpecies::BlueJay,
                BirdSpecies::Sparrow | BirdSpecies::Chickadee | BirdSpecies::Goldfinch | 
                BirdSpecies::HouseFinch | BirdSpecies::CarolinaWren | BirdSpecies::TuftedTitmouse |
                BirdSpecies::WhiteBreastedNuthatch => crate::animation::components::BirdSpecies::Sparrow,
                _ => crate::animation::components::BirdSpecies::Sparrow,
            }
        },
    ));
}

fn bird_movement(
    mut bird_query: Query<(&mut Velocity, &mut Transform), With<Bird>>,
    feeder_query: Query<&Transform, (With<crate::feeder::Feeder>, Without<Bird>)>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform) in bird_query.iter_mut() {
        let mut force = Vec2::ZERO;
        
        // Attraction to feeders
        for feeder_transform in feeder_query.iter() {
            let distance = transform.translation.truncate().distance(feeder_transform.translation.truncate());
            if distance < config.bird_attraction_radius {
                let direction = (feeder_transform.translation.truncate() - transform.translation.truncate()).normalize();
                let strength = (config.bird_attraction_radius - distance) / config.bird_attraction_radius;
                force += direction * strength * config.bird_attraction_force;
            }
        }
        
        // Simple obstacle avoidance (removed Rapier context usage)
        let mut avoidance_force = Vec2::ZERO;
        for other_feeder in feeder_query.iter() {
            let distance = transform.translation.truncate().distance(other_feeder.translation.truncate());
            if distance < config.bird_avoidance_radius && distance > 0.1 {
                let direction = (transform.translation.truncate() - other_feeder.translation.truncate()).normalize();
                avoidance_force += direction * (config.bird_avoidance_radius - distance) / config.bird_avoidance_radius;
            }
        }
        force += avoidance_force * config.bird_attraction_force;
        
        // Apply wandering velocity
        let target_velocity = velocity.0 + force * time.delta().as_secs_f32();
        let max_speed = config.bird_wander_speed;
        if target_velocity.length() > max_speed {
            velocity.0 = target_velocity.normalize() * max_speed;
        } else {
            velocity.0 = target_velocity;
        }
        
        // Update transform
        transform.translation += velocity.0.extend(0.0) * time.delta().as_secs_f32();
    }
}

fn update_wander_timer(
    mut bird_query: Query<(&mut Velocity, &mut WanderTimer), With<Bird>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    
    for (mut velocity, mut timer) in bird_query.iter_mut() {
        timer.0.tick(time.delta());
        
        if timer.0.just_finished() {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let speed = config.bird_wander_speed * rng.random_range(0.3..1.0);
            velocity.0 = Vec2::new(angle.cos(), angle.sin()) * speed;
        }
    }
}

pub fn environmental_bird_spawning_system(
    mut commands: Commands,
    bird_count: Res<BirdCount>,
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    seasonal_state: Res<SeasonalState>,
    time: Res<Time>,
) {
    // Environmental spawning logic
    let season = time_state.get_season();
    let base_activity = season.bird_activity_modifier();
    let weather_activity = weather_state.current_weather.bird_activity_modifier();
    let time_activity = time_state.daylight_factor();
    
    let spawn_chance = base_activity * weather_activity * time_activity * 0.001; // Base spawn rate per frame
    
    // Only spawn if we're under the bird limit and conditions are favorable
    if bird_count.0 < 15 && rand::rng().random::<f32>() < spawn_chance {
        spawn_seasonal_bird(&mut commands, &seasonal_state);
    }
}

fn spawn_seasonal_bird(commands: &mut Commands, seasonal_state: &SeasonalState) {
    let mut rng = rand::rng();
    
    // Select species based on seasonal availability
    let available_species: Vec<(BirdSpecies, f32)> = seasonal_state.available_species.iter()
        .map(|(species, probability)| (*species, *probability))
        .collect();
    
    if available_species.is_empty() {
        return; // No species available
    }
    
    // Weighted random selection
    let total_weight: f32 = available_species.iter().map(|(_, weight)| weight).sum();
    let mut random_weight = rng.random::<f32>() * total_weight;
    
    let mut selected_species = available_species[0].0;
    for (species, weight) in available_species {
        random_weight -= weight;
        if random_weight <= 0.0 {
            selected_species = species;
            break;
        }
    }
    
    // Spawn the selected bird
    spawn_specific_bird(commands, selected_species);
}

fn spawn_specific_bird(commands: &mut Commands, species: BirdSpecies) {
    let mut rng = rand::rng();
    
    let x = rng.random_range(-400.0..400.0);
    let y = rng.random_range(-300.0..300.0);
    
    commands.spawn((
        Sprite::from_color(species.color(), Vec2::new(20.0, 20.0)),
        Transform::from_xyz(x, y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(10.0),
        Bird { species },
        Velocity(Vec2::ZERO),
        WanderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        GravityScale(0.0),
        Damping { linear_damping: 2.0, angular_damping: 10.0 },
        // AI Components
        BirdAI,
        BirdState::Wandering,
        Blackboard {
            internal: InternalState {
                hunger: rng.random_range(0.2..0.8),
                thirst: rng.random_range(0.2..0.8),
                energy: rng.random_range(0.5..1.0),
                fear: 0.0,
            },
            world_knowledge: crate::bird_ai::components::WorldKnowledge::default(),
            current_target: None,
        },
        // Animation component
        AnimatedBird {
            species: match species {
                BirdSpecies::Cardinal => crate::animation::components::BirdSpecies::Cardinal,
                BirdSpecies::BlueJay => crate::animation::components::BirdSpecies::BlueJay,
                _ => crate::animation::components::BirdSpecies::Sparrow, // Default for others
            },
        },
    ));
}
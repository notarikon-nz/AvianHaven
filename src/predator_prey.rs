// Predator-Prey Dynamics System - Phase 4
use bevy::prelude::*;
use crate::bird::{BirdSpecies, Bird};
use crate::bird_ai::components::{BirdAI, BirdState, Blackboard};
// Note: TimeState, WeatherState, and rand::Rng reserved for future environmental integration

pub struct PredatorPreyPlugin;

impl Plugin for PredatorPreyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PredatorManager>()
            .add_event::<PredatorAttackEvent>()
            .add_event::<AlertCallEvent>()
            .add_systems(Update, (
                predator_hunting_system,
                prey_response_system,
                alert_call_system,
                predator_detection_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// Resources
#[derive(Resource, Default)]
pub struct PredatorManager {
    pub active_predators: Vec<Entity>,
    pub attack_cooldown: Timer,
    pub threat_level: f32, // 0.0-1.0, affects all birds
}

// Components
#[derive(Component, Default)]
pub struct Predator {
    pub hunting_style: HuntingStyle,
    pub attack_range: f32,
    pub success_rate: f32,
    pub energy: f32,
    pub last_hunt_time: f32,
    pub preferred_prey: Vec<BirdSpecies>,
}

#[derive(Component)]
pub struct PreyResponse {
    pub escape_behavior: EscapeBehavior,
    pub alert_range: f32,
    pub fear_recovery_time: f32,
    pub group_safety_bonus: f32,
}

// Enums
#[derive(Default, Clone, Copy, PartialEq)]
pub enum HuntingStyle {
    #[default]
    Ambush,       // Wait in cover, strike quickly
    Pursuit,      // Chase prey over distance
    Soaring,      // Attack from above while flying
    Perching,     // Hunt from a perch, dive on prey
}

#[derive(Clone, Copy, PartialEq)]
pub enum EscapeBehavior {
    Scatter,      // Individual birds flee in all directions
    Freeze,       // Stay motionless to avoid detection
    Mob,          // Group together to harass predator
    Dive,         // Dive into dense cover
}

// Events
#[derive(Event)]
pub struct PredatorAttackEvent {
    pub predator: Entity,
    pub target: Entity,
    pub attack_position: Vec3,
    pub success: bool,
}

#[derive(Event)]
pub struct AlertCallEvent {
    pub caller: Entity,
    pub predator_location: Vec3,
    pub urgency: f32,
    pub call_range: f32,
}

// Predator hunting system
pub fn predator_hunting_system(
    mut predator_query: Query<(Entity, &mut Predator, &mut Transform, &Bird), With<BirdAI>>,
    prey_query: Query<(Entity, &Transform, &Bird), (With<BirdAI>, Without<Predator>)>,
    mut predator_manager: ResMut<PredatorManager>,
    mut attack_events: EventWriter<PredatorAttackEvent>,
    time: Res<Time>,
) {
    // Update attack cooldown
    predator_manager.attack_cooldown.tick(time.delta());
    
    for (predator_entity, mut predator, mut predator_transform, predator_bird) in predator_query.iter_mut() {
        // Only certain species are predators
        if !is_predator_species(predator_bird.species) {
            continue;
        }
        
        // Update predator energy over time
        predator.energy += time.delta_secs() * 0.1; // Slowly recover energy
        predator.energy = predator.energy.min(1.0);
        
        // Only hunt if enough energy and not recently hunted
        if predator.energy < 0.3 || (time.elapsed_secs() - predator.last_hunt_time) < 30.0 {
            continue;
        }
        
        // Find potential prey within range
        let mut closest_prey: Option<(Entity, f32)> = None;
        let predator_pos = predator_transform.translation;
        
        for (prey_entity, prey_transform, prey_bird) in prey_query.iter() {
            let distance = predator_pos.distance(prey_transform.translation);
            
            if distance <= predator.attack_range {
                // Check if this prey species is preferred
                let preference_bonus = if predator.preferred_prey.contains(&prey_bird.species) {
                    0.5
                } else {
                    0.0
                };
                
                let hunt_probability = (predator.success_rate + preference_bonus) * 
                                      (predator.attack_range - distance) / predator.attack_range;
                
                if rand::random::<f32>() < hunt_probability {
                    if let Some((_, closest_distance)) = closest_prey {
                        if distance < closest_distance {
                            closest_prey = Some((prey_entity, distance));
                        }
                    } else {
                        closest_prey = Some((prey_entity, distance));
                    }
                }
            }
        }
        
        // Execute attack if prey found
        if let Some((prey_entity, _)) = closest_prey {
            let success = rand::random::<f32>() < predator.success_rate;
            
            attack_events.write(PredatorAttackEvent {
                predator: predator_entity,
                target: prey_entity,
                attack_position: predator_pos,
                success,
            });
            
            // Update predator state
            predator.energy -= 0.4; // Hunting costs energy
            predator.last_hunt_time = time.elapsed_secs();
        }
    }
}

// Prey response system
pub fn prey_response_system(
    mut attack_events: EventReader<PredatorAttackEvent>,
    mut prey_query: Query<(Entity, &mut BirdState, &mut Blackboard, &PreyResponse), With<BirdAI>>,
    mut alert_events: EventWriter<AlertCallEvent>,
) {
    for attack_event in attack_events.read() {
        // Target bird responds to attack
        if let Ok((_, mut bird_state, mut blackboard, prey_response)) = prey_query.get_mut(attack_event.target) {
            // Set high fear level
            blackboard.internal.fear = 1.0;
            
            // Execute escape behavior
            match prey_response.escape_behavior {
                EscapeBehavior::Scatter => {
                    *bird_state = BirdState::Fleeing;
                    
                    // Send alert call
                    alert_events.write(AlertCallEvent {
                        caller: attack_event.target,
                        predator_location: attack_event.attack_position,
                        urgency: 0.9,
                        call_range: prey_response.alert_range,
                    });
                },
                EscapeBehavior::Freeze => {
                    *bird_state = BirdState::Resting; // Stay motionless
                    blackboard.internal.fear = 0.8; // Less panic when freezing
                },
                EscapeBehavior::Mob => {
                    *bird_state = BirdState::Territorial; // Aggressive response
                    
                    // Send urgent mobbing call
                    alert_events.write(AlertCallEvent {
                        caller: attack_event.target,
                        predator_location: attack_event.attack_position,
                        urgency: 1.0,
                        call_range: prey_response.alert_range * 1.5,
                    });
                },
                EscapeBehavior::Dive => {
                    *bird_state = BirdState::Sheltering; // Seek dense cover
                },
            }
        }
    }
}

// Alert call system - other birds respond to warning calls
pub fn alert_call_system(
    mut alert_events: EventReader<AlertCallEvent>,
    mut bird_query: Query<(Entity, &Transform, &mut BirdState, &mut Blackboard), With<BirdAI>>,
) {
    for alert_event in alert_events.read() {
        for (bird_entity, bird_transform, mut bird_state, mut blackboard) in bird_query.iter_mut() {
            // Skip the caller
            if bird_entity == alert_event.caller {
                continue;
            }
            
            let distance = bird_transform.translation.distance(alert_event.predator_location);
            
            if distance <= alert_event.call_range {
                // Increase fear based on proximity and urgency
                let fear_increase = alert_event.urgency * (1.0 - distance / alert_event.call_range);
                blackboard.internal.fear += fear_increase * 0.5;
                blackboard.internal.fear = blackboard.internal.fear.min(1.0);
                
                // Change behavior if fear is high enough
                if blackboard.internal.fear > 0.6 {
                    *bird_state = BirdState::Fleeing;
                }
            }
        }
    }
}

// Predator detection system - visual detection of predators
pub fn predator_detection_system(
    predator_query: Query<(Entity, &Transform, &Bird), With<Predator>>,
    mut prey_query: Query<(Entity, &Transform, &mut Blackboard), (With<BirdAI>, Without<Predator>)>,
    mut alert_events: EventWriter<AlertCallEvent>,
) {
    for (predator_entity, predator_transform, _predator_bird) in predator_query.iter() {
        for (prey_entity, prey_transform, mut blackboard) in prey_query.iter_mut() {
            let distance = predator_transform.translation.distance(prey_transform.translation);
            let detection_range = 300.0; // Visual detection range
            
            if distance <= detection_range {
                // Probability of detection based on distance
                let detection_chance = 1.0 - (distance / detection_range);
                
                if rand::random::<f32>() < detection_chance * 0.3 { // 30% base chance per frame
                    // Spotted predator - increase fear
                    let fear_increase = 0.4 * (1.0 - distance / detection_range);
                    blackboard.internal.fear += fear_increase;
                    blackboard.internal.fear = blackboard.internal.fear.min(1.0);
                    
                    // Sometimes emit warning call
                    if rand::random::<f32>() < 0.2 { // 20% chance to call
                        alert_events.write(AlertCallEvent {
                            caller: prey_entity,
                            predator_location: predator_transform.translation,
                            urgency: 0.7,
                            call_range: 200.0,
                        });
                    }
                }
            }
        }
    }
}

// Helper function to identify predator species
pub fn is_predator_species(species: BirdSpecies) -> bool {
    matches!(species, 
        BirdSpecies::CoopersHawk | 
        BirdSpecies::RedTailedHawk |
        BirdSpecies::PeregrineFalcon |
        BirdSpecies::GreatHornedOwl |
        BirdSpecies::BarredOwl
    )
}

// Setup predator traits for existing birds
pub fn setup_predator_traits(
    mut commands: Commands,
    bird_query: Query<(Entity, &Bird), (With<BirdAI>, Without<Predator>, Without<PreyResponse>)>,
) {
    for (entity, bird) in bird_query.iter() {
        if is_predator_species(bird.species) {
            // Add predator component
            commands.entity(entity).insert(Predator {
                hunting_style: match bird.species {
                    BirdSpecies::CoopersHawk | BirdSpecies::RedTailedHawk => HuntingStyle::Pursuit,
                    BirdSpecies::RedTailedHawk => HuntingStyle::Soaring,
                    BirdSpecies::PeregrineFalcon => HuntingStyle::Soaring,
                    BirdSpecies::GreatHornedOwl | BirdSpecies::BarredOwl => HuntingStyle::Ambush,
                    _ => HuntingStyle::Perching,
                },
                attack_range: match bird.species {
                    BirdSpecies::PeregrineFalcon => 500.0, // High-speed attacks
                    BirdSpecies::RedTailedHawk => 400.0,   // Soaring attacks
                    BirdSpecies::GreatHornedOwl => 300.0,  // Silent approach
                    _ => 250.0, // Standard predators
                },
                success_rate: match bird.species {
                    BirdSpecies::PeregrineFalcon => 0.7,   // Very successful
                    BirdSpecies::CoopersHawk => 0.6,       // Agile hunter
                    BirdSpecies::GreatHornedOwl => 0.65,   // Nocturnal advantage
                    _ => 0.5, // Average success rate
                },
                energy: 0.8,
                last_hunt_time: 0.0,
                preferred_prey: get_preferred_prey(bird.species),
            });
        } else {
            // Add prey response component
            commands.entity(entity).insert(PreyResponse {
                escape_behavior: match bird.species {
                    BirdSpecies::Chickadee | BirdSpecies::TuftedTitmouse => EscapeBehavior::Mob,
                    BirdSpecies::CarolinaWren | BirdSpecies::WhiteBreastedNuthatch => EscapeBehavior::Dive,
                    BirdSpecies::HouseFinch | BirdSpecies::Sparrow => EscapeBehavior::Scatter,
                    _ => EscapeBehavior::Scatter,
                },
                alert_range: match bird.species {
                    BirdSpecies::BlueJay | BirdSpecies::CommonCrow => 400.0, // Loud warning calls
                    BirdSpecies::Chickadee => 300.0, // Social species
                    _ => 200.0, // Standard alert range
                },
                fear_recovery_time: 30.0, // 30 seconds to recover from fear
                group_safety_bonus: 0.3,  // 30% fear reduction in groups
            });
        }
    }
}

// Get preferred prey species for each predator
fn get_preferred_prey(predator_species: BirdSpecies) -> Vec<BirdSpecies> {
    match predator_species {
        BirdSpecies::CoopersHawk | BirdSpecies::RedTailedHawk => {
            vec![
                BirdSpecies::HouseFinch,
                BirdSpecies::Sparrow,
                BirdSpecies::Chickadee,
                BirdSpecies::CarolinaWren,
            ]
        },
        BirdSpecies::RedTailedHawk => {
            vec![
                BirdSpecies::EuropeanStarling,
                BirdSpecies::MourningDove,
                BirdSpecies::BlueJay,
            ]
        },
        BirdSpecies::PeregrineFalcon => {
            vec![
                BirdSpecies::EuropeanStarling,
                BirdSpecies::MourningDove,
                BirdSpecies::Robin,
            ]
        },
        BirdSpecies::GreatHornedOwl => {
            vec![
                BirdSpecies::EuropeanStarling,
                BirdSpecies::MourningDove,
                BirdSpecies::BlueJay,
                BirdSpecies::CommonCrow,
            ]
        },
        BirdSpecies::BarredOwl => {
            vec![
                BirdSpecies::CarolinaWren,
                BirdSpecies::Chickadee,
                BirdSpecies::WhiteBreastedNuthatch,
            ]
        },
        _ => vec![], // Non-predator species
    }
}
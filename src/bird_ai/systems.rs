use bevy::prelude::*;
use crate::bird_ai::{components::*, resources::*, bt::*, states::*};
use rand::Rng;
use crate::bird::Bird;
use crate::feeder::Feeder;
use crate::environment::resources::{TimeState, WeatherState};

pub fn setup_test_world(mut commands: Commands) {
    // Water source for drinking (supplement to nectar feeders)
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 0.8), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(200.0, -100.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Drink,
            base_utility: 0.8,
            range: 180.0,
        },
    ));
    
    // Bird bath for bathing
    commands.spawn((
        Sprite::from_color(Color::srgb(0.7, 0.7, 0.9), Vec2::new(35.0, 35.0)),
        Transform::from_xyz(80.0, -120.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Bathe,
            base_utility: 0.5,
            range: 150.0,
        },
    ));
    
    // Shelter structure (tree/bush for weather protection)
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.7, 0.2), Vec2::new(60.0, 80.0)),
        Transform::from_xyz(-150.0, 50.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Shelter,
            base_utility: 0.6,
            range: 200.0,
        },
    ));
    
    // Roosting site (large tree/structure for evening gathering)
    commands.spawn((
        Sprite::from_color(Color::srgb(0.4, 0.2, 0.1), Vec2::new(50.0, 70.0)),
        Transform::from_xyz(100.0, 150.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Roost,
            base_utility: 0.7,
            range: 220.0,
        },
    ));
}

pub fn world_utility_query_system(
    mut bird_query: Query<(&Transform, &mut Blackboard, &Bird), With<BirdAI>>,
    object_query: Query<(Entity, &Transform, &ProvidesUtility), With<SmartObject>>,
    feeder_query: Query<(Entity, &Transform, &ProvidesUtility, &Feeder), With<SmartObject>>,
    mut timer: ResMut<UtilityTimer>,
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() { return; }
    
    for (bird_transform, mut blackboard, bird) in bird_query.iter_mut() {
        blackboard.world_knowledge.available_actions.clear();
        
        // Process feeders with species preferences
        for (entity, obj_transform, utility, feeder) in feeder_query.iter() {
            let distance = bird_transform.translation.distance(obj_transform.translation);
            if distance <= utility.range {
                let distance_factor = 1.0 - (distance / utility.range);
                let species_modifier = bird.species.feeder_utility_modifier(feeder.feeder_type);
                
                // Apply environmental modifiers
                let weather_modifier = weather_state.current_weather.feeder_preference_modifier(&feeder.feeder_type);
                let time_modifier = if time_state.is_prime_feeding_time() { 1.2 } else { 0.8 };
                let daylight_modifier = time_state.daylight_factor();
                let song_activity_modifier = time_state.song_period_activity(); // Dawn chorus boost
                
                // Apply new species-specific modifiers
                let seasonal_modifier = bird.species.seasonal_feeding_modifier(feeder.feeder_type, time_state.get_season());
                let time_based_modifier = bird.species.time_based_feeding_modifier(feeder.feeder_type, time_state.hour);
                let technique_modifier = bird.species.feeding_technique_preference(feeder.feeder_type);
                
                let final_score = utility.base_utility * distance_factor * species_modifier * 
                                weather_modifier * time_modifier * daylight_modifier * song_activity_modifier *
                                seasonal_modifier * time_based_modifier * technique_modifier;
                
                let entry = UtilityEntry { entity, score: final_score };
                
                if let Some(existing) = blackboard.world_knowledge.available_actions.get(&utility.action) {
                    if final_score > existing.score {
                        blackboard.world_knowledge.available_actions.insert(utility.action, entry);
                    }
                } else {
                    blackboard.world_knowledge.available_actions.insert(utility.action, entry);
                }
            }
        }
        
        // Process non-feeder smart objects (water sources, baths)
        for (entity, obj_transform, utility) in object_query.iter() {
            // Skip entities that are already processed as feeders
            if feeder_query.contains(entity) { continue; }
            
            let distance = bird_transform.translation.distance(obj_transform.translation);
            if distance <= utility.range {
                let distance_factor = 1.0 - (distance / utility.range);
                let song_activity_modifier = time_state.song_period_activity(); // Dawn chorus boost for all activities
                
                // Apply weather modifiers
                let mut weather_modifier = 1.0;
                if utility.action == BirdAction::Shelter {
                    // Dramatically increase shelter utility during bad weather
                    weather_modifier = 1.0 + weather_state.current_weather.shelter_urgency() * 3.0; // Up to 4x utility in storms
                } else if weather_state.current_weather.prefer_cover() {
                    // Slightly reduce utility of exposed activities during bad weather
                    weather_modifier = match utility.action {
                        BirdAction::Eat | BirdAction::Drink => 0.7, // Reduce feeding in bad weather
                        BirdAction::Play | BirdAction::Explore => 0.4, // Greatly reduce exploration
                        BirdAction::Bathe => 0.2, // Avoid bathing in bad weather
                        _ => 0.8, // Slight reduction for other activities
                    };
                }
                
                let final_score = utility.base_utility * distance_factor * song_activity_modifier * weather_modifier;
                
                let entry = UtilityEntry { entity, score: final_score };
                
                if let Some(existing) = blackboard.world_knowledge.available_actions.get(&utility.action) {
                    if final_score > existing.score {
                        blackboard.world_knowledge.available_actions.insert(utility.action, entry);
                    }
                } else {
                    blackboard.world_knowledge.available_actions.insert(utility.action, entry);
                }
            }
        }
    }
}

pub fn social_awareness_system(
    mut bird_query: Query<(Entity, &Transform, &mut Blackboard, &Bird, &SocialBirdTraits), With<BirdAI>>,
    all_birds_query: Query<(Entity, &Transform, &Bird, &SocialBirdTraits), With<BirdAI>>,
    mut timer: ResMut<UtilityTimer>,
    time: Res<Time>,
    time_state: Res<TimeState>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() { return; }
    
    for (current_bird_entity, bird_transform, mut blackboard, bird, social_traits) in bird_query.iter_mut() {
        blackboard.world_knowledge.nearby_birds.clear();
        blackboard.world_knowledge.potential_mates.clear();
        blackboard.world_knowledge.territory_challengers.clear();
        
        let bird_pos = bird_transform.translation.truncate();
        let social_range = 300.0; // Range for detecting other birds
        
        for (other_entity, other_transform, other_bird, other_traits) in all_birds_query.iter() {
            // Skip self
            if other_entity == current_bird_entity {
                continue;
            }
            let other_pos = other_transform.translation.truncate();
            let distance = bird_pos.distance(other_pos);
            
            if distance <= social_range {
                let is_same_species = bird.species == other_bird.species;
                
                // Calculate social compatibility based on species and traits
                let social_compatibility = calculate_social_compatibility(
                    bird.species, 
                    other_bird.species,
                    social_traits,
                    other_traits,
                );
                
                let social_info = SocialBirdInfo {
                    entity: other_entity,
                    species: other_bird.species,
                    position: other_pos,
                    distance,
                    is_same_species,
                    dominance_level: other_traits.dominance_level,
                    social_compatibility,
                };
                
                blackboard.world_knowledge.nearby_birds.push(social_info.clone());
                
                // Identify potential mates (same species, appropriate mating conditions)
                if is_same_species && 
                   social_traits.mating_receptivity > 0.3 && 
                   other_traits.mating_receptivity > 0.3 &&
                   is_breeding_season(&time_state) {
                    blackboard.world_knowledge.potential_mates.push(other_entity);
                }
                
                // Identify territorial challengers (same species, high dominance, close proximity)
                if is_same_species && 
                   other_traits.dominance_level > 0.6 &&
                   distance < 150.0 && 
                   social_traits.territorial_aggression > 0.4 {
                    blackboard.world_knowledge.territory_challengers.push(other_entity);
                }
            }
        }
        
        // Add social actions to available actions based on nearby birds
        let potential_mates = blackboard.world_knowledge.potential_mates.clone();
        if !potential_mates.is_empty() {
            if let Some(mate_entity) = potential_mates.first() {
                let mate_info = blackboard.world_knowledge.nearby_birds.iter()
                    .find(|info| info.entity == *mate_entity);
                    
                if let Some(info) = mate_info {
                    let mate_utility = calculate_mate_utility(social_traits, info, &time_state);
                    blackboard.world_knowledge.available_actions.insert(
                        BirdAction::Court,
                        UtilityEntry { entity: *mate_entity, score: mate_utility }
                    );
                }
            }
        }
        
        // Add territorial challenge actions
        let territory_challengers = blackboard.world_knowledge.territory_challengers.clone();
        if !territory_challengers.is_empty() {
            if let Some(challenger_entity) = territory_challengers.first() {
                let challenge_utility = social_traits.territorial_aggression * 0.8;
                blackboard.world_knowledge.available_actions.insert(
                    BirdAction::Challenge,
                    UtilityEntry { entity: *challenger_entity, score: challenge_utility }
                );
            }
        }
        
        // Add flocking opportunities (mixed species flocking)
        let nearby_birds = blackboard.world_knowledge.nearby_birds.clone();
        let flock_candidates: Vec<_> = nearby_birds.iter()
            .filter(|info| !info.is_same_species && info.social_compatibility > 0.5)
            .collect();
            
        if !flock_candidates.is_empty() && social_traits.flock_tendency > 0.4 {
            if let Some(flock_target) = flock_candidates.first() {
                let flock_utility = social_traits.flock_tendency * flock_target.social_compatibility * 0.6;
                blackboard.world_knowledge.available_actions.insert(
                    BirdAction::Flock,
                    UtilityEntry { entity: flock_target.entity, score: flock_utility }
                );
            }
        }
    }
}

pub fn behavior_tree_system(
    mut bird_query: Query<(&mut BirdState, &mut Blackboard), With<BirdAI>>,
    mut timer: ResMut<BehaviorTreeTimer>,
    time: Res<Time>,
    time_state: Res<TimeState>,
    weather_state: Res<WeatherState>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() { return; }
    
    for (mut state, mut blackboard) in bird_query.iter_mut() {
        let new_state = evaluate_behavior_tree(&blackboard, &time_state, &weather_state);
        
        if new_state == BirdState::MovingToTarget {
            // Set target based on highest priority need
            let internal = &blackboard.internal;
            let actions = &blackboard.world_knowledge.available_actions;
            
            let weather = weather_state.current_weather;
            let shelter_urgency = weather.shelter_urgency();
            
            blackboard.current_target = if shelter_urgency > 0.6 && actions.contains_key(&BirdAction::Shelter) {
                // Critical weather - seek shelter immediately
                actions.get(&BirdAction::Shelter).map(|e| e.entity)
            } else if shelter_urgency > 0.3 && internal.energy < 0.7 && actions.contains_key(&BirdAction::Shelter) {
                // Moderate weather with low energy - prefer shelter
                actions.get(&BirdAction::Shelter).map(|e| e.entity)
            } else if time_state.hour >= 18.0 && time_state.hour <= 20.0 && actions.contains_key(&BirdAction::Roost) {
                // Evening roosting takes priority during dusk hours
                actions.get(&BirdAction::Roost).map(|e| e.entity)
            } else if internal.territorial_stress > 0.6 && actions.contains_key(&BirdAction::Challenge) {
                // Territorial challenge takes priority when stress is high
                actions.get(&BirdAction::Challenge).map(|e| e.entity)
            } else if internal.social_need > 0.5 && actions.contains_key(&BirdAction::Court) {
                // Mating behavior for high social need
                actions.get(&BirdAction::Court).map(|e| e.entity)
            } else if internal.social_need > 0.4 && actions.contains_key(&BirdAction::Flock) {
                // Flocking behavior for moderate social need
                actions.get(&BirdAction::Flock).map(|e| e.entity)
            } else if internal.social_need > 0.3 && actions.contains_key(&BirdAction::Follow) {
                // Following behavior for light social need
                actions.get(&BirdAction::Follow).map(|e| e.entity)
            } else if internal.hunger > 0.6 && actions.contains_key(&BirdAction::HoverFeed) {
                // Hover feeding for high hunger
                actions.get(&BirdAction::HoverFeed).map(|e| e.entity)
            } else if internal.hunger > 0.5 {
                actions.get(&BirdAction::Eat).map(|e| e.entity)
            } else if internal.hunger > 0.6 && actions.contains_key(&BirdAction::Retrieve) {
                // Retrieve cached food when very hungry
                actions.get(&BirdAction::Retrieve).map(|e| e.entity)
            } else if internal.hunger < 0.3 && internal.energy > 0.6 && actions.contains_key(&BirdAction::Cache) {
                // Cache food when well-fed and energetic
                actions.get(&BirdAction::Cache).map(|e| e.entity)
            } else if internal.thirst > 0.5 {
                actions.get(&BirdAction::Drink).map(|e| e.entity)
            } else if internal.energy < 0.3 && actions.contains_key(&BirdAction::Nest) {
                actions.get(&BirdAction::Nest).map(|e| e.entity)
            } else if internal.energy < 0.3 && actions.contains_key(&BirdAction::Perch) {
                actions.get(&BirdAction::Perch).map(|e| e.entity)
            } else if internal.energy > 0.7 && actions.contains_key(&BirdAction::Play) {
                actions.get(&BirdAction::Play).map(|e| e.entity)
            } else if internal.fear < 0.3 && actions.contains_key(&BirdAction::Explore) {
                actions.get(&BirdAction::Explore).map(|e| e.entity)
            } else if actions.contains_key(&BirdAction::Bathe) {
                actions.get(&BirdAction::Bathe).map(|e| e.entity)
            } else {
                actions.get(&BirdAction::Perch).map(|e| e.entity)
            };
        }
        
        *state = new_state;
    }
}

pub fn wandering_system(
    mut bird_query: Query<&mut Transform, (With<BirdAI>, With<BirdState>)>,
    state_query: Query<&BirdState, With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, state) in bird_query.iter_mut().zip(state_query.iter()) {
        if *state == BirdState::Wandering {
            execute_wandering(&mut transform, &time);
        }
    }
}

pub fn moving_to_target_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState), With<BirdAI>>,
    target_query: Query<&Transform, Without<BirdAI>>,
    time: Res<Time>,
) {
    for (mut bird_transform, mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::MovingToTarget {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    let reached = execute_moving_to_target(&mut bird_transform, target_transform, &time);
                    
                    if reached {
                        // Determine appropriate action based on the target's utility
                        if let Some(target_entity) = blackboard.current_target {
                            let actions = &blackboard.world_knowledge.available_actions;
                            
                            // Find what action this target provides
                            let target_action = actions.iter()
                                .find(|(_, entry)| entry.entity == target_entity)
                                .map(|(action, _)| action);
                                
                            if let Some(action) = target_action {
                                *state = match action {
                                    BirdAction::Eat => BirdState::Eating,
                                    BirdAction::Drink => BirdState::Drinking,
                                    BirdAction::Bathe => BirdState::Bathing,
                                    BirdAction::Perch => BirdState::Resting,
                                    BirdAction::Play => BirdState::Playing,
                                    BirdAction::Explore => BirdState::Exploring,
                                    BirdAction::Nest => BirdState::Nesting,
                                    BirdAction::Roost => BirdState::Roosting,
                                    BirdAction::Shelter => BirdState::Sheltering,
                                    BirdAction::Court => BirdState::Courting,
                                    BirdAction::Follow => BirdState::Following,
                                    BirdAction::Challenge => BirdState::Territorial,
                                    BirdAction::Flock => BirdState::Flocking,
                                    BirdAction::Forage => BirdState::Foraging,
                                    BirdAction::Cache => BirdState::Caching,
                                    BirdAction::Retrieve => BirdState::Retrieving,
                                    BirdAction::HoverFeed => BirdState::HoverFeeding,
                                };
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn eating_system(
    mut commands: Commands,
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    feeder_query: Query<&Feeder>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Eating {
            let consumption_rate = 0.5 * time.delta().as_secs_f32();
            blackboard.internal.hunger -= consumption_rate;
            blackboard.internal.hunger = blackboard.internal.hunger.max(0.0);
            
            // Trigger feeder depletion if eating from a feeder
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(feeder) = feeder_query.get(target_entity) {
                    commands.trigger(crate::feeder::FeederDepletionEvent {
                        feeder_entity: target_entity,
                        amount: feeder.depletion_rate * time.delta().as_secs_f32(),
                    });
                }
            }
            
            if blackboard.internal.hunger < 0.1 {
                *state = BirdState::Wandering;
                blackboard.current_target = None;
                info!("Bird finished eating and is now wandering");
            }
        }
    }
}

pub fn drinking_system(
    mut commands: Commands,
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    feeder_query: Query<&Feeder>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Drinking {
            let consumption_rate = 0.6 * time.delta().as_secs_f32();
            blackboard.internal.thirst -= consumption_rate;
            blackboard.internal.thirst = blackboard.internal.thirst.max(0.0);
            
            // Trigger feeder depletion if drinking from a nectar feeder
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(feeder) = feeder_query.get(target_entity) {
                    commands.trigger(crate::feeder::FeederDepletionEvent {
                        feeder_entity: target_entity,
                        amount: feeder.depletion_rate * time.delta().as_secs_f32(),
                    });
                }
            }
            
            if blackboard.internal.thirst < 0.1 {
                *state = BirdState::Wandering;
                blackboard.current_target = None;
                info!("Bird finished drinking and is now wandering");
            }
        }
    }
}

pub fn bathing_system(
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Bathing {
            blackboard.internal.energy += 0.3 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.min(1.0);
            
            // Bathing duration
            if blackboard.internal.energy > 0.8 {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn fleeing_system(
    mut bird_query: Query<(&mut Transform, &Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Fleeing {
            if let Some(threat_dir) = blackboard.world_knowledge.perceived_threat {
                execute_fleeing(&mut transform, threat_dir, &time);
                
                if blackboard.internal.fear < 0.3 {
                    *state = BirdState::Wandering;
                }
            }
        }
    }
}

pub fn resting_system(
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Resting {
            blackboard.internal.energy += 0.4 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.min(1.0);
            
            if blackboard.internal.energy > 0.7 {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn playing_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Playing {
            execute_playing(&mut transform, &time);
            
            // Playing is energizing but uses some energy over time
            blackboard.internal.energy -= 0.1 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.max(0.0);
            
            // Reduce fear through play (enrichment effect)
            blackboard.internal.fear -= 0.2 * time.delta().as_secs_f32();
            blackboard.internal.fear = blackboard.internal.fear.max(0.0);
            
            // Stop playing when energy gets low or after some time
            if blackboard.internal.energy < 0.3 || blackboard.internal.fear < 0.1 {
                *state = BirdState::Wandering;
                blackboard.current_target = None;
            }
        }
    }
}

pub fn exploring_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState), With<BirdAI>>,
    target_query: Query<&Transform, Without<BirdAI>>,
    time: Res<Time>,
) {
    for (mut bird_transform, mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Exploring {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    execute_exploring(&mut bird_transform, target_transform, &time);
                    
                    // Exploration slightly drains energy but satisfies curiosity
                    blackboard.internal.energy -= 0.05 * time.delta().as_secs_f32();
                    blackboard.internal.energy = blackboard.internal.energy.max(0.0);
                    
                    // Reduce fear through successful exploration
                    blackboard.internal.fear -= 0.1 * time.delta().as_secs_f32();
                    blackboard.internal.fear = blackboard.internal.fear.max(0.0);
                    
                    // Stop exploring when energy gets low or curiosity is satisfied
                    if blackboard.internal.energy < 0.4 || blackboard.internal.fear < 0.05 {
                        *state = BirdState::Wandering;
                        blackboard.current_target = None;
                    }
                }
            } else {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn nesting_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Nesting {
            execute_nesting(&mut transform, &time);
            
            // Nesting is very restorative
            blackboard.internal.energy += 0.6 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.min(1.0);
            
            // Reduces fear significantly (safe space)
            blackboard.internal.fear -= 0.3 * time.delta().as_secs_f32();
            blackboard.internal.fear = blackboard.internal.fear.max(0.0);
            
            // Birds stay in nests longer than other activities
            if blackboard.internal.energy > 0.9 {
                *state = BirdState::Wandering;
                blackboard.current_target = None;
            }
        }
    }
}

pub fn roosting_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Roosting {
            execute_roosting(&mut transform, &time);
            
            // Roosting is very restorative - birds gather in safe spots for the night
            blackboard.internal.energy += 0.4 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.min(1.0);
            
            // Roosting significantly reduces fear (safety in numbers)
            blackboard.internal.fear -= 0.4 * time.delta().as_secs_f32();
            blackboard.internal.fear = blackboard.internal.fear.max(0.0);
            
            // Birds typically roost for extended periods during evening/night
            if blackboard.internal.energy > 0.9 && blackboard.internal.fear < 0.1 {
                *state = BirdState::Wandering;
                blackboard.current_target = None;
                info!("Bird finished roosting and is now wandering");
            }
        }
    }
}

pub fn sheltering_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState), With<BirdAI>>,
    weather_state: Res<WeatherState>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Sheltering {
            execute_sheltering(&mut transform, &time);
            
            // Sheltering provides safety and conserves energy
            blackboard.internal.energy += 0.2 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.min(1.0);
            
            // Weather-induced fear reduction
            let weather_fear_reduction = weather_state.current_weather.weather_fear_factor() * 0.3;
            blackboard.internal.fear -= weather_fear_reduction * time.delta().as_secs_f32();
            blackboard.internal.fear = blackboard.internal.fear.max(0.0);
            
            // Continue sheltering while weather is bad or bird is still fearful
            let shelter_urgency = weather_state.current_weather.shelter_urgency();
            let weather_fear = weather_state.current_weather.weather_fear_factor();
            
            if shelter_urgency < 0.2 && blackboard.internal.fear + weather_fear < 0.4 {
                *state = BirdState::Wandering;
                blackboard.current_target = None;
                info!("Weather cleared, bird finished sheltering and is now wandering");
            }
        }
    }
}

pub fn courting_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &SocialBirdTraits), With<BirdAI>>,
    target_query: Query<&Transform, Without<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, social_traits) in bird_query.iter_mut() {
        if *state == BirdState::Courting {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    execute_courting(&mut transform, target_transform, &time);
                    
                    // Courting satisfies social need and can be energizing
                    blackboard.internal.social_need -= 0.4 * time.delta().as_secs_f32();
                    blackboard.internal.social_need = blackboard.internal.social_need.max(0.0);
                    
                    // Courting uses some energy
                    blackboard.internal.energy -= 0.2 * time.delta().as_secs_f32();
                    blackboard.internal.energy = blackboard.internal.energy.max(0.0);
                    
                    // Continue courting while social need exists and energy is sufficient
                    if blackboard.internal.social_need < 0.2 || blackboard.internal.energy < 0.3 {
                        *state = BirdState::Wandering;
                        blackboard.current_target = None;
                        info!("Bird finished courting and is now wandering");
                    }
                } else {
                    *state = BirdState::Wandering;
                    blackboard.current_target = None;
                }
            } else {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn territorial_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &SocialBirdTraits), With<BirdAI>>,
    target_query: Query<&Transform, Without<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, social_traits) in bird_query.iter_mut() {
        if *state == BirdState::Territorial {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    execute_territorial(&mut transform, target_transform, &time);
                    
                    // Territorial defense reduces territorial stress over time
                    blackboard.internal.territorial_stress -= 0.3 * time.delta().as_secs_f32();
                    blackboard.internal.territorial_stress = blackboard.internal.territorial_stress.max(0.0);
                    
                    // But uses significant energy
                    blackboard.internal.energy -= 0.4 * time.delta().as_secs_f32();
                    blackboard.internal.energy = blackboard.internal.energy.max(0.0);
                    
                    // Stop territorial behavior when stress is low or energy is depleted
                    if blackboard.internal.territorial_stress < 0.3 || blackboard.internal.energy < 0.2 {
                        *state = BirdState::Wandering;
                        blackboard.current_target = None;
                        info!("Bird finished territorial display and is now wandering");
                    }
                } else {
                    *state = BirdState::Wandering;
                    blackboard.current_target = None;
                }
            } else {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn flocking_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &SocialBirdTraits), With<BirdAI>>,
    target_query: Query<&Transform, Without<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, social_traits) in bird_query.iter_mut() {
        if *state == BirdState::Flocking {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    execute_flocking(&mut transform, target_transform, &time);
                    
                    // Flocking satisfies social need and provides safety (reduces fear)
                    blackboard.internal.social_need -= 0.3 * time.delta().as_secs_f32();
                    blackboard.internal.social_need = blackboard.internal.social_need.max(0.0);
                    
                    blackboard.internal.fear -= 0.2 * time.delta().as_secs_f32();
                    blackboard.internal.fear = blackboard.internal.fear.max(0.0);
                    
                    // Continue flocking while social need exists
                    if blackboard.internal.social_need < 0.1 {
                        *state = BirdState::Wandering;
                        blackboard.current_target = None;
                        info!("Bird left flock and is now wandering");
                    }
                } else {
                    *state = BirdState::Wandering;
                    blackboard.current_target = None;
                }
            } else {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn following_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &SocialBirdTraits), With<BirdAI>>,
    target_query: Query<&Transform, Without<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, social_traits) in bird_query.iter_mut() {
        if *state == BirdState::Following {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    execute_following(&mut transform, target_transform, &time);
                    
                    // Following satisfies social need
                    blackboard.internal.social_need -= 0.2 * time.delta().as_secs_f32();
                    blackboard.internal.social_need = blackboard.internal.social_need.max(0.0);
                    
                    // Light energy cost
                    blackboard.internal.energy -= 0.1 * time.delta().as_secs_f32();
                    blackboard.internal.energy = blackboard.internal.energy.max(0.0);
                    
                    // Stop following when social need is satisfied or energy is low
                    if blackboard.internal.social_need < 0.2 || blackboard.internal.energy < 0.3 {
                        *state = BirdState::Wandering;
                        blackboard.current_target = None;
                        info!("Bird stopped following and is now wandering");
                    }
                } else {
                    *state = BirdState::Wandering;
                    blackboard.current_target = None;
                }
            } else {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn need_decay_system(
    mut bird_query: Query<&mut Blackboard, With<BirdAI>>,
    weather_state: Res<WeatherState>,
    time: Res<Time>,
) {
    for mut blackboard in bird_query.iter_mut() {
        let decay_rate = 0.1 * time.delta().as_secs_f32();
        
        blackboard.internal.hunger += decay_rate;
        blackboard.internal.hunger = blackboard.internal.hunger.min(1.0);
        
        blackboard.internal.thirst += decay_rate * 1.2;
        blackboard.internal.thirst = blackboard.internal.thirst.min(1.0);
        
        blackboard.internal.energy -= decay_rate * 0.5;
        blackboard.internal.energy = blackboard.internal.energy.max(0.0);
        
        // Social needs increase over time
        blackboard.internal.social_need += decay_rate * 0.8;
        blackboard.internal.social_need = blackboard.internal.social_need.min(1.0);
        
        // Territorial stress increases when near rivals
        let rival_count = blackboard.world_knowledge.territory_challengers.len() as f32;
        if rival_count > 0.0 {
            blackboard.internal.territorial_stress += decay_rate * rival_count * 0.5;
            blackboard.internal.territorial_stress = blackboard.internal.territorial_stress.min(1.0);
        } else {
            blackboard.internal.territorial_stress *= 0.9; // Decays when no challengers
        }
        
        // Weather affects fear levels
        let weather_fear = weather_state.current_weather.weather_fear_factor();
        if weather_fear > 0.0 {
            blackboard.internal.fear += weather_fear * 0.5 * time.delta().as_secs_f32();
            blackboard.internal.fear = blackboard.internal.fear.min(1.0);
        } else {
            blackboard.internal.fear *= 0.95; // Fear decays faster in good weather
        }
    }
}

pub fn foraging_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &ForagingTraits, &mut ForagingState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, foraging_traits, mut foraging_state) in bird_query.iter_mut() {
        if *state == BirdState::Foraging {
            let mut rng = rand::rng();
            execute_foraging(&mut transform, foraging_traits, &mut foraging_state, &time, &mut rng);
            
            // Foraging gradually reduces hunger but uses energy
            blackboard.internal.hunger -= 0.3 * time.delta().as_secs_f32();
            blackboard.internal.hunger = blackboard.internal.hunger.max(0.0);
            
            blackboard.internal.energy -= 0.15 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.max(0.0);
            
            // Track energy spent and items found
            foraging_state.energy_spent += 0.15 * time.delta().as_secs_f32();
            
            // Occasionally find food items
            if rng.random_range(0.0..1.0) < 0.1 * time.delta().as_secs_f32() {
                foraging_state.items_found += 1;
                // Small hunger reduction for finding food
                blackboard.internal.hunger -= 0.1;
                blackboard.internal.hunger = blackboard.internal.hunger.max(0.0);
            }
            
            // Stop foraging when satisfied or exhausted
            if blackboard.internal.hunger < 0.3 || blackboard.internal.energy < 0.2 || foraging_state.energy_spent > 1.0 {
                *state = BirdState::Wandering;
                blackboard.current_target = None;
                foraging_state.energy_spent = 0.0;
                foraging_state.items_found = 0;
                info!("Bird finished foraging session");
            }
        }
    }
}

pub fn caching_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &ForagingTraits, &mut CacheData), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, foraging_traits, mut cache_data) in bird_query.iter_mut() {
        if *state == BirdState::Caching {
            execute_caching(&mut transform, &time);
            
            // Caching uses some energy but provides future security
            blackboard.internal.energy -= 0.1 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.max(0.0);
            
            // Create cache after some time
            let mut rng = rand::rng();
            if rng.random_range(0.0..1.0) < 0.3 * time.delta().as_secs_f32() && cache_data.current_cache_count < cache_data.max_cache_capacity {
                let cache_location = transform.translation.truncate() + Vec2::new(
                    rng.random_range(-50.0..50.0),
                    rng.random_range(-50.0..50.0)
                );
                
                cache_data.cached_locations.push(CacheSpot {
                    location: cache_location,
                    food_amount: rng.random_range(0.5..1.0),
                    cache_time: time.elapsed().as_secs_f64(),
                    decay_rate: 0.01, // Food spoils slowly
                });
                
                cache_data.current_cache_count += 1;
                
                // Slight hunger increase from giving up immediate food
                blackboard.internal.hunger += 0.05;
                blackboard.internal.hunger = blackboard.internal.hunger.min(1.0);
                
                *state = BirdState::Wandering;
                blackboard.current_target = None;
                info!("Bird cached food at {:?} (Total caches: {})", cache_location, cache_data.current_cache_count);
            }
        }
    }
}

pub fn retrieving_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &ForagingTraits, &mut CacheData), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, foraging_traits, mut cache_data) in bird_query.iter_mut() {
        if *state == BirdState::Retrieving {
            if let Some(target_entity) = blackboard.current_target {
                execute_retrieving(&mut transform, &time);
                
                // Light energy cost for retrieval
                blackboard.internal.energy -= 0.05 * time.delta().as_secs_f32();
                blackboard.internal.energy = blackboard.internal.energy.max(0.0);
                
                // Attempt to retrieve cached food
                let mut rng = rand::rng();
                if rng.random_range(0.0..1.0) < 0.4 * time.delta().as_secs_f32() {
                    // Find cache at current location (simplified)
                    let current_pos = transform.translation.truncate();
                    let cache_index = cache_data.cached_locations.iter().position(|cache| 
                        cache.location.distance(current_pos) < 30.0
                    );
                    
                    if let Some(index) = cache_index {
                        let cache = cache_data.cached_locations.remove(index);
                        cache_data.current_cache_count -= 1;
                        
                        // Reduce hunger based on cached food amount and decay
                        let food_value = cache.food_amount * (1.0 - cache.decay_rate * (time.elapsed().as_secs_f64() - cache.cache_time) as f32 / 3600.0);
                        blackboard.internal.hunger -= food_value * 0.5;
                        blackboard.internal.hunger = blackboard.internal.hunger.max(0.0);
                        
                        info!("Bird successfully retrieved cached food (value: {:.2})", food_value);
                    }
                    
                    *state = BirdState::Wandering;
                    blackboard.current_target = None;
                }
            } else {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn hover_feeding_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState, &ForagingTraits), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, mut blackboard, mut state, foraging_traits) in bird_query.iter_mut() {
        if *state == BirdState::HoverFeeding {
            if let Some(target_entity) = blackboard.current_target {
                execute_hover_feeding(&mut transform, &time);
                
                // Hover feeding has high energy cost but efficient feeding
                blackboard.internal.energy -= 0.4 * time.delta().as_secs_f32();
                blackboard.internal.energy = blackboard.internal.energy.max(0.0);
                
                // Very efficient hunger reduction
                blackboard.internal.hunger -= 0.6 * time.delta().as_secs_f32();
                blackboard.internal.hunger = blackboard.internal.hunger.max(0.0);
                
                // Stop hover feeding when satisfied or energy depleted
                if blackboard.internal.hunger < 0.1 || blackboard.internal.energy < 0.2 {
                    *state = BirdState::Wandering;
                    blackboard.current_target = None;
                    info!("Bird finished hover feeding");
                }
            } else {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn competitive_feeding_system(
    mut bird_query: Query<(Entity, &Transform, &mut Blackboard, &mut BirdState, &Bird), With<BirdAI>>,
    feeder_query: Query<(Entity, &Transform), (With<SmartObject>, With<crate::feeder::Feeder>)>,
    time: Res<Time>,
) {
    // Create a list of birds currently feeding or moving to feeders
    let mut feeding_birds: Vec<(Entity, Vec2, f32, crate::bird::BirdSpecies)> = Vec::new();
    
    for (entity, transform, blackboard, state, bird) in bird_query.iter() {
        if matches!(*state, BirdState::Eating | BirdState::MovingToTarget) {
            if let Some(_target) = blackboard.current_target {
                let aggression = bird.species.feeding_aggression_level();
                feeding_birds.push((entity, transform.translation.truncate(), aggression, bird.species));
            }
        }
    }
    
    // Check for feeding competition at each feeder
    for (feeder_entity, feeder_transform) in feeder_query.iter() {
        let feeder_pos = feeder_transform.translation.truncate();
        let competition_radius = 120.0; // Distance where competition occurs
        
        // Find birds competing for this feeder
        let competitors: Vec<_> = feeding_birds.iter()
            .filter(|(_, pos, _, _)| pos.distance(feeder_pos) < competition_radius)
            .collect();
            
        if competitors.len() > 1 {
            // Determine dominance hierarchy for this competition
            let mut dominant_bird: Option<(Entity, f32)> = None;
            let mut subordinates: Vec<Entity> = Vec::new();
            
            for (entity, _, aggression, species) in competitors.iter() {
                // Calculate dominance score based on aggression and size
                let size_factor = get_species_size_factor(*species);
                let dominance_score = *aggression * size_factor;
                
                if let Some((_, current_best)) = dominant_bird {
                    if dominance_score > current_best {
                        if let Some((old_dominant, _)) = dominant_bird {
                            subordinates.push(old_dominant);
                        }
                        dominant_bird = Some((*entity, dominance_score));
                    } else {
                        subordinates.push(*entity);
                    }
                } else {
                    dominant_bird = Some((*entity, dominance_score));
                }
            }
            
            // Apply competition effects
            if let Some((dominant_entity, _)) = dominant_bird {
                // Dominant bird gets feeding priority - no changes needed
                
                // Subordinate birds may be displaced
                for subordinate_entity in subordinates {
                    if let Ok((_, _, mut blackboard, mut state, bird)) = bird_query.get_mut(subordinate_entity) {
                        let (feeding_duration, feeding_frequency, group_tolerance) = bird.species.feeding_style_traits();
                        
                        // Check if bird should be displaced based on tolerance and competition intensity  
                        let displacement_chance = (1.0 - group_tolerance) * (competitors.len() as f32 - 1.0) * 0.3;
                        
                        if rand::rng().random::<f32>() < displacement_chance * time.delta().as_secs_f32() {
                            // Bird is displaced from feeder
                            *state = BirdState::Wandering;
                            blackboard.current_target = None;
                            
                            // Increase territorial stress slightly
                            blackboard.internal.territorial_stress += 0.1;
                            blackboard.internal.territorial_stress = blackboard.internal.territorial_stress.min(1.0);
                            
                            info!("Bird displaced from feeder due to competition");
                        }
                    }
                }
            }
        }
    }
}

// Helper function to determine relative size factor for dominance calculations
fn get_species_size_factor(species: crate::bird::BirdSpecies) -> f32 {
    use crate::bird::BirdSpecies as BS;
    match species {
        // Large birds
        BS::PileatedWoodpecker | BS::CommonCrow | BS::BaldEagle => 2.0,
        BS::RedTailedHawk | BS::CoopersHawk | BS::GreatHornedOwl | BS::BarredOwl => 1.8,
        
        // Medium-large birds
        BS::BlueJay | BS::NorthernMockingbird | BS::CommonGrackle => 1.4,
        BS::Cardinal | BS::BrownThrasher | BS::MourningDove => 1.2,
        
        // Medium birds
        BS::Robin | BS::RedWingedBlackbird | BS::EuropeanStarling => 1.0,
        BS::HairyWoodpecker | BS::RedHeadedWoodpecker | BS::BaltimoreOriole => 0.9,
        
        // Small birds  
        BS::DownyWoodpecker | BS::HouseFinch | BS::PurpleFinch => 0.7,
        BS::Sparrow | BS::Goldfinch | BS::Chickadee | BS::TuftedTitmouse => 0.6,
        
        // Very small birds
        BS::WhiteBreastedNuthatch | BS::CarolinaWren | BS::BrownCreeper => 0.4,
        BS::RubyThroatedHummingbird | BS::WinterWren | BS::BlueGrayGnatcatcher => 0.2,
        
        _ => 0.8, // Default medium-small
    }
}

// Helper functions for foraging behaviors
fn calculate_social_compatibility(
    species1: crate::bird::BirdSpecies,
    species2: crate::bird::BirdSpecies,
    traits1: &SocialBirdTraits,
    traits2: &SocialBirdTraits,
) -> f32 {
    let mut compatibility = 0.5; // Base compatibility
    
    // Same species have higher compatibility
    if species1 == species2 {
        compatibility += 0.3;
    }
    
    // Similar social tolerance levels increase compatibility
    let tolerance_diff = (traits1.social_tolerance - traits2.social_tolerance).abs();
    compatibility += (1.0 - tolerance_diff) * 0.2;
    
    // High dominance difference can create either attraction or conflict
    let dominance_diff = (traits1.dominance_level - traits2.dominance_level).abs();
    if dominance_diff > 0.3 {
        // Significant dominance difference - can be attractive for mating
        compatibility += 0.1;
    } else {
        // Similar dominance - good for flocking
        compatibility += 0.2;
    }
    
    // Flock tendency increases mixed-species compatibility
    compatibility += (traits1.flock_tendency + traits2.flock_tendency) * 0.1;
    
    compatibility.clamp(0.0, 1.0)
}

fn calculate_mate_utility(
    social_traits: &SocialBirdTraits,
    mate_info: &SocialBirdInfo,
    time_state: &TimeState,
) -> f32 {
    let mut utility = social_traits.mating_receptivity;
    
    // Distance factor - closer is better
    let distance_factor = (200.0 - mate_info.distance.min(200.0)) / 200.0;
    utility *= distance_factor;
    
    // Social compatibility
    utility *= mate_info.social_compatibility;
    
    // Breeding season bonus
    if is_breeding_season(time_state) {
        utility *= 1.5;
    }
    
    // Time of day factor - more active during dawn/dusk
    if time_state.is_dawn_chorus() || time_state.is_evening_song() {
        utility *= 1.3;
    }
    
    utility.clamp(0.0, 1.0)
}

fn is_breeding_season(time_state: &TimeState) -> bool {
    // Spring and early summer are breeding seasons
    matches!(time_state.hour, 6.0..=18.0) && time_state.day_of_year > 80 && time_state.day_of_year < 200
}
use crate::bird_ai::components::*;
use crate::bird_ai::config::*;
use crate::environment::resources::{TimeState, WeatherState};

// Legacy rule struct - kept for compatibility
pub struct Rule {
    pub priority: u32,
    pub check: fn(&Blackboard, &TimeState, &WeatherState) -> bool,
    pub result: BirdState,
}

// New configurable behavior tree evaluator
pub fn evaluate_behavior_tree_configurable(
    blackboard: &Blackboard,
    time_state: &TimeState,
    weather_state: &WeatherState,
    config: &BehaviorTreeConfig,
) -> BirdState {
    let internal = &blackboard.internal;
    let world = &blackboard.world_knowledge;
    
    // Sort rules by priority (higher first)
    let mut sorted_rules = config.rules.clone();
    sorted_rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    
    // Evaluate rules in priority order
    for rule in &sorted_rules {
        if evaluate_rule_conditions(&rule.conditions, blackboard, time_state, weather_state, config) {
            return config.get_bird_state_from_string(&rule.result);
        }
    }
    
    // Return default behavior if no rules match
    config.get_bird_state_from_string(&config.default_behavior)
}

// Legacy behavior tree evaluator - kept for backward compatibility
pub fn evaluate_behavior_tree(blackboard: &Blackboard, time_state: &TimeState, weather_state: &WeatherState) -> BirdState {
    let internal = &blackboard.internal;
    let world = &blackboard.world_knowledge;
    
    // Highest priority: Weather-induced fear and shelter seeking
    let weather = weather_state.current_weather;
    let weather_fear = weather.weather_fear_factor();
    let shelter_urgency = weather.shelter_urgency();
    
    // Weather increases fear levels
    if internal.fear + weather_fear > 0.7 {
        return BirdState::Fleeing;
    }
    
    // Critical weather conditions - seek shelter immediately
    if shelter_urgency > 0.6 && world.available_actions.contains_key(&BirdAction::Shelter) {
        return BirdState::MovingToTarget;
    }
    
    // Moderate weather conditions - prefer shelter if available
    if shelter_urgency > 0.3 && internal.energy < 0.7 && world.available_actions.contains_key(&BirdAction::Shelter) {
        return BirdState::MovingToTarget;
    }
    
    // Roosting behavior during dusk/evening hours (high priority before nightfall)
    if time_state.hour >= 18.0 && time_state.hour <= 20.0 {
        if world.available_actions.contains_key(&BirdAction::Roost) {
            return BirdState::MovingToTarget;
        }
        // If no roosting spots available, prioritize safe perching
        if world.available_actions.contains_key(&BirdAction::Perch) {
            return BirdState::MovingToTarget;
        }
    }
    
    // Critical needs - survival first
    if internal.hunger > 0.8 && world.available_actions.contains_key(&BirdAction::Eat) {
        return BirdState::MovingToTarget;
    }
    
    if internal.thirst > 0.8 && world.available_actions.contains_key(&BirdAction::Drink) {
        return BirdState::MovingToTarget;
    }
    
    // Low energy - need rest
    if internal.energy < 0.3 {
        // Prefer nesting areas if available for resting
        if world.available_actions.contains_key(&BirdAction::Nest) {
            return BirdState::MovingToTarget;
        } else if world.available_actions.contains_key(&BirdAction::Perch) {
            return BirdState::MovingToTarget;
        }
        return BirdState::Resting;
    }
    
    // Moderate needs - include foraging behaviors
    if internal.hunger > 0.6 && world.available_actions.contains_key(&BirdAction::HoverFeed) {
        return BirdState::MovingToTarget;
    }
    
    if internal.hunger > 0.5 && world.available_actions.contains_key(&BirdAction::Eat) {
        return BirdState::MovingToTarget;
    }
    
    // Ground foraging when moderately hungry but no feeders available
    if internal.hunger > 0.4 && !world.available_actions.contains_key(&BirdAction::Eat) {
        return BirdState::Foraging;
    }
    
    // Cache behavior when food is abundant but not immediately needed
    if internal.hunger < 0.3 && internal.energy > 0.6 && world.available_actions.contains_key(&BirdAction::Cache) {
        return BirdState::MovingToTarget;
    }
    
    // Retrieve cached food when hungry and no other food sources available
    if internal.hunger > 0.6 && !world.available_actions.contains_key(&BirdAction::Eat) && world.available_actions.contains_key(&BirdAction::Retrieve) {
        return BirdState::MovingToTarget;
    }
    
    if internal.thirst > 0.5 && world.available_actions.contains_key(&BirdAction::Drink) {
        return BirdState::MovingToTarget;
    }
    
    // Bathing behavior (hygiene and temperature regulation)
    if world.available_actions.contains_key(&BirdAction::Bathe) {
        return BirdState::MovingToTarget;
    }
    
    // Social behaviors - when basic needs are met, engage in social activities
    // Territorial disputes take priority when stress is high
    if internal.territorial_stress > 0.6 && world.available_actions.contains_key(&BirdAction::Challenge) {
        return BirdState::MovingToTarget;
    }
    
    // Mating behavior during breeding season with high social need
    if internal.social_need > 0.5 && world.available_actions.contains_key(&BirdAction::Court) {
        return BirdState::MovingToTarget;
    }
    
    // Flocking behavior for social species when social need is moderate
    if internal.social_need > 0.4 && world.available_actions.contains_key(&BirdAction::Flock) {
        return BirdState::MovingToTarget;
    }
    
    // Following behavior for social interaction
    if internal.social_need > 0.3 && internal.fear < 0.4 && world.available_actions.contains_key(&BirdAction::Follow) {
        return BirdState::MovingToTarget;
    }
    
    // Enrichment behaviors - when basic needs are met, explore environment
    // High energy birds are more likely to engage in enrichment activities
    if internal.energy > 0.6 {
        // Nesting behavior (seasonal and species-dependent)
        if world.available_actions.contains_key(&BirdAction::Nest) && internal.energy > 0.8 {
            return BirdState::MovingToTarget;
        }
        
        // Play behavior for high-energy birds
        if world.available_actions.contains_key(&BirdAction::Play) && internal.energy > 0.7 {
            return BirdState::MovingToTarget;
        }
        
        // Exploration and curiosity
        if world.available_actions.contains_key(&BirdAction::Explore) && internal.fear < 0.3 {
            return BirdState::MovingToTarget;
        }
        
        // Perching for observation and rest
        if world.available_actions.contains_key(&BirdAction::Perch) {
            return BirdState::MovingToTarget;
        }
    }
    
    // Default behavior - wander and look for opportunities
    BirdState::Wandering
}

// Rule condition evaluation
fn evaluate_rule_conditions(
    conditions: &[BehaviorCondition],
    blackboard: &Blackboard,
    time_state: &TimeState,
    weather_state: &WeatherState,
    config: &BehaviorTreeConfig,
) -> bool {
    let internal = &blackboard.internal;
    let world = &blackboard.world_knowledge;
    
    for condition in conditions {
        let condition_met = match condition {
            BehaviorCondition::WeatherFear { threshold } => {
                let weather_fear = weather_state.current_weather.weather_fear_factor();
                internal.fear + weather_fear > *threshold
            }
            BehaviorCondition::WeatherShelterUrgency { threshold } => {
                let shelter_urgency = weather_state.current_weather.shelter_urgency();
                shelter_urgency > *threshold
            }
            BehaviorCondition::InternalStateAbove { state, threshold } => {
                match state.as_str() {
                    "hunger" => internal.hunger > *threshold,
                    "thirst" => internal.thirst > *threshold,
                    "energy" => internal.energy > *threshold,
                    "fear" => internal.fear > *threshold,
                    "social_need" => internal.social_need > *threshold,
                    "territorial_stress" => internal.territorial_stress > *threshold,
                    _ => false,
                }
            }
            BehaviorCondition::InternalStateBelow { state, threshold } => {
                match state.as_str() {
                    "hunger" => internal.hunger < *threshold,
                    "thirst" => internal.thirst < *threshold,
                    "energy" => internal.energy < *threshold,
                    "fear" => internal.fear < *threshold,
                    "social_need" => internal.social_need < *threshold,
                    "territorial_stress" => internal.territorial_stress < *threshold,
                    _ => false,
                }
            }
            BehaviorCondition::TimeRange { start, end } => {
                time_state.hour >= *start && time_state.hour <= *end
            }
            BehaviorCondition::ActionAvailable(action_str) => {
                if let Some(action) = config.get_bird_action_from_string(action_str) {
                    world.available_actions.contains_key(&action)
                } else {
                    false
                }
            }
            BehaviorCondition::ActionNotAvailable(action_str) => {
                if let Some(action) = config.get_bird_action_from_string(action_str) {
                    !world.available_actions.contains_key(&action)
                } else {
                    true
                }
            }
        };
        
        if !condition_met {
            return false;
        }
    }
    
    true
}

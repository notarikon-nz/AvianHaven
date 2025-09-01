use crate::bird_ai::components::*;
use crate::environment::resources::{TimeState, WeatherState};

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

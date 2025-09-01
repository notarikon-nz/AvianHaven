use crate::bird_ai::components::*;

pub fn evaluate_behavior_tree(blackboard: &Blackboard) -> BirdState {
    let internal = &blackboard.internal;
    let world = &blackboard.world_knowledge;
    
    // High priority: Fear response
    if internal.fear > 0.7 {
        return BirdState::Fleeing;
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
    
    // Moderate needs
    if internal.hunger > 0.5 && world.available_actions.contains_key(&BirdAction::Eat) {
        return BirdState::MovingToTarget;
    }
    
    if internal.thirst > 0.5 && world.available_actions.contains_key(&BirdAction::Drink) {
        return BirdState::MovingToTarget;
    }
    
    // Bathing behavior (hygiene and temperature regulation)
    if world.available_actions.contains_key(&BirdAction::Bathe) {
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

use crate::bird_ai::components::*;

pub fn evaluate_behavior_tree(blackboard: &Blackboard) -> BirdState {
    let internal = &blackboard.internal;
    let world = &blackboard.world_knowledge;
    
    // High priority: Fear response
    if internal.fear > 0.7 {
        return BirdState::Fleeing;
    }
    
    // Critical needs
    if internal.hunger > 0.8 && world.available_actions.contains_key(&BirdAction::Eat) {
        return BirdState::MovingToTarget;
    }
    
    if internal.thirst > 0.8 && world.available_actions.contains_key(&BirdAction::Drink) {
        return BirdState::MovingToTarget;
    }
    
    // Low energy
    if internal.energy < 0.3 {
        return BirdState::Resting;
    }
    
    // Moderate needs
    if internal.hunger > 0.5 && world.available_actions.contains_key(&BirdAction::Eat) {
        return BirdState::MovingToTarget;
    }
    
    if internal.thirst > 0.5 && world.available_actions.contains_key(&BirdAction::Drink) {
        return BirdState::MovingToTarget;
    }
    
    if world.available_actions.contains_key(&BirdAction::Bathe) {
        return BirdState::MovingToTarget;
    }
    
    // Default behavior
    BirdState::Wandering
}

use bevy::prelude::*;
use crate::bird_ai::components::{BirdState, BirdAction, Blackboard};
use crate::environment::resources::{TimeState, WeatherState};

// Lua API functions that scripts can call

pub fn log_info(message: String) {
    info!("Lua Script: {}", message);
}

pub fn get_bird_hunger() -> f32 {
    // This would need access to the specific bird's blackboard
    // For now, return a placeholder value
    // In a real implementation, you'd pass bird entity ID and look it up
    0.5
}

pub fn set_bird_state(state_name: String) -> bool {
    // This would set the bird's state based on the string
    // Return success/failure
    info!("Lua requested state change to: {}", state_name);
    true
}

pub fn check_action_available(action_name: String) -> bool {
    // This would check if a specific action is available to the bird
    // For now, return placeholder
    match action_name.as_str() {
        "Eat" | "Drink" | "Perch" => true,
        _ => false,
    }
}

pub fn get_weather_fear() -> f32 {
    // This would get the current weather fear factor
    // Placeholder implementation
    0.2
}

pub fn get_time_of_day() -> f32 {
    // This would get the current game time
    // Placeholder implementation
    12.0
}

pub fn random_float() -> f32 {
    rand::random::<f32>()
}

pub fn distance_to_target() -> f32 {
    // This would calculate distance to the current target
    // Placeholder implementation
    50.0
}

// More advanced API functions that would require system access
pub struct LuaBirdAPI {
    pub entity: Entity,
}

impl LuaBirdAPI {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
    
    // These would be instance methods that have access to the specific bird
    pub fn get_hunger(&self, world: &World) -> f32 {
        if let Some(blackboard) = world.entity(self.entity).get::<Blackboard>() {
            blackboard.internal.hunger
        } else {
            0.0
        }
    }
    
    pub fn get_energy(&self, world: &World) -> f32 {
        if let Some(blackboard) = world.entity(self.entity).get::<Blackboard>() {
            blackboard.internal.energy
        } else {
            0.0
        }
    }
    
    pub fn has_action_available(&self, world: &World, action: BirdAction) -> bool {
        if let Some(blackboard) = world.entity(self.entity).get::<Blackboard>() {
            blackboard.world_knowledge.available_actions.contains_key(&action)
        } else {
            false
        }
    }
}
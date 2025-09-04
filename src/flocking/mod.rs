use bevy::prelude::*;

pub mod components;
pub mod systems;

// use components::*;
use systems::*;

pub struct FlockingPlugin;

impl Plugin for FlockingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                flocking_behavior_system,
                territorial_behavior_system,
                predator_avoidance_system,
                social_feeding_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}
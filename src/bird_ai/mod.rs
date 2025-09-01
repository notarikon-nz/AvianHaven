use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
pub mod bt;
pub mod states;

use resources::*;
use systems::*;
use crate::{AppState};

pub struct BirdAiPlugin;

impl Plugin for BirdAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UtilityTimer>()
            .init_resource::<BehaviorTreeTimer>()
            .add_systems(Startup, setup_test_world)
            .add_systems(Update, (
                // Core AI systems
                world_utility_query_system,
                social_awareness_system,
                behavior_tree_system,
                need_decay_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Basic behavior systems
                wandering_system,
                moving_to_target_system,
                eating_system,
                drinking_system,
                bathing_system,
                fleeing_system,
                resting_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Advanced behavior systems
                playing_system,
                exploring_system,
                nesting_system,
                roosting_system,
                sheltering_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Social behavior systems
                courting_system,
                territorial_system,
                flocking_system,
                following_system,
            ).run_if(in_state(AppState::Playing)))
            .add_systems(Update, (
                // Foraging behavior systems
                foraging_system,
                caching_system,
                retrieving_system,
                hover_feeding_system,
                competitive_feeding_system,
            ).run_if(in_state(AppState::Playing)));
    }
}

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
                world_utility_query_system,
                behavior_tree_system,
                wandering_system,
                moving_to_target_system,
                eating_system,
                drinking_system,
                bathing_system,
                fleeing_system,
                resting_system,
                playing_system,
                exploring_system,
                nesting_system,
                need_decay_system,
            ).run_if(in_state(AppState::Playing)));
    }
}

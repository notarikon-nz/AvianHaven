use bevy::prelude::*;

pub mod systems;

use systems::*;

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                bird_culling_system,
                performance_monitoring_system,
                memory_optimization_system,
            ));
    }
}
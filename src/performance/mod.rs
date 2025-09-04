use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;
pub mod profiling;

// use components::*;
use resources::*;
use systems::*;
use profiling::*;

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FpsCounter>()
            .init_resource::<PerformanceMetrics>()
            .init_resource::<ProfilingData>()
            .init_resource::<PerformanceSettings>()
            .add_systems(Update, (
                // Existing performance systems
                bird_culling_system,
                performance_monitoring_system,
                memory_optimization_system,
                // New profiling and monitoring systems
                fps_counter_system,
                performance_metrics_system,
                profiling_system,
                fps_display_system,
                performance_display_system,
            ));
    }
}
use bevy::prelude::*;

/// Component for UI elements that display FPS information
#[derive(Component)]
pub struct FpsDisplay;

/// Component for UI elements that display performance metrics
#[derive(Component)]
pub struct PerformanceDisplay;

/// Component for UI elements that display profiling data
#[derive(Component)]
pub struct ProfilingDisplay;

/// Component to mark entities that should be included in performance monitoring
#[derive(Component)]
pub struct PerformanceMonitored {
    pub system_name: String,
}

/// Component for tracking entity count in different categories
#[derive(Component)]
pub struct EntityCounter {
    pub category: String,
    pub count: usize,
}
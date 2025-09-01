use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use std::time::Instant;
use crate::bird::Bird;
use crate::bird_ai::components::BirdAI;
use crate::feeder::Feeder;
use crate::performance::resources::*;
use crate::performance::components::*;

pub fn bird_culling_system(
    mut commands: Commands,
    camera_query: Query<&Transform, (With<Camera2d>, Without<Bird>)>,
    bird_query: Query<(Entity, &Transform), With<Bird>>,
    bird_count: ResMut<crate::resources::BirdCount>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    
    let camera_pos = camera_transform.translation.truncate();
    let cull_distance = 800.0; // Birds beyond this distance get culled
    let max_birds = 15;
    
    let mut birds_to_cull = Vec::new();
    let mut bird_distances: Vec<(Entity, f32)> = Vec::new();
    
    // Calculate distances from camera
    for (entity, transform) in &bird_query {
        let distance = camera_pos.distance(transform.translation.truncate());
        bird_distances.push((entity, distance));
        
        // Mark very distant birds for culling
        if distance > cull_distance {
            birds_to_cull.push(entity);
        }
    }
    
    // If we're over the bird limit, cull the furthest birds
    if bird_distances.len() > max_birds {
        // Sort by distance (furthest first)
        bird_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Cull excess birds
        for (entity, _) in bird_distances.iter().skip(max_birds) {
            birds_to_cull.push(*entity);
        }
    }
    
    // Perform culling
    for entity in birds_to_cull {
        commands.entity(entity).despawn();
    }
}

pub fn performance_monitoring_system(
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    time: Res<Time>,
) {
    // Log performance metrics periodically
    static mut LAST_LOG: f32 = 0.0;
    
    unsafe {
        LAST_LOG += time.delta().as_secs_f32();
        if LAST_LOG > 10.0 { // Log every 10 seconds
            if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    if average < 50.0 {
                        warn!("Performance warning: FPS dropped to {:.1}", average);
                    }
                }
            }
            LAST_LOG = 0.0;
        }
    }
}

pub fn memory_optimization_system(
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
) {
    // Periodic cleanup of unused assets
    static mut CLEANUP_TIMER: f32 = 0.0;
    
    unsafe {
        CLEANUP_TIMER += time.delta().as_secs_f32();
        if CLEANUP_TIMER > 60.0 { // Cleanup every minute
            // In a real implementation, this would remove unused image assets
            // For now, just log that cleanup would occur
            let image_count = images.len();
            if image_count > 100 {
                info!("Memory optimization: {} images in memory", image_count);
            }
            CLEANUP_TIMER = 0.0;
        }
    }
}

/// Enhanced FPS counter system with smoothing and statistics
pub fn fps_counter_system(
    mut fps_counter: ResMut<FpsCounter>,
    diagnostics: Res<DiagnosticsStore>,
    _time: Res<Time>,
) {
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(current_fps) = fps_diagnostic.smoothed() {
            fps_counter.current_fps = current_fps as f32;
            
            // Update min/max
            fps_counter.min_fps = fps_counter.min_fps.min(current_fps as f32);
            fps_counter.max_fps = fps_counter.max_fps.max(current_fps as f32);
            
            // Add to rolling average
            fps_counter.frame_times.push_back(current_fps as f32);
            if fps_counter.frame_times.len() > 60 {
                fps_counter.frame_times.pop_front();
            }
            
            // Calculate average
            if !fps_counter.frame_times.is_empty() {
                fps_counter.average_fps = fps_counter.frame_times.iter().sum::<f32>() / fps_counter.frame_times.len() as f32;
            }
        }
    }
    
    fps_counter.last_update = Instant::now();
}

/// Comprehensive performance metrics collection
pub fn performance_metrics_system(
    mut metrics: ResMut<PerformanceMetrics>,
    fps_counter: Res<FpsCounter>,
    bird_query: Query<(), With<Bird>>,
    ai_bird_query: Query<(), (With<Bird>, With<BirdAI>)>,
    feeder_query: Query<(), With<Feeder>>,
    ui_query: Query<(), With<Node>>,
    world: &World,
) {
    // Update entity counts
    metrics.total_entities = world.entities().len() as usize;
    metrics.bird_count = bird_query.iter().count();
    metrics.ai_bird_count = ai_bird_query.iter().count();
    metrics.feeder_count = feeder_query.iter().count();
    metrics.ui_element_count = ui_query.iter().count();
    
    // Estimate memory usage (rough approximations)
    metrics.component_memory_mb = (metrics.total_entities * 500) as f32 / 1024.0 / 1024.0; // ~500 bytes per entity avg
    metrics.resource_memory_mb = 16.0; // Rough estimate for resources
    metrics.estimated_memory_mb = metrics.component_memory_mb + metrics.resource_memory_mb;
    
    // Update historical data
    metrics.fps_history.push_back(fps_counter.current_fps);
    if metrics.fps_history.len() > 300 {
        metrics.fps_history.pop_front();
    }
    
    let total_entities = metrics.total_entities;
    let estimated_memory = metrics.estimated_memory_mb;
    
    metrics.entity_count_history.push_back(total_entities);
    if metrics.entity_count_history.len() > 300 {
        metrics.entity_count_history.pop_front();
    }
    
    metrics.memory_history.push_back(estimated_memory);
    if metrics.memory_history.len() > 300 {
        metrics.memory_history.pop_front();
    }
}

/// System to display FPS counter in UI
pub fn fps_display_system(
    mut commands: Commands,
    fps_counter: Res<FpsCounter>,
    settings: Res<PerformanceSettings>,
    fps_display_query: Query<Entity, With<FpsDisplay>>,
    mut text_query: Query<&mut Text, With<FpsDisplay>>,
) {
    if !settings.enable_fps_display {
        return;
    }
    
    // Create FPS display if it doesn't exist
    if fps_display_query.is_empty() {
        commands.spawn((
            Text::new(format!("FPS: {:.1}", fps_counter.current_fps)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            FpsDisplay,
            TextColor(Color::WHITE),
            TextFont {
                font_size: 24.0,
                ..default()
            },
        ));
    } else {
        // Update existing FPS display
        for mut text in text_query.iter_mut() {
            text.0 = format!(
                "FPS: {:.1}\nAvg: {:.1}\nMin: {:.1} Max: {:.1}",
                fps_counter.current_fps,
                fps_counter.average_fps,
                fps_counter.min_fps,
                fps_counter.max_fps
            );
        }
    }
}

/// System to display detailed performance metrics
pub fn performance_display_system(
    mut commands: Commands,
    metrics: Res<PerformanceMetrics>,
    settings: Res<PerformanceSettings>,
    profiling_data: Res<ProfilingData>,
    display_query: Query<Entity, With<PerformanceDisplay>>,
    mut text_query: Query<&mut Text, With<PerformanceDisplay>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Toggle performance display with F3 key
    static mut DISPLAY_ENABLED: bool = false;
    
    unsafe {
        if input.just_pressed(KeyCode::F3) {
            DISPLAY_ENABLED = !DISPLAY_ENABLED;
        }
        
        if !DISPLAY_ENABLED || !settings.enable_performance_display {
            // Remove display if disabled
            for entity in display_query.iter() {
                commands.entity(entity).despawn();
            }
            return;
        }
    }
    
    // Create performance display if it doesn't exist
    if display_query.is_empty() {
        commands.spawn((
            Text::new("Performance Data".to_string()),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(10.0),
                width: Val::Px(300.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            PerformanceDisplay,
            TextColor(Color::WHITE),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ));
    } else {
        // Update existing performance display
        for mut text in text_query.iter_mut() {
            let warnings_text = if metrics.warnings.is_empty() {
                "No warnings".to_string()
            } else {
                format!("⚠ {}", metrics.warnings.join("\n⚠ "))
            };
            
            text.0 = format!(
                "=== PERFORMANCE METRICS ===\n\
                Entities: {} (Birds: {}, AI: {})\n\
                Memory: {:.1} MB\n\
                Feeders: {}, UI: {}\n\
                \n\
                === BOTTLENECKS ===\n\
                Active: {}\n\
                \n\
                === WARNINGS ===\n\
                {}\n\
                \n\
                Press F3 to toggle",
                metrics.total_entities,
                metrics.bird_count,
                metrics.ai_bird_count,
                metrics.estimated_memory_mb,
                metrics.feeder_count,
                metrics.ui_element_count,
                profiling_data.bottlenecks.len(),
                warnings_text
            );
        }
    }
}
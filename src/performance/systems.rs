use bevy::prelude::*;
use crate::bird::Bird;
use crate::bird_ai::components::BirdAI;

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
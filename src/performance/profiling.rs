use bevy::prelude::*;
use std::time::Instant;
use crate::performance::resources::*;

/// System for comprehensive performance profiling
pub fn profiling_system(
    mut profiling_data: ResMut<ProfilingData>,
    mut performance_metrics: ResMut<PerformanceMetrics>,
    settings: Res<PerformanceSettings>,
    _time: Res<Time>,
) {
    if !settings.enable_profiling {
        return;
    }
    
    let now = Instant::now();
    if now.duration_since(profiling_data.last_profile_time).as_millis() 
        < settings.profiling_interval_ms as u128 {
        return;
    }
    
    profiling_data.last_profile_time = now;
    
    // Clear old bottlenecks (older than 30 seconds)
    profiling_data.bottlenecks.retain(|bottleneck| {
        now.duration_since(bottleneck.detected_at).as_secs() < 30
    });
    
    // Check for performance bottlenecks
    check_fps_bottlenecks(&mut profiling_data, &performance_metrics, &settings);
    check_memory_bottlenecks(&mut profiling_data, &performance_metrics, &settings);
    check_entity_count_bottlenecks(&mut profiling_data, &performance_metrics, &settings);
    
    // Update performance warnings
    performance_metrics.warnings.clear();
    for bottleneck in &profiling_data.bottlenecks {
        if matches!(bottleneck.severity, BottleneckSeverity::High | BottleneckSeverity::Critical) {
            performance_metrics.warnings.push(format!(
                "{}: {}", 
                bottleneck.system_name, 
                bottleneck.description
            ));
        }
    }
}

fn check_fps_bottlenecks(
    profiling_data: &mut ResMut<ProfilingData>,
    metrics: &PerformanceMetrics,
    settings: &PerformanceSettings,
) {
    if metrics.fps_history.len() < 30 { // Need at least 30 frames of data
        return;
    }
    
    let recent_fps: Vec<f32> = metrics.fps_history.iter().rev().take(30).cloned().collect();
    let avg_recent_fps = recent_fps.iter().sum::<f32>() / recent_fps.len() as f32;
    
    if avg_recent_fps < settings.min_acceptable_fps {
        let severity = if avg_recent_fps < settings.min_acceptable_fps * 0.5 {
            BottleneckSeverity::Critical
        } else if avg_recent_fps < settings.min_acceptable_fps * 0.75 {
            BottleneckSeverity::High
        } else {
            BottleneckSeverity::Medium
        };
        
        profiling_data.bottlenecks.push(PerformanceBottleneck {
            system_name: "Frame Rate".to_string(),
            issue_type: BottleneckType::LowFrameRate,
            severity,
            description: format!("FPS below target: {:.1} (target: {:.1})", avg_recent_fps, settings.target_fps),
            suggested_fix: "Consider reducing bird count, disabling complex behaviors, or optimizing systems".to_string(),
            detected_at: Instant::now(),
        });
    }
}

fn check_memory_bottlenecks(
    profiling_data: &mut ResMut<ProfilingData>,
    metrics: &PerformanceMetrics,
    settings: &PerformanceSettings,
) {
    if metrics.estimated_memory_mb > settings.memory_warning_threshold_mb {
        let severity = if metrics.estimated_memory_mb > settings.memory_warning_threshold_mb * 1.5 {
            BottleneckSeverity::High
        } else {
            BottleneckSeverity::Medium
        };
        
        profiling_data.bottlenecks.push(PerformanceBottleneck {
            system_name: "Memory Usage".to_string(),
            issue_type: BottleneckType::HighMemoryUsage,
            severity,
            description: format!("High memory usage: {:.1} MB", metrics.estimated_memory_mb),
            suggested_fix: "Consider reducing entity count or implementing more aggressive culling".to_string(),
            detected_at: Instant::now(),
        });
    }
}

fn check_entity_count_bottlenecks(
    profiling_data: &mut ResMut<ProfilingData>,
    metrics: &PerformanceMetrics,
    settings: &PerformanceSettings,
) {
    if metrics.total_entities > settings.max_entities {
        let severity = if metrics.total_entities > settings.max_entities * 2 {
            BottleneckSeverity::High
        } else {
            BottleneckSeverity::Medium
        };
        
        profiling_data.bottlenecks.push(PerformanceBottleneck {
            system_name: "Entity Count".to_string(),
            issue_type: BottleneckType::TooManyEntities,
            severity,
            description: format!("High entity count: {} (limit: {})", metrics.total_entities, settings.max_entities),
            suggested_fix: "Implement entity pooling or more aggressive culling strategies".to_string(),
            detected_at: Instant::now(),
        });
    }
}
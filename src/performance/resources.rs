use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// FPS counter resource with smoothed average calculation
#[derive(Resource)]
pub struct FpsCounter {
    pub current_fps: f32,
    pub average_fps: f32,
    pub frame_times: VecDeque<f32>,
    pub last_update: Instant,
    pub min_fps: f32,
    pub max_fps: f32,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            current_fps: 0.0,
            average_fps: 0.0,
            frame_times: VecDeque::with_capacity(60), // Store last 60 frames
            last_update: Instant::now(),
            min_fps: f32::MAX,
            max_fps: 0.0,
        }
    }
}

/// Comprehensive performance metrics
#[derive(Resource)]
pub struct PerformanceMetrics {
    // Entity counts
    pub total_entities: usize,
    pub bird_count: usize,
    pub ai_bird_count: usize,
    pub feeder_count: usize,
    pub ui_element_count: usize,
    
    // System performance
    pub update_time_ms: f32,
    pub render_time_ms: f32,
    pub ai_system_time_ms: f32,
    pub physics_time_ms: f32,
    
    // Memory usage estimates
    pub estimated_memory_mb: f32,
    pub component_memory_mb: f32,
    pub resource_memory_mb: f32,
    
    // Performance warnings
    pub warnings: Vec<String>,
    
    // Historical data for trending
    pub fps_history: VecDeque<f32>,
    pub entity_count_history: VecDeque<usize>,
    pub memory_history: VecDeque<f32>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_entities: 0,
            bird_count: 0,
            ai_bird_count: 0,
            feeder_count: 0,
            ui_element_count: 0,
            update_time_ms: 0.0,
            render_time_ms: 0.0,
            ai_system_time_ms: 0.0,
            physics_time_ms: 0.0,
            estimated_memory_mb: 0.0,
            component_memory_mb: 0.0,
            resource_memory_mb: 0.0,
            warnings: Vec::new(),
            fps_history: VecDeque::with_capacity(300), // 5 minutes at 60fps
            entity_count_history: VecDeque::with_capacity(300),
            memory_history: VecDeque::with_capacity(300),
        }
    }
}

/// Detailed profiling data for specific systems
#[derive(Resource)]
pub struct ProfilingData {
    pub system_times: HashMap<String, Duration>,
    pub system_call_counts: HashMap<String, u32>,
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub last_profile_time: Instant,
    pub profiling_enabled: bool,
}

impl Default for ProfilingData {
    fn default() -> Self {
        Self {
            system_times: HashMap::new(),
            system_call_counts: HashMap::new(),
            bottlenecks: Vec::new(),
            last_profile_time: Instant::now(),
            profiling_enabled: true,
        }
    }
}

/// Represents a performance bottleneck or optimization opportunity
#[derive(Debug, Clone)]
pub struct PerformanceBottleneck {
    pub system_name: String,
    pub issue_type: BottleneckType,
    pub severity: BottleneckSeverity,
    pub description: String,
    pub suggested_fix: String,
    pub detected_at: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckType {
    HighCpuUsage,
    HighMemoryUsage,
    LowFrameRate,
    SlowSystem,
    TooManyEntities,
    IneffientQuery,
    MemoryLeak,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance settings and thresholds
#[derive(Resource)]
pub struct PerformanceSettings {
    pub target_fps: f32,
    pub min_acceptable_fps: f32,
    pub max_entities: usize,
    pub memory_warning_threshold_mb: f32,
    pub enable_profiling: bool,
    pub enable_fps_display: bool,
    pub enable_performance_display: bool,
    pub profiling_interval_ms: u64,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            target_fps: 60.0,
            min_acceptable_fps: 30.0,
            max_entities: 500,
            memory_warning_threshold_mb: 256.0,
            enable_profiling: true,
            enable_fps_display: true,
            enable_performance_display: false, // Hidden by default
            profiling_interval_ms: 1000, // Profile every second
        }
    }
}
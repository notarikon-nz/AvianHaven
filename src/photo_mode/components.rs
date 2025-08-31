use bevy::prelude::*;
use crate::bird::BirdSpecies;

#[derive(Component)]
pub struct PhotoTarget;

#[derive(Component)]
pub struct ViewfinderUI;

#[derive(Component)]
pub struct CompositionGrid {
    pub rule_of_thirds: bool,
    pub center_guides: bool,
    pub golden_ratio: bool,
}

#[derive(Component)]
pub struct CameraControls {
    pub zoom_level: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
    pub focus_distance: f32,
    pub aperture: f32,
    pub exposure: f32,
    pub iso: f32,
}

#[derive(Component)]
pub struct DepthOfFieldPreview {
    pub enabled: bool,
    pub focus_plane: f32,
    pub blur_intensity: f32,
}

#[derive(Component)]
pub struct PhotoModeUI;

#[derive(Component)]
pub struct CameraSettingsPanel;

#[derive(Component)]
pub struct ExposureSlider;

#[derive(Component)]
pub struct ApertureSlider;

#[derive(Component)]
pub struct ISOSlider;

#[derive(Component)]
pub struct ScoreToast {
    pub timer: Timer,
}

impl Default for ScoreToast {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}

#[derive(Event)]
pub struct PhotoTakenEvent {
    pub score: PhotoScore,
    pub species: Option<BirdSpecies>,
    pub image_handle: Handle<Image>,
}

#[derive(Debug, Clone)]
pub struct PhotoScore {
    pub species_score: u32,
    pub centering_score: u32,
    pub clarity_score: u32,
    pub behavior_score: u32,
    pub timing_score: u32,
    pub rarity_bonus: u32,
    pub composition_score: u32,    // New: Rule of thirds, framing, etc.
    pub lighting_score: u32,       // New: Quality of lighting conditions
    pub environment_score: u32,    // New: Background elements and setting
    pub technical_score: u32,      // New: Camera settings optimization
    pub storytelling_score: u32,   // New: Narrative elements in the photo
    pub total_score: u32,
}

#[derive(Debug, Clone)]
pub struct CompositionAnalysis {
    pub rule_of_thirds_alignment: f32,  // 0.0-1.0
    pub leading_lines: bool,
    pub framing_elements: u32,           // Trees, branches, etc.
    pub negative_space_balance: f32,     // 0.0-1.0
    pub depth_layers: u32,               // Foreground, midground, background
}

#[derive(Debug, Clone)]
pub struct LightingAnalysis {
    pub golden_hour_bonus: bool,
    pub lighting_direction: Vec3,
    pub shadow_quality: f32,           // 0.0-1.0
    pub color_temperature_harmony: f32, // 0.0-1.0
    pub dynamic_range: f32,            // 0.0-1.0
}
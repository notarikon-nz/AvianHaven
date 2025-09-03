use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShadowQuality {
    Off,
    Low,
    Medium,
    High,
}

#[derive(Resource, Default)]
pub struct MenuState {
    pub current_menu: MenuType,
    pub previous_menu: Option<MenuType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MenuType {
    #[default]
    MainMenu,
    Settings,
    SettingsControls,
    LoadGame,
    InGame,
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameSettings {
    // Audio settings
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    
    // Gameplay settings
    pub auto_save_enabled: bool,
    pub auto_save_interval: f32, // minutes
    
    // Graphics settings
    pub vsync_enabled: bool,
    pub fullscreen: bool,
    pub window_resolution: (u32, u32),
    pub graphics_quality: GraphicsQuality,
    pub particle_density: f32,
    pub shadow_quality: ShadowQuality,
    
    // Controls
    pub camera_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.8,
            auto_save_enabled: true,
            auto_save_interval: 5.0,
            vsync_enabled: true,
            fullscreen: false,
            window_resolution: (1920, 1080),
            graphics_quality: GraphicsQuality::High,
            particle_density: 1.0,
            shadow_quality: ShadowQuality::Medium,
            camera_sensitivity: 1.0,
            zoom_sensitivity: 1.0,
        }
    }
}

impl GameSettings {
    pub fn get_settings_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("AvianHaven")
            .join("settings.ron")
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_path = Self::get_settings_path();
        
        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let serialized = ron::to_string(self)?;
        fs::write(settings_path, serialized)?;
        Ok(())
    }
    
    pub fn load_from_file() -> Self {
        let settings_path = Self::get_settings_path();
        
        if settings_path.exists() {
            if let Ok(content) = fs::read_to_string(settings_path) {
                if let Ok(settings) = ron::from_str::<GameSettings>(&content) {
                    return settings;
                }
            }
        }
        
        // Return default settings if loading fails
        Self::default()
    }
}

#[derive(Event)]
pub struct MenuNavigationEvent {
    pub target_menu: MenuType,
    pub target_app_state: Option<crate::AppState>,
}
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use super::components::TutorialStep;

#[derive(Resource)]
pub struct TutorialState {
    pub is_active: bool,
    pub current_step: TutorialStep,
    pub step_start_time: f32,
    pub is_skipped: bool,
    pub show_ui: bool,
}

impl Default for TutorialState {
    fn default() -> Self {
        Self {
            is_active: false,
            current_step: TutorialStep::Welcome,
            step_start_time: 0.0,
            is_skipped: false,
            show_ui: true,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct TutorialProgress {
    pub completed_steps: Vec<TutorialStep>,
    pub tutorial_completed: bool,
    pub tutorial_skipped: bool,
}

impl TutorialProgress {
    pub fn get_progress_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("AvianHaven")
            .join("tutorial_progress.ron")
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let progress_path = Self::get_progress_path();
        
        if let Some(parent) = progress_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let serialized = ron::to_string(self)?;
        fs::write(progress_path, serialized)?;
        Ok(())
    }
    
    pub fn load_from_file() -> Self {
        let progress_path = Self::get_progress_path();
        
        if progress_path.exists() {
            if let Ok(content) = fs::read_to_string(progress_path) {
                if let Ok(progress) = ron::from_str::<TutorialProgress>(&content) {
                    return progress;
                }
            }
        }
        
        Self::default()
    }
    
    pub fn is_step_completed(&self, step: TutorialStep) -> bool {
        self.completed_steps.contains(&step)
    }
    
    pub fn complete_step(&mut self, step: TutorialStep) {
        if !self.completed_steps.contains(&step) {
            self.completed_steps.push(step);
        }
    }
    
    pub fn should_show_tutorial(&self) -> bool {
        !self.tutorial_completed && !self.tutorial_skipped
    }
}

#[derive(Event)]
pub struct TutorialEvent {
    pub action: TutorialAction,
}

#[derive(Debug, Clone)]
pub enum TutorialAction {
    Start,
    NextStep,
    Skip,
    Complete,
    Show,
    Hide,
}

#[derive(Event)]
pub struct TutorialStepCompleteEvent {
    pub step: TutorialStep,
    pub auto_advance: bool,
}
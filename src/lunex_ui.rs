use bevy::prelude::*;

// Lunex UI Migration Module
// This module provides a gradual migration path from Bevy UI to Lunex UI
// Starting with simple components and expanding to complex layouts

pub struct LunexUiPlugin;

impl Plugin for LunexUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_lunex_system);
    }
}

pub fn setup_lunex_system() {
    info!("Initializing Lunex UI system for gradual migration");
    
    // Foundation setup for Lunex UI migration
    // In future phases, this will initialize Lunex-specific systems
}

// Migration utilities for converting Bevy UI to Lunex UI

#[derive(Component)]
pub struct LunexMigrationMarker;

#[derive(Component)]
pub struct LunexContainer {
    pub width: LunexSize,
    pub height: LunexSize,
    pub alignment: LunexAlignment,
}

#[derive(Debug, Clone)]
pub enum LunexSize {
    Pixels(f32),
    Percentage(f32),
    Auto,
}

#[derive(Debug, Clone)]
pub enum LunexAlignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl LunexContainer {
    pub fn new_pixel(width: f32, height: f32) -> Self {
        Self {
            width: LunexSize::Pixels(width),
            height: LunexSize::Pixels(height),
            alignment: LunexAlignment::Center,
        }
    }
    
    pub fn new_percentage(width: f32, height: f32) -> Self {
        Self {
            width: LunexSize::Percentage(width),
            height: LunexSize::Percentage(height),
            alignment: LunexAlignment::Center,
        }
    }
    
    pub fn with_alignment(mut self, alignment: LunexAlignment) -> Self {
        self.alignment = alignment;
        self
    }
}

// Migration helpers for common UI patterns

pub fn migrate_simple_button(
    commands: &mut Commands,
    text: &str,
    width: f32,
    height: f32,
) -> Entity {
    commands.spawn((
        LunexContainer::new_pixel(width, height),
        LunexMigrationMarker,
        Name::new(format!("Lunex Button: {}", text)),
    )).id()
}

pub fn migrate_text_label(
    commands: &mut Commands,
    text: &str,
    font_size: f32,
) -> Entity {
    commands.spawn((
        LunexContainer::new_auto(),
        LunexMigrationMarker,
        Name::new(format!("Lunex Text: {}", text)),
    )).id()
}

impl LunexContainer {
    pub fn new_auto() -> Self {
        Self {
            width: LunexSize::Auto,
            height: LunexSize::Auto,
            alignment: LunexAlignment::Center,
        }
    }
}

// Migration priority components - which UI elements to migrate first

#[derive(Component)]
pub struct MigrationPriority {
    pub priority: u32, // Lower number = higher priority
    pub complexity: MigrationComplexity,
}

#[derive(Debug, Clone)]
pub enum MigrationComplexity {
    Simple,    // Basic buttons, text labels
    Medium,    // Layout containers, lists
    Complex,   // Interactive widgets, animations
    Advanced,  // Custom drawing, complex state
}

impl MigrationPriority {
    pub fn simple(priority: u32) -> Self {
        Self { priority, complexity: MigrationComplexity::Simple }
    }
    
    pub fn medium(priority: u32) -> Self {
        Self { priority, complexity: MigrationComplexity::Medium }
    }
    
    pub fn complex(priority: u32) -> Self {
        Self { priority, complexity: MigrationComplexity::Complex }
    }
}

// Recommended migration order:
// 1. Simple buttons (Settings menu buttons) - Priority 1
// 2. Text labels and static content - Priority 2  
// 3. Layout containers (menu panels) - Priority 3
// 4. Interactive widgets (sliders, toggles) - Priority 4
// 5. Complex layouts (journal tabs) - Priority 5
// 6. Advanced features (tooltips, animations) - Priority 6
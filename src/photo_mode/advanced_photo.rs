// Advanced Photography Features - Phase 4
use bevy::prelude::*;
use crate::bird::BirdSpecies;

// Lens System
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LensType {
    Standard,     // 50mm equivalent - balanced field of view
    Telephoto,    // 200mm equivalent - wildlife photography
    Macro,        // Close-up detail photography
    WideAngle,    // 24mm equivalent - environmental shots
}

impl LensType {
    pub fn zoom_range(&self) -> (f32, f32) {
        match self {
            Self::Standard => (0.8, 2.0),
            Self::Telephoto => (2.0, 8.0),
            Self::Macro => (4.0, 12.0),
            Self::WideAngle => (0.3, 1.2),
        }
    }
    
    pub fn field_of_view(&self) -> f32 {
        match self {
            Self::Standard => 46.0,
            Self::Telephoto => 12.0,
            Self::Macro => 8.0,
            Self::WideAngle => 84.0,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::Standard => "Standard 50mm",
            Self::Telephoto => "Telephoto 200mm",
            Self::Macro => "Macro 100mm",
            Self::WideAngle => "Wide Angle 24mm",
        }
    }
    
    pub fn unlock_cost(&self) -> u32 {
        match self {
            Self::Standard => 0,     // Default
            Self::Telephoto => 500,  // Great for bird photography
            Self::Macro => 750,      // Detail shots
            Self::WideAngle => 300,  // Environmental context
        }
    }
}

// Filter System
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhotoFilter {
    None,
    Warm,           // Enhance golden hour
    Cool,           // Morning/overcast enhancement
    Vibrant,        // Boost bird colors
    BlackWhite,     // Classic monochrome
    Vintage,        // Nostalgic look
    HighContrast,   // Dramatic shadows
}

impl PhotoFilter {
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "No Filter",
            Self::Warm => "Warm",
            Self::Cool => "Cool",
            Self::Vibrant => "Vibrant",
            Self::BlackWhite => "Black & White",
            Self::Vintage => "Vintage",
            Self::HighContrast => "High Contrast",
        }
    }
    
    pub fn unlock_cost(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Warm | Self::Cool => 100,
            Self::Vibrant => 200,
            Self::BlackWhite => 150,
            Self::Vintage => 250,
            Self::HighContrast => 300,
        }
    }
    
    pub fn apply_color_adjustment(&self, base_color: Color) -> Color {
        let srgba = base_color.to_srgba();
        match self {
            Self::None => base_color,
            Self::Warm => Color::srgba(
                srgba.red * 1.1,
                srgba.green * 1.05,
                srgba.blue * 0.95,
                srgba.alpha
            ),
            Self::Cool => Color::srgba(
                srgba.red * 0.95,
                srgba.green * 1.0,
                srgba.blue * 1.1,
                srgba.alpha
            ),
            Self::Vibrant => Color::srgba(
                srgba.red * 1.2,
                srgba.green * 1.2,
                srgba.blue * 1.2,
                srgba.alpha
            ),
            Self::BlackWhite => {
                let gray = (srgba.red + srgba.green + srgba.blue) / 3.0;
                Color::srgba(gray, gray, gray, srgba.alpha)
            },
            Self::Vintage => Color::srgba(
                srgba.red * 1.05,
                srgba.green * 0.95,
                srgba.blue * 0.85,
                srgba.alpha
            ),
            Self::HighContrast => {
                let enhance = |c: f32| if c > 0.5 { (c * 1.3).min(1.0) } else { c * 0.7 };
                Color::srgba(
                    enhance(srgba.red),
                    enhance(srgba.green),
                    enhance(srgba.blue),
                    srgba.alpha
                )
            }
        }
    }
}

// Advanced Camera Settings
#[derive(Component, Clone)]
pub struct AdvancedCameraSettings {
    pub current_lens: LensType,
    pub available_lenses: Vec<LensType>,
    pub current_filter: PhotoFilter,
    pub available_filters: Vec<PhotoFilter>,
    pub focus_mode: FocusMode,
    pub metering_mode: MeteringMode,
    pub white_balance: WhiteBalance,
    pub exposure_compensation: f32, // -2.0 to +2.0
}

impl Default for AdvancedCameraSettings {
    fn default() -> Self {
        Self {
            current_lens: LensType::Standard,
            available_lenses: vec![LensType::Standard],
            current_filter: PhotoFilter::None,
            available_filters: vec![PhotoFilter::None],
            focus_mode: FocusMode::Single,
            metering_mode: MeteringMode::Center,
            white_balance: WhiteBalance::Auto,
            exposure_compensation: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusMode {
    Single,      // Single point focus
    Continuous,  // Continuous autofocus for moving birds
    Manual,      // Manual focus control
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeteringMode {
    Center,      // Center-weighted metering
    Spot,        // Spot metering on bird
    Matrix,      // Evaluate entire scene
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WhiteBalance {
    Auto,
    Daylight,
    Cloudy,
    Shade,
    Tungsten,
}

// Composition Tools
#[derive(Component)]
pub struct CompositionGuides {
    pub rule_of_thirds: bool,
    pub golden_spiral: bool,
    pub center_guides: bool,
    pub diagonal_guides: bool,
    pub safe_zones: bool, // TV safe areas
}

impl Default for CompositionGuides {
    fn default() -> Self {
        Self {
            rule_of_thirds: true,
            golden_spiral: false,
            center_guides: false,
            diagonal_guides: false,
            safe_zones: false,
        }
    }
}

// Photo Collections
#[derive(Resource, Default)]
pub struct PhotoCollection {
    pub photos: Vec<SavedPhoto>,
    pub albums: Vec<PhotoAlbum>,
    pub favorites: Vec<usize>, // Photo indices
}

#[derive(Debug, Clone)]
pub struct SavedPhoto {
    pub id: u32,
    pub species: Option<BirdSpecies>,
    pub score: PhotoScore,
    pub timestamp: String,
    pub location: Vec3,
    pub camera_settings: PhotoMetadata,
    pub tags: Vec<String>,
    pub image_handle: Handle<Image>,
}

#[derive(Debug, Clone)]
pub struct PhotoAlbum {
    pub name: String,
    pub photos: Vec<u32>, // Photo IDs
    pub created: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PhotoMetadata {
    pub lens_used: LensType,
    pub filter_used: PhotoFilter,
    pub aperture: f32,
    pub shutter_speed: f32,
    pub iso: f32,
    pub focal_length: f32,
    pub white_balance: WhiteBalance,
    pub exposure_compensation: f32,
}

// Enhanced Photo Score for new features
use crate::photo_mode::components::PhotoScore;

impl PhotoScore {
    pub fn calculate_lens_bonus(&self, lens: LensType, species: Option<BirdSpecies>) -> u32 {
        match (lens, species) {
            (LensType::Telephoto, Some(_)) => 20, // Great for bird photography
            (LensType::Macro, Some(species)) if species == BirdSpecies::Goldfinch => 30, // Detail shots
            (LensType::WideAngle, Some(_)) => 15, // Environmental context
            _ => 0,
        }
    }
    
    pub fn calculate_filter_bonus(&self, filter: PhotoFilter) -> u32 {
        match filter {
            PhotoFilter::BlackWhite => 15,     // Artistic choice
            PhotoFilter::Vibrant => 10,        // Bird colors pop
            PhotoFilter::Warm => 12,           // Golden hour enhancement
            _ => 0,
        }
    }
    
    pub fn calculate_advanced_total(&self, lens: LensType, filter: PhotoFilter, species: Option<BirdSpecies>) -> u32 {
        self.total_score + 
        self.calculate_lens_bonus(lens, species) +
        self.calculate_filter_bonus(filter)
    }
}

// Events
#[derive(Event)]
pub struct LensSwitchEvent {
    pub new_lens: LensType,
}

#[derive(Event)]
pub struct FilterChangeEvent {
    pub new_filter: PhotoFilter,
}

#[derive(Event)]
pub struct PhotoSavedEvent {
    pub photo: SavedPhoto,
}
use bevy::prelude::*;

#[derive(Component)]
pub struct RainParticle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub intensity: f32,
    pub depth_layer: u8,
}

#[derive(Component)]
pub struct SnowParticle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub wind_drift: f32,
    pub size: f32,
    pub rotation_speed: f32,
    pub depth_layer: u8,
}

#[derive(Component)]
pub struct EnvironmentalParticle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub particle_type: EnvironmentalParticleType,
    pub fade_rate: f32,
}

#[derive(Clone, Copy)]
pub enum EnvironmentalParticleType {
    FallingLeaf { color: Color, rotation_speed: f32 },
    Pollen { drift_strength: f32 },
    DustMote { float_pattern: f32 },
    FeatherDrift { source_species: crate::bird::BirdSpecies },
}

#[derive(Resource, Default)]
pub struct WeatherEffectsState {
    pub active_weather: Option<WeatherType>,
    pub spawn_timer: f32,
    pub particle_count: usize,
    pub environmental_timer: f32,
    pub wind_strength: f32,
    pub wind_direction: Vec2,
}

#[derive(Component)]
pub struct InteractiveParticle {
    pub particle_type: InteractiveParticleType,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub source_entity: Option<Entity>,
}

#[derive(Clone)]
pub enum InteractiveParticleType {
    SeedScatter { seed_type: String },
    WaterSplash { ripple_strength: f32 },
    BirdSplash { splash_size: f32 },
    FeederCrumbs { food_type: String },
}

#[derive(Clone, Copy, PartialEq)]
pub enum WeatherType {
    Rain,
    Snow,
}
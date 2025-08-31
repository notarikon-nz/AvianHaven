use bevy::prelude::*;

#[derive(Component)]
pub struct RainParticle {
    pub velocity: Vec2,
    pub lifetime: f32,
}

#[derive(Component)]
pub struct SnowParticle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub wind_drift: f32,
}

#[derive(Resource, Default)]
pub struct WeatherEffectsState {
    pub active_weather: Option<WeatherType>,
    pub spawn_timer: f32,
    pub particle_count: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WeatherType {
    Rain,
    Snow,
}
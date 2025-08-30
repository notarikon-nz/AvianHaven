use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub bird_attraction_radius: f32,
    pub bird_wander_speed: f32,
    pub bird_attraction_force: f32,
    pub bird_avoidance_radius: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            bird_attraction_radius: 150.0,
            bird_wander_speed: 50.0,
            bird_attraction_force: 100.0,
            bird_avoidance_radius: 30.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct BirdCount(pub usize);

#[derive(Event)]
pub struct SpawnBirdEvent;
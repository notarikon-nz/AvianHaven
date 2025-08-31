use bevy::prelude::*;
use super::components::*;
use crate::environment::{resources::{WeatherState, WeatherChangeEvent}, components::Weather};
use rand::Rng;

const RAIN_SPAWN_RATE: f32 = 0.01; // particles per frame per intensity
const SNOW_SPAWN_RATE: f32 = 0.005;
const MAX_PARTICLES: usize = 300;

pub fn setup_weather_particles(
    mut weather_state: ResMut<WeatherEffectsState>,
) {
    weather_state.spawn_timer = 0.0;
    weather_state.particle_count = 0;
}

pub fn weather_particle_system(
    weather_state: Res<WeatherState>,
    mut effects_state: ResMut<WeatherEffectsState>,
    mut weather_events: EventReader<WeatherChangeEvent>,
    mut commands: Commands,
    time: Res<Time>,
    rain_query: Query<Entity, With<RainParticle>>,
    snow_query: Query<Entity, With<SnowParticle>>,
) {
    // Handle weather change events
    for event in weather_events.read() {
        let new_weather_type = match event.new_weather {
            Weather::Rainy => Some(WeatherType::Rain),
            Weather::Snowy => Some(WeatherType::Snow),
            _ => None,
        };
        
        // Clean up old particles when weather changes
        if effects_state.active_weather != new_weather_type {
            // Despawn existing particles
            for entity in &rain_query {
                commands.entity(entity).despawn();
            }
            for entity in &snow_query {
                commands.entity(entity).despawn();
            }
            effects_state.particle_count = 0;
        }
        
        effects_state.active_weather = new_weather_type;
    }
    
    // Spawn new particles based on current weather
    effects_state.spawn_timer += time.delta().as_secs_f32();
    
    if let Some(weather_type) = effects_state.active_weather {
        let spawn_rate = match weather_type {
            WeatherType::Rain => RAIN_SPAWN_RATE,
            WeatherType::Snow => SNOW_SPAWN_RATE,
        };
        
        if effects_state.spawn_timer > spawn_rate && effects_state.particle_count < MAX_PARTICLES {
            effects_state.spawn_timer = 0.0;
            spawn_weather_particle(&mut commands, weather_type, &weather_state);
            effects_state.particle_count += 1;
        }
    }
}

fn spawn_weather_particle(
    commands: &mut Commands,
    weather_type: WeatherType,
    weather_state: &WeatherState,
) {
    let mut rng = rand::rng();
    
    // Spawn particles above the screen
    let spawn_x = rng.random_range(-800.0..800.0);
    let spawn_y = 600.0;
    
    match weather_type {
        WeatherType::Rain => {
            let velocity = Vec2::new(
                rng.random_range(-20.0..20.0), // Small horizontal drift
                rng.random_range(-350.0..-250.0), // Fast downward
            );
            
            commands.spawn((
                Sprite::from_color(
                    Color::srgba(0.7, 0.8, 1.0, 0.6),
                    Vec2::new(1.0, 8.0) // Thin raindrop shape
                ),
                Transform::from_xyz(spawn_x, spawn_y, 2.0),
                RainParticle {
                    velocity,
                    lifetime: 4.0,
                },
            ));
        },
        WeatherType::Snow => {
            let velocity = Vec2::new(
                rng.random_range(-30.0..30.0), // More horizontal drift
                rng.random_range(-80.0..-30.0), // Slower fall
            );
            
            commands.spawn((
                Sprite::from_color(
                    Color::srgba(1.0, 1.0, 1.0, 0.8),
                    Vec2::new(3.0, 3.0) // Small snowflake
                ),
                Transform::from_xyz(spawn_x, spawn_y, 2.0),
                SnowParticle {
                    velocity,
                    lifetime: 10.0,
                    wind_drift: rng.random_range(-0.5..0.5),
                },
            ));
        }
    }
}

pub fn rain_particle_movement(
    mut rain_query: Query<(&mut Transform, &mut RainParticle)>,
    time: Res<Time>,
) {
    for (mut transform, mut particle) in &mut rain_query {
        // Move particle
        transform.translation.x += particle.velocity.x * time.delta().as_secs_f32();
        transform.translation.y += particle.velocity.y * time.delta().as_secs_f32();
        
        // Update lifetime
        particle.lifetime -= time.delta().as_secs_f32();
    }
}

pub fn snow_particle_movement(
    mut snow_query: Query<(&mut Transform, &mut SnowParticle)>,
    time: Res<Time>,
) {
    for (mut transform, mut particle) in &mut snow_query {
        // Add wind drift effect
        particle.velocity.x += particle.wind_drift * time.delta().as_secs_f32() * 10.0;
        
        // Move particle
        transform.translation.x += particle.velocity.x * time.delta().as_secs_f32();
        transform.translation.y += particle.velocity.y * time.delta().as_secs_f32();
        
        // Update lifetime
        particle.lifetime -= time.delta().as_secs_f32();
    }
}

pub fn particle_cleanup_system(
    mut commands: Commands,
    mut effects_state: ResMut<WeatherEffectsState>,
    rain_query: Query<(Entity, &Transform, &RainParticle)>,
    snow_query: Query<(Entity, &Transform, &SnowParticle)>,
) {
    // Clean up expired or off-screen rain particles
    for (entity, transform, particle) in &rain_query {
        if particle.lifetime <= 0.0 || transform.translation.y < -600.0 {
            commands.entity(entity).despawn();
            effects_state.particle_count = effects_state.particle_count.saturating_sub(1);
        }
    }
    
    // Clean up expired or off-screen snow particles
    for (entity, transform, particle) in &snow_query {
        if particle.lifetime <= 0.0 || transform.translation.y < -600.0 {
            commands.entity(entity).despawn();
            effects_state.particle_count = effects_state.particle_count.saturating_sub(1);
        }
    }
}
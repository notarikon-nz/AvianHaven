use bevy::prelude::*;
use super::components::*;
use crate::environment::{resources::{WeatherState, WeatherChangeEvent, TimeState}, components::{Weather, Season}};
use crate::bird_ai::components::{BirdAI, BirdState};
use crate::animation::components::AnimatedBird;
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
                    lifetime: rng.random_range(3.0..5.0),
                    intensity: rng.random_range(0.4..1.0),
                    depth_layer: rng.random_range(0..3),
                },
            ));
        },
        WeatherType::Snow => {
            let velocity = Vec2::new(
                rng.random_range(-30.0..30.0), // More horizontal drift
                rng.random_range(-80.0..-30.0), // Slower fall
            );
            let size = rng.random_range(2.0..4.0);
            
            commands.spawn((
                Sprite::from_color(
                    Color::srgba(1.0, 1.0, 1.0, 0.8),
                    Vec2::new(size, size) // Variable snowflake size
                ),
                Transform::from_xyz(spawn_x, spawn_y, 2.0),
                SnowParticle {
                    velocity,
                    lifetime: rng.random_range(8.0..12.0),
                    wind_drift: rng.random_range(-0.5..0.5),
                    size,
                    rotation_speed: rng.random_range(-1.0..1.0),
                    depth_layer: rng.random_range(0..3),
                },
            ));
        }
    }
}

pub fn environmental_particle_system(
    mut commands: Commands,
    mut effects_state: ResMut<WeatherEffectsState>,
    time_state: Res<TimeState>,
    time: Res<Time>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    effects_state.environmental_timer += time.delta().as_secs_f32();
    
    // Spawn environmental particles based on season
    if effects_state.environmental_timer > 2.0 { // Every 2 seconds
        effects_state.environmental_timer = 0.0;
        
        let Ok(camera_transform) = camera_query.single() else {
            return;
        };
        let camera_pos = camera_transform.translation.truncate();
        
        let season = time_state.get_season();
        spawn_seasonal_particles(&mut commands, season, camera_pos, &time_state);
    }
}

fn spawn_seasonal_particles(commands: &mut Commands, season: Season, camera_pos: Vec2, time_state: &TimeState) {
    let mut rng = rand::rng();
    
    match season {
        Season::Fall => {
            // Falling leaves
            for _ in 0..rng.random_range(1..4) {
                let spawn_pos = camera_pos + Vec2::new(
                    rng.random_range(-400.0..400.0),
                    rng.random_range(200.0..300.0)
                );
                
                let leaf_colors = [
                    Color::srgb(0.8, 0.4, 0.1), // Orange
                    Color::srgb(0.7, 0.2, 0.1), // Red
                    Color::srgb(0.9, 0.7, 0.2), // Yellow
                    Color::srgb(0.6, 0.3, 0.1), // Brown
                ];
                let color = leaf_colors[rng.random_range(0..leaf_colors.len())];
                
                commands.spawn((
                    Sprite::from_color(color, Vec2::new(4.0, 6.0)),
                    Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.5),
                    EnvironmentalParticle {
                        velocity: Vec2::new(
                            rng.random_range(-20.0..20.0),
                            rng.random_range(-40.0..-10.0)
                        ),
                        lifetime: rng.random_range(8.0..15.0),
                        particle_type: EnvironmentalParticleType::FallingLeaf {
                            color,
                            rotation_speed: rng.random_range(-2.0..2.0),
                        },
                        fade_rate: 0.1,
                    },
                ));
            }
        },
        Season::Spring => {
            // Pollen particles
            if rng.random::<f32>() < 0.3 {
                let spawn_pos = camera_pos + Vec2::new(
                    rng.random_range(-300.0..300.0),
                    rng.random_range(-100.0..200.0)
                );
                
                commands.spawn((
                    Sprite::from_color(Color::srgba(1.0, 1.0, 0.6, 0.6), Vec2::new(1.5, 1.5)),
                    Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.0),
                    EnvironmentalParticle {
                        velocity: Vec2::new(
                            rng.random_range(-10.0..10.0),
                            rng.random_range(-5.0..5.0)
                        ),
                        lifetime: rng.random_range(12.0..20.0),
                        particle_type: EnvironmentalParticleType::Pollen {
                            drift_strength: rng.random_range(0.5..1.5),
                        },
                        fade_rate: 0.05,
                    },
                ));
            }
        },
        Season::Summer => {
            // Dust motes in sunbeams
            if time_state.hour >= 10.0 && time_state.hour <= 16.0 && rng.random::<f32>() < 0.2 {
                let spawn_pos = camera_pos + Vec2::new(
                    rng.random_range(-200.0..200.0),
                    rng.random_range(-150.0..150.0)
                );
                
                commands.spawn((
                    Sprite::from_color(Color::srgba(1.0, 0.9, 0.7, 0.3), Vec2::new(1.0, 1.0)),
                    Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.8),
                    EnvironmentalParticle {
                        velocity: Vec2::new(
                            rng.random_range(-2.0..2.0),
                            rng.random_range(-1.0..1.0)
                        ),
                        lifetime: rng.random_range(15.0..25.0),
                        particle_type: EnvironmentalParticleType::DustMote {
                            float_pattern: rng.random_range(0.5..2.0),
                        },
                        fade_rate: 0.02,
                    },
                ));
            }
        },
        Season::Winter => {
            // Ice crystals in cold air (minimal particles)
        },
    }
}

pub fn interactive_particle_system(
    mut commands: Commands,
    bird_query: Query<(&Transform, &BirdState, &AnimatedBird), (With<BirdAI>, Changed<BirdState>)>,
    _feeder_query: Query<&Transform, With<crate::feeder::Feeder>>,
    bird_data: Res<crate::bird_data::BirdDataRegistry>,
) {
    // Spawn particles based on bird interactions
    for (bird_transform, bird_state, animated_bird) in &bird_query {
        let bird_pos = bird_transform.translation;
        
        match bird_state {
            BirdState::Eating => {
                // Seed scatter particles when birds feed
                if rand::rng().random::<f32>() < 0.2 {
                    spawn_seed_scatter(&mut commands, bird_pos.truncate(), animated_bird.species, &bird_data);
                }
            },
            BirdState::Drinking | BirdState::Bathing => {
                // Water splash effects
                if rand::rng().random::<f32>() < 0.15 {
                    spawn_water_splash(&mut commands, bird_pos.truncate(), *bird_state == BirdState::Bathing);
                }
            },
            _ => {}
        }
    }
}

fn spawn_seed_scatter(commands: &mut Commands, position: Vec2, species: crate::bird::BirdSpecies, bird_data: &crate::bird_data::BirdDataRegistry) {
    let mut rng = rand::rng();
    
    // Different species create different amounts of scatter
    let size_category = bird_data.get_size_category(&species);
    let scatter_count = match size_category {
        1..=2 => 2, // Small birds, minimal scatter
        3..=4 => 4, // Medium birds
        5..=8 => 6, // Large birds, more messy eaters
        _ => 3,
    };
    
    for _ in 0..scatter_count {
        let scatter_offset = Vec2::new(
            rng.random_range(-15.0..15.0),
            rng.random_range(-10.0..5.0)
        );
        
        commands.spawn((
            Sprite::from_color(Color::srgb(0.7, 0.5, 0.2), Vec2::new(1.5, 1.5)),
            Transform::from_xyz(position.x + scatter_offset.x, position.y + scatter_offset.y, 0.5),
            InteractiveParticle {
                particle_type: InteractiveParticleType::SeedScatter {
                    seed_type: "mixed".to_string(),
                },
                velocity: Vec2::new(scatter_offset.x * 0.1, rng.random_range(-5.0..0.0)),
                lifetime: rng.random_range(5.0..10.0),
                source_entity: None,
            },
        ));
    }
}

fn spawn_water_splash(commands: &mut Commands, position: Vec2, is_bathing: bool) {
    let mut rng = rand::rng();
    
    let splash_size = if is_bathing { 8.0 } else { 6.0 }; // Ensure minimum size > 5.0
    let particle_count = if is_bathing { 8 } else { 4 };
    
    for _ in 0..particle_count {
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let distance = rng.random_range(2.0..splash_size); // Start from 2.0 instead of 5.0
        let splash_pos = position + Vec2::new(
            angle.cos() * distance,
            angle.sin() * distance
        );
        
        commands.spawn((
            Sprite::from_color(Color::srgba(0.6, 0.8, 1.0, 0.7), Vec2::new(2.0, 2.0)),
            Transform::from_xyz(splash_pos.x, splash_pos.y, 1.2),
            InteractiveParticle {
                particle_type: InteractiveParticleType::WaterSplash {
                    ripple_strength: if is_bathing { 1.0 } else { 0.5 },
                },
                velocity: Vec2::new(
                    angle.cos() * rng.random_range(10.0..30.0),
                    angle.sin() * rng.random_range(10.0..30.0)
                ),
                lifetime: rng.random_range(1.0..3.0),
                source_entity: None,
            },
        ));
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
    mut snow_query: Query<(&mut Transform, &mut SnowParticle, &mut Sprite)>,
    time: Res<Time>,
    effects_state: Res<WeatherEffectsState>,
) {
    for (mut transform, mut particle, mut sprite) in &mut snow_query {
        // Add wind drift effect with global wind
        let wind_effect = effects_state.wind_direction * effects_state.wind_strength * 20.0;
        particle.velocity.x += (particle.wind_drift + wind_effect.x) * time.delta().as_secs_f32();
        
        // Move particle
        transform.translation.x += particle.velocity.x * time.delta().as_secs_f32();
        transform.translation.y += particle.velocity.y * time.delta().as_secs_f32();
        
        // Rotate snowflake
        transform.rotation *= Quat::from_rotation_z(particle.rotation_speed * time.delta().as_secs_f32());
        
        // Update size and alpha based on depth layer
        let depth_alpha = match particle.depth_layer {
            0 => 0.9, // Foreground - bright
            1 => 0.6, // Middle - medium
            _ => 0.3, // Background - faded
        };
        sprite.color.set_alpha(depth_alpha);
        
        // Update lifetime
        particle.lifetime -= time.delta().as_secs_f32();
    }
}

pub fn environmental_particle_movement(
    mut env_query: Query<(&mut Transform, &mut EnvironmentalParticle, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut transform, mut particle, mut sprite) in &mut env_query {
        match particle.particle_type {
            EnvironmentalParticleType::FallingLeaf { rotation_speed, .. } => {
                // Swaying leaf motion
                let sway = (time.elapsed_secs() * 2.0).sin() * 10.0;
                particle.velocity.x = sway;
                transform.rotation *= Quat::from_rotation_z(rotation_speed * time.delta().as_secs_f32());
            },
            EnvironmentalParticleType::Pollen { drift_strength } => {
                // Gentle floating motion
                let drift = (time.elapsed_secs() * drift_strength).sin() * 5.0;
                particle.velocity.x += drift * time.delta().as_secs_f32();
            },
            EnvironmentalParticleType::DustMote { float_pattern } => {
                // Floating dust motion in sunbeams
                let float_x = (time.elapsed_secs() * float_pattern).sin() * 3.0;
                let float_y = (time.elapsed_secs() * float_pattern * 0.7).cos() * 2.0;
                particle.velocity = Vec2::new(float_x, float_y);
            },
            EnvironmentalParticleType::FeatherDrift { .. } => {
                // Gentle floating motion for feathers
                particle.velocity.y = -5.0 + (time.elapsed_secs() * 2.0).sin() * 3.0;
            },
        }
        
        // Move particle
        transform.translation.x += particle.velocity.x * time.delta().as_secs_f32();
        transform.translation.y += particle.velocity.y * time.delta().as_secs_f32();
        
        // Fade particle over time
        particle.lifetime -= time.delta().as_secs_f32();
        let alpha = (particle.lifetime * particle.fade_rate).min(1.0);
        sprite.color.set_alpha(alpha);
    }
}

pub fn interactive_particle_movement(
    mut particle_query: Query<(&mut Transform, &mut InteractiveParticle, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut transform, mut particle, mut sprite) in &mut particle_query {
        // Apply gravity to most interactive particles
        match particle.particle_type {
            InteractiveParticleType::SeedScatter { .. } => {
                particle.velocity.y -= 98.0 * time.delta().as_secs_f32(); // Gravity
            },
            InteractiveParticleType::WaterSplash { .. } => {
                particle.velocity.y -= 150.0 * time.delta().as_secs_f32(); // Faster gravity for water
                particle.velocity *= 0.95; // Air resistance
            },
            _ => {}
        }
        
        // Move particle
        transform.translation.x += particle.velocity.x * time.delta().as_secs_f32();
        transform.translation.y += particle.velocity.y * time.delta().as_secs_f32();
        
        // Fade out over lifetime
        particle.lifetime -= time.delta().as_secs_f32();
        let alpha = (particle.lifetime / 3.0).min(1.0);
        sprite.color.set_alpha(alpha);
    }
}

pub fn particle_cleanup_system(
    mut commands: Commands,
    mut effects_state: ResMut<WeatherEffectsState>,
    rain_query: Query<(Entity, &Transform, &RainParticle)>,
    snow_query: Query<(Entity, &Transform, &SnowParticle)>,
    env_query: Query<(Entity, &Transform, &EnvironmentalParticle)>,
    interactive_query: Query<(Entity, &Transform, &InteractiveParticle)>,
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
    
    // Clean up environmental particles
    for (entity, transform, particle) in &env_query {
        if particle.lifetime <= 0.0 || 
           transform.translation.y < -600.0 || 
           transform.translation.distance(Vec3::ZERO) > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
    
    // Clean up interactive particles
    for (entity, transform, particle) in &interactive_query {
        if particle.lifetime <= 0.0 || transform.translation.y < -600.0 {
            commands.entity(entity).despawn();
        }
    }
}
// Bevy Hanabi GPU-Accelerated Particle Effects - Phase 4
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::environment::components::{Weather, Season};
use crate::environment::resources::{WeatherState, TimeState};
use crate::despawn::SafeDespawn;

pub struct HanabiEffectsPlugin;

impl Plugin for HanabiEffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(HanabiPlugin)
            .init_resource::<ParticleEffects>()
            .add_event::<SpawnParticleEvent>()
            .add_systems(Startup, setup_particle_effects)
            .add_systems(Update, (
                update_weather_particles,
                handle_particle_events,
                update_seasonal_particles,
                cleanup_interactive_particles,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct ParticleEffects {
    pub rain_effect: Handle<EffectAsset>,
    pub snow_effect: Handle<EffectAsset>,
    pub wind_effect: Handle<EffectAsset>,
    pub leaves_effect: Handle<EffectAsset>,
    pub pollen_effect: Handle<EffectAsset>,
    pub splash_effect: Handle<EffectAsset>,
    pub seed_effect: Handle<EffectAsset>,
    pub dust_effect: Handle<EffectAsset>,
    pub active_weather_particles: Vec<Entity>,
    pub active_seasonal_particles: Vec<Entity>,
}

#[derive(Event)]
pub struct SpawnParticleEvent {
    pub effect_type: ParticleEffectType,
    pub position: Vec3,
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum ParticleEffectType {
    // Weather
    Rain,
    Snow,
    Wind,
    // Interactive
    WaterSplash,
    SeedScatter,
    Dust,
    // Seasonal
    Pollen,
    FallingLeaves,
}

fn setup_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<ParticleEffects>,
) {
    info!("Initializing GPU particle effects with Hanabi 0.16");
    
    // Rain Effect
    let rain_writer = ExprWriter::new();
    
    let rain_init_pos = SetPositionSphereModifier {
        center: rain_writer.lit(Vec3::new(0.0, 15.0, 0.0)).expr(),
        radius: rain_writer.lit(20.0).expr(),
        dimension: ShapeDimension::Volume,
    };
    
    let rain_init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        rain_writer.lit(Vec3::new(0.0, -8.0, 0.0)).expr(),
    );
    
    let rain_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        rain_writer.lit(3.0).expr(),
    );
    
    let mut rain_color_gradient = Gradient::new();
    rain_color_gradient.add_key(0.0, Vec4::new(0.6, 0.7, 1.0, 0.8));
    rain_color_gradient.add_key(1.0, Vec4::new(0.4, 0.5, 0.9, 0.0));
    
    let mut rain_size_gradient = Gradient::new();
    rain_size_gradient.add_key(0.0, Vec3::new(0.1, 0.1, 1.0));
    rain_size_gradient.add_key(1.0, Vec3::new(0.05, 0.05, 1.0));
    
    let rain_effect = EffectAsset::new(
        2000, // Max particles
        SpawnerSettings::rate(100.0.into()), // 100 particles per second
        rain_writer.finish(),
    )
    .with_name("Rain")
    .init(rain_init_pos)
    .init(rain_init_vel)
    .init(rain_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient: rain_color_gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    })
    .render(SizeOverLifetimeModifier {
        gradient: rain_size_gradient,
        screen_space_size: false,
    });
    particle_effects.rain_effect = effects.add(rain_effect);
    
    // Snow Effect  
    let snow_writer = ExprWriter::new();
    
    let snow_init_pos = SetPositionSphereModifier {
        center: snow_writer.lit(Vec3::new(0.0, 12.0, 0.0)).expr(),
        radius: snow_writer.lit(15.0).expr(),
        dimension: ShapeDimension::Volume,
    };
    
    let snow_init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        snow_writer.lit(Vec3::new(0.0, -2.0, 0.0)).expr(),
    );
    
    let snow_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        snow_writer.lit(6.0).expr(),
    );
    
    let snow_accel = AccelModifier::new(
        snow_writer.lit(Vec3::new(0.0, -0.5, 0.0)).expr(),
    );
    
    let mut snow_color_gradient = Gradient::new();
    snow_color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.9));
    snow_color_gradient.add_key(1.0, Vec4::new(0.9, 0.9, 1.0, 0.0));
    
    let mut snow_size_gradient = Gradient::new();
    snow_size_gradient.add_key(0.0, Vec3::new(0.2, 0.2, 1.0));
    snow_size_gradient.add_key(0.5, Vec3::new(0.3, 0.3, 1.0));
    snow_size_gradient.add_key(1.0, Vec3::new(0.1, 0.1, 1.0));
    
    let snow_effect = EffectAsset::new(
        1000,
        SpawnerSettings::rate(50.0.into()),
        snow_writer.finish(),
    )
    .with_name("Snow")
    .init(snow_init_pos)
    .init(snow_init_vel)
    .init(snow_lifetime)
    .update(snow_accel)
    .render(ColorOverLifetimeModifier {
        gradient: snow_color_gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    })
    .render(SizeOverLifetimeModifier {
        gradient: snow_size_gradient,
        screen_space_size: false,
    });
    particle_effects.snow_effect = effects.add(snow_effect);
    
    // Water Splash Effect
    let splash_writer = ExprWriter::new();
    
    let splash_init_pos = SetPositionSphereModifier {
        center: splash_writer.lit(Vec3::ZERO).expr(),
        radius: splash_writer.lit(0.5).expr(),
        dimension: ShapeDimension::Surface,
    };
    
    let splash_init_vel = SetVelocitySphereModifier {
        center: splash_writer.lit(Vec3::ZERO).expr(),
        speed: splash_writer.lit(5.0).expr(),
    };
    
    let splash_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        splash_writer.lit(1.5).expr(),
    );
    
    let splash_accel = AccelModifier::new(
        splash_writer.lit(Vec3::new(0.0, -9.8, 0.0)).expr(),
    );
    
    let mut splash_color_gradient = Gradient::new();
    splash_color_gradient.add_key(0.0, Vec4::new(0.3, 0.6, 1.0, 0.8));
    splash_color_gradient.add_key(1.0, Vec4::new(0.2, 0.4, 0.8, 0.0));
    
    let splash_effect = EffectAsset::new(
        500,
        SpawnerSettings::once(30.0.into()),
        splash_writer.finish(),
    )
    .with_name("WaterSplash")
    .init(splash_init_pos)
    .init(splash_init_vel)
    .init(splash_lifetime)
    .update(splash_accel)
    .render(ColorOverLifetimeModifier {
        gradient: splash_color_gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    });
    particle_effects.splash_effect = effects.add(splash_effect);
    
    // Falling Leaves Effect
    let leaves_writer = ExprWriter::new();
    
    let leaves_init_pos = SetPositionSphereModifier {
        center: leaves_writer.lit(Vec3::new(0.0, 10.0, 0.0)).expr(),
        radius: leaves_writer.lit(12.0).expr(),
        dimension: ShapeDimension::Volume,
    };
    
    let leaves_init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        leaves_writer.lit(Vec3::new(0.0, -1.5, 0.0)).expr(),
    );
    
    let leaves_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        leaves_writer.lit(8.0).expr(),
    );
    
    let mut leaves_color_gradient = Gradient::new();
    leaves_color_gradient.add_key(0.0, Vec4::new(0.8, 0.4, 0.1, 0.9));
    leaves_color_gradient.add_key(0.5, Vec4::new(0.9, 0.6, 0.2, 0.8));
    leaves_color_gradient.add_key(1.0, Vec4::new(0.6, 0.3, 0.1, 0.0));
    
    let mut leaves_size_gradient = Gradient::new();
    leaves_size_gradient.add_key(0.0, Vec3::new(0.4, 0.4, 1.0));
    leaves_size_gradient.add_key(1.0, Vec3::new(0.6, 0.6, 1.0));
    
    let leaves_effect = EffectAsset::new(
        800,
        SpawnerSettings::rate(20.0.into()),
        leaves_writer.finish(),
    )
    .with_name("FallingLeaves")
    .init(leaves_init_pos)
    .init(leaves_init_vel)
    .init(leaves_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient: leaves_color_gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    })
    .render(SizeOverLifetimeModifier {
        gradient: leaves_size_gradient,
        screen_space_size: false,
    });
    particle_effects.leaves_effect = effects.add(leaves_effect);
    
    // Seed Scatter Effect
    let seed_writer = ExprWriter::new();
    
    let seed_init_pos = SetPositionSphereModifier {
        center: seed_writer.lit(Vec3::ZERO).expr(),
        radius: seed_writer.lit(0.3).expr(),
        dimension: ShapeDimension::Volume,
    };
    
    let seed_init_vel = SetVelocitySphereModifier {
        center: seed_writer.lit(Vec3::ZERO).expr(),
        speed: seed_writer.lit(3.0).expr(),
    };
    
    let seed_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        seed_writer.lit(2.0).expr(),
    );
    
    let seed_accel = AccelModifier::new(
        seed_writer.lit(Vec3::new(0.0, -5.0, 0.0)).expr(),
    );
    
    let mut seed_color_gradient = Gradient::new();
    seed_color_gradient.add_key(0.0, Vec4::new(0.7, 0.5, 0.3, 1.0));
    seed_color_gradient.add_key(1.0, Vec4::new(0.5, 0.3, 0.2, 0.0));
    
    let seed_effect = EffectAsset::new(
        300,
        SpawnerSettings::once(15.0.into()),
        seed_writer.finish(),
    )
    .with_name("SeedScatter")
    .init(seed_init_pos)
    .init(seed_init_vel)
    .init(seed_lifetime)
    .update(seed_accel)
    .render(ColorOverLifetimeModifier {
        gradient: seed_color_gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    });
    particle_effects.seed_effect = effects.add(seed_effect);
    
    // Wind Effect (dust particles moving with wind)
    let wind_writer = ExprWriter::new();
    
    let wind_init_pos = SetPositionSphereModifier {
        center: wind_writer.lit(Vec3::new(0.0, 2.0, 0.0)).expr(),
        radius: wind_writer.lit(8.0).expr(),
        dimension: ShapeDimension::Volume,
    };
    
    let wind_init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        wind_writer.lit(Vec3::new(3.0, 0.0, 0.0)).expr(),
    );
    
    let wind_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        wind_writer.lit(4.0).expr(),
    );
    
    let mut wind_color_gradient = Gradient::new();
    wind_color_gradient.add_key(0.0, Vec4::new(0.8, 0.7, 0.5, 0.4));
    wind_color_gradient.add_key(1.0, Vec4::new(0.6, 0.5, 0.3, 0.0));
    
    let wind_effect = EffectAsset::new(
        600,
        SpawnerSettings::rate(30.0.into()),
        wind_writer.finish(),
    )
    .with_name("Wind")
    .init(wind_init_pos)
    .init(wind_init_vel)
    .init(wind_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient: wind_color_gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    });
    particle_effects.wind_effect = effects.add(wind_effect);
    
    // Pollen Effect
    let pollen_writer = ExprWriter::new();
    
    let pollen_init_pos = SetPositionSphereModifier {
        center: pollen_writer.lit(Vec3::new(0.0, 8.0, 0.0)).expr(),
        radius: pollen_writer.lit(10.0).expr(),
        dimension: ShapeDimension::Volume,
    };
    
    let pollen_init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        pollen_writer.lit(Vec3::new(1.0, -0.5, 0.5)).expr(),
    );
    
    let pollen_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        pollen_writer.lit(10.0).expr(),
    );
    
    let mut pollen_color_gradient = Gradient::new();
    pollen_color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.3, 0.6));
    pollen_color_gradient.add_key(1.0, Vec4::new(0.8, 0.8, 0.1, 0.0));
    
    let pollen_effect = EffectAsset::new(
        400,
        SpawnerSettings::rate(15.0.into()),
        pollen_writer.finish(),
    )
    .with_name("Pollen")
    .init(pollen_init_pos)
    .init(pollen_init_vel)
    .init(pollen_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient: pollen_color_gradient,
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    });
    particle_effects.pollen_effect = effects.add(pollen_effect);
    
    // Initialize dust effect as clone for now
    particle_effects.dust_effect = particle_effects.seed_effect.clone();
    particle_effects.active_weather_particles = Vec::new();
    particle_effects.active_seasonal_particles = Vec::new();
    
    info!("GPU particle effects initialized successfully");
}

fn update_weather_particles(
    mut commands: Commands,
    weather_state: Res<WeatherState>,
    particle_effects: Res<ParticleEffects>,
    mut query: Query<Entity, With<ParticleEffect>>,
) {
    // Remove existing weather particles
    for entity in query.iter() {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.safe_despawn();
        }
    }
    
    // Spawn new weather particles based on current weather
    match weather_state.current_weather {
        Weather::Rainy => {
            commands.spawn((
                Name::new("Rain Particles"),
                ParticleEffect::new(particle_effects.rain_effect.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ));
        },
        Weather::Snowy => {
            commands.spawn((
                Name::new("Snow Particles"),
                ParticleEffect::new(particle_effects.snow_effect.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ));
        },
        Weather::Windy => {
            commands.spawn((
                Name::new("Wind Particles"),
                ParticleEffect::new(particle_effects.wind_effect.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ));
        },
        _ => {}
    }
}

fn update_seasonal_particles(
    mut commands: Commands,
    time_state: Res<TimeState>,
    particle_effects: Res<ParticleEffects>,
    seasonal_query: Query<Entity, (With<ParticleEffect>, With<SeasonalParticle>)>,
) {
    // Remove existing seasonal particles
    for entity in seasonal_query.iter() {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.safe_despawn();
        }
    }
    
    // Spawn seasonal particles
    let current_season = time_state.get_season();
    match current_season {
        Season::Fall => {
            commands.spawn((
                Name::new("Falling Leaves"),
                ParticleEffect::new(particle_effects.leaves_effect.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                SeasonalParticle,
            ));
        },
        Season::Spring => {
            commands.spawn((
                Name::new("Pollen Particles"),
                ParticleEffect::new(particle_effects.pollen_effect.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                SeasonalParticle,
            ));
        },
        _ => {}
    }
}

fn handle_particle_events(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnParticleEvent>,
    particle_effects: Res<ParticleEffects>,
) {
    for event in spawn_events.read() {
        let effect_handle = match event.effect_type {
            ParticleEffectType::WaterSplash => particle_effects.splash_effect.clone(),
            ParticleEffectType::SeedScatter => particle_effects.seed_effect.clone(),
            ParticleEffectType::Dust => particle_effects.dust_effect.clone(),
            _ => continue,
        };
        
        commands.spawn((
            Name::new(format!("Interactive {:?} Particles", event.effect_type)),
            ParticleEffect::new(effect_handle),
            Transform::from_translation(event.position),
            InteractiveParticle {
                lifetime: Timer::from_seconds(3.0, TimerMode::Once),
            },
        ));
        
        info!("Spawned {:?} particle effect at {:?} with intensity {}", 
              event.effect_type, event.position, event.intensity);
    }
}

// Cleanup system for temporary particle effects
fn cleanup_interactive_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut interactive_query: Query<(Entity, &mut InteractiveParticle)>,
) {
    for (entity, mut interactive) in interactive_query.iter_mut() {
        interactive.lifetime.tick(time.delta());
        if interactive.lifetime.finished() {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.safe_despawn();
            }
        }
    }
}

// Marker components
#[derive(Component)]
struct SeasonalParticle;

#[derive(Component)]
struct InteractiveParticle {
    lifetime: Timer,
}
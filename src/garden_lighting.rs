// Garden Lighting System - Solar Lights and Garden Lamps
use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::environment::resources::TimeState;
use crate::bird_ai::components::{SmartObject, ProvidesUtility, BirdAction};
use std::collections::HashMap;

pub struct GardenLightingPlugin;

impl Plugin for GardenLightingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Light2dPlugin)
            .init_resource::<LightingManager>()
            .init_resource::<InsectAttractor>()
            .add_event::<MothSpawnEvent>()
            .add_systems(Startup, setup_lighting_system)
            .add_systems(Update, (
                solar_light_management_system,
                garden_lamp_management_system,
                moth_attraction_system,
                insect_spawning_system,
                light_optimization_system,
                debug_lighting_info,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

// Components for lighting objects
#[derive(Component)]
pub struct SolarLight {
    pub charge_level: f32,        // 0.0-1.0, charged by day
    pub max_runtime_hours: f32,   // 8 hours when fully charged
    pub current_runtime: f32,     // Hours of operation remaining
    pub charging_rate: f32,       // Per hour during daylight
    pub is_active: bool,
    pub light_intensity: f32,     // Base intensity when at full charge
    pub light_radius: f32,        // Light radius in pixels
}

#[derive(Component)]
pub struct GardenLamp {
    pub is_active: bool,
    pub light_intensity: f32,     // Base intensity
    pub light_radius: f32,        // Light radius in pixels
    pub power_consumption: f32,   // Not used yet but for future features
}

#[derive(Component)]
pub struct LightSource {
    pub current_intensity: f32,
    pub target_intensity: f32,
    pub light_entity: Option<Entity>, // Reference to the actual Light2d entity
    pub insect_attraction_radius: f32,
    pub attracted_insects: Vec<Entity>,
}

#[derive(Component)]
pub struct NocturnalInsect {
    pub insect_type: InsectType,
    pub attracted_to_light: bool,
    pub attraction_strength: f32,
    pub movement_speed: f32,
    pub lifespan: Timer,
    pub target_light: Option<Entity>,
}

#[derive(Clone, Copy, Debug)]
pub enum InsectType {
    Moth,
    Gnat,
    Beetle,
}

impl InsectType {
    pub fn attraction_strength(&self) -> f32 {
        match self {
            InsectType::Moth => 0.9,      // Very attracted
            InsectType::Gnat => 0.7,      // Moderately attracted  
            InsectType::Beetle => 0.3,    // Less attracted
        }
    }
    
    pub fn movement_speed(&self) -> f32 {
        match self {
            InsectType::Moth => 50.0,
            InsectType::Gnat => 80.0,
            InsectType::Beetle => 30.0,
        }
    }
    
    pub fn lifespan_minutes(&self) -> f32 {
        match self {
            InsectType::Moth => 30.0,     // 30 minutes
            InsectType::Gnat => 15.0,     // 15 minutes
            InsectType::Beetle => 45.0,   // 45 minutes
        }
    }
}

#[derive(Resource, Default)]
pub struct LightingManager {
    pub ambient_light_level: f32,     // Global ambient light (0.0 = full dark, 1.0 = full day)
    pub active_lights: Vec<Entity>,
    pub total_light_power: f32,       // For performance monitoring
}

#[derive(Resource)]
pub struct InsectAttractor {
    pub spawn_timer: Timer,
    pub max_insects_per_light: u32,
    pub spawn_radius: f32,
    pub active_insects: HashMap<Entity, Vec<Entity>>, // light_entity -> insects
}

#[derive(Event)]
pub struct MothSpawnEvent {
    pub light_entity: Entity,
    pub spawn_position: Vec3,
    pub insect_type: InsectType,
}

impl Default for SolarLight {
    fn default() -> Self {
        Self {
            charge_level: 1.0,
            max_runtime_hours: 8.0,
            current_runtime: 8.0,
            charging_rate: 2.0, // Charges to full in ~6 hours of daylight
            is_active: false,
            light_intensity: 15.0,  // Reasonable intensity for solar lights
            light_radius: 120.0,    // Good coverage radius
        }
    }
}

impl Default for GardenLamp {
    fn default() -> Self {
        Self {
            is_active: false,
            light_intensity: 25.0,   // Brighter than solar lights
            light_radius: 180.0,     // Larger coverage than solar lights
            power_consumption: 1.0,
        }
    }
}

impl Default for LightSource {
    fn default() -> Self {
        Self {
            current_intensity: 0.0,
            target_intensity: 0.0,
            light_entity: None,
            insect_attraction_radius: 100.0,
            attracted_insects: Vec::new(),
        }
    }
}

impl Default for InsectAttractor {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
            max_insects_per_light: 5,
            spawn_radius: 50.0,
            active_insects: HashMap::new(),
        }
    }
}

// Setup function to initialize the 2D lighting system
pub fn setup_lighting_system(
    mut commands: Commands,
    mut lighting_manager: ResMut<LightingManager>,
) {
    info!("Initializing garden lighting system with bevy_light_2d");
    
    // Set up ambient lighting - starts dim for dawn/dusk effect
    lighting_manager.ambient_light_level = 0.3;
    
    // Spawn a few test lights for demonstration
    let solar1 = spawn_solar_light(&mut commands, Vec3::new(-200.0, 100.0, 0.0));
    let solar2 = spawn_solar_light(&mut commands, Vec3::new(200.0, 100.0, 0.0));
    let lamp1 = spawn_garden_lamp(&mut commands, Vec3::new(0.0, -100.0, 0.0));
    
    info!("Spawned test lighting objects - Solar1: {:?}, Solar2: {:?}, Lamp1: {:?}", 
        solar1, solar2, lamp1);
}

// System to manage solar lights - charging during day, operating at night
pub fn solar_light_management_system(
    mut commands: Commands,
    time_state: Res<TimeState>,
    time: Res<Time>,
    mut lighting_manager: ResMut<LightingManager>,
    mut solar_light_query: Query<(Entity, &mut SolarLight, &mut LightSource, &Transform), With<SolarLight>>,
) {
    let current_hour = time_state.hour as f32;
    let is_daylight = current_hour >= 6.0 && current_hour <= 18.0;
    let is_night = current_hour >= 20.0 || current_hour <= 5.0;
    
    // Debug logging every few seconds
    static mut LAST_DEBUG: f32 = 0.0;
    if unsafe { time.elapsed_secs() - LAST_DEBUG > 10.0 } {
        info!("Solar light system: current_hour={}, is_daylight={}, is_night={}, solar_lights_count={}", 
            current_hour, is_daylight, is_night, solar_light_query.iter().len());
        unsafe { LAST_DEBUG = time.elapsed_secs(); }
    }
    
    for (entity, mut solar_light, mut light_source, transform) in solar_light_query.iter_mut() {
        let delta_hours = time.delta_secs() / 3600.0; // Convert seconds to hours
        
        if is_daylight {
            // Charge during daylight
            solar_light.charge_level = (solar_light.charge_level + 
                solar_light.charging_rate * delta_hours).min(1.0);
            solar_light.current_runtime = solar_light.charge_level * solar_light.max_runtime_hours;
            
            // Turn off during day
            if solar_light.is_active {
                solar_light.is_active = false;
                light_source.target_intensity = 0.0;
                
                // Remove the actual light entity if it exists
                if let Some(light_entity) = light_source.light_entity.take() {
                    commands.entity(light_entity).despawn();
                }
            }
        } else if is_night && solar_light.current_runtime > 0.0 { // Fixed: Back to proper night and charge detection
            // Operate at night if charged
            if !solar_light.is_active {
                solar_light.is_active = true;
                
                // Create the actual light entity
                let light_entity = commands.spawn((
                    PointLight2d {
                        intensity: solar_light.light_intensity,
                        radius: solar_light.light_radius,
                        falloff: 1.5,
                        color: Color::srgb(1.0, 0.95, 0.8), // Warm white
                        cast_shadows: false,
                    },
                    Transform::from_translation(transform.translation),
                )).id();
                
                light_source.light_entity = Some(light_entity);
                light_source.target_intensity = solar_light.light_intensity;
                
                info!("🌙 Solar light activated at {:?} with intensity {} and radius {}", 
                    transform.translation, solar_light.light_intensity, solar_light.light_radius);
            }
            
            // Consume charge
            solar_light.current_runtime = (solar_light.current_runtime - delta_hours).max(0.0);
            
            // Dim as charge depletes
            let charge_ratio = solar_light.current_runtime / solar_light.max_runtime_hours;
            let dimmed_intensity = solar_light.light_intensity * charge_ratio;
            light_source.target_intensity = dimmed_intensity;
            
            // Update actual light intensity
            if let Some(light_entity) = light_source.light_entity {
                if let Ok(mut point_light) = commands.get_entity(light_entity)
                    .and_then(|mut entity_commands| {
                        // This is a workaround since we can't directly query the light component
                        // In a real implementation, you'd maintain the light component reference
                        Ok(())
                    }) {
                    // Update light intensity - this would need proper component access
                    // For now, we'll recreate the light if intensity changes significantly
                    if (light_source.current_intensity - dimmed_intensity).abs() > 0.5 {
                        commands.entity(light_entity).despawn();
                        
                        let new_light_entity = commands.spawn((
                            PointLight2d {
                                intensity: dimmed_intensity,
                                radius: solar_light.light_radius,
                                falloff: 1.5,
                                color: Color::srgb(1.0, 0.95, 0.8),
                                cast_shadows: false,
                            },
                            Transform::from_translation(transform.translation),
                        )).id();
                        
                        light_source.light_entity = Some(new_light_entity);
                    }
                }
            }
            
            light_source.current_intensity = dimmed_intensity;
            
            // Turn off when depleted
            if solar_light.current_runtime <= 0.0 {
                solar_light.is_active = false;
                light_source.target_intensity = 0.0;
                
                if let Some(light_entity) = light_source.light_entity.take() {
                    commands.entity(light_entity).despawn();
                }
            }
        }
    }
}

// System to manage garden lamps - operate all night when turned on
pub fn garden_lamp_management_system(
    mut commands: Commands,
    time_state: Res<TimeState>,
    time: Res<Time>,
    mut lighting_manager: ResMut<LightingManager>,
    mut garden_lamp_query: Query<(Entity, &mut GardenLamp, &mut LightSource, &Transform), With<GardenLamp>>,
) {
    let current_hour = time_state.hour as f32;
    let is_night = current_hour >= 20.0 || current_hour <= 6.0;
    
    // Debug logging for garden lamps
    static mut LAST_LAMP_DEBUG: f32 = 0.0;
    if unsafe { time.elapsed_secs() - LAST_LAMP_DEBUG > 10.0 } {
        info!("Garden lamp system: current_hour={}, is_night={}, garden_lamps_count={}", 
            current_hour, is_night, garden_lamp_query.iter().len());
        unsafe { LAST_LAMP_DEBUG = time.elapsed_secs(); }
    }
    
    for (entity, mut garden_lamp, mut light_source, transform) in garden_lamp_query.iter_mut() {
        if is_night && !garden_lamp.is_active { // Fixed: Back to proper night detection
            // Turn on at night
            garden_lamp.is_active = true;
            
            let light_entity = commands.spawn((
                PointLight2d {
                    intensity: garden_lamp.light_intensity,
                    radius: garden_lamp.light_radius,
                    falloff: 2.0,
                    color: Color::srgb(1.0, 1.0, 0.9), // Cool white
                    cast_shadows: false,
                },
                Transform::from_translation(transform.translation),
            )).id();
            
            light_source.light_entity = Some(light_entity);
            light_source.current_intensity = garden_lamp.light_intensity;
            light_source.target_intensity = garden_lamp.light_intensity;
            
            info!("🏮 Garden lamp activated at {:?} with intensity {} and radius {}", 
                transform.translation, garden_lamp.light_intensity, garden_lamp.light_radius);
            
        } else if !is_night && garden_lamp.is_active {
            // Turn off during day
            garden_lamp.is_active = false;
            light_source.target_intensity = 0.0;
            
            if let Some(light_entity) = light_source.light_entity.take() {
                commands.entity(light_entity).despawn();
            }
            
            light_source.current_intensity = 0.0;
        }
    }
}

// System to spawn and manage moths/insects attracted to lights
pub fn moth_attraction_system(
    mut insect_attractor: ResMut<InsectAttractor>,
    time: Res<Time>,
    light_query: Query<(Entity, &Transform, &LightSource), (With<LightSource>, Without<NocturnalInsect>)>,
    mut insect_query: Query<(Entity, &mut Transform, &mut NocturnalInsect), Without<LightSource>>,
    mut moth_events: EventWriter<MothSpawnEvent>,
) {
    insect_attractor.spawn_timer.tick(time.delta());
    
    // Spawn new insects near active lights
    if insect_attractor.spawn_timer.finished() {
        for (light_entity, light_transform, light_source) in light_query.iter() {
            if light_source.current_intensity > 0.5 {
                let current_insect_count = insect_attractor.active_insects
                    .get(&light_entity)
                    .map(|insects| insects.len() as u32)
                    .unwrap_or(0);
                    
                if current_insect_count < insect_attractor.max_insects_per_light {
                    // Random spawn position around the light
                    use rand::Rng;
                    let mut rng = rand::rng();
                    let angle = rng.random_range(0.0..std::f32::consts::TAU);
                    let distance = rng.random_range(30.0..insect_attractor.spawn_radius);
                    
                    let spawn_pos = light_transform.translation + Vec3::new(
                        angle.cos() * distance,
                        angle.sin() * distance,
                        1.0,
                    );
                    
                    // Random insect type
                    let insect_type = match rng.random_range(0..10) {
                        0..=6 => InsectType::Moth,   // 70% moths
                        7..=8 => InsectType::Gnat,   // 20% gnats  
                        _ => InsectType::Beetle,     // 10% beetles
                    };
                    
                    moth_events.write(MothSpawnEvent {
                        light_entity,
                        spawn_position: spawn_pos,
                        insect_type,
                    });
                }
            }
        }
    }
    
    // Update existing insect behavior
    for (insect_entity, mut insect_transform, mut insect) in insect_query.iter_mut() {
        insect.lifespan.tick(time.delta());
        
        // Move towards target light if attracted
        if let Some(target_light) = insect.target_light {
            if let Ok((_, light_transform, light_source)) = light_query.get(target_light) {
                if light_source.current_intensity > 0.1 {
                    let direction = (light_transform.translation - insect_transform.translation).normalize();
                    let movement = direction * insect.movement_speed * time.delta_secs();
                    
                    // Add some randomness to movement
                    use rand::Rng;
                    let mut rng = rand::rng();
                    let random_offset = Vec3::new(
                        rng.random_range(-10.0..10.0),
                        rng.random_range(-10.0..10.0),
                        0.0,
                    ) * time.delta_secs();
                    
                    insect_transform.translation += movement + random_offset;
                }
            }
        }
    }
}

// System to handle insect spawning events
pub fn insect_spawning_system(
    mut commands: Commands,
    mut moth_events: EventReader<MothSpawnEvent>,
    mut insect_attractor: ResMut<InsectAttractor>,
) {
    for event in moth_events.read() {
        let insect_entity = commands.spawn((
            Transform::from_translation(event.spawn_position),
            Sprite {
                color: match event.insect_type {
                    InsectType::Moth => Color::srgb(0.9, 0.9, 0.7),
                    InsectType::Gnat => Color::srgb(0.3, 0.3, 0.3),
                    InsectType::Beetle => Color::srgb(0.2, 0.1, 0.0),
                },
                custom_size: Some(Vec2::splat(match event.insect_type {
                    InsectType::Moth => 4.0,
                    InsectType::Gnat => 2.0,
                    InsectType::Beetle => 3.0,
                })),
                ..default()
            },
            NocturnalInsect {
                insect_type: event.insect_type,
                attracted_to_light: true,
                attraction_strength: event.insect_type.attraction_strength(),
                movement_speed: event.insect_type.movement_speed(),
                lifespan: Timer::from_seconds(
                    event.insect_type.lifespan_minutes() * 60.0, 
                    TimerMode::Once
                ),
                target_light: Some(event.light_entity),
            },
            // Make insects provide utility for nocturnal birds
            SmartObject,
            ProvidesUtility {
                action: BirdAction::Eat,
                base_utility: 0.6,
                range: 15.0,
            },
        )).id();
        
        // Track the insect
        insect_attractor.active_insects
            .entry(event.light_entity)
            .or_insert_with(Vec::new)
            .push(insect_entity);
    }
}

// System to optimize lighting performance and clean up dead insects
pub fn light_optimization_system(
    mut commands: Commands,
    mut insect_attractor: ResMut<InsectAttractor>,
    insect_query: Query<(Entity, &NocturnalInsect), With<NocturnalInsect>>,
) {
    // Clean up dead insects
    let mut dead_insects = Vec::new();
    
    for (entity, insect) in insect_query.iter() {
        if insect.lifespan.finished() {
            dead_insects.push(entity);
            commands.entity(entity).despawn();
        }
    }
    
    // Remove dead insects from tracking
    for light_insects in insect_attractor.active_insects.values_mut() {
        light_insects.retain(|entity| !dead_insects.contains(entity));
    }
    
    // Remove empty light entries
    insect_attractor.active_insects.retain(|_, insects| !insects.is_empty());
}


// Function to spawn a solar light at a position
pub fn spawn_solar_light(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    info!("🔧 Creating solar light at {:?}", position);
    let entity = commands.spawn((
        Transform::from_translation(position),
        SolarLight::default(),
        LightSource::default(),
        SmartObject,
        Sprite {
            color: Color::srgb(0.8, 0.8, 0.9),
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..default()
        },
        // Physics for collision
        RigidBody::Fixed,
        Collider::cuboid(8.0, 16.0),
        CollisionGroups::new(Group::GROUP_2, Group::ALL), // Environment collision group
    )).id();
    info!("🔧 Solar light spawned with entity: {:?}", entity);
    entity
}

// Function to spawn a garden lamp at a position  
pub fn spawn_garden_lamp(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    info!("🔧 Creating garden lamp at {:?}", position);
    let entity = commands.spawn((
        Transform::from_translation(position),
        GardenLamp::default(),
        LightSource::default(),
        SmartObject,
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.95),
            custom_size: Some(Vec2::new(20.0, 48.0)),
            ..default()
        },
        // Physics for collision
        RigidBody::Fixed,
        Collider::cuboid(10.0, 24.0),
        CollisionGroups::new(Group::GROUP_2, Group::ALL), // Environment collision group
    )).id();
    info!("🔧 Garden lamp spawned with entity: {:?}", entity);
    entity
}

// Debug system to display current time and lighting state
pub fn debug_lighting_info(
    time_state: Res<TimeState>,
    time: Res<Time>,
    solar_light_query: Query<&SolarLight>,
    garden_lamp_query: Query<&GardenLamp>,
) {
    static mut LAST_DEBUG: f32 = 0.0;
    if unsafe { time.elapsed_secs() - LAST_DEBUG > 15.0 } {
        let current_hour = time_state.hour as f32;
        let active_solars = solar_light_query.iter().filter(|s| s.is_active).count();
        let active_lamps = garden_lamp_query.iter().filter(|l| l.is_active).count();
        
        info!("🕐 TIME: {}:00 | 🌙 Night: {} | ☀️ Solar: {}/{} active | 🏮 Lamps: {}/{} active", 
            current_hour as u32, 
            current_hour >= 20.0 || current_hour <= 6.0,
            active_solars, solar_light_query.iter().len(),
            active_lamps, garden_lamp_query.iter().len()
        );
        unsafe { LAST_DEBUG = time.elapsed_secs(); }
    }
}
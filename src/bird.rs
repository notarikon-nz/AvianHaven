use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{AppState, resources::{GameConfig, BirdCount, SpawnBirdEvent}};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BirdCount>()
            .add_event::<SpawnBirdEvent>()
            .add_systems(
                Update,
                (
                    handle_spawn_events,
                    bird_movement,
                    update_wander_timer,
                ).run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BirdSpecies {
    Cardinal,
    BlueJay,
    Robin,
    Sparrow,
}

impl BirdSpecies {
    fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Self::Cardinal,
            1 => Self::BlueJay,
            2 => Self::Robin,
            _ => Self::Sparrow,
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Cardinal => Color::srgb(0.8, 0.2, 0.2),
            Self::BlueJay => Color::srgb(0.2, 0.4, 0.8),
            Self::Robin => Color::srgb(0.6, 0.3, 0.1),
            Self::Sparrow => Color::srgb(0.5, 0.4, 0.3),
        }
    }
}

#[derive(Component)]
pub struct Bird {
    pub species: BirdSpecies,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
struct WanderTimer(Timer);

fn handle_spawn_events(
    mut commands: Commands,
    mut events: EventReader<SpawnBirdEvent>,
    mut bird_count: ResMut<BirdCount>,
) {
    for _ in events.read() {
        spawn_bird(&mut commands);
        bird_count.0 += 1;
    }
}

fn spawn_bird(commands: &mut Commands) {
    let species = BirdSpecies::random();
    let mut rng = rand::rng();
    
    let x = rng.random_range(-400.0..400.0);
    let y = rng.random_range(-300.0..300.0);
    
    commands.spawn((
        Sprite::from_color(species.color(), Vec2::new(20.0, 20.0)),
        Transform::from_xyz(x, y, 1.0),
        RigidBody::Dynamic,
        Collider::ball(10.0),
        Bird { species },
        Velocity(Vec2::ZERO),
        WanderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
        GravityScale(0.0),
        Damping { linear_damping: 2.0, angular_damping: 10.0 },
    ));
}

fn bird_movement(
    mut bird_query: Query<(&mut Velocity, &mut Transform), With<Bird>>,
    feeder_query: Query<&Transform, (With<crate::feeder::Feeder>, Without<Bird>)>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform) in bird_query.iter_mut() {
        let mut force = Vec2::ZERO;
        
        // Attraction to feeders
        for feeder_transform in feeder_query.iter() {
            let distance = transform.translation.truncate().distance(feeder_transform.translation.truncate());
            if distance < config.bird_attraction_radius {
                let direction = (feeder_transform.translation.truncate() - transform.translation.truncate()).normalize();
                let strength = (config.bird_attraction_radius - distance) / config.bird_attraction_radius;
                force += direction * strength * config.bird_attraction_force;
            }
        }
        
        // Simple obstacle avoidance (removed Rapier context usage)
        let mut avoidance_force = Vec2::ZERO;
        for other_feeder in feeder_query.iter() {
            let distance = transform.translation.truncate().distance(other_feeder.translation.truncate());
            if distance < config.bird_avoidance_radius && distance > 0.1 {
                let direction = (transform.translation.truncate() - other_feeder.translation.truncate()).normalize();
                avoidance_force += direction * (config.bird_avoidance_radius - distance) / config.bird_avoidance_radius;
            }
        }
        force += avoidance_force * config.bird_attraction_force;
        
        // Apply wandering velocity
        let target_velocity = velocity.0 + force * time.delta().as_secs_f32();
        let max_speed = config.bird_wander_speed;
        if target_velocity.length() > max_speed {
            velocity.0 = target_velocity.normalize() * max_speed;
        } else {
            velocity.0 = target_velocity;
        }
        
        // Update transform
        transform.translation += velocity.0.extend(0.0) * time.delta().as_secs_f32();
    }
}

fn update_wander_timer(
    mut bird_query: Query<(&mut Velocity, &mut WanderTimer), With<Bird>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    
    for (mut velocity, mut timer) in bird_query.iter_mut() {
        timer.0.tick(time.delta());
        
        if timer.0.just_finished() {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let speed = config.bird_wander_speed * rng.random_range(0.3..1.0);
            velocity.0 = Vec2::new(angle.cos(), angle.sin()) * speed;
        }
    }
}
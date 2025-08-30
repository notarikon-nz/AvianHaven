use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct FeederPlugin;

impl Plugin for FeederPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_feeder);
    }
}

#[derive(Component)]
pub struct Feeder {
    pub feeder_type: FeederType,
    pub attraction_radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum FeederType {
    Seed,
    Suet,
    Nectar,
}

impl FeederType {
    fn color(&self) -> Color {
        match self {
            Self::Seed => Color::srgb(0.6, 0.4, 0.2),
            Self::Suet => Color::srgb(0.3, 0.3, 0.3),
            Self::Nectar => Color::srgb(0.8, 0.2, 0.4),
        }
    }
}

fn spawn_feeder(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(FeederType::Seed.color(), Vec2::new(40.0, 60.0)),
        Transform::from_xyz(100.0, -50.0, 0.5),
        RigidBody::Fixed,
        Collider::cuboid(20.0, 30.0),
        Sensor,
        Feeder {
            feeder_type: FeederType::Seed,
            attraction_radius: 150.0,
        },
    ));
    
    commands.spawn((
        Sprite::from_color(FeederType::Nectar.color(), Vec2::new(30.0, 50.0)),
        Transform::from_xyz(-120.0, 80.0, 0.5),
        RigidBody::Fixed,
        Collider::cuboid(15.0, 25.0),
        Sensor,
        Feeder {
            feeder_type: FeederType::Nectar,
            attraction_radius: 120.0,
        },
    ));
}
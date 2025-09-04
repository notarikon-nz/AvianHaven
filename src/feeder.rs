use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::bird_ai::components::{SmartObject, ProvidesUtility, BirdAction};

pub struct FeederPlugin;

impl Plugin for FeederPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FeederDepletionEvent>()
            .add_event::<FeederUpgradeEvent>()
            .add_systems(Startup, spawn_feeder)
            .add_systems(Update, (
                update_feeder_capacity_system,
                update_feeder_visual_system,
                handle_feeder_upgrade_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}

#[derive(Component)]
pub struct Feeder {
    pub feeder_type: FeederType,
    pub attraction_radius: f32,
    pub current_capacity: f32,
    pub max_capacity: f32,
    pub depletion_rate: f32,  // Amount consumed per feeding action
    pub upgrade_level: u32,   // 0 = basic, higher = better
}

#[derive(Debug, Clone, Copy)]
pub enum FeederType {
    Seed,
    Suet,
    Nectar,
    Fruit,
    Ground,
}

impl FeederType {
    fn provides_action(&self) -> BirdAction {
        match self {
            Self::Seed => BirdAction::Eat,
            Self::Suet => BirdAction::Eat,
            Self::Nectar => BirdAction::Drink,
            Self::Fruit => BirdAction::Eat,
            Self::Ground => BirdAction::Eat,
        }
    }
    
    fn base_utility(&self) -> f32 {
        match self {
            Self::Seed => 0.7,
            Self::Suet => 0.8,
            Self::Nectar => 0.6,
            Self::Fruit => 0.6,
            Self::Ground => 0.5,
        }
    }
    
    fn base_max_capacity(&self) -> f32 {
        match self {
            Self::Seed => 100.0,
            Self::Suet => 80.0,
            Self::Nectar => 120.0,
            Self::Fruit => 90.0,
            Self::Ground => 200.0,  // Ground feeding areas hold more
        }
    }
    
    fn base_depletion_rate(&self) -> f32 {
        match self {
            Self::Seed => 5.0,
            Self::Suet => 8.0,
            Self::Nectar => 3.0,
            Self::Fruit => 6.0,
            Self::Ground => 4.0,
        }
    }
    
    // Calculate stats based on upgrade level
    pub fn max_capacity(&self, upgrade_level: u32) -> f32 {
        self.base_max_capacity() * (1.0 + upgrade_level as f32 * 0.25)
    }
    
    pub fn depletion_rate(&self, upgrade_level: u32) -> f32 {
        // Higher level feeders are more efficient (last longer)
        self.base_depletion_rate() * (1.0 / (1.0 + upgrade_level as f32 * 0.15))
    }
    
    pub fn upgrade_cost(upgrade_level: u32) -> u32 {
        match upgrade_level {
            0 => 100,  // Level 0 -> 1
            1 => 250,  // Level 1 -> 2  
            2 => 500,  // Level 2 -> 3
            3 => 1000, // Level 3 -> 4
            _ => u32::MAX, // Max level reached
        }
    }
}

impl FeederType {
    fn color(&self) -> Color {
        match self {
            Self::Seed => Color::srgb(0.6, 0.4, 0.2),
            Self::Suet => Color::srgb(0.3, 0.3, 0.3),
            Self::Nectar => Color::srgb(0.8, 0.2, 0.4),
            Self::Fruit => Color::srgb(0.8, 0.4, 0.2),
            Self::Ground => Color::srgb(0.4, 0.3, 0.2),
        }
    }
}

fn spawn_feeder(mut commands: Commands) {
    let seed_feeder = FeederType::Seed;
    commands.spawn((
        Sprite::from_color(seed_feeder.color(), Vec2::new(40.0, 60.0)),
        Transform::from_xyz(100.0, -50.0, 0.5),
        RigidBody::Fixed,
        Collider::cuboid(20.0, 30.0),
        Sensor,
        Feeder {
            feeder_type: seed_feeder,
            attraction_radius: 150.0,
            current_capacity: seed_feeder.max_capacity(0),
            max_capacity: seed_feeder.max_capacity(0),
            depletion_rate: seed_feeder.depletion_rate(0),
            upgrade_level: 0,
        },
        SmartObject,
        ProvidesUtility {
            action: seed_feeder.provides_action(),
            base_utility: seed_feeder.base_utility(),
            range: 150.0,
        },
    ));
    
    let nectar_feeder = FeederType::Nectar;
    commands.spawn((
        Sprite::from_color(nectar_feeder.color(), Vec2::new(30.0, 50.0)),
        Transform::from_xyz(-120.0, 80.0, 0.5),
        RigidBody::Fixed,
        Collider::cuboid(15.0, 25.0),
        Sensor,
        Feeder {
            feeder_type: nectar_feeder,
            attraction_radius: 120.0,
            current_capacity: nectar_feeder.max_capacity(0),
            max_capacity: nectar_feeder.max_capacity(0),
            depletion_rate: nectar_feeder.depletion_rate(0),
            upgrade_level: 0,
        },
        SmartObject,
        ProvidesUtility {
            action: nectar_feeder.provides_action(),
            base_utility: nectar_feeder.base_utility(),
            range: 120.0,
        },
    ));
    
    // Add Suet feeder
    let suet_feeder = FeederType::Suet;
    commands.spawn((
        Sprite::from_color(suet_feeder.color(), Vec2::new(25.0, 35.0)),
        Transform::from_xyz(50.0, 150.0, 0.5),
        RigidBody::Fixed,
        Collider::cuboid(12.5, 17.5),
        Sensor,
        Feeder {
            feeder_type: suet_feeder,
            attraction_radius: 140.0,
            current_capacity: suet_feeder.max_capacity(0),
            max_capacity: suet_feeder.max_capacity(0),
            depletion_rate: suet_feeder.depletion_rate(0),
            upgrade_level: 0,
        },
        SmartObject,
        ProvidesUtility {
            action: suet_feeder.provides_action(),
            base_utility: suet_feeder.base_utility(),
            range: 140.0,
        },
    ));
    
    // Add Ground feeding area
    let ground_feeder = FeederType::Ground;
    commands.spawn((
        Sprite::from_color(ground_feeder.color(), Vec2::new(60.0, 20.0)),
        Transform::from_xyz(-200.0, -50.0, 0.1),
        RigidBody::Fixed,
        Collider::cuboid(30.0, 10.0),
        Sensor,
        Feeder {
            feeder_type: ground_feeder,
            attraction_radius: 100.0,
            current_capacity: ground_feeder.max_capacity(0),
            max_capacity: ground_feeder.max_capacity(0),
            depletion_rate: ground_feeder.depletion_rate(0),
            upgrade_level: 0,
        },
        SmartObject,
        ProvidesUtility {
            action: ground_feeder.provides_action(),
            base_utility: ground_feeder.base_utility(),
            range: 100.0,
        },
    ));
}

#[derive(Event)]
pub struct FeederDepletionEvent {
    pub feeder_entity: Entity,
    pub amount: f32,
}

fn update_feeder_capacity_system(
    mut feeder_query: Query<(Entity, &mut Feeder, &mut ProvidesUtility)>,
    mut depletion_events: EventReader<FeederDepletionEvent>,
) {
    for event in depletion_events.read() {
        if let Ok((_, mut feeder, mut utility)) = feeder_query.get_mut(event.feeder_entity) {
            feeder.current_capacity = (feeder.current_capacity - event.amount).max(0.0);
            
            // Reduce utility as feeder empties
            let capacity_ratio = feeder.current_capacity / feeder.max_capacity;
            utility.base_utility = feeder.feeder_type.base_utility() * capacity_ratio.max(0.1);
            
            if feeder.current_capacity <= 0.0 {
                info!("Feeder {:?} is empty!", feeder.feeder_type);
            }
        }
    }
}

fn update_feeder_visual_system(
    mut feeder_query: Query<(&Feeder, &mut Sprite), Changed<Feeder>>,
) {
    for (feeder, mut sprite) in feeder_query.iter_mut() {
        let capacity_ratio = feeder.current_capacity / feeder.max_capacity;
        
        // Darken the feeder as it empties
        let base_color = feeder.feeder_type.color();
        let dimmed_color = Color::srgb(
            base_color.to_srgba().red * (0.3 + 0.7 * capacity_ratio),
            base_color.to_srgba().green * (0.3 + 0.7 * capacity_ratio),
            base_color.to_srgba().blue * (0.3 + 0.7 * capacity_ratio),
        );
        sprite.color = dimmed_color;
    }
}

#[derive(Event)]
pub struct FeederUpgradeEvent {
    pub feeder_entity: Entity,
}

fn handle_feeder_upgrade_system(
    commands: Commands,
    mut upgrade_events: EventReader<FeederUpgradeEvent>,
    mut feeder_query: Query<&mut Feeder>,
    mut currency: ResMut<crate::photo_mode::resources::CurrencyResource>,
) {
    for event in upgrade_events.read() {
        if let Ok(mut feeder) = feeder_query.get_mut(event.feeder_entity) {
            let upgrade_cost = FeederType::upgrade_cost(feeder.upgrade_level);
            
            if currency.0 >= upgrade_cost && feeder.upgrade_level < 4 {
                // Deduct currency
                currency.0 -= upgrade_cost;
                
                // Upgrade feeder
                feeder.upgrade_level += 1;
                let new_max_capacity = feeder.feeder_type.max_capacity(feeder.upgrade_level);
                let capacity_ratio = feeder.current_capacity / feeder.max_capacity;
                
                feeder.max_capacity = new_max_capacity;
                feeder.current_capacity = new_max_capacity * capacity_ratio; // Maintain fill ratio
                feeder.depletion_rate = feeder.feeder_type.depletion_rate(feeder.upgrade_level);
                
                info!("Upgraded {:?} feeder to level {} (Cost: {})", 
                      feeder.feeder_type, feeder.upgrade_level, upgrade_cost);
                info!("New capacity: {}, New efficiency: {:.2}", 
                      feeder.max_capacity, feeder.depletion_rate);
            } else if currency.0 < upgrade_cost {
                info!("Not enough currency to upgrade feeder. Need: {}, Have: {}", 
                      upgrade_cost, currency.0);
            } else {
                info!("Feeder already at maximum level!");
            }
        }
    }
}
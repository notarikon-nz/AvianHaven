use bevy::prelude::*;
use crate::bird_ai::{components::*, resources::*, bt::*, states::*};

pub fn setup_test_world(mut commands: Commands) {
    // Spawn test bird
    commands.spawn((
        Sprite::from_color(Color::srgb(0.8, 0.3, 0.3), Vec2::new(20.0, 20.0)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        BirdAI,
        BirdState::Wandering,
        Blackboard {
            internal: InternalState {
                hunger: 0.6,
                thirst: 0.4,
                energy: 0.8,
                fear: 0.0,
            },
            ..default()
        },
    ));
    
    // Spawn feeder
    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.4, 0.2), Vec2::new(30.0, 40.0)),
        Transform::from_xyz(150.0, 100.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Eat,
            base_utility: 0.4,
            range: 300.0,
        },
    ));

    // Spawn premium feeder
    commands.spawn((
        Sprite::from_color(Color::srgb(0.6, 0.4, 0.2), Vec2::new(30.0, 40.0)),
        Transform::from_xyz(-50.0, 100.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Eat,
            base_utility: 0.9,
            range: 500.0,
        },
    ));    
    
    // Spawn water source
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 0.8), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(-120.0, 80.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Drink,
            base_utility: 0.8,
            range: 180.0,
        },
    ));
    
    // Spawn bird bath
    commands.spawn((
        Sprite::from_color(Color::srgb(0.7, 0.7, 0.9), Vec2::new(35.0, 35.0)),
        Transform::from_xyz(80.0, -120.0, 0.5),
        SmartObject,
        ProvidesUtility {
            action: BirdAction::Bathe,
            base_utility: 0.5,
            range: 150.0,
        },
    ));
}

pub fn world_utility_query_system(
    mut bird_query: Query<(&Transform, &mut Blackboard), With<BirdAI>>,
    object_query: Query<(Entity, &Transform, &ProvidesUtility), With<SmartObject>>,
    mut timer: ResMut<UtilityTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() { return; }
    
    for (bird_transform, mut blackboard) in bird_query.iter_mut() {
        blackboard.world_knowledge.available_actions.clear();
        
        for (entity, obj_transform, utility) in object_query.iter() {
            let distance = bird_transform.translation.distance(obj_transform.translation);
            if distance <= utility.range {
                let distance_factor = 1.0 - (distance / utility.range);
                let final_score = utility.base_utility * distance_factor;
                
                let entry = UtilityEntry { entity, score: final_score };
                
                if let Some(existing) = blackboard.world_knowledge.available_actions.get(&utility.action) {
                    if final_score > existing.score {
                        blackboard.world_knowledge.available_actions.insert(utility.action, entry);
                    }
                } else {
                    blackboard.world_knowledge.available_actions.insert(utility.action, entry);
                }
            }
        }
    }
}

pub fn behavior_tree_system(
    mut bird_query: Query<(&mut BirdState, &mut Blackboard), With<BirdAI>>,
    mut timer: ResMut<BehaviorTreeTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() { return; }
    
    for (mut state, mut blackboard) in bird_query.iter_mut() {
        let new_state = evaluate_behavior_tree(&blackboard);
        
        if new_state == BirdState::MovingToTarget {
            // Set target based on highest priority need
            let internal = &blackboard.internal;
            let actions = &blackboard.world_knowledge.available_actions;
            
            blackboard.current_target = if internal.hunger > 0.5 {
                actions.get(&BirdAction::Eat).map(|e| e.entity)
            } else if internal.thirst > 0.5 {
                actions.get(&BirdAction::Drink).map(|e| e.entity)
            } else {
                actions.get(&BirdAction::Bathe).map(|e| e.entity)
            };
        }
        
        *state = new_state;
    }
}

pub fn wandering_system(
    mut bird_query: Query<&mut Transform, (With<BirdAI>, With<BirdState>)>,
    state_query: Query<&BirdState, With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, state) in bird_query.iter_mut().zip(state_query.iter()) {
        if *state == BirdState::Wandering {
            execute_wandering(&mut transform, &time);
        }
    }
}

pub fn moving_to_target_system(
    mut bird_query: Query<(&mut Transform, &mut Blackboard, &mut BirdState), With<BirdAI>>,
    target_query: Query<&Transform, Without<BirdAI>>,
    time: Res<Time>,
) {
    for (mut bird_transform, mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::MovingToTarget {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    let reached = execute_moving_to_target(&mut bird_transform, target_transform, &time);
                    
                    if reached {
                        let internal = &blackboard.internal;
                        *state = if internal.hunger > internal.thirst {
                            BirdState::Eating
                        } else if internal.thirst > 0.3 {
                            BirdState::Drinking
                        } else {
                            BirdState::Bathing
                        };
                    }
                }
            }
        }
    }
}

pub fn eating_system(
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Eating {
            blackboard.internal.hunger -= 0.5 * time.delta().as_secs_f32();
            blackboard.internal.hunger = blackboard.internal.hunger.max(0.0);
            
            if blackboard.internal.hunger < 0.1 {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn drinking_system(
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Drinking {
            blackboard.internal.thirst -= 0.6 * time.delta().as_secs_f32();
            blackboard.internal.thirst = blackboard.internal.thirst.max(0.0);
            
            if blackboard.internal.thirst < 0.1 {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn bathing_system(
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Bathing {
            blackboard.internal.energy += 0.3 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.min(1.0);
            
            // Bathing duration
            if blackboard.internal.energy > 0.8 {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn fleeing_system(
    mut bird_query: Query<(&mut Transform, &Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut transform, blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Fleeing {
            if let Some(threat_dir) = blackboard.world_knowledge.perceived_threat {
                execute_fleeing(&mut transform, threat_dir, &time);
                
                if blackboard.internal.fear < 0.3 {
                    *state = BirdState::Wandering;
                }
            }
        }
    }
}

pub fn resting_system(
    mut bird_query: Query<(&mut Blackboard, &mut BirdState), With<BirdAI>>,
    time: Res<Time>,
) {
    for (mut blackboard, mut state) in bird_query.iter_mut() {
        if *state == BirdState::Resting {
            blackboard.internal.energy += 0.4 * time.delta().as_secs_f32();
            blackboard.internal.energy = blackboard.internal.energy.min(1.0);
            
            if blackboard.internal.energy > 0.7 {
                *state = BirdState::Wandering;
            }
        }
    }
}

pub fn need_decay_system(
    mut bird_query: Query<&mut Blackboard, With<BirdAI>>,
    time: Res<Time>,
) {
    for mut blackboard in bird_query.iter_mut() {
        let decay_rate = 0.1 * time.delta().as_secs_f32();
        
        blackboard.internal.hunger += decay_rate;
        blackboard.internal.hunger = blackboard.internal.hunger.min(1.0);
        
        blackboard.internal.thirst += decay_rate * 1.2;
        blackboard.internal.thirst = blackboard.internal.thirst.min(1.0);
        
        blackboard.internal.energy -= decay_rate * 0.5;
        blackboard.internal.energy = blackboard.internal.energy.max(0.0);
        
        blackboard.internal.fear *= 0.95; // Fear decays faster
    }
}
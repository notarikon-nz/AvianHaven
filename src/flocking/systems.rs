use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::components::*;
use crate::bird::{Bird, BirdSpecies};
use crate::bird_ai::components::{BirdAI, BirdState};
use rand::Rng;

const FLOCKING_SEARCH_RADIUS: f32 = 100.0;
const SEPARATION_WEIGHT: f32 = 2.0;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 1.0;

pub fn flocking_behavior_system(
    mut bird_query: Query<(Entity, &Transform, &mut Velocity, &Bird, Option<&mut FlockMember>), With<BirdAI>>,
    flock_query: Query<&Flock>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let birds: Vec<_> = bird_query.iter().map(|(e, t, v, b, _)| (e, t.translation.truncate(), v.linvel, b.species)).collect();
    
    for (entity, transform, mut velocity, bird, flock_member) in bird_query.iter_mut() {
        let bird_pos = transform.translation.truncate();
        
        // Find nearby birds of compatible species
        let nearby_birds: Vec<_> = birds.iter()
            .filter(|(other_entity, other_pos, _, other_species)| {
                *other_entity != entity && 
                bird_pos.distance(*other_pos) <= FLOCKING_SEARCH_RADIUS &&
                bird.species.social_compatibility(other_species) > 0.5
            })
            .collect();
        
        if nearby_birds.is_empty() {
            continue;
        }
        
        // Calculate flocking forces
        let separation = calculate_separation(&bird_pos, &nearby_birds);
        let alignment = calculate_alignment(&nearby_birds);
        let cohesion = calculate_cohesion(&bird_pos, &nearby_birds);
        
        // Apply flocking behavior
        let flocking_force = separation * SEPARATION_WEIGHT + 
                           alignment * ALIGNMENT_WEIGHT + 
                           cohesion * COHESION_WEIGHT;
        
        let flocking_strength = flock_member.as_ref().map(|fm| fm.flocking_strength).unwrap_or(0.5);
        velocity.linvel += flocking_force * flocking_strength * time.delta().as_secs_f32() * 50.0;
        
        // Limit velocity
        if velocity.linvel.length() > 100.0 {
            velocity.linvel = velocity.linvel.normalize() * 100.0;
        }
        
        // Auto-assign FlockMember component if bird shows flocking behavior
        if flock_member.is_none() && bird.species.max_flock_size() > 1 {
            commands.entity(entity).insert(FlockMember::default());
        }
    }
}

pub fn territorial_behavior_system(
    mut bird_query: Query<(Entity, &Transform, &mut Velocity, &Bird, Option<&Territory>), With<BirdAI>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let birds: Vec<_> = bird_query.iter().map(|(e, t, _, b, ter)| (e, t.translation.truncate(), b.species, ter.is_some())).collect();
    
    for (entity, transform, mut velocity, bird, territory) in bird_query.iter_mut() {
        let bird_pos = transform.translation.truncate();
        
        // Establish territory if bird is territorial and doesn't have one
        if territory.is_none() && bird.species.aggression_level() > 0.5 {
            commands.entity(entity).insert(Territory::new(bird_pos, bird.species));
            continue;
        }
        
        if let Some(territory) = territory {
            // Defend territory against intruders
            for (other_entity, other_pos, other_species, _) in &birds {
                if *other_entity == entity {
                    continue;
                }
                
                let distance = bird_pos.distance(*other_pos);
                if distance < territory.radius {
                    // Check if this is an unwelcome intruder
                    let compatibility = bird.species.social_compatibility(other_species);
                    
                    if compatibility < 0.4 {
                        // Chase away the intruder
                        let chase_direction = (*other_pos - bird_pos).normalize();
                        let chase_force = chase_direction * territory.aggression_level * 80.0;
                        
                        velocity.linvel += chase_force * time.delta().as_secs_f32();
                        
                        // Limit aggressive chase speed
                        if velocity.linvel.length() > 120.0 {
                            velocity.linvel = velocity.linvel.normalize() * 120.0;
                        }
                    }
                }
            }
        }
    }
}

// This system is now handled by the dedicated predator-prey system
// Keeping it minimal to avoid conflicts
pub fn predator_avoidance_system(
    mut bird_query: Query<(Entity, &Transform, &mut Velocity, &Bird, &mut BirdState), With<BirdAI>>,
    mut commands: Commands,
) {
    // This system now just ensures PredatorAvoidance components are added when needed
    for (entity, _transform, _velocity, _bird, state) in bird_query.iter_mut() {
        // Add predator avoidance component if bird is fleeing and doesn't have it
        if *state == BirdState::Fleeing {
            commands.entity(entity).try_insert(PredatorAvoidance::default());
        }
    }
}

pub fn social_feeding_system(
    mut bird_query: Query<(Entity, &Transform, &Bird, &BirdState), With<BirdAI>>,
    feeder_query: Query<(Entity, &Transform), With<crate::feeder::Feeder>>,
    mut commands: Commands,
) {
    // Enhanced feeding behavior - birds are more likely to feed when others are feeding
    for (feeder_entity, feeder_transform) in &feeder_query {
        let feeder_pos = feeder_transform.translation.truncate();
        
        // Count birds currently feeding at this feeder
        let feeding_birds: Vec<_> = bird_query.iter()
            .filter(|(_, transform, _, state)| {
                matches!(state, BirdState::Eating) &&
                feeder_pos.distance(transform.translation.truncate()) < 60.0
            })
            .collect();
        
        if feeding_birds.len() >= 2 {
            // Social feeding bonus - attract more birds
            for (entity, transform, bird, state) in &bird_query {
                if matches!(state, BirdState::Wandering | BirdState::MovingToTarget) {
                    let distance = feeder_pos.distance(transform.translation.truncate());
                    
                    if distance < 200.0 && bird.species.max_flock_size() > 1 {
                        // Boost attraction to social feeding
                        // This would integrate with the utility system to increase feeder appeal
                        
                        // TO BE IMPLEMENTED
                        // ("Bird {:?} attracted to social feeding activity", bird.species);
                    }
                }
            }
        }
    }
}

// Flocking calculation helpers
fn calculate_separation(bird_pos: &Vec2, nearby_birds: &[&(Entity, Vec2, Vec2, BirdSpecies)]) -> Vec2 {
    let mut separation = Vec2::ZERO;
    
    for (_, other_pos, _, _) in nearby_birds {
        let diff = *bird_pos - *other_pos;
        let distance = diff.length();
        
        if distance > 0.0 && distance < 40.0 { // Too close
            separation += diff.normalize() / distance; // Stronger push when closer
        }
    }
    
    if separation.length() > 0.0 {
        separation.normalize()
    } else {
        Vec2::ZERO
    }
}

fn calculate_alignment(nearby_birds: &[&(Entity, Vec2, Vec2, BirdSpecies)]) -> Vec2 {
    if nearby_birds.is_empty() {
        return Vec2::ZERO;
    }
    
    let avg_velocity: Vec2 = nearby_birds.iter()
        .map(|(_, _, velocity, _)| *velocity)
        .sum::<Vec2>() / nearby_birds.len() as f32;
    
    if avg_velocity.length() > 0.0 {
        avg_velocity.normalize()
    } else {
        Vec2::ZERO
    }
}

fn calculate_cohesion(bird_pos: &Vec2, nearby_birds: &[&(Entity, Vec2, Vec2, BirdSpecies)]) -> Vec2 {
    if nearby_birds.is_empty() {
        return Vec2::ZERO;
    }
    
    let center_of_mass: Vec2 = nearby_birds.iter()
        .map(|(_, pos, _, _)| *pos)
        .sum::<Vec2>() / nearby_birds.len() as f32;
    
    let to_center = center_of_mass - *bird_pos;
    if to_center.length() > 0.0 {
        to_center.normalize()
    } else {
        Vec2::ZERO
    }
}
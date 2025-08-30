use bevy::prelude::*;

pub fn execute_wandering(transform: &mut Transform, time: &Time) {
    let speed = 30.0;
    let angle = time.elapsed().as_secs_f32() * 0.5;
    let direction = Vec2::new(angle.cos(), angle.sin()).normalize_or_zero();
    transform.translation += (direction * speed * time.delta().as_secs_f32()).extend(0.0);
}

pub fn execute_moving_to_target(
    transform: &mut Transform,
    target_transform: &Transform,
    time: &Time,
) -> bool {
    let speed = 80.0;
    let current_pos = transform.translation.truncate();
    let target_pos = target_transform.translation.truncate();
    let distance = current_pos.distance(target_pos);
    
    if distance < 25.0 {
        return true; // Reached target
    }
    
    let direction = (target_pos - current_pos).normalize_or_zero();
    transform.translation += (direction * speed * time.delta().as_secs_f32()).extend(0.0);
    false
}

pub fn execute_fleeing(transform: &mut Transform, threat_direction: Vec2, time: &Time) {
    let speed = 120.0;
    let flee_direction = -threat_direction.normalize_or_zero();
    transform.translation += (flee_direction * speed * time.delta().as_secs_f32()).extend(0.0);
}
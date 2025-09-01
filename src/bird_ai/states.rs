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

pub fn execute_playing(transform: &mut Transform, time: &Time) {
    // Playful movement - small hops and movements around the play object
    let play_intensity = 2.0;
    let hop_frequency = 3.0;
    let elapsed = time.elapsed().as_secs_f32();
    
    // Small hopping motion
    let hop_offset = (elapsed * hop_frequency).sin() * play_intensity;
    let lateral_movement = (elapsed * hop_frequency * 1.3).cos() * play_intensity * 0.5;
    
    transform.translation.x += lateral_movement * time.delta().as_secs_f32();
    transform.translation.y += hop_offset.abs() * time.delta().as_secs_f32() * 0.3;
}

pub fn execute_exploring(transform: &mut Transform, target_transform: &Transform, time: &Time) {
    // Cautious approach and investigation behavior
    let speed = 40.0; // Slower than normal movement
    let current_pos = transform.translation.truncate();
    let target_pos = target_transform.translation.truncate();
    let distance = current_pos.distance(target_pos);
    
    // Circle around the object of interest
    if distance > 60.0 {
        // Move closer if too far
        let direction = (target_pos - current_pos).normalize_or_zero();
        transform.translation += (direction * speed * time.delta().as_secs_f32()).extend(0.0);
    } else if distance < 30.0 {
        // Back away if too close (cautious exploration)
        let direction = (current_pos - target_pos).normalize_or_zero();
        transform.translation += (direction * speed * 0.5 * time.delta().as_secs_f32()).extend(0.0);
    } else {
        // Circle around at comfortable distance
        let angle_offset = time.elapsed().as_secs_f32() * 0.8;
        let circle_center = target_pos;
        let circle_radius = 45.0;
        let desired_pos = circle_center + Vec2::new(
            (angle_offset).cos() * circle_radius,
            (angle_offset).sin() * circle_radius
        );
        let direction = (desired_pos - current_pos).normalize_or_zero();
        transform.translation += (direction * speed * 0.7 * time.delta().as_secs_f32()).extend(0.0);
    }
}

pub fn execute_nesting(transform: &mut Transform, time: &Time) {
    // Nesting behavior - minimal movement, occasionally adjusting position
    let adjustment_frequency = 0.5; // Very slow adjustments
    let adjustment_amplitude = 1.0;
    let elapsed = time.elapsed().as_secs_f32();
    
    // Very subtle position adjustments (like settling into the nest)
    let x_adjust = (elapsed * adjustment_frequency).sin() * adjustment_amplitude;
    let y_adjust = (elapsed * adjustment_frequency * 1.1).cos() * adjustment_amplitude;
    
    transform.translation.x += x_adjust * time.delta().as_secs_f32() * 0.1;
    transform.translation.y += y_adjust * time.delta().as_secs_f32() * 0.1;
}

pub fn execute_roosting(transform: &mut Transform, time: &Time) {
    // Roosting behavior - communal gathering with slight social adjustments
    let social_frequency = 0.3; // Gentle social positioning
    let social_amplitude = 2.0;
    let elapsed = time.elapsed().as_secs_f32();
    
    // Slight movements as birds settle in roosting spots
    // Birds may adjust position to maintain social distance or warmth
    let x_adjust = (elapsed * social_frequency).sin() * social_amplitude;
    let y_adjust = (elapsed * social_frequency * 0.8).cos() * social_amplitude;
    
    transform.translation.x += x_adjust * time.delta().as_secs_f32() * 0.05;
    transform.translation.y += y_adjust * time.delta().as_secs_f32() * 0.05;
}

pub fn execute_sheltering(transform: &mut Transform, time: &Time) {
    // Sheltering behavior - staying put with minimal movement for safety
    let shelter_frequency = 0.2; // Very minimal movement
    let shelter_amplitude = 0.5;
    let elapsed = time.elapsed().as_secs_f32();
    
    // Very small adjustments - birds hunkering down against weather
    let x_adjust = (elapsed * shelter_frequency).sin() * shelter_amplitude;
    let y_adjust = (elapsed * shelter_frequency * 1.2).cos() * shelter_amplitude;
    
    transform.translation.x += x_adjust * time.delta().as_secs_f32() * 0.02;
    transform.translation.y += y_adjust * time.delta().as_secs_f32() * 0.02;
}
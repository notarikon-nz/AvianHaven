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

pub fn execute_courting(transform: &mut Transform, target_transform: &Transform, time: &Time) {
    // Courting behavior - elaborate display movements around the target
    let courtship_intensity = 50.0;
    let display_frequency = 4.0;
    let elapsed = time.elapsed().as_secs_f32();
    
    let current_pos = transform.translation.truncate();
    let target_pos = target_transform.translation.truncate();
    let distance = current_pos.distance(target_pos);
    
    // Maintain courting distance (close but not too close)
    let desired_distance = 60.0;
    
    if distance > desired_distance + 20.0 {
        // Move closer if too far
        let direction = (target_pos - current_pos).normalize_or_zero();
        transform.translation += (direction * courtship_intensity * time.delta().as_secs_f32()).extend(0.0);
    } else if distance < desired_distance - 20.0 {
        // Back away if too close
        let direction = (current_pos - target_pos).normalize_or_zero();
        transform.translation += (direction * courtship_intensity * 0.5 * time.delta().as_secs_f32()).extend(0.0);
    } else {
        // Perform elaborate display movements in a figure-8 pattern
        let display_center = target_pos + Vec2::new(0.0, 30.0);
        let figure8_x = (elapsed * display_frequency).sin() * 40.0;
        let figure8_y = (elapsed * display_frequency * 2.0).sin() * 20.0;
        let desired_pos = display_center + Vec2::new(figure8_x, figure8_y);
        
        let direction = (desired_pos - current_pos).normalize_or_zero();
        transform.translation += (direction * courtship_intensity * 0.8 * time.delta().as_secs_f32()).extend(0.0);
    }
}

pub fn execute_territorial(transform: &mut Transform, target_transform: &Transform, time: &Time) {
    // Territorial behavior - aggressive posturing and chase movements
    let aggression_speed = 90.0;
    let territorial_distance = 100.0;
    
    let current_pos = transform.translation.truncate();
    let target_pos = target_transform.translation.truncate();
    let distance = current_pos.distance(target_pos);
    
    if distance > territorial_distance {
        // Chase the intruder if they're too far
        let direction = (target_pos - current_pos).normalize_or_zero();
        transform.translation += (direction * aggression_speed * time.delta().as_secs_f32()).extend(0.0);
    } else {
        // Aggressive posturing - fast, jerky movements to intimidate
        let posture_frequency = 6.0;
        let posture_intensity = 15.0;
        let elapsed = time.elapsed().as_secs_f32();
        
        let posture_x = (elapsed * posture_frequency).sin() * posture_intensity;
        let posture_y = (elapsed * posture_frequency * 1.3).cos() * posture_intensity * 0.5;
        
        transform.translation.x += posture_x * time.delta().as_secs_f32();
        transform.translation.y += posture_y * time.delta().as_secs_f32();
    }
}

pub fn execute_flocking(transform: &mut Transform, target_transform: &Transform, time: &Time) {
    // Flocking behavior - maintain position relative to flock leader/member
    let flock_speed = 60.0;
    let flock_distance = 80.0;
    
    let current_pos = transform.translation.truncate();
    let target_pos = target_transform.translation.truncate();
    let distance = current_pos.distance(target_pos);
    
    if distance > flock_distance + 30.0 {
        // Move closer to maintain flock cohesion
        let direction = (target_pos - current_pos).normalize_or_zero();
        transform.translation += (direction * flock_speed * time.delta().as_secs_f32()).extend(0.0);
    } else if distance < flock_distance - 30.0 {
        // Move away to avoid overcrowding
        let direction = (current_pos - target_pos).normalize_or_zero();
        transform.translation += (direction * flock_speed * 0.5 * time.delta().as_secs_f32()).extend(0.0);
    } else {
        // Gentle adjustments to maintain formation
        let formation_frequency = 0.8;
        let formation_amplitude = 3.0;
        let elapsed = time.elapsed().as_secs_f32();
        
        let formation_x = (elapsed * formation_frequency).sin() * formation_amplitude;
        let formation_y = (elapsed * formation_frequency * 0.7).cos() * formation_amplitude;
        
        transform.translation.x += formation_x * time.delta().as_secs_f32();
        transform.translation.y += formation_y * time.delta().as_secs_f32();
    }
}

pub fn execute_following(transform: &mut Transform, target_transform: &Transform, time: &Time) {
    // Following behavior - stay behind and slightly to the side of target
    let follow_speed = 55.0;
    let follow_distance = 70.0;
    
    let current_pos = transform.translation.truncate();
    let target_pos = target_transform.translation.truncate();
    
    // Calculate desired position (behind and slightly offset from target)
    let offset_angle: f32 = 2.5; // Slightly behind and to the side
    let desired_pos = target_pos + Vec2::new(
        offset_angle.cos() * follow_distance,
        offset_angle.sin() * follow_distance
    );
    
    let direction = (desired_pos - current_pos).normalize_or_zero();
    transform.translation += (direction * follow_speed * time.delta().as_secs_f32()).extend(0.0);
}

pub fn execute_foraging(
    transform: &mut Transform, 
    foraging_traits: &crate::bird_ai::components::ForagingTraits,
    foraging_state: &mut crate::bird_ai::components::ForagingState,
    time: &Time,
    rng: &mut impl rand::Rng,
) {
    use crate::bird_ai::components::{ForagingStyle, SearchPattern};
    
    let foraging_speed = match foraging_traits.foraging_style {
        ForagingStyle::Scatter => 40.0,      // Fast, erratic movement
        ForagingStyle::Methodical => 20.0,   // Slow, deliberate
        ForagingStyle::Specialist => 35.0,   // Focused movement
        ForagingStyle::Opportunistic => 30.0, // Moderate speed
    };
    
    // Initialize search center if needed
    if foraging_state.search_center.length() < 0.1 {
        foraging_state.search_center = transform.translation.truncate();
        foraging_state.search_radius = rng.random_range(50.0..150.0);
        foraging_state.search_progress = 0.0;
    }
    
    let current_pos = transform.translation.truncate();
    let center = foraging_state.search_center;
    let radius = foraging_state.search_radius;
    
    // Calculate movement based on search pattern
    let movement_direction = match foraging_traits.search_pattern {
        SearchPattern::Random => {
            // Random walk within search area
            let random_angle = rng.random_range(0.0..std::f32::consts::TAU);
            Vec2::new(random_angle.cos(), random_angle.sin())
        },
        
        SearchPattern::Grid => {
            // Grid-like systematic search
            let grid_size = 8.0;
            let progress_step = 0.1 * time.delta().as_secs_f32();
            foraging_state.search_progress += progress_step;
            
            let grid_x = ((foraging_state.search_progress * grid_size) % grid_size).floor();
            let grid_y = (foraging_state.search_progress * grid_size / grid_size).floor();
            let target_pos = center + Vec2::new(
                (grid_x - grid_size/2.0) * radius / grid_size,
                (grid_y - grid_size/2.0) * radius / grid_size
            );
            (target_pos - current_pos).normalize_or_zero()
        },
        
        SearchPattern::Spiral => {
            // Spiral outward from center
            let angle = foraging_state.search_progress * 4.0;
            let spiral_radius = (foraging_state.search_progress * radius).min(radius);
            let target_pos = center + Vec2::new(
                angle.cos() * spiral_radius,
                angle.sin() * spiral_radius
            );
            
            foraging_state.search_progress += 0.2 * time.delta().as_secs_f32();
            (target_pos - current_pos).normalize_or_zero()
        },
        
        SearchPattern::Linear => {
            // Back and forth linear search
            let line_progress = foraging_state.search_progress % 2.0;
            let direction_multiplier = if line_progress < 1.0 { 1.0 } else { -1.0 };
            let target_pos = center + Vec2::new(
                direction_multiplier * radius,
                rng.random_range(-radius/4.0..radius/4.0) // Some vertical variation
            );
            
            foraging_state.search_progress += 0.3 * time.delta().as_secs_f32();
            (target_pos - current_pos).normalize_or_zero()
        }
    };
    
    // Apply movement with some randomness
    let random_factor = rng.random_range(0.8..1.2);
    transform.translation += (movement_direction * foraging_speed * random_factor * time.delta().as_secs_f32()).extend(0.0);
    
    // Reset search area if moved too far
    if current_pos.distance(center) > radius * 1.5 {
        foraging_state.search_center = current_pos;
        foraging_state.search_progress = 0.0;
    }
}

pub fn execute_caching(transform: &mut Transform, time: &Time) {
    // Caching behavior - small movements while finding a good hiding spot
    let cache_frequency = 0.8;
    let cache_amplitude = 5.0;
    let elapsed = time.elapsed().as_secs_f32();
    
    // Small searching movements to find the perfect cache spot
    let x_movement = (elapsed * cache_frequency).sin() * cache_amplitude;
    let y_movement = (elapsed * cache_frequency * 1.3).cos() * cache_amplitude;
    
    transform.translation.x += x_movement * time.delta().as_secs_f32() * 0.1;
    transform.translation.y += y_movement * time.delta().as_secs_f32() * 0.1;
}

pub fn execute_retrieving(transform: &mut Transform, time: &Time) {
    // Retrieving behavior - focused movement toward cache location
    let retrieval_frequency = 1.2;
    let retrieval_amplitude = 3.0;
    let elapsed = time.elapsed().as_secs_f32();
    
    // Quick, focused movements while searching for cached food
    let x_search = (elapsed * retrieval_frequency).sin() * retrieval_amplitude;
    let y_search = (elapsed * retrieval_frequency * 0.9).cos() * retrieval_amplitude;
    
    transform.translation.x += x_search * time.delta().as_secs_f32() * 0.2;
    transform.translation.y += y_search * time.delta().as_secs_f32() * 0.2;
}

pub fn execute_hover_feeding(transform: &mut Transform, time: &Time) {
    // Hover feeding behavior - rapid micro-movements to maintain position
    let hover_frequency = 8.0; // High frequency for rapid stabilization
    let hover_amplitude = 1.5;  // Small amplitude for precise hovering
    let elapsed = time.elapsed().as_secs_f32();
    
    // Rapid micro-corrections to maintain hovering position
    let stabilization_x = (elapsed * hover_frequency).sin() * hover_amplitude;
    let stabilization_y = (elapsed * hover_frequency * 1.1).cos() * hover_amplitude;
    
    transform.translation.x += stabilization_x * time.delta().as_secs_f32() * 0.5;
    transform.translation.y += stabilization_y * time.delta().as_secs_f32() * 0.5;
    
    // Slight upward bias to maintain hovering altitude
    transform.translation.y += 2.0 * time.delta().as_secs_f32();
}
use bevy::prelude::*;
use super::{components::*, resources::*};
use crate::environment::{resources::TimeState, components::Season};
use crate::bird_ai::components::{BirdAI, BirdState};
use crate::animation::components::AnimatedBird;

pub fn setup_advanced_photo_ui(mut commands: Commands) {
    // Composition grid overlay
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        CompositionGrid {
            rule_of_thirds: true,
            center_guides: false,
            golden_ratio: false,
        },
        PhotoModeUI,
        Visibility::Hidden,
    ));
    
    // Camera settings panel
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(250.0),
            height: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            row_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        BorderColor(Color::WHITE),
        CameraSettingsPanel,
        PhotoModeUI,
        Visibility::Hidden,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("Camera Settings"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // Zoom control
        parent.spawn((
            Text::new("Zoom: 1.0x"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // Aperture control  
        parent.spawn((
            Text::new("Aperture: f/5.6"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // Exposure control
        parent.spawn((
            Text::new("Exposure: 1/125s"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // ISO control
        parent.spawn((
            Text::new("ISO: 200"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        // Controls help
        parent.spawn((
            Text::new("Controls:\nMouse Wheel: Zoom\nQ/E: Aperture\nW/S: Exposure\nA/D: ISO\nG: Toggle Grid\nC: Toggle Panel"),
            TextFont { font_size: 12.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));
    });
}

pub fn camera_controls_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut camera_query: Query<(&mut Transform, &mut Projection, &mut CameraControls), With<PhotoTarget>>,
    settings: Res<PhotoModeSettings>,
    time: Res<Time>,
) {
    if !settings.is_active {
        return;
    }
    
    let Ok((mut camera_transform, mut projection, mut controls)) = camera_query.single_mut() else {
        return;
    };
    
    // Handle zoom with mouse wheel
    for scroll in scroll_events.read() {
        let zoom_delta = -scroll.y * controls.zoom_speed * time.delta().as_secs_f32();
        controls.zoom_level = (controls.zoom_level + zoom_delta).clamp(controls.min_zoom, controls.max_zoom);
        
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scale = 1.0 / controls.zoom_level;
        }
    }
    
    // Aperture control (Q/E keys)
    if keyboard.pressed(KeyCode::KeyQ) {
        controls.aperture = (controls.aperture - 0.5 * time.delta().as_secs_f32()).max(1.4);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        controls.aperture = (controls.aperture + 0.5 * time.delta().as_secs_f32()).min(22.0);
    }
    
    // Exposure control (W/S keys)
    if keyboard.pressed(KeyCode::KeyW) {
        controls.exposure = (controls.exposure + 0.3 * time.delta().as_secs_f32()).min(2.0);
    }
    if keyboard.pressed(KeyCode::KeyS) {
        controls.exposure = (controls.exposure - 0.3 * time.delta().as_secs_f32()).max(-2.0);
    }
    
    // ISO control (A/D keys)
    if keyboard.pressed(KeyCode::KeyA) {
        controls.iso = (controls.iso - 100.0 * time.delta().as_secs_f32()).max(100.0);
    }
    if keyboard.pressed(KeyCode::KeyD) {
        controls.iso = (controls.iso + 100.0 * time.delta().as_secs_f32()).min(6400.0);
    }
}

pub fn composition_grid_system(
    mut grid_query: Query<&mut Visibility, (With<CompositionGrid>, With<PhotoModeUI>)>,
    settings: Res<PhotoModeSettings>,
) {
    for mut visibility in &mut grid_query {
        *visibility = if settings.is_active && settings.show_composition_grid {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub fn camera_settings_panel_system(
    mut panel_query: Query<&mut Visibility, (With<CameraSettingsPanel>, With<PhotoModeUI>)>,
    camera_query: Query<&CameraControls, With<PhotoTarget>>,
    mut text_query: Query<&mut Text>,
    settings: Res<PhotoModeSettings>,
) {
    // Show/hide panel
    for mut visibility in &mut panel_query {
        *visibility = if settings.is_active && settings.show_camera_settings {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    
    // Update camera settings display
    if let Ok(controls) = camera_query.single() {
        // This would update the text display with current camera values
        // Implementation would require tracking specific text entities
    }
}

pub fn photo_mode_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<PhotoModeSettings>,
) {
    if !settings.is_active {
        return;
    }
    
    // Toggle composition grid
    if keyboard.just_pressed(settings.grid_toggle_key) {
        settings.show_composition_grid = !settings.show_composition_grid;
    }
    
    // Toggle camera settings panel
    if keyboard.just_pressed(settings.settings_toggle_key) {
        settings.show_camera_settings = !settings.show_camera_settings;
    }
}

pub fn enhanced_photo_scoring_system(
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
    time_state: &TimeState,
    camera_controls: &CameraControls,
    closest_bird: Option<(Transform, AnimatedBird, BirdState)>,
) -> PhotoScore {
    let mut score = PhotoScore {
        species_score: 0,
        centering_score: 0,
        clarity_score: 0,
        behavior_score: 0,
        timing_score: 0,
        rarity_bonus: 0,
        composition_score: 0,
        lighting_score: 0,
        environment_score: 0,
        technical_score: 0,
        storytelling_score: 0,
        total_score: 0,
    };
    
    let Some((bird_transform, animated_bird, bird_state)) = closest_bird else {
        return score;
    };
    
    // Original scoring
    score.species_score = get_species_rarity_score(animated_bird.species);
    score.behavior_score = get_behavior_score(bird_state);
    score.timing_score = get_timing_score(bird_state, bird_query);
    
    // Enhanced composition scoring
    score.composition_score = analyze_composition(&bird_transform, bird_query);
    
    // Lighting analysis
    score.lighting_score = analyze_lighting_conditions(time_state, &bird_transform);
    
    // Environment scoring
    score.environment_score = analyze_environment_context(&bird_transform, bird_query);
    
    // Technical settings scoring
    score.technical_score = analyze_camera_settings(camera_controls, &bird_transform, time_state);
    
    // Storytelling elements
    score.storytelling_score = analyze_storytelling_elements(bird_state, bird_query, &bird_transform);
    
    // Enhanced rarity bonuses
    score.rarity_bonus = calculate_enhanced_rarity_bonus(bird_query, bird_state, time_state);
    
    // Centering and clarity (enhanced versions)
    score.centering_score = calculate_enhanced_centering(&bird_transform);
    score.clarity_score = calculate_enhanced_clarity(&bird_transform, camera_controls);
    
    score.total_score = score.species_score + score.centering_score + score.clarity_score + 
                       score.behavior_score + score.timing_score + score.rarity_bonus +
                       score.composition_score + score.lighting_score + score.environment_score +
                       score.technical_score + score.storytelling_score;
    
    score
}

fn analyze_composition(
    bird_transform: &Transform,
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
) -> u32 {
    let bird_pos = bird_transform.translation.truncate();
    let mut composition_score = 0;
    
    // Rule of thirds analysis
    let thirds_x = [800.0 / 3.0, 800.0 * 2.0 / 3.0];
    let thirds_y = [600.0 / 3.0, 600.0 * 2.0 / 3.0];
    
    // Check proximity to rule of thirds intersections
    for &x in &thirds_x {
        for &y in &thirds_y {
            let intersection = Vec2::new(x - 400.0, y - 300.0); // Center at origin
            let distance = bird_pos.distance(intersection);
            if distance < 50.0 {
                composition_score += (50.0 - distance) as u32 / 2; // Up to 25 points
            }
        }
    }
    
    // Leading lines bonus (multiple birds creating visual flow)
    let bird_count = bird_query.iter().count();
    if bird_count >= 2 {
        composition_score += 15; // Multiple subjects create visual interest
    }
    
    // Depth layers (birds at different distances)
    let mut z_distances: Vec<f32> = bird_query.iter()
        .map(|(transform, _, _)| transform.translation.z)
        .collect();
    z_distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    if z_distances.len() >= 2 {
        let depth_range = z_distances.last().unwrap() - z_distances.first().unwrap();
        if depth_range > 20.0 {
            composition_score += 20; // Good depth separation
        }
    }
    
    composition_score.min(60) // Cap at 60 points
}

fn analyze_lighting_conditions(time_state: &TimeState, bird_transform: &Transform) -> u32 {
    let mut lighting_score: u32 = 0;
    
    // Golden hour bonus (6-8 AM, 6-8 PM)
    if (time_state.hour >= 6.0 && time_state.hour <= 8.0) || 
       (time_state.hour >= 18.0 && time_state.hour <= 20.0) {
        lighting_score += 30; // Golden hour lighting
    }
    
    // Blue hour bonus (5-6 AM, 8-9 PM)
    if (time_state.hour >= 5.0 && time_state.hour < 6.0) || 
       (time_state.hour > 20.0 && time_state.hour <= 21.0) {
        lighting_score += 25; // Blue hour drama
    }
    
    // Avoid harsh midday lighting penalty
    if time_state.hour >= 11.0 && time_state.hour <= 14.0 {
        lighting_score = lighting_score.saturating_sub(10);
    }
    
    // Seasonal lighting bonuses
    let season = time_state.get_season();
    match season {
        Season::Fall => lighting_score += 10, // Beautiful fall lighting
        Season::Winter => {
            if time_state.hour >= 7.0 && time_state.hour <= 16.0 {
                lighting_score += 15; // Short winter days are precious
            }
        },
        Season::Spring => lighting_score += 5, // Fresh spring light
        _ => {}
    }
    
    lighting_score.min(50) // Cap at 50 points
}

fn analyze_environment_context(
    bird_transform: &Transform,
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
) -> u32 {
    let mut env_score = 0;
    
    // Multiple species bonus
    let mut species_set = std::collections::HashSet::new();
    for (_, animated_bird, _) in bird_query.iter() {
        species_set.insert(animated_bird.species);
    }
    
    match species_set.len() {
        1 => env_score += 5,   // Single species
        2 => env_score += 15,  // Two species interaction
        3 => env_score += 25,  // Multi-species gathering
        4.. => env_score += 35, // Rare congregation
        _ => {}
    }
    
    // Environmental storytelling (birds in natural poses)
    let bird_count = bird_query.iter().count();
    if bird_count >= 2 {
        // Check for natural grouping patterns
        let birds_close_together = bird_query.iter()
            .filter(|(transform, _, _)| {
                bird_transform.translation.distance(transform.translation) < 100.0
            })
            .count();
            
        if birds_close_together >= 2 {
            env_score += 20; // Natural flocking behavior captured
        }
    }
    
    env_score.min(40) // Cap at 40 points
}

fn analyze_camera_settings(
    camera_controls: &CameraControls,
    bird_transform: &Transform,
    time_state: &TimeState,
) -> u32 {
    let mut tech_score = 0;
    
    // Optimal aperture for bird photography (f/5.6 to f/8.0 is ideal)
    let aperture_optimality = if camera_controls.aperture >= 5.6 && camera_controls.aperture <= 8.0 {
        1.0
    } else {
        1.0 - ((camera_controls.aperture - 6.8).abs() / 10.0).min(0.8)
    };
    tech_score += (aperture_optimality * 20.0) as u32;
    
    // ISO optimization (lower is better, but needs to match lighting)
    let lighting_factor = time_state.daylight_factor();
    let optimal_iso = if lighting_factor > 0.8 {
        400.0 // Bright conditions
    } else if lighting_factor > 0.5 {
        800.0 // Medium light
    } else {
        1600.0 // Low light
    };
    
    let iso_optimality = 1.0 - ((camera_controls.iso - optimal_iso).abs() / 2000.0).min(0.8);
    tech_score += (iso_optimality * 15.0) as u32;
    
    // Zoom appropriateness (not too close, not too far)
    let zoom_optimality = if camera_controls.zoom_level >= 1.5 && camera_controls.zoom_level <= 3.0 {
        1.0 // Ideal bird photography range
    } else {
        1.0 - ((camera_controls.zoom_level - 2.25).abs() / 3.0).min(0.7)
    };
    tech_score += (zoom_optimality * 15.0) as u32;
    
    tech_score.min(50) // Cap at 50 points
}

fn analyze_storytelling_elements(
    bird_state: BirdState,
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
    bird_transform: &Transform,
) -> u32 {
    let mut story_score = 0;
    
    // Behavioral context scoring
    match bird_state {
        BirdState::Eating => {
            // Check if multiple birds are feeding together
            let feeding_birds = bird_query.iter()
                .filter(|(_, _, state)| matches!(state, BirdState::Eating))
                .count();
            if feeding_birds > 1 {
                story_score += 25; // Social feeding story
            } else {
                story_score += 15; // Solo feeding moment
            }
        },
        BirdState::Foraging => {
            // Ground foraging shows natural behavior
            let foraging_birds = bird_query.iter()
                .filter(|(_, _, state)| matches!(state, BirdState::Foraging))
                .count();
            if foraging_birds > 1 {
                story_score += 30; // Multiple birds foraging together
            } else {
                story_score += 20; // Solo natural foraging
            }
        },
        BirdState::Caching => {
            story_score += 40; // Rare intelligent caching behavior
        },
        BirdState::Retrieving => {
            story_score += 35; // Smart cache retrieval behavior
        },
        BirdState::HoverFeeding => {
            story_score += 45; // Spectacular hovering behavior
        },
        BirdState::Bathing => {
            story_score += 35; // Rare and interesting behavior
        },
        BirdState::Fleeing => {
            // Drama and action
            story_score += 30;
        },
        BirdState::MovingToTarget => {
            // Bird with purpose/intention
            story_score += 20;
        },
        _ => story_score += 10,
    }
    
    // Interaction stories (birds near each other with different behaviors)
    let nearby_birds = bird_query.iter()
        .filter(|(transform, _, _)| {
            bird_transform.translation.distance(transform.translation) < 80.0
        })
        .collect::<Vec<_>>();
        
    if nearby_birds.len() >= 2 {
        let different_behaviors = nearby_birds.iter()
            .map(|(_, _, state)| *state)
            .collect::<std::collections::HashSet<_>>()
            .len();
            
        if different_behaviors > 1 {
            story_score += 20; // Multiple behaviors create story
        }
    }
    
    story_score.min(40) // Cap at 40 points
}

fn calculate_enhanced_rarity_bonus(
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
    bird_state: BirdState,
    time_state: &TimeState,
) -> u32 {
    let mut bonus = 0;
    
    // Multi-bird shots
    let bird_count = bird_query.iter().count();
    match bird_count {
        2 => bonus += 10,
        3 => bonus += 25,
        4 => bonus += 45,
        5.. => bonus += 70,
        _ => {}
    }
    
    // Time-of-day rarity
    if time_state.hour < 6.0 || time_state.hour > 21.0 {
        bonus += 15; // Night photography is rare
    }
    
    // Behavior rarity
    match bird_state {
        BirdState::Bathing => bonus += 25,      // Rare behavior
        BirdState::Fleeing => bonus += 20,      // Action shots are harder
        BirdState::Caching => bonus += 30,      // Very rare intelligent behavior
        BirdState::HoverFeeding => bonus += 35, // Spectacular rare behavior
        BirdState::Retrieving => bonus += 25,   // Smart cache retrieval
        BirdState::Foraging => bonus += 10,     // Natural but noteworthy
        _ => {}
    }
    
    // Seasonal bonuses
    let season = time_state.get_season();
    match season {
        Season::Winter => bonus += 10, // Harder to photograph in winter
        Season::Fall => bonus += 5,   // Migration season
        _ => {}
    }
    
    bonus.min(80) // Cap at 80 points
}

fn calculate_enhanced_centering(bird_transform: &Transform) -> u32 {
    let bird_pos = bird_transform.translation.truncate();
    
    // Advanced centering that considers rule of thirds as well as center
    let center_distance = bird_pos.length();
    let center_score = ((150.0 - center_distance.min(150.0)) / 150.0 * 20.0) as u32;
    
    // Rule of thirds bonus
    let thirds_positions = [
        Vec2::new(-133.0, -100.0), Vec2::new(133.0, -100.0),
        Vec2::new(-133.0, 100.0), Vec2::new(133.0, 100.0),
    ];
    
    let mut thirds_bonus = 0;
    for thirds_pos in thirds_positions {
        let distance = bird_pos.distance(thirds_pos);
        if distance < 50.0 {
            thirds_bonus = ((50.0 - distance) / 50.0 * 15.0) as u32;
            break;
        }
    }
    
    (center_score + thirds_bonus).min(35) // Cap at 35 points
}

fn calculate_enhanced_clarity(bird_transform: &Transform, camera_controls: &CameraControls) -> u32 {
    let bird_z = bird_transform.translation.z;
    let focus_difference = (bird_z - camera_controls.focus_distance).abs();
    
    // Depth of field calculation based on aperture
    let dof_range = camera_controls.aperture * 2.0; // Wider aperture = shallower DOF
    
    let clarity = if focus_difference < dof_range {
        1.0 - (focus_difference / dof_range) * 0.3 // In focus
    } else {
        0.7 - ((focus_difference - dof_range) / 100.0).min(0.6) // Out of focus penalty
    };
    
    (clarity * 25.0) as u32
}

// Helper functions for new scoring components
fn get_behavior_score(bird_state: BirdState) -> u32 {
    match bird_state {
        BirdState::Eating => 50,
        BirdState::Drinking => 45,
        BirdState::Bathing => 60,
        BirdState::Playing => 65,     // Rare and very cute behavior
        BirdState::Nesting => 70,     // Rare nesting behavior
        BirdState::Exploring => 40,   // Interesting investigative behavior
        BirdState::Roosting => 55,    // Evening gathering behavior
        BirdState::Sheltering => 50,  // Weather response behavior
        BirdState::Courting => 75,    // Spectacular courtship display
        BirdState::Following => 35,   // Social following behavior
        BirdState::Territorial => 55, // Aggressive territorial display
        BirdState::Flocking => 40,    // Mixed species flocking behavior
        BirdState::Foraging => 45,    // Natural ground foraging behavior
        BirdState::Caching => 65,     // Rare seed caching behavior
        BirdState::Retrieving => 55,  // Intelligent cache retrieval behavior
        BirdState::HoverFeeding => 70, // Spectacular hovering nectar feeding
        BirdState::Fleeing => 30,
        BirdState::Resting => 25,
        BirdState::MovingToTarget => 20,
        BirdState::Wandering => 15,
    }
}

fn get_timing_score(
    bird_state: BirdState,
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
) -> u32 {
    match bird_state {
        BirdState::Eating | BirdState::Drinking => {
            if bird_query.iter().count() > 1 { 25 } else { 15 }
        },
        BirdState::Bathing => 40,
        _ => 10,
    }
}

fn get_species_rarity_score(species: crate::bird::BirdSpecies) -> u32 {
    use crate::bird::BirdSpecies as BS;
    match species {
        // Tier 1 - Common (10-25 points)
        BS::Sparrow | BS::Robin | BS::Chickadee | BS::HouseFinch | BS::EuropeanStarling => 15,
        BS::Cardinal | BS::BlueJay | BS::Goldfinch | BS::MourningDove => 20,
        BS::NorthernMockingbird | BS::RedWingedBlackbird | BS::CommonGrackle | BS::CommonCrow => 18,
        BS::BrownThrasher | BS::CedarWaxwing | BS::WhiteBreastedNuthatch | BS::TuftedTitmouse => 30,
        BS::CarolinaWren | BS::BlueGrayGnatcatcher | BS::YellowWarbler => 35,
        
        // Tier 2 - Uncommon (40-60 points)
        BS::DownyWoodpecker | BS::HairyWoodpecker | BS::BrownCreeper | BS::WinterWren => 40,
        BS::RedHeadedWoodpecker | BS::YellowBelledSapsucker | BS::PurpleFinch | BS::IndianaBunting => 45,
        BS::PileatedWoodpecker | BS::RubyThroatedHummingbird | BS::RoseBreastedGrosbeak => 50,
        BS::WoodThrush | BS::Catbird | BS::ScarletTanager | BS::BaltimoreOriole => 55,
        
        // Tier 3 - Rare (70-90 points)
        BS::EasternBluebird | BS::PaintedBunting | BS::CeruleanWarbler | BS::HoodedWarbler => 70,
        BS::BelttedKingfisher | BS::GrandSlamAmerican => 75,
        BS::RedTailedHawk | BS::CoopersHawk | BS::BarredOwl => 80,
        BS::GreatHornedOwl => 90,
        
        // Tier 4 - Legendary (100-150 points)
        BS::ProthonotaryWarbler | BS::KentuckyWarbler | BS::GoldenWingedWarbler => 100,
        BS::PeregrineFalcon => 120,
        BS::BaldEagle => 150,
    }
}
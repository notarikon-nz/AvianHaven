use bevy::prelude::*;
use bevy::render::camera::{RenderTarget,ImageRenderTarget};
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use crate::photo_mode::{components::*, resources::*};
use crate::bird::BirdSpecies;
use crate::animation::components::AnimatedBird;
use crate::bird_ai::components::{BirdAI, BirdState};

pub fn setup_photo_ui(mut commands: Commands) {
    // Viewfinder UI - initially hidden
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ViewfinderUI,
        Visibility::Hidden,
    )).with_children(|parent| {
        // Viewfinder frame
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(300.0),
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::NONE),
        )).with_children(|parent| {
            // Center reticle
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Px(-10.0),
                        top: Val::Px(-10.0),
                        ..default()
                    },
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::srgb(1.0,0.0,0.0)),
                BackgroundColor(Color::NONE),
            ));
        });
    });

    // Score toast - initially hidden
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            right: Val::Px(50.0),
            width: Val::Px(300.0),
            height: Val::Px(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        BorderColor(Color::WHITE),
        ScoreToast,
        Visibility::Hidden,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Photo Saved!"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn toggle_photo_mode_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<PhotoModeSettings>,
    mut viewfinder_query: Query<&mut Visibility, With<ViewfinderUI>>,
) {
    if keyboard.just_pressed(settings.toggle_key) {
        settings.is_active = !settings.is_active;
        
        for mut visibility in &mut viewfinder_query {
            *visibility = if settings.is_active {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        
        info!("Photo mode {}", if settings.is_active { "activated" } else { "deactivated" });
    }
}

pub fn capture_photo_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<PhotoModeSettings>,
    mut camera_query: Query<&mut Camera, With<PhotoTarget>>,
    bird_query: Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
    mut photo_events: EventWriter<PhotoTakenEvent>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    if !settings.is_active || !keyboard.just_pressed(settings.capture_key) {
        return;
    }

    let Ok(mut camera) = camera_query.single_mut() else {
        warn!("No photo target camera found");
        return;
    };

    // Create render texture for screenshot
    let size = Extent3d {
        width: 800,
        height: 600,
        depth_or_array_layers: 1,
    };

    let mut render_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        ..default()
    };
    render_texture.resize(size);

    let image_handle = images.add(render_texture);
    
    // Temporarily set camera to render to our texture
    let original_target = camera.target.clone();
    camera.target = RenderTarget::Image(ImageRenderTarget::from(image_handle.clone()));
    
    // Schedule restoration of camera target for next frame
    commands.queue(move |world: &mut World| {
        let mut camera_query = world.query::<&mut Camera>();
        if let Ok(mut camera) = camera_query.single_mut(world) {
            camera.target = original_target;
        }
    });

    // Find closest bird for subject analysis
    let camera_pos = Vec2::ZERO; // Camera center for analysis
    let closest_bird = find_closest_bird_to_center(&bird_query, camera_pos);
    
    // Calculate photo score
    let score = calculate_photo_score(&bird_query, closest_bird);
    
    // Log score breakdown
    info!("Photo Score Breakdown:");
    info!("  Species: {}", score.species_score);
    info!("  Behavior: {}", score.behavior_score);
    info!("  Timing: {}", score.timing_score);
    info!("  Centering: {}", score.centering_score);
    info!("  Clarity: {}", score.clarity_score);
    info!("  Rarity Bonus: {}", score.rarity_bonus);
    info!("  Total: {}", score.total_score);
    
    photo_events.write(PhotoTakenEvent {
        score,
        species: closest_bird.map(|(_, bird, _)| bird.species),
        image_handle,
    });
}

pub fn photo_reward_system(
    mut photo_events: EventReader<PhotoTakenEvent>,
    mut currency: ResMut<CurrencyResource>,
    mut discovered_species: ResMut<DiscoveredSpecies>,
    mut photo_collection: ResMut<PhotoCollection>,
    mut toast_query: Query<(&mut Visibility, &Children), With<ScoreToast>>,
    mut text_query: Query<&mut Text>,
    time: Res<Time>,
) {
    for event in photo_events.read() {
        // Grant currency based on total score
        currency.0 += event.score.total_score;
        
        let mut bonus_text = String::new();
        
        // Check for new species discovery
        if let Some(species) = event.species {
            if discovered_species.discover(species) {
                bonus_text = format!(" New Species Bonus!");
                currency.0 += 50; // Bonus for first discovery
            }
        }
        
        // Additional bonuses for exceptional photos
        if event.score.behavior_score >= 50 {
            bonus_text.push_str(" Action Shot!");
        }
        if event.score.rarity_bonus > 0 {
            bonus_text.push_str(" Multi-Bird!");
        }
        
        // Show toast
        for (mut visibility, children) in &mut toast_query {
            *visibility = Visibility::Inherited;
            
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = format!("Photo Saved! +{} Points!{}", event.score.total_score, bonus_text);
                }
            }
        }
        
        // Save photo to collection
        photo_collection.add_photo(SavedPhoto {
            species: event.species,
            score: event.score.clone(),
            image_handle: event.image_handle.clone(),
            timestamp: time.elapsed().as_secs_f64(),
        });
        
        info!("Currency awarded: {} (Total: {})", event.score.total_score, currency.0);
        info!("Photo saved to collection (Total photos: {})", photo_collection.photos.len());
    }
}

pub fn photo_ui_system(
    time: Res<Time>,
    mut toast_query: Query<&mut Visibility, With<ScoreToast>>,
) {
    // Simple toast auto-hide after 3 seconds (in real implementation, would use a timer)
    static mut TOAST_TIMER: f32 = 0.0;
    
    for mut visibility in &mut toast_query {
        if *visibility == Visibility::Inherited {
            unsafe {
                TOAST_TIMER += time.delta().as_secs_f32();
                if TOAST_TIMER > 3.0 {
                    *visibility = Visibility::Hidden;
                    TOAST_TIMER = 0.0;
                }
            }
        }
    }
}

// Helper functions for photo scoring

fn find_closest_bird_to_center(
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
    camera_pos: Vec2,
) -> Option<(Transform, AnimatedBird, BirdState)> {
    let mut closest_bird = None;
    let mut closest_distance = f32::MAX;
    
    for (transform, animated_bird, bird_state) in bird_query {
        let distance = camera_pos.distance(transform.translation.truncate());
        if distance < closest_distance {
            closest_distance = distance;
            closest_bird = Some((*transform, *animated_bird, *bird_state));
        }
    }
    
    closest_bird
}

fn calculate_photo_score(
    bird_query: &Query<(&Transform, &AnimatedBird, &BirdState), With<BirdAI>>,
    closest_bird: Option<(Transform, AnimatedBird, BirdState)>,
) -> PhotoScore {
    let mut score = PhotoScore {
        species_score: 0,
        centering_score: 0,
        clarity_score: 0,
        behavior_score: 0,
        timing_score: 0,
        rarity_bonus: 0,
        total_score: 0,
    };
    
    let Some((bird_transform, animated_bird, bird_state)) = closest_bird else {
        return score; // No birds in shot
    };
    
    // Species scoring - based on rarity tiers from design doc
    score.species_score = get_species_rarity_score(animated_bird.species);
    
    // Behavior scoring - interesting behaviors worth more points
    score.behavior_score = match bird_state {
        BirdState::Eating => 50,     // Very photogenic
        BirdState::Drinking => 45,   // Also very photogenic
        BirdState::Bathing => 60,    // Rare and exciting behavior
        BirdState::Fleeing => 30,    // Action shot bonus
        BirdState::Resting => 25,    // Peaceful moment
        BirdState::MovingToTarget => 20, // Bird in motion
        BirdState::Wandering => 15,  // Standard pose
    };
    
    // Timing scoring - photogenic moments get bonuses
    score.timing_score = match bird_state {
        BirdState::Eating | BirdState::Drinking => {
            // Bonus for catching bird in feeding action
            if bird_query.iter().count() > 1 { 25 } else { 15 } // More points with multiple birds
        },
        BirdState::Bathing => 40, // Always exciting to capture
        _ => 10,
    };
    
    // Rarity bonus for special circumstances
    if bird_query.iter().count() >= 3 {
        score.rarity_bonus += 20; // Multiple birds in frame
    }
    
    // Centering scoring - closer to center = higher score
    let bird_pos = bird_transform.translation.truncate();
    let distance_from_center = bird_pos.length(); // Distance from origin (camera center)
    score.centering_score = ((200.0 - distance_from_center.min(200.0)) / 200.0 * 30.0) as u32;
    
    // Clarity scoring - closer birds are "clearer" (based on Z distance from camera)
    let bird_z = bird_transform.translation.z;
    let z_distance = bird_z.abs(); // Distance from camera Z plane
    score.clarity_score = ((100.0 - z_distance.min(100.0)) / 100.0 * 20.0) as u32;
    
    score.total_score = score.species_score + score.centering_score + score.clarity_score + 
                       score.behavior_score + score.timing_score + score.rarity_bonus;
    score
}

fn get_species_rarity_score(species: BirdSpecies) -> u32 {
    match_bird_species_score(species)
}

fn match_bird_species_score(species: crate::bird::BirdSpecies) -> u32 {
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
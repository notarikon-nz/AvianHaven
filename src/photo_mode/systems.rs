use bevy::prelude::*;
use bevy::render::camera::{RenderTarget,ImageRenderTarget};
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use crate::photo_mode::{components::*, resources::*};
use crate::animation::components::{BirdSpecies, AnimatedBird};
use crate::bird_ai::components::BirdAI;

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
    bird_query: Query<(&Transform, &AnimatedBird), With<BirdAI>>,
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
    info!("  Centering: {}", score.centering_score);
    info!("  Clarity: {}", score.clarity_score);
    info!("  Total: {}", score.total_score);
    
    photo_events.write(PhotoTakenEvent {
        score: score.total_score,
        species: closest_bird.map(|(_, bird)| bird.species),
        image_handle,
    });
}

pub fn photo_reward_system(
    mut photo_events: EventReader<PhotoTakenEvent>,
    mut currency: ResMut<CurrencyResource>,
    mut discovered_species: ResMut<DiscoveredSpecies>,
    mut toast_query: Query<(&mut Visibility, &Children), With<ScoreToast>>,
    mut text_query: Query<&mut Text>,
) {
    for event in photo_events.read() {
        // Grant currency
        currency.0 += event.score;
        
        let mut bonus_text = String::new();
        
        // Check for new species discovery
        if let Some(species) = event.species {
            if discovered_species.discover(species) {
                bonus_text = format!(" New Species Bonus!");
                currency.0 += 50; // Bonus for first discovery
            }
        }
        
        // Show toast
        for (mut visibility, children) in &mut toast_query {
            *visibility = Visibility::Inherited;
            
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = format!("Photo Saved! +{} Points!{}", event.score, bonus_text);
                }
            }
        }
        
        info!("Currency awarded: {} (Total: {})", event.score, currency.0);
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
    bird_query: &Query<(&Transform, &AnimatedBird), With<BirdAI>>,
    camera_pos: Vec2,
) -> Option<(Transform, AnimatedBird)> {
    let mut closest_bird = None;
    let mut closest_distance = f32::MAX;
    
    for (transform, animated_bird) in bird_query {
        let distance = camera_pos.distance(transform.translation.truncate());
        if distance < closest_distance {
            closest_distance = distance;
            closest_bird = Some((*transform, *animated_bird));
        }
    }
    
    closest_bird
}

fn calculate_photo_score(
    bird_query: &Query<(&Transform, &AnimatedBird), With<BirdAI>>,
    closest_bird: Option<(Transform, AnimatedBird)>,
) -> PhotoScore {
    let mut score = PhotoScore {
        species_score: 0,
        centering_score: 0,
        clarity_score: 0,
        total_score: 0,
    };
    
    let Some((bird_transform, animated_bird)) = closest_bird else {
        return score; // No birds in shot
    };
    
    // Species scoring - rarer species worth more
    score.species_score = match animated_bird.species {
        BirdSpecies::Sparrow => 10,     // Common
        BirdSpecies::Cardinal => 20,    // Uncommon  
        BirdSpecies::BlueJay => 35,     // Rare
    };
    
    // Centering scoring - closer to center = higher score
    let bird_pos = bird_transform.translation.truncate();
    let distance_from_center = bird_pos.length(); // Distance from origin (camera center)
    score.centering_score = ((200.0 - distance_from_center.min(200.0)) / 200.0 * 30.0) as u32;
    
    // Clarity scoring - closer birds are "clearer" (based on Z distance from camera)
    let bird_z = bird_transform.translation.z;
    let z_distance = bird_z.abs(); // Distance from camera Z plane
    score.clarity_score = ((100.0 - z_distance.min(100.0)) / 100.0 * 20.0) as u32;
    
    score.total_score = score.species_score + score.centering_score + score.clarity_score;
    score
}
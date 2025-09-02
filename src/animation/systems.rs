use bevy::prelude::*;
use crate::animation::{components::*, resources::*};
use crate::bird_ai::components::BirdState;
use crate::bird::{BirdSpecies, Velocity};

pub fn setup_animation_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut cache: ResMut<TextureAtlasCache>,
) {
    // Setup spritesheet-based animations for each species
    setup_species_spritesheet(BirdSpecies::Cardinal, &asset_server, &mut texture_atlas_layouts, &mut cache);
    setup_species_spritesheet(BirdSpecies::Sparrow, &asset_server, &mut texture_atlas_layouts, &mut cache);
    setup_species_spritesheet(BirdSpecies::BlueJay, &asset_server, &mut texture_atlas_layouts, &mut cache);
    setup_species_spritesheet(BirdSpecies::Robin, &asset_server, &mut texture_atlas_layouts, &mut cache);
    setup_species_spritesheet(BirdSpecies::Chickadee, &asset_server, &mut texture_atlas_layouts, &mut cache);
    setup_species_spritesheet(BirdSpecies::Goldfinch, &asset_server, &mut texture_atlas_layouts, &mut cache);
}

fn setup_species_spritesheet(
    species: BirdSpecies,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    cache: &mut TextureAtlasCache,
) {
    let species_name = species_filename(&species);
    let texture_handle = asset_server.load(&format!("birds/{}.png", species_name));
    
    // Spritesheet layout: 7 rows (one per state), 6 frames per row
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32), // Each frame is 32x32 pixels
        6,  // 6 frames per row
        7,  // 7 rows (one per state)
        None,
        None,
    );
    let atlas_handle = texture_atlas_layouts.add(layout);
    
    // Map each row to a bird state
    let state_rows = [
        (BirdState::Wandering, 0, 12.0),      // Row 0
        (BirdState::MovingToTarget, 1, 16.0), // Row 1
        (BirdState::Eating, 2, 8.0),          // Row 2
        (BirdState::Drinking, 3, 6.0),        // Row 3
        (BirdState::Resting, 4, 2.0),         // Row 4
        (BirdState::Fleeing, 5, 20.0),        // Row 5
        (BirdState::Bathing, 6, 10.0),        // Row 6
    ];
    
    for (state, row, fps) in state_rows {
        let start_frame = row * 6;
        let end_frame = start_frame + 5;
        
        cache.atlases.insert(
            (species, state),
            AnimationData {
                texture_atlas_handle: atlas_handle.clone(),
                texture_handle: texture_handle.clone(),
                frame_range: (start_frame, end_frame),
                fps,
            },
        );
    }
}

fn species_filename(species: &BirdSpecies) -> String {
    match species {
        BirdSpecies::Cardinal => "cardinal".to_string(),
        BirdSpecies::BlueJay => "bluejay".to_string(),
        BirdSpecies::Robin => "robin".to_string(),
        BirdSpecies::Sparrow => "sparrow".to_string(),
        BirdSpecies::Chickadee => "chickadee".to_string(),
        BirdSpecies::Goldfinch => "goldfinch".to_string(),
        BirdSpecies::NorthernMockingbird => "mockingbird".to_string(),
        BirdSpecies::RedWingedBlackbird => "redwinged_blackbird".to_string(),
        BirdSpecies::CommonGrackle => "common_grackle".to_string(),
        BirdSpecies::BrownThrasher => "brown_thrasher".to_string(),
        BirdSpecies::CedarWaxwing => "cedar_waxwing".to_string(),
        BirdSpecies::WhiteBreastedNuthatch => "whitebreasted_nuthatch".to_string(),
        BirdSpecies::TuftedTitmouse => "tufted_titmouse".to_string(),
        BirdSpecies::CarolinaWren => "carolina_wren".to_string(),
        BirdSpecies::HouseFinch => "house_finch".to_string(),
        BirdSpecies::EuropeanStarling => "european_starling".to_string(),
        BirdSpecies::MourningDove => "mourning_dove".to_string(),
        BirdSpecies::CommonCrow => "common_crow".to_string(),
        BirdSpecies::BlueGrayGnatcatcher => "bluegray_gnatcatcher".to_string(),
        BirdSpecies::YellowWarbler => "yellow_warbler".to_string(),
        // Add more species as needed
        _ => "placeholder".to_string(),
    }
}

pub fn animation_state_system(
    mut commands: Commands,
    mut bird_query: Query<
        (Entity, &AnimatedBird, &BirdState, &mut AnimationLibrary, &mut AnimationController),
        Changed<BirdState>,
    >,
    cache: Res<TextureAtlasCache>,
) {
    for (entity, animated_bird, bird_state, mut library, mut controller) in bird_query.iter_mut() {
        if let Some(animation_data) = cache.atlases.get(&(animated_bird.species, *bird_state)) {
            library.animations.insert(*bird_state, animation_data.clone());
            
            let frame_count = animation_data.frame_range.1 - animation_data.frame_range.0 + 1;
            controller.frames = frame_count;
            controller.current_frame = 0;
            controller.atlas_layout = animation_data.texture_atlas_handle.clone();
            controller.timer = Timer::from_seconds(1.0 / animation_data.fps, TimerMode::Repeating);
            
            commands.entity(entity).insert(AnimationStateChange);
        } else {
            // Fallback: Create a colored rectangle for this bird species when assets are missing
            let fallback_color = get_species_fallback_color(animated_bird.species);
            commands.entity(entity).insert(Sprite {
                color: fallback_color,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            });
        }
    }
}

pub fn sprite_flip_system(
    mut sprite_query: Query<(&Velocity, &mut Sprite), Without<AnimationStateChange>>,
) {
    for (velocity, mut sprite) in sprite_query.iter_mut() {
        if velocity.0.x > 0.1 {
            sprite.flip_x = false;
        } else if velocity.0.x < -0.1 {
            sprite.flip_x = true;
        }
    }
}

pub fn advance_animation_frames_system(
    mut animation_query: Query<(&mut AnimationController, &AnimationLibrary, &BirdState, &mut Sprite), Without<AnimationStateChange>>,
    time: Res<Time>,
) {
    for (mut controller, library, bird_state, mut sprite) in animation_query.iter_mut() {
        controller.timer.tick(time.delta());
        
        if controller.timer.just_finished() {
            controller.current_frame = (controller.current_frame + 1) % controller.frames;
            
            if let Some(animation_data) = library.animations.get(bird_state) {
                let atlas_index = animation_data.frame_range.0 + controller.current_frame;
                if let Some(texture_atlas) = &mut sprite.texture_atlas {
                    texture_atlas.index = atlas_index;
                }
            }
        }
    }
}

pub fn update_sprite_on_state_change_system(
    mut commands: Commands,
    mut change_query: Query<
        (Entity, &AnimationLibrary, &BirdState, &mut Sprite, &AnimationController),
        With<AnimationStateChange>,
    >,
) {
    for (entity, library, bird_state, mut sprite, controller) in change_query.iter_mut() {
        if let Some(animation_data) = library.animations.get(bird_state) {
            sprite.image = animation_data.texture_handle.clone();
            
            let atlas_index = animation_data.frame_range.0 + controller.current_frame;
            sprite.texture_atlas = Some(TextureAtlas {
                layout: animation_data.texture_atlas_handle.clone(),
                index: atlas_index,
            });
        }
        
        commands.entity(entity).remove::<AnimationStateChange>();
    }
}

/// Get a fallback color for a bird species when sprites are unavailable
fn get_species_fallback_color(species: BirdSpecies) -> Color {
    match species {
        BirdSpecies::Cardinal => Color::srgb(0.8, 0.2, 0.2),          // Red
        BirdSpecies::BlueJay => Color::srgb(0.2, 0.4, 0.8),          // Blue
        BirdSpecies::Robin => Color::srgb(0.8, 0.4, 0.1),            // Orange-brown
        BirdSpecies::Sparrow => Color::srgb(0.6, 0.5, 0.3),          // Brown
        BirdSpecies::Chickadee => Color::srgb(0.3, 0.3, 0.3),        // Gray
        BirdSpecies::Goldfinch => Color::srgb(0.9, 0.8, 0.2),        // Yellow
        BirdSpecies::NorthernMockingbird => Color::srgb(0.7, 0.7, 0.7),  // Light gray
        BirdSpecies::RedWingedBlackbird => Color::srgb(0.2, 0.2, 0.2),   // Black
        BirdSpecies::CommonGrackle => Color::srgb(0.1, 0.1, 0.3),    // Dark blue-black
        BirdSpecies::BrownThrasher => Color::srgb(0.6, 0.3, 0.2),    // Rust brown
        BirdSpecies::CedarWaxwing => Color::srgb(0.8, 0.6, 0.4),     // Tan
        BirdSpecies::WhiteBreastedNuthatch => Color::srgb(0.8, 0.8, 0.9), // Light blue-gray
        BirdSpecies::TuftedTitmouse => Color::srgb(0.6, 0.6, 0.7),   // Blue-gray
        BirdSpecies::CarolinaWren => Color::srgb(0.7, 0.4, 0.2),     // Reddish brown
        BirdSpecies::HouseFinch => Color::srgb(0.7, 0.3, 0.3),       // Rose red
        BirdSpecies::EuropeanStarling => Color::srgb(0.3, 0.3, 0.4), // Dark gray
        BirdSpecies::MourningDove => Color::srgb(0.7, 0.6, 0.5),     // Soft brown
        BirdSpecies::CommonCrow => Color::srgb(0.1, 0.1, 0.1),       // Black
        BirdSpecies::BlueGrayGnatcatcher => Color::srgb(0.5, 0.6, 0.7), // Blue-gray
        BirdSpecies::YellowWarbler => Color::srgb(0.9, 0.9, 0.3),    // Bright yellow
        BirdSpecies::DownyWoodpecker => Color::srgb(0.9, 0.9, 0.9),  // White
        _ => Color::srgb(0.5, 0.5, 0.5), // Default gray for any unspecified species
    }
}
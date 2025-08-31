use bevy::prelude::*;
use crate::animation::{components::*, resources::*};
use crate::bird_ai::components::BirdState;
use crate::bird::{BirdSpecies, Velocity};

pub fn setup_animation_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut cache: ResMut<TextureAtlasCache>,
) {
    // Cardinal animations
    setup_species_animations(
        BirdSpecies::Cardinal,
        &asset_server,
        &mut texture_atlas_layouts,
        &mut cache,
        &[
            (BirdState::Wandering, "cardinal_flying.png", (0, 5), 12.0),
            (BirdState::MovingToTarget, "cardinal_flying.png", (0, 5), 16.0),
            (BirdState::Eating, "cardinal_eating.png", (0, 3), 8.0),
            (BirdState::Drinking, "cardinal_drinking.png", (0, 3), 6.0),
            (BirdState::Resting, "cardinal_idle.png", (0, 1), 2.0),
            (BirdState::Fleeing, "cardinal_flying.png", (0, 5), 20.0),
            (BirdState::Bathing, "cardinal_bathing.png", (0, 4), 10.0),
        ],
    );

    // Sparrow animations
    setup_species_animations(
        BirdSpecies::Sparrow,
        &asset_server,
        &mut texture_atlas_layouts,
        &mut cache,
        &[
            (BirdState::Wandering, "sparrow_flying.png", (0, 4), 10.0),
            (BirdState::MovingToTarget, "sparrow_flying.png", (0, 4), 14.0),
            (BirdState::Eating, "sparrow_eating.png", (0, 2), 6.0),
            (BirdState::Drinking, "sparrow_drinking.png", (0, 2), 5.0),
            (BirdState::Resting, "sparrow_idle.png", (0, 0), 1.0),
            (BirdState::Fleeing, "sparrow_flying.png", (0, 4), 18.0),
            (BirdState::Bathing, "sparrow_bathing.png", (0, 3), 8.0),
        ],
    );

    // Blue Jay animations
    setup_species_animations(
        BirdSpecies::BlueJay,
        &asset_server,
        &mut texture_atlas_layouts,
        &mut cache,
        &[
            (BirdState::Wandering, "bluejay_flying.png", (0, 6), 14.0),
            (BirdState::MovingToTarget, "bluejay_flying.png", (0, 6), 18.0),
            (BirdState::Eating, "bluejay_eating.png", (0, 4), 9.0),
            (BirdState::Drinking, "bluejay_drinking.png", (0, 4), 7.0),
            (BirdState::Resting, "bluejay_idle.png", (0, 2), 3.0),
            (BirdState::Fleeing, "bluejay_flying.png", (0, 6), 22.0),
            (BirdState::Bathing, "bluejay_bathing.png", (0, 5), 12.0),
        ],
    );
}

fn setup_species_animations(
    species: BirdSpecies,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    cache: &mut TextureAtlasCache,
    animations: &[(BirdState, &str, (usize, usize), f32)],
) {
    for &(state, texture_path, frame_range, fps) in animations {
        let texture_handle = asset_server.load(texture_path);
        let frame_count = frame_range.1 - frame_range.0 + 1;
        
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),
            frame_count as u32,
            1,
            None,
            None,
        );
        let atlas_handle = texture_atlas_layouts.add(layout);
        
        cache.atlases.insert(
            (species, state),
            AnimationData {
                texture_atlas_handle: atlas_handle,
                texture_handle,
                frame_range,
                fps,
            },
        );
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
    mut animation_query: Query<&mut AnimationController, Without<AnimationStateChange>>,
    mut sprite_query: Query<&mut Sprite>,
    time: Res<Time>,
) {
    for mut controller in animation_query.iter_mut() {
        controller.timer.tick(time.delta());
        
        if controller.timer.just_finished() {
            controller.current_frame = (controller.current_frame + 1) % controller.frames;
            // Note: In a real implementation, we'd need to find the corresponding sprite
            // and update its texture atlas index. For now, this is a placeholder.
        }
    }
}

pub fn update_sprite_on_state_change_system(
    mut commands: Commands,
    mut change_query: Query<
        (Entity, &AnimationLibrary, &BirdState, &mut Sprite),
        With<AnimationStateChange>,
    >,
) {
    for (entity, library, bird_state, mut sprite) in change_query.iter_mut() {
        if let Some(animation_data) = library.animations.get(bird_state) {
            sprite.image = animation_data.texture_handle.clone();
        }
        
        commands.entity(entity).remove::<AnimationStateChange>();
    }
}
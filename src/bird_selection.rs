use bevy::prelude::*;
use crate::AppState;
use crate::bird::{Bird, BirdSpecies};
use crate::bird_ai::components::{BirdAI, BirdState, Blackboard};
use crate::animation::components::AnimatedBird;

pub struct BirdSelectionPlugin;

impl Plugin for BirdSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BirdSelection>()
            .init_resource::<SelectionSettings>()
            .add_systems(
                Update,
                (
                    bird_selection_system,
                    selection_highlight_system,
                    update_selection_ui,
                    cleanup_selection_ui,
                ).run_if(in_state(AppState::Playing))
            )
            .add_systems(OnEnter(AppState::Playing), setup_selection_ui);
    }
}

#[derive(Resource, Default)]
pub struct BirdSelection {
    pub selected_bird: Option<Entity>,
    pub last_selected_time: f64,
}

#[derive(Resource)]
pub struct SelectionSettings {
    pub selection_radius: f32,
    pub double_click_time: f64,
    pub show_info_card: bool,
}

impl Default for SelectionSettings {
    fn default() -> Self {
        Self {
            selection_radius: 50.0,
            double_click_time: 0.3,
            show_info_card: true,
        }
    }
}

#[derive(Component)]
pub struct SelectionHighlight {
    pub pulse_timer: f32,
    pub base_scale: f32,
}

#[derive(Component)]
pub struct BirdInfoCard;

#[derive(Component)]
pub struct BirdInfoText;

/// System to handle bird selection via mouse clicks
pub fn bird_selection_system(
    mut commands: Commands,
    mut selection: ResMut<BirdSelection>,
    settings: Res<SelectionSettings>,
    time: Res<Time>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &Transform), With<Camera2d>>,
    bird_query: Query<(Entity, &Transform, &Bird), With<BirdAI>>,
    // Remove any existing highlights
    highlight_query: Query<Entity, With<SelectionHighlight>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    
    let Some(cursor_position) = window.cursor_position() else { return };
    
    // Convert screen coordinates to world coordinates
    // For 2D, we can use a simple approach - just offset by camera position
    let camera_pos = camera_transform.translation.truncate();
    let window_size = Vec2::new(window.width(), window.height());
    let ndc = (cursor_position - window_size * 0.5) / window_size * 2.0;
    let world_position = camera_pos + Vec2::new(ndc.x, -ndc.y) * 400.0; // Scale factor for camera zoom

    // Find the closest bird within selection radius
    let mut closest_bird = None;
    let mut closest_distance = f32::MAX;

    for (entity, bird_transform, _bird) in &bird_query {
        let distance = world_position.distance(bird_transform.translation.truncate());
        if distance < settings.selection_radius && distance < closest_distance {
            closest_distance = distance;
            closest_bird = Some(entity);
        }
    }

    // Clear existing highlights
    for highlight_entity in &highlight_query {
        commands.entity(highlight_entity).despawn();
    }

    if let Some(bird_entity) = closest_bird {
        // Check for double-click to toggle info card
        let current_time = time.elapsed().as_secs_f64();
        if let Some(last_selected) = selection.selected_bird {
            if last_selected == bird_entity && 
               current_time - selection.last_selected_time < settings.double_click_time {
                // Double-click detected - toggle info card
                selection.selected_bird = if settings.show_info_card { Some(bird_entity) } else { None };
            } else {
                selection.selected_bird = Some(bird_entity);
            }
        } else {
            selection.selected_bird = Some(bird_entity);
        }
        
        selection.last_selected_time = current_time;

        // Add highlight to selected bird
        if let Ok((_, bird_transform, _)) = bird_query.get(bird_entity) {
            commands.spawn((
                Sprite::from_color(Color::srgba(1.0, 1.0, 0.0, 0.3), Vec2::splat(80.0)),
                Transform::from_translation(bird_transform.translation + Vec3::new(0.0, 0.0, -0.1)),
                SelectionHighlight {
                    pulse_timer: 0.0,
                    base_scale: 1.0,
                },
            ));
        }
    } else {
        // Clicked on empty space - deselect
        selection.selected_bird = None;
    }
}

/// System to animate the selection highlight
pub fn selection_highlight_system(
    time: Res<Time>,
    mut highlight_query: Query<(&mut Transform, &mut SelectionHighlight)>,
) {
    for (mut transform, mut highlight) in &mut highlight_query {
        highlight.pulse_timer += time.delta().as_secs_f32() * 2.0;
        
        let pulse = (highlight.pulse_timer.sin() * 0.5 + 0.5) * 0.3 + 0.7;
        transform.scale = Vec3::splat(highlight.base_scale * pulse);
    }
}

/// System to set up the selection UI
pub fn setup_selection_ui(mut commands: Commands) {
    // Create the info card container (initially hidden)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            right: Val::Px(20.0),
            width: Val::Px(300.0),
            min_height: Val::Px(200.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(16.0)),
            row_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.15, 0.2, 0.95)),
        BorderRadius::all(Val::Px(8.0)),
        Visibility::Hidden,
        BirdInfoCard,
    )).with_children(|card| {
        // Title
        card.spawn((
            Text::new("Bird Information"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(12.0)),
                ..default()
            },
        ));
        
        // Info text container
        card.spawn((
            Text::new("No bird selected"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            BirdInfoText,
        ));
    });
}

/// System to update the selection UI
pub fn update_selection_ui(
    selection: Res<BirdSelection>,
    settings: Res<SelectionSettings>,
    bird_query: Query<(&Bird, &BirdState, &Blackboard, &AnimatedBird), With<BirdAI>>,
    mut card_query: Query<&mut Visibility, (With<BirdInfoCard>, Without<BirdInfoText>)>,
    mut text_query: Query<&mut Text, With<BirdInfoText>>,
) {
    let Ok(mut card_visibility) = card_query.single_mut() else { return };
    let Ok(mut info_text) = text_query.single_mut() else { return };

    if let Some(selected_entity) = selection.selected_bird {
        if settings.show_info_card {
            if let Ok((bird, bird_state, blackboard, animated_bird)) = bird_query.get(selected_entity) {
                *card_visibility = Visibility::Inherited;
                
                // Generate info text
                let species_name = format_species_name(animated_bird.species);
                let state_description = format_state_description(*bird_state);
                let needs_info = format_needs_info(&blackboard.internal);
                let behavior_info = format_behavior_info(bird, *bird_state);
                
                **info_text = format!(
                    "{}\n\nState: {}\n\n{}\n\n{}",
                    species_name,
                    state_description,
                    needs_info,
                    behavior_info
                );
            } else {
                *card_visibility = Visibility::Hidden;
                **info_text = "No bird selected".to_string();
            }
        } else {
            *card_visibility = Visibility::Hidden;
        }
    } else {
        *card_visibility = Visibility::Hidden;
        **info_text = "No bird selected".to_string();
    }
}

/// System to cleanup selection UI when leaving the game state
pub fn cleanup_selection_ui(
    mut commands: Commands,
    card_query: Query<Entity, With<BirdInfoCard>>,
    highlight_query: Query<Entity, With<SelectionHighlight>>,
) {
    // Clean up info cards
    for entity in &card_query {
        commands.entity(entity).despawn();
    }
    
    // Clean up highlights
    for entity in &highlight_query {
        commands.entity(entity).despawn();
    }
}

// Helper functions for formatting information

fn format_species_name(species: BirdSpecies) -> String {
    match species {
        BirdSpecies::Cardinal => "Northern Cardinal".to_string(),
        BirdSpecies::BlueJay => "Blue Jay".to_string(),
        BirdSpecies::Robin => "American Robin".to_string(),
        BirdSpecies::Sparrow => "House Sparrow".to_string(),
        BirdSpecies::Chickadee => "Black-capped Chickadee".to_string(),
        BirdSpecies::Goldfinch => "American Goldfinch".to_string(),
        BirdSpecies::NorthernMockingbird => "Northern Mockingbird".to_string(),
        BirdSpecies::RedWingedBlackbird => "Red-winged Blackbird".to_string(),
        BirdSpecies::CommonGrackle => "Common Grackle".to_string(),
        BirdSpecies::BrownThrasher => "Brown Thrasher".to_string(),
        BirdSpecies::CedarWaxwing => "Cedar Waxwing".to_string(),
        BirdSpecies::WhiteBreastedNuthatch => "White-breasted Nuthatch".to_string(),
        BirdSpecies::TuftedTitmouse => "Tufted Titmouse".to_string(),
        BirdSpecies::CarolinaWren => "Carolina Wren".to_string(),
        BirdSpecies::BlueGrayGnatcatcher => "Blue-gray Gnatcatcher".to_string(),
        BirdSpecies::YellowWarbler => "Yellow Warbler".to_string(),
        BirdSpecies::DownyWoodpecker => "Downy Woodpecker".to_string(),
        BirdSpecies::HairyWoodpecker => "Hairy Woodpecker".to_string(),
        BirdSpecies::RedHeadedWoodpecker => "Red-headed Woodpecker".to_string(),
        BirdSpecies::YellowBelledSapsucker => "Yellow-bellied Sapsucker".to_string(),
        BirdSpecies::PileatedWoodpecker => "Pileated Woodpecker".to_string(),
        BirdSpecies::RubyThroatedHummingbird => "Ruby-throated Hummingbird".to_string(),
        BirdSpecies::BrownCreeper => "Brown Creeper".to_string(),
        BirdSpecies::WinterWren => "Winter Wren".to_string(),
        BirdSpecies::PurpleFinch => "Purple Finch".to_string(),
        BirdSpecies::IndianaBunting => "Indiana Bunting".to_string(),
        BirdSpecies::RoseBreastedGrosbeak => "Rose-breasted Grosbeak".to_string(),
        BirdSpecies::WoodThrush => "Wood Thrush".to_string(),
        BirdSpecies::Catbird => "Gray Catbird".to_string(),
        BirdSpecies::ScarletTanager => "Scarlet Tanager".to_string(),
        BirdSpecies::BaltimoreOriole => "Baltimore Oriole".to_string(),
        BirdSpecies::EasternBluebird => "Eastern Bluebird".to_string(),
        BirdSpecies::PaintedBunting => "Painted Bunting".to_string(),
        BirdSpecies::CeruleanWarbler => "Cerulean Warbler".to_string(),
        BirdSpecies::HoodedWarbler => "Hooded Warbler".to_string(),
        BirdSpecies::BelttedKingfisher => "Belted Kingfisher".to_string(),
        BirdSpecies::GrandSlamAmerican => "Grand Slam American".to_string(), // Custom species
        BirdSpecies::RedTailedHawk => "Red-tailed Hawk".to_string(),
        BirdSpecies::CoopersHawk => "Cooper's Hawk".to_string(),
        BirdSpecies::BarredOwl => "Barred Owl".to_string(),
        BirdSpecies::GreatHornedOwl => "Great Horned Owl".to_string(),
        BirdSpecies::ProthonotaryWarbler => "Prothonotary Warbler".to_string(),
        BirdSpecies::KentuckyWarbler => "Kentucky Warbler".to_string(),
        BirdSpecies::GoldenWingedWarbler => "Golden-winged Warbler".to_string(),
        BirdSpecies::PeregrineFalcon => "Peregrine Falcon".to_string(),
        BirdSpecies::BaldEagle => "Bald Eagle".to_string(),
        _ => format!("{:?}", species).replace('_', " "),
    }
}

fn format_state_description(state: BirdState) -> String {
    match state {
        BirdState::Wandering => "Wandering around".to_string(),
        BirdState::MovingToTarget => "Moving to target".to_string(),
        BirdState::Eating => "Eating".to_string(),
        BirdState::Drinking => "Drinking".to_string(),
        BirdState::Bathing => "Bathing".to_string(),
        BirdState::Fleeing => "Fleeing from danger".to_string(),
        BirdState::Resting => "Resting".to_string(),
        BirdState::Playing => "Playing".to_string(),
        BirdState::Exploring => "Exploring".to_string(),
        BirdState::Nesting => "Nesting".to_string(),
        BirdState::Roosting => "Roosting".to_string(),
        BirdState::Sheltering => "Taking shelter".to_string(),
        BirdState::Courting => "Courting".to_string(),
        BirdState::Following => "Following another bird".to_string(),
        BirdState::Territorial => "Defending territory".to_string(),
        BirdState::Flocking => "Flocking behavior".to_string(),
        BirdState::Foraging => "Foraging for food".to_string(),
        BirdState::Caching => "Caching food".to_string(),
        BirdState::Retrieving => "Retrieving cached food".to_string(),
        BirdState::HoverFeeding => "Hover feeding".to_string(),
    }
}

fn format_needs_info(internal: &crate::bird_ai::components::InternalState) -> String {
    let mut needs = Vec::new();
    
    if internal.hunger > 0.7 {
        needs.push("Very hungry");
    } else if internal.hunger > 0.4 {
        needs.push("Hungry");
    }
    
    if internal.thirst > 0.7 {
        needs.push("Very thirsty");
    } else if internal.thirst > 0.4 {
        needs.push("Thirsty");
    }
    
    if internal.energy < 0.3 {
        needs.push("Tired");
    } else if internal.energy < 0.6 {
        needs.push("Low energy");
    }
    
    if internal.fear > 0.5 {
        needs.push("Frightened");
    }
    
    if internal.social_need > 0.6 {
        needs.push("Seeking company");
    }
    
    if internal.territorial_stress > 0.5 {
        needs.push("Stressed (territory)");
    }
    
    if needs.is_empty() {
        "Content".to_string()
    } else {
        format!("Needs: {}", needs.join(", "))
    }
}

fn format_behavior_info(bird: &Bird, state: BirdState) -> String {
    let mut info = Vec::new();
    
    // Add species-specific behavioral traits
    match bird.species {
        BirdSpecies::BlueJay => info.push("Intelligent, bold"),
        BirdSpecies::Cardinal => info.push("Territorial, loyal"),
        BirdSpecies::Robin => info.push("Friendly, active"),
        BirdSpecies::Chickadee => info.push("Acrobatic, curious"),
        BirdSpecies::RedWingedBlackbird => info.push("Aggressive defender"),
        BirdSpecies::CommonGrackle => info.push("Social, adaptable"),
        BirdSpecies::Goldfinch => info.push("Peaceful, social"),
        BirdSpecies::NorthernMockingbird => info.push("Vocal mimic"),
        BirdSpecies::BrownThrasher => info.push("Secretive, thorough"),
        BirdSpecies::WhiteBreastedNuthatch => info.push("Agile, upside-down feeder"),
        BirdSpecies::TuftedTitmouse => info.push("Bold, acrobatic"),
        BirdSpecies::CarolinaWren => info.push("Energetic, loud"),
        BirdSpecies::DownyWoodpecker => info.push("Patient, methodical"),
        BirdSpecies::RubyThroatedHummingbird => info.push("Territorial, high-energy"),
        _ => info.push("Unique behavioral patterns"),
    }
    
    // Add state-specific info
    match state {
        BirdState::Eating | BirdState::Drinking => {
            info.push("focused on feeding");
        },
        BirdState::Fleeing => {
            info.push("alert to danger");
        },
        BirdState::Territorial => {
            info.push("defending territory");
        },
        BirdState::Courting => {
            info.push("performing courtship display");
        },
        BirdState::Nesting => {
            info.push("engaged in nesting behavior");
        },
        BirdState::Flocking => {
            info.push("socializing with other birds");
        },
        _ => {}
    }
    
    format!("Behavior: {}", info.join(", "))
}
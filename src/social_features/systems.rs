// Social Features Systems - Phase 4
use bevy::prelude::*;
use crate::social_features::*;
use crate::photo_mode::components::PhotoTakenEvent;
use crate::bird::BirdSpecies;
use crate::{AppState, resources::BirdCount};

// Community Hub Plugin
pub struct SocialFeaturesPlugin;

impl Plugin for SocialFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CommunitySystem>()
            .add_event::<ChallengeCompletedEvent>()
            .add_event::<PhotoSharedEvent>()
            .add_event::<LeaderboardUpdateEvent>()
            .add_event::<BadgeEarnedEvent>()
            .add_systems(OnEnter(AppState::Playing), setup_community_hub)
            .add_systems(Update, (
                community_hub_input_system,
                challenge_tracking_system,
                photo_submission_system,
                leaderboard_update_system,
                badge_notification_system,
                challenge_ui_update_system,
                daily_challenge_refresh_system,
            ).run_if(in_state(AppState::Playing)));
    }
}

// Setup community hub UI
pub fn setup_community_hub(mut commands: Commands) {
    // Community hub overlay (initially hidden)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            display: Display::None,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        CommunityHubUI,
    )).with_children(|parent| {
        // Main hub container
        parent.spawn((
            Node {
                width: Val::Percent(90.0),
                height: Val::Percent(85.0),
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
            BorderColor(Color::srgb(0.6, 0.4, 0.2)),
        )).with_children(|hub| {
            
            // Left panel - Challenges
            hub.spawn((
                Node {
                    width: Val::Percent(40.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::right(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.7, 0.7, 0.7)),
                ChallengesUI,
            )).with_children(|challenges| {
                
                // Challenges header
                challenges.spawn((
                    Text::new("Daily Challenges"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));
                
                // Active challenges container
                challenges.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(15.0),
                        ..default()
                    },
                    ActiveChallengesContainer,
                ));
            });
            
            // Middle panel - Leaderboards
            hub.spawn((
                Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::right(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.7, 0.7, 0.7)),
                LeaderboardUI,
            )).with_children(|leaderboard| {
                
                // Leaderboard header
                leaderboard.spawn((
                    Text::new("Top Photographers"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));
                
                // Leaderboard entries
                leaderboard.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    LeaderboardContainer,
                ));
            });
            
            // Right panel - Photo gallery and player stats
            hub.spawn((
                Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                PhotoGalleryUI,
            )).with_children(|gallery| {
                
                // Player stats header
                gallery.spawn((
                    Text::new("Your Stats"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.3, 0.2, 0.1)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));
                
                // Stats display
                gallery.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    PlayerStatsContainer,
                ));
            });
        });
        
        // Close button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                right: Val::Px(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.3, 0.3)),
            BorderColor(Color::srgb(0.6, 0.2, 0.2)),
            CommunityHubCloseButton,
        )).with_children(|button| {
            button.spawn((
                Text::new("X"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

#[derive(Component)]
pub struct ActiveChallengesContainer;

#[derive(Component)]
pub struct LeaderboardContainer;

#[derive(Component)]
pub struct PlayerStatsContainer;

#[derive(Component)]
pub struct CommunityHubCloseButton;

#[derive(Component)]
pub struct ChallengeCard {
    pub challenge_id: u32,
}

// Input system for community hub
pub fn community_hub_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hub_query: Query<&mut Node, With<CommunityHubUI>>,
    mut close_interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<CommunityHubCloseButton>),
    >,
) {
    // Toggle community hub with H key
    if keyboard.just_pressed(KeyCode::KeyH) {
        for mut node in hub_query.iter_mut() {
            node.display = match node.display {
                Display::None => Display::Flex,
                Display::Flex => Display::None,
                _ => Display::None,
            };
        }
    }
    
    // Handle close button
    for interaction in close_interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for mut node in hub_query.iter_mut() {
                node.display = Display::None;
            }
        }
    }
}

// Challenge tracking system
pub fn challenge_tracking_system(
    mut photo_events: EventReader<PhotoTakenEvent>,
    mut community_system: ResMut<CommunitySystem>,
    mut challenge_events: EventWriter<ChallengeCompletedEvent>,
    mut badge_events: EventWriter<BadgeEarnedEvent>,
) {
    for photo_event in photo_events.read() {
        // Check each active challenge
        for challenge in &mut community_system.active_challenges {
            if challenge.is_active() {
                let completed = challenge.check_completion(
                    photo_event.score.total_score,
                    photo_event.species,
                );
                
                if completed {
                    challenge_events.send(ChallengeCompletedEvent {
                        challenge_id: challenge.id,
                        photo_score: photo_event.score.total_score,
                    });
                    
                    // Update player stats
                    community_system.player_stats.challenges_completed += 1;
                    community_system.player_stats.current_streak += 1;
                    
                    if community_system.player_stats.current_streak > community_system.player_stats.longest_streak {
                        community_system.player_stats.longest_streak = community_system.player_stats.current_streak;
                    }
                    
                    // Check for streak badges
                    if community_system.player_stats.current_streak == 7 {
                        badge_events.send(BadgeEarnedEvent {
                            badge: Badge {
                                id: "week_streak".to_string(),
                                name: "Week Warrior".to_string(),
                                description: "Complete challenges for 7 days straight".to_string(),
                                icon: "streak_7".to_string(),
                                earned_date: "2025-01-01".to_string(), // Would use actual date
                                rarity: BadgeRarity::Uncommon,
                            },
                        });
                    }
                }
            }
        }
        
        // Update general stats
        community_system.player_stats.total_photos += 1;
        if photo_event.score.total_score > community_system.player_stats.best_photo_score {
            community_system.player_stats.best_photo_score = photo_event.score.total_score;
        }
        
        if let Some(species) = photo_event.species {
            if !community_system.player_stats.species_photographed.contains(&species) {
                community_system.player_stats.species_photographed.push(species);
                
                // Species milestone badges
                let species_count = community_system.player_stats.species_photographed.len();
                match species_count {
                    5 => {
                        badge_events.send(BadgeEarnedEvent {
                            badge: Badge {
                                id: "species_5".to_string(),
                                name: "Beginner Birder".to_string(),
                                description: "Photograph 5 different species".to_string(),
                                icon: "species_5".to_string(),
                                earned_date: "2025-01-01".to_string(),
                                rarity: BadgeRarity::Common,
                            },
                        });
                    }
                    10 => {
                        badge_events.send(BadgeEarnedEvent {
                            badge: Badge {
                                id: "species_10".to_string(),
                                name: "Avian Explorer".to_string(),
                                description: "Photograph 10 different species".to_string(),
                                icon: "species_10".to_string(),
                                earned_date: "2025-01-01".to_string(),
                                rarity: BadgeRarity::Uncommon,
                            },
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

// Photo submission system
pub fn photo_submission_system(
    mut photo_events: EventReader<PhotoTakenEvent>,
    mut community_system: ResMut<CommunitySystem>,
    mut share_events: EventWriter<PhotoSharedEvent>,
) {
    for photo_event in photo_events.read() {
        // Auto-share high-scoring photos
        if photo_event.score.total_score >= 800 {
            let shared_photo = SharedPhoto {
                id: community_system.shared_photos.len() as u32 + 1,
                author: community_system.player_stats.username.clone(),
                species: photo_event.species,
                score: photo_event.score.clone(),
                title: format!("Great {} Shot!", 
                    photo_event.species.map_or("Bird".to_string(), |s| format!("{:?}", s))
                ),
                description: "Amazing capture!".to_string(),
                tags: vec!["photography".to_string(), "birds".to_string()],
                likes: 0,
                views: 0,
                shared_date: "2025-01-01".to_string(), // Would use actual date
                featured: photo_event.score.total_score >= 900,
                challenge_submission: None,
            };
            
            community_system.shared_photos.push(shared_photo.clone());
            share_events.send(PhotoSharedEvent {
                photo: shared_photo,
            });
        }
    }
}

// Leaderboard update system
pub fn leaderboard_update_system(
    mut leaderboard_events: EventReader<LeaderboardUpdateEvent>,
    mut community_system: ResMut<CommunitySystem>,
) {
    for _event in leaderboard_events.read() {
        // Update leaderboards based on player stats
        // In a real implementation, this would sync with server data
        
        let player_entry = LeaderboardEntry {
            rank: 1, // Would be calculated based on actual rankings
            player_name: community_system.player_stats.username.clone(),
            score: community_system.player_stats.best_photo_score,
            badge: community_system.player_stats.badges_earned.last()
                .map(|b| b.name.clone()),
            recent_achievement: Some("Species Explorer".to_string()),
        };
        
        // Update the top photographers leaderboard
        if let Some(leaderboard) = community_system.leaderboards.iter_mut()
            .find(|lb| matches!(lb.leaderboard_type, LeaderboardType::TopPhotographers)) {
            leaderboard.entries.clear();
            leaderboard.entries.push(player_entry);
        } else {
            // Create new leaderboard
            community_system.leaderboards.push(Leaderboard {
                leaderboard_type: LeaderboardType::TopPhotographers,
                entries: vec![player_entry],
                season: "Winter 2025".to_string(),
                last_updated: "2025-01-01".to_string(),
            });
        }
    }
}

// Badge notification system
pub fn badge_notification_system(
    mut badge_events: EventReader<BadgeEarnedEvent>,
    mut community_system: ResMut<CommunitySystem>,
    mut commands: Commands,
) {
    for badge_event in badge_events.read() {
        // Add badge to player's collection
        community_system.player_stats.badges_earned.push(badge_event.badge.clone());
        
        // Create notification UI for badge earned
        commands.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(80.0),
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                right: Val::Px(20.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.6, 0.2, 0.9)),
            BorderColor(Color::srgb(0.4, 0.8, 0.4)),
            BadgeNotification {
                timer: Timer::from_seconds(4.0, TimerMode::Once),
            },
        )).with_children(|notification| {
            notification.spawn((
                Text::new(format!("Badge Earned: {}", badge_event.badge.name)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    }
}

#[derive(Component)]
pub struct BadgeNotification {
    pub timer: Timer,
}

// Challenge UI update system
pub fn challenge_ui_update_system(
    community_system: Res<CommunitySystem>,
    mut commands: Commands,
    container_query: Query<Entity, With<ActiveChallengesContainer>>,
    challenge_card_query: Query<Entity, With<ChallengeCard>>,
) {
    if community_system.is_changed() {
        // Clear existing challenge cards
        for card_entity in challenge_card_query.iter() {
            commands.entity(card_entity).despawn_recursive();
        }
        
        // Rebuild challenge cards
        if let Ok(container) = container_query.single() {
            for challenge in &community_system.active_challenges {
                if challenge.is_active() {
                    let difficulty_color = challenge.difficulty.color();
                    
                    commands.entity(container).with_children(|container| {
                        container.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                min_height: Val::Px(100.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(15.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                            BorderColor(difficulty_color),
                            ChallengeCard { challenge_id: challenge.id },
                        )).with_children(|card| {
                            // Challenge title
                            card.spawn((
                                Text::new(challenge.title.clone()),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.2, 0.2, 0.2)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));
                            
                            // Challenge description
                            card.spawn((
                                Text::new(challenge.description.clone()),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.4, 0.4, 0.4)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));
                            
                            // Progress and reward info
                            card.spawn((
                                Text::new(format!(
                                    "Progress: {}/{} | Reward: {} credits",
                                    challenge.progress.current_value,
                                    challenge.progress.target_value,
                                    challenge.rewards.currency
                                )),
                                TextFont {
                                    font_size: 11.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.3, 0.3)),
                            ));
                        });
                    });
                }
            }
        }
    }
}

// Daily challenge refresh system
pub fn daily_challenge_refresh_system(
    mut community_system: ResMut<CommunitySystem>,
    // In a real implementation, you'd check the actual date/time
) {
    // For now, just ensure we have some active challenges
    if community_system.active_challenges.is_empty() {
        community_system.active_challenges = Challenge::generate_daily_challenges();
    }
}
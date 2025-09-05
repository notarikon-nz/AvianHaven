// Social Features Systems - Phase 4
use bevy::prelude::*;
use super::components::*;
use crate::photo_mode::components::PhotoTakenEvent;
use crate::bird::BirdSpecies;
use crate::{AppState};

use rand::{Rng, SeedableRng};
use rand::rngs::{StdRng};


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
pub fn setup_community_hub(
    mut commands: Commands,
    hub_query: Query<Entity, With<CommunityHubUI>>,
) {
    // Only create the community hub if it doesn't already exist
    if !hub_query.is_empty() {
        return;
    }
    
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
                _ => Display::Flex,
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
    let mut completed_challenges = Vec::new();
    let mut badge_to_send = None;
    
    for photo_event in photo_events.read() {
        // First pass: check challenges and collect completion info
        for (index, challenge) in community_system.active_challenges.iter_mut().enumerate() {
            if challenge.is_active() {
                let completed = challenge.check_completion(
                    photo_event.score.total_score,
                    photo_event.species,
                );
                
                if completed {
                    completed_challenges.push((index, challenge.id, photo_event.score.total_score));
                }
            }
        }
        
        // Second pass: update player stats and send events
        for (_, challenge_id, photo_score) in &completed_challenges {
            challenge_events.write(ChallengeCompletedEvent {
                challenge_id: *challenge_id,
                photo_score: *photo_score,
            });
            
            // Update player stats
            community_system.player_stats.challenges_completed += 1;
            community_system.player_stats.current_streak += 1;
            
            let current_streak = community_system.player_stats.current_streak;
            let longest_streak = community_system.player_stats.longest_streak;
            
            if current_streak > longest_streak {
                community_system.player_stats.longest_streak = current_streak;
            }
            
            // Check for streak badges
            if current_streak == 7 {
                badge_to_send = Some(Badge {
                    id: "week_streak".to_string(),
                    name: "Week Warrior".to_string(),
                    description: "Complete challenges for 7 days straight".to_string(),
                    icon: "streak_7".to_string(),
                    earned_date: "2025-01-01".to_string(),
                    rarity: BadgeRarity::Uncommon,
                });
            }
        }
        
        // Send badge if earned
        if let Some(badge) = badge_to_send.take() {
            badge_events.write(BadgeEarnedEvent { badge });
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
                        badge_events.write(BadgeEarnedEvent {
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
                        badge_events.write(BadgeEarnedEvent {
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
            share_events.write(PhotoSharedEvent {
                photo: shared_photo,
            });
        }
    }
}

// Leaderboard update system
pub fn leaderboard_update_system(
    mut leaderboard_events: EventReader<LeaderboardUpdateEvent>,
    mut community_system: ResMut<CommunitySystem>,
    photo_events: EventReader<PhotoTakenEvent>,
    time: Res<Time>,
) {
    // Update leaderboards when triggered by events
    for _event in leaderboard_events.read() {
        update_all_leaderboards(&mut community_system, time.elapsed().as_secs());
    }
    
    // Also update periodically when photos are taken
    if !photo_events.is_empty() {
        update_all_leaderboards(&mut community_system, time.elapsed().as_secs());
    }
}

fn update_all_leaderboards(community_system: &mut CommunitySystem, current_time: u64) {
    let player_stats = &community_system.player_stats;
    
    // Create current player entry for rankings
    let player_entry = LeaderboardEntry {
        rank: 1, // Will be recalculated based on comparison with others
        player_name: player_stats.username.clone(),
        score: player_stats.best_photo_score,
        badge: player_stats.badges_earned.last().map(|b| b.name.clone()),
        recent_achievement: get_most_recent_achievement(player_stats),
    };
    
    // Update Top Photographers leaderboard
    update_leaderboard_by_type(
        &mut community_system.leaderboards,
        LeaderboardType::TopPhotographers,
        player_entry.clone(),
        current_time,
    );
    
    // Update Species Collectors leaderboard
    let species_entry = LeaderboardEntry {
        rank: 1,
        player_name: player_stats.username.clone(),
        score: player_stats.species_photographed.len() as u32,
        badge: player_stats.badges_earned.last().map(|b| b.name.clone()),
        recent_achievement: get_most_recent_achievement(player_stats),
    };
    
    update_leaderboard_by_type(
        &mut community_system.leaderboards,
        LeaderboardType::RareFinds,
        species_entry,
        current_time,
    );
    
    // Update Challenge Masters leaderboard
    let challenge_entry = LeaderboardEntry {
        rank: 1,
        player_name: player_stats.username.clone(),
        score: player_stats.challenges_completed,
        badge: player_stats.badges_earned.last().map(|b| b.name.clone()),
        recent_achievement: get_most_recent_achievement(player_stats),
    };
    
    update_leaderboard_by_type(
        &mut community_system.leaderboards,
        LeaderboardType::CommunityContributor,
        challenge_entry,
        current_time,
    );
    
    // Generate mock competitive entries for demonstration
    add_mock_leaderboard_entries(&mut community_system.leaderboards, current_time);
}

fn update_leaderboard_by_type(
    leaderboards: &mut Vec<Leaderboard>,
    leaderboard_type: LeaderboardType,
    player_entry: LeaderboardEntry,
    current_time: u64,
) {
    if let Some(leaderboard) = leaderboards.iter_mut()
        .find(|lb| matches!(&lb.leaderboard_type, leaderboard_type)) {
        
        // Update or add player entry
        if let Some(existing_entry) = leaderboard.entries.iter_mut()
            .find(|entry| entry.player_name == player_entry.player_name) {
            *existing_entry = player_entry;
        } else {
            leaderboard.entries.push(player_entry);
        }
        
        // Sort by score (highest first) and update ranks
        leaderboard.entries.sort_by(|a, b| b.score.cmp(&a.score));
        for (index, entry) in leaderboard.entries.iter_mut().enumerate() {
            entry.rank = (index + 1) as u32;
        }
        
        leaderboard.last_updated = format!("Updated {} seconds ago", current_time % 3600);
        
    } else {
        // Create new leaderboard
        let mut new_leaderboard = Leaderboard {
            leaderboard_type,
            entries: vec![player_entry],
            season: get_current_season_name(),
            last_updated: "Just now".to_string(),
        };
        new_leaderboard.entries[0].rank = 1;
        leaderboards.push(new_leaderboard);
    }
}

fn add_mock_leaderboard_entries(leaderboards: &mut Vec<Leaderboard>, _current_time: u64) {
    let mock_entries = vec![
        ("BirdMaster2024", 1250, Some("Legendary Photographer".to_string())),
        ("FeatheredFriend", 1100, Some("Species Hunter".to_string())),
        ("NatureLover88", 980, Some("Challenge Champion".to_string())),
        ("WildlifeExpert", 875, Some("Photo Pro".to_string())),
        ("BirderPro", 790, None),
    ];
    
    for leaderboard in leaderboards.iter_mut() {
        // Add mock entries if leaderboard is relatively empty
        if leaderboard.entries.len() < 6 {
            for (name, base_score, badge) in &mock_entries {
                let adjusted_score = match leaderboard.leaderboard_type {
                    LeaderboardType::TopPhotographers => *base_score,
                    LeaderboardType::RareFinds => (*base_score / 100).max(1), // Convert to species count
                    LeaderboardType::CommunityContributor => (*base_score / 50).max(1), // Convert to challenge count
                    _ => *base_score,
                };
                
                // Only add if not already present
                if !leaderboard.entries.iter().any(|e| &e.player_name == name) {
                    leaderboard.entries.push(LeaderboardEntry {
                        rank: 1, // Will be recalculated
                        player_name: name.to_string(),
                        score: adjusted_score,
                        badge: badge.clone(),
                        recent_achievement: Some("Mock Achievement".to_string()),
                    });
                }
            }
            
            // Re-sort and update ranks
            leaderboard.entries.sort_by(|a, b| b.score.cmp(&a.score));
            for (index, entry) in leaderboard.entries.iter_mut().enumerate() {
                entry.rank = (index + 1) as u32;
            }
        }
    }
}

fn get_most_recent_achievement(player_stats: &PlayerStats) -> Option<String> {
    player_stats.badges_earned.last().map(|badge| badge.name.clone())
        .or_else(|| Some("First Steps".to_string()))
}

fn get_current_season_name() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // Simple season calculation based on timestamp
    match (timestamp / 2592000) % 4 { // Roughly 30-day seasons
        0 => "Spring 2025".to_string(),
        1 => "Summer 2025".to_string(),
        2 => "Fall 2025".to_string(),
        3 => "Winter 2025".to_string(),
        _ => "Current Season".to_string(),
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
            commands.entity(card_entity).despawn();
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
    time: Res<Time>,
) {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Calculate current day (UTC) for challenge refresh timing
    let current_day = current_timestamp / 86400; // Seconds in a day
    
    // Check if we need to refresh challenges
    let should_refresh = community_system.active_challenges.is_empty() || 
        should_refresh_challenges(&community_system.active_challenges);
    
    if should_refresh {
        info!("Refreshing daily challenges for day {}", current_day);
        
        // Archive completed challenges
        archive_completed_challenges(&mut community_system);
        
        // Generate new challenges based on current day and player progress
        community_system.active_challenges = generate_contextual_challenges(
            current_day,
            &community_system.player_stats,
        );
        
        // Reset player's daily progress counters
        reset_daily_progress(&mut community_system.player_stats);
        
        info!("Generated {} new challenges", community_system.active_challenges.len());
    }
    
    // Update challenge progress and check for expiring challenges
    update_challenge_progress(&mut community_system.active_challenges, time.elapsed().as_secs_f64());
}

fn should_refresh_challenges(challenges: &[Challenge]) -> bool {
    // Check if all challenges are completed
    challenges.iter().all(|c| c.progress.completed)
}

fn archive_completed_challenges(community_system: &mut CommunitySystem) {
    let completed_challenges: Vec<_> = community_system.active_challenges
        .iter()
        .filter(|c| c.progress.completed)
        .cloned()
        .collect();
    
    if !completed_challenges.is_empty() {
        info!("Archiving {} completed challenges", completed_challenges.len());
        // In a real implementation, save completed challenges to persistent storage
        // For now, just log them
        for challenge in completed_challenges {
            info!("Archived challenge: {} (Progress: {}/{})", 
                  challenge.title, 
                  challenge.progress.current_value, 
                  challenge.progress.target_value);
        }
    }
}

fn generate_contextual_challenges(current_day: u64, player_stats: &PlayerStats) -> Vec<Challenge> {
    
    // Use current day as seed for consistent daily challenges
    let mut rng = StdRng::seed_from_u64(current_day);
    let mut challenges = Vec::new();
    
    // Generate 3-5 challenges based on player level and progress
    let num_challenges = rng.random_range(3..=5);
    
    for i in 0..num_challenges {
        let challenge = match i {
            0 => generate_photo_challenge(&mut rng, player_stats, current_day),
            1 => generate_species_challenge(&mut rng, player_stats, current_day),
            2 => generate_behavior_challenge(&mut rng, player_stats, current_day),
            3 => generate_progression_challenge(&mut rng, player_stats, current_day),
            4 => generate_exploration_challenge(&mut rng, player_stats, current_day),
            _ => generate_photo_challenge(&mut rng, player_stats, current_day),
        };
        challenges.push(challenge);
    }
    
    challenges
}

fn generate_photo_challenge(rng: &mut StdRng, player_stats: &PlayerStats, current_day: u64) -> Challenge {
    
    
    let base_target = 3 + (player_stats.total_photos / 100).min(7); // Scale with experience
    let target = rng.random_range(base_target..=base_target + 2);
    let min_score = rng.random_range(600..=900);
    
    Challenge {
        id: (current_day * 1000 + 1) as u32,
        title: format!("Daily Snapshot Challenge"),
        description: format!("Take {} high-quality photos (minimum {} points each)", target, min_score),
        challenge_type: ChallengeType::SpeciesPhoto {
            target_species: BirdSpecies::Chickadee,
            min_score,
        },
        difficulty: if target > 5 { ChallengeDifficulty::Expert } else { ChallengeDifficulty::Intermediate },
        start_date: format!("Day {}", current_day),
        end_date: format!("Day {}", current_day + 1),
        progress: ChallengeProgress {
            completed: false,
            current_value: 0,
            target_value: target,
            best_submission: None,
        },
        rewards: ChallengeRewards {
            currency: target * 50,
            unlockable_lens: None,
            unlockable_filter: None,
            title: None,
            badge: Some("Daily Challenge Master".to_string()),
        },
        participants: 0,
    }
}

fn generate_species_challenge(rng: &mut StdRng, player_stats: &PlayerStats, current_day: u64) -> Challenge {
    use rand::Rng;
    use crate::bird::BirdSpecies;
    
    let species_pool = [
        BirdSpecies::Cardinal, BirdSpecies::BlueJay, BirdSpecies::Robin,
        BirdSpecies::Goldfinch, BirdSpecies::Chickadee, BirdSpecies::RedWingedBlackbird,
    ];
    
    let target_species = species_pool[rng.random_range(0..species_pool.len())];
    let target_count = rng.random_range(2..=4);
    
    Challenge {
        id: (current_day * 1000 + 2) as u32,
        title: format!("{:?} Specialist", target_species),
        description: format!("Photograph {} different {:?} birds today", target_count, target_species),
        challenge_type: ChallengeType::SpeciesPhoto {
            target_species,
            min_score: 400,
        },
        difficulty: ChallengeDifficulty::Intermediate,
        start_date: format!("Day {}", current_day),
        end_date: format!("Day {}", current_day + 1),
        progress: ChallengeProgress {
            completed: false,
            current_value: 0,
            target_value: target_count,
            best_submission: None,
        },
        rewards: ChallengeRewards {
            currency: target_count * 75,
            unlockable_lens: None,
            unlockable_filter: None,
            title: None,
            badge: Some("Species Specialist".to_string()),
        },
        participants: 0,
    }
}

fn generate_behavior_challenge(rng: &mut StdRng, player_stats: &PlayerStats, current_day: u64) -> Challenge {
    use rand::Rng;
    use crate::bird_ai::components::BirdState;
    
    let behaviors = [
        BirdState::Eating, BirdState::Drinking, BirdState::Bathing,
        BirdState::Playing, BirdState::Courting, BirdState::Territorial,
    ];
    
    let target_behavior = behaviors[rng.random_range(0..behaviors.len())];
    let target_count = rng.random_range(1..=3);
    
    let behavior_name = match target_behavior {
        BirdState::Eating => "Feeding",
        BirdState::Drinking => "Drinking",
        BirdState::Bathing => "Bathing",
        BirdState::Playing => "Playing",
        BirdState::Courting => "Courtship",
        BirdState::Territorial => "Territorial",
        _ => "Active",
    };
    
    Challenge {
        id: (current_day * 1000 + 3) as u32,
        title: format!("{} Action Shots", behavior_name),
        description: format!("Capture {} birds engaged in {} behavior", target_count, behavior_name.to_lowercase()),
        challenge_type: ChallengeType::BehaviorCapture {
            target_behavior: behavior_name.to_string(),
            min_duration: 2.0,
        },
        difficulty: ChallengeDifficulty::Advanced,
        start_date: format!("Day {}", current_day),
        end_date: format!("Day {}", current_day + 1),
        progress: ChallengeProgress {
            completed: false,
            current_value: 0,
            target_value: target_count,
            best_submission: None,
        },
        rewards: ChallengeRewards {
            currency: target_count * 100,
            unlockable_lens: None,
            unlockable_filter: None,
            title: None,
            badge: Some("Behavior Expert".to_string()),
        },
        participants: 0,
    }
}

fn generate_progression_challenge(rng: &mut StdRng, player_stats: &PlayerStats, current_day: u64) -> Challenge {
    use rand::Rng;
    
    let target_currency = rng.random_range(200..=500);
    
    Challenge {
        id: (current_day * 1000 + 4) as u32,
        title: format!("Currency Collector"),
        description: format!("Earn {} currency through photography today", target_currency),
        challenge_type: ChallengeType::SeriesPhoto {
            theme: "Currency Earning".to_string(),
            required_count: 5,
        },
        difficulty: ChallengeDifficulty::Beginner,
        start_date: format!("Day {}", current_day),
        end_date: format!("Day {}", current_day + 1),
        progress: ChallengeProgress {
            completed: false,
            current_value: 0,
            target_value: target_currency,
            best_submission: None,
        },
        rewards: ChallengeRewards {
            currency: target_currency / 2,
            unlockable_lens: None,
            unlockable_filter: None,
            title: None,
            badge: Some("Currency Master".to_string()),
        },
        participants: 0,
    }
}

fn generate_exploration_challenge(rng: &mut StdRng, player_stats: &PlayerStats, current_day: u64) -> Challenge {
    use rand::Rng;
    
    let target_locations = rng.random_range(2..=4);
    
    Challenge {
        id: (current_day * 1000 + 5) as u32,
        title: format!("Area Explorer"),
        description: format!("Take photos in {} different feeding areas", target_locations),
        challenge_type: ChallengeType::SeriesPhoto {
            theme: "Location Exploration".to_string(),
            required_count: target_locations as u32,
        },
        difficulty: ChallengeDifficulty::Intermediate,
        start_date: format!("Day {}", current_day),
        end_date: format!("Day {}", current_day + 1),
        progress: ChallengeProgress {
            completed: false,
            current_value: 0,
            target_value: target_locations as u32,
            best_submission: None,
        },
        rewards: ChallengeRewards {
            currency: target_locations * 60,
            unlockable_lens: None,
            unlockable_filter: None,
            title: None,
            badge: Some("Explorer".to_string()),
        },
        participants: 0,
    }
}

fn calculate_streak_bonus(streak: u32) -> f32 {
    match streak {
        0..=2 => 1.0,
        3..=6 => 1.2,
        7..=13 => 1.5,
        14..=29 => 1.8,
        _ => 2.0,
    }
}

fn reset_daily_progress(player_stats: &mut PlayerStats) {
    // Reset daily counters (if they exist in the future)
    info!("Reset daily progress counters for player: {}", player_stats.username);
}

fn update_challenge_progress(challenges: &mut [Challenge], elapsed_seconds: f64) {
    for challenge in challenges.iter_mut() {
        // Update any time-based aspects of challenges
        // Update progress based on elapsed time
        if challenge.progress.completed {
            info!("Challenge '{}' completed", challenge.title);
        }
    }
}
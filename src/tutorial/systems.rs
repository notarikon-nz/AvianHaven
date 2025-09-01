use bevy::prelude::*;
use crate::tutorial::{components::*, resources::*};
use crate::photo_mode::resources::PhotoModeSettings;

// Tutorial Management Systems

pub fn check_tutorial_needed(
    mut tutorial_state: ResMut<TutorialState>,
    tutorial_progress: Res<TutorialProgress>,
    mut tutorial_events: EventWriter<TutorialEvent>,
) {
    if tutorial_progress.should_show_tutorial() {
        tutorial_state.is_active = true;
        tutorial_state.current_step = TutorialStep::Welcome;
        tutorial_events.send(TutorialEvent {
            action: TutorialAction::Start,
        });
    }
}

pub fn tutorial_step_system(
    mut tutorial_events: EventReader<TutorialEvent>,
    mut tutorial_state: ResMut<TutorialState>,
    mut commands: Commands,
    time: Res<Time>,
    tutorial_ui_query: Query<Entity, With<TutorialUI>>,
) {
    // Update step timer
    tutorial_state.step_start_time += time.delta_secs();
    
    for event in tutorial_events.read() {
        match event.action {
            TutorialAction::Start => {
                tutorial_state.is_active = true;
                tutorial_state.show_ui = true;
                spawn_tutorial_ui(&mut commands);
            }
            TutorialAction::NextStep => {
                if let Some(next_step) = tutorial_state.current_step.next() {
                    tutorial_state.current_step = next_step;
                    tutorial_state.step_start_time = 0.0;
                } else {
                    tutorial_state.is_active = false;
                }
            }
            TutorialAction::Skip => {
                tutorial_state.is_active = false;
                tutorial_state.is_skipped = true;
                
                // Clean up UI
                for entity in tutorial_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            TutorialAction::Complete => {
                tutorial_state.is_active = false;
                
                // Clean up UI
                for entity in tutorial_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            TutorialAction::Show => {
                tutorial_state.show_ui = true;
                if tutorial_ui_query.is_empty() {
                    spawn_tutorial_ui(&mut commands);
                }
            }
            TutorialAction::Hide => {
                tutorial_state.show_ui = false;
                for entity in tutorial_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn tutorial_input_handler(
    mut tutorial_events: EventWriter<TutorialEvent>,
    mut step_complete_events: EventWriter<TutorialStepCompleteEvent>,
    tutorial_state: Res<TutorialState>,
    input: Res<ButtonInput<KeyCode>>,
    photo_mode_settings: Res<PhotoModeSettings>,
) {
    if !tutorial_state.is_active {
        return;
    }
    
    // ESC to skip tutorial
    if input.just_pressed(KeyCode::Escape) {
        tutorial_events.send(TutorialEvent {
            action: TutorialAction::Skip,
        });
        return;
    }
    
    // F1 to toggle tutorial UI
    if input.just_pressed(KeyCode::F1) {
        if tutorial_state.show_ui {
            tutorial_events.send(TutorialEvent {
                action: TutorialAction::Hide,
            });
        } else {
            tutorial_events.send(TutorialEvent {
                action: TutorialAction::Show,
            });
        }
    }
    
    // Check for step-specific input completions
    match tutorial_state.current_step {
        TutorialStep::CameraMovement => {
            if input.just_pressed(KeyCode::KeyW) || 
               input.just_pressed(KeyCode::KeyA) || 
               input.just_pressed(KeyCode::KeyS) || 
               input.just_pressed(KeyCode::KeyD) ||
               input.just_pressed(KeyCode::ArrowUp) ||
               input.just_pressed(KeyCode::ArrowDown) ||
               input.just_pressed(KeyCode::ArrowLeft) ||
               input.just_pressed(KeyCode::ArrowRight) {
                step_complete_events.send(TutorialStepCompleteEvent {
                    step: TutorialStep::CameraMovement,
                    auto_advance: true,
                });
            }
        }
        TutorialStep::PhotoMode => {
            if input.just_pressed(KeyCode::KeyP) {
                step_complete_events.send(TutorialStepCompleteEvent {
                    step: TutorialStep::PhotoMode,
                    auto_advance: true,
                });
            }
        }
        TutorialStep::TakePhoto => {
            if input.just_pressed(KeyCode::Space) && photo_mode_settings.is_active {
                step_complete_events.send(TutorialStepCompleteEvent {
                    step: TutorialStep::TakePhoto,
                    auto_advance: true,
                });
            }
        }
        TutorialStep::ViewJournal => {
            if input.just_pressed(KeyCode::Tab) {
                step_complete_events.send(TutorialStepCompleteEvent {
                    step: TutorialStep::ViewJournal,
                    auto_advance: true,
                });
            }
        }
        TutorialStep::OpenCatalog => {
            if input.just_pressed(KeyCode::KeyC) {
                step_complete_events.send(TutorialStepCompleteEvent {
                    step: TutorialStep::OpenCatalog,
                    auto_advance: true,
                });
            }
        }
        _ => {}
    }
}

pub fn tutorial_ui_update_system(
    tutorial_state: Res<TutorialState>,
    mut tutorial_dialog_query: Query<&mut Text, (With<TutorialDialog>, Without<TutorialNextButton>)>,
    mut tutorial_button_query: Query<&mut Text, With<TutorialNextButton>>,
) {
    if !tutorial_state.is_active || !tutorial_state.show_ui {
        return;
    }
    
    // Update dialog text
    if let Ok(mut text) = tutorial_dialog_query.get_single_mut() {
        *text = Text::new(format!(
            "{}\n\n{}",
            tutorial_state.current_step.title(),
            tutorial_state.current_step.description()
        ));
    }
    
    // Update button text based on step
    if let Ok(mut button_text) = tutorial_button_query.get_single_mut() {
        let button_label = match tutorial_state.current_step {
            TutorialStep::Welcome => "Begin Tutorial",
            TutorialStep::Complete => "Finish",
            _ => {
                if let Some(hint) = tutorial_state.current_step.input_hint() {
                    hint
                } else {
                    "Continue"
                }
            }
        };
        *button_text = Text::new(button_label);
    }
}

pub fn tutorial_highlight_system(
    tutorial_state: Res<TutorialState>,
    mut commands: Commands,
    highlight_query: Query<Entity, With<TutorialHighlight>>,
) {
    if !tutorial_state.is_active {
        // Clean up highlights if tutorial is not active
        for entity in highlight_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }
    
    // For now, we'll add basic highlighting system
    // In a full implementation, this would highlight specific UI elements
    // based on the current tutorial step
}

pub fn tutorial_completion_system(
    mut step_complete_events: EventReader<TutorialStepCompleteEvent>,
    mut tutorial_events: EventWriter<TutorialEvent>,
    mut tutorial_progress: ResMut<TutorialProgress>,
    tutorial_state: Res<TutorialState>,
) {
    for event in step_complete_events.read() {
        tutorial_progress.complete_step(event.step);
        
        if event.auto_advance {
            if tutorial_state.current_step.next().is_some() {
                tutorial_events.send(TutorialEvent {
                    action: TutorialAction::NextStep,
                });
            } else {
                tutorial_progress.tutorial_completed = true;
                let _ = tutorial_progress.save_to_file();
                
                tutorial_events.send(TutorialEvent {
                    action: TutorialAction::Complete,
                });
            }
        }
    }
}

// Button System

pub fn tutorial_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&TutorialSkipButton>, Option<&TutorialNextButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut tutorial_events: EventWriter<TutorialEvent>,
    mut step_complete_events: EventWriter<TutorialStepCompleteEvent>,
    tutorial_state: Res<TutorialState>,
) {
    for (interaction, mut bg_color, skip_button, next_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.5, 0.7, 0.5).into();
                
                if skip_button.is_some() {
                    tutorial_events.send(TutorialEvent {
                        action: TutorialAction::Skip,
                    });
                } else if next_button.is_some() {
                    match tutorial_state.current_step {
                        TutorialStep::Complete => {
                            tutorial_events.send(TutorialEvent {
                                action: TutorialAction::Complete,
                            });
                        }
                        _ => {
                            step_complete_events.send(TutorialStepCompleteEvent {
                                step: tutorial_state.current_step,
                                auto_advance: true,
                            });
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.7, 0.6, 0.5).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.6, 0.5, 0.4).into();
            }
        }
    }
}

// UI Spawning

fn spawn_tutorial_ui(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(400.0),
            height: Val::Px(200.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
        BorderColor(Color::srgb(0.8, 0.6, 0.4)),
        TutorialUI,
    )).with_children(|tutorial| {
        // Tutorial text area
        tutorial.spawn((
            Text::new("Tutorial Step"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            TutorialDialog,
        ));
        
        // Button container
        tutorial.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
        )).with_children(|buttons| {
            // Skip button
            buttons.spawn((
                Button,
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(35.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.6, 0.3, 0.3)),
                TutorialSkipButton,
            )).with_children(|button| {
                button.spawn((
                    Text::new("Skip"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
            
            // Next/Continue button
            buttons.spawn((
                Button,
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(35.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.6, 0.5, 0.4)),
                TutorialNextButton,
            )).with_children(|button| {
                button.spawn((
                    Text::new("Continue"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
        
        // Tutorial hint (F1 to toggle, ESC to skip)
        tutorial.spawn((
            Text::new("F1: Toggle Tutorial | ESC: Skip Tutorial"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            },
        ));
    });
}
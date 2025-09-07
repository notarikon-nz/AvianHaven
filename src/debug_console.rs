// Debug Console System with ~ toggle and automated testing integration
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct DebugConsolePlugin;

impl Plugin for DebugConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ConsoleState>()
            .init_resource::<ConsoleHistory>()
            .add_event::<ConsoleCommand>()
            .add_systems(Update, (
                console_toggle_system.before(console_text_input_system),
                console_text_input_system.run_if(console_is_visible),
                console_command_processor,
                console_ui_update_system.run_if(console_is_visible),
                console_message_display_system.run_if(console_is_visible),
            ))
            .add_systems(Startup, setup_console_ui);
    }
}

// Run condition to check if console is visible
pub fn console_is_visible(console_state: Res<ConsoleState>) -> bool {
    console_state.visible
}

// Run condition to check if console is NOT visible (for blocking game input)
pub fn console_is_not_visible(console_state: Res<ConsoleState>) -> bool {
    !console_state.visible
}

#[derive(Resource)]
pub struct ConsoleState {
    pub visible: bool,
}

impl Default for ConsoleState {
    fn default() -> Self {
        Self {
            visible: false,
        }
    }
}

#[derive(Resource)]
pub struct ConsoleHistory {
    pub messages: VecDeque<ConsoleMessage>,
    pub command_history: VecDeque<String>,
    pub history_index: usize,
    pub max_messages: usize,
    pub max_history: usize,
}

impl Default for ConsoleHistory {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
            command_history: VecDeque::new(),
            history_index: 0,
            max_messages: 100,
            max_history: 50,
        }
    }
}

#[derive(Clone)]
pub struct ConsoleMessage {
    pub text: String,
    pub message_type: MessageType,
    pub timestamp: f64,
}

#[derive(Clone, PartialEq)]
pub enum MessageType {
    Command,
    Info,
    Warning,
    Error,
    Success,
}

#[derive(Event)]
pub struct ConsoleCommand {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Component)]
pub struct ConsoleRootUI;

#[derive(Component)]
pub struct ConsoleInputText;

#[derive(Component)]
pub struct ConsoleOutputText;

#[derive(Component)]
pub struct ConsoleTextInput {
    pub current_text: String,
    pub cursor_position: usize,
    pub focused: bool,
}

pub fn setup_console_ui(mut commands: Commands) {
    // Create console UI that's initially hidden
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            display: Display::None, // Initially hidden
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        ConsoleRootUI,
    )).with_children(|parent| {
        // Output area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(85.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            ConsoleOutputText,
        )).with_children(|output| {
            // Initial welcome message
            output.spawn((
                Text::new("Debug Console - Type 'help' for commands"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::vertical(Val::Px(2.0)),
                    ..default()
                },
            ));
        });
        
        // Input area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
        )).with_children(|input_row| {
            // Prompt
            input_row.spawn((
                Text::new("> "),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 1.0, 0.7)),
            ));
            
            // Input field
            input_row.spawn((
                Text::new(""),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ConsoleInputText,
                ConsoleTextInput {
                    current_text: String::new(),
                    cursor_position: 0,
                    focused: false,
                },
            ));
        });
    });
}

pub fn console_toggle_system(
    mut console_state: ResMut<ConsoleState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut console_query: Query<&mut Node, With<ConsoleRootUI>>,
    mut input_query: Query<&mut ConsoleTextInput>,
) {
    if keyboard.just_pressed(KeyCode::Backquote) { // ~ key
        console_state.visible = !console_state.visible;
        
        if let Ok(mut console_node) = console_query.single_mut() {
            console_node.display = if console_state.visible {
                Display::Flex
            } else {
                Display::None
            };
        }
        
        // Reset input when opening/closing
        if let Ok(mut input) = input_query.single_mut() {
            if console_state.visible {
                input.current_text.clear();
                input.cursor_position = 0;
                input.focused = true;
            } else {
                input.focused = false;
            }
        }
    }
}

pub fn console_text_input_system(
    console_state: Res<ConsoleState>,
    mut console_history: ResMut<ConsoleHistory>,
    mut command_events: EventWriter<ConsoleCommand>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input_query: Query<&mut ConsoleTextInput>,
) {
    if !console_state.visible {
        return;
    }

    let Ok(mut input) = input_query.single_mut() else { return; };
    if !input.focused {
        return;
    }

    // Handle Enter key for command execution
    if keyboard.just_pressed(KeyCode::Enter) {
        if !input.current_text.is_empty() {
            // Parse and send command
            let parts: Vec<String> = input.current_text
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            
            if let Some(command) = parts.first() {
                let command = command.clone();
                let args = parts.into_iter().skip(1).collect();
                
                // Add to history
                console_history.command_history.push_back(input.current_text.clone());
                if console_history.command_history.len() > console_history.max_history {
                    console_history.command_history.pop_front();
                }
                console_history.history_index = console_history.command_history.len();
                
                // Add command to message history
                add_console_message(
                    &mut console_history,
                    format!("> {}", input.current_text),
                    MessageType::Command,
                );
                
                // Send command event
                command_events.write(ConsoleCommand {
                    command,
                    args,
                });
            }
            
            input.current_text.clear();
            input.cursor_position = 0;
        }
    }
    
    // Handle Backspace
    if keyboard.just_pressed(KeyCode::Backspace) {
        if input.cursor_position > 0 {
            let pos = input.cursor_position - 1;
            input.current_text.remove(pos);
            input.cursor_position = pos;
        }
    }
    
    // Handle Delete
    if keyboard.just_pressed(KeyCode::Delete) {
        let pos = input.cursor_position;
        if pos < input.current_text.len() {
            input.current_text.remove(pos);
        }
    }

    // Handle arrow keys and special keys
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if input.cursor_position > 0 {
            input.cursor_position -= 1;
        }
    }
    
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        if input.cursor_position < input.current_text.len() {
            input.cursor_position += 1;
        }
    }
    
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        if console_history.history_index > 0 {
            console_history.history_index -= 1;
            if let Some(cmd) = console_history.command_history.get(console_history.history_index) {
                input.current_text = cmd.clone();
                input.cursor_position = input.current_text.len();
            }
        }
    }
    
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if console_history.history_index < console_history.command_history.len() {
            console_history.history_index += 1;
            if console_history.history_index == console_history.command_history.len() {
                input.current_text.clear();
                input.cursor_position = 0;
            } else if let Some(cmd) = console_history.command_history.get(console_history.history_index) {
                input.current_text = cmd.clone();
                input.cursor_position = input.current_text.len();
            }
        }
    }
    
    // Handle character input using a simple approach
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    
    // Handle all letter and number keys
    for key in [
        KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC, KeyCode::KeyD, KeyCode::KeyE,
        KeyCode::KeyF, KeyCode::KeyG, KeyCode::KeyH, KeyCode::KeyI, KeyCode::KeyJ,
        KeyCode::KeyK, KeyCode::KeyL, KeyCode::KeyM, KeyCode::KeyN, KeyCode::KeyO,
        KeyCode::KeyP, KeyCode::KeyQ, KeyCode::KeyR, KeyCode::KeyS, KeyCode::KeyT,
        KeyCode::KeyU, KeyCode::KeyV, KeyCode::KeyW, KeyCode::KeyX, KeyCode::KeyY,
        KeyCode::KeyZ, KeyCode::Digit0, KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3,
        KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8,
        KeyCode::Digit9, KeyCode::Space, KeyCode::Period, KeyCode::Comma, KeyCode::Semicolon,
        KeyCode::Quote, KeyCode::Slash, KeyCode::Backslash, KeyCode::BracketLeft,
        KeyCode::BracketRight, KeyCode::Minus, KeyCode::Equal,
    ] {
        if keyboard.just_pressed(key) {
            if let Some(ch) = key_code_to_char(key, shift) {
                let pos = input.cursor_position;
                input.current_text.insert(pos, ch);
                input.cursor_position = pos + 1;
            }
        }
    }
}

pub fn console_command_processor(
    mut command_events: EventReader<ConsoleCommand>,
    mut console_history: ResMut<ConsoleHistory>,
    mut test_events: EventWriter<crate::automated_testing::TestEvent>,
    mut acceleration: ResMut<crate::automated_testing::TimeAcceleration>,
    testing_state: Res<crate::automated_testing::TestingState>,
    bird_query: Query<&crate::bird::Bird>,
    mut commands: Commands,
) {
    for command in command_events.read() {
        match command.command.as_str() {
            "help" => {
                add_console_message(&mut console_history, "Available commands:".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  test run <scenario>  - Run test scenario (population_stress, seasonal_cycle)".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  test stop           - Stop current test".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  test list           - List available test scenarios".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  test status         - Show current test status".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  time <multiplier>   - Set time acceleration (1-100)".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  spawn <species> <count> - Spawn birds (robin, cardinal, bluejay)".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  population          - Show current bird population".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  clear              - Clear console".to_string(), MessageType::Info);
                add_console_message(&mut console_history, "  help               - Show this help".to_string(), MessageType::Info);
            },
            "test" => {
                if command.args.is_empty() {
                    add_console_message(&mut console_history, "Usage: test <run|stop|list|status>".to_string(), MessageType::Warning);
                } else {
                    match command.args[0].as_str() {
                        "run" => {
                            if command.args.len() < 2 {
                                add_console_message(&mut console_history, "Usage: test run <scenario>".to_string(), MessageType::Warning);
                            } else {
                                let scenario = command.args[1].clone();
                                test_events.write(crate::automated_testing::TestEvent::StartTest(scenario.clone()));
                                add_console_message(&mut console_history, format!("Starting test: {}", scenario), MessageType::Success);
                            }
                        },
                        "stop" => {
                            test_events.write(crate::automated_testing::TestEvent::StopTest);
                            add_console_message(&mut console_history, "Stopping current test".to_string(), MessageType::Info);
                        },
                        "list" => {
                            add_console_message(&mut console_history, "Available test scenarios:".to_string(), MessageType::Info);
                            add_console_message(&mut console_history, "  population_stress - Test high bird populations".to_string(), MessageType::Info);
                            add_console_message(&mut console_history, "  seasonal_cycle   - Test seasonal transitions".to_string(), MessageType::Info);
                        },
                        "status" => {
                            let status = crate::automated_testing::get_test_status(&testing_state);
                            add_console_message(&mut console_history, format!("Test Status: {}", status), MessageType::Info);
                        },
                        _ => {
                            add_console_message(&mut console_history, format!("Unknown test command: {}", command.args[0]), MessageType::Error);
                        }
                    }
                }
            },
            "time" => {
                if command.args.is_empty() {
                    add_console_message(&mut console_history, format!("Current time multiplier: {:.1}x", acceleration.multiplier), MessageType::Info);
                } else if let Ok(multiplier) = command.args[0].parse::<f32>() {
                    if multiplier >= 1.0 && multiplier <= acceleration.max_multiplier {
                        acceleration.multiplier = multiplier;
                        acceleration.enabled = multiplier > 1.0;
                        add_console_message(&mut console_history, format!("Time multiplier set to {:.1}x", multiplier), MessageType::Success);
                    } else {
                        add_console_message(&mut console_history, format!("Time multiplier must be between 1.0 and {:.1}", acceleration.max_multiplier), MessageType::Error);
                    }
                } else {
                    add_console_message(&mut console_history, "Invalid time multiplier. Use a number between 1.0 and 100.0".to_string(), MessageType::Error);
                }
            },
            "spawn" => {
                if command.args.len() < 2 {
                    add_console_message(&mut console_history, "Usage: spawn <species> <count>".to_string(), MessageType::Warning);
                } else {
                    let species_str = command.args[0].to_lowercase();
                    let species = match species_str.as_str() {
                        "robin" => Some(crate::bird::BirdSpecies::Robin),
                        "cardinal" => Some(crate::bird::BirdSpecies::Cardinal),
                        "bluejay" => Some(crate::bird::BirdSpecies::BlueJay),
                        "chickadee" => Some(crate::bird::BirdSpecies::Chickadee),
                        _ => None,
                    };
                    
                    if let Some(species) = species {
                        if let Ok(count) = command.args[1].parse::<u32>() {
                            if count > 0 && count <= 100 {
                                // Spawn birds via bird system (would need integration)
                                add_console_message(&mut console_history, format!("Spawning {} {} birds", count, species_str), MessageType::Success);
                                // TODO: Actually spawn the birds via the bird spawning system
                            } else {
                                add_console_message(&mut console_history, "Count must be between 1 and 100".to_string(), MessageType::Error);
                            }
                        } else {
                            add_console_message(&mut console_history, "Invalid count. Use a number between 1 and 100".to_string(), MessageType::Error);
                        }
                    } else {
                        add_console_message(&mut console_history, format!("Unknown species: {}. Try: robin, cardinal, bluejay, chickadee", species_str), MessageType::Error);
                    }
                }
            },
            "population" => {
                let mut species_counts = std::collections::HashMap::new();
                for bird in bird_query.iter() {
                    *species_counts.entry(bird.species).or_insert(0) += 1;
                }
                
                if species_counts.is_empty() {
                    add_console_message(&mut console_history, "No birds currently in the game".to_string(), MessageType::Info);
                } else {
                    add_console_message(&mut console_history, "Current bird population:".to_string(), MessageType::Info);
                    for (species, count) in species_counts {
                        add_console_message(&mut console_history, format!("  {:?}: {}", species, count), MessageType::Info);
                    }
                }
            },
            "clear" => {
                console_history.messages.clear();
                add_console_message(&mut console_history, "Console cleared".to_string(), MessageType::Info);
            },
            _ => {
                add_console_message(&mut console_history, format!("Unknown command: {}. Type 'help' for available commands.", command.command), MessageType::Error);
            }
        }
    }
}

pub fn console_ui_update_system(
    input_query: Query<&ConsoleTextInput>,
    mut input_text_query: Query<&mut Text, With<ConsoleInputText>>,
    time: Res<Time>,
) {
    // Update input text display
    if let Ok(input) = input_query.single() {
        if let Ok(mut text) = input_text_query.single_mut() {
            let mut display_text = input.current_text.clone();
            
            // Add blinking cursor
            let cursor_blink = (time.elapsed_secs() * 2.0) % 2.0 > 1.0;
            if cursor_blink && input.focused {
                display_text.insert(input.cursor_position.min(display_text.len()), '|');
            }
            
            text.0 = display_text;
        }
    }
}

pub fn console_message_display_system(
    console_history: Res<ConsoleHistory>,
    output_query: Query<Entity, With<ConsoleOutputText>>,
    mut commands: Commands,
) {
    if console_history.is_changed() {
        if let Ok(output_entity) = output_query.single() {
            // Add new messages to the output
            if let Some(latest_message) = console_history.messages.back() {
                let color = match latest_message.message_type {
                    MessageType::Command => Color::srgb(0.7, 0.7, 1.0),
                    MessageType::Info => Color::srgb(1.0, 1.0, 1.0),
                    MessageType::Warning => Color::srgb(1.0, 1.0, 0.0),
                    MessageType::Error => Color::srgb(1.0, 0.3, 0.3),
                    MessageType::Success => Color::srgb(0.3, 1.0, 0.3),
                };
                
                commands.entity(output_entity).with_children(|output| {
                    output.spawn((
                        Text::new(&latest_message.text),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(color),
                        Node {
                            margin: UiRect::vertical(Val::Px(2.0)),
                            ..default()
                        },
                    ));
                });
            }
        }
    }
}

fn add_console_message(history: &mut ConsoleHistory, text: String, message_type: MessageType) {
    history.messages.push_back(ConsoleMessage {
        text,
        message_type,
        timestamp: 0.0, // Could use actual timestamp if needed
    });
    
    if history.messages.len() > history.max_messages {
        history.messages.pop_front();
    }
}

fn key_code_to_char(key_code: KeyCode, shift: bool) -> Option<char> {
    match key_code {
        KeyCode::Space => Some(' '),
        KeyCode::KeyA => Some(if shift { 'A' } else { 'a' }),
        KeyCode::KeyB => Some(if shift { 'B' } else { 'b' }),
        KeyCode::KeyC => Some(if shift { 'C' } else { 'c' }),
        KeyCode::KeyD => Some(if shift { 'D' } else { 'd' }),
        KeyCode::KeyE => Some(if shift { 'E' } else { 'e' }),
        KeyCode::KeyF => Some(if shift { 'F' } else { 'f' }),
        KeyCode::KeyG => Some(if shift { 'G' } else { 'g' }),
        KeyCode::KeyH => Some(if shift { 'H' } else { 'h' }),
        KeyCode::KeyI => Some(if shift { 'I' } else { 'i' }),
        KeyCode::KeyJ => Some(if shift { 'J' } else { 'j' }),
        KeyCode::KeyK => Some(if shift { 'K' } else { 'k' }),
        KeyCode::KeyL => Some(if shift { 'L' } else { 'l' }),
        KeyCode::KeyM => Some(if shift { 'M' } else { 'm' }),
        KeyCode::KeyN => Some(if shift { 'N' } else { 'n' }),
        KeyCode::KeyO => Some(if shift { 'O' } else { 'o' }),
        KeyCode::KeyP => Some(if shift { 'P' } else { 'p' }),
        KeyCode::KeyQ => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::KeyR => Some(if shift { 'R' } else { 'r' }),
        KeyCode::KeyS => Some(if shift { 'S' } else { 's' }),
        KeyCode::KeyT => Some(if shift { 'T' } else { 't' }),
        KeyCode::KeyU => Some(if shift { 'U' } else { 'u' }),
        KeyCode::KeyV => Some(if shift { 'V' } else { 'v' }),
        KeyCode::KeyW => Some(if shift { 'W' } else { 'w' }),
        KeyCode::KeyX => Some(if shift { 'X' } else { 'x' }),
        KeyCode::KeyY => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::KeyZ => Some(if shift { 'Z' } else { 'z' }),
        KeyCode::Digit0 => Some(if shift { ')' } else { '0' }),
        KeyCode::Digit1 => Some(if shift { '!' } else { '1' }),
        KeyCode::Digit2 => Some(if shift { '@' } else { '2' }),
        KeyCode::Digit3 => Some(if shift { '#' } else { '3' }),
        KeyCode::Digit4 => Some(if shift { '$' } else { '4' }),
        KeyCode::Digit5 => Some(if shift { '%' } else { '5' }),
        KeyCode::Digit6 => Some(if shift { '^' } else { '6' }),
        KeyCode::Digit7 => Some(if shift { '&' } else { '7' }),
        KeyCode::Digit8 => Some(if shift { '*' } else { '8' }),
        KeyCode::Digit9 => Some(if shift { '(' } else { '9' }),
        KeyCode::Period => Some(if shift { '>' } else { '.' }),
        KeyCode::Comma => Some(if shift { '<' } else { ',' }),
        KeyCode::Semicolon => Some(if shift { ':' } else { ';' }),
        KeyCode::Quote => Some(if shift { '"' } else { '\'' }),
        KeyCode::Slash => Some(if shift { '?' } else { '/' }),
        KeyCode::Backslash => Some(if shift { '|' } else { '\\' }),
        KeyCode::BracketLeft => Some(if shift { '{' } else { '[' }),
        KeyCode::BracketRight => Some(if shift { '}' } else { ']' }),
        KeyCode::Minus => Some(if shift { '_' } else { '-' }),
        KeyCode::Equal => Some(if shift { '+' } else { '=' }),
        _ => None,
    }
}
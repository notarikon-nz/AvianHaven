use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameAction {
    // Camera Controls
    CameraMoveUp,
    CameraMoveDown,
    CameraMoveLeft,
    CameraMoveRight,
    CameraZoomIn,
    CameraZoomOut,
    CameraPan,
    
    // Photo Mode
    TogglePhotoMode,
    TakePhoto,
    PhotoModeSettings,
    
    // UI Navigation
    OpenJournal,
    OpenSettings,
    CloseMenu,
    PauseGame,
    
    // Gameplay
    PlaceFeeder,
    RemoveObject,
    QuickSave,
    QuickLoad,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputBinding {
    Keyboard(KeyCode),
    Mouse(MouseButton),
    MouseWheelUp,
    MouseWheelDown,
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct KeyBindings {
    pub bindings: HashMap<GameAction, Vec<InputBinding>>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        
        // Camera Controls
        bindings.insert(GameAction::CameraMoveUp, vec![InputBinding::Keyboard(KeyCode::KeyW)]);
        bindings.insert(GameAction::CameraMoveDown, vec![InputBinding::Keyboard(KeyCode::KeyS)]);
        bindings.insert(GameAction::CameraMoveLeft, vec![InputBinding::Keyboard(KeyCode::KeyA)]);
        bindings.insert(GameAction::CameraMoveRight, vec![InputBinding::Keyboard(KeyCode::KeyD)]);
        bindings.insert(GameAction::CameraZoomIn, vec![InputBinding::MouseWheelUp]);
        bindings.insert(GameAction::CameraZoomOut, vec![InputBinding::MouseWheelDown]);
        bindings.insert(GameAction::CameraPan, vec![InputBinding::Mouse(MouseButton::Middle)]);
        
        // Photo Mode
        bindings.insert(GameAction::TogglePhotoMode, vec![InputBinding::Keyboard(KeyCode::KeyP)]);
        bindings.insert(GameAction::TakePhoto, vec![InputBinding::Keyboard(KeyCode::Space)]);
        bindings.insert(GameAction::PhotoModeSettings, vec![InputBinding::Keyboard(KeyCode::KeyF)]);
        
        // UI Navigation
        bindings.insert(GameAction::OpenJournal, vec![InputBinding::Keyboard(KeyCode::Tab)]);
        bindings.insert(GameAction::OpenSettings, vec![InputBinding::Keyboard(KeyCode::Escape)]);
        bindings.insert(GameAction::CloseMenu, vec![InputBinding::Keyboard(KeyCode::Escape)]);
        bindings.insert(GameAction::PauseGame, vec![InputBinding::Keyboard(KeyCode::Escape)]);
        
        // Gameplay
        bindings.insert(GameAction::PlaceFeeder, vec![InputBinding::Keyboard(KeyCode::KeyF)]);
        bindings.insert(GameAction::RemoveObject, vec![InputBinding::Keyboard(KeyCode::Delete)]);
        bindings.insert(GameAction::QuickSave, vec![InputBinding::Keyboard(KeyCode::F5)]);
        bindings.insert(GameAction::QuickLoad, vec![InputBinding::Keyboard(KeyCode::F9)]);
        
        Self { bindings }
    }
}

impl KeyBindings {
    pub fn get_bindings_path() -> std::path::PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("AvianHaven")
            .join("keybindings.ron")
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let bindings_path = Self::get_bindings_path();
        
        if let Some(parent) = bindings_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let serialized = ron::to_string(self)?;
        fs::write(bindings_path, serialized)?;
        Ok(())
    }
    
    pub fn load_from_file() -> Self {
        let bindings_path = Self::get_bindings_path();
        
        if bindings_path.exists() {
            if let Ok(content) = fs::read_to_string(bindings_path) {
                if let Ok(bindings) = ron::from_str::<KeyBindings>(&content) {
                    return bindings;
                }
            }
        }
        
        // Return default bindings if loading fails
        Self::default()
    }
    
    pub fn is_action_pressed(&self, action: GameAction, input: &ButtonInput<KeyCode>, mouse: &ButtonInput<MouseButton>) -> bool {
        if let Some(bindings) = self.bindings.get(&action) {
            for binding in bindings {
                match binding {
                    InputBinding::Keyboard(key) => {
                        if input.pressed(*key) {
                            return true;
                        }
                    }
                    InputBinding::Mouse(button) => {
                        if mouse.pressed(*button) {
                            return true;
                        }
                    }
                    InputBinding::MouseWheelUp | InputBinding::MouseWheelDown => {
                        // Mouse wheel is handled separately in scroll events
                        continue;
                    }
                }
            }
        }
        false
    }
    
    pub fn is_action_just_pressed(&self, action: GameAction, input: &ButtonInput<KeyCode>, mouse: &ButtonInput<MouseButton>) -> bool {
        if let Some(bindings) = self.bindings.get(&action) {
            for binding in bindings {
                match binding {
                    InputBinding::Keyboard(key) => {
                        if input.just_pressed(*key) {
                            return true;
                        }
                    }
                    InputBinding::Mouse(button) => {
                        if mouse.just_pressed(*button) {
                            return true;
                        }
                    }
                    InputBinding::MouseWheelUp | InputBinding::MouseWheelDown => {
                        // Mouse wheel is handled separately
                        continue;
                    }
                }
            }
        }
        false
    }
    
    pub fn add_binding(&mut self, action: GameAction, binding: InputBinding) {
        self.bindings.entry(action).or_insert_with(Vec::new).push(binding);
    }
    
    pub fn remove_binding(&mut self, action: GameAction, binding: &InputBinding) {
        if let Some(bindings) = self.bindings.get_mut(&action) {
            bindings.retain(|b| b != binding);
        }
    }
    
    pub fn clear_action_bindings(&mut self, action: GameAction) {
        self.bindings.insert(action, Vec::new());
    }
    
    pub fn get_primary_binding(&self, action: GameAction) -> Option<&InputBinding> {
        self.bindings.get(&action).and_then(|bindings| bindings.first())
    }
    
    pub fn get_display_string(&self, action: GameAction) -> String {
        if let Some(binding) = self.get_primary_binding(action) {
            match binding {
                InputBinding::Keyboard(key) => format!("{:?}", key),
                InputBinding::Mouse(button) => format!("{:?}", button),
                InputBinding::MouseWheelUp => "Mouse Wheel Up".to_string(),
                InputBinding::MouseWheelDown => "Mouse Wheel Down".to_string(),
            }
        } else {
            "Unbound".to_string()
        }
    }
}

pub struct KeyBindingsPlugin;

impl Plugin for KeyBindingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<KeyBindings>()
            .add_systems(Startup, load_keybindings)
            .add_systems(Update, (
                handle_camera_input,
                handle_ui_input,
                handle_gameplay_input,
            ));
    }
}

fn load_keybindings(mut commands: Commands) {
    let keybindings = KeyBindings::load_from_file();
    commands.insert_resource(keybindings);
}

fn handle_camera_input(
    keybindings: Res<KeyBindings>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let speed = 300.0 * time.delta_secs();
        
        if keybindings.is_action_pressed(GameAction::CameraMoveUp, &keyboard, &mouse) {
            transform.translation.y += speed;
        }
        if keybindings.is_action_pressed(GameAction::CameraMoveDown, &keyboard, &mouse) {
            transform.translation.y -= speed;
        }
        if keybindings.is_action_pressed(GameAction::CameraMoveLeft, &keyboard, &mouse) {
            transform.translation.x -= speed;
        }
        if keybindings.is_action_pressed(GameAction::CameraMoveRight, &keyboard, &mouse) {
            transform.translation.x += speed;
        }
    }
}

fn handle_ui_input(
    keybindings: Res<KeyBindings>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut app_state: ResMut<NextState<crate::AppState>>,
    current_state: Res<State<crate::AppState>>,
) {
    if keybindings.is_action_just_pressed(GameAction::OpenJournal, &keyboard, &mouse) {
        if *current_state.get() == crate::AppState::Playing {
            app_state.set(crate::AppState::Journal);
        }
    }
    
    if keybindings.is_action_just_pressed(GameAction::CloseMenu, &keyboard, &mouse) {
        match current_state.get() {
            crate::AppState::Settings | crate::AppState::LoadGame => {
                app_state.set(crate::AppState::MainMenu);
            }
            crate::AppState::Journal => {
                app_state.set(crate::AppState::Playing);
            }
            crate::AppState::Playing => {
                app_state.set(crate::AppState::MainMenu);
            }
            _ => {}
        }
    }
}

fn handle_gameplay_input(
    keybindings: Res<KeyBindings>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    // Add other resources as needed for specific gameplay actions
) {
    if keybindings.is_action_just_pressed(GameAction::TakePhoto, &keyboard, &mouse) {
        info!("Photo action triggered!");
        // Handle photo taking logic
    }
    
    if keybindings.is_action_just_pressed(GameAction::TogglePhotoMode, &keyboard, &mouse) {
        info!("Toggle photo mode!");
        // Handle photo mode toggle
    }
}
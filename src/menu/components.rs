use bevy::prelude::*;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct MainMenuButton {
    pub action: MainMenuAction,
}

#[derive(Component)]
pub struct SettingsButton {
    pub action: SettingsAction,
}

#[derive(Component)]
pub struct LoadGameButton {
    pub save_slot: u32,
}

#[derive(Component)]
pub struct MenuTitle;

#[derive(Component)]
pub struct SaveSlotCard {
    pub slot: u32,
}

#[derive(Component)]
pub struct SettingsSlider {
    pub setting: SettingType,
    pub current_value: f32,
    pub min_value: f32,
    pub max_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMenuAction {
    NewGame,
    LoadGame,
    Settings,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsAction {
    BackToMain,
    ResetToDefaults,
    ApplySettings,
    OpenControls,
    BackToSettings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingType {
    MasterVolume,
    MusicVolume,
    SfxVolume,
    AutoSave,
}

#[derive(Component)]
pub struct VolumeSlider {
    pub setting_type: SettingType,
}

#[derive(Component)]
pub struct GraphicsToggle {
    pub setting_type: GraphicsSettingType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphicsSettingType {
    VSync,
    Fullscreen,
    Resolution,
    GraphicsQuality,
}

#[derive(Component)]
pub struct KeybindingButton {
    pub action: crate::keybindings::GameAction,
}

#[derive(Component)]
pub struct KeybindingText {
    pub action: crate::keybindings::GameAction,
}

#[derive(Component)]
pub struct KeybindingWaiting {
    pub action: crate::keybindings::GameAction,
}

// New dropdown components for settings
#[derive(Component)]
pub struct ResolutionDropdown;

#[derive(Component)]
pub struct GraphicsQualityDropdown;

#[derive(Component)]
pub struct AudioSection;

#[derive(Component)]
pub struct ResolutionDropdownLabel;

#[derive(Component)]
pub struct GraphicsSection;

#[derive(Component)]
pub struct FullscreenToggle;

#[derive(Component)]
pub struct FullscreenToggleContainer;
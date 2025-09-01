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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingType {
    MasterVolume,
    MusicVolume,
    SfxVolume,
    AutoSave,
}
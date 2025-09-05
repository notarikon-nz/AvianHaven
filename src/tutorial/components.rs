use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component)]
pub struct TutorialUI;

#[derive(Component)]
pub struct TutorialDialog;

#[derive(Component)]
pub struct TutorialHighlight;

#[derive(Component)]
pub struct TutorialPointer;

#[derive(Component)]
pub struct TutorialSkipButton;

#[derive(Component)]
pub struct TutorialNextButton;

#[derive(Component)]
pub struct TutorialTarget {
    pub step: TutorialStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TutorialStep {
    Welcome,
    CameraMovement,
    BirdObservation,
    PhotoMode,
    TakePhoto,
    ViewJournal,
    OpenCatalog,
    PlaceFeeder,
    BirdFeeding,
    UpgradeFeeder,
    Complete,
}

impl TutorialStep {
    pub fn title(&self) -> &'static str {
        match self {
            TutorialStep::Welcome => "Welcome to Perch!",
            TutorialStep::CameraMovement => "Camera Controls",
            TutorialStep::BirdObservation => "Observe Birds",
            TutorialStep::PhotoMode => "Photo Mode",
            TutorialStep::TakePhoto => "Take a Photo",
            TutorialStep::ViewJournal => "Bird Journal",
            TutorialStep::OpenCatalog => "Shop & Catalog",
            TutorialStep::PlaceFeeder => "Place a Feeder",
            TutorialStep::BirdFeeding => "Watch Birds Feed",
            TutorialStep::UpgradeFeeder => "Upgrade System",
            TutorialStep::Complete => "Tutorial Complete!",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            TutorialStep::Welcome => "Learn the basics of bird watching and sanctuary management. This tutorial will guide you through all the essential features.",
            TutorialStep::CameraMovement => "Use WASD or arrow keys to move the camera around. Mouse wheel or +/- keys to zoom in and out.",
            TutorialStep::BirdObservation => "Look around and observe the birds that spawn naturally. Each species has unique behaviors and preferences.",
            TutorialStep::PhotoMode => "Press P to enter Photo Mode. This allows you to take pictures of birds for your journal and earn currency.",
            TutorialStep::TakePhoto => "In Photo Mode, use the mouse to aim and press Space to take a photo. Photos are scored based on bird behavior and composition.",
            TutorialStep::ViewJournal => "Press Tab to open your Bird Journal. Here you can view discovered species, photos, and achievements.",
            TutorialStep::OpenCatalog => "Press C to open the Catalog. This is where you can purchase feeders and decorative items for your sanctuary.",
            TutorialStep::PlaceFeeder => "Purchase a feeder from the catalog and place it in your world. Birds will discover and use feeders based on their preferences.",
            TutorialStep::BirdFeeding => "Watch as birds discover your feeder and feed from it. Different species prefer different feeder types and foods.",
            TutorialStep::UpgradeFeeder => "Use currency earned from photos to upgrade feeders. Higher level feeders attract more rare birds.",
            TutorialStep::Complete => "You've completed the tutorial! Continue exploring, photographing birds, and building your sanctuary. Happy bird watching!",
        }
    }
    
    pub fn input_hint(&self) -> Option<&'static str> {
        match self {
            TutorialStep::CameraMovement => Some("Try moving with WASD and zooming with mouse wheel"),
            TutorialStep::PhotoMode => Some("Press P to enter Photo Mode"),
            TutorialStep::TakePhoto => Some("Press Space to take a photo"),
            TutorialStep::ViewJournal => Some("Press Tab to open journal"),
            TutorialStep::OpenCatalog => Some("Press C to open catalog"),
            _ => None,
        }
    }
    
    pub fn next(&self) -> Option<TutorialStep> {
        match self {
            TutorialStep::Welcome => Some(TutorialStep::CameraMovement),
            TutorialStep::CameraMovement => Some(TutorialStep::BirdObservation),
            TutorialStep::BirdObservation => Some(TutorialStep::PhotoMode),
            TutorialStep::PhotoMode => Some(TutorialStep::TakePhoto),
            TutorialStep::TakePhoto => Some(TutorialStep::ViewJournal),
            TutorialStep::ViewJournal => Some(TutorialStep::OpenCatalog),
            TutorialStep::OpenCatalog => Some(TutorialStep::PlaceFeeder),
            TutorialStep::PlaceFeeder => Some(TutorialStep::BirdFeeding),
            TutorialStep::BirdFeeding => Some(TutorialStep::UpgradeFeeder),
            TutorialStep::UpgradeFeeder => Some(TutorialStep::Complete),
            TutorialStep::Complete => None,
        }
    }
}
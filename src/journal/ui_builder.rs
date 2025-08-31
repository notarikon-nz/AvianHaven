use bevy::prelude::*;
use crate::animation::components::BirdSpecies;
use crate::user_interface::styles::{ButtonStyle};
use crate::journal::components::SpeciesButton;

pub fn ui_species_button(
    commands: &mut Commands,
    species: BirdSpecies,
    discovered: bool,
    button_style: &ButtonStyle,
) -> Entity {
    let button_id = commands.spawn((
        Button,
        Node {
            width: button_style.width,
            height: button_style.height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(if discovered { button_style.normal_color } else { Color::srgb(0.05, 0.05, 0.05) }),
        BorderColor(button_style.border_color),
        SpeciesButton(species),
    )).with_children(|parent| {
        let color = if discovered {
            species_color(species)
        } else {
            Color::srgb(0.2, 0.2, 0.2) // Silhouette
        };
        
        parent.spawn((
            Node {
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                ..default()
            },
            BackgroundColor(color),
        ));
        
        if discovered {
            parent.spawn((
                Text::new(&format!("{:?}", species)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
            ));
        }
    }).id();
    
    button_id
}

fn species_color(species: BirdSpecies) -> Color {
    match species {
        BirdSpecies::Cardinal => Color::srgb(0.8, 0.2, 0.2),
        BirdSpecies::BlueJay => Color::srgb(0.2, 0.4, 0.8),
        BirdSpecies::Sparrow => Color::srgb(0.5, 0.4, 0.3),
    }
}
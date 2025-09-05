# Nine-Slice Patch Sprites for UI Widget Backgrounds

## Overview

Nine-slice (9-patch) sprites allow UI elements to scale without distorting their borders. This is particularly useful for buttons, panels, and other UI widgets that need to maintain their visual proportions at different sizes.

In Bevy 0.16, nine-slice functionality is built-in using `NodeImageMode::Sliced` with `TextureSlicer` for UI elements and `SpriteImageMode::Sliced` for sprites.

## How Nine-Slice Works

A nine-slice sprite divides a texture into 9 sections:

```
+-------+-------+-------+
| TL    | Top   | TR    |
+-------+-------+-------+
| Left  |Center | Right |
+-------+-------+-------+
| BL    | Bot   | BR    |
+-------+-------+-------+
```

- **Corners (TL, TR, BL, BR)**: Never stretch, maintain original size
- **Sides (Top, Bottom, Left, Right)**: Stretch or tile in one direction
- **Center**: Stretches or tiles in both directions

## Implementation in Bevy 0.16

### Basic Nine-Slice UI Node

```rust
use bevy::prelude::*;

commands.spawn((
    Node {
        width: Val::Px(200.0),
        height: Val::Px(100.0),
        ..default()
    },
    ImageNode {
        image: asset_server.load("ui/button_9slice.png"),
        image_mode: NodeImageMode::Sliced(TextureSlicer {
            // Border defines the slice lines (in pixels from edges)
            border: BorderRect::all(16.0), // 16px border all around
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0, // Corners never scale beyond original size
        }),
        ..default()
    },
));
```

### Advanced Nine-Slice Configuration

```rust
ImageNode {
    image: asset_server.load("ui/panel_9slice.png"),
    image_mode: NodeImageMode::Sliced(TextureSlicer {
        // Different borders for each side
        border: BorderRect {
            left: 20.0,
            right: 20.0,
            top: 15.0,
            bottom: 15.0,
        },
        center_scale_mode: SliceScaleMode::Tile, // Tile center instead of stretch
        sides_scale_mode: SliceScaleMode::Stretch, // Stretch sides
        max_corner_scale: 1.0,
    }),
    ..default()
}
```

## Integration with Perch UI

### Enhanced Button Component

Create a reusable nine-slice button component:

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct NineSliceButton {
    pub button_type: ButtonType,
    pub size: Vec2,
}

#[derive(Clone, Copy)]
pub enum ButtonType {
    Primary,
    Secondary,
    Danger,
}

impl ButtonType {
    pub fn texture_path(&self) -> &'static str {
        match self {
            Self::Primary => "ui/button_primary_9slice.png",
            Self::Secondary => "ui/button_secondary_9slice.png", 
            Self::Danger => "ui/button_danger_9slice.png",
        }
    }
    
    pub fn border_size(&self) -> f32 {
        match self {
            Self::Primary => 12.0,
            Self::Secondary => 10.0,
            Self::Danger => 14.0,
        }
    }
}

pub fn spawn_nine_slice_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    button_type: ButtonType,
    size: Vec2,
    text: &str,
) -> Entity {
    commands.spawn((
        Button,
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode {
            image: asset_server.load(button_type.texture_path()),
            image_mode: NodeImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(button_type.border_size()),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            }),
            ..default()
        },
        BorderRadius::all(Val::Px(6.0)), // Can still use rounded corners
        NineSliceButton { button_type, size },
    )).with_children(|parent| {
        parent.spawn((
            Text::new(text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    }).id()
}
```

### Panel/Container Component

```rust
#[derive(Component)]
pub struct NineSlicePanel {
    pub panel_type: PanelType,
}

#[derive(Clone, Copy)]
pub enum PanelType {
    Dialog,
    Settings,
    Tooltip,
    Card,
}

impl PanelType {
    pub fn texture_path(&self) -> &'static str {
        match self {
            Self::Dialog => "ui/dialog_panel_9slice.png",
            Self::Settings => "ui/settings_panel_9slice.png",
            Self::Tooltip => "ui/tooltip_9slice.png",
            Self::Card => "ui/card_9slice.png",
        }
    }
    
    pub fn border(&self) -> BorderRect {
        match self {
            Self::Dialog => BorderRect::all(24.0),
            Self::Settings => BorderRect::all(20.0),
            Self::Tooltip => BorderRect::all(8.0),
            Self::Card => BorderRect::all(16.0),
        }
    }
}

pub fn spawn_nine_slice_panel(
    commands: &mut Commands,
    asset_server: &AssetServer,
    panel_type: PanelType,
    size: Vec2,
) -> Entity {
    commands.spawn((
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ImageNode {
            image: asset_server.load(panel_type.texture_path()),
            image_mode: NodeImageMode::Sliced(TextureSlicer {
                border: panel_type.border(),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            }),
            ..default()
        },
        BorderRadius::all(Val::Px(8.0)),
        NineSlicePanel { panel_type },
    )).id()
}
```

## Updating Existing UI Systems

### Enhanced UI Colors with Nine-Slice Support

```rust
// In src/ui_widgets.rs

use bevy::prelude::*;

pub struct NineSliceUIColors;

impl NineSliceUIColors {
    // Texture paths for different UI elements
    pub const BUTTON_PRIMARY: &'static str = "ui/button_primary_9slice.png";
    pub const BUTTON_SECONDARY: &'static str = "ui/button_secondary_9slice.png";
    pub const PANEL_MAIN: &'static str = "ui/panel_main_9slice.png";
    pub const DROPDOWN_BG: &'static str = "ui/dropdown_9slice.png";
    pub const SLIDER_TRACK: &'static str = "ui/slider_track_9slice.png";
    pub const SLIDER_HANDLE: &'static str = "ui/slider_handle_9slice.png";
    
    // Border sizes for consistent scaling
    pub const BUTTON_BORDER: f32 = 12.0;
    pub const PANEL_BORDER: f32 = 16.0;
    pub const DROPDOWN_BORDER: f32 = 8.0;
    pub const SLIDER_BORDER: f32 = 6.0;
}

// Enhanced slider with nine-slice background
#[derive(Component)]
pub struct NineSliceSliderWidget {
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
    pub step: f32,
    pub show_percentage: bool,
}

impl NineSliceSliderWidget {
    pub fn new(min: f32, max: f32, initial: f32) -> Self {
        Self {
            min_value: min,
            max_value: max,
            current_value: initial.clamp(min, max),
            step: 0.01,
            show_percentage: true,
        }
    }
}

pub fn spawn_nine_slice_slider(
    commands: &mut Commands,
    asset_server: &AssetServer,
    slider: NineSliceSliderWidget,
    width: f32,
) -> Entity {
    commands.spawn((
        Node {
            width: Val::Px(width),
            height: Val::Px(20.0),
            position_type: PositionType::Relative,
            ..default()
        },
    )).with_children(|parent| {
        // Track background
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode {
                image: asset_server.load(NineSliceUIColors::SLIDER_TRACK),
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(NineSliceUIColors::SLIDER_BORDER),
                    center_scale_mode: SliceScaleMode::Stretch,
                    sides_scale_mode: SliceScaleMode::Stretch,
                    max_corner_scale: 1.0,
                }),
                ..default()
            },
            SliderTrack,
        ));
        
        // Handle
        let handle_position = (slider.current_value - slider.min_value) / 
                             (slider.max_value - slider.min_value);
        parent.spawn((
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(handle_position * 100.0),
                top: Val::Px(-2.0),
                ..default()
            },
            ImageNode {
                image: asset_server.load(NineSliceUIColors::SLIDER_HANDLE),
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(NineSliceUIColors::SLIDER_BORDER),
                    center_scale_mode: SliceScaleMode::Stretch,
                    sides_scale_mode: SliceScaleMode::Stretch,
                    max_corner_scale: 1.0,
                }),
                ..default()
            },
            SliderHandle,
        ));
    }).insert(slider).id()
}
```

## Asset Creation Guidelines

### Nine-Slice Texture Requirements

1. **Consistent Border Size**: All borders should be the same width for symmetrical elements
2. **Power of 2 Dimensions**: Use textures with power-of-2 dimensions when possible (32x32, 64x64, etc.)
3. **Clear Borders**: Make sure the border areas can be clearly distinguished from the center
4. **Seamless Edges**: Ensure side sections can tile seamlessly if using `SliceScaleMode::Tile`

### Recommended Texture Sizes

- **Small Buttons**: 48x32 with 12px border
- **Large Buttons**: 64x48 with 16px border  
- **Panels**: 96x96 with 24px border
- **Dropdown/Input**: 32x24 with 8px border
- **Sliders**: 128x16 with 6px border

### Folder Structure

```
assets/
├── ui/
│   ├── buttons/
│   │   ├── button_primary_9slice.png
│   │   ├── button_secondary_9slice.png
│   │   └── button_danger_9slice.png
│   ├── panels/
│   │   ├── dialog_panel_9slice.png
│   │   ├── settings_panel_9slice.png
│   │   └── tooltip_9slice.png
│   ├── widgets/
│   │   ├── dropdown_9slice.png
│   │   ├── slider_track_9slice.png
│   │   └── slider_handle_9slice.png
│   └── cards/
│       └── card_9slice.png
```

## Performance Considerations

### GPU vs CPU Rendering

- Bevy 0.16 renders nine-slice sprites on the GPU via shaders, which is much faster than CPU-based approaches
- Nine-slice UI elements have minimal performance impact compared to solid color backgrounds
- Use nine-slice sparingly for non-interactive decorative elements

### Memory Usage

- One nine-slice texture can replace dozens of pre-sized textures
- Shared textures across similar UI elements reduce memory usage
- Consider texture atlasing for multiple nine-slice sprites

## Troubleshooting

### Common Issues

1. **Stretching Artifacts**: Ensure border values match actual texture border sizes
2. **Scaling Issues**: Check `max_corner_scale` value - should usually be 1.0
3. **Sizing Problems**: Remember that `ImageNode` affects node sizing - consider using fixed dimensions
4. **Border Inconsistencies**: Use consistent border sizes across related UI elements

### Debug Tools

```rust
// Add this system for debugging nine-slice borders
pub fn debug_nine_slice_system(
    mut gizmos: Gizmos,
    query: Query<(&Node, &GlobalTransform, &ImageNode)>,
) {
    for (node, transform, image_node) in &query {
        if let NodeImageMode::Sliced(ref slicer) = image_node.image_mode {
            let size = Vec2::new(
                node.width.resolve(0.0, Vec2::ZERO).unwrap_or(100.0),
                node.height.resolve(0.0, Vec2::ZERO).unwrap_or(100.0),
            );
            
            let pos = transform.translation().truncate();
            
            // Draw border lines
            let border = &slicer.border;
            
            // Vertical lines
            gizmos.line_2d(
                Vec2::new(pos.x - size.x/2.0 + border.left, pos.y - size.y/2.0),
                Vec2::new(pos.x - size.x/2.0 + border.left, pos.y + size.y/2.0),
                Color::RED,
            );
            gizmos.line_2d(
                Vec2::new(pos.x + size.x/2.0 - border.right, pos.y - size.y/2.0),
                Vec2::new(pos.x + size.x/2.0 - border.right, pos.y + size.y/2.0),
                Color::RED,
            );
            
            // Horizontal lines  
            gizmos.line_2d(
                Vec2::new(pos.x - size.x/2.0, pos.y - size.y/2.0 + border.bottom),
                Vec2::new(pos.x + size.x/2.0, pos.y - size.y/2.0 + border.bottom),
                Color::RED,
            );
            gizmos.line_2d(
                Vec2::new(pos.x - size.x/2.0, pos.y + size.y/2.0 - border.top),
                Vec2::new(pos.x + size.x/2.0, pos.y + size.y/2.0 - border.top),
                Color::RED,
            );
        }
    }
}
```

This guide provides a comprehensive foundation for implementing nine-slice patch sprites in Perch's UI system, enhancing visual consistency and scalability while maintaining performance.
# Smart Objects Development Guide

## Overview

Smart Objects in Perch are interactive entities that birds can discover and use through their AI system. They provide utility to birds and create interesting emergent behaviors through the three-tier AI architecture:

1. **UtilityQuery** (1s timer) - Discovers objects and calculates utility scores
2. **BehaviorTree** (2s timer) - Decides which objects to target based on bird needs
3. **State Execution** (every frame) - Executes behaviors with the targeted objects

## Core Components

### Required Components

Every smart object must have these components:

```rust
use crate::bird_ai::components::{SmartObject, ProvidesUtility, BirdAction};

// Mark as discoverable by bird AI
SmartObject,

// Define what utility this object provides
ProvidesUtility {
    action: BirdAction::YourAction,  // What action birds can perform
    base_utility: 0.8,               // Base appeal (0.0-1.0)
    range: 100.0,                    // Detection range in world units
},
```

### Basic Entity Setup

```rust
commands.spawn((
    // Visual representation
    Sprite::from_color(Color::srgb(0.5, 0.3, 0.2), Vec2::new(50.0, 50.0)),
    Transform::from_xyz(100.0, 0.0, 0.3),
    
    // Physics (for bird interaction detection)
    RigidBody::Fixed,
    Collider::cuboid(25.0, 25.0),
    Sensor, // Important: allows birds to overlap
    
    // Smart object components
    YourCustomComponent { /* ... */ },
    SmartObject,
    ProvidesUtility {
        action: BirdAction::YourAction,
        base_utility: 0.8,
        range: 100.0,
    },
    
    // Optional: tooltip for debugging/info
    Hoverable::new("Your object description"),
));
```

## Step-by-Step Implementation

### 1. Define Your BirdAction

First, add your new action to the `BirdAction` enum in `src/bird_ai/components.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirdAction {
    // Existing actions...
    YourNewAction,  // Add your action here
}
```

### 2. Add Action Properties

In `src/smart_objects.rs`, extend the `BirdAction` implementation with duration and decay properties:

```rust
impl BirdAction {
    pub fn utility_decay_rate(&self) -> f32 {
        match self {
            // Existing cases...
            Self::YourNewAction => 0.05, // How fast utility decreases after use
        }
    }
    
    pub fn duration_range(&self) -> (f32, f32) {
        match self {
            // Existing cases...
            Self::YourNewAction => (5.0, 15.0), // Min/max duration in seconds
        }
    }
}
```

### 3. Create Your Component

Define a component to hold your object's specific data:

```rust
#[derive(Component)]
pub struct YourSmartObject {
    pub object_type: YourObjectType,
    pub capacity: u32,
    pub current_users: Vec<Entity>,
    pub effectiveness: f32,
    // Add any properties specific to your object
}

#[derive(Debug, Clone, Copy)]
pub enum YourObjectType {
    TypeA,
    TypeB,
    TypeC,
}

impl YourObjectType {
    pub fn base_utility(&self) -> f32 {
        match self {
            Self::TypeA => 0.7,
            Self::TypeB => 0.8,
            Self::TypeC => 0.9,
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            Self::TypeA => Color::srgb(0.5, 0.3, 0.2),
            Self::TypeB => Color::srgb(0.3, 0.5, 0.2),
            Self::TypeC => Color::srgb(0.2, 0.3, 0.5),
        }
    }
}
```

### 4. Add Bird State

Add a corresponding state to the `BirdState` enum in `src/bird_ai/components.rs`:

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum BirdState {
    // Existing states...
    YourNewState,  // State when interacting with your object
}
```

### 5. Implement Behavior Tree Logic

In `src/bird_ai/bt.rs`, add logic for when birds should use your object:

```rust
// Find the evaluate_available_actions function and add your case:
if let Some(entry) = available_actions.get(&BirdAction::YourNewAction) {
    let need_factor = calculate_your_need_factor(&internal); // Implement this
    let utility_score = entry.score * need_factor * environmental_modifiers;
    
    if utility_score > best_utility {
        best_utility = utility_score;
        best_action = Some((BirdAction::YourNewAction, entry.entity));
    }
}

// Add transition logic in the decision function:
if let Some((BirdAction::YourNewAction, target_entity)) = best_action {
    blackboard.current_target = Some(target_entity);
    return BirdState::YourNewState;
}
```

### 6. Add State Execution System

Create a system in `src/bird_ai/systems.rs` to handle your new state:

```rust
pub fn your_object_interaction_system(
    mut bird_query: Query<(Entity, &mut Transform, &mut BirdState, &mut Blackboard), With<BirdAI>>,
    mut object_query: Query<(&Transform, &mut YourSmartObject), (With<SmartObject>, Without<BirdAI>)>,
    time: Res<Time>,
) {
    for (bird_entity, mut bird_transform, mut bird_state, mut blackboard) in bird_query.iter_mut() {
        if matches!(*bird_state, BirdState::YourNewState) {
            if let Some(target_entity) = blackboard.current_target {
                if let Ok((object_transform, mut object)) = object_query.get_mut(target_entity) {
                    let distance = bird_transform.translation.distance(object_transform.translation);
                    
                    if distance > 30.0 {
                        // Move toward object
                        let direction = (object_transform.translation - bird_transform.translation).normalize();
                        bird_transform.translation += direction * 50.0 * time.delta().as_secs_f32();
                    } else {
                        // Interact with object
                        perform_your_interaction(bird_entity, &mut object, &mut blackboard);
                        
                        // Check if interaction is complete
                        if should_finish_interaction(&blackboard) {
                            *bird_state = BirdState::Wandering;
                            blackboard.current_target = None;
                        }
                    }
                } else {
                    // Object no longer exists, return to wandering
                    *bird_state = BirdState::Wandering;
                    blackboard.current_target = None;
                }
            }
        }
    }
}

fn perform_your_interaction(
    bird_entity: Entity,
    object: &mut YourSmartObject,
    blackboard: &mut Blackboard,
) {
    // Implement your specific interaction logic here
    // Update bird's internal state
    // Update object's state
    // Add bird to object's user list if needed
}

fn should_finish_interaction(blackboard: &Blackboard) -> bool {
    // Implement your completion logic
    // Example: check if bird's need is satisfied
    blackboard.internal.your_need < 0.2
}
```

### 7. Add Spawning Function

Create a spawning function in `src/smart_objects.rs`:

```rust
fn spawn_your_objects(commands: &mut Commands) {
    // Example object
    let object_type = YourObjectType::TypeA;
    commands.spawn((
        Sprite::from_color(object_type.color(), Vec2::new(60.0, 40.0)),
        Transform::from_xyz(150.0, 100.0, 0.3),
        RigidBody::Fixed,
        Collider::cuboid(30.0, 20.0),
        Sensor,
        YourSmartObject {
            object_type,
            capacity: 3,
            current_users: Vec::new(),
            effectiveness: 0.8,
        },
        SmartObject,
        ProvidesUtility {
            action: BirdAction::YourNewAction,
            base_utility: object_type.base_utility(),
            range: 80.0,
        },
        Hoverable::new("Description of your object and its function"),
    ));
}
```

### 8. Add Update System (Optional)

If your object needs ongoing updates, create a system:

```rust
pub fn your_object_update_system(
    mut object_query: Query<&mut YourSmartObject>,
    time: Res<Time>,
    // Add other resources as needed
) {
    for mut object in object_query.iter_mut() {
        // Update object state over time
        // Handle capacity, maintenance, seasonal changes, etc.
        object.current_users.clear(); // Reset each frame
        
        // Add your update logic here
    }
}
```

### 9. Register Systems

Add your systems to the plugin in `src/smart_objects.rs`:

```rust
impl Plugin for SmartObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_smart_objects)
            .add_systems(Update, (
                // Existing systems...
                your_object_update_system,
            ).run_if(in_state(crate::AppState::Playing)));
    }
}
```

And update the setup function:

```rust
pub fn setup_smart_objects(mut commands: Commands) {
    // Existing spawning...
    spawn_your_objects(&mut commands);
}
```

### 10. Add to Bird AI Plugin

Register your interaction system in `src/bird_ai/mod.rs`:

```rust
impl Plugin for BirdAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                // Existing systems...
                your_object_interaction_system,
            ).run_if(in_state(AppState::Playing)));
    }
}
```

## Advanced Features

### Species Preferences

Add species-specific preferences to your object:

```rust
#[derive(Component)]
pub struct YourSmartObject {
    // ... existing fields
    pub species_preference: Vec<BirdSpecies>,
    pub species_multipliers: HashMap<BirdSpecies, f32>,
}

// In utility calculation:
let species_multiplier = object.species_multipliers
    .get(&bird.species)
    .unwrap_or(&1.0);
let final_utility = base_utility * species_multiplier;
```

### Seasonal Behavior

Make your object respond to seasons:

```rust
pub fn seasonal_your_object_system(
    time_state: Res<crate::environment::resources::TimeState>,
    mut object_query: Query<&mut YourSmartObject>,
) {
    let season = time_state.get_season();
    
    for mut object in object_query.iter_mut() {
        object.effectiveness = match season {
            Season::Spring => 1.2,
            Season::Summer => 1.0,
            Season::Fall => 0.8,
            Season::Winter => 0.6,
        };
    }
}
```

### Multi-Action Objects

Objects can provide multiple actions:

```rust
// In spawning, create multiple ProvidesUtility components
for action in [BirdAction::Drink, BirdAction::Bathe] {
    commands.spawn((
        // ... other components
        ProvidesUtility {
            action,
            base_utility: action.base_utility(),
            range: 80.0,
        },
    ));
}
```

## Best Practices

1. **Start Simple**: Begin with basic functionality, add complexity gradually
2. **Use Existing Patterns**: Follow the patterns established by feeders and water features
3. **Consider Balance**: Utility values should be balanced with existing objects
4. **Add Tooltips**: Always include `Hoverable` for debugging and player information
5. **Test Interactions**: Ensure birds can pathfind to and use your objects properly
6. **Handle Edge Cases**: What happens if the object is destroyed while in use?
7. **Performance**: Avoid expensive operations in update systems that run every frame

## Common Pitfalls

- **Forgetting Sensor**: Without `Sensor`, birds can't overlap with your object
- **Wrong Collider Size**: Make sure colliders match visual representation
- **Missing State Transitions**: Always provide a way to exit your new bird state
- **Infinite Loops**: Ensure interaction completion conditions are reachable
- **Z-fighting**: Use appropriate Z values for layering (0.1-0.5 range typically)

## Debugging Tips

1. **Use Console Logs**: Add `info!()` statements to track bird decisions
2. **Visual Debugging**: Draw debug circles for utility ranges
3. **State Inspection**: Monitor bird state changes in development
4. **Utility Scores**: Log utility calculations to ensure they're reasonable

This guide provides the foundation for creating rich, interactive smart objects that integrate seamlessly with Perch's bird AI system.
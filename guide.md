# Bird AI Integration Guide

## System Architecture

**Flow**: `UtilityQuery` → `BehaviorTree` → `StateExecution`

- **UtilityQuery** (1s timer): Scans world objects, updates `Blackboard.available_actions`
- **BehaviorTree** (2s timer): Reads needs + world knowledge → decides state
- **State Systems** (every frame): Execute current `BirdState` behavior

## Adding New Birds

```rust
commands.spawn((
    Sprite::from_color(Color::RED, Vec2::new(20.0, 20.0)),
    Transform::from_xyz(x, y, 1.0),
    BirdAI,
    BirdState::Wandering,
    Blackboard {
        internal: InternalState {
            hunger: 0.7,
            thirst: 0.3,
            energy: 0.8,
            fear: 0.0,
        },
        ..default()
    },
));
```

## Adding Smart Objects

```rust
commands.spawn((
    Sprite::from_color(Color::BROWN, Vec2::new(30.0, 40.0)),
    Transform::from_xyz(x, y, 0.5),
    SmartObject,
    ProvidesUtility {
        action: BirdAction::Eat,  // or Drink, Bathe, Perch
        base_utility: 0.9,       // 0.0-1.0 attraction strength
        range: 200.0,            // detection radius
    },
));
```

## Notifications
```rust
  // Send notification
  notification_events.write(ShowNotificationEvent {
      notification: NotificationType::Achievement {
          title: "First Photo".to_string(),
          description: "Take your first photo".to_string(),
          currency_reward: 25,
      },
  });
```

## Extending System

**New Action**: Add to `BirdAction` enum, create corresponding `BirdState`, implement execution system
**New Behavior**: Add state to `BirdState` enum, implement in `states.rs`, add system in `systems.rs`
**New Need**: Add field to `InternalState`, update `need_decay_system` and `bt.rs` logic

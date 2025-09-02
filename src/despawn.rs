use bevy::prelude::*;

#[derive(Component)]
pub struct PendingDespawn {
    pub delay: f32,
}

// Safer despawning system - prevents double despawns and crashes
pub fn robust_despawn_system(
    mut commands: Commands,
    mut pending_query: Query<(Entity, &mut PendingDespawn)>,
    time: Res<Time>,
) {
    let mut entities_to_despawn = Vec::new();
    
    // Process pending despawns with delay for safety
    for (entity, mut pending) in pending_query.iter_mut() {
        pending.delay -= time.delta_secs();
        
        if pending.delay <= 0.0 {
            entities_to_despawn.push(entity);
        }
    }
    
    // Safely despawn entities that are ready
    for entity in entities_to_despawn {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn();
        }
    }
}

// Safe despawn helper - use instead of direct .despawn()
pub trait SafeDespawn {
    fn safe_despawn(&mut self) -> &mut Self;
    fn safe_despawn_delayed(&mut self, delay: f32) -> &mut Self;
}

impl SafeDespawn for EntityCommands<'_> {
    fn safe_despawn(&mut self) -> &mut Self {
        self.try_insert(PendingDespawn { delay: 0.016 }); // One frame delay
        self
    }
    
    fn safe_despawn_delayed(&mut self, delay: f32) -> &mut Self {
        self.try_insert(PendingDespawn { delay });
        self
    }
}
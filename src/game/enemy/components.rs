use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // why does the enemy needs to have a direction, why not have another component direction
}

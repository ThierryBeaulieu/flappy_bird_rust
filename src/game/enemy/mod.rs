use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::game::SimulationState;
use crate::AppState;

pub const NUMBER_OF_ENEMIES: usize = 5;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Init
            .init_resource::<EnemySpawnTimer>()
            // Startup
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            // Update
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement,
                    update_enemy_direction,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}

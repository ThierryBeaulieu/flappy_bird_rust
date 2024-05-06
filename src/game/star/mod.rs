use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size
pub const NUMBER_OF_STARS: usize = 10;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;
pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_star);
    }
}

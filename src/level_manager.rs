use crate::dot::SpawnDot;
use crate::game::{CLEAR_COLOR_MISS, GameProgress, GameState, GameStateCooldown};
use bevy::prelude::*;

#[derive(Event)]
pub struct DotHit;

#[derive(Event)]
pub struct DotMissed;

pub struct LevelManagerPlugin;
impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_dot_hit)
            .add_observer(on_dot_missed)
            .add_systems(OnEnter(GameState::GameOver), on_game_over)
            .add_systems(
                Update,
                (
                    on_input_go_to(GameState::Playing, false).
                        run_if(in_state(GameState::Idle)),
                    on_input_go_to(GameState::Resetting, true)
                        .run_if(in_state(GameState::LevelCleared)),
                    on_input_go_to(GameState::Resetting, true)
                        .run_if(in_state(GameState::GameOver)),
                ),
            );
    }
}

fn on_dot_hit(
    _: On<DotHit>,
    mut commands: Commands,
    mut progress: ResMut<GameProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    progress.hits_remaining -= 1;
    if progress.hits_remaining == 0 {
        next_state.set(GameState::LevelCleared);
        progress.level_up();
    } else {
        commands.trigger(SpawnDot);
    }
}

fn on_dot_missed(
    _: On<DotMissed>,
    mut progress: ResMut<GameProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    progress.reset();
    next_state.set(GameState::GameOver);
}

fn on_game_over(mut clear_color: ResMut<ClearColor>, mut cooldown: ResMut<GameStateCooldown>) {
    *clear_color = ClearColor(CLEAR_COLOR_MISS);
    cooldown.0.reset();
}

fn on_input_go_to(
    state: GameState,
    cooldown: bool,
) -> impl Fn(
    Res<ButtonInput<KeyCode>>,
    Res<ButtonInput<MouseButton>>,
    ResMut<NextState<GameState>>,
    ResMut<GameStateCooldown>,
    Res<Time>,
    ResMut<ClearColor>,
) {
    move |keys, mouse, mut next_state, mut cd, time, mut clear_color| {
        if cooldown {
            cd.0.tick(time.delta());
            if !cd.0.is_finished() {
                return;
            }
        }

        let pressed = keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left);
        if !pressed {
            return;
        }

        next_state.set(state.clone());
        if cooldown {
            cd.0.reset();
        }
    }
}

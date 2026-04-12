use crate::components::dot::SpawnDot;
use crate::game::{CLEAR_COLOR_MISS, GameProgress, GameState, GameStateCooldown};
use bevy::prelude::*;
use crate::managers::{HidePrompt, ShowPrompt, UpdateLevelText, UpdateStarsText};
use crate::managers::audio_manager::PlayUnlockSfx;

pub struct LevelManagerPlugin;
impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_dot_hit)
            .add_observer(on_dot_missed)
            .add_systems(OnEnter(GameState::GameOver), on_game_over)
            .add_systems(
                Update,
                (
                    on_input_go_to(GameState::Playing, false).run_if(in_state(GameState::Idle)),
                    on_input_go_to(GameState::Resetting, true)
                        .run_if(in_state(GameState::LevelCleared)),
                    on_input_go_to(GameState::Resetting, true)
                        .run_if(in_state(GameState::GameOver)),
                ),
            );
    }
}

#[derive(Event)]
pub struct DotHit {
    pub should_score: bool,
}

#[derive(Event)]
pub struct DotMissed;

fn on_dot_hit(
    hit: On<DotHit>,
    mut commands: Commands,
    mut progress: ResMut<GameProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if hit.should_score {
        progress.score += 1;
        commands.trigger(UpdateStarsText);
    }
    progress.hits_remaining -= 1;
    commands.trigger(UpdateLevelText(progress.hits_remaining));

    if progress.hits_remaining == 0 {
        next_state.set(GameState::LevelCleared);
        progress.level_up();
        commands.trigger(PlayUnlockSfx);
    } else {
        commands.trigger(SpawnDot);
    }
}

fn on_dot_missed(
    _: On<DotMissed>,
    mut commands: Commands,
    mut progress: ResMut<GameProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    progress.reset();
    next_state.set(GameState::GameOver);
    commands.trigger(ShowPrompt("TRY AGAIN"))
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
    Commands,
) {
    move |keys, mouse, mut next_state, mut cd, time, mut commands| {
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

        commands.trigger(HidePrompt);
        next_state.set(state.clone());
        if cooldown {
            cd.0.reset();
        }
    }
}

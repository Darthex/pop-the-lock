use crate::game::{GameAssets, GameStartup};
use crate::managers::{DotHit, DotMissed};
use bevy::prelude::*;

pub struct AudioManagerPlugin;
impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bgs.after(GameStartup))
            .add_observer(on_dot_hit_audio)
            .add_observer(on_dot_missed_audio)
            .add_observer(on_unlock_audio);
    }
}

#[derive(Event)]
pub struct PlayUnlockSfx;

fn spawn_bgs(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        AudioPlayer::new(game_assets.background_score_sfx.clone()),
        PlaybackSettings::LOOP,
    ));
}

fn on_dot_hit_audio(hit: On<DotHit>, mut commands: Commands, game_assets: Res<GameAssets>) {
    let sound = if hit.should_score {
        game_assets.star_pop_sfx.clone()
    } else {
        game_assets.dot_pop_sfx.clone()
    };

    commands.spawn(AudioPlayer::new(sound));
}

fn on_dot_missed_audio(_: On<DotMissed>, mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(AudioPlayer::new(game_assets.game_over_sfx.clone()));
}

fn on_unlock_audio(_: On<PlayUnlockSfx>, mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(AudioPlayer::new(game_assets.game_won_sfx.clone()));
}

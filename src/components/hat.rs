use crate::game::{GameAssets, GameCleanup, GameStartup, GameState, Lock, RING_RADIUS, RING_SIZE};
use bevy::prelude::*;
use my_macros::serialize;

const HAT_WIDTH: f32 = 721.0;
pub const HAT_HEIGHT: f32 = 878.0;
pub const HAT_SCALE: f32 = RING_SIZE / 1201.0;
pub const HAT_OFFSET: f32 = 80.0;
const HAT_ANIMATE_SPEED: f32 = 700.0; // pixels per second
const HAT_ANIMATE_DISTANCE: f32 = 100.0;

pub struct LockHatPlugin;
impl Plugin for LockHatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lock_hat.after(GameStartup))
            .add_systems(
                OnEnter(GameState::Resetting),
                spawn_lock_hat.after(GameCleanup),
            )
            .add_systems(
                Update,
                animate_hat.run_if(in_state(GameState::LevelCleared)),
            );
        LockHat::register(app);
    }
}

#[serialize]
struct LockHat;

fn spawn_lock_hat(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    lock: Single<Entity, With<Lock>>,
) {
    let y = RING_RADIUS * 2.0 - HAT_OFFSET;
    commands.entity(*lock).with_children(|parent| {
        parent.spawn((
            Sprite {
                custom_size: Some(Vec2::new(HAT_WIDTH * HAT_SCALE, HAT_HEIGHT * HAT_SCALE)),
                image: game_assets.lock_hat_asset.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, y, -1.0),
            LockHat,
            Name::new("Lock Hat"),
        ));
    });
}

fn animate_hat(hat: Single<&mut Transform, With<LockHat>>, time: Res<Time>) {
    let mut transform = hat.into_inner();
    let target_y = RING_RADIUS * 2.0 - HAT_OFFSET + HAT_ANIMATE_DISTANCE;

    if transform.translation.y < target_y {
        transform.translation.y += HAT_ANIMATE_SPEED * time.delta_secs();
        transform.translation.y = transform.translation.y.min(target_y);
    }
}

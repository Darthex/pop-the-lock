use crate::game::{GameAssets, Lock, RING_RADIUS, RING_SIZE};
use crate::trigger::Trigger;
use crate::utils::get_safe_dot_angle;
use bevy::prelude::*;
use my_macros::serialize;

const DOT_SIZE: f32 = RING_SIZE * (121.0 / 1201.0);
pub const DOT_ANGLE_SIZE: f32 = DOT_SIZE / RING_RADIUS;

pub struct DotPlugin;
impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_dot);
    }
}

#[derive(Event, Debug)]
pub struct SpawnDot;

#[serialize]
pub struct TargetDot {
    pub loc: f32,
}

pub fn spawn_dot(
    _: On<SpawnDot>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    lock: Single<Entity, With<Lock>>,
    trigger: Option<Single<&Trigger>>,
) {
    let trigger_angle = trigger.as_ref().map(|t| t.angle).unwrap_or(0.0);
    let direction = trigger.as_ref().map(|t| t.direction.0).unwrap_or(1.0);

    let angle = get_safe_dot_angle(trigger_angle, direction);
    let x = angle.cos() * RING_RADIUS;
    let y = angle.sin() * RING_RADIUS;

    commands.entity(*lock).with_children(|parent| {
        parent.spawn((
            Sprite {
                custom_size: Some(Vec2::splat(DOT_SIZE)),
                image: game_assets.dot_asset.clone(),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
            TargetDot { loc: angle },
            Name::new("Dot"),
        ));
    });
}
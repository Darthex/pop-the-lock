use crate::game::{Lock, RING_SIZE};
use crate::level_manager::DotMissed;
use bevy::prelude::*;

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_dot_missed_shake)
            .add_systems(Update, tick_shake);
    }
}

#[derive(Component)]
struct Shake {
    timer: Timer,
    origin: Vec3,
}

fn on_dot_missed_shake(
    _: On<DotMissed>,
    mut commands: Commands,
    lock: Single<(Entity, &Transform), With<Lock>>,
) {
    let (entity, transform) = lock.into_inner();
    commands.entity(entity).insert(Shake {
        timer: Timer::from_seconds(0.5, TimerMode::Once),
        origin: transform.translation,
    });
}

fn tick_shake(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Shake)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut shake) in &mut query {
        shake.timer.tick(time.delta());

        if shake.timer.is_finished() {
            transform.translation = shake.origin;
            transform.rotation = Quat::IDENTITY;
            commands.entity(entity).remove::<Shake>();
        } else {
            let progress = shake.timer.elapsed_secs();
            let frequency = 18.0;
            let amplitude = 0.15 * (1.0 - shake.timer.fraction()); // fades out in radians

            let angle = (progress * frequency).sin() * amplitude;

            // pivot from top — rotate around a point above the lock
            let pivot_offset = RING_SIZE * 0.5; // top of the lock
            transform.rotation = Quat::from_rotation_z(angle);
            // offset translation to simulate pivot at top
            transform.translation.x = shake.origin.x + angle * pivot_offset;
            transform.translation.y = shake.origin.y - (1.0 - angle.cos()) * pivot_offset;
        }
    }
}
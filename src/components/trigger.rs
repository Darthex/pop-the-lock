use crate::components::dot::{DOT_ANGLE_SIZE, TargetDot};
use crate::game::{GameAssets, GameCleanup, GameStartup, GameState, Lock, RING_RADIUS, RING_SIZE};
use crate::level_manager::{DotHit, DotMissed};
use crate::utils::random_choice;
use bevy::prelude::*;
use my_macros::serialize;

const PICK_WIDTH: f32 = 56.0;
const PICK_HEIGHT: f32 = 172.0;
const PICK_SCALE: f32 = RING_SIZE / 1201.0;
const PICK_SPEED: f32 = 4.0;

pub struct TriggerPlugin;
impl Plugin for TriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_trigger.after(GameStartup))
            .add_systems(
                OnEnter(GameState::Resetting),
                spawn_trigger.after(GameCleanup),
            )
            .add_systems(
                FixedUpdate,
                rotate_trigger.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, check_input.run_if(in_state(GameState::Playing)));
        Trigger::register(app);
        Direction::register(app);
    }
}

#[serialize]
pub struct Trigger {
    pub angle: f32,
    pub speed: f32,
    pub direction: Direction,
    pub inside_dot: bool,
    pub exited_dot: bool,
    pub clicked_during_overlap: bool,
}

#[serialize]
pub struct Direction(pub f32);
impl Direction {
    pub fn negate(&mut self) {
        self.0 = -self.0;
    }
}

fn spawn_trigger(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    lock: Single<Entity, With<Lock>>,
) {
    commands.entity(*lock).with_children(|parent| {
        parent.spawn((
            Sprite {
                custom_size: Some(Vec2::new(PICK_WIDTH * PICK_SCALE, PICK_HEIGHT * PICK_SCALE)),
                image: game_assets.lock_pick_asset.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, RING_RADIUS, 2.0),
            Trigger {
                angle: std::f32::consts::FRAC_PI_2,
                speed: PICK_SPEED,
                direction: Direction(*random_choice(&[1.0f32, -1.0])),
                ..default()
            },
            Name::new("Pick"),
        ));
    });
}

fn rotate_trigger(
    trigger: Single<(&mut Trigger, &mut Transform)>,
    dot: Single<&TargetDot>,
    time: Res<Time>,
) {
    let (mut trig, mut transform) = trigger.into_inner();
    trig.angle += trig.speed * time.delta_secs() * trig.direction.0;

    let x = trig.angle.cos() * RING_RADIUS;
    let y = trig.angle.sin() * RING_RADIUS;
    transform.translation = Vec3::new(x, y, 2.0);
    transform.rotation = Quat::from_rotation_z(trig.angle - std::f32::consts::FRAC_PI_2);

    let diff = {
        let trigger_angle = trig.angle.rem_euclid(std::f32::consts::TAU);
        let dot_angle = dot.loc.rem_euclid(std::f32::consts::TAU);
        let d = (trigger_angle - dot_angle).abs();
        d.min(std::f32::consts::TAU - d)
    };

    let was_inside = trig.inside_dot;
    trig.inside_dot = diff <= DOT_ANGLE_SIZE;

    if was_inside && !trig.inside_dot {
        if !trig.clicked_during_overlap {
            trig.exited_dot = true;
        }
        trig.clicked_during_overlap = false;
    }
}

fn check_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut trigger: Single<&mut Trigger>,
    dot: Single<(Entity, &TargetDot)>,
) {
    // exited without clicking
    if trigger.exited_dot {
        trigger.exited_dot = false;
        commands.trigger(DotMissed);
        return;
    }

    let pressed = keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left);
    if !pressed {
        return;
    }

    if trigger.inside_dot {
        // hit
        let (dot_entity, data) = dot.into_inner();
        trigger.clicked_during_overlap = true;
        trigger.speed += 0.2;
        trigger.inside_dot = false;
        trigger.direction.negate();
        commands.entity(dot_entity).despawn();
        commands.trigger(DotHit {
            should_score: data.is_star,
        });
    } else {
        // clicked outside
        commands.trigger(DotMissed);
    }
}

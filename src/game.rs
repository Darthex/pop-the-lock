use crate::dot::SpawnDot;
use crate::utils::random_choice;
use bevy::prelude::*;
use my_macros::serialize;

pub const RING_SIZE: f32 = 500.0; // 350
pub const RING_RADIUS: f32 = RING_SIZE * (500.0 / 1201.0);

const CLEAR_COLOR_MISS: Color = Color::srgb_u8(199, 78, 81);
const CLEAR_COLORS: [Color; 5] = [
    Color::srgb_u8(56, 73, 101),
    Color::srgb_u8(255, 196, 155),
    Color::srgb_u8(137, 183, 194),
    Color::srgb_u8(168, 191, 138),
    Color::srgb_u8(170, 136, 187),
];

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameStartup;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameCleanup;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Idle,
    Playing,
    LevelCleared,
    GameOver,
    Resetting,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                Startup,
                (load_resources, spawn_scene).chain().in_set(GameStartup),
            )
            .add_systems(
                OnEnter(GameState::Resetting),
                (cleanup_scene, spawn_scene).chain().in_set(GameCleanup),
            )
            .add_systems(
                OnEnter(GameState::Resetting),
                transition_to_idle.after(GameCleanup),
            )
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
        Lock::register(app);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub dot_asset: Handle<Image>,
    dot_point_asset: Handle<Image>,
    lock_frame_asset: Handle<Image>,
    pub lock_hat_asset: Handle<Image>,
    pub lock_pick_asset: Handle<Image>,
}

#[derive(Resource)]
pub struct GameProgress {
    pub level: u32,
    pub hits_remaining: u32,
}

impl GameProgress {
    fn new() -> Self {
        Self {
            level: 1,
            hits_remaining: 1,
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.hits_remaining = self.level;
    }

    pub fn reset(&mut self) {
        self.hits_remaining = self.level;
    }
}

#[derive(Resource)]
pub struct GameStateCooldown(Timer);

impl Default for GameStateCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        dot_asset: asset_server.load("dot.png"),
        dot_point_asset: asset_server.load("dot_point.png"),
        lock_frame_asset: asset_server.load("frame.png"),
        lock_hat_asset: asset_server.load("hat.png"),
        lock_pick_asset: asset_server.load("pick.png"),
    });
    commands.insert_resource(GameProgress::new());
    commands.insert_resource(GameStateCooldown::default());
}

#[serialize]
pub struct Lock;

pub fn spawn_scene(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut clear_color: ResMut<ClearColor>,
) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(RING_SIZE)),
            image: game_assets.lock_frame_asset.clone(),
            ..default()
        },
        Transform::from_xyz(0.0, -220.0, 0.0),
        Lock,
        Name::new("Lock"),
    ));
    commands.trigger(SpawnDot);

    *clear_color = ClearColor(*random_choice(&CLEAR_COLORS));
}

fn cleanup_scene(mut commands: Commands, lock: Single<Entity, With<Lock>>) {
    commands.entity(*lock).despawn(); // despawns lock + all children (dot, trigger)
}

fn transition_to_idle(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Idle);
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
) {
    move |keys, mouse, mut next_state, mut cd, time| {
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

fn on_game_over(mut clear_color: ResMut<ClearColor>) {
    *clear_color = ClearColor(CLEAR_COLOR_MISS);
}

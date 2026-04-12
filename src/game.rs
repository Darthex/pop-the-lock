use crate::components::dot::SpawnDot;
use crate::managers::spawn_world_ui;
use crate::utils::random_choice;
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use my_macros::serialize;

pub const RING_SIZE: f32 = 500.0;
pub const RING_RADIUS: f32 = RING_SIZE * (500.0 / 1201.0);

pub const CLEAR_COLOR_MISS: Color = Color::srgb_u8(199, 78, 81);
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
            .insert_resource(PkvStore::new("cxc", "pop-the-lock"))
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
            );
        Lock::register(app);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    // sprites
    pub dot_asset: Handle<Image>,
    pub dot_point_asset: Handle<Image>,
    pub lock_frame_asset: Handle<Image>,
    pub lock_hat_asset: Handle<Image>,
    pub lock_pick_asset: Handle<Image>,
    // sounds
    pub background_score_sfx: Handle<AudioSource>,
    pub star_pop_sfx: Handle<AudioSource>,
    pub dot_pop_sfx: Handle<AudioSource>,
    pub game_over_sfx: Handle<AudioSource>,
    pub game_won_sfx: Handle<AudioSource>,
    // fonts
    pub base_font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

#[derive(Resource)]
pub struct GameProgress {
    level: u32,
    score: u32,
    hits_remaining: u32,
}

impl GameProgress {
    fn new(pkv: Res<PkvStore>) -> Self {
        let level = pkv.get("level").unwrap_or(1);
        let score = pkv.get("score").unwrap_or(0);
        Self {
            level,
            score,
            hits_remaining: level,
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn hits_remaining(&self) -> u32 {
        self.hits_remaining
    }

    pub fn decrease_hits(&mut self) {
        self.hits_remaining -= 1;
    }

    pub fn level_up(&mut self, pkv: &mut PkvStore) {
        self.level += 1;
        self.hits_remaining = self.level;

        pkv.set("level", &self.level).expect("Failed to save level");
    }

    pub fn score_up(&mut self, pkv: &mut PkvStore) {
        self.score += 1;
        pkv.set("score", &self.score).expect("Failed to save score");
    }

    pub fn reset(&mut self) {
        self.hits_remaining = self.level;
    }
}

#[derive(Resource)]
pub struct GameStateCooldown(pub Timer);

impl Default for GameStateCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

#[serialize]
pub struct Lock;

fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>, pkv: Res<PkvStore>) {
    commands.insert_resource(GameAssets {
        dot_asset: asset_server.load("sprites/dot.png"),
        dot_point_asset: asset_server.load("sprites/dot_point.png"),
        lock_frame_asset: asset_server.load("sprites/frame.png"),
        lock_hat_asset: asset_server.load("sprites/hat.png"),
        lock_pick_asset: asset_server.load("sprites/pick.png"),
        background_score_sfx: asset_server.load("sounds/background.mp3"),
        star_pop_sfx: asset_server.load("sounds/starPop.mp3"),
        dot_pop_sfx: asset_server.load("sounds/dotPopBoosted.mp3"),
        game_over_sfx: asset_server.load("sounds/lostBoosted.mp3"),
        game_won_sfx: asset_server.load("sounds/unlockBoosted.mp3"),
        base_font: asset_server.load("fonts/Coiny-Regular.ttf"),
        icon_font: asset_server.load("fonts/NotoSansSymbols2-Regular.ttf"),
    });
    commands.insert_resource(GameProgress::new(pkv));
    commands.insert_resource(GameStateCooldown::default());
}

pub fn spawn_scene(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    progress: Res<GameProgress>,
    mut clear_color: ResMut<ClearColor>,
) {
    commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::splat(RING_SIZE)),
                image: game_assets.lock_frame_asset.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, -220.0, 0.0),
            Lock,
            Name::new("Lock"),
        ))
        .with_children(|parent| {
            spawn_world_ui(parent, &game_assets, &progress);
        });
    commands.trigger(SpawnDot);
    *clear_color = ClearColor(*random_choice(&CLEAR_COLORS));
}

fn cleanup_scene(mut commands: Commands, lock: Single<Entity, With<Lock>>) {
    commands.entity(*lock).despawn();
}

fn transition_to_idle(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Idle);
}

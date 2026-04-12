use crate::components::{HAT_HEIGHT, HAT_OFFSET, HAT_SCALE};
use crate::game::{GameAssets, GameProgress, GameStartup, RING_RADIUS};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use my_macros::serialize;

const PROMPT_Y: f32 = RING_RADIUS * 2.0 - HAT_OFFSET + (HAT_HEIGHT * HAT_SCALE / 2.0) + 120.0;

#[derive(Event)]
pub struct ShowPrompt(pub &'static str);

#[derive(Event)]
pub struct HidePrompt;

#[derive(Event)]
pub struct UpdateStarsText;

#[derive(Event)]
pub struct UpdateLevelText(pub u32);

pub struct UiManagerPlugin;
impl Plugin for UiManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PromptTimer>()
            .add_systems(Startup, spawn_screen_ui.after(GameStartup))
            .add_systems(Update, tick_prompt_timer)
            .add_observer(on_show_prompt)
            .add_observer(on_hide_prompt)
            .add_observer(on_update_stars)
            .add_observer(on_update_level);

        PromptText::register(app);
        StarsText::register(app);
        LevelText::register(app);
    }
}

#[serialize]
pub struct PromptText;
#[serialize]
pub struct StarsText;
#[serialize]
pub struct LevelText;

#[derive(Resource)]
struct PromptTimer {
    timer: Timer,
    pending_text: String,
}

impl Default for PromptTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.5, TimerMode::Once);
        timer.tick(std::time::Duration::from_secs_f32(0.5));
        Self {
            timer,
            pending_text: String::new(),
        }
    }
}

fn spawn_screen_ui(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("★"),
                    TextFont {
                        font: game_assets.icon_font.clone(),
                        font_size: 48.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextSpan::new("0"),
                        TextFont {
                            font: game_assets.base_font.clone(),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        StarsText,
                    ));
                });
        });
}

// called from spawn_scene in game.rs to spawn world space UI as children of lock
pub fn spawn_world_ui(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    game_assets: &GameAssets,
    progress: &GameProgress,
) {
    parent.spawn((
        Text2d::new(progress.hits_remaining.to_string()),
        TextFont {
            font: game_assets.base_font.clone(),
            font_size: 160.0,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        Transform::from_xyz(0.0, 0.0, 3.0),
        LevelText,
    ));

    parent.spawn((
        Text2d::new("POP THE LOCK"),
        TextFont {
            font: game_assets.base_font.clone(),
            font_size: 90.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, PROMPT_Y, 3.0),
        Visibility::Visible,
        PromptText,
    ));
}

fn on_show_prompt(trigger: On<ShowPrompt>, mut timer: ResMut<PromptTimer>) {
    timer.timer.reset();
    timer.pending_text = trigger.0.to_string();
}

fn on_hide_prompt(_: On<HidePrompt>, prompt: Option<Single<&mut Visibility, With<PromptText>>>) {
    if let Some(mut vis) = prompt {
        **vis = Visibility::Hidden;
    }
}

fn on_update_stars(
    _: On<UpdateStarsText>,
    mut stars: Single<&mut Text, With<StarsText>>,
    progress: Res<GameProgress>,
) {
    stars.0 = progress.score.to_string();
}

fn on_update_level(
    trigger: On<UpdateLevelText>,
    level: Option<Single<&mut Text2d, With<LevelText>>>,
) {
    if let Some(mut text) = level {
        text.0 = trigger.0.to_string();
    }
}

fn tick_prompt_timer(
    mut timer: ResMut<PromptTimer>,
    time: Res<Time>,
    prompt: Option<Single<(&mut Text2d, &mut Visibility), With<PromptText>>>,
) {
    if timer.timer.is_finished() {
        return;
    }
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        if let Some(p) = prompt {
            let (mut text, mut vis) = p.into_inner();
            text.0 = timer.pending_text.clone();
            *vis = Visibility::Visible;
        }
    }
}

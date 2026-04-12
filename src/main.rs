mod components;
mod game;
mod managers;
mod utils;

use crate::components::animation::AnimationPlugin;
use crate::components::*;
use crate::game::*;
use crate::managers::*;
use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

const CANVAS_SCALE: u32 = 2;
const CANVAS_HEIGHT: u32 = 720 * CANVAS_SCALE;
const CANVAS_WIDTH: u32 = 1280 * CANVAS_SCALE;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(170, 136, 187)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Pop The Lock"),
                        resolution: WindowResolution::new(CANVAS_WIDTH, CANVAS_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // inspector plugins
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        // game plugins
        .add_plugins(GamePlugin)
        .add_plugins(LevelManagerPlugin)
        .add_plugins(UiManagerPlugin)
        .add_plugins(AudioManagerPlugin)
        .add_plugins(DotPlugin)
        .add_plugins(TriggerPlugin)
        .add_plugins(LockHatPlugin)
        .add_plugins(AnimationPlugin)
        //
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d,));
}

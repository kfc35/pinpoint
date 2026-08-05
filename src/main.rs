use bevy::{
    DefaultPlugins,
    asset::{AssetMetaCheck, AssetPlugin},
    image::{ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
    reflect::{Reflect, std_traits::ReflectDefault},
    settings::{ReflectSettingsGroup, SaveSettingsSync, SettingsGroup, SettingsPlugin},
};
use chrono::Utc;

mod create;
mod grid_axes;
pub(crate) use grid_axes::axes_descriptions;
mod menu;
mod ui;

pub const SETTINGS_APP_NAME: &'static str = "com.github.kfc35.pinpoint";

/// States that the app can transition between that trigger the whole screen to change.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum AppState {
    #[default]
    Menu,
    Create,
    // Play,
}

/// Data about the date and time that this game was initialized.
/// Used to determine what [`grid_axes::Axes`] are used for this session.
#[derive(Resource)]
pub(crate) struct StartDateTime {
    /// Current date as "%Y/%m/%d"
    date: String,
    /// Current time as "%H:%M:%S%.3f"
    time: String,
}

/// The user's name provided for this session.
/// The string within this resource is always valid.
#[derive(Resource, Reflect, Clone, Default, Deref, DerefMut, SettingsGroup)]
#[reflect(Resource, Default, SettingsGroup)]
pub(crate) struct Username(String);

impl Username {
    /// Returns whether the name is valid (at least 1 character, alphamumeric incl. underscore, max 10 characters)
    pub(crate) fn is_valid(name: &String) -> bool {
        name.len() <= 10
            && !name.is_empty()
            && name.chars().all(|c| char::is_alphanumeric(c) || c == '_')
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    // All of the assets are pixel art, so pixelated looks best.
                    default_sampler: ImageSamplerDescriptor::nearest(),
                })
                .set(AssetPlugin {
                    // Prevent 404's from happening on the web.
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }),
        )
        .init_state::<AppState>()
        .init_resource::<Username>()
        .add_plugins(SettingsPlugin::new(SETTINGS_APP_NAME))
        .add_plugins((menu::MenuPlugin, create::CreatePlugin))
        .add_systems(
            Startup,
            (setup, create::init_created_round, create::setup_create).chain(),
        )
        .add_systems(Startup, menu::setup_menu.after(setup))
        .add_systems(OnEnter(AppState::Menu), menu::show_menu)
        .add_systems(OnExit(AppState::Menu), menu::hide_menu)
        .add_systems(OnEnter(AppState::Create), create::show_create)
        .add_systems(OnExit(AppState::Create), create::hide_create)
        .run();
}

fn setup(mut commands: Commands, mut username: ResMut<Username>) {
    commands.spawn(Camera2d);

    // The game will change categories every day in Eastern time.
    let date_time = Utc::now().with_timezone(&chrono_tz::US::Eastern);
    let date = format!("{}", date_time.format("%Y/%m/%d"));
    let time = format!("{}", date_time.format("%H:%M:%S%.3f"));

    commands.insert_resource(StartDateTime { date, time });

    // This can happen if the user manually edits the settings file.
    if !Username::is_valid(&username.0) {
        username.0.clear();
        commands.queue(SaveSettingsSync::IfChanged);
    }
}

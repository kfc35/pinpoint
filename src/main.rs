use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use bevy::{
    DefaultPlugins,
    asset::{AssetMetaCheck, AssetPlugin},
    image::{ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
    reflect::{Reflect, serde::TypedReflectDeserializer, std_traits::ReflectDefault},
    settings::{ReflectSettingsGroup, SaveSettingsSync, SettingsGroup, SettingsPlugin},
};
use chrono::Utc;
use serde::de::DeserializeSeed;

mod animation;
mod grid_axes;
mod ui;
pub(crate) use grid_axes::axes_descriptions;

mod create;
mod how_to_modal;
mod load;
mod menu;
mod play;
mod results;

mod playable_round;
use playable_round::PlayableRound;

use crate::menu::MenuHeaderText;

pub const SETTINGS_APP_NAME: &'static str = "com.github.kfc35.pinpoint";

/// States that the app can transition between that trigger the whole screen to change.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum AppState {
    #[default]
    Menu,
    Create,
    Play,
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

/// The encoded information of any user's created round for the day.
/// As a [`Resource`], it contains the data for the current user's shareable round for today.
/// The current user can also receive data of this type from another person,
/// in which case it is stored under [`crate::load::LoadableRounds`].
///
/// The value is a [`crate::playable_round::PlayableRound`] that has been:
/// - Serialized
/// - Base64 Encoded
#[derive(Resource, Reflect, Clone, Default, Deref, DerefMut, SettingsGroup, PartialEq)]
#[reflect(Resource, Default, SettingsGroup)]
pub(crate) struct EncodedRound(String);

impl EncodedRound {
    fn is_empty(&self) -> bool {
        self.0 == ""
    }

    fn try_decode(&self, app_type_registry: &AppTypeRegistry) -> Option<PlayableRound> {
        if self.0 == "" {
            return None;
        }
        let type_registry = app_type_registry.read();

        let decoded = URL_SAFE.decode(self.0.clone()).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&decoded).unwrap();

        let registration = type_registry
            .get(std::any::TypeId::of::<PlayableRound>())
            .unwrap();
        let deserializer = TypedReflectDeserializer::new(registration, &type_registry);
        let reflect_value = deserializer.deserialize(value).unwrap();

        if reflect_value.represents::<PlayableRound>()
            && let Some(round) = reflect_value.try_downcast_ref::<PlayableRound>()
        {
            Some((*round).clone())
        } else {
            None
        }
    }

    fn decode(&self, app_type_registry: &AppTypeRegistry) -> PlayableRound {
        self.try_decode(app_type_registry)
            .expect("EncodedRound should be valid.")
    }

    fn is_valid(&self, today: &String, app_type_registry: &AppTypeRegistry) -> bool {
        !self.is_empty()
            && self
                .try_decode(app_type_registry)
                .is_some_and(|playable_round| *playable_round.get_date() == *today)
    }
}

/// System that preps the [`EncodedRound`] resource.
pub(crate) fn init_encoded_round(
    mut commands: Commands,
    start_date_time: Res<StartDateTime>,
    encoded_round: Option<ResMut<EncodedRound>>,
    type_registry: Res<AppTypeRegistry>,
) {
    if let Some(round) = encoded_round
        && round.is_valid(&start_date_time.date, &type_registry)
    {
        return;
    }
    let round = EncodedRound("".to_string());
    commands.insert_resource(round);
    commands.queue(SaveSettingsSync::Always);
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
        .init_resource::<MenuHeaderText>()
        .add_plugins(SettingsPlugin::new(SETTINGS_APP_NAME))
        .add_plugins((
            animation::AnimateGifPlugin,
            menu::MenuPlugin,
            create::CreatePlugin,
        ))
        .add_systems(
            Startup,
            (
                setup,
                create::init_created_round,
                init_encoded_round,
                load::init_loadable_rounds,
                // TODO need an init_loaded_rounds,
                #[cfg(target_arch = "wasm32")]
                load::parse_window_url,
                create::setup_create,
                play::setup_play_skeleton,
                menu::setup_menu,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                ui::update_scrollbar_visibility,
                ui::update_scrollbar_with_scroll,
            ),
        )
        .add_systems(
            OnEnter(AppState::Menu),
            (menu::show_menu, how_to_modal::spawn_how_to_modal),
        )
        .add_systems(
            OnExit(AppState::Menu),
            (menu::hide_menu, how_to_modal::despawn_how_to_modal),
        )
        .add_systems(
            OnEnter(AppState::Create),
            (create::show_create, how_to_modal::spawn_how_to_modal),
        )
        .add_systems(
            OnExit(AppState::Create),
            (create::hide_create, how_to_modal::despawn_how_to_modal),
        )
        .add_systems(
            OnEnter(AppState::Play),
            (play::show_play, how_to_modal::spawn_how_to_modal),
        )
        .add_systems(
            OnExit(AppState::Play),
            (play::hide_play, how_to_modal::despawn_how_to_modal),
        )
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

use age::secrecy::SecretString;
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

mod create;
mod grid_axes;
pub(crate) use grid_axes::axes_descriptions;

mod animation;
mod menu;
mod playable_round;
mod ui;

pub const SETTINGS_APP_NAME: &'static str = "com.github.kfc35.pinpoint";

/// Used by the app to obfuscate playable rounds that people send each other.
#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct SecretPassphrase(String);

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

/// The encrypted information of any user's created round for the day.
/// As a [`Resource`], it contains the data for the current user's shareable round for today.
/// The current user can also receive this data from another person.
///
/// The value is a [`crate::playable_round::PlayableRound`] that has been:
/// - Serialized
/// - Encrypted
/// - Base64 Encoded
#[derive(Resource, Reflect, Clone, Default, Deref, DerefMut, SettingsGroup)]
#[reflect(Resource, Default, SettingsGroup)]
pub(crate) struct EncryptedShareableRound {
    date: String,
    #[deref]
    value: String,
}

impl EncryptedShareableRound {
    fn decode(
        &self,
        secret_passphrase: &SecretPassphrase,
        type_registry: &AppTypeRegistry,
    ) -> Option<Box<dyn PartialReflect>> {
        let type_registry = type_registry.read();

        let encrypted = URL_SAFE.decode(self.value.clone()).ok()?;
        let passphrase = SecretString::from((*secret_passphrase).clone());
        let identity = age::scrypt::Identity::new(passphrase);
        let decrypted = age::decrypt(&identity, &encrypted).ok()?;

        let value: serde_json::Value = serde_json::from_slice(&decrypted).unwrap();

        let registration = type_registry
            .get(std::any::TypeId::of::<playable_round::PlayableRound>())
            .unwrap();
        let deserializer = TypedReflectDeserializer::new(registration, &type_registry);
        let reflect_value = deserializer.deserialize(value).unwrap();

        if reflect_value.represents::<playable_round::PlayableRound>() {
            Some(reflect_value)
        } else {
            None
        }
    }

    fn is_valid(
        &self,
        today: &String,
        secret_passphrase: &SecretPassphrase,
        type_registry: &AppTypeRegistry,
    ) -> bool {
        self.date == *today
            && self.value != ""
            && self.decode(secret_passphrase, type_registry).is_some()
    }
}

/// System that preps the `EncryptedShareableRound` resource.
pub(crate) fn init_encrypted_shareable_round(
    mut commands: Commands,
    start_date_time: Res<StartDateTime>,
    encrypted_shareable_round: Option<ResMut<EncryptedShareableRound>>,
    secret_passphrase: Res<SecretPassphrase>,
    type_registry: Res<AppTypeRegistry>,
) {
    if let Some(round) = encrypted_shareable_round
        && round.is_valid(&start_date_time.date, &secret_passphrase, &type_registry)
    {
        return;
    }

    println!("Clearing encrypted round...");
    let round = EncryptedShareableRound {
        date: "".to_string(),
        value: "".to_string(),
    };
    commands.insert_resource(round);
    commands.queue(SaveSettingsSync::Always);
}

#[derive(Component, Clone, Default)]
pub struct Modal;

fn main() {
    let passphrase: &'static str = env!("SECRET_PASSPHRASE");
    let secret_passphrase = SecretPassphrase(passphrase.to_string());

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
        .insert_resource(secret_passphrase)
        .add_plugins(SettingsPlugin::new(SETTINGS_APP_NAME))
        .add_plugins((
            animation::AnimateGifPlugin,
            menu::MenuPlugin,
            create::CreatePlugin,
            playable_round::EncryptedRoundCreationPlugin,
        ))
        .add_systems(
            Startup,
            (
                setup,
                create::init_created_round,
                init_encrypted_shareable_round,
                create::setup_create,
            )
                .chain(),
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

use bevy::{
    DefaultPlugins,
    asset::{AssetMetaCheck, AssetPlugin},
    image::{ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
    reflect::{Reflect, std_traits::ReflectDefault},
    settings::{ReflectSettingsGroup, SaveSettingsSync, SettingsGroup, SettingsPlugin},
    text::FontSourceTemplate,
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};
use chrono::Utc;

mod create;
mod grid_axes;
pub(crate) use grid_axes::axes_descriptions;
mod menu;

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

pub const MIDDLE_BLUE_COLOR: Color = Color::srgb(0. / 255., 149. / 255., 233. / 255.);
pub const DARK_BLUE_COLOR: Color = Color::srgb(18. / 255., 78. / 255., 137. / 255.);
pub const DARK_ORANGE_COLOR: Color = Color::srgb(247. / 255., 118. / 255., 34. / 255.);
pub const DARK_RED_COLOR: Color = Color::srgb(158. / 255., 40. / 255., 53. / 255.);

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
        .add_plugins(menu::MenuPlugin)
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

/// Utility shorthand for the font.
pub(crate) fn pinpoint_font() -> impl Scene {
    bsn! {
        TextFont {
            font: FontSourceTemplate::Handle("font/Pinpoint.ttf"),
        }
    }
}

/// Utility to attach an observer to an entity for the given Pointer Event `E` that changes:
/// - the `BorderColor` of this entity to the provided color
/// - the `texture_atlas` of the `ImageNode` on this entity and its
///   direct child to use the provided index.
/// - the cursor to be the provided `system_cursor_icon`.
pub(crate) fn on_handler_style_button_image<E>(
    border_color: bevy::color::Color,
    texture_atlas_index: usize,
    system_cursor_icon: SystemCursorIcon,
) -> impl Scene
where
    E: core::fmt::Debug + Clone + bevy::reflect::Reflect,
{
    bsn! {
        on(
            move |event: On<Pointer<E>>,
            mut commands: Commands,
            children_query: Query<&Children>,
            mut image_q: Query<&mut ImageNode>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,| {
                commands.entity(event.entity).insert(BorderColor::all(border_color));
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(system_cursor_icon));
                }

                if let Some(Ok(mut image_node)) = image_q.get_mut(event.entity).into()
                    && let Some(atlas) = &mut image_node.texture_atlas {
                        atlas.index = texture_atlas_index;
                }

                if let Some(Ok(mut image_node)) = children_query
                    .iter_descendants(event.entity)
                    .find(|e| image_q.contains(*e))
                    .map(|e| image_q.get_mut(e))
                    && let Some(atlas) = &mut image_node.texture_atlas {
                        atlas.index = texture_atlas_index;
                }
        })
    }
}

/// Utility to return an [`ImageNode`].
pub(crate) fn image_node_with_texture_atlas(
    path: &'static str,
    tile_size: UVec2,
    num_rows: u32,
    index: usize,
) -> impl Scene {
    bsn! {
        template(move |context| {
            let layout = TextureAtlasLayout::from_grid(tile_size, 1, num_rows, None, None);
            let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
            let texture_atlas = TextureAtlas {
                layout: layout_handle,
                index,
            };
            Ok(ImageNode {
                image: context.resource::<AssetServer>().load(path),
                texture_atlas: Some(texture_atlas),
                ..Default::default()
            })
        })
    }
}

/// Observer to style text inputs
pub(crate) fn on_pointer_over_text_cursor(
    mut event: On<Pointer<Over>>,
    mut window_q: Query<Entity, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    for window in window_q.iter_mut() {
        commands
            .entity(window)
            .insert(CursorIcon::System(SystemCursorIcon::Text));
    }

    event.propagate(false);
}

/// Observer to unstyle cursor
pub(crate) fn on_pointer_out_default_cursor(
    mut event: On<Pointer<Out>>,
    mut window_q: Query<Entity, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    for window in window_q.iter_mut() {
        commands
            .entity(window)
            .insert(CursorIcon::System(SystemCursorIcon::Default));
    }
    event.propagate(false);
}

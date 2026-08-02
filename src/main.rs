use bevy::{
    DefaultPlugins,
    asset::{AssetMetaCheck, AssetPlugin},
    image::{ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
    text::FontSourceTemplate,
    ui_widgets::Activate,
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};
use chrono::Utc;

mod create;
mod menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum AppState {
    #[default]
    Menu,
    Create,
    // Play,
}

#[derive(Resource)]
pub(crate) struct StartDateTime {
    /// Current date as "%Y/%m/%d"
    date: String,
    /// Current time as "%H:%M:%S%.3f"
    time: String,
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
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), menu::setup_menu)
        .add_systems(OnExit(AppState::Menu), menu::teardown_menu)
        .add_systems(
            OnEnter(AppState::Create),
            (create::init_created_round, create::setup_create).chain(),
        )
        .add_systems(OnExit(AppState::Create), create::teardown_create)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // The game will change categories every day in Eastern time.
    let date_time = Utc::now().with_timezone(&chrono_tz::US::Eastern);
    let date = format!("{}", date_time.format("%Y/%m/%d"));
    let time = format!("{}", date_time.format("%H/%M/%S%.3f"));

    commands.insert_resource(StartDateTime { date, time });
}

pub fn pinpoint_font() -> impl Scene {
    bsn! {
        TextFont {
            font: FontSourceTemplate::Handle("font/Pinpoint.ttf"),
        }
    }
}

/// Helper to create an observer that changes the app state on activate of a button.
pub fn on_activate_change_state(next: AppState) -> impl Scene {
    bsn! {
        on(move |_: On<Activate>, mut next_state: ResMut<NextState<AppState>>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,
            mut commands: Commands,| {
            for window in window_q.iter_mut() {
                commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Default));
            }
            next_state.set(next);
        })
    }
}

/// Helper to attach an observer to an entity for the given Pointer Event `E` that changes:
/// - the `BorderColor` of this entity to the provided color
/// - the `texture_atlas` of the `ImageNode` on this entity and its
///   direct child to use the provided index.
/// - the cursor to be the provided `system_cursor_icon`.
pub fn on_handler_style_button_image<E>(
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

use bevy::{
    DefaultPlugins,
    asset::{AssetMetaCheck, AssetPlugin},
    image::{ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
    text::FontSourceTemplate,
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

mod menu;
mod create;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Create,
    // Play,
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
        .add_systems(OnEnter(AppState::Create), create::setup_create)
        .add_systems(OnExit(AppState::Create), create::teardown_create)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn pinpoint_font() -> impl Scene {
    bsn! {
        TextFont {
            font: FontSourceTemplate::Handle("font/Pinpoint.ttf"),
        }
    }
}

/// Helper to attach an observer to an entity for the given Pointer Event `E` that changes:
/// the `BorderColor` of this entity to the provided color and the `texture_atlas` of the
/// `ImageNode` on this entity and its direct child to use the provided index.
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

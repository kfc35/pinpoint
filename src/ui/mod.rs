use crate::AppState;
///! A hodgepodge of ui utilities to make composing ui elements easier.
use bevy::{
    prelude::*,
    text::FontSourceTemplate,
    ui::InteractionDisabled,
    ui_widgets::{Activate, Button},
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

/// Marker component for modals
#[derive(Component, Clone, Default)]
pub struct Modal;

// Colors used for text and buttons
pub const MIDDLE_BLUE_COLOR: Color = Color::srgb(0. / 255., 149. / 255., 233. / 255.);
pub const DARK_BLUE_COLOR: Color = Color::srgb(18. / 255., 78. / 255., 137. / 255.);
pub const MIDDLE_ORANGE_COLOR: Color = Color::srgb(254. / 255., 174. / 255., 52. / 255.);
pub const DARK_ORANGE_COLOR: Color = Color::srgb(247. / 255., 118. / 255., 34. / 255.);
pub const DARK_RED_COLOR: Color = Color::srgb(158. / 255., 40. / 255., 53. / 255.);
pub const DARK_GRAY_COLOR: Color = Color::srgb(90. / 255., 105. / 255., 136. / 255.);
pub const DARK_GREEN_COLOR: Color = Color::srgb(38. / 255., 92. / 255., 66. / 255.);

/// Base button scene for anywhere the app requires a button with
/// an [`ImageNode`] as its content.
pub(crate) fn base_button(
    path: &'static str,
    tile_size: UVec2,
    height: i32,
    width: i32,
    starting_index: usize,
    num_rows: u32,
    button_border_px: i32,
) -> impl Scene {
    bsn! {
        Button
        Node {
            border: UiRect::all(px(button_border_px)),
            height: percent(height),
            width: percent(width),
            min_width: px(280),
        }
        template_value({
            BorderColor::all(DARK_BLUE_COLOR)
        })
        on_handler_style_image_node::<Over>(DARK_ORANGE_COLOR, 1, SystemCursorIcon::Pointer)
        on_handler_style_image_node::<Press>(DARK_RED_COLOR, 2, SystemCursorIcon::Pointer)
        on_handler_style_image_node::<Release>(DARK_ORANGE_COLOR, 1, SystemCursorIcon::Pointer)
        on_handler_style_image_node::<Out>(DARK_BLUE_COLOR, 0, SystemCursorIcon::Default)
        Children [
            Node {
                height: percent(100),
                width: percent(100),
            }
            image_node_with_texture_atlas(path, tile_size, starting_index, num_rows)
        ]
    }
}

/// A base [`ImageNode`] scene.
pub(crate) fn image_node_with_texture_atlas(
    path: &'static str,
    tile_size: UVec2,
    index: usize,
    num_rows: u32,
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

/// Utility to attach an observer to an entity for the given Pointer Event `E` that changes:
/// - the `BorderColor` of this entity to the provided color
/// - the `texture_atlas` of the `ImageNode` on this entity and its
///   direct child to use the provided index.
/// - the cursor to be the provided `system_cursor_icon`.
pub(crate) fn on_handler_style_image_node<E>(
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
            is_disabled_q: Query<(), With<InteractionDisabled>>,
            children_q: Query<&Children>,
            mut image_q: Query<&mut ImageNode>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,| {
                // We only change the cursor no matter if it is disabled or not
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(system_cursor_icon));
                }

                if !is_disabled_q.contains(event.entity) {
                    commands.entity(event.entity).insert(BorderColor::all(border_color));
                    change_image_node_index(event.entity, texture_atlas_index, &children_q, &mut image_q);
                }
        })
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

/// Utility to update an image node's index, used when a button is changing states.
pub(crate) fn change_image_node_index(
    entity: Entity,
    texture_atlas_index: usize,
    children_query: &Query<&Children>,
    image_q: &mut Query<&mut ImageNode>,
) {
    if let Some(Ok(mut image_node)) = image_q.get_mut(entity).into()
        && let Some(atlas) = &mut image_node.texture_atlas
    {
        atlas.index = texture_atlas_index;
    }

    if let Some(Ok(mut image_node)) = children_query
        .iter_descendants(entity)
        .find(|e| image_q.contains(*e))
        .map(|e| image_q.get_mut(e))
        && let Some(atlas) = &mut image_node.texture_atlas
    {
        atlas.index = texture_atlas_index;
    }
}

/// Utility to create an observer that changes the app state on activate of the button
/// that this observer is attached to.
/// All menu buttons that change state check that the username is valid.
pub(crate) fn on_activate_change_state(next: AppState) -> impl Scene {
    bsn! {
        on(move |_: On<Activate>,
            mut next_state: ResMut<NextState<AppState>>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,
            mut commands: Commands,| {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Default));
                }
                next_state.set(next);
        })
    }
}

/// Observer to style text input cursor
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

/// Observer to unstyle cursors
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

/// Indices for the confirmation button
pub enum ConfirmationButtonIndex {
    GreenCheckmark = 0,
    RedX = 1,
}

/// Confirmation button (simple red X or green checkmark)
pub(crate) fn confirmation_button(
    background_color: Color,
    image_index: ConfirmationButtonIndex,
) -> impl Scene {
    bsn! {
        Button
        Node {
            width: px(75),
            min_width: px(75),
            border: px(5),
        }
        BorderColor::all(background_color)
        Children [
            Node {
                height: percent(100),
                width: percent(100),
                padding: px(5),
            }
            image_node_with_texture_atlas("button/confirmation_modal.png", UVec2::new(32, 32), image_index as usize, 2)
        ]
        on(
            move |_: On<Pointer<Over>>,
            mut commands: Commands,
            mut window_q: Query<Entity, With<PrimaryWindow>>,| {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Pointer));
                }
            }
        )
        on(
            move |_: On<Pointer<Out>>,
            mut commands: Commands,
            mut window_q: Query<Entity, With<PrimaryWindow>>,| {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Default));
                }
            }
        )
    }
}

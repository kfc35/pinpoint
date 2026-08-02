use bevy::{prelude::*, ui_widgets::Button, window::SystemCursorIcon};

use crate::{
    AppState, DARK_BLUE_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR, MIDDLE_BLUE_COLOR,
    image_node_with_texture_atlas, on_activate_change_state, on_handler_style_button_image,
    pinpoint_font,
};

/// Marker component for the menu
#[derive(Component, Clone, Default)]
pub struct AppMenu;

pub fn setup_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        AppMenu
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
        }
        Children [
            Node {
                height: percent(35),
                width: percent(80),
            }
            Children [
                logo()
            ],

            Node {
                height: percent(65),
                width: percent(100),
            }
            Children [
                buttons()
            ],
        ]
    });
}

pub fn teardown_menu(mut commands: Commands, app_menu_q: Single<Entity, With<AppMenu>>) {
    commands.entity(app_menu_q.entity()).despawn();
}

fn logo() -> impl Scene {
    bsn! {
        Node {
            height: percent(100),
            width: percent(100),
        }
        ImageNode {
            image: "logo/logo.png"
        }
    }
}

fn buttons() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            height: percent(100),
            width: percent(100),
        }
        Children [
            button("button/create.png", UVec2::new(192, 32), 20, 50, AppState::Create),
        ]
    }
}

fn button(
    path: &'static str,
    tile_size: UVec2,
    height: i32,
    width: i32,
    next_state_on_activate: AppState,
) -> impl Scene {
    bsn! {
        Button
        Node {
            border: UiRect::all(px(5)),
            height: percent(height),
            width: percent(width),
        }
        BorderColor::all(DARK_BLUE_COLOR)
        on_handler_style_button_image::<Over>(DARK_ORANGE_COLOR, 1, SystemCursorIcon::Pointer)
        on_handler_style_button_image::<Press>(DARK_RED_COLOR, 2, SystemCursorIcon::Pointer)
        on_handler_style_button_image::<Release>(DARK_ORANGE_COLOR, 1, SystemCursorIcon::Pointer)
        on_handler_style_button_image::<Out>(DARK_BLUE_COLOR, 0, SystemCursorIcon::Default)
        on_activate_change_state(next_state_on_activate)
        Children [
            // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
            Node {
                height: percent(100),
                width: percent(100),
            }
            image_node_with_texture_atlas(path, tile_size, 3, 0)
        ]
    }
}

// TODO date has to match.
fn invited_you_to_play_text(name: &'static str) -> impl Scene {
    bsn! {
        Node {
            width: percent(80)
        }
        Children [
            Text::new(format!("{} has invited you to guess their pin!", name))
            TextFont {
                font_size: px(32)
            }
            TextColor(MIDDLE_BLUE_COLOR)
            pinpoint_font()
        ]
    }
}

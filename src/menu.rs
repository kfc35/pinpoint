use bevy::{
    input_focus::tab_navigation::TabIndex,
    prelude::*,
    settings::SaveSettingsDeferred,
    text::{EditableText, EditableTextFilter, TextCursorStyle},
    ui_widgets::Button,
    window::SystemCursorIcon,
};

use crate::{
    AppState, DARK_BLUE_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR, MIDDLE_BLUE_COLOR, Username,
    image_node_with_texture_atlas, on_activate_change_state, on_handler_style_button_image,
    on_pointer_out_default_cursor, on_pointer_over_text_cursor, pinpoint_font,
};

/// Marker component for the menu
#[derive(Component, Clone, Default)]
pub struct AppMenu;

/// Marker component for the Username input
#[derive(Component, Clone, Default)]
pub struct UsernameInput;

#[derive(Component, Clone, Default)]
pub struct UsernameValidationIndicator;

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
                menu()
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

fn menu() -> impl Scene {
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
            username_input_row(),
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

fn username_input_row() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
        }
        Children [
            Node
            Children [
                Text::new("Username")
                TextFont {
                    font_size: FontSize::Px(32.)
                }
                pinpoint_font()
            ],

            UsernameInput
            Node {
                width: px(250),
                border: px(5),
                border_radius: BorderRadius::all(px(10)),
                padding: UiRect::axes(px(5), px(2)),
            }
            template(|ctx| {
                let username = ctx.resource::<Username>();
                let mut editable = EditableText {
                    max_characters: Some(10),
                    ..default()
                };
                editable.editor.set_text(username.as_ref());
                Ok(editable)
            })
            TextFont {
                font_size: FontSize::Px(16.)
            }
            pinpoint_font()
            TextLayout::justify(Justify::Center)
            EditableTextFilter::new(char::is_alphanumeric)
            TabIndex(0)
            TextCursorStyle::default()
            BackgroundColor(Color::BLACK)
            BorderColor::all(Color::BLACK)
            on(on_pointer_over_text_cursor)
            on(on_pointer_out_default_cursor),

            // Node {
            // }
            // Children [
            //     {
            //         let username = ctx.resource::<Username>();
            //         if username.is_valid() {
            //             image_node_with_texture_atlas("", UVec2::splat(32), 2, 0)
            //         }
            //     }
            // ]


            Node {
                width: percent(100),
                padding: px(5),
            }
            Children [
                Text::new("Username must be alphanumeric and between 1 - 10 chars.")
                TextFont {
                    font_size: FontSize::Px(12.)
                }
                pinpoint_font()
            ]
            ,

        ]
    }
}

/// System that sets the username when the editable text field is modified.
fn on_changed_username_input(
    mut username_input_q: Query<(&EditableText, &mut BorderColor), With<UsernameInput>>,
    mut username: ResMut<Username>,
    mut commands: Commands,
) {
    let Ok((editable_text, mut border_color)) = username_input_q.single_mut() else {
        return;
    };

    let new_name = editable_text.value().to_string();
    if new_name == username.0 {
        return;
    }
    if !Username::is_valid(&new_name) {
        *border_color = BorderColor::all(DARK_RED_COLOR);
        if new_name == "" {
            // Save the fact that the user wanted to clear their name only.
            username.0 = new_name;
            commands.queue(SaveSettingsDeferred::default());
        }
    } else {
        username.0 = new_name;
        *border_color = BorderColor::all(Color::BLACK);
        commands.queue(SaveSettingsDeferred::default());
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

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            on_changed_username_input.run_if(in_state(AppState::Menu)),
        );
    }
}

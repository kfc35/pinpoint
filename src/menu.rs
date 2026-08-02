use bevy::{
    input_focus::{FocusCause, InputFocus, tab_navigation::TabIndex},
    prelude::*,
    settings::SaveSettingsDeferred,
    text::{EditableText, EditableTextFilter, TextCursorStyle},
    ui_widgets::{Activate, Button},
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

use crate::{
    AppState, DARK_BLUE_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR, MIDDLE_BLUE_COLOR, Username,
    image_node_with_texture_atlas, on_handler_style_button_image, on_pointer_out_default_cursor,
    on_pointer_over_text_cursor, pinpoint_font,
};

// Marker Components

#[derive(Component, Clone, Default)]
pub struct AppMenu;

#[derive(Component, Clone, Default)]
pub struct UsernameInput;

#[derive(Component, Clone, Default)]
pub struct UsernameRequirements;

pub fn setup_menu(mut commands: Commands, username: Res<Username>) {
    commands.spawn_scene(bsn! {
        AppMenu
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
            row_gap: percent(5),
            margin: UiRect::top(percent(5)),
        }
        Children [
            Node {
                min_width: px(280),
                height: percent(20),
                width: percent(80),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
            }
            Children [
                logo()
            ],

            Node {
                // height: percent(65),
                width: percent(100),
            }
            Children [
                menu(&username)
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
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        ImageNode {
            image: "logo/logo.png"
        }
    }
}

fn menu(username: &Username) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            height: percent(100),
            width: percent(100),
            row_gap: percent(10),
        }
        Children [
            // TODO ensure text fits for Loading Text.
            
            button("button/create.png", UVec2::new(192, 32), 20, 50)
            on_activate_change_state(AppState::Create),

            Node {
                padding: px(3),
            }
            button("button/load.png", UVec2::new(128, 32), 20, 50),

            username_input_col(username),
        ]
    }
}

fn button(
    path: &'static str,
    tile_size: UVec2,
    height: i32,
    width: i32,
) -> impl Scene {
    bsn! {
        Button
        Node {
            border: UiRect::all(px(5)),
            height: percent(height),
            width: percent(width),
            min_width: px(280),
        }
        BorderColor::all(DARK_BLUE_COLOR)
        on_handler_style_button_image::<Over>(DARK_ORANGE_COLOR, 1, SystemCursorIcon::Pointer)
        on_handler_style_button_image::<Press>(DARK_RED_COLOR, 2, SystemCursorIcon::Pointer)
        on_handler_style_button_image::<Release>(DARK_ORANGE_COLOR, 1, SystemCursorIcon::Pointer)
        on_handler_style_button_image::<Out>(DARK_BLUE_COLOR, 0, SystemCursorIcon::Default)
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

fn username_input_col(username: &Username) -> impl Scene {
    let mut editable_text = EditableText {
        max_characters: Some(10),
        ..default()
    };
    // This can run before `setup` in the `Startup` schedule,
    // so check validity here too.
    if Username::is_valid(&username.0) {
        editable_text.editor.set_text(username.as_ref());
    }

    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: percent(5),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
        }
        Children [
            Node
            Children [
                Node
                username_greeting(username)
                TextFont {
                    font_size: FontSize::Px(24.)
                }
                pinpoint_font()
            ],

            UsernameInput
            Node {
                min_width: px(250),
                border: px(5),
                border_radius: BorderRadius::all(px(10)),
                padding: UiRect::axes(px(5), px(2)),
            }
            template_value(editable_text)
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
            username_directions(username),
        ]
    }
}

fn username_greeting(username: &Username) -> Box<dyn Scene> {
    if Username::is_valid(&username.0) {
        Box::new(bsn! {Text::new("What's Up?")})
    } else {
        Box::new(bsn! {Text::new("Username")})
    }
}

fn username_directions(username: &Username) -> Box<dyn Scene> {
    let base_directions = || {
        bsn! {
        UsernameRequirements
        Node {
            max_width: px(280),
            align_items: AlignItems::Center,
        }
        Children [
            Text::new("Username must be:\nalphanumeric incl. _ \nbetween 1 and 10 chars.")
            TextFont {
                font_size: FontSize::Px(12.)
            }
            pinpoint_font()
            TextLayout::justify(Justify::Center)
        ]}
    };

    if Username::is_valid(username) {
        Box::new(bsn! {
            base_directions()
            Visibility::Hidden
        })
    } else {
        Box::new(bsn! {
            base_directions()
        })
    }
}

/// System that sets the username when the editable text field is modified.
fn on_changed_username_input(
    mut username_input_q: Query<(&EditableText, &mut BorderColor), With<UsernameInput>>,
    mut username_dirs_q: Query<&mut Visibility, With<UsernameRequirements>>,
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
        let Ok(mut visibility) = username_dirs_q.single_mut() else {
            return;
        };
        *visibility = Visibility::Inherited;
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
                font_size: px(24)
            }
            TextColor(MIDDLE_BLUE_COLOR)
            pinpoint_font()
        ]
    }
}

/// Utility to create an observer that changes the app state on activate of the button
/// that this observer is attached to.
/// All menu buttons that change state check that the username is valid.
fn on_activate_change_state(next: AppState) -> impl Scene {
    bsn! {
        on(move |_: On<Activate>,
            mut next_state: ResMut<NextState<AppState>>,
            username: Res<Username>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,
            username_input_q: Query<Entity, With<UsernameInput>>,
            mut commands: Commands,
            mut focus: ResMut<InputFocus>| {
            if Username::is_valid(&username.0) {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Grabbing));
                }
                next_state.set(next);
            }
            else {
                let Ok(input_entity) = username_input_q.single_inner() else {
                    return;
                };
                commands.entity(input_entity)
                    .insert(BorderColor::all(DARK_RED_COLOR));
                focus.set(input_entity, FocusCause::Navigated);
                // Flash the input entity
                commands.entity(input_entity)
                    .insert(BackgroundColor(DARK_RED_COLOR));
                commands.delayed().secs(0.1).entity(input_entity).insert(BackgroundColor(Color::BLACK));
                commands.delayed().secs(0.2).entity(input_entity).insert(BackgroundColor(DARK_RED_COLOR));
                commands.delayed().secs(0.3).entity(input_entity).insert(BackgroundColor(Color::BLACK));
            }
        })
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

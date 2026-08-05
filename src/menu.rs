use bevy::{
    input_focus::{FocusCause, InputFocus, tab_navigation::TabIndex},
    picking::hover::Hovered,
    prelude::*,
    settings::SaveSettingsDeferred,
    text::{EditableText, EditableTextFilter, TextCursorStyle},
    ui::InteractionDisabled,
    ui_widgets::Activate,
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

use crate::{
    AppState, Username,
    ui::{
        DARK_BLUE_COLOR, DARK_GRAY_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR, base_button,
        change_image_node_index, on_pointer_out_default_cursor, on_pointer_over_text_cursor,
        pinpoint_font,
    },
};

// Marker Components

#[derive(Component, Clone, Default)]
pub struct AppMenu;

#[derive(Component, Clone, Default)]
pub struct UsernameInput;

#[derive(Component, Clone, Default)]
pub struct UsernameRequirements;

#[derive(Component, Clone, Default)]
pub struct NeedsValidUsername;

pub fn setup_menu(mut commands: Commands, username: Res<Username>) {
    commands.spawn_scene(bsn! {
        AppMenu
        Visibility::Inherited
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(97),
            row_gap: percent(3),
            margin: UiRect::top(percent(3)),
        }
        Children [
            Node {
                min_width: px(280),
                width: percent(80),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
            }
            Children [
                logo()
            ],

            Node {
                height: percent(75),
                width: percent(100),
            }
            Children [
                menu(&username)
            ],
        ]
    });
}

pub fn show_menu(app_menu_q: Single<&mut Visibility, With<AppMenu>>) {
    *app_menu_q.into_inner() = Visibility::Inherited;
}

pub fn hide_menu(app_menu_q: Single<&mut Visibility, With<AppMenu>>) {
    *app_menu_q.into_inner() = Visibility::Hidden;
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
    let (button_height, button_width) = (15, 50);
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            height: percent(100),
            width: percent(100),
            row_gap: percent(5),
        }
        Children [
            invited_you_to_play_text("TenChars!!"),

            needs_valid_username_button(username, "button/create.png", UVec2::new(192, 32), button_height, button_width, 4)
            on_activate_change_state(AppState::Create),

            Node {
                padding: px(3),
            }
            needs_valid_username_button(username, "button/load.png", UVec2::new(128, 32), button_height, button_width, 4),

            base_button("button/how_to.png", UVec2::new(170, 32), button_height, button_width, 0, 3, 5),

            username_input_col(username),
        ]
    }
}

fn needs_valid_username_button(
    username: &Username,
    path: &'static str,
    tile_size: UVec2,
    height: i32,
    width: i32,
    num_rows: u32,
) -> Box<dyn Scene> {
    if Username::is_valid(&username.0) {
        Box::new(bsn! {
            NeedsValidUsername
            Hovered::default()
            on_click_if_inactive()
            base_button(path, tile_size, height, width, 0, num_rows, 5)
        })
    } else {
        Box::new(bsn! {
            NeedsValidUsername
            Hovered::default()
            on_click_if_inactive()
            // The disabled state should be the last index (aka num_rows - 1)
            base_button(path, tile_size, height, width, (num_rows - 1) as usize, num_rows, 5)
            InteractionDisabled
            // Override border color
            BorderColor::all(DARK_GRAY_COLOR)
        })
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
    mut username_input_q: Query<
        (&EditableText, &mut BorderColor),
        (With<UsernameInput>, Without<NeedsValidUsername>),
    >,
    mut username_directions_q: Query<&mut Visibility, With<UsernameRequirements>>,
    mut needs_valid_username_q: Query<
        (Entity, &Hovered, &mut BorderColor),
        (With<NeedsValidUsername>, Without<UsernameInput>),
    >,
    children_query: Query<&Children>,
    mut image_q: Query<&mut ImageNode>,
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
        let Ok(mut visibility) = username_directions_q.single_mut() else {
            return;
        };
        *visibility = Visibility::Inherited;

        for (entity, _, mut border_color) in needs_valid_username_q.iter_mut() {
            commands.entity(entity).insert(InteractionDisabled);
            change_image_node_index(entity, 3, &children_query, &mut image_q);
            *border_color = BorderColor::all(DARK_GRAY_COLOR);
        }
    } else {
        username.0 = new_name;
        *border_color = BorderColor::all(Color::BLACK);
        commands.queue(SaveSettingsDeferred::default());

        for (entity, is_hovered, mut border_color) in needs_valid_username_q.iter_mut() {
            commands.entity(entity).remove::<InteractionDisabled>();

            if is_hovered.get() {
                change_image_node_index(entity, 1, &children_query, &mut image_q);
                *border_color = BorderColor::all(DARK_ORANGE_COLOR);
            } else {
                change_image_node_index(entity, 0, &children_query, &mut image_q);
                *border_color = BorderColor::all(DARK_BLUE_COLOR);
            }
        }
    }
}

// TODO date has to match.
fn invited_you_to_play_text(name: &'static str) -> impl Scene {
    bsn! {
        Node {
            min_width: px(280),
            width: percent(100),
        }
        Children [
            Node {
                width: percent(100),
                margin: UiRect::horizontal(px(5)),
            }
            Text::new(format!("Successfully loaded a round from {}!", name))
            TextFont {
                font_size: px(16.)
            }
            TextLayout::justify(Justify::Center)
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
            mut window_q: Query<Entity, With<PrimaryWindow>>,
            mut commands: Commands,| {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Grabbing));
                }
                next_state.set(next);
        })
    }
}

/// Utility to create an observer for interaction disabled buttons.
/// Certain buttons are disabled if the username is empty
fn on_click_if_inactive() -> impl Scene {
    bsn! {
        on(|event: On<Pointer<Click>>,
            mut commands: Commands,
            has_interaction_disabled_q: Query<Has<InteractionDisabled>>,
            username_input_q: Query<Entity, With<UsernameInput>>,
            mut focus: ResMut<InputFocus>| {
            if let Ok(is_disabled) = has_interaction_disabled_q.get(event.entity) && is_disabled &&
                let Ok(input_entity) = username_input_q.single_inner() {
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

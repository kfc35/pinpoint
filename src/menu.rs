use bevy::{
    input_focus::{FocusCause, InputFocus, tab_navigation::TabIndex},
    picking::hover::Hovered,
    prelude::*,
    settings::SaveSettingsDeferred,
    text::{EditableText, EditableTextFilter, TextCursorStyle},
    ui::InteractionDisabled,
};

use crate::{
    AppState, EncodedRound, StartDateTime, Username,
    load::{LoadableRounds, on_activate_show_load_modal},
    ui::{
        DARK_BLUE_COLOR, DARK_GRAY_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR, Modal, base_button,
        change_image_node_index, on_activate_change_state, on_pointer_out_default_cursor,
        on_pointer_over_text_cursor, pinpoint_font,
    },
};

/// Text shown to the user when the menu is loaded.
/// This text might contain something about a game that could be loaded
/// on startup on the web version.
#[derive(Resource, Clone, Default, Deref, DerefMut)]
pub(crate) struct MenuHeaderText(pub(crate) String);

// Marker Components
#[derive(Component, Clone, Default)]
pub struct AppMenu;

#[derive(Component, Clone, Default)]
pub struct MenuContainer;

#[derive(Component, Clone, Default)]
pub struct MenuHeader;

#[derive(Component, Clone, Default)]
pub struct UsernameInput;

#[derive(Component, Clone, Default)]
pub struct UsernameInputColumn;

#[derive(Component, Clone, Default)]
pub struct UsernameRequirements;

#[derive(Component, Clone, Default)]
pub struct NeedsValidUsername;

pub fn setup_menu(
    mut commands: Commands,
    username: Res<Username>,
    encoded_round: Res<EncodedRound>,
    start_date_time: Res<StartDateTime>,
    menu_header_text: Res<MenuHeaderText>,
    loadable_rounds: Res<LoadableRounds>,
    app_type_registry: Res<AppTypeRegistry>,
) {
    let encoded_round_is_valid = encoded_round.is_valid(&start_date_time.date, &app_type_registry);
    commands.spawn_scene_list(bsn_list! {
        crate::load::load_modal(&loadable_rounds, &app_type_registry),

        AppMenu
        Visibility::Inherited
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(97),
            row_gap: percent(2),
            margin: UiRect::top(percent(3)),
        }
        Children [
            Node {
                min_width: px(280),
                width: percent(80),
                max_height: percent(15),
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
                menu(&username, &start_date_time, &menu_header_text, encoded_round_is_valid)
            ],
        ],
    });
}

pub fn show_menu(
    app_menu_q: Single<&mut Visibility, (With<AppMenu>, Without<UsernameRequirements>)>,
    username_input_q: Single<Entity, With<UsernameInput>>,
    username_input_col_q: Single<Entity, With<UsernameInputColumn>>,
    username_directions_q: Single<&mut Visibility, (With<UsernameRequirements>, Without<AppMenu>)>,
    username: Res<Username>,
    encoded_round: Res<EncodedRound>,
    start_date_time: Res<StartDateTime>,
    app_type_registry: Res<AppTypeRegistry>,
    mut commands: Commands,
) {
    *app_menu_q.into_inner() = Visibility::Inherited;

    if encoded_round.is_valid(&start_date_time.date, &app_type_registry) {
        commands.entity(username_input_q.entity()).despawn();

        let username_id = commands.spawn_scene(static_username(&username)).id();
        commands
            .entity(username_input_col_q.entity())
            .insert_child(1, username_id);

        *username_directions_q.into_inner() = Visibility::Hidden;
    }
}

pub fn hide_menu(
    start_date_time: Res<StartDateTime>,
    mut to_hide_q: Query<&mut Visibility, Or<(With<AppMenu>, With<Modal>)>>,
    header_text_q: Single<&mut Text, With<MenuHeader>>,
) {
    for mut vis in to_hide_q.iter_mut() {
        *vis = Visibility::Hidden;
    }

    *header_text_q.into_inner() = Text::new(format!("{}", start_date_time.date));
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

fn menu(
    username: &Username,
    start_date_time: &Res<StartDateTime>,
    menu_header_text: &Res<MenuHeaderText>,
    encoded_round_is_valid: bool,
) -> impl Scene {
    let (button_height, button_width) = (15, 50);
    bsn! {
        MenuContainer
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            height: percent(100),
            width: percent(100),
            row_gap: percent(4),
        }
        Children [
            header_text(start_date_time, menu_header_text)
            ,

            needs_valid_username_button(username, "button/create.png", UVec2::new(192, 32), button_height, button_width, 4)
            on_activate_change_state(AppState::Create)
            ,

            Node {
                padding: px(3),
            }
            base_button("button/load.png", UVec2::new(128, 32), button_height, button_width, 0, 4, 5)
            on(on_activate_show_load_modal)
            ,

            base_button("button/how_to.png", UVec2::new(170, 32), button_height, button_width, 0, 3, 5),

            username_input_col(username, encoded_round_is_valid)
            ,
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

fn username_input_col(username: &Username, encoded_round_is_valid: bool) -> impl Scene {
    let mut editable_text = EditableText {
        max_characters: Some(10),
        ..default()
    };
    // This can run before `setup` in the `Startup` schedule,
    // so check validity here too.
    if Username::is_valid(&username.0) {
        editable_text.editor.set_text(username.as_ref());
    }

    let username_input = || -> Box<dyn Scene> {
        if !encoded_round_is_valid {
            Box::new(bsn! {
                UsernameInput
                Node {
                    min_width: px(250),
                    border: px(5),
                    border_radius: BorderRadius::all(px(10)),
                    padding: UiRect::axes(px(5), px(2)),
                }
                template_value(editable_text)
                TextFont {
                    font_size: FontSize::Rem(1.)
                }
                pinpoint_font()
                TextLayout::justify(Justify::Center)
                EditableTextFilter::new(|char| char.is_alphanumeric() || char == '_')
                TabIndex(0)
                TextCursorStyle::default()
                BackgroundColor(Color::BLACK)
                BorderColor::all(Color::BLACK)
                on(on_pointer_over_text_cursor)
                on(on_pointer_out_default_cursor)
            })
        } else {
            // You cannot edit your username after you have created a round for the day.
            Box::new(static_username(username))
        }
    };

    bsn! {
        UsernameInputColumn
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
                    font_size: FontSize::Rem(1.)
                }
                pinpoint_font()
            ],

            username_input(),

            username_directions(username),
        ]
    }
}

/// Username scene that is not editable. (Bevy 0.19 does not contain TextReadWriteMode)
fn static_username(username: &Username) -> impl Scene {
    let username = username.0.clone();
    bsn! {
        UsernameInput
        Node {
            min_width: px(250),
            border: px(5),
            border_radius: BorderRadius::all(px(10)),
            padding: UiRect::axes(px(5), px(2)),
        }
        Text::new(format!("{}", username))
        TextFont {
            font_size: FontSize::Rem(1.)
        }
        pinpoint_font()
        TextLayout::justify(Justify::Center)
        BackgroundColor(Color::BLACK)
        BorderColor::all(Color::BLACK)
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
                font_size: FontSize::Rem(0.8)
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

fn header_text(
    start_date_time: &Res<StartDateTime>,
    menu_header_text: &Res<MenuHeaderText>,
) -> impl Scene {
    let text = if menu_header_text.0.len() > 0 {
        format!("{}\n\n{}", start_date_time.date, menu_header_text.0)
    } else {
        format!("{}", start_date_time.date)
    };
    bsn! {
        Node {
            min_width: px(280),
            width: percent(80),
        }
        Children [
            MenuHeader
            Node {
                width: percent(100),
                margin: UiRect::horizontal(px(5)),
            }
            Text::new(text)
            TextFont {
                font_size: FontSize::Rem(0.8)
            }
            TextLayout::justify(Justify::Center)
            pinpoint_font()
        ]
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
            (on_changed_username_input, crate::load::on_changed_url_input)
                .run_if(in_state(AppState::Menu)),
        );
    }
}

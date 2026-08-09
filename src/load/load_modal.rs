use crate::{
    AppState, EncodedRound, StartDateTime,
    load::{LoadableRound, LoadableRounds},
    ui::{
        ConfirmationButtonIndex, DARK_BLUE_COLOR, DARK_GRAY_COLOR, DARK_GREEN_COLOR,
        DARK_ORANGE_COLOR, DARK_RED_COLOR, MIDDLE_BLUE_COLOR, MIDDLE_ORANGE_COLOR, Modal,
        base_button, confirmation_button, pinpoint_font,
    },
};
use bevy::{
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    text::{EditableText, TextCursorStyle},
    ui::{Checked, InteractionDisabled},
    ui_widgets::{
        Activate, ControlOrientation, RadioButton, RadioGroup, Scrollbar, ScrollbarThumb,
        ValueChange,
    },
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

#[derive(Component, Default, Clone)]
pub struct LoadModal;

#[derive(Component, Default, Clone)]
struct LoadSelect;

#[derive(Component, Default, Clone)]
pub struct LoadRadioGroup;

#[derive(Component, Default, Clone)]
pub struct LoadUrlTextInput;

#[derive(Component, Default, Clone)]
pub struct PlusButton;

#[derive(Component, Default, Clone)]
pub struct PlayButton;

/// Placed on a RadioButton signifying which round in [`LoadableRounds`] is selected.
#[derive(Component, Default, Clone)]
pub struct RoundIndex(usize);

/// An observer that shows the load modal on activate.
pub fn on_activate_show_load_modal(
    _: On<Activate>,
    to_show_q: Single<&mut Visibility, With<LoadModal>>,
    previously_checked_q: Query<&RoundIndex, With<Checked>>,
    radio_group_q: Single<Entity, With<LoadRadioGroup>>,
    loadable_rounds: Res<LoadableRounds>,
    app_type_registry: Res<AppTypeRegistry>,
    mut commands: Commands,
) {
    let selected_index = previously_checked_q
        .single()
        .map(|round_index| round_index.0)
        .ok();
    commands
        .entity(radio_group_q.entity())
        .despawn_children()
        .queue_spawn_related_scenes::<Children>(load_select_children(
            &loadable_rounds,
            &app_type_registry,
            selected_index,
        ));

    *to_show_q.into_inner() = Visibility::Inherited;
}

pub fn hide_load_modal(to_hide_q: Single<&mut Visibility, With<LoadModal>>) {
    *to_hide_q.into_inner() = Visibility::Hidden;
}

/// Pops up the load modal containing rounds to play / way to import rounds.
/// This is intended to be used on the menu when the load button is activated.
pub fn load_modal(
    loadable_rounds: &LoadableRounds,
    app_type_registry: &AppTypeRegistry,
) -> impl Scene {
    let maybe_input_form = || -> Box<dyn Scene> {
        // Doing this because copy and paste is not working on wasm32.
        if cfg!(not(target_arch = "wasm32")) {
            Box::new(bsn! { load_game_input_form() })
        } else {
            Box::new(bsn! {})
        }
    };

    bsn! {
        // Background Node to center the modal.
        Modal
        LoadModal
        Visibility::Hidden
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
        }
        GlobalZIndex(1)
        BackgroundColor({DARK_BLUE_COLOR.with_alpha(0.5)})
        Children [
            Node {
                border: px(5),
                padding: UiRect::axes(px(10), px(10)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                width: percent(95),
                height: percent(80),
                row_gap: px(10),
            }
            BorderColor::all(DARK_BLUE_COLOR)
            BackgroundColor(Color::BLACK)
            Children [
                Node {
                    position_type: PositionType::Absolute,
                    top: px(5),
                    left: px(5),
                    width: px(50),
                    height: px(50),
                }
                ZIndex(1)
                Children [
                    confirmation_button(DARK_RED_COLOR, ConfirmationButtonIndex::RedX)
                    on(|_: On<Activate>,
                        load_modal_q : Single<&mut Visibility, With<LoadModal>>| {
                            hide_load_modal(load_modal_q);
                    })
                ]
                ,

                load_select(&loadable_rounds,&app_type_registry)
                Node {
                    // the X takes up 55 px from the top
                    // The select should be 60px from the top
                    // The row gap is already 10px
                    margin: UiRect::top(px(50))
                }
                ,

                maybe_input_form()
                ,

                play_button(&loadable_rounds)
                ,
            ]
        ]
    }
}

fn load_select(
    loadable_rounds: &LoadableRounds,
    app_type_registry: &AppTypeRegistry,
) -> impl Scene {
    bsn! {
        LoadSelect
        Node {
            display: Display::Grid,
            width: percent(90),
            height: percent(50),
            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.),RepeatedGridTrack::auto(1)],
            justify_content: JustifyContent::SpaceAround,
        }
        on(crate::ui::handle_mouse_drag_as_scroll)
        Children [
            #Content
            RadioGroup
            LoadRadioGroup
            Node {
                flex_direction: FlexDirection::Column,
                height: percent(100),
                padding: px(5),
                border: px(5),
                overflow: Overflow::scroll_y(),
            }
            BackgroundColor(Color::WHITE)
            on(|event: On<ValueChange<Entity>>,
                index_q: Query<(Entity, &RoundIndex), With<RadioButton>>,
                checked_index_q: Single<(Entity, &RoundIndex), With<Checked>>,
                mut commands: Commands| {
                    let Ok((new_checked_entity, RoundIndex(idx))) = index_q.get(event.value) else {
                        return;
                    };
                    let (checked_entity, RoundIndex(checked_index)) = checked_index_q.into_inner();
                    if idx == checked_index {
                        return;
                    }

                    // Update styles.
                    commands.entity(checked_entity)
                        .remove::<Checked>()
                        .remove::<BackgroundColor>();

                    commands.entity(new_checked_entity)
                        .insert((Checked, BackgroundColor(Color::BLACK)));
            })
            Children [
                { load_select_children(loadable_rounds, app_type_registry, None) }
            ],

            // Scrollbar
            Node {
                min_width: px(12),
                height: percent(100),
            }
            // Hide it by default since users most likely do not need it.
            // A system will update this to visible if needed.
            Visibility::Hidden
            BackgroundColor(Color::WHITE)
            Scrollbar {
                orientation: ControlOrientation::Vertical,
                target: #Content,
                min_thumb_length: 8.0,
            }
            Children [
                BorderColor::all(MIDDLE_BLUE_COLOR)
                BackgroundColor(MIDDLE_BLUE_COLOR)
                ScrollbarThumb {
                    border_radius: BorderRadius::all(px(4)),
                    border: UiRect::all(px(1)),
                }
            ]
        ]
    }
}

fn load_select_children(
    loadable_rounds: &LoadableRounds,
    app_type_registry: &AppTypeRegistry,
    selected_index: Option<usize>,
) -> Box<dyn SceneList> {
    let (unplayed, played): (Vec<(usize, &LoadableRound)>, Vec<(usize, &LoadableRound)>) =
        loadable_rounds
            .iter()
            .enumerate()
            .partition(|&(_, round)| round.final_guess.is_none());
    let unplayed_len = unplayed.len();
    let played_len = played.len();

    if loadable_rounds.len() == 0 {
        return Box::new(bsn_list! {
            Text::new("No games to play")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.2)
            }
            TextColor(Color::BLACK),
        });
    }
    Box::new(bsn_list! {
        Node {
            flex_direction: FlexDirection::Column,
            width: percent(100),
        }
        Children [
            Node {
                width: percent(100),
                padding: px(5),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
            }
            BackgroundColor({DARK_GRAY_COLOR.with_alpha(0.5)})
            Children [
                Node {
                    width: percent(100),
                }
                Text::new(format!("Unplayed ({unplayed_len})"))
                pinpoint_font()
                TextFont {
                    font_size: FontSize::Rem(0.8)
                }
                TextColor(Color::BLACK)
            ],

            {
                unplayed
                    .iter()
                    .enumerate()
                    .map(|(unplayed_idx, (index, round))|
                        loadable_round_to_radio_button(
                            *index,
                            round,
                            (selected_index.is_none() && unplayed_idx == 0) || selected_index.is_some_and(|s_idx| s_idx == *index),
                            app_type_registry,
                        )
                    )
                    .collect::<Vec<_>>()
            }
        ],

        Node {
            flex_direction: FlexDirection::Column,
            width: percent(100),
        }
        Children [
            Node {
                width: percent(100),
                padding: px(5),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
            }
            BackgroundColor({DARK_GRAY_COLOR.with_alpha(0.5)})
            Children [
                Node {
                    width: percent(100),
                }
                Text::new(format!("Played ({played_len})"))
                pinpoint_font()
                TextFont {
                    font_size: FontSize::Rem(0.8)
                }
                TextColor(Color::BLACK)
            ],

            {
                played
                    .iter()
                    .enumerate()
                    .map(|(played_idx, (index, round))|
                        loadable_round_to_radio_button(
                            *index,
                            round,
                            (selected_index.is_none() && unplayed_len == 0 && played_idx == 0) || selected_index.is_some_and(|s_idx| s_idx == *index),
                            app_type_registry
                        )
                    )
                    .collect::<Vec<_>>()
            }
        ],
    })
}

fn loadable_round_to_radio_button(
    index: usize,
    round: &LoadableRound,
    is_checked: bool,
    app_type_registry: &AppTypeRegistry,
) -> impl Scene {
    let playable_round = round.as_playable_round(app_type_registry);
    let text = format!("  {}", playable_round.get_creator());
    let checked = || -> Box<dyn Scene> {
        if is_checked {
            Box::new(bsn! {
                Checked
                BackgroundColor(Color::BLACK)
            })
        } else {
            Box::new(bsn! {})
        }
    };

    bsn! {
        RadioButton
        checked()
        RoundIndex(index)
        Node {
            width: percent(100),
            padding: px(5),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
        }
        Children [
            Node {
                width: percent(100),
            }
            Text::new(text)
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(0.8),
            }
            TextColor(MIDDLE_BLUE_COLOR)
        ]
        on_handler_style_radio_button::<Over>(MIDDLE_ORANGE_COLOR.with_alpha(0.75), SystemCursorIcon::Pointer)
        on_handler_style_radio_button::<Out>(Color::WHITE, SystemCursorIcon::Default)
    }
}

pub(crate) fn on_handler_style_radio_button<E>(
    background_color: bevy::color::Color,
    cursor: SystemCursorIcon,
) -> impl Scene
where
    E: core::fmt::Debug + Clone + bevy::reflect::Reflect,
{
    bsn! {
        on(
            move |event: On<Pointer<E>>,
            mut commands: Commands,
            checked_q: Query<(),With<Checked>>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,| {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(cursor));
                }
                if !checked_q.contains(event.entity) {
                    commands.entity(event.entity).insert(BackgroundColor(background_color));
                }
        })
    }
}

fn load_game_input_form() -> impl Scene {
    bsn! {
        Node {
            width: percent(90),
            min_width: px(280),
            row_gap: px(5),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
        }
        Children [
            Node {
                width: percent(100),
            }
            Text::new("Add Game via URL")
            TextFont {
                font_size: FontSize::Rem(0.75)
            }
            TextLayout::justify(Justify::Center)
            pinpoint_font()
            ,

            Node {
                flex_direction: FlexDirection::Row,
                width: percent(100),
                height: px(100),
            }
            Children[
                LoadUrlTextInput
                Node {
                    width: percent(80),
                    height: percent(100),
                    border: px(5),
                }
                // While EditableText is weird in Bevy 0.19,
                // Allow for new lines so that rendering for
                // viewport can be avoided via user circumvention.
                template_value({
                    let mut editable = EditableText::new("");
                    editable.visible_lines = Some(3.);
                    editable.allow_newlines = true;
                    editable
                })
                TextFont {
                    font_size: FontSize::Rem(0.75)
                }
                pinpoint_font()
                TextColor(Color::BLACK)
                TextLayout::new(Justify::Left, LineBreak::WordOrCharacter)
                TabIndex(0)
                TextCursorStyle::default()
                BackgroundColor(Color::WHITE)
                BorderColor::all(Color::WHITE)
                on(crate::ui::on_pointer_over_text_cursor)
                on(crate::ui::on_pointer_out_default_cursor), // TODO need to add system to detect change.

                PlusButton
                // start off disabled
                crate::ui::base_button("button/plus_icon.png", UVec2::splat(32), 100, 19, 3, 4, 5)
                Node {
                    min_width: Val::Auto,
                }
                Hovered::default()
                InteractionDisabled
                BorderColor::all(DARK_GRAY_COLOR)
                on(|_: On<Activate>,
                    load_url_text_q: Single<(Entity, &mut EditableText), With<LoadUrlTextInput>>,
                    load_radio_group_q: Single<Entity, With<LoadRadioGroup>>,
                    checked_q: Query<&RoundIndex, With<Checked>>,
                    play_button_q: Single<(Entity, &Hovered), With<PlayButton>>,
                    children_query: Query<&Children>,
                    mut image_q: Query<&mut ImageNode>,
                    (start_date_time, app_type_registry, mut loadable_rounds, my_created_round):
                        (Res<StartDateTime>, Res<AppTypeRegistry>, ResMut<LoadableRounds>, Res<EncodedRound>),
                    mut commands: Commands,| {
                        let (text_entity, mut editable_text) = load_url_text_q.into_inner();
                        let url = editable_text.value().to_string();
                        let (success, _) = crate::load::load_shared_round(url, &start_date_time, &app_type_registry, &mut loadable_rounds, &my_created_round, &mut commands);
                        editable_text.clear(); // `on_changed_url_input` will handle styling.
                        if success {
                            // Refresh the load radio group.
                            let rg_entity = load_radio_group_q.into_inner();
                            let selected_index = if let Ok(index) = checked_q.single() {
                                Some(index.0)
                            } else {
                                None
                            };
                            commands.entity(rg_entity).despawn_children();
                            commands.entity(rg_entity).queue_spawn_related_scenes::<Children>(load_select_children(&loadable_rounds, &app_type_registry, selected_index));
                            // Flash the radio groups border.
                            commands.entity(rg_entity)
                                .insert(BorderColor::all(DARK_GREEN_COLOR));
                            commands.delayed().secs(0.3).entity(rg_entity).insert(BorderColor::all(Color::WHITE));


                            let (play_entity, hovered) = play_button_q.into_inner();
                            if hovered.get() {
                                    commands.entity(play_entity)
                                    .remove::<InteractionDisabled>()
                                    .insert(BorderColor::all(DARK_ORANGE_COLOR));
                                crate::ui::change_image_node_index(play_entity, 1, &children_query, &mut image_q);
                            } else {
                                    commands.entity(play_entity)
                                    .remove::<InteractionDisabled>()
                                    .insert(BorderColor::all(DARK_BLUE_COLOR));
                                crate::ui::change_image_node_index(play_entity, 0, &children_query, &mut image_q);
                            }

                        } else {
                            // Flash the input entity
                            commands.entity(text_entity)
                                .insert(BackgroundColor(DARK_RED_COLOR));
                            commands.delayed().secs(0.3).entity(text_entity).insert(BackgroundColor(Color::WHITE));
                        }
                }),
            ]
        ]
    }
}

/// System that enables the + button if the field is not empty.
pub fn on_changed_url_input(
    mut clue_input_q: Query<
        (&EditableText, &mut BorderColor),
        (With<LoadUrlTextInput>, Without<PlusButton>),
    >,
    mut needs_valid_clue_input_q: Query<
        (Entity, &Hovered, &mut BorderColor),
        (With<PlusButton>, Without<LoadUrlTextInput>),
    >,
    children_query: Query<&Children>,
    mut image_q: Query<&mut ImageNode>,
    mut commands: Commands,
) {
    let Ok((editable_text, mut border_color)) = clue_input_q.single_mut() else {
        return;
    };

    if editable_text.value().to_string().is_empty() {
        for (entity, _, mut border_color) in needs_valid_clue_input_q.iter_mut() {
            commands.entity(entity).insert(InteractionDisabled);
            crate::ui::change_image_node_index(entity, 3, &children_query, &mut image_q);
            *border_color = BorderColor::all(DARK_GRAY_COLOR);
        }
    } else {
        *border_color = BorderColor::all(Color::BLACK);

        for (entity, is_hovered, mut border_color) in needs_valid_clue_input_q.iter_mut() {
            commands.entity(entity).remove::<InteractionDisabled>();

            if is_hovered.get() {
                crate::ui::change_image_node_index(entity, 1, &children_query, &mut image_q);
                *border_color = BorderColor::all(DARK_ORANGE_COLOR);
            } else {
                crate::ui::change_image_node_index(entity, 0, &children_query, &mut image_q);
                *border_color = BorderColor::all(DARK_BLUE_COLOR);
            }
        }
    }
}

fn play_button(loadable_rounds: &LoadableRounds) -> Box<dyn Scene> {
    if !loadable_rounds.rounds.is_empty() {
        Box::new(bsn! {
            PlayButton
            Hovered::default()
            base_button("button/play.png", UVec2::new(128, 32), 7, 50, 0, 4, 5)
            Node {
                width: percent(90),
                height: px(50),
            }
            on_activate_play()
        })
    } else {
        Box::new(bsn! {
            PlayButton
            Hovered::default()
            base_button("button/play.png", UVec2::new(128, 32), 7, 50, 3, 4, 5)
            InteractionDisabled
            // Override border color
            BorderColor::all(DARK_GRAY_COLOR)
            Node {
                width: percent(90),
                height: px(50),
            }
            on_activate_play()
        })
    }
}

fn on_activate_play() -> impl Scene {
    bsn! {
        on(|_: On<Activate>,
            checked_q: Single<&RoundIndex, With<Checked>>,
            mut next_state: ResMut<NextState<AppState>>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,
            mut commands: Commands,| {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Default));
                }

                let round_index = checked_q.into_inner();
                crate::play::init_play_round(round_index.0, &mut commands);
                next_state.set(AppState::Play);
        })
    }
}

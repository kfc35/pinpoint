use crate::{
    load::{LoadableRound, LoadableRounds},
    ui::{
        ConfirmationButtonIndex, DARK_BLUE_COLOR, DARK_GRAY_COLOR, DARK_RED_COLOR,
        MIDDLE_BLUE_COLOR, MIDDLE_ORANGE_COLOR, Modal, base_button, confirmation_button,
        pinpoint_font,
    },
};
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui::{Checked, InteractionDisabled},
    ui_widgets::{Activate, RadioButton, RadioGroup, ValueChange},
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

#[derive(Component, Default, Clone)]
struct LoadModal;

#[derive(Component, Default, Clone)]
struct RoundIndex(usize);

/// Pops up the load modal containing rounds to play / way to import rounds.
/// This is intended to be used on the menu when the load button is activated.
pub fn load_modal(
    loadable_rounds: &LoadableRounds,
    app_type_registry: &AppTypeRegistry,
) -> impl Scene {
    bsn! {
        // Background Node to center the modal.
        Modal
        LoadModal
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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: percent(95),
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
                        mut commands: Commands,
                        modal_q: Single<Entity, With<LoadModal>>| {
                            commands.entity(modal_q.entity()).despawn();
                    }),
                ],

                load_select(&loadable_rounds,&app_type_registry)
                Node {
                    // the X takes up 55 px from the top
                    // The select should be 60px from the top
                    // The row gap is already 10px
                    margin: UiRect::top(px(50))
                }
                ,

                // TODO it can be disabled if there are no games to play.
                play_button(&loadable_rounds),
            ]
        ]
    }
}

fn load_select(
    loadable_rounds: &LoadableRounds,
    app_type_registry: &AppTypeRegistry,
) -> impl Scene {
    let (unplayed, played): (Vec<(usize, &LoadableRound)>, Vec<(usize, &LoadableRound)>) =
        loadable_rounds
            .iter()
            .enumerate()
            .partition(|&(_, round)| round.final_guess.is_none());
    let unplayed_len = unplayed.len();
    let played_len = played.len();
    bsn! {
        RadioGroup
        Node {
            flex_direction: FlexDirection::Column,
            width: percent(90)
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
            Node {
                width: percent(100),
            }
            Children [
                Node
                Children [
                    Text::new(format!("Unplayed ({unplayed_len})"))
                    pinpoint_font()
                    TextFont {
                        font_size: FontSize::Rem(0.8)
                    }
                    TextColor(Color::BLACK)
                ],

                {
                    unplayed.iter().map(|(index, round)| loadable_round_to_radio_button(*index, round, false, app_type_registry,)).collect::<Vec<_>>()
                }
            ],

            Node {
                width: percent(100),
            }
            Children [
                Node
                Children [
                    Text::new(format!("Played ({played_len})"))
                    pinpoint_font()
                    TextFont {
                        font_size: FontSize::Rem(0.8)
                    }
                    TextColor(Color::BLACK)
                ],

                {
                    played.iter().enumerate().map(|(played_index, (index, round))| loadable_round_to_radio_button(*index, round, played_index == 0, app_type_registry)).collect::<Vec<_>>()
                }
            ],
        ]
    }
}

fn loadable_round_to_radio_button(
    index: usize,
    round: &LoadableRound,
    is_checked: bool,
    app_type_registry: &AppTypeRegistry,
) -> impl Scene {
    let playable_round = round.get_round_as_playable_round(app_type_registry);
    let text = if let Some(guess) = round.final_guess {
        let distance = playable_round
            .get_location()
            .as_vec2()
            .distance(guess.as_vec2());
        format!("\t{} - {:.1}", playable_round.get_creator(), distance)
    } else {
        format!("\t{}", playable_round.get_creator())
    };
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
        }
        Children [
            Text::new(text)
            pinpoint_font()
            TextColor(MIDDLE_BLUE_COLOR)
        ]
        on_handler_style_radio_button::<Over>(MIDDLE_ORANGE_COLOR.with_alpha(0.25))
        on_handler_style_radio_button::<Out>(Color::WHITE)
    }
}

pub(crate) fn on_handler_style_radio_button<E>(background_color: bevy::color::Color) -> impl Scene
where
    E: core::fmt::Debug + Clone + bevy::reflect::Reflect,
{
    bsn! {
        on(
            move |event: On<Pointer<E>>,
            mut commands: Commands,
            checked_q: Query<Has<Checked>>,
            mut window_q: Query<Entity, With<PrimaryWindow>>,| {
                for window in window_q.iter_mut() {
                    commands.entity(window).insert(CursorIcon::System(SystemCursorIcon::Pointer));
                }
                if !checked_q.contains(event.entity) {
                    commands.entity(event.entity).insert(BackgroundColor(background_color));
                }
        })
    }
}

fn play_button(loadable_rounds: &LoadableRounds) -> Box<dyn Scene> {
    if !loadable_rounds.rounds.is_empty() {
        Box::new(bsn! {
            Hovered::default()
            base_button("button/play.png", UVec2::new(128, 32), 7, 50, 0, 4, 5)
            Node {
                width: px(250),
                height: px(50),
            }
        })
    } else {
        Box::new(bsn! {
            Hovered::default()
            base_button("button/play.png", UVec2::new(128, 32), 7, 50, 3, 4, 5)
            InteractionDisabled
            // Override border color
            BorderColor::all(DARK_GRAY_COLOR)
            Node {
                width: px(250),
                height: px(50),
            }
        })
    }
}

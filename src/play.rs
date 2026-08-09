use bevy::{
    picking::hover::Hovered,
    prelude::*,
    settings::SaveSettingsSync,
    ui::InteractionDisabled,
    ui_widgets::{Activate, ValueChange},
};

use crate::{
    StartDateTime,
    animation::AnimationTimer,
    axes_descriptions,
    load::LoadableRounds,
    playable_round::PlayableRound,
    ui::{
        AnswerPin, ConfirmationButtonIndex, DARK_BLUE_COLOR, DARK_COLOR, DARK_GRAY_COLOR,
        DARK_GREEN_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR, LIGHT_GREEN_COLOR, MIDDLE_BLUE_COLOR,
        MIDDLE_GREEN_COLOR, MIDDLE_RED_COLOR, Modal, MovablePin, PrimaryButtonContainer,
        YELLOW_COLOR, base_button, bottom_buttons, change_image_node_index, confirmation_button,
        location_grid, on_pointer_out_back_to_share, on_pointer_out_default_cursor,
        on_pointer_over_pointer_cursor, pinpoint_font, place_answer_pin, share_primary_button,
        update_crosshair_pin_node_with_location, update_pin_location,
    },
};

#[derive(Component, Clone, Default)]
pub struct AppPlay;

#[derive(Component, Clone, Default)]
pub struct ClueText;

#[derive(Component, Clone, Default)]
pub struct FromCreatorText;

#[derive(Component, Clone, Default)]
pub struct PlayHeaderText;

#[derive(Component, Clone, Default)]
pub struct PlayLocationGrid;

#[derive(Component, Clone, Default)]
pub struct PlayPrimaryButtonContainer;

#[derive(Component, Clone, Default)]
pub struct PlayDoneButton;

#[derive(Component, Clone, Default)]
pub struct PlayConfirmationModal;

#[derive(Component, Clone, Default)]
pub struct ResultsModal;

const DIRECTIONS_TEXT: &'static str = "Guess where the clue is";

/// This resource should only exist when in [`AppState::Play`].
/// This is set by the game loader when the play button is pressed.
#[derive(Resource)]
pub struct PlayRound {
    /// The current round as an index into [`crate::load::LoadableRounds`]
    loadable_rounds_index: usize,
}

impl PlayRound {
    pub fn get_index(&self) -> usize {
        self.loadable_rounds_index
    }
}

/// The current location of the guess
#[derive(Resource)]
pub struct CurrentGuess(Option<UVec2>);

/// Preps the `PlayRound` resource.
pub fn init_play_round(selected_index: usize, commands: &mut Commands) {
    commands.insert_resource(PlayRound {
        loadable_rounds_index: selected_index,
    });
}

/// System that is called when [`crate::AppState::Play`] is entered.
/// The [`PlayRound`] resource must be available before this is called.
pub fn show_play(
    to_show_q: Single<&mut Visibility, With<AppPlay>>,
    movable_pin_q: Single<(&mut Node, &mut MovablePin)>,
    location_grid_q: Single<Entity, With<PlayLocationGrid>>,
    play_round: Res<PlayRound>,
    loadable_rounds: Res<LoadableRounds>,
    app_type_registry: Res<AppTypeRegistry>,
    creator_text_q: Single<&mut Text, (With<FromCreatorText>, Without<ClueText>)>,
    clue_text_q: Single<&mut Text, (With<ClueText>, Without<FromCreatorText>)>,
    primary_button_q: Single<Entity, With<PlayPrimaryButtonContainer>>,
    mut commands: Commands,
) {
    let loadable_round = loadable_rounds.get_round(play_round.loadable_rounds_index);
    let location = loadable_round.get_final_guess();
    let playable_round = loadable_rounds
        .get_round(play_round.loadable_rounds_index)
        .as_playable_round(&app_type_registry);
    let (mut node, mut movable_pin) = movable_pin_q.into_inner();
    let button_container = primary_button_q.into_inner();
    match location {
        Some(loc) => {
            movable_pin.0 = false;
            commands.insert_resource(CurrentGuess(Some(loc)));
            node.display = Display::default();
            update_crosshair_pin_node_with_location(&mut node, loc);
            commands
                .entity(button_container)
                .queue_spawn_related_scenes::<Children>(bsn! {
                    share_primary_button()
                    create_on_activate_share_link()
                });

            commands.spawn_scene(results_modal(&playable_round));

            place_answer_pin(
                location_grid_q.entity(),
                loadable_round
                    .get_answer(&app_type_registry)
                    .expect("final guess has been submitted so this should be ok."),
                &mut commands,
            );
        }
        None => {
            movable_pin.0 = true;
            commands.insert_resource(CurrentGuess(None));
            node.display = Display::None;
            // TODO deactivated done button.
            commands
                .entity(button_container)
                .queue_spawn_related_scenes::<Children>(bsn! {
                    PlayDoneButton
                    InteractionDisabled
                    base_button("button/done.png", UVec2::new(128, 32), 100, 100, 3, 4, 5)
                    // Override border color
                    BorderColor::all(DARK_GRAY_COLOR)
                    Hovered::default()
                    on_click_if_inactive()
                    on(|_: On<Activate>,
                        modal_q: Single<&mut Visibility, With<PlayConfirmationModal>>| {
                            *modal_q.into_inner() = Visibility::Inherited;
                    })
                });
        }
    };

    let mut creator_text = creator_text_q.into_inner();
    *creator_text = Text::new(format!("Clue by {}", playable_round.get_creator()));

    let mut clue_text = clue_text_q.into_inner();
    *clue_text = Text::new(format!("{}", playable_round.get_clue()));

    *to_show_q.into_inner() = Visibility::Inherited;
}

/// System that is called when [`crate::AppState::Play`] is left.
pub fn hide_play(
    mut to_hide_q: Query<&mut Visibility, Or<(With<AppPlay>, With<Modal>)>>,
    primary_button_q: Single<Entity, With<PlayPrimaryButtonContainer>>,
    to_despawn_q: Query<Entity, Or<(With<ResultsModal>, With<AnswerPin>)>>,

    mut commands: Commands,
) {
    for mut vis in to_hide_q.iter_mut() {
        *vis = Visibility::Hidden;
    }

    for entity in to_despawn_q.iter() {
        commands.entity(entity).try_despawn();
    }
    commands
        .entity(primary_button_q.entity())
        .despawn_children();

    commands.remove_resource::<PlayRound>();
    commands.remove_resource::<CurrentGuess>();
}

pub fn setup_play_skeleton(mut commands: Commands, start_date_time: Res<StartDateTime>) {
    commands.spawn_scene_list(play_skeleton(&start_date_time));
}

pub fn play_skeleton(start_date_time: &Res<StartDateTime>) -> impl SceneList {
    bsn_list! {
        confirmation_modal(),

        AppPlay
        Visibility::Hidden
        BackgroundColor({DARK_COLOR})
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            row_gap: px(15),
            width: percent(100),
            height: percent(100),
        }
        Children [
            header_text()
            ,

            PlayLocationGrid
            location_grid(None, true, true)
            on(|event: On<ValueChange<UVec2>>,
                mut commands: Commands,
                mut current_guess: ResMut<CurrentGuess>,
                done_button_q: Single<(Entity, &Hovered, &mut BorderColor), With<PlayDoneButton>>,
                children_query: Query<&Children>,
                mut image_q: Query<&mut ImageNode>,
                pin_q: Single<(&mut Node, &MovablePin)>|{
                    current_guess.0 = Some(event.value);
                    let (entity, is_hovered, mut border_color) = done_button_q.into_inner();
                    if is_hovered.get() {
                        change_image_node_index(entity, 1, &children_query, &mut image_q);
                        *border_color = BorderColor::all(DARK_ORANGE_COLOR);
                    } else {
                        change_image_node_index(entity, 0, &children_query, &mut image_q);
                        *border_color = BorderColor::all(DARK_BLUE_COLOR);
                    }
                    commands.entity(entity)
                        .remove::<InteractionDisabled>();

                    update_pin_location(event.value, pin_q, true);
            })
            on(on_pointer_over_pointer_cursor)
            on(on_pointer_out_default_cursor)
            ,

            axes_descriptions(&start_date_time)
            ,

            clue_placeholder()
            ,

            primary_button_placeholder()
            ,

            bottom_buttons()
            ,
        ]
    }
}

fn header_text() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            width: px(280),
        }
        Children [
            PlayHeaderText
            Text::new(DIRECTIONS_TEXT)
            TextFont {
                font_size: FontSize::Rem(0.6)
            }
            pinpoint_font()
            TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
        ]
    }
}

fn on_click_if_inactive() -> impl Scene {
    bsn! {
        on(|event: On<Pointer<Click>>,
            mut commands: Commands,
            has_interaction_disabled_q: Query<Has<InteractionDisabled>>,
            location_grid_q: Single<Entity, With<PlayLocationGrid>>| {
            if let Ok(is_disabled) = has_interaction_disabled_q.get(event.entity) && is_disabled {
                // Flash the location grid
                commands.entity(location_grid_q.entity())
                    .insert(Outline::new(px(5), Val::ZERO, DARK_RED_COLOR));
                commands.delayed().secs(0.3).entity(location_grid_q.entity())
                    .insert(Outline::new(px(5), Val::ZERO, Color::WHITE));
            }
        })
    }
}

fn clue_placeholder() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            width: px(280),
        }
        Children [
            FromCreatorText
            Text::new("Clue by PLACEHOLDER")
            TextFont {
                font_size: FontSize::Rem(0.7)
            }
            pinpoint_font()
            TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),

            Node {
                min_width: px(280),
                border: px(5),
                border_radius: BorderRadius::all(px(10)),
            }
            BackgroundColor(Color::BLACK)
            BorderColor::all(Color::BLACK)
            Children [
                ClueText
                Text::new("PLACEHOLDER")
                TextFont {
                    font_size: FontSize::Rem(0.7)
                }
                pinpoint_font()
                TextLayout::new(Justify::Left, LineBreak::WordOrCharacter)
            ]
        ]
    }
}

fn primary_button_placeholder() -> impl Scene {
    bsn! {
        PlayPrimaryButtonContainer
        PrimaryButtonContainer
        Node {
            // override the width because we don't care
            width: Val::Auto,
            max_width: px(280),
            height: percent(7),
        }
        // Children will be added later.
    }
}

fn create_on_activate_share_link() -> impl Scene {
    bsn! {
        on(move |event: On<Activate>,
            start_date_time: Res<StartDateTime>,
            play_round: Res<PlayRound>,
            loadable_rounds: Res<LoadableRounds>,
            app_type_registry: Res<AppTypeRegistry>,
            mut clipboard: ResMut<Clipboard>,
            asset_server: Res<AssetServer>,
            mut layouts: ResMut<Assets<TextureAtlasLayout>>,
            mut commands: Commands,|
                {
                    let encoded_round = loadable_rounds.get_round(play_round.loadable_rounds_index);
                    let distance = get_share_distance_text(encoded_round.get_guess_distance(&app_type_registry));
                    let link = format!("https://kfc35.github.io/pinpoint/?share={}", encoded_round.get_encoded_value());

                    let playable_round = loadable_rounds.get_round(play_round.loadable_rounds_index).as_playable_round(&app_type_registry);
                    let title = format!("#Pinpoint - {} - Clue Giver: {}", start_date_time.date, playable_round.get_creator());
                    match clipboard.set_text(format!("{title}\n{distance}\n{link}")) {
                        Ok(_) => {
                            let layout = TextureAtlasLayout::from_grid(UVec2::new(160, 32), 1, 3, None, None);
                            let layout_handle = layouts.add(layout);
                            let texture_atlas = TextureAtlas {
                                layout: layout_handle,
                                index: 1,
                            };
                            commands.entity(event.entity).despawn_children();
                            let new_child = commands.spawn((
                                Node {
                                        width: percent(100),
                                        height: percent(100),
                                        ..default()
                                },
                                ImageNode {
                                    image: asset_server.load("button/copied.png"),
                                    texture_atlas: Some(texture_atlas),
                                    ..default()
                            })).id();
                            commands.entity(event.entity).add_child(new_child);

                        }
                        _ => {
                            commands.entity(event.entity).remove::<ImageNode>();
                            commands.entity(event.entity).insert(Text::new("Unable to Copy Results =/"));
                        }
                    }
                })
    }
}

fn get_share_distance_text(distance: f32) -> String {
    if distance <= 3. {
        format!("🎯 {distance:.3} away from pin")
    } else if distance <= 6.25 {
        format!("🟩 {distance:.3} away from pin")
    } else if distance <= 12.5 {
        format!("🟨 {distance:.3} away from pin")
    } else if distance <= 25. {
        format!("🟧 {distance:.3} away from pin")
    } else {
        format!("🟥 {distance:.3} away from pin")
    }
}

/// Confirmation modal that pops up when the user clicks the Done button
/// on the play screen.
fn confirmation_modal() -> impl Scene {
    bsn! {
        Modal
        PlayConfirmationModal
        GlobalZIndex(1)
        Visibility::Hidden
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
        }
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
            BorderColor::all(DARK_RED_COLOR)
            BackgroundColor(Color::BLACK)
            Children [
                Text::new("Are you sure that's the location?")
                pinpoint_font(),

                Text::new("You only have one guess!")
                pinpoint_font(),

                Node {
                    flex_direction: FlexDirection::Row,
                    width: px(250),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                }
                Children [
                    confirmation_button(DARK_RED_COLOR, ConfirmationButtonIndex::RedX)
                    on(|_: On<Activate>,
                        modal_q: Single<&mut Visibility, With<PlayConfirmationModal>>| {
                            *modal_q.into_inner() = Visibility::Hidden;
                    }),

                    confirmation_button(DARK_GREEN_COLOR, ConfirmationButtonIndex::GreenCheckmark)
                    on(|_: On<Activate>,
                        (current_guess, play_round, mut loadable_rounds, app_type_registry):
                        (Res<CurrentGuess>, Res<PlayRound>, ResMut<LoadableRounds>, Res<AppTypeRegistry>),
                        mut need_to_hide_q: Query<&mut Visibility, (With<PlayConfirmationModal>, Without<ResultsModal>)>,
                        location_grid_q: Single<Entity, With<PlayLocationGrid>>,
                        primary_button_q: Single<Entity, With<PlayPrimaryButtonContainer>>,
                        movable_pin_q: Single<&mut MovablePin>,
                        mut commands: Commands,| {
                            let loadable_round = loadable_rounds.get_round_mut(play_round.loadable_rounds_index);
                            loadable_round.set_final_guess(current_guess.0.expect("There should be a current guess."));
                            commands.queue(SaveSettingsSync::Always);

                            let mut movable_pin = movable_pin_q.into_inner();
                            movable_pin.0 = false;

                            let new_child = commands.spawn_scene(bsn! {
                                share_primary_button()
                                create_on_activate_share_link()
                            }).id();
                            commands
                                .entity(primary_button_q.entity())
                                .despawn_children()
                                .add_child(new_child);

                            for mut vis in need_to_hide_q.iter_mut() {
                                *vis = Visibility::Hidden;
                            }

                            place_answer_pin(location_grid_q.entity(),
                                loadable_round.get_answer(&app_type_registry).expect("final guess has been submitted so this should be ok."),
                                    &mut commands);

                            commands.spawn_scene(
                                results_modal(&loadable_round.as_playable_round(&app_type_registry))
                            );
                    }),
                ]
            ]
        ]
    }
}

/// Modal that pops up when the round is over
fn results_modal(playable_round: &PlayableRound) -> impl Scene {
    bsn! {
        Modal
        ResultsModal
        GlobalZIndex(1)
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
        }
        BackgroundColor({DARK_BLUE_COLOR.with_alpha(0.5)})
        Children [
            Node {
                border: px(5),
                padding: UiRect::axes(px(10), px(10)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: percent(95),
                row_gap: px(20),
            }
            BorderColor::all(DARK_GREEN_COLOR)
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
                        modal_q: Single<Entity, With<ResultsModal>>,
                        mut commands: Commands| {
                            commands.entity(modal_q.entity()).despawn();
                    }),
                ],

                Node {
                    width: px(250),
                    height: px(250),
                    margin: UiRect::top(px(50))
                }
                Children [
                    Node {
                        width: percent(100),
                        height: percent(100),
                    }
                    template(crate::results::get_result_image)
                    template(crate::results::get_animated_image_node)
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
                ],

                get_results_text_title(playable_round),

                get_distance_text_first_part()
                pinpoint_font()
                TextFont {
                    font_size: FontSize::Rem(0.7),
                }
                get_distance_text_color()
                Children [
                    TextSpan::new("You were ")
                    pinpoint_font()
                    TextFont {
                        font_size: FontSize::Rem(0.7),
                    },

                    get_distance_text_distance()
                    get_distance_text_color()
                    pinpoint_font()
                    TextFont {
                        font_size: FontSize::Rem(0.7),
                    },

                    get_distance_text_third_part()
                    pinpoint_font()
                    TextFont {
                        font_size: FontSize::Rem(0.7),
                    },
                ],

                base_button("button/share.png", UVec2::new(137, 32), 10, 80, 0, 3, 5)
                Node {
                    width: px(250),
                    height: px(50),
                }
                on_pointer_out_back_to_share()
                create_on_activate_share_link(),
            ]
        ]
    }
}

fn get_results_text_title(playable_round: &PlayableRound) -> impl Scene {
    let creator = playable_round.get_creator().clone();
    let date = playable_round.get_date().clone();
    bsn! {
        Text::new("You finished ")
        pinpoint_font()
        TextFont {
            font_size: FontSize::Rem(0.7)
        }
        Children [
            TextSpan::new(format!("{}", creator))
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(0.7)
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("'s ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(0.7)
            }
            ,

            TextSpan::new("#Pinpoint ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(0.7)
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("Round for ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(0.7)
            }
            ,

            TextSpan::new(format!("{date}"))
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(0.7)
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("!\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(0.7)
            }
            ,
        ]
    }
}

fn get_distance_text_first_part() -> impl Scene {
    let distance_text = |distance: f32| {
        if distance <= 3. {
            format!("Bullseye! ")
        } else if distance <= 6.25 {
            format!("Nice job! ")
        } else if distance <= 12.5 {
            format!("Not bad. ")
        } else if distance <= 25. {
            format!("Barely made it... ")
        } else {
            format!("Are you lost? ")
        }
    };
    bsn! {
        template(move |ctx| {
            let play_round = ctx.resource::<PlayRound>();
            let loadable_rounds = ctx.resource::<LoadableRounds>();
            let app_type_registry = ctx.resource::<AppTypeRegistry>();
            let loadable_round = loadable_rounds.get_round(play_round.loadable_rounds_index);
            let distance = loadable_round.get_guess_distance(app_type_registry);
            let text = format!("{}", distance_text(distance));
            Ok(Text::new(text))
        })
    }
}

fn get_distance_text_color() -> impl Scene {
    bsn! {
        template(move |ctx| {
            let play_round = ctx.resource::<PlayRound>();
            let loadable_rounds = ctx.resource::<LoadableRounds>();
            let app_type_registry = ctx.resource::<AppTypeRegistry>();
            let loadable_round = loadable_rounds.get_round(play_round.loadable_rounds_index);
            let distance = loadable_round.get_guess_distance(app_type_registry);
            let text_color = if distance <= 3. {
                TextColor(LIGHT_GREEN_COLOR)
            } else if distance <= 6.25 {
                TextColor(MIDDLE_GREEN_COLOR)
            } else if distance <= 12.5 {
                TextColor(YELLOW_COLOR)
            } else if distance <= 25. {
                TextColor(DARK_ORANGE_COLOR)
            } else {
                TextColor(MIDDLE_RED_COLOR)
            };
            Ok(text_color)
        })
    }
}

fn get_distance_text_distance() -> impl Scene {
    bsn! {
        template(move |ctx| {
            let play_round = ctx.resource::<PlayRound>();
            let loadable_rounds = ctx.resource::<LoadableRounds>();
            let app_type_registry = ctx.resource::<AppTypeRegistry>();
            let loadable_round = loadable_rounds.get_round(play_round.loadable_rounds_index);
            let distance = loadable_round.get_guess_distance(app_type_registry);
            let text = format!("{distance:.3}");
            Ok(TextSpan::new(text))
        })
    }
}

fn get_distance_text_third_part() -> impl Scene {
    let distance_text = |distance: f32| {
        if distance <= 1. {
            format!(" units away from the pin! Wow!")
        } else if distance <= 3. {
            format!(" units away from the pin!")
        } else if distance <= 6.25 {
            format!(" units away from the pin.")
        } else if distance <= 12.5 {
            format!(" units away from the pin.")
        } else if distance <= 25. {
            format!(" units away from the pin.")
        } else {
            format!(" units away from the pin...")
        }
    };
    bsn! {
        template(move |ctx| {
            let play_round = ctx.resource::<PlayRound>();
            let loadable_rounds = ctx.resource::<LoadableRounds>();
            let app_type_registry = ctx.resource::<AppTypeRegistry>();
            let loadable_round = loadable_rounds.get_round(play_round.loadable_rounds_index);
            let distance = loadable_round.get_guess_distance(app_type_registry);
            let text = format!("{}", distance_text(distance));
            Ok(TextSpan::new(text))
        })
    }
}

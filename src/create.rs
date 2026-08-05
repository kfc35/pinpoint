use bevy::{
    input_focus::{FocusCause, InputFocus, tab_navigation::TabIndex},
    picking::hover::Hovered,
    prelude::*,
    reflect::{Reflect, std_traits::ReflectDefault},
    settings::{ReflectSettingsGroup, SaveSettingsDeferred, SaveSettingsSync, SettingsGroup},
    text::{EditableText, EditableTextGeneration, TextCursorStyle},
    ui::InteractionDisabled,
    ui_widgets::{Activate, Button},
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

use crate::{
    AppState, StartDateTime, axes_descriptions,
    ui::{
        DARK_BLUE_COLOR, DARK_GRAY_COLOR, DARK_GREEN_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR,
        MIDDLE_BLUE_COLOR, base_button, change_image_node_index, image_node_with_texture_atlas,
        on_pointer_out_default_cursor, on_pointer_over_text_cursor, pinpoint_font,
    },
};
use rand::{RngExt, SeedableRng};

// Marker Components

#[derive(Component, Clone, Default)]
pub struct AppCreate;

#[derive(Component, Clone, Default)]
pub struct LocationGrid;

#[derive(Component, Clone, Default)]
pub struct Pin;

#[derive(Component, Clone, Default)]
pub struct ClueInput;

#[derive(Component, Clone, Default)]
pub struct ClueInputContainer;

#[derive(Component, Clone, Default)]
pub struct DoneButton;

#[derive(Component, Clone, Default)]
pub struct ConfirmationModal;

#[derive(Component, Clone, Default)]
pub struct ClueReadback;

/// A round of Pinpoint that can be shared with friends.
/// It can be deserialized
#[derive(Reflect, Resource, Clone, Hash, PartialEq, Eq)]
#[reflect(Resource)]
pub struct ShareableCreatedRound {
    /// The creator of this round
    creator: String,
    /// The date of this round
    date: String,
    /// The time this round was created.
    /// In combination with creator and date, uniquely identifies a created round.
    create_time: String,
    /// The clue the creator has given for this round.
    clue: String,
    /// The "correct answer" of this round.
    /// This is the location the creator was given that they
    /// crafted the clue from.
    location: UVec2,
}

/// A round of Pinpoint that is saved on the creator's end.
#[derive(Reflect, Resource, Default, SettingsGroup, Clone, Hash, PartialEq, Eq)]
#[reflect(Resource, Default, SettingsGroup)]
pub struct CreatedRound {
    /// The date of this round
    date: String,
    /// The time this round was created.
    /// In combination with creator and date, uniquely identifies a created round.
    create_time: String,
    /// The clue the creator has given for this round.
    clue: String,
    /// The "correct answer" of this round.
    /// This is the location the creator was given that they
    /// crafted the clue from.
    location: UVec2,
    /// Whether this created round is still under draft or locked in (no more edits allowed).
    /// A drafted created round is not shareable.
    is_draft: bool,
}

impl ShareableCreatedRound {
    /// Returns the identifier for this created round.
    /// Used to detect whether this player has played this round already.
    pub fn get_identifier(&self) -> String {
        return format!("{}-{}-{}", self.date, self.create_time, self.creator);
    }

    /// Returns the distance from `location` to the `guessed_location`
    pub fn get_distance(&self, guessed_location: UVec2) -> f32 {
        self.location
            .as_vec2()
            .distance_squared(guessed_location.as_vec2())
    }
}

/// System that preps the `CreatedRound` resource.
pub fn init_created_round(
    mut commands: Commands,
    start_date_time: Res<StartDateTime>,
    created_round: Option<ResMut<CreatedRound>>,
) {
    let mut rng = rand_pcg::Pcg32::from_rng(&mut rand::rng());

    if let Some(created_round) = created_round
        && created_round.date == start_date_time.date
    {
        return;
    }

    let location: UVec2 = UVec2::new(rng.random_range(0..=100), rng.random_range(0..=100));
    let round = CreatedRound {
        date: start_date_time.date.clone(),
        create_time: start_date_time.time.clone(),
        clue: "".to_string(),
        location,
        is_draft: true,
    };
    commands.insert_resource(round);
    commands.queue(SaveSettingsSync::Always);
}

pub fn setup_create(mut commands: Commands, created_round: Res<CreatedRound>) {
    commands.spawn_scene_list(setup_create_vertical(&created_round));
}

pub fn show_create(app_create_q: Single<&mut Visibility, With<AppCreate>>) {
    *app_create_q.into_inner() = Visibility::Inherited;
}

pub fn hide_create(app_create_q: Single<&mut Visibility, With<AppCreate>>) {
    *app_create_q.into_inner() = Visibility::Hidden;
}

fn setup_create_vertical(created_round: &CreatedRound) -> impl SceneList {
    bsn_list! {
        confirmation_modal(created_round),

        AppCreate
        Visibility::Hidden
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            row_gap: px(20),
            width: percent(100),
            height: percent(100),
        }
        Children [
            LocationGrid
            Node {
                border: px(5),
            }
            BorderColor::all(Color::WHITE)
            Children [
                Node {
                    min_width: px(280),
                    min_height: px(280),
                }
                ImageNode {
                    image: "game_area/grid.png"
                },

                Pin
                Node {
                    position_type: PositionType::Absolute,
                    // We subtract 7.5 so that the pin center is exactly where
                    // we want it to be.
                    // 42 (size of crosshair) / 2 = 21.
                    // the bullseye center is at 21 x 21, so we want the bottom
                    // left of the crosshair below and to the left of where the
                    // center should go by 21 / 280 = 7.5%
                    left: percent(created_round.location.x as f32 - 7.5),
                    bottom: percent(created_round.location.y as f32 - 7.5),
                }
                ZIndex(1)
                Children [
                    Node {
                        width: px(42),
                        height: px(42),
                    }
                    ImageNode {
                        image: "game_area/crosshair.png"
                    }
                ],
            ],

            axes_descriptions(&created_round.date),

            // Text Input
            clue_input_container(created_round),

            done_button(created_round),
        ]
    }
}

fn clue_input_container(created_round: &CreatedRound) -> impl Scene {
    let text = if created_round.is_draft {
        Text::new("Type in Your Clue")
    } else {
        Text::new("Your Clue")
    };

    let clue_input = || -> Box<dyn Scene> {
        let clue = created_round.clue.clone();
        if created_round.is_draft {
            Box::new(bsn! {
                ClueInput
                Node {
                    min_width: px(280),
                    border: px(5),
                    border_radius: BorderRadius::all(px(10)),
                }
                // While EditableText is weird in Bevy 0.19,
                // Allow for new lines so that rendering for
                // viewport can be avoided via user circumvention.
                template_value({
                    let mut editable = EditableText::new(clue);
                    editable.max_characters = Some(50);
                    editable.visible_lines = Some(3.);
                    editable.allow_newlines = true;
                    editable
                })
                TextFont {
                    font_size: FontSize::Rem(1.)
                }
                pinpoint_font()
                TextLayout::new(Justify::Left, LineBreak::WordOrCharacter)
                TabIndex(0)
                TextCursorStyle::default()
                BackgroundColor(Color::BLACK)
                BorderColor::all(Color::BLACK)
                on(on_pointer_over_text_cursor)
                on(on_pointer_out_default_cursor)
            })
        } else {
            // TODO scrollbar?
            Box::new(bsn! {
                ClueInput
                Node {
                    min_width: px(280),
                    border: px(5),
                    border_radius: BorderRadius::all(px(10)),
                }
                Text::new(format!("{}", clue))
                TextFont {
                    font_size: FontSize::Rem(1.)
                }
                pinpoint_font()
                TextLayout::new(Justify::Left, LineBreak::WordOrCharacter)
                BackgroundColor(Color::BLACK)
                BorderColor::all(Color::BLACK)
            })
        }
    };

    bsn! {
        ClueInputContainer
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: percent(5),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
        }
        Children [
            Node {
                width: percent(100),
            }
            Children [
                Node {
                    width: percent(100),
                }
                template_value(text)
                TextFont {
                    font_size: px(16),
                }
                TextLayout::justify(Justify::Center)
                pinpoint_font()
            ],

            clue_input(),
        ]
    }
}

/// System that sets the username when the editable text field is modified.
fn on_changed_clue_input(
    mut clue_input_q: Query<
        (&EditableText, &mut BorderColor),
        (
            With<ClueInput>,
            Without<DoneButton>,
            Without<InteractionDisabled>,
        ),
    >,
    clue_readback_q: Query<Entity, With<ClueReadback>>,
    mut needs_valid_clue_input_q: Query<
        (Entity, &Hovered, &mut BorderColor),
        (With<DoneButton>, Without<ClueInput>),
    >,
    children_query: Query<&Children>,
    mut image_q: Query<&mut ImageNode>,
    mut created_round: ResMut<CreatedRound>,
    mut commands: Commands,
) {
    let Ok((editable_text, mut border_color)) = clue_input_q.single_mut() else {
        return;
    };

    let new_clue = editable_text.value().to_string();
    if new_clue == created_round.clue {
        return;
    }
    created_round.clue = new_clue.replace("\n", " ");
    commands.queue(SaveSettingsDeferred::default());

    for readback in clue_readback_q.iter() {
        commands
            .entity(readback)
            .insert(Text::new(format!("{}", created_round.clue)));
    }

    if created_round.clue.is_empty() {
        *border_color = BorderColor::all(DARK_RED_COLOR);

        for (entity, _, mut border_color) in needs_valid_clue_input_q.iter_mut() {
            commands.entity(entity).insert(InteractionDisabled);
            change_image_node_index(entity, 3, &children_query, &mut image_q);
            *border_color = BorderColor::all(DARK_GRAY_COLOR);
        }
    } else {
        *border_color = BorderColor::all(Color::BLACK);

        for (entity, is_hovered, mut border_color) in needs_valid_clue_input_q.iter_mut() {
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

fn done_button(created_round: &CreatedRound) -> impl Scene {
    let maybe_disabled = || -> Box<dyn Scene> {
        if created_round.clue.is_empty() {
            Box::new(bsn! {
                InteractionDisabled
                // Override border color
                BorderColor::all(DARK_GRAY_COLOR)
                base_button("button/done.png", UVec2::new(170, 32), 15, 50, 3, 4, 5)
            })
        } else {
            Box::new(bsn! {
                base_button("button/done.png", UVec2::new(170, 32), 15, 50, 0, 4, 5)
            })
        }
    };
    let node = || -> Box<dyn Scene> {
        if created_round.is_draft {
            Box::new(bsn! {
                Node {
                    width: Val::Auto,
                    max_width: px(280),
                }
            })
        } else {
            Box::new(bsn! {
                Node {
                    display: Display::None,
                    width: Val::Auto,
                    max_width: px(280),
                }
            })
        }
    };
    bsn! {
        DoneButton
        on_click_if_inactive()
        Hovered::default()
        node()
        maybe_disabled()
        on(|_: On<Activate>,
            modal_q: Single<&mut Visibility, With<ConfirmationModal>>| {
                *modal_q.into_inner() = Visibility::Inherited;
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
            clue_input_q: Query<Entity, With<ClueInput>>,
            mut focus: ResMut<InputFocus>| {
            if let Ok(is_disabled) = has_interaction_disabled_q.get(event.entity) && is_disabled &&
                let Ok(input_entity) = clue_input_q.single_inner() {
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

/// Confirmation modal that pops up when the user clicks the Done button
/// on an in-draft created round.
fn confirmation_modal(created_round: &CreatedRound) -> impl Scene {
    let clue = created_round.clue.clone();
    bsn! {
        ConfirmationModal
        GlobalZIndex(1)
        Visibility::Hidden
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
        }
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
                Text::new("Are you sure you are done?")
                pinpoint_font(),

                Text::new("You cannot edit afterwards!")
                pinpoint_font(),

                Text::new("Your clue:")
                pinpoint_font(),

                ClueReadback
                Text::new(format!("{clue}\n"))
                pinpoint_font()
                TextColor(MIDDLE_BLUE_COLOR),

                Node {
                    flex_direction: FlexDirection::Row,
                    width: percent(65),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                }
                Children [
                    Button
                    Node {
                        width: px(75),
                        border: px(5),
                    }
                    BorderColor::all(DARK_RED_COLOR)
                    Children [
                        Node {
                            height: percent(100),
                            width: percent(100),
                            padding: px(5),
                        }
                        image_node_with_texture_atlas("button/confirmation_modal.png", UVec2::new(32, 32), 1, 2)
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
                    on(|_: On<Activate>,
                        modal_q: Single<&mut Visibility, With<ConfirmationModal>>| {
                            *modal_q.into_inner() = Visibility::Hidden;
                    }),

                    Button
                    Node {
                        width: px(75),
                        border: px(5),
                    }
                    BorderColor::all(DARK_GREEN_COLOR)
                    Children [
                        Node {
                            height: percent(100),
                            width: percent(100),
                            padding: px(5),
                        }
                        image_node_with_texture_atlas("button/confirmation_modal.png", UVec2::new(32, 32), 0, 2)
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
                    on(|_: On<Activate>,
                        mut round: ResMut<CreatedRound>,
                        clue_input_container_q: Single<Entity, With<ClueInputContainer>>,
                        mut need_to_hide_q: Query<&mut Visibility, With<ConfirmationModal>>,
                        done_button_q: Single<Entity, With<DoneButton>>,
                        app_create_q: Single<Entity, With<AppCreate>>,
                        mut commands: Commands,| {
                            round.is_draft = false;
                            commands.queue(SaveSettingsSync::Always);
                            commands.entity(clue_input_container_q.entity()).despawn();
                            commands.entity(done_button_q.entity()).despawn();
                            let child = commands.spawn_scene(clue_input_container(&round)).id();
                            commands.entity(app_create_q.entity()).add_child(child);

                            for mut vis in need_to_hide_q.iter_mut() {
                                *vis = Visibility::Hidden;
                            }
                    }),
                ]
            ]
        ]
    }
}

pub(crate) struct CreatePlugin;

impl Plugin for CreatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            on_changed_clue_input.run_if(in_state(AppState::Create)),
        );
    }
}

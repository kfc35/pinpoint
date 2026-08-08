use bevy::{
    picking::hover::Hovered,
    prelude::*,
    settings::SaveSettingsSync,
    ui::InteractionDisabled,
    ui_widgets::{Activate, ValueChange},
};

use crate::{
    StartDateTime,
    animation::{AnimatedImageNode, AnimationTimer},
    axes_descriptions,
    load::LoadableRounds,
    ui::{
        DARK_COLOR, DARK_GRAY_COLOR, DARK_RED_COLOR, Modal, MovablePin, PrimaryButtonContainer,
        base_button, bottom_buttons, change_image_node_index, confirmation_button, location_grid,
        on_pointer_out_default_cursor, on_pointer_over_pointer_cursor, pinpoint_font,
        share_primary_button, update_pin_location, update_pin_node_with_location,
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
pub struct PlayPrimaryButtonContainer;

const DIRECTIONS_TEXT: &'static str = "Press where the clue is";

/// This resource should only exist when in [`AppState::Play`].
/// This is set by the game loader when the play button is pressed.
#[derive(Resource)]
pub struct PlayRound {
    /// The current round as an index into [`crate::load::LoadableRounds`]
    loadable_rounds_index: usize,
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

/// System that is called when [`AppState::Play`] is entered.
/// The [`PlayRound`] resource must be available before this is called.
pub fn show_play(
    to_show_q: Single<&mut Visibility, With<AppPlay>>,
    movable_pin_q: Single<(&mut Node, &mut MovablePin)>,
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
    let (mut node, mut movable_pin) = movable_pin_q.into_inner();
    let button_container = primary_button_q.into_inner();
    match location {
        Some(loc) => {
            movable_pin.0 = false;
            commands.insert_resource(CurrentGuess(Some(loc)));
            update_pin_node_with_location(&mut node, loc);
            commands
                .entity(button_container)
                .queue_spawn_related_scenes::<Children>(bsn! {
                    share_primary_button()
                    create_on_activate_share_link()
                });
        }
        None => {
            movable_pin.0 = true;
            commands.insert_resource(CurrentGuess(None));
            node.display = Display::None;
            // TODO deactivated done button.
        }
    };

    let playable_round = loadable_rounds
        .get_round(play_round.loadable_rounds_index)
        .as_playable_round(&app_type_registry);

    let mut creator_text = creator_text_q.into_inner();
    *creator_text = Text::new(format!("Clue by {}", playable_round.get_creator()));

    let mut clue_text = clue_text_q.into_inner();
    *clue_text = Text::new(format!("{}", playable_round.get_clue()));

    *to_show_q.into_inner() = Visibility::Inherited;
}

/// System that is called when [`AppState::Play`] is left.
pub fn hide_play(
    mut to_hide_q: Query<&mut Visibility, Or<(With<AppPlay>, With<Modal>)>>,
    mut commands: Commands,
) {
    for mut vis in to_hide_q.iter_mut() {
        *vis = Visibility::Hidden;
    }

    commands.remove_resource::<PlayRound>();
    commands.remove_resource::<CurrentGuess>();
}

pub fn setup_play_skeleton(mut commands: Commands, start_date_time: Res<StartDateTime>) {
    commands.spawn_scene_list(play_skeleton(&start_date_time));
}

pub fn play_skeleton(start_date_time: &Res<StartDateTime>) -> impl SceneList {
    bsn_list! {
        // confirmation_modal(),
        // share_modal(),

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

            location_grid(None, true)
            on(|event: On<ValueChange<UVec2>>,
                mut current_guess: ResMut<CurrentGuess>,
                pin_q: Single<(&mut Node, &MovablePin)>|{
                    current_guess.0 = Some(event.value);
                    update_pin_location(event.value, pin_q);
            })
            on(on_pointer_over_pointer_cursor)
            on(on_pointer_out_default_cursor)
            ,

            axes_descriptions(&start_date_time.date)
            ,

            clue_placeholder()
            ,

            primary_button_placeholder()
            ,

            bottom_buttons(Box::new(bsn!{})
        ),
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

fn clue_placeholder() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            min_width: px(280),
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
                    let distance = get_distance_text(encoded_round.get_guess_distance(&app_type_registry));
                    let link = format!("https://kfc35.github.io/pinpoint/?share={}", encoded_round.get_encoded_value());

                    let playable_round = loadable_rounds.get_round(play_round.loadable_rounds_index).as_playable_round(&app_type_registry);
                    let title = format!("#Pinpoint - {} - {}", start_date_time.date, playable_round.get_creator());
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

fn get_distance_text(distance: f32) -> String {
    if distance < 3.125 {
        format!("🎯 {distance:.3} away from pin")
    } else if distance < 6.25 {
        format!("🟩 {distance:.3} away from pin")
    } else if distance < 12.5 {
        format!("🟨 {distance:.3} away from pin")
    } else if distance < 25. {
        format!("🟧 {distance:.3} away from pin")
    } else {
        format!("🟥 {distance:.3} away from pin")
    }
}

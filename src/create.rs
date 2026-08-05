use bevy::{
    input_focus::tab_navigation::TabIndex,
    prelude::*,
    reflect::{Reflect, std_traits::ReflectDefault},
    settings::{ReflectSettingsGroup, SaveSettingsSync, SettingsGroup},
    text::{EditableText, TextCursorStyle},
};

use crate::{
    StartDateTime, axes_descriptions,
    ui::{on_pointer_out_default_cursor, on_pointer_over_text_cursor, pinpoint_font},
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
pub struct DoneButton;

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
    clue: Option<String>,
    /// The "correct answer" of this round.
    /// This is the location the creator was given that they
    /// crafted the clue from.
    location: UVec2,
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
        clue: None,
        location,
    };
    commands.insert_resource(round);
    commands.queue(SaveSettingsSync::Always);
}

pub fn setup_create(mut commands: Commands, created_round: Res<CreatedRound>) {
    commands.spawn_scene(setup_create_vertical(&created_round));
}

pub fn show_create(app_create_q: Single<&mut Visibility, With<AppCreate>>) {
    *app_create_q.into_inner() = Visibility::Inherited;
}

pub fn hide_create(app_create_q: Single<&mut Visibility, With<AppCreate>>) {
    *app_create_q.into_inner() = Visibility::Hidden;
}

fn setup_create_vertical(created_round: &CreatedRound) -> impl Scene {
    bsn! {
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
            clue_input(),


        ]
    }
}

fn clue_input() -> impl Scene {
    bsn! {
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
                Text::new("Type in Your Clue")
                TextFont {
                    font_size: px(16),
                }
                TextLayout::justify(Justify::Center)
                pinpoint_font()
            ],

            ClueInput
            Node {
                min_width: px(280),
                border: px(5),
                border_radius: BorderRadius::all(px(10)),
            }
            // While EditableText is weird in Bevy 0.19,
            // Allow for new lines so that rendering for
            // viewport can be avoided via user circumvention.
            template_value(EditableText {
                max_characters: Some(50),
                visible_lines: Some(3.),
                allow_newlines: true,
                ..default()
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
            on(on_pointer_out_default_cursor),
        ]
    }
}

fn done_button() -> impl Scene {
    bsn! {}
}

fn confirmation_modal() -> impl Scene {
    bsn! {}
}

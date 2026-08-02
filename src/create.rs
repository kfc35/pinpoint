use bevy::{
    prelude::*,
    reflect::{Reflect, std_traits::ReflectDefault},
    settings::{ReflectSettingsGroup, SettingsGroup, SettingsPlugin},
    ui_widgets::Slider,
};

use crate::{StartDateTime, axes_descriptions, pinpoint_font};
use rand::{RngExt, SeedableRng};

/// Marker component for the menu
#[derive(Component, Clone, Default)]
pub struct AppCreate;

/// Marker component for the location grid
#[derive(Component, Clone, Default)]
pub struct LocationGrid;

/// Marker component for the pin
#[derive(Component, Clone, Default)]
pub struct Pin;

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
    pin: String,
    /// The "correct answer" of this round.
    /// This is the location the creator was given that they
    /// crafted the clue from.
    location: UVec2,
}

/// A round of Pinpoint that is saved on the creator's end.
#[derive(Reflect, Resource, SettingsGroup, Clone, Hash, PartialEq, Eq)]
// #[reflect(Resource, Default, SettingsGroup)]
pub struct CreatedRound {
    /// The date of this round
    date: String,
    /// The time this round was created.
    /// In combination with creator and date, uniquely identifies a created round.
    create_time: String,
    /// The clue the creator has given for this round.
    pin: Option<String>,
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
    println!("location: {location:?}");
    let round = CreatedRound {
        date: start_date_time.date.clone(),
        create_time: start_date_time.time.clone(),
        pin: None,
        location,
    };
    commands.insert_resource(round);
}

// TODO we should hook an observer to change the layout depending on if
// height is larger or width is larger
pub fn setup_create(mut commands: Commands, created_round: Res<CreatedRound>) {
    commands.spawn_scene(setup_create_vertical(&created_round));
}

fn setup_create_vertical(created_round: &CreatedRound) -> impl Scene {
    bsn! {
        AppCreate
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            row_gap: px(20),
            width: percent(100),
            height: percent(100),
        }
        Children [
            Node {
                width: percent(100),
            }
            Children [
                Node {
                    width: percent(100),
                }
                Text::new("Type in your clue")
                TextFont {
                    font_size: px(16),
                }
                TextLayout::justify(Justify::Center)
                pinpoint_font()
            ],

            LocationGrid
            Node
            Outline::new(px(5), Val::ZERO, Color::WHITE)
            BorderColor::all(Color::WHITE)
            Children [
                Node {
                    min_width: px(300),
                    min_height: px(300),
                }
                ImageNode {
                    image: "game_area/grid.png"
                },

                Pin
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(created_round.location.x as f32 -5.),
                    bottom: percent(created_round.location.y as f32 - 5.),
                }
                ZIndex(1)
                Children [
                    Node {
                        width: px(30),
                        height: px(30),
                    }
                    ImageNode {
                        image: "game_area/crosshair.png"
                    }
                ],
                // Should be able to switch views between 2d grid
                // and two spectrums
            ],

            axes_descriptions(&created_round.date),

            // Text Input

        ]
    }
}

pub fn teardown_create(mut commands: Commands, app_create_q: Single<Entity, With<AppCreate>>) {
    commands.entity(app_create_q.entity()).despawn();
}

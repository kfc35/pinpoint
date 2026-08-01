use bevy::{prelude::*, reflect::Reflect, settings::SettingsGroup, ui_widgets::Slider};

use crate::{StartDateTime, pinpoint_font};
use rand::{RngExt, SeedableRng};

/// Marker component for the menu
#[derive(Component, Clone, Default)]
pub struct AppCreate;

/// Marker component for the location square
#[derive(Component, Clone, Default)]
pub struct LocationSquare;

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
#[reflect(Resource)]
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

    let location: UVec2 = rng.random();
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
pub fn setup_create(mut commands: Commands, created_round: ResMut<CreatedRound>) {
    commands.spawn_scene(setup_create_vertical());
}

fn setup_create_vertical() -> impl Scene {
    bsn! {
        AppCreate
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Default,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
        }
        Children [
            // The middle portion of the children must be a square grid, minimum
            // 300 by 300 grid
            Node
            Children [
                Text::new("Type In Your Clue for the given pin")
                TextFont {
                    font_size: px(16),
                }
                pinpoint_font(),

                LocationSquare
                Node {
                    min_width: px(300),
                    min_height: px(300),
                    // TODO this has to change depending on orientation.
                    width: percent(100),
                }
                ImageNode {
                    image: "game_area/grid.png"
                }
                Children [
                    // Location of pin must be a child.

                    // Should be able to switch views between 2d grid
                    // and two spectrums
                ]
            ]
        ]
    }
}

pub fn teardown_create(mut commands: Commands, app_create_q: Single<Entity, With<AppCreate>>) {
    commands.entity(app_create_q.entity()).despawn();
}

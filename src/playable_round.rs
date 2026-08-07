use crate::{AppState, EncodedRound, StartDateTime, Username, create::CreatedRound};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use bevy::{prelude::*, reflect::serde::TypedReflectSerializer, settings::SaveSettingsSync};

/// A round of Pinpoint that can be loaded into play.
/// It was decoded from an [`EncodedRound`].
#[derive(Reflect, Clone, Hash, PartialEq, Eq)]
pub struct PlayableRound {
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

impl PlayableRound {
    pub fn from_current_user(username: &Username, created_round: &CreatedRound) -> Self {
        Self {
            creator: username.0.clone(),
            date: created_round.get_date().clone(),
            create_time: created_round.get_create_time().clone(),
            clue: created_round.get_clue().clone(),
            location: created_round.get_location(),
        }
    }

    /// Returns the identifier for this created round.
    /// Used to detect whether this player has played this round already.
    pub fn get_identifier(&self) -> String {
        return format!("{}-{}-{}", self.date, self.create_time, self.creator);
    }
}

/// A system that will encode a newly [`CreatedRound`] as a [`PlayableRound`]
/// so that it can be shared with others.
pub(crate) fn set_encoded_round_resource(
    username: Res<Username>,
    created_round: Res<CreatedRound>,
    mut encoded_round: ResMut<EncodedRound>,
    type_registry: Res<AppTypeRegistry>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    let type_registry = type_registry.read();
    let round = PlayableRound::from_current_user(&username, &created_round);
    let serializer = TypedReflectSerializer::new(&round, &type_registry);
    let json = serde_json::to_string(&serializer).unwrap();
    let value = URL_SAFE.encode(json);

    encoded_round.date = created_round.get_date().clone();
    encoded_round.value = value;
    commands.queue(SaveSettingsSync::Always);
    Ok(())
}

pub(crate) struct EncodedRoundCreationPlugin;

impl Plugin for EncodedRoundCreationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                set_encoded_round_resource,
                crate::create::update_create_ui_after_encoding.run_if(in_state(AppState::Create)),
            )
                .chain()
                .run_if(
                    |start_date_time: Res<StartDateTime>,
                     encoded_round: Res<EncodedRound>,
                     created_round: Res<CreatedRound>| {
                        encoded_round.date != start_date_time.date
                            && !created_round.get_is_draft()
                            && *created_round.get_date() == start_date_time.date
                    },
                ),
        );
    }
}

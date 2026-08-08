use crate::{AppState, EncodedRound, StartDateTime, Username, create::CreatedRound};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use bevy::{prelude::*, reflect::serde::TypedReflectSerializer, settings::SaveSettingsSync};

/// A round of Pinpoint that can be loaded into play.
/// It can be decoded from an [`EncodedRound`].
#[derive(Reflect, Clone, Debug, Hash, PartialEq, Eq)]
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
    /// Create a PlayableRound from the currently logged in user and the round
    /// they created.
    pub fn from_current_user(username: &Username, created_round: &CreatedRound) -> Self {
        Self {
            creator: username.0.clone(),
            date: created_round.get_date().clone(),
            create_time: created_round.get_create_time().clone(),
            clue: created_round.get_clue().clone(),
            location: created_round.get_location(),
        }
    }

    /// Uniquely identifies this playable round from others.
    pub fn get_identifier(&self) -> String {
        format!("{}-{}-{}", self.date, self.create_time, self.creator)
    }

    /// Gets the creator of this round
    pub fn get_creator(&self) -> &String {
        &self.creator
    }

    /// Gets the date of this round
    pub fn get_date(&self) -> &String {
        &self.date
    }

    /// Gets the clue the creator made for this round.
    pub fn get_clue(&self) -> &String {
        &self.clue
    }

    /// Gets the location (the answer) for this round.
    pub(crate) fn get_location(&self) -> UVec2 {
        self.location
    }
}

/// Encodes a newly [`CreatedRound`] as an [`EncodedRound`] so that it can be shared with others.
pub(crate) fn set_encoded_round_resource(
    username: &Res<Username>,
    created_round: &ResMut<CreatedRound>,
    encoded_round: &mut ResMut<EncodedRound>,
    type_registry: &Res<AppTypeRegistry>,
) {
    let type_registry = type_registry.read();
    let round = PlayableRound::from_current_user(&username, &created_round);
    let serializer = TypedReflectSerializer::new(&round, &type_registry);
    let json = serde_json::to_string(&serializer).unwrap();
    let value = URL_SAFE.encode(json);

    encoded_round.0 = value;
}

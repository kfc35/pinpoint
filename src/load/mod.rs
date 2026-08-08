use crate::{EncodedRound, playable_round::PlayableRound};
use bevy::{
    prelude::*,
    settings::{ReflectSettingsGroup, SettingsGroup},
};

mod load_modal;
pub use load_modal::load_modal;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// A round that the current user can play / has played.
#[derive(Reflect, Clone, Default)]
#[reflect(Default)]
pub(crate) struct LoadableRound {
    /// An encoded round that is loadable. The round MUST be infallible to decode.
    /// The encoded round is stored instead of the decoded [`crate::PlayableRound`]
    /// so that [`LoadableRounds`] does not leak solutions easily in the file system / session storage.
    round: EncodedRound,
    /// The current user's finalized guess. [`None`] means that the user has not played this round yet.
    final_guess: Option<UVec2>,
}

impl LoadableRound {
    pub(crate) fn new(round: EncodedRound) -> Self {
        Self {
            round,
            final_guess: None,
        }
    }

    /// Gets this LoadableRound as a [`PlayableRound`].
    pub(crate) fn get_round_as_playable_round(
        &self,
        app_type_registry: &AppTypeRegistry,
    ) -> PlayableRound {
        self.round.decode(app_type_registry)
    }

    /// Updates this LoadableRound after the user has submitted their final guess.
    pub(crate) fn set_final_guess(&mut self, guess: UVec2) {
        if self.final_guess.is_none() {
            self.final_guess = Some(guess);
        }
    }

    /// Fetches the location in the [`PlayableRound`] this represents.
    /// Can only access the answer after the final guess has been submitted.
    pub(crate) fn get_answer(&self, app_type_registry: &AppTypeRegistry) -> Option<UVec2> {
        if self.final_guess.is_some() {
            Some(
                self.get_round_as_playable_round(app_type_registry)
                    .get_location(),
            )
        } else {
            None
        }
    }

    /// Fetches the distance from the guess the the location in [`PlayableRound`].
    /// Can only access the answer after the final guess has been submitted.
    pub(crate) fn get_guess_distance(&self, app_type_registry: &AppTypeRegistry) -> Option<f32> {
        if let Some(guess) = self.final_guess {
            self.get_answer(app_type_registry)
                .map(|location| location.as_vec2().distance(guess.as_vec2()))
        } else {
            None
        }
    }
}

/// The rounds that the current user can load.
/// The user receives these rounds from shareable links, which contain a [`crate::EncodedRound`].
/// This resource will never contain any of the current user's created rounds.
/// The current user's created round will be available as the [`crate::EncodedRound`] resource.
/// Currently, we limit loadable rounds to only contain rounds from the current day.
#[derive(Resource, Reflect, Clone, Default, Deref, DerefMut, SettingsGroup)]
#[reflect(Resource, Default, SettingsGroup)]
pub(crate) struct LoadableRounds {
    pub(crate) rounds: Vec<LoadableRound>,
}

#[cfg(target_arch = "wasm32")]
/// A System that runs on startup on the web.
/// If the user got to this page via clicking a share link, it parses the share link
/// into a [`LoadableRound`].
pub fn parse_share_link(
    start_date_time: Res<StartDateTime>,
    app_type_registry: Res<AppTypeRegistry>,
    mut loadable_rounds: ResMut<LoadableRounds>,
    mut menu_header_text: ResMut<MenuHeaderText>,
) {
    if let Some(window) = window()
        && let Some(document) = window.document()
        && let Ok(url) = document.url()
    {
        if !url.contains("?share=") {
            return;
        }
        let Some(encoded) = url.strip_prefix("https://kfc35.github.io/pinpoint/?share=") else {
            return;
        };

        let encoded_round = EncodedRound(encoded.to_string());
        let Some(playable_round) = encoded_round.try_decode(&app_type_registry) else {
            menu_header_text.0 =
                "Round cannot be loaded. If this was a mistake, please double-check the URL."
                    .to_string();
            return;
        };

        if *playable_round.get_date() != start_date_time.date {
            menu_header_text.0 = "Round invite expired - It's for an earlier day.".to_string();
            return;
        }

        let identifier = playable_round.get_identifier();
        if loadable_rounds
            .iter()
            .map(|round| round.get_round_as_playable_round(&app_type_registry))
            .any(|round| round.get_identifier() == identifier)
        {
            menu_header_text.0 = format!(
                "Round from {} is already loadable.",
                playable_round.get_creator()
            );
            return;
        }

        loadable_rounds.push(LoadableRound::new(encoded_round));
    }
}

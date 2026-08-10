use crate::{EncodedRound, StartDateTime, playable_round::PlayableRound};
use bevy::{
    prelude::*,
    settings::{ReflectSettingsGroup, SaveSettingsSync, SettingsGroup},
};

mod load_modal;
pub use load_modal::{load_modal, on_activate_show_load_modal, on_changed_url_input};

#[cfg(target_arch = "wasm32")]
use crate::menu::MenuHeaderText;
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

    pub(crate) fn get_encoded_value(&self) -> &String {
        &self.round.0
    }

    /// Gets this LoadableRound as a [`PlayableRound`].
    pub(crate) fn as_playable_round(&self, app_type_registry: &AppTypeRegistry) -> PlayableRound {
        self.round.decode(app_type_registry)
    }

    /// Updates this LoadableRound after the user has submitted their final guess.
    pub(crate) fn set_final_guess(&mut self, guess: UVec2) {
        if self.final_guess.is_none() {
            self.final_guess = Some(guess);
        }
    }

    pub(crate) fn get_final_guess(&self) -> Option<UVec2> {
        self.final_guess
    }

    /// Fetches the location in the [`PlayableRound`] this represents.
    /// Can only access the answer after the final guess has been submitted.
    pub(crate) fn get_answer(&self, app_type_registry: &AppTypeRegistry) -> Option<UVec2> {
        if self.final_guess.is_some() {
            Some(self.as_playable_round(app_type_registry).get_location())
        } else {
            None
        }
    }

    /// Fetches the distance from the guess the the location in [`PlayableRound`].
    /// Can only access the answer after the final guess has been submitted.
    /// ### Panics
    /// Panics if the final guess has not been submitted. Only call this at the appropriate time.
    pub(crate) fn get_guess_distance(&self, app_type_registry: &AppTypeRegistry) -> f32 {
        self.get_answer(app_type_registry)
            .expect("Game must be over before calling `get_guess_distance`")
            .as_vec2()
            .distance(
                self.final_guess
                    .expect("Game must be over before calling `get_guess_distance`")
                    .as_vec2(),
            )
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

impl LoadableRounds {
    pub fn get_round(&self, idx: usize) -> &LoadableRound {
        &self.rounds[idx]
    }

    pub fn get_round_mut(&mut self, idx: usize) -> &mut LoadableRound {
        &mut self.rounds[idx]
    }
}

/// System that inits the [`LoadableRounds`] resource.
pub(crate) fn init_loadable_rounds(
    start_date_time: Res<StartDateTime>,
    loadable_rounds: Option<ResMut<LoadableRounds>>,
    app_type_registry: Res<AppTypeRegistry>,
    mut commands: Commands,
) {
    let Some(mut loadable_rounds) = loadable_rounds else {
        commands.init_resource::<LoadableRounds>();
        return;
    };
    let length = loadable_rounds.rounds.len();
    let new_rounds = loadable_rounds
        .rounds
        .clone()
        .into_iter()
        .filter(|round| {
            *round.as_playable_round(&app_type_registry).get_date() == start_date_time.date
        })
        .collect::<Vec<_>>();
    loadable_rounds.rounds = new_rounds;
    if loadable_rounds.rounds.len() != length {
        commands.queue(SaveSettingsSync::Always);
    }
}

/// Attempts to load a round from a share link.
/// Returns whether it was successful and a message that can be shown to the
/// user depending on what happened.
pub(crate) fn load_shared_round(
    mut url: String,
    start_date_time: &Res<StartDateTime>,
    app_type_registry: &Res<AppTypeRegistry>,
    loadable_rounds: &mut ResMut<LoadableRounds>,
    my_created_round: &Res<EncodedRound>,
    commands: &mut Commands,
) -> (bool, String) {
    let Some(index) = url.find("share=") else {
        return (false, "".to_string());
    };
    let encoded = url.split_off(index + "share=".len());
    let encoded_round = EncodedRound(encoded.to_string());
    let Some(playable_round) = encoded_round.try_decode(&app_type_registry) else {
        return (
            false,
            "Game cannot be added.\nIf this was a mistake, please double-check the URL."
                .to_string(),
        );
    };

    if my_created_round.0 == encoded_round.0 {
        return (
            false,
            "Game cannot be added.\nIt is your own game.".to_string(),
        );
    }

    if *playable_round.get_date() != start_date_time.date {
        return (
            false,
            "Game cannot be added.\nThe invite expired because it is for an earlier day."
                .to_string(),
        );
    }

    let identifier = playable_round.get_identifier();
    if loadable_rounds
        .iter()
        .map(|round| round.as_playable_round(&app_type_registry))
        .any(|round| round.get_identifier() == identifier)
    {
        return (
            false,
            format!(
                "Game from {} has already been added for today.",
                playable_round.get_creator()
            ),
        );
    }

    loadable_rounds.push(LoadableRound::new(encoded_round));
    commands.queue(SaveSettingsSync::Always);
    (
        true,
        format!(
            "Game from {} successfully added.",
            playable_round.get_creator()
        ),
    )
}

#[cfg(target_arch = "wasm32")]
/// A System that runs on startup on the web.
/// If the user got to this page via clicking a share link, it parses the share link
/// into a [`LoadableRound`].
pub fn parse_window_url(
    start_date_time: Res<StartDateTime>,
    app_type_registry: Res<AppTypeRegistry>,
    mut loadable_rounds: ResMut<LoadableRounds>,
    my_encoded_round: Res<EncodedRound>,
    mut menu_header_text: ResMut<MenuHeaderText>,
    mut commands: Commands,
) {
    if let Some(window) = window()
        && let Some(document) = window.document()
        && let Ok(url) = document.url()
    {
        menu_header_text.0 = load_shared_round(
            url,
            &start_date_time,
            &app_type_registry,
            &mut loadable_rounds,
            &my_encoded_round,
            &mut commands,
        )
        .1;
    }
}

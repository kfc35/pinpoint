use bevy::{
    input_focus::{FocusCause, InputFocus, tab_navigation::TabIndex},
    picking::hover::Hovered,
    prelude::*,
    reflect::{Reflect, std_traits::ReflectDefault},
    settings::{ReflectSettingsGroup, SaveSettingsDeferred, SaveSettingsSync, SettingsGroup},
    text::{EditableText, TextCursorStyle},
    ui::InteractionDisabled,
    ui_widgets::Activate,
};

use crate::{
    AppState, EncodedRound, StartDateTime, Username,
    animation::{AnimatedImageNode, AnimationTimer},
    axes_descriptions,
    load::LoadableRounds,
    ui::{
        ConfirmationButtonIndex, DARK_BLUE_COLOR, DARK_GRAY_COLOR, DARK_GREEN_COLOR,
        DARK_ORANGE_COLOR, DARK_RED_COLOR, MIDDLE_BLUE_COLOR, Modal, base_button,
        change_image_node_index, confirmation_button, on_activate_change_state,
        on_pointer_out_default_cursor, on_pointer_over_text_cursor, pinpoint_font,
    },
};

#[derive(Component, Clone, Default)]
pub struct AppPlay;

#[derive(Component, Clone, Default)]
pub struct PlayLocationGrid;

#[derive(Component, Clone, Default)]
pub struct PlayPin;

/// This resource should only exist when in [`AppState::Play`].
/// This is set by the game loader when the play button is pressed.
#[derive(Resource)]
pub struct PlayRound {
    /// The current round as an index into [`crate::load::LoadableRounds`]
    loadable_rounds_index: usize,
}

/// System that preps the `PlayRound` resource.
pub fn init_play_round(mut commands: Commands, selected_index: usize) {
    commands.insert_resource(PlayRound {
        loadable_rounds_index: selected_index,
    });
}

/// System that is called when [`AppState::Play`] is entered.
/// The [`PlayRound`] resource must be available before this is called.
pub fn show_play(to_show_q: Single<&mut Visibility, With<AppPlay>>, _play_round: Res<PlayRound>) {
    *to_show_q.into_inner() = Visibility::Inherited;

    // TODO fill in the correct places with the play round info.
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
}

pub fn setup_play_skeleton(mut commands: Commands) {
    commands.spawn_scene_list(bsn_list! {});
}

pub fn play_skeleton() -> impl Scene {}

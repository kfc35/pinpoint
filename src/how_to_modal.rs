use crate::{
    AppState,
    ui::{
        ConfirmationButtonIndex, DARK_BLUE_COLOR, DARK_ORANGE_COLOR, DARK_RED_COLOR,
        LIGHT_GREEN_COLOR, MIDDLE_BLUE_COLOR, MIDDLE_GREEN_COLOR, MIDDLE_RED_COLOR, Modal,
        ModalContent, YELLOW_COLOR, base_button, confirmation_button, pinpoint_font,
    },
};
use bevy::{
    prelude::*,
    ui_widgets::{Activate, ControlOrientation, ScrollArea, Scrollbar, ScrollbarThumb},
};

#[derive(Component, Default, Clone)]
pub struct HowToModal;

/// An observer that shows the load modal on activate.
pub fn on_activate_show_how_to_modal(
    _: On<Activate>,
    to_show_q: Single<&mut Visibility, With<HowToModal>>,
) {
    *to_show_q.into_inner() = Visibility::Inherited;
}

pub fn hide_load_modal(to_hide_q: Single<&mut Visibility, With<HowToModal>>) {
    *to_hide_q.into_inner() = Visibility::Hidden;
}

pub fn despawn_how_to_modal(modal_q: Single<Entity, With<HowToModal>>, mut commands: Commands) {
    commands.entity(modal_q.entity()).despawn();
}

/// Creates the how to modal
pub fn spawn_how_to_modal(app_state: Res<State<AppState>>, mut commands: Commands) {
    let content = || -> Box<dyn SceneList> {
        match **app_state {
            AppState::Menu => Box::new(bsn_list! {{menu_content()}}),
            AppState::Create => Box::new(bsn_list! {{clue_creator_content()}}),
            AppState::Play => Box::new(bsn_list! {{clue_receivers_content()}}),
        }
    };
    commands.spawn_scene( bsn! {
        // Background Node to center the modal.
        Modal
        HowToModal
        Visibility::Hidden
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100),
            height: percent(100),
        }
        GlobalZIndex(1)
        BackgroundColor({DARK_BLUE_COLOR.with_alpha(0.5)})
        Children [
            // Inset node
            Node {
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.),RepeatedGridTrack::auto(1)],
                border: px(5),
                width: percent(95),
                min_width: px(300),
                height: percent(90),
            }
            BorderColor::all(DARK_BLUE_COLOR)
            BackgroundColor(Color::BLACK)
            on(crate::ui::handle_mouse_drag_as_scroll)
            Children [
                ModalContent
                #Content
                ScrollArea
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(px(5), px(5)),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    overflow: Overflow::scroll_y(),
                }
                Children [
                    Node {
                        position_type: PositionType::Absolute,
                        top: px(5),
                        left: px(5),
                        width: px(50),
                        height: px(50),
                    }
                    ZIndex(1)
                    Children [
                        confirmation_button(DARK_RED_COLOR, ConfirmationButtonIndex::RedX)
                        on(|_: On<Activate>,
                            load_modal_q : Single<&mut Visibility, With<HowToModal>>| {
                                hide_load_modal(load_modal_q);
                        })
                    ]
                    ,

                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Start,
                        margin: UiRect::top(px(55))
                        width: percent(90),
                        height: percent(100),
                    }
                    Children [
                        { content() },
                        bottom_button(),
                    ]
                    ,
                ]
                ,

                // Scrollbar
                Node {
                    min_width: px(12),
                    height: percent(100),
                }
                BackgroundColor(Color::WHITE)
                Scrollbar {
                    orientation: ControlOrientation::Vertical,
                    target: #Content,
                    min_thumb_length: 8.0,
                }
                Children [
                    BorderColor::all(MIDDLE_BLUE_COLOR)
                    BackgroundColor(MIDDLE_BLUE_COLOR)
                    ScrollbarThumb {
                        border_radius: BorderRadius::all(px(4)),
                        border: UiRect::all(px(1)),
                    }
                ]
                ,
            ],
        ]
    });
}

fn menu_content() -> impl SceneList {
    bsn_list! {
        { introduction_content() },

        // Node {
        //     align_self: AlignSelf::Center,
        // }
        // Text::new("---")
        // pinpoint_font()
        // TextFont {
        //     font_size: FontSize::Rem(1.0),
        // }
        // ,

        { technical_limitations_content() },

        { clue_creator_content() },

        { clue_receivers_content() },
    }
}

fn introduction_content() -> impl SceneList {
    bsn_list! {
        Text::new("Pinpoint")
        pinpoint_font()
        TextFont {
            font_size: FontSize::Rem(1.3),
        }
        TextColor(MIDDLE_BLUE_COLOR)
        TextLayout::new(Justify::Left, LineBreak::WordOrCharacter)
        Children [
            TextSpan::new(" is a daily version of a party game similar to Wavelength and Codenames.\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.3),
            },

            TextSpan::new("Clue Creators")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new(" must ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,

            TextSpan::new("create a clue that conveys a randomized position ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("on a 2D grid with a given pair of daily axes.\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,

            TextSpan::new("Clue receivers")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new(" must ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,

            TextSpan::new("pinpoint where ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("on the 2D grid ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,

            TextSpan::new("the clue would lie. ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("Clue receivers are graded on how close their guess is to the actual position.\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,

            TextSpan::new("The axes change every Midnight on Eastern Time!\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_GREEN_COLOR)
            ,
        ]
    }
}

fn technical_limitations_content() -> impl SceneList {
    bsn_list! {
        Text::new("Technical Limitations:\n\n")
        pinpoint_font()
        TextFont {
            font_size: FontSize::Rem(1.3),
        }
        TextColor(MIDDLE_BLUE_COLOR)
        TextLayout::new(Justify::Left, LineBreak::WordOrCharacter)
        Children [
            TextSpan::new("Mobile Devices cannot access their soft keyboards for text inputs. ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_RED_COLOR),

            TextSpan::new("Click the keyboard icon above text inputs to use the in-app keyboard to type.\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },
        ]
    }
}

/// Content for clue creators. This appears in the menu how to and the creator screen question mark.
fn clue_creator_content() -> impl SceneList {
    bsn_list! {
        Text::new("Clue Creators:\n\n")
        pinpoint_font()
        TextFont {
            font_size: FontSize::Rem(1.3),
        }
        TextColor(MIDDLE_BLUE_COLOR)
        Children [
            TextSpan::new("Clue Creation Requires a Username!\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_GREEN_COLOR)
            ,

            TextSpan::new("Type in a clue ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("that conveys a randomized position on a 2D grid with the given daily axes.\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },

            TextSpan::new("Clue Creation Guidance:\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("- ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },
            TextSpan::new("Do not ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_RED_COLOR)
            ,
            TextSpan::new("convey the randomized position ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },
            TextSpan::new("using numbers ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_RED_COLOR)
            ,
            TextSpan::new("i.e. \"halfway left and at the very top\". Use the axes to guide what clue you should give.\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },


            TextSpan::new("- Try to ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },
            TextSpan::new("avoid qualifiers ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_RED_COLOR)
            ,
            TextSpan::new("i.e. \"very\", \"little\". Try to make your clue a singular concept.\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },

            TextSpan::new("- Your clue ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },

            TextSpan::new("must be 50 characters or less.\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_GREEN_COLOR)
            ,

            TextSpan::new("Note: You cannot edit your username for the day after creating a round.\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,
        ],
    }
}

/// Content for clue receivers. This appears in the menu how to and the play screen question mark.
fn clue_receivers_content() -> impl SceneList {
    bsn_list! {
        Text::new("Clue Receivers:\n\n")
        pinpoint_font()
        TextFont {
            font_size: FontSize::Rem(1.3),
        }
        TextColor(MIDDLE_BLUE_COLOR)
        Children [
            TextSpan::new("Simply ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            },

            TextSpan::new("click/press on the grid where you think the pin is ")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("based on the Clue Creator's clue.\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,

            TextSpan::new("Clue Receiver Performance Grades:\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_BLUE_COLOR)
            ,

            TextSpan::new("- Bullseye: <= 3 units away\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_GREEN_COLOR)
            ,

            TextSpan::new("- Great: <= 6.25 units away\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(LIGHT_GREEN_COLOR)
            ,

            TextSpan::new("- OK: <= 12.5 units away\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(YELLOW_COLOR)
            ,

            TextSpan::new("- Passing: <= 25 units away\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(DARK_ORANGE_COLOR)
            ,

            TextSpan::new("- Needs Improvement: > 25 units away\n\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            TextColor(MIDDLE_RED_COLOR)
            ,

            TextSpan::new("Do not feel bad if you did poorly! It may be the fault of the Clue Creator...\n")
            pinpoint_font()
            TextFont {
                font_size: FontSize::Rem(1.0),
            }
            ,
        ],
    }
}

fn bottom_button() -> impl Scene {
    bsn! {
        base_button("button/close.png", UVec2::new(128, 32), 10, 80, 0, 3, 5)
        Node {
            padding: UiRect::horizontal(px(3)),
            min_width: px(272),
            max_height: percent(10),
            align_self: AlignSelf::Center,
        }
        on(|_: On<Activate>,
            load_modal_q : Single<&mut Visibility, With<HowToModal>>| {
                hide_load_modal(load_modal_q);
        })
    }
}

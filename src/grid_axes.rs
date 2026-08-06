///! Module dealing with the assigned axes for a given day of Pinpoint.
use bevy::prelude::*;

use crate::ui::{image_node_with_texture_atlas, pinpoint_font};

/// Contains the axes for a given day of Pinpoint.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Axes {
    horizontal: AxisSpectrum,
    vertical: AxisSpectrum,
}

impl Axes {
    pub(crate) fn horizontal(&self) -> AxisSpectrum {
        self.horizontal
    }

    pub(crate) fn vertical(&self) -> AxisSpectrum {
        self.vertical
    }
}

/// Holds a possible axis that can be used for a given game.
/// The first entry is left/up. The right entry is right/bottom.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct AxisSpectrum(&'static str, &'static str);

impl AxisSpectrum {
    pub(crate) fn first(&self) -> &'static str {
        self.0
    }

    pub(crate) fn second(&self) -> &'static str {
        self.1
    }
}

/// Returns the Axes for a given day.
fn get_axes(_date: &String) -> Axes {
    AXES[0]
}

/// Returns the axes as a compass scene
pub(crate) fn axes_descriptions(date: &String) -> impl Scene {
    let axes = get_axes(date);
    let date = date.clone();
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: px(2),
            max_width: percent(100),
        }
        BorderColor::all(Color::WHITE)
        Children [
            Node
            Children [
                Text::new(format!("Axes for {}", date.clone()))
                pinpoint_font()
                TextFont {
                    font_size: FontSize::Rem(0.7)
                }
            ]
            ,
            axis_vertical_desc(axes.vertical().first(), 0),
            axis_horizontal_desc(axes.horizontal().first(), axes.horizontal().second()),
            axis_vertical_desc(axes.vertical().second(), 1),
        ]
    }
}

fn axis_vertical_desc(axis: &'static str, image_index: usize) -> Box<dyn Scene> {
    if image_index == 0 {
        // Pointing Up
        Box::new(bsn! {
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: percent(100)
                padding: px(5),
                row_gap: px(5),
            }
            Children [
                axis_text(axis),
                arrow_image_node(image_index),
            ]
        })
    } else {
        Box::new(bsn! {
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: percent(100)
                padding: px(5),
                row_gap: px(5),
            }
            Children [
                // Ordering of children is different.
                arrow_image_node(image_index),
                axis_text(axis),
            ]
        })
    }
}

fn axis_horizontal_desc(left_axis: &'static str, right_axis: &'static str) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100),
            column_gap: px(2),
            padding: UiRect::horizontal(px(5)),
        }
        Children [
            Node {
                flex_direction: FlexDirection::Row,
                // flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: px(2),
            }
            Children [
                axis_text(left_axis),
                arrow_image_node(2),
            ],

            Node {
                flex_direction: FlexDirection::Row,
                // flex_wrap: FlexWrap::WrapReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: px(2),
            }
            Children [
                arrow_image_node(3),
                axis_text(right_axis),
            ],
        ]
    }
}

fn axis_text(axis: &'static str) -> impl Scene {
    bsn! {
        Text::new(axis)
        TextFont {
            font_size: FontSize::Rem(0.8),
        }
        // TextLayout::linebreak(LineBreak::AnyCharacter)
        pinpoint_font()
    }
}

fn arrow_image_node(image_index: usize) -> impl Scene {
    bsn! {
        Node {
            min_width: px(15),
            min_height: px(15),
        }
        image_node_with_texture_atlas("game_area/arrows.png", UVec2::splat(15), image_index, 4)
    }
}

const AXIS_SPECTRA: [AxisSpectrum; 12] = [
    // Seasons
    AxisSpectrum("Springy", "Summery"),
    AxisSpectrum("Autumnal", "Wintry"),
    // Directions
    AxisSpectrum("Western", "Eastern"),
    AxisSpectrum("Northern", "Southern"),
    // US Cities
    AxisSpectrum("NYC", "BOS"),
    AxisSpectrum("SF", "LA"),
    // Knowledge
    AxisSpectrum("Obscure", "Well Known"),
    AxisSpectrum("Important", "Insignificant"),
    // Moments
    AxisSpectrum("Unpleasant", "Enjoyable"),
    AxisSpectrum("Rare", "Common"),
    // Moments
    AxisSpectrum("Breakfast", "Dinner"),
    AxisSpectrum("Diet", "Indulgent"),
];

const AXES: [Axes; 5] = [
    Axes {
        horizontal: AXIS_SPECTRA[0],
        vertical: AXIS_SPECTRA[1],
    },
    Axes {
        horizontal: AXIS_SPECTRA[2],
        vertical: AXIS_SPECTRA[3],
    },
    Axes {
        horizontal: AXIS_SPECTRA[4],
        vertical: AXIS_SPECTRA[5],
    },
    Axes {
        horizontal: AXIS_SPECTRA[6],
        vertical: AXIS_SPECTRA[7],
    },
    Axes {
        horizontal: AXIS_SPECTRA[8],
        vertical: AXIS_SPECTRA[9],
    },
];

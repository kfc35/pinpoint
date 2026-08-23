///! Module dealing with the assigned axes for a given day of Pinpoint.
use bevy::prelude::*;
use chrono::{Datelike, NaiveDate};

use crate::{
    StartDateTime,
    ui::{image_node_with_texture_atlas, pinpoint_font},
};

// 2026/08/09
const APP_BEGIN_NAIVE_DATE: NaiveDate = NaiveDate::from_ymd_opt(2026, 08, 09).unwrap();

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
fn get_axes(start_date_time: &Res<StartDateTime>) -> Axes {
    let index = (start_date_time.naive_date.num_days_from_ce()
        - APP_BEGIN_NAIVE_DATE.num_days_from_ce())
        % (AXIS_SPECTRA.len() as i32 / 2);

    Axes {
        horizontal: AXIS_SPECTRA[(index * 2) as usize],
        vertical: AXIS_SPECTRA[(index * 2 + 1) as usize],
    }
}

/// Returns the axes as a compass scene
pub(crate) fn axes_descriptions(date: &Res<StartDateTime>) -> impl Scene {
    let axes = get_axes(date);
    let date = date.date.clone();
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // border: px(2),
            row_gap: px(2),
            max_width: percent(100),
        }
        // BorderColor::all(Color::WHITE)
        Children [
            Node
            Children [
                Text::new(format!("Axes for {}", date))
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
                width: percent(100),
                max_width: px(300),
                padding: px(5),
                row_gap: px(5),
            }
            Children [
                Node
                Children[
                    axis_text(axis),
                ],

                arrow_image_node(image_index),
            ]
        })
    } else {
        Box::new(bsn! {
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: percent(100),
                max_width: px(300),
                padding: px(5),
                row_gap: px(5),
            }
            Children [
                // Ordering of children is different.
                arrow_image_node(image_index),

                Node
                Children[
                    axis_text(axis),
                ],
            ]
        })
    }
}

fn axis_horizontal_desc(left_axis: &'static str, right_axis: &'static str) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            // flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: percent(100),
            column_gap: px(2),
            padding: UiRect::horizontal(px(5)),
        }
        Children [
            Node {
                flex_direction: FlexDirection::Row,
                // flex_wrap: FlexWrap::Wrap,
                width: percent(49),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                column_gap: px(6),
            }
            Children [
                Node
                Children[
                    axis_text(left_axis)
                    TextLayout {
                        justify: Justify::End,
                        linebreak: LineBreak::WordOrCharacter,
                    }
                ],

                arrow_image_node(2),
            ],

            Node {
                flex_direction: FlexDirection::Row,
                width: percent(49),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: px(6),
            }
            Children [
                arrow_image_node(3),

                Node
                Children[
                    axis_text(right_axis),
                ],
            ],
        ]
    }
}

fn axis_text(axis: &'static str) -> impl Scene {
    bsn! {
        Text::new(axis)
        TextFont {
            font_size: FontSize::Rem(0.7),
        }
        pinpoint_font()
        TextLayout::linebreak(LineBreak::WordOrCharacter)
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

const AXIS_SPECTRA: [AxisSpectrum; 42] = [
    // Seasons
    AxisSpectrum("Springy", "Summery"),
    AxisSpectrum("Autumnal", "Wintry"),
    // Directions
    AxisSpectrum("Western", "Eastern"),
    AxisSpectrum("Northern", "Southern"),
    // East vs West coast cities
    AxisSpectrum("NYC", "BOS"),
    AxisSpectrum("SF", "LA"),
    // Knowledge
    AxisSpectrum("Obscure", "Well Known"),
    AxisSpectrum("Important", "Trivial"),
    // Moments
    AxisSpectrum("Rare", "Common"),
    AxisSpectrum("Unpleasant", "Enjoyable"),
    // Food
    AxisSpectrum("Breakfast", "Dinner"),
    AxisSpectrum("Diet", "Indulgent"),
    // Activities
    AxisSpectrum("Overhyped", "Secret"),
    AxisSpectrum("Fun", "Interesting"),
    // Attraction
    AxisSpectrum("Cliche", "Unique"),
    AxisSpectrum("Attractive", "Turn off"),
    // Flavor
    AxisSpectrum("Sweet", "Savory"),
    AxisSpectrum("Salty", "Sour"),
    // Dreams
    AxisSpectrum("Nightmare", "Pleasant Dream"),
    AxisSpectrum("Implausible", "Plausible"),
    // Worldly Phenomenon
    AxisSpectrum("Aquatic", "Terrestrial"),
    AxisSpectrum("Interactable", "Keep a Safe Distance"),
    // Occupations
    AxisSpectrum("Chore", "Job"),
    AxisSpectrum("Requires Special Tools", "Use Bare Hands"),
    // Desires
    AxisSpectrum("Want", "Need"),
    AxisSpectrum("For World", "For Self"),
    // Music
    AxisSpectrum("Study Mix", "Karaoke"),
    AxisSpectrum("Hidden Gem", "Popular"),
    // Food (pizza / sandwich)
    AxisSpectrum("Pizza Topping", "Sandwich Filling"),
    AxisSpectrum("Want More Of", "Remove"),
    // Technology related
    AxisSpectrum("Creates Problems", "Solves Problems"),
    AxisSpectrum("Man-made", "Natural Phenomenon"),
    // Spoken
    AxisSpectrum("Corny", "Cool"),
    AxisSpectrum("Said on a Date", "Said while Networking"),
    // Recipes
    AxisSpectrum("Quick", "Time Intensive"),
    AxisSpectrum("Potluck Dish", "Meal for One"),
    // Actions
    AxisSpectrum("Performative", "Genuine"),
    AxisSpectrum("Rewarded", "Despicable"),
    // Hobbies
    AxisSpectrum("Not Worth Doing", "Worth Doing"),
    AxisSpectrum("Hobby", "Hustle"),
    // Topics
    AxisSpectrum("Heavily Debated", "Everyone Agrees"),
    AxisSpectrum("Serious Convo", "Casual Convo"),
    // AxisSpectrum("Asian", "Asian American"),
    // AxisSpectrum("Appreciated by Westerners", "Subject to Xenophobia"),
    // Asian - Asian American
];

use bevy::{
    ecs::template::EntityTemplate,
    prelude::*,
    text::{EditableText, TextEdit},
    ui_widgets::{Activate, ActivateOnPress, Button},
};
use smol_str::{SmolStr, ToSmolStr};

use crate::ui::{MIDDLE_BLUE_COLOR, pinpoint_font};

#[derive(Component, Clone, Default)]
pub struct VirtualKeyboard;

/// A key on the virtual keyboard that would submit the following text edit into an [`bevy::text::EditableText`]
#[derive(Component, Clone, PartialEq)]
struct VirtualKeyboardKey(pub TextEdit, Option<Entity>);

// impl Default just to use in bsn!
impl Default for VirtualKeyboardKey {
    fn default() -> Self {
        Self(TextEdit::TextEnd(false), None)
    }
}

pub fn virtual_keyboard(editable_text_entity: EntityTemplate) -> impl Scene {
    bsn! {
        VirtualKeyboard
        Node {
            display: Display::None,
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            width: percent(95),
        }
        Children [
            virtual_keyboard_row("1234567890".chars().collect::<Vec<_>>(), editable_text_entity),
            virtual_keyboard_row("qwertyuiop".chars().collect::<Vec<_>>(), editable_text_entity),
            virtual_keyboard_row("asdfghjkl".chars().collect::<Vec<_>>(), editable_text_entity),
            virtual_keyboard_row("zxcvbnm".chars().collect::<Vec<_>>(), editable_text_entity)
            Children [
                virtual_keyboard_backspace(editable_text_entity)
            ],
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: px(3),
            }
            Children [
                virtual_keyboard_key(',', editable_text_entity),
                virtual_keyboard_spacebar(editable_text_entity),
                virtual_keyboard_key('.', editable_text_entity),
                virtual_keyboard_key('_', editable_text_entity),
            ]
        ]
    }
}

fn virtual_keyboard_row(chars: Vec<char>, editable_text_entity: EntityTemplate) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(3),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
            { chars.iter().map(|c| virtual_keyboard_key(*c, editable_text_entity)).collect::<Vec<_>>() }
        ]
    }
}

fn virtual_keyboard_key(c: char, editable_text_entity: EntityTemplate) -> impl Scene {
    bsn! {
        Node {
            padding: UiRect::axes(px(2), px(4)),
            border: px(1),
        }
        BorderColor::all(Color::BLACK)
        ActivateOnPress
        Button
        virtual_keyboard_key_inner(TextEdit::Insert(c.to_smolstr()), editable_text_entity)
        Children [
            Text::new(c)
            pinpoint_font()
            TextFont {
                font_size: px(20),
            }
        ]
        on(key_on_activate)
    }
}

fn virtual_keyboard_backspace(editable_text_entity: EntityTemplate) -> impl Scene {
    bsn! {
        Node {
            padding: UiRect::axes(px(2), px(4)),
            border: px(1),
        }
        BorderColor::all(Color::BLACK)
        ActivateOnPress
        Button
        virtual_keyboard_key_inner(TextEdit::Backspace, editable_text_entity)
        Children [
            ImageNode {
                image: "button/keyboard/backspace_icon.png"
            }
        ]
        on(key_on_activate)
    }
}

fn virtual_keyboard_spacebar(editable_text_entity: EntityTemplate) -> impl Scene {
    bsn! {
        Node {
            width: percent(50),
            height: px(26),
            border: px(1),
        }
        BorderColor::all(Color::BLACK)
        ActivateOnPress
        Button
        virtual_keyboard_key_inner(TextEdit::Insert(SmolStr::new(" ")), editable_text_entity)
        on(key_on_activate)
    }
}

fn key_on_activate(
    event: On<Activate>,
    key_q: Query<&VirtualKeyboardKey>,
    mut editable_text_q: Query<&mut EditableText>,
    mut commands: Commands,
) {
    let Ok(key) = key_q.get(event.entity) else {
        return;
    };
    let Some(editable_text_entity) = key.1 else {
        return;
    };
    let Ok(mut editable_text) = editable_text_q.get_mut(editable_text_entity) else {
        return;
    };
    editable_text.queue_edit(key.0.clone());

    commands
        .entity(event.entity)
        .insert(BackgroundColor(MIDDLE_BLUE_COLOR.with_alpha(0.5)));
    commands
        .delayed()
        .secs(0.1)
        .entity(event.entity)
        .remove::<BackgroundColor>();
}

fn virtual_keyboard_key_inner(edit: TextEdit, editable_text_entity: EntityTemplate) -> impl Scene {
    bsn! {
        template(move |ctx| match editable_text_entity {
            EntityTemplate::Entity(ent) => Ok(VirtualKeyboardKey(edit.clone(), Some(ent))),
            EntityTemplate::SceneEntityReference(scene_entity_reference) =>Ok(VirtualKeyboardKey(edit.clone(), Some(ctx.get_entity(scene_entity_reference)))),
            EntityTemplate::None => Err(BevyError::error("Did not set up app correctly!"))
        })
    }
}

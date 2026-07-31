use bevy::{prelude::*};

/// Marker component for the menu
#[derive(Component, Clone, Default)]
pub struct AppCreate;

// TODO we should hook an observer to change the layout depending on if
// height is larger or width is larger
pub fn setup_create(mut commands: Commands) {
    commands.spawn_scene(bsn! {
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

        ]
    });
}

fn setup_create_vertical() -> impl Scene {
    bsn! {
        
    }

}

pub fn teardown_create(mut commands: Commands, app_create_q: Single<Entity, With<AppCreate>>) {
    commands.entity(app_create_q.entity()).despawn();
}
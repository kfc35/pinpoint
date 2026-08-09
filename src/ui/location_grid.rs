use bevy::{prelude::*, ui_widgets::ValueChange};

#[derive(Component, Clone, Default)]
struct LocationGrid;

#[derive(Component, Clone, Default)]
struct Pin;

#[derive(Component, Clone, Default)]
pub struct AnswerPin;

const GRID_SIZE_PX: f32 = 280.;
const CROSSHAIR_SIZE_PX: f32 = 52.;
const ANSWER_PIN_SIZE_PX: f32 = 35.;

/// Marker Component for the movable pin on a location grid.
/// There should only ever be once of these in the whole app.
/// Contains whether this pin can move or is currently locked in place.
#[derive(Component, Clone, Default)]
pub struct MovablePin(pub bool);

/// The scene for the location grid widget.
pub fn location_grid(location: Option<UVec2>, is_movable: bool, use_crosshair: bool) -> impl Scene {
    let pin_node = if let Some(loc) = location {
        let mut node = Node {
            position_type: PositionType::Absolute,
            ..default()
        };

        if use_crosshair {
            update_pin_node_with_location_inner(&mut node, loc, GRID_SIZE_PX, CROSSHAIR_SIZE_PX);
        } else {
            let shift_percent = (1. / ANSWER_PIN_SIZE_PX) / GRID_SIZE_PX * 100.0;
            node.left = percent(loc.x as f32 - shift_percent);
            node.bottom = percent(loc.y as f32 - shift_percent);
        }

        node
    } else {
        Node {
            position_type: PositionType::Absolute,
            display: Display::None,
            ..default()
        }
    };
    let maybe_movable = || -> Box<dyn Scene> {
        if is_movable {
            Box::new(bsn! {MovablePin})
        } else {
            Box::new(bsn! {})
        }
    };
    let maybe_observers = || -> Box<dyn Scene> {
        if is_movable {
            Box::new(bsn! {pressable_location_grid_observers()})
        } else {
            Box::new(bsn! {})
        }
    };
    let image = || -> Box<dyn Scene> {
        if use_crosshair {
            Box::new(bsn! {
                Node {
                    width: px(CROSSHAIR_SIZE_PX),
                    height: px(CROSSHAIR_SIZE_PX),
                }
                ImageNode {
                    image: "game_area/crosshair.png"
                }
            })
        } else {
            Box::new(bsn! {
                Node {
                    width: px(ANSWER_PIN_SIZE_PX),
                    height: px(ANSWER_PIN_SIZE_PX),
                }
                ImageNode {
                    image: "game_area/answer_pin.png"
                }
            })
        }
    };
    bsn! {
        LocationGrid
        Node {
            margin: px(5),
        }
        // We use outline here instead of border so that presses ignore it.
        Outline::new(px(5), Val::ZERO, Color::WHITE)
        maybe_observers()
        Children [
            Node {
                width: px(GRID_SIZE_PX),
                height: px(GRID_SIZE_PX),
            }
            ImageNode {
                image: "game_area/grid.png"
            },

            template_value(pin_node)
            Pin
            maybe_movable()
            ZIndex(1)
            Children [
                image()
            ],
        ]
    }
}

pub fn place_answer_pin(location_grid_entity: Entity, location: UVec2, commands: &mut Commands) {
    let answer_pin = commands
        .spawn_scene(bsn! {
            Node {
                position_type: PositionType::Absolute,
                left: percent(location.x),
                bottom: percent(location.y),
            }
            AnswerPin
            Pin
            ZIndex(2)
            Children [
                Node {
                    width: px(ANSWER_PIN_SIZE_PX),
                    height: px(ANSWER_PIN_SIZE_PX),
                }
                ImageNode {
                    image: "game_area/answer_pin.png"
                }
            ]
        })
        .id();
    commands.entity(location_grid_entity).add_child(answer_pin);
}

/// Observers that move the location of the pin.
/// The observers emit a `ValueChange<UVec2>` with the value
/// of the new location.
/// Observers of the value change event should call `update_pin_location`
/// after syncing their state.
fn pressable_location_grid_observers() -> impl Scene {
    bsn! {
        on(|mut event: On<Pointer<Press>>,
            node_q: Query<
                (
                    &ComputedNode,
                    &ComputedUiRenderTargetInfo,
                    &UiGlobalTransform,
                ),
                With<LocationGrid>,
            >,
            ui_scale: Res<UiScale>,
            mut commands: Commands| {
                let Some(new_location) = get_new_location(event.entity, event.pointer_location.position, node_q, ui_scale) else {
                    return;
                };
                event.propagate(false);

                commands.trigger(ValueChange {
                    source: event.entity,
                    value: new_location,
                    is_final: true,
                });
        })
        on(|mut event: On<Pointer<Drag>>,
            node_q: Query<
                (
                    &ComputedNode,
                    &ComputedUiRenderTargetInfo,
                    &UiGlobalTransform,
                ),
                With<LocationGrid>,
            >,
            ui_scale: Res<UiScale>,
            mut commands: Commands,| {
                let Some(new_location) = get_new_location(event.entity, event.pointer_location.position, node_q, ui_scale) else {
                    return;
                };
                event.propagate(false);

                commands.trigger(ValueChange {
                    source: event.entity,
                    value: new_location,
                    is_final: false,
                });
        })
        on(|mut event: On<Pointer<DragEnd>>,
            node_q: Query<
                (
                    &ComputedNode,
                    &ComputedUiRenderTargetInfo,
                    &UiGlobalTransform,
                ),
                With<LocationGrid>,
            >,
            ui_scale: Res<UiScale>,
            mut commands: Commands,| {
                let Some(new_location) = get_new_location(event.entity, event.pointer_location.position, node_q, ui_scale) else {
                    return;
                };
                event.propagate(false);

                commands.trigger(ValueChange {
                    source: event.entity,
                    value: new_location,
                    is_final: true,
                });
        })
    }
}

/// Utility to fetch the new pin location after a click into the [`LocationGrid`]
fn get_new_location(
    event_entity: Entity,
    pointer_position: Vec2,
    node_q: Query<
        (
            &ComputedNode,
            &ComputedUiRenderTargetInfo,
            &UiGlobalTransform,
        ),
        With<LocationGrid>,
    >,
    ui_scale: Res<UiScale>,
) -> Option<UVec2> {
    let Ok((node, node_target, transform)) = node_q.get(event_entity) else {
        return None;
    };
    let Some(pos) = node.normalize_point(
        *transform,
        pointer_position * node_target.scale_factor() / ui_scale.0,
    ) else {
        return None;
    };

    let new_location =
        (pos * Vec2::new(1.0, -1.0) + Vec2::splat(0.5)).clamp(Vec2::ZERO, Vec2::ONE) * 100.;
    Some(new_location.round().as_uvec2())
}

/// Updates the [`MovablePin`]'s location given a new location in game logic coordinates (vals are 0..=100)
pub fn update_pin_location(
    new_location: UVec2,
    pin_q: Single<(&mut Node, &MovablePin)>,
    is_crosshair: bool,
) {
    let (mut node, movable) = pin_q.into_inner();
    if movable.0 {
        node.display = Display::default();
        if is_crosshair {
            update_crosshair_pin_node_with_location(&mut node, new_location);
        } else {
            let shift_percent = (1. / ANSWER_PIN_SIZE_PX) / GRID_SIZE_PX * 100.0;
            node.left = percent(new_location.x as f32 - shift_percent);
            node.bottom = percent(new_location.y as f32 - shift_percent);
        }
    }
}

/// Updates the [`Node`]'s left and bottom with the given the location
pub fn update_crosshair_pin_node_with_location(mut node: &mut Node, location: UVec2) {
    update_pin_node_with_location_inner(&mut node, location, GRID_SIZE_PX, CROSSHAIR_SIZE_PX);
}

/// Updates the [`Node`]'s left and bottom with the given the location
fn update_pin_node_with_location_inner(
    node: &mut Node,
    location: UVec2,
    grid_size: f32,
    crosshair_size: f32,
) {
    // We subtract 7.5 so that the pin center is exactly where
    // we want it to be.
    // 42 (size of crosshair) / 2 = 21.
    // the bullseye center is at 21 x 21, so we want the bottom
    // left of the crosshair below and to the left of where the
    // center should go by 21 / 280 = 7.5%
    let shift_percent = (crosshair_size / 2.) / grid_size * 100.0;
    node.left = percent(location.x as f32 - shift_percent);
    node.bottom = percent(location.y as f32 - shift_percent);
}

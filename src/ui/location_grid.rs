use bevy::prelude::*;

#[derive(Component, Clone, Default)]
struct LocationGrid;

#[derive(Component, Clone, Default)]
struct Pin;

const GRID_SIZE_PX: f32 = 280.;
const CROSSHAIR_SIZE_PX: f32 = 52.;

/// Marker component for a movable pin on a location grid.
/// There should only ever be once of these in the whole app.
#[derive(Component, Clone, Default)]
pub struct MovablePin;

pub fn location_grid(location: Option<UVec2>, is_movable: bool) -> impl Scene {
    let pin_node = if let Some(loc) = location {
        let mut node = Node {
            position_type: PositionType::Absolute,
            ..default()
        };
        update_pin_node_with_location(&mut node, loc, GRID_SIZE_PX, CROSSHAIR_SIZE_PX);
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
    bsn! {
        LocationGrid
        Node {
            border: px(5),
        }
        BorderColor::all(Color::WHITE)
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
                Node {
                    width: px(CROSSHAIR_SIZE_PX),
                    height: px(CROSSHAIR_SIZE_PX),
                }
                ImageNode {
                    image: "game_area/crosshair.png"
                }
            ],
        ]
    }
}

pub fn update_pin_location<T>(new_location: UVec2, pin_q: Single<&mut Node, With<MovablePin>>) {
    let mut node = pin_q.into_inner();
    node.display = Display::default();
    update_pin_node_with_location(&mut node, new_location, GRID_SIZE_PX, CROSSHAIR_SIZE_PX);
}

fn update_pin_node_with_location(
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

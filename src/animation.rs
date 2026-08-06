use bevy::{prelude::*, time::Timer};

/// Marker component for an animated image node containing the number of frames
#[derive(Component, Clone, Default, Deref)]
pub struct AnimatedImageNode(pub usize);

/// Component to used for image animations
#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// System to animate images tagged with an AnimatedImageNode
fn animate_images(
    time: Res<Time>,
    mut query: Query<
        (&AnimatedImageNode, &mut ImageNode, &mut AnimationTimer),
        With<AnimatedImageNode>,
    >,
) {
    for (length, mut image_node, mut timer) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut image_node.texture_atlas
        {
            atlas.index = if atlas.index + 1 == length.0 {
                0
            } else {
                atlas.index + 1
            };
        }
    }
}

pub struct AnimateGifPlugin;

impl Plugin for AnimateGifPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_images);
    }
}

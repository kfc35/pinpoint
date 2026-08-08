use bevy::{
    ecs::{error::Result, template::TemplateContext},
    prelude::*,
};

use crate::{animation::AnimatedImageNode, load::LoadableRounds, play::PlayRound};

const RESULT_BANNERS: [(&'static str, usize); 1] = [("images/results/bullseye.png", 9)];

/// Returns the result image after an imported round has been played.
/// Must only be called when in [`crate::AppState::Play`].
pub fn get_result_image(context: &mut TemplateContext) -> Result<ImageNode> {
    let (image_path, num_rows) = get_image(&context);
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, num_rows as u32, None, None);
    let layout_handle = context
        .resource_mut::<Assets<TextureAtlasLayout>>()
        .add(layout);
    let texture_atlas = TextureAtlas {
        layout: layout_handle,
        index: 0,
    };
    Ok(ImageNode {
        image: context.resource::<AssetServer>().load(image_path),
        texture_atlas: Some(texture_atlas),
        ..Default::default()
    })
}

/// Returns a result image's animation node after an imported round has been played.
/// Must only be called when in [`crate::AppState::Play`].
pub fn get_animated_image_node(context: &mut TemplateContext) -> Result<AnimatedImageNode> {
    let (_, num_rows) = get_image(&context);
    Ok(AnimatedImageNode(num_rows))
}

fn get_image(context: &TemplateContext) -> (&'static str, usize) {
    let play_round = context.resource::<PlayRound>();
    let app_type_registry: &AppTypeRegistry = context.resource::<AppTypeRegistry>();
    let loadable_round = context
        .resource::<LoadableRounds>()
        .get_round(play_round.get_index());
    let _distance = loadable_round.get_guess_distance(app_type_registry);

    // TODO this should be based on distance.
    RESULT_BANNERS[0]
}

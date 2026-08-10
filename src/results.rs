use bevy::{
    ecs::{error::Result, template::TemplateContext},
    prelude::*,
};
use rand::{RngExt, SeedableRng};

use crate::{animation::AnimatedImageNode, load::LoadableRounds, play::PlayRound};

const RESULT_BANNERS: [(&'static str, usize); 3] = [
    ("images/results/in_the_neighborhood.png", 8),
    ("images/results/ehh_close_enough.png", 8),
    ("images/results/where_am_i.png", 7),
];

const RESULT_BANNERS_A: [(&'static str, usize); 2] = [
    ("images/results/A/bullseye.png", 9),
    ("images/results/A/threaded_the_needle.png", 16),
];

const RESULT_BANNERS_B: [(&'static str, usize); 2] = [
    ("images/results/B/on_the_green.png", 11),
    ("images/results/B/lollipop_rich.png", 8),
];

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
    let distance = loadable_round.get_guess_distance(app_type_registry);

    let results_banner_seed =
        bytemuck::cast::<[f32; 4], [u8; 16]>([distance, distance, distance, distance]);
    let mut rng = rand_pcg::Pcg32::from_seed(results_banner_seed);

    if distance <= 3. {
        RESULT_BANNERS_A[rng.random_range(0..RESULT_BANNERS_A.len())]
    } else if distance <= 6.25 {
        RESULT_BANNERS_B[rng.random_range(0..RESULT_BANNERS_B.len())]
    } else if distance <= 12.5 {
        RESULT_BANNERS[0]
    } else if distance <= 25. {
        RESULT_BANNERS[1]
    } else {
        RESULT_BANNERS[2]
    }
}

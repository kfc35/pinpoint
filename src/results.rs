use bevy::{
    ecs::{error::Result, template::TemplateContext},
    prelude::*,
};
use rand::{RngExt, SeedableRng};

use crate::{animation::AnimatedImageNode, load::LoadableRounds, play::PlayRound};

const RESULT_BANNERS_A: [(&'static str, usize); 2] = [
    ("images/results/A/bullseye.png", 9),
    ("images/results/A/threaded_the_needle.png", 16),
];

const RESULT_BANNERS_B: [(&'static str, usize); 2] = [
    ("images/results/B/on_the_green.png", 11),
    ("images/results/B/lollipop_rich.png", 8),
];

const RESULT_BANNERS_C: [(&'static str, usize); 3] = [
    ("images/results/C/enjoy_a_banana.png", 21),
    ("images/results/C/in_the_neighborhood.png", 8),
    ("images/results/C/i_lift_my_pin.png", 5),
];

const RESULT_BANNERS_D: [(&'static str, usize); 3] = [
    ("images/results/D/ehh_close_enough.png", 8),
    ("images/results/D/at_least_you_placed.png", 13),
    ("images/results/D/orange_you_glad.png", 7),
];

const RESULT_BANNERS_F: [(&'static str, usize); 5] = [
    ("images/results/F/where_am_i.png", 7),
    ("images/results/F/aimless.png", 8),
    ("images/results/F/got_a_tomato.png", 14),
    ("images/results/F/so_cold.png", 4),
    ("images/results/F/no_mans_land.png", 14),
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
        RESULT_BANNERS_C[rng.random_range(0..RESULT_BANNERS_C.len())]
    } else if distance <= 25. {
        RESULT_BANNERS_D[rng.random_range(0..RESULT_BANNERS_D.len())]
    } else {
        RESULT_BANNERS_F[rng.random_range(0..RESULT_BANNERS_F.len())]
    }
}

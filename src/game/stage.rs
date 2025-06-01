use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<StageAssets>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct StageAssets {
    #[asset(path = "image/ui/stage.png")]
    bg: Handle<Image>,
}

impl Configure for StageAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

pub fn stage(stage_assets: &StageAssets) -> impl Bundle {
    (
        Name::new("Stage"),
        ImageNode::from(stage_assets.bg.clone()),
        Node {
            aspect_ratio: Some(356.0 / 58.0),
            align_self: AlignSelf::End,
            flex_grow: 1.0,
            ..Node::ROW_MID
        },
        BackgroundColor(tailwind::AMBER_200.into()),
        children![],
    )
}

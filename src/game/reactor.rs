use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ReactorAssets>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ReactorAssets {
    #[asset(path = "image/ui/reactor.png")]
    bg: Handle<Image>,
}

impl Configure for ReactorAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

pub fn reactor(reactor_assets: &ReactorAssets) -> impl Bundle {
    (
        Name::new("Reactor"),
        ImageNode::from(reactor_assets.bg.clone()),
        Node {
            aspect_ratio: Some(124.0 / 270.0),
            padding: UiRect::all(Vw(2.0)),
            row_gap: Vw(2.0),
            ..Node::COLUMN_MID.full_height()
        },
        children![],
    )
}

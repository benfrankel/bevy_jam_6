use super::module::Module;
use super::module::ModuleStatus;
use super::module::module;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<StageAssets>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct StageAssets {
    #[asset(path = "image/ui/stage.png")]
    bg: Handle<Image>,
    #[asset(path = "image/module/face_down.png")]
    draw_pile_image: Handle<Image>,
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
            ..Node::ROW.full_width()
        },
        children![
            hand(),
            (
                Name::new("Row"),
                Node {
                    padding: UiRect::all(Vw(1.69)),
                    ..Node::ROW_MID.reverse().full_width()
                },
                children![draw_pile(stage_assets)],
            ),
        ],
    )
}

fn hand() -> impl Bundle {
    let face_up = Module {
        status: ModuleStatus::FaceUp,
        ..default()
    };

    (
        Name::new("Hand"),
        Node {
            column_gap: Vw(1.22),
            ..Node::ROW_CENTER.full_size().abs()
        },
        children![
            module(face_up, Anchor::TopCenter),
            module(face_up, Anchor::TopCenter),
            module(face_up, Anchor::TopCenter),
            module(face_up, Anchor::TopCenter),
            module(face_up, Anchor::TopCenter),
        ],
    )
}

fn draw_pile(stage_assets: &StageAssets) -> impl Bundle {
    (
        Name::new("DrawPile"),
        ImageNode::from(stage_assets.draw_pile_image.clone()),
        Node {
            width: Vw(6.66),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        Tooltip::fixed(Anchor::TopCenter, parse_rich("[b]Draw pile")),
        children![widget::small_colored_label("15", ThemeColor::IconText)],
    )
}

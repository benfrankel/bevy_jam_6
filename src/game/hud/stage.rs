use crate::game::hud::HudAssets;
use crate::game::hud::module::Module;
use crate::game::hud::module::ModuleStatus;
use crate::game::hud::module::module;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let _ = app;
}

pub fn stage(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("Stage"),
        ImageNode::from(hud_assets.stage.clone()),
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
                children![draw_pile(hud_assets)],
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

fn draw_pile(hud_assets: &HudAssets) -> impl Bundle {
    (
        Name::new("DrawPile"),
        ImageNode::from(hud_assets.module_face_down.clone()),
        Node {
            width: Vw(6.66),
            aspect_ratio: Some(1.0),
            ..Node::ROW_CENTER
        },
        Tooltip::fixed(Anchor::TopCenter, parse_rich("[b]Draw pile")),
        children![widget::small_colored_label("15", ThemeColor::IconText)],
    )
}

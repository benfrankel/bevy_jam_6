use crate::combat::death::DieOnClick;
use crate::combat::death::FadeOutOnDeath;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Toast>();
}

pub fn toast(text: impl AsRef<str>) -> impl Bundle {
    (
        Name::new("Toast"),
        Toast,
        Node {
            left: Vw(45.2),
            bottom: Vw(27.5),
            padding: UiRect::all(Vw(1.0)),
            border: UiRect::all(Px(1.0)),
            ..Node::COLUMN.width(Vw(35.0)).abs()
        },
        ThemeColor::Popup.set::<BackgroundColor>(),
        BorderRadius::all(Vw(1.0)),
        ThemeColor::BorderColor.set::<BorderColor>(),
        BoxShadow::from(ShadowStyle {
            color: Color::BLACK.with_alpha(0.5),
            x_offset: Val::ZERO,
            y_offset: Val::ZERO,
            spread_radius: Val::ZERO,
            blur_radius: Val::Vw(4.0),
        }),
        FocusPolicy::Block,
        DieOnClick,
        FadeOutOnDeath { duration: 0.15 },
        children![widget::paragraph(text)],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Toast;

impl Configure for Toast {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(despawn_older_toasts);
    }
}

fn despawn_older_toasts(
    trigger: Trigger<OnAdd, Toast>,
    mut commands: Commands,
    toast_query: Query<Entity, With<Toast>>,
) {
    let target = rq!(trigger.get_target());
    for entity in &toast_query {
        cq!(entity != target);
        commands.entity(entity).despawn();
    }
}

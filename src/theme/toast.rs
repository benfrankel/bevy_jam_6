use crate::combat::death::DieOnClick;
use crate::combat::death::FadeOutOnDeath;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Toaster, Toast)>();
}

/// A marker component for the entity that will contain [`Toast`] entities.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Toaster;

impl Configure for Toaster {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn toast(text: impl AsRef<str>) -> impl Bundle {
    (
        Name::new("Toast"),
        Toast,
        Node {
            padding: UiRect::all(Vw(1.0)),
            border: UiRect::all(Px(1.0)),
            ..default()
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
        children![widget::label_base(
            Vw(1.8),
            ThemeColor::BodyText,
            JustifyText::Center,
            1.5,
            text,
        )],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Toast;

impl Configure for Toast {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(prepare_toast);
    }
}

fn prepare_toast(
    trigger: Trigger<OnAdd, Toast>,
    mut commands: Commands,
    toast_query: Query<Entity, With<Toast>>,
    toaster_query: Query<Entity, With<Toaster>>,
) {
    let target = rq!(trigger.get_target());
    if let Ok(toaster) = toaster_query.single() {
        commands.entity(target).insert(ChildOf(toaster));
    }
    for entity in &toast_query {
        cq!(entity != target);
        commands.entity(entity).despawn();
    }
}

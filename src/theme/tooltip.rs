use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let container = app
        .world_mut()
        .spawn((
            Name::new("PrimaryTooltip"),
            Node {
                padding: UiRect::all(Vw(1.0)),
                //border: UiRect::all(Px(1.0)),
                ..Node::DEFAULT.abs()
            },
            //BackgroundColor(Color::srgba(0.106, 0.118, 0.122, 0.5)),
            ThemeColor::Popup.set::<BackgroundColor>(),
            //BorderRadius::all(Vw(3.0)),
            //ThemeColor::BorderColor.set::<BorderColor>(),
            BoxShadow(vec![ShadowStyle {
                color: Color::BLACK.with_alpha(0.5),
                x_offset: Val::ZERO,
                y_offset: Val::ZERO,
                spread_radius: Vw(0.5),
                blur_radius: Vw(0.5),
            }]),
            Visibility::Hidden,
            GlobalZIndex(999),
            Pickable::IGNORE,
        ))
        .id();

    let text = app
        .world_mut()
        .spawn((
            Name::new("Text"),
            Node::default(),
            RichText::default(),
            DynamicFontSize::new(Vw(2.0)).with_step(8.0),
            Pickable::IGNORE,
            ChildOf(container),
        ))
        .id();

    app.add_plugins(TooltipPlugin {
        container,
        text,
        ..default()
    });
}

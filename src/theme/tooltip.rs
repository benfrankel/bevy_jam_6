use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let container = app
        .world_mut()
        .spawn((
            Name::new("PrimaryTooltip"),
            Node {
                position_type: PositionType::Absolute,
                padding: UiRect::all(Vw(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.106, 0.118, 0.122, 0.9)),
            Visibility::Hidden,
            GlobalZIndex(999),
        ))
        .id();

    let text = app
        .world_mut()
        .spawn((
            Name::new("Text"),
            Node::default(),
            RichText::default(),
            DynamicFontSize::new(Vw(2.0)).with_step(8.0),
            ChildOf(container),
        ))
        .id();

    app.add_plugins(TooltipPlugin { container, text });
}

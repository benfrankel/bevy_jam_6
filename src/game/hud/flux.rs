use crate::animation::offset::NodeOffset;
use crate::animation::shake::NodeShake;
use crate::game::deck::Deck;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsFluxLabel>();
}

pub fn flux_display() -> impl Bundle {
    (
        Name::new("FluxDisplay"),
        Node {
            height: Vw(5.0),
            ..Node::ROW_CENTER.full_width()
        },
        Tooltip::fixed(
            Anchor::CenterRight,
            parse_rich(
                "[b]Flux multiplier[r]\n\nChain \"reactor modules\" together to multiply their output.",
            ),
        ),
        children![(
            widget::colored_label("", ThemeColor::MonitorText),
            IsFluxLabel,
            NodeOffset::new(Px(0.), Px(0.)),
            NodeShake::new(vec2(0., 0.), 0.),
        )],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsFluxLabel;

impl Configure for IsFluxLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, sync_flux_label.in_set(UpdateSystems::SyncLate));
    }
}

fn sync_flux_label(
    deck: Res<Deck>,
    mut label_query: Query<(&mut RichText, &mut NodeShake), With<IsFluxLabel>>,
) {
    for (mut text, mut shake) in &mut label_query {
        let new_rich = RichText::from_sections(parse_rich(format!("flux {}x", deck.flux)));
        let new = &new_rich.sections[0].value;
        if text.sections.len() > 0 && ! text.sections[0].value.eq(new) {
            *shake = NodeShake::new(vec2(100., 15.), 1.1);
        }
        *text = new_rich;
    }
}

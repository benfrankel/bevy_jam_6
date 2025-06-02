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
                "[b]Flux counter[r]\n\nChain \"reactor modules\" together to multiply their output.",
            ),
        ),
        children![(
            widget::colored_label("", ThemeColor::MonitorText),
            IsFluxLabel,
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

fn sync_flux_label(deck: Res<Deck>, mut label_query: Query<&mut RichText, With<IsFluxLabel>>) {
    for mut text in &mut label_query {
        *text = RichText::from_sections(parse_rich(format!("flux {}x", deck.flux)));
    }
}

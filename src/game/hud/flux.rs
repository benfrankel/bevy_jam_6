use crate::animation::shake::NodeShake;
use crate::game::deck::Deck;
use crate::game::hud::HudConfig;
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
            NodeShake::default(),
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
    hud_config: ConfigRef<HudConfig>,
    deck: Res<Deck>,
    mut label_query: Query<(&mut RichText, &mut NodeShake), With<IsFluxLabel>>,
) {
    let hud_config = r!(hud_config.get());
    for (mut text, mut shake) in &mut label_query {
        let new_text = RichText::from_sections(parse_rich(format!("flux {}x", deck.flux)));
        if text.sections.len() > 0 && text.sections[0].value != new_text.sections[0].value {
            shake.magnitude += hud_config.flux_shake_magnitude;
            shake.decay = hud_config.flux_shake_decay;
        }
        *text = new_text;
    }
}

use crate::animation::shake::NodeShake;
use crate::game::deck::PlayerDeck;
use crate::game::hud::HudConfig;
use crate::game::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsFluxLabel>();
}

pub fn flux_display() -> impl Bundle {
    (
        Name::new("FluxDisplay"),
        Node {
            width: Vw(22.5),
            height: Vw(5.0),
            border: UiRect::all(Vw(0.2083)),
            ..Node::ROW_CENTER
        },
        ThemeColor::Monitor.set::<BackgroundColor>(),
        ThemeColor::MonitorDimText.set::<BorderColor>(),
        Tooltip::fixed(
            Anchor::CenterRight,
            parse_rich(
                "[b]Flux multiplier[r]\n\nChain \"reactor modules\" together to multiply their output.",
            ),
        ),
        children![(
            IsFluxLabel,
            widget::colored_label("", default()),
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
        app.add_systems(
            Update,
            sync_flux_label
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<IsFluxLabel>>)),
        );
        app.add_systems(StateFlush, Phase::ANY.on_enter(sync_flux_display_to_phase));
    }
}

fn sync_flux_display_to_phase(
    phase: NextRef<Phase>,
    mut label_query: Query<&ChildOf, With<IsFluxLabel>>,
    mut border_query: Query<&mut ThemeColorFor<BorderColor>>,
) {
    for child_of in &mut label_query {
        let mut border_color = cq!(border_query.get_mut(child_of.parent()));
        border_color.0 =
            if phase.will_be_in(&state!(Phase::PowerUp | Phase::Player | Phase::PowerDown)) {
                ThemeColor::MonitorText
            } else {
                ThemeColor::MonitorDimText
            };
    }
}

fn sync_flux_label(
    hud_config: ConfigRef<HudConfig>,
    player_deck: Res<PlayerDeck>,
    mut label_query: Query<
        (&mut RichText, &mut ThemeColorForText, &mut NodeShake),
        With<IsFluxLabel>,
    >,
) {
    let hud_config = r!(hud_config.get());
    for (mut text, mut text_color, mut shake) in &mut label_query {
        text_color.0 = if player_deck.flux > f32::EPSILON {
            vec![ThemeColor::MonitorText]
        } else {
            vec![ThemeColor::MonitorDimText]
        };

        let new_text = RichText::from_sections(parse_rich(format!("flux {}x", player_deck.flux)));
        if !text.sections.is_empty() && text.sections[0].value != new_text.sections[0].value {
            shake.magnitude += hud_config.flux_shake_magnitude * player_deck.flux.max(3.0);
            shake.decay = hud_config.flux_shake_decay;
        }
        *text = new_text;
    }
}

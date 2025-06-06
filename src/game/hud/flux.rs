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
            height: Vw(4.5833),
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
            Name::new("Label"),
            IsFluxLabel,
            RichText::from_sections(parse_rich("a[b]c")).with_justify(JustifyText::Center),
            DynamicFontSize::new(Vw(3.5)).with_step(8.0),
            ThemeColorForText(vec![default(), default()]),
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
    mut label_query: Query<(&ChildOf, &mut ThemeColorForText), With<IsFluxLabel>>,
    mut border_query: Query<&mut ThemeColorFor<BorderColor>>,
) {
    for (child_of, mut text_color) in &mut label_query {
        let mut border_color = cq!(border_query.get_mut(child_of.parent()));
        let color = if phase.will_be_in(&Phase::Reactor) {
            ThemeColor::MonitorText
        } else {
            ThemeColor::MonitorDimText
        };
        text_color.0[0] = color;
        border_color.0 = color;
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
        text_color.0[1] = if player_deck.flux > f32::EPSILON {
            ThemeColor::MonitorText
        } else {
            ThemeColor::MonitorDimText
        };

        let new_text = RichText::from_sections(
            parse_rich("flux ")
                .into_iter()
                .chain(parse_rich(format!("{}x", player_deck.flux))),
        );
        if !text.sections.is_empty() && text.sections[1].value != new_text.sections[1].value {
            shake.magnitude += hud_config.flux_shake_magnitude * player_deck.flux.max(3.0);
            shake.decay = hud_config.flux_shake_decay;
        }
        *text = new_text;
    }
}

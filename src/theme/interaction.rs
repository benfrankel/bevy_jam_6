use bevy::ecs::component::Mutable;
use bevy::reflect::GetTypeRegistration;
use bevy::reflect::Typed;

use crate::animation::offset::NodeOffset;
use crate::core::audio::AudioSettings;
use crate::core::audio::ui_audio;
use crate::prelude::*;
use crate::theme::ThemeAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        Previous<Interaction>,
        InteractionDisabled,
        InteractionTheme<ThemeColorFor<BackgroundColor>>,
        InteractionTheme<NodeOffset>,
        TargetInteractionTheme<ThemeColorForText>,
        TargetInteractionTheme<NodeOffset>,
        ParentInteractionTheme<ThemeColorForText>,
        ParentInteractionTheme<NodeOffset>,
        InteractionSfx,
        InteractionGlassSfx,
    )>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InteractionDisabled(pub bool);

impl Configure for InteractionDisabled {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

/// A table of values to set a component to by interaction state.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Interaction, Previous<Interaction>)]
pub struct InteractionTheme<C: Component<Mutability = Mutable> + Clone> {
    pub none: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for InteractionTheme<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_interaction_theme::<C>.in_set(UpdateSystems::RecordInput),
        );
    }
}

fn apply_interaction_theme<C: Component<Mutability = Mutable> + Clone>(
    mut interaction_query: Query<
        (
            Option<&InteractionDisabled>,
            &Previous<Interaction>,
            &Interaction,
            &InteractionTheme<C>,
            &mut C,
        ),
        Or<(
            Changed<InteractionDisabled>,
            Changed<Previous<Interaction>>,
            Changed<Interaction>,
        )>,
    >,
) {
    for (is_disabled, previous, current, table, mut value) in &mut interaction_query {
        // Add 1 frame of delay when going from pressed -> hovered.
        cq!(!matches!(
            (previous.0, current),
            (Interaction::Pressed, Interaction::Hovered),
        ));

        // Clone the field corresponding to the current interaction state.
        *value = if matches!(is_disabled, Some(InteractionDisabled(true))) {
            &table.disabled
        } else {
            match current {
                Interaction::None => &table.none,
                Interaction::Hovered => &table.hovered,
                Interaction::Pressed => &table.pressed,
            }
        }
        .clone();
    }
}

/// Values to set a component to by a target entity's interaction state.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TargetInteractionTheme<C: Component<Mutability = Mutable> + Clone> {
    pub target: Entity,
    pub none: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for TargetInteractionTheme<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_target_interaction_theme::<C>.in_set(UpdateSystems::RecordInput),
        );
    }
}

fn apply_target_interaction_theme<C: Component<Mutability = Mutable> + Clone>(
    mut table_query: Query<(&TargetInteractionTheme<C>, &mut C)>,
    interaction_query: Query<
        (
            Option<&InteractionDisabled>,
            &Previous<Interaction>,
            &Interaction,
        ),
        Or<(
            Changed<InteractionDisabled>,
            Changed<Previous<Interaction>>,
            Changed<Interaction>,
        )>,
    >,
) {
    for (table, mut value) in &mut table_query {
        let (is_disabled, previous, current) = cq!(interaction_query.get(table.target));
        // Add 1 frame of delay when going from pressed -> hovered.
        cq!(!matches!(
            (previous.0, current),
            (Interaction::Pressed, Interaction::Hovered),
        ));

        // Clone the field corresponding to the current interaction state.
        *value = if matches!(is_disabled, Some(InteractionDisabled(true))) {
            &table.disabled
        } else {
            match current {
                Interaction::None => &table.none,
                Interaction::Hovered => &table.hovered,
                Interaction::Pressed => &table.pressed,
            }
        }
        .clone();
    }
}

/// Values to set a component to by a target entity's interaction state.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ParentInteractionTheme<C: Component<Mutability = Mutable> + Clone> {
    pub none: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for ParentInteractionTheme<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_parent_interaction_theme::<C>.in_set(UpdateSystems::RecordInput),
        );
    }
}

fn apply_parent_interaction_theme<C: Component<Mutability = Mutable> + Clone>(
    mut table_query: Query<(&ParentInteractionTheme<C>, &ChildOf, &mut C)>,
    interaction_query: Query<
        (
            Option<&InteractionDisabled>,
            &Previous<Interaction>,
            &Interaction,
        ),
        Or<(
            Changed<InteractionDisabled>,
            Changed<Previous<Interaction>>,
            Changed<Interaction>,
        )>,
    >,
) {
    for (table, child_of, mut value) in &mut table_query {
        let (is_disabled, previous, current) = cq!(interaction_query.get(child_of.parent()));
        // Add 1 frame of delay when going from pressed -> hovered.
        cq!(!matches!(
            (previous.0, current),
            (Interaction::Pressed, Interaction::Hovered),
        ));

        // Clone the field corresponding to the current interaction state.
        *value = if matches!(is_disabled, Some(InteractionDisabled(true))) {
            &table.disabled
        } else {
            match current {
                Interaction::None => &table.none,
                Interaction::Hovered => &table.hovered,
                Interaction::Pressed => &table.pressed,
            }
        }
        .clone();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InteractionSfx;

impl Configure for InteractionSfx {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(play_hover_sfx);
        app.add_observer(play_click_sfx);
    }
}

fn play_hover_sfx(
    trigger: Trigger<Pointer<Over>>,
    audio_settings: Res<AudioSettings>,
    theme_assets: Res<ThemeAssets>,
    sfx_query: Query<Option<&InteractionDisabled>, With<InteractionSfx>>,
    mut commands: Commands,
) {
    let target = r!(trigger.get_target());
    let disabled = rq!(sfx_query.get(target));
    rq!(!matches!(disabled, Some(InteractionDisabled(true))));

    commands.spawn(ui_audio(
        &audio_settings,
        theme_assets.sfx_hover.clone(),
        thread_rng().gen_range(0.9..1.5),
    ));
}

fn play_click_sfx(
    trigger: Trigger<Pointer<Click>>,
    audio_settings: Res<AudioSettings>,
    theme_assets: Res<ThemeAssets>,
    sfx_query: Query<Option<&InteractionDisabled>, With<InteractionSfx>>,
    mut commands: Commands,
) {
    let target = r!(trigger.get_target());
    let disabled = rq!(sfx_query.get(target));
    rq!(!matches!(disabled, Some(InteractionDisabled(true))));

    commands.spawn(ui_audio(
        &audio_settings,
        theme_assets.sfx_click.clone(),
        thread_rng().gen_range(0.9..1.5),
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InteractionGlassSfx;

impl Configure for InteractionGlassSfx {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(play_hover_glass_sfx);
        app.add_observer(play_click_glass_sfx);
    }
}

fn play_hover_glass_sfx(
    trigger: Trigger<Pointer<Over>>,
    audio_settings: Res<AudioSettings>,
    theme_assets: Res<ThemeAssets>,
    sfx_query: Query<Option<&InteractionDisabled>, With<InteractionGlassSfx>>,
    mut commands: Commands,
) {
    let target = r!(trigger.get_target());
    let disabled = rq!(sfx_query.get(target));
    rq!(!matches!(disabled, Some(InteractionDisabled(true))));

    commands.spawn(ui_audio(
        &audio_settings,
        theme_assets.sfx_hover_glass.clone(),
        1.0,
    ));
}

fn play_click_glass_sfx(
    trigger: Trigger<Pointer<Click>>,
    audio_settings: Res<AudioSettings>,
    theme_assets: Res<ThemeAssets>,
    sfx_query: Query<Option<&InteractionDisabled>, With<InteractionGlassSfx>>,
    mut commands: Commands,
) {
    let target = r!(trigger.get_target());
    let disabled = rq!(sfx_query.get(target));
    rq!(!matches!(disabled, Some(InteractionDisabled(true))));

    commands.spawn(ui_audio(
        &audio_settings,
        theme_assets.sfx_click_glass.clone(),
        1.0,
    ));
}

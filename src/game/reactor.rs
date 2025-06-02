use crate::game::module::Module;
use crate::game::module::module;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ReactorAssets, Flux, IsFluxLabel)>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ReactorAssets {
    #[asset(path = "image/ui/reactor.png")]
    bg: Handle<Image>,
}

impl Configure for ReactorAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

pub fn reactor(reactor_assets: &ReactorAssets) -> impl Bundle {
    (
        Name::new("Reactor"),
        ImageNode::from(reactor_assets.bg.clone()),
        Node {
            aspect_ratio: Some(124.0 / 270.0),
            padding: UiRect::all(Vw(1.69)),
            row_gap: Vw(1.69),
            ..Node::COLUMN_MID.full_height()
        },
        children![flux_display(), module_grid()],
    )
}

fn flux_display() -> impl Bundle {
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

fn module_grid() -> impl Bundle {
    (
        Name::new("ModuleGrid"),
        Node {
            display: Display::Grid,
            row_gap: Vw(1.25),
            column_gap: Vw(1.25),
            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
            ..Node::default().full_width()
        },
        IsModuleGrid,
        children![
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
            module(Module::EMPTY, Anchor::CenterRight),
        ],
    )
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Flux(pub f32);

impl Configure for Flux {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
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

fn sync_flux_label(flux: Res<Flux>, mut label_query: Query<&mut RichText, With<IsFluxLabel>>) {
    for mut text in &mut label_query {
        *text = RichText::from_sections(parse_rich(format!("flux {}x", flux.0)));
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsModuleGrid;

impl Configure for IsModuleGrid {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, sync_module_grid.in_set(UpdateSystems::SyncLate));
    }
}

fn sync_module_grid(grid_query: Query<Entity, With<IsModuleGrid>>) {
    for entity in &grid_query {
        let _ = entity;
    }
}

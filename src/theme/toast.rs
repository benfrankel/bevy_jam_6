use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ToastBox>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ToastBox;

impl Configure for ToastBox {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Startup, spawn_toast_box);
    }
}

fn spawn_toast_box(mut commands: Commands) {
    commands.spawn((
        Name::new("ToastBox"),
        ToastBox,
        Node {
            left: Vw(1.5),
            bottom: Vw(1.5),
            row_gap: Vw(0.5),
            ..Node::COLUMN.abs()
        },
        GlobalZIndex(1),
        Pickable::IGNORE,
    ));
}

use crate::game::GameLayer;
use crate::game::combat::damage::Damage;
use crate::game::combat::faction::Faction;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(MissileAssets, IsMissile)>();
}

pub fn missile(missile_assets: &MissileAssets, faction: Faction, damage: f32) -> impl Bundle {
    (
        Name::new("Missile"),
        IsMissile,
        Sprite::from_image(missile_assets.image.clone()),
        Damage(damage),
        faction,
        RigidBody::Dynamic,
        // TODO: Increase initial velocity, taking initial rotation into account.
        LinearVelocity::ZERO,
        MaxLinearSpeed(150.0),
        Collider::capsule(3.0, 7.0),
        CollisionLayers::new(GameLayer::Default, faction.opponent().layer()),
        CollisionEventsEnabled,
    )
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct MissileAssets {
    #[asset(path = "image/ship/missile.png")]
    image: Handle<Image>,
}

impl Configure for MissileAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsMissile;

impl Configure for IsMissile {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_missile_thrusters.in_set(UpdateSystems::Update),
        );
    }
}

fn apply_missile_thrusters(
    mut missile_query: Query<(&mut ExternalForce, &GlobalTransform), With<IsMissile>>,
) {
    const THRUSTER_FORCE: f32 = 100.0;
    for (mut force, gt) in &mut missile_query {
        force.apply_force(THRUSTER_FORCE * (gt.rotation() * Vec3::Y).truncate());
    }
}

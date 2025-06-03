use crate::game::GameLayer;
use crate::game::health::Health;
use crate::game::level::Level;
use crate::game::ship::IsPlayerShip;
use crate::game::ship::IsWeapon;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(MissileAssets, IsMissile)>();
}

pub fn missile(missile_assets: &MissileAssets, damage: f32) -> impl Bundle {
    (
        Name::new("Missile"),
        IsMissile,
        Sprite::from_image(missile_assets.image.clone()),
        RigidBody::Dynamic,
        // TODO: Increase initial velocity, taking initial rotation into account.
        LinearVelocity::ZERO,
        MaxLinearSpeed(150.0),
        Collider::capsule(3.0, 7.0),
        CollisionEventsEnabled,
        Patch(|entity| {
            entity.observe(hit_ship);
        }),
        Damage::new(damage),
    )
}

fn hit_ship(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    missile_assets: Res<MissileAssets>,
    missile_query: Query<(&IsMissile, &Damage)>,
    player_ship_children: Single<&Children, With<IsPlayerShip>>,
    weapon_query: Query<&GlobalTransform, With<IsWeapon>>,
    mut health_query: Query<&mut Health>,
) {
    // Apply damage to target ship
    let mut target_health = r!(health_query.get_mut(trigger.collider));
    let target = r!(trigger.get_target());
    target_health.current -= r!(missile_query.get(target)).1.damage;

    // Despawn the missile.
    commands.entity(target).despawn();

    if target_health.current <= 0. {
        commands.entity(trigger.collider).despawn();
        return;
    }

    // Fire a new missile from the player ship.
    let weapons = player_ship_children
        .iter()
        .filter_map(|entity| weapon_query.get(entity).ok())
        .collect::<Vec<_>>();
    let weapon = weapons.choose(&mut thread_rng()).unwrap();
    commands.spawn((
        missile(&missile_assets, thread_rng().gen_range(0.0..15.)),
        CollisionLayers::new(LayerMask::ALL, GameLayer::Enemy),
        weapon.compute_transform(),
        DespawnOnExitState::<Level>::default(),
    ));
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

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Damage {
    damage: f32,
}

impl Configure for Damage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl Damage {
    fn new(damage: f32) -> Self {
        Damage { damage }
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

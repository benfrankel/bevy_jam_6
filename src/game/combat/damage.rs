use crate::game::GameAssets;
use crate::game::combat::health::Health;
use crate::game::ship::IsEnemyShip;
use crate::game::ship::IsPlayerShip;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<DamageConfig>, Damage, OnDamage, IsDamagePopup)>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Damage(pub f32);

impl Configure for Damage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct DamageConfig {
    damage_popup_offset: Vec2,
    damage_popup_offset_spread: Vec2,
    damage_popup_velocity: Vec2,
    damage_popup_fade_rate: f32,
}

impl Config for DamageConfig {
    const FILE: &'static str = "damage.ron";
}

#[derive(Event, Reflect, Debug)]
pub struct OnDamage(pub f32);

impl Configure for OnDamage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(deal_damage_on_collision);
        app.add_observer(reduce_health_on_damage);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsDamagePopup;

impl Configure for IsDamagePopup {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(display_damage_indicator);
        app.add_systems(
            Update,
            apply_fade_out_to_damage_popup
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn deal_damage_on_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    damage_query: Query<&Damage>,
    health_query: Query<(), With<Health>>,
) {
    let hitbox = r!(trigger.get_target());
    let damage = rq!(damage_query.get(hitbox));

    let hurtbox = trigger.body.unwrap_or(trigger.collider);
    rq!(health_query.contains(hurtbox));

    commands.entity(hitbox).try_despawn();
    commands.entity(hurtbox).trigger(OnDamage(damage.0));
}

fn reduce_health_on_damage(trigger: Trigger<OnDamage>, mut health_query: Query<&mut Health>) {
    let target = r!(trigger.get_target());
    r!(health_query.get_mut(target)).current -= trigger.0;
}

fn display_damage_indicator(
    trigger: Trigger<OnDamage>,
    mut commands: Commands,
    assets: Res<Assets<Image>>,
    damage_config: ConfigRef<DamageConfig>,
    player_ship: Single<(Entity, &Transform), With<IsPlayerShip>>,
    enemy_ship: Single<(Entity, &Transform), With<IsEnemyShip>>,
    sprite_query: Query<&Sprite>
) {
    let rng = &mut thread_rng();
    let entity = r!(trigger.get_target());
    let damage_config = r!(damage_config.get());
    let (player, player_transform) = *player_ship;
    let (enemy, enemy_transform) = *enemy_ship;
    let (sprite, mut transform) = if entity == player {
        (r!(sprite_query.get(player)), *player_transform)
    } else if entity == enemy {
        (r!(sprite_query.get(enemy)), *enemy_transform)
    } else {
        return;
    };
    let sprite_size = r!(assets.get(&sprite.image)).size_f32();
    let point = Rectangle::from_size(sprite_size).sample_interior(rng);
    transform.translation.x += point.x;
    transform.translation.y += point.y;

    commands.spawn((
        IsDamagePopup,
        Text2d::new(trigger.0.to_string()),
        TextFont::from_font_size(16.),
        TextColor::from(Color::srgba(0.7, 0.2, 0.2, 1.)),
        transform,
        RigidBody::Kinematic,
        LinearVelocity(damage_config.damage_popup_velocity),
    ));
}

fn apply_fade_out_to_damage_popup(
    mut commands: Commands,
    time: Res<Time>,
    damage_config: ConfigRef<DamageConfig>,
    damage_popup_query: Query<(Entity, &mut TextColor), With<IsDamagePopup>>,
) {
    let damage_config = r!(damage_config.get());
    for (entity, mut text_color) in damage_popup_query {
        let alpha = text_color.0.alpha() - damage_config.damage_popup_fade_rate * time.delta_secs();
        text_color.0 = text_color.0.with_alpha(alpha);

        if text_color.0.alpha() < f32::EPSILON {
            commands.entity(entity).try_despawn();
        }
    }
}

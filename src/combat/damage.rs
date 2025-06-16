use crate::combat::death::Dead;
use crate::combat::faction::Faction;
use crate::combat::health::Health;
use crate::core::audio::AudioSettings;
use crate::core::audio::sfx_audio;
use crate::level::Level;
use crate::prelude::*;
use crate::screen::gameplay::GameplayAssets;
use crate::ship::EnemyShip;
use crate::ship::PlayerShip;
use crate::ship::PlayerShipBody;
use crate::stats::Stats;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<DamageConfig>, Damage, OnDamage, DamagePopup)>();
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
pub struct DamageConfig {
    damage_popup_font_size: f32,
    damage_popup_font_color: Color,
    damage_popup_offset: Vec2,
    damage_popup_offset_spread: Vec2,
    damage_popup_velocity: Vec2,
    damage_popup_fade_rate: f32,
    damage_popup_scale: f32,
    damage_popup_scale_factor: f32,
    damage_popup_scale_max: f32,
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
        app.add_observer(play_ship_hurt_sfx_on_damage);
    }
}

fn deal_damage_on_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    damage_query: Query<&Damage>,
    health_query: Query<(), With<Health>>,
    faction_query: Query<&Faction>,
    mut stats: ResMut<Stats>,
) {
    let hitbox = r!(trigger.get_target());
    let damage = rq!(damage_query.get(hitbox));

    // Record stats:
    let hitbox_faction = r!(faction_query.get(hitbox));
    // The hitbox_faction is for a missile, laser or other
    // projectile, so we must reverse factions here.
    match hitbox_faction {
        Faction::Enemy => stats.damage_taken += damage.0,
        Faction::Player => {
            stats.damage_given += damage.0;
            stats.highest_damage = stats.highest_damage.max(damage.0);
        },
    }

    let hurtbox = trigger.body.unwrap_or(trigger.collider);
    rq!(health_query.contains(hurtbox));

    commands.entity(hitbox).try_despawn();
    commands.entity(hurtbox).trigger(OnDamage(damage.0));
}

fn reduce_health_on_damage(
    trigger: Trigger<OnDamage>,
    mut health_query: Query<&mut Health, Without<Dead /* Stop! He's already dead! */>>,
) {
    let target = r!(trigger.get_target());
    rq!(health_query.get_mut(target)).current -= trigger.0;
}

fn play_ship_hurt_sfx_on_damage(
    _: Trigger<OnDamage>,
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    game_assets: Res<GameplayAssets>,
) {
    commands.spawn((
        sfx_audio(&audio_settings, game_assets.ship_hurt_sfx.clone(), 1.0),
        DespawnOnExitState::<Level>::default(),
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct DamagePopup;

impl Configure for DamagePopup {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(spawn_damage_popup_on_damage);
        app.add_systems(
            Update,
            apply_fade_out_to_damage_popup
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn spawn_damage_popup_on_damage(
    trigger: Trigger<OnDamage>,
    mut commands: Commands,
    assets: Res<Assets<Image>>,
    damage_config: ConfigRef<DamageConfig>,
    player_ship: Single<Entity, With<PlayerShip>>,
    player_ship_body: Single<(&Sprite, &GlobalTransform), With<PlayerShipBody>>,
    enemy_ship: Single<(Entity, &Sprite, &Transform), With<EnemyShip>>,
) {
    let target = r!(trigger.get_target());
    let damage_config = r!(damage_config.get());

    let (sprite, mut transform) = if target == *player_ship {
        (player_ship_body.0, player_ship_body.1.compute_transform())
    } else if target == enemy_ship.0 {
        (enemy_ship.1, *enemy_ship.2)
    } else {
        warn!("No match found for entity.");
        return;
    };
    let sprite_size = r!(assets.get(&sprite.image)).size_f32();

    // Randomize position.
    let rng = &mut thread_rng();
    let point = Rectangle::from_size(sprite_size).sample_interior(rng);
    transform.translation += point.extend(5.0);

    // Scale with number.
    let scale = (damage_config.damage_popup_scale
        * damage_config
            .damage_popup_scale_factor
            .max(1.0)
            .powf(trigger.0))
    .min(damage_config.damage_popup_scale_max);
    transform.scale = (transform.scale.xy() * Vec2::splat(scale)).extend(transform.scale.z);

    commands.spawn((
        Name::new("DamagePopup"),
        DamagePopup,
        Text2d::new("-".to_string()),
        TextFont {
            font: FONT_HANDLE,
            font_size: damage_config.damage_popup_font_size,
            ..default()
        },
        TextColor::from(damage_config.damage_popup_font_color),
        transform,
        RigidBody::Kinematic,
        LinearVelocity(damage_config.damage_popup_velocity),
        DespawnOnExitState::<Level>::default(),
        children![(
            Name::new("Number"),
            TextSpan(trigger.0.to_string()),
            TextFont {
                font: BOLD_FONT_HANDLE,
                font_size: damage_config.damage_popup_font_size,
                ..default()
            },
            TextColor::from(damage_config.damage_popup_font_color),
        )],
    ));
}

fn apply_fade_out_to_damage_popup(
    mut commands: Commands,
    time: Res<Time>,
    damage_config: ConfigRef<DamageConfig>,
    damage_popup_query: Query<(Entity, &Children), With<DamagePopup>>,
    mut text_color_query: Query<&mut TextColor>,
) {
    let damage_config = r!(damage_config.get());
    for (entity, children) in damage_popup_query {
        if let Ok(mut text_color) = text_color_query.get_mut(entity) {
            let alpha =
                text_color.0.alpha() - damage_config.damage_popup_fade_rate * time.delta_secs();
            text_color.0 = text_color.0.with_alpha(alpha);
            if text_color.0.alpha() < f32::EPSILON {
                commands.entity(entity).try_despawn();
            }
        }

        for &child in children {
            let mut text_color = cq!(text_color_query.get_mut(child));
            let alpha =
                text_color.0.alpha() - damage_config.damage_popup_fade_rate * time.delta_secs();
            text_color.0 = text_color.0.with_alpha(alpha);
        }
    }
}

use crate::animation::oscillate::Oscillate;
use crate::combat::damage::Damage;
use crate::combat::faction::Faction;
use crate::core::physics::GameLayer;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<ProjectileConfig>,
        Thruster,
        Homing,
        RotateWithThruster,
        Growth,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct ProjectileConfig {
    pub projectiles: HashMap<String, ProjectileInfo>,
}

impl Config for ProjectileConfig {
    const FILE: &'static str = "projectile.ron";

    fn on_load(&mut self, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        for projectile in self.projectiles.values_mut() {
            projectile.load(asset_server);
        }
    }

    fn count_progress(&self, asset_server: &AssetServer) -> Progress {
        let mut progress = true.into();
        for projectile in self.projectiles.values() {
            progress += projectile.count_progress(asset_server);
        }
        progress
    }
}

#[derive(Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectileInfo {
    pub name: String,
    #[serde(rename = "sprite")]
    pub sprite_path: String,
    #[serde(skip)]
    pub sprite: Handle<Image>,
    #[serde(rename = "sprite_empowered")]
    pub sprite_empowered_path: String,
    #[serde(skip)]
    pub sprite_empowered: Handle<Image>,
    #[serde(rename = "spawn_sfx", default)]
    pub spawn_sfx_path: String,
    #[serde(skip)]
    pub spawn_sfx: Option<Handle<AudioSource>>,
    #[serde(default = "one")]
    pub spawn_sfx_volume: f64,
    pub collider_radius: f32,
    pub collider_a: Vec2,
    pub collider_b: Vec2,

    pub damage: f32,
    pub scale: Vec2,
    pub scale_max: Vec2,
    pub scale_max_flux_factor: Vec2,
    pub growth: Vec2,
    pub growth_flux_factor: Vec2,
    pub position: Vec2,
    pub position_spread: Vec2,
    pub angle: f32,
    pub angle_spread: f32,
    pub speed: f32,
    pub speed_spread: f32,
    pub speed_max: f32,
    pub speed_max_flux_factor: f32,
    pub acceleration: Vec2,
    pub acceleration_flux_factor: f32,
    pub homing_approach: f32,
    pub homing_target_spread: Vec2,
    pub oscillate_amplitude: Vec2,
    pub oscillate_phase: Vec2,
    pub oscillate_rate: Vec2,
}

fn one() -> f64 {
    1.0
}

impl ProjectileInfo {
    pub fn generate(
        &self,
        rng: &mut impl Rng,
        mut transform: Transform,
        velocity: Vec2,
        faction: Faction,
        target: Entity,
        flux: f32,
    ) -> impl Bundle {
        // Calculate initial direction.
        let angle = transform.rotation.to_degrees()
            + self.angle
            + self.angle_spread * rng.gen_range(-1.0..=1.0);
        let angle = angle.to_radians();
        let direction = Vec2::from_angle(angle);

        // Calculate initial transform.
        transform.translation += direction
            .rotate(
                self.position
                    + self.position_spread
                        * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)),
            )
            .extend(0.0);
        transform.rotation = Quat::radians(angle);
        transform.scale *= self.scale.extend(1.0);

        // Calculate initial velocity and acceleration.
        let speed = self.speed + self.speed_spread * rng.gen_range(-1.0..=1.0);
        let velocity = velocity + speed * direction;
        let acceleration = direction.rotate(self.acceleration);

        // Calculate homing target position offset.
        let offset =
            self.homing_target_spread * vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));

        (
            Name::new(self.name.clone()),
            faction,
            Sprite::from_image(if flux >= 10.0 {
                self.sprite_empowered.clone()
            } else {
                self.sprite.clone()
            }),
            Damage(self.damage * flux),
            Growth {
                rate: self.growth * self.growth_flux_factor.powf(flux - 1.0),
                max_scale: self.scale_max * self.scale_max_flux_factor.powf(flux - 1.0),
            },
            Oscillate::new(
                self.oscillate_amplitude,
                self.oscillate_phase,
                self.oscillate_rate,
            ),
            Thruster(acceleration * self.acceleration_flux_factor.powf(flux - 1.0)),
            Homing {
                target,
                offset,
                approach: self.homing_approach,
            },
            RotateWithThruster,
            (
                LinearVelocity(velocity),
                MaxLinearSpeed(self.speed_max * self.speed_max_flux_factor.powf(flux - 1.0)),
                RigidBody::Dynamic,
                Mass(1.0),
                ExternalForce::ZERO.with_persistence(false),
                // TODO: Collider
                Collider::capsule_endpoints(self.collider_radius, self.collider_a, self.collider_b),
                CollisionLayers::new(GameLayer::Default, faction.opponent().layer()),
                CollisionEventsEnabled,
            ),
            transform,
            GlobalTransform::from(transform),
        )
    }

    fn load(&mut self, asset_server: &AssetServer) {
        self.sprite = asset_server.load(&self.sprite_path);
        self.sprite_empowered = asset_server.load(&self.sprite_empowered_path);
        if !self.spawn_sfx_path.is_empty() {
            self.spawn_sfx = Some(asset_server.load(&self.spawn_sfx_path));
        }
    }

    fn count_progress(&self, asset_server: &AssetServer) -> Progress {
        let mut progress = Progress::default();
        progress += asset_server
            .is_loaded_with_dependencies(&self.sprite)
            .into();
        progress += asset_server
            .is_loaded_with_dependencies(&self.sprite_empowered)
            .into();
        progress += self
            .spawn_sfx
            .as_ref()
            .is_none_or(|x| asset_server.is_loaded_with_dependencies(x))
            .into();
        progress
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Thruster(Vec2);

impl Configure for Thruster {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_thruster
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_thruster(mut thruster_query: Query<(&mut ExternalForce, &Thruster)>) {
    for (mut force, thruster) in &mut thruster_query {
        force.apply_force(thruster.0);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Homing {
    target: Entity,
    offset: Vec2,
    approach: f32,
}

impl Configure for Homing {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_homing
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_homing(
    time: Res<Time>,
    mut homing_query: Query<(
        &mut LinearVelocity,
        &mut Thruster,
        Option<&MaxLinearSpeed>,
        &GlobalTransform,
        &Homing,
    )>,
    target_query: Query<&GlobalTransform>,
) {
    for (mut velocity, mut thruster, maybe_max_speed, gt, homing) in &mut homing_query {
        let target_gt = cq!(target_query.get(homing.target));
        cq!(velocity.0 != Vec2::ZERO);

        // Calculate the exponential approach factor.
        let time_scale = if let Some(max_speed) = maybe_max_speed {
            if max_speed.0 == 0.0 {
                1.0
            } else {
                velocity.length() / max_speed.0
            }
        } else {
            1.0
        } * velocity.length()
            / 100.0;
        let decay = homing.approach.powf(time.delta_secs() * time_scale);

        // Calculate the target direction.
        let target_pos = target_gt.translation().xy() + homing.offset;
        let delta = target_pos - gt.translation().xy();

        // Rotate velocity and thruster towards the target direction.
        let rotation = velocity.angle_to(delta) * (1.0 - decay).clamp(0.0, 1.0);
        velocity.0 = Vec2::from_angle(rotation).rotate(velocity.0);
        let rotation = thruster.0.angle_to(delta) * (1.0 - decay).clamp(0.0, 1.0);
        thruster.0 = Vec2::from_angle(rotation).rotate(thruster.0);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RotateWithThruster;

impl Configure for RotateWithThruster {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            rotate_with_thruster
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn rotate_with_thruster(
    mut projectile_query: Query<(&mut Transform, &Thruster), With<RotateWithThruster>>,
) {
    for (mut transform, thruster) in &mut projectile_query {
        cq!(thruster.0 != Vec2::ZERO);
        transform.rotation = Quat::radians(thruster.0.to_angle());
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Growth {
    rate: Vec2,
    max_scale: Vec2,
}

impl Configure for Growth {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_growth
                .in_set(UpdateSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_growth(time: Res<Time>, mut growth_query: Query<(&mut Transform, &Growth)>) {
    for (mut transform, growth) in &mut growth_query {
        let delta = growth.rate * time.delta_secs();
        transform.scale = (transform.scale.xy() + delta)
            .min(growth.max_scale)
            .extend(transform.scale.z);
    }
}

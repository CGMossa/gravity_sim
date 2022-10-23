//! Tasks
//!
//! - [ ] Add softening parameter
//! - [ ] Add quality check calculations.
//! - [ ] Add visuals
#![allow(dead_code)]
use std::collections;

use bevy::{diagnostic::LogDiagnosticsPlugin, math::DVec2, prelude::*, time::FixedTimestep};
use itertools::Itertools;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 640.,
            height: 360.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_initial_objects)
        //TODO: add `Acceleration` to the initial objects.. And maybe `Jerk`.
        .add_startup_stage_after(
            StartupStage::Startup,
            "amend",
            SystemStage::single_threaded().with_system(amend_objects),
        )
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_planets)
        // .add_system(update_acceleration)
        // .add_system(update_forward_euler.after(update_acceleration))
        .add_stage_after(
            CoreStage::Last,
            "info",
            SystemStage::single_threaded().with_run_criteria(FixedTimestep::step(1.)),
        )
        .add_system_to_stage("info", print_status)
        .run()
}

fn setup_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_planets: Query<(Entity, &Mass, &Position, &SizeCategory)>,
    window_info: Res<WindowDescriptor>,
) {
    commands.spawn(Camera2dBundle::default());


    //TODO: do it differently.. provide non-decreasing list of sizes, and
    // reorder it according to the index given by `SizeCategory`.
    
    // index this with `SizeCategory`
    // [1] 3 2 5 1 4
    let sizes = [
        1000., //None
        3.,    // earth.gif
        2.5,   // mars.gif
        10.,   // mercury.gif
        25.,   // sun.gif
        3.,    // venus.gif
    ];

    q_planets.for_each(|(entity, mass, position, size_category)| {
        // let radius = mass.0 as _;
        let radius = sizes[size_category.0];
        let position = position.0 / UNIVERSE_RADIUS;
        todo!();
        commands
            .entity(entity)
            .insert(bevy::sprite::MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(Color::SILVER)),
                transform: Transform::from_translation(Vec3::new(10., 50., 0.)),
                ..default()
            });
    });
}

fn print_status() {
    /*
    Conserved quantities:
    1. Momentum
    2. Angular Momentum
    3. Energy
    */
}

fn add_initial_objects(mut commands: Commands) {
    // Source: https://www.cs.princeton.edu/courses/archive/spr16/cos126/assignments/nbody.html
    /*
    5
    2.50e+11
    1.4960e+11  0.0000e+00  0.0000e+00  2.9800e+04  5.9740e+24    earth.gif
    2.2790e+11  0.0000e+00  0.0000e+00  2.4100e+04  6.4190e+23     mars.gif
    5.7900e+10  0.0000e+00  0.0000e+00  4.7900e+04  3.3020e+23  mercury.gif
    0.0000e+00  0.0000e+00  0.0000e+00  0.0000e+00  1.9890e+30      sun.gif
    1.0820e+11  0.0000e+00  0.0000e+00  3.5000e+04  4.8690e+24    venus.gif
    */
    // N particles
    // size of the universe
    // positions x,y velocity x,y mass, asset name
    //> planets[, "m"]
    // [1] 5.974e+24 6.419e+23 3.302e+23 1.989e+30
    // [5] 4.869e+24
    // > planets[, "m"] %>%
    // +   order()
    // [1] 3 2 5 1 4
    commands.spawn((
        Position::new_x(1.4960e+11),
        Velocity::new_y(2.9800e+04),
        Mass::new(5.9740e+24),
        // "earth.gif",
        SizeCategory::new(3),
    ));
    commands.spawn((
        Position::new_x(2.2790e+11),
        Velocity::new_y(2.4100e+04),
        Mass::new(6.4190e+23),
        // "mars.gif",
        SizeCategory::new(2),
    ));
    commands.spawn((
        Position::new_x(5.7900e+10),
        Velocity::new_y(4.7900e+04),
        Mass::new(3.3020e+23),
        // "mercury.gif",
        SizeCategory::new(5),
    ));
    commands.spawn((
        Position::new_x(0.0000e+00),
        Velocity::new_y(0.0000e+00),
        Mass::new(1.9890e+30),
        // "sun.gif",
        SizeCategory::new(1),
    ));
    commands.spawn((
        Position::new_x(1.0820e+11),
        Velocity::new_y(3.5000e+04),
        Mass::new(4.8690e+24),
        // "venus.gif",
        SizeCategory::new(4),
    ));
}

fn amend_objects(mut commands: Commands, q_objects: Query<Entity, With<Position>>) {
    q_objects.for_each(|entity| {
        commands
            .entity(entity)
            .insert((Acceleration::zero(), Jerk::zero()));
    });
}

#[derive(Debug, Component)]
struct SizeCategory(usize);

impl SizeCategory {
    fn new(arg: usize) -> Self {
        Self(arg)
    }
}

#[derive(Debug, Component)]
struct Mass(f64);

impl Mass {
    fn new(arg: f64) -> Self {
        Self(arg)
    }
}

#[derive(Debug, Component)]
struct Position(DVec2);

impl Position {
    fn new_x(arg: f64) -> Self {
        Self(DVec2::new(arg, 0.))
    }
}

#[derive(Debug, Component)]
struct Velocity(DVec2);

impl Velocity {
    fn new_y(arg: f64) -> Self {
        Self(DVec2::new(0., arg))
    }
}

#[derive(Debug, Clone, Component)]
struct Acceleration(DVec2);

impl Acceleration {
    fn zero() -> Self {
        Self(DVec2::ZERO)
    }
}

#[derive(Debug, Clone, Component)]
struct Jerk(DVec2);
impl Jerk {
    fn zero() -> Self {
        Self(DVec2::ZERO)
    }
}

const GRAVITY_CONSTANT: f64 = 6.67e-11;
const UNIVERSE_RADIUS: f64 = 2.50e-11;

fn update_acceleration(mut q_objects: Query<(Entity, &Mass, &Position, &mut Acceleration)>) {
    // reset acceleration
    q_objects.for_each_mut(|x| {
        let mut acceleration: Mut<Acceleration> = x.3;
        acceleration.0 = DVec2::ZERO;
    });

    while let Some([mut origin, mut target]) = q_objects.iter_combinations_mut().fetch_next() {
        if origin.0 == target.0 {
            panic!("should this happen?")
        }
        let offset = origin.2 .0 - target.2 .0;
        origin.3 .0 -= GRAVITY_CONSTANT * target.1 .0 * offset / offset.length().powi(3);
        target.3 .0 += GRAVITY_CONSTANT * origin.1 .0 * offset / offset.length().powi(3);
    }
}

fn update_forward_euler(
    time: Res<Time>,
    mut q_objects: Query<(&mut Position, &mut Velocity, &Acceleration)>,
) {
    // x_{k+1} = x_k + v_k × \delta t
    // v_{k+1} = v_k + a_k × \delta t
    let delta_t = time.delta_seconds_f64();
    q_objects.for_each_mut(|(mut pos, mut vel, acc)| {
        pos.0 += vel.0 * delta_t;
        vel.0 += acc.0 * delta_t;
    });
}

fn update_jerk(
    q_objects: Query<(Entity, &Mass, &Position, &Velocity)>,
    q_jerk: Query<(Entity, &mut Jerk)>,
) {
    todo!()
}

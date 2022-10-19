use std::collections;

use bevy::{math::DVec2, prelude::*};
use itertools::Itertools;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_initial_objects)
        .add_system(update_acceleration)
        .add_system(update_forward_euler.after(update_acceleration))
        .add_system_to_stage(CoreStage::PostUpdate, print_status)
        .run()
}

fn print_status() {

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
    commands.spawn((
        Position::new_x(1.4960e+11),
        Velocity::new_y(2.9800e+04),
        Mass::new(5.9740e+24),
        // "earth.gif",
    ));
    commands.spawn((
        Position::new_x(2.2790e+11),
        Velocity::new_y(2.4100e+04),
        Mass::new(6.4190e+23),
        // "mars.gif",
    ));
    commands.spawn((
        Position::new_x(5.7900e+10),
        Velocity::new_y(4.7900e+04),
        Mass::new(3.3020e+23),
        // "mercury.gif",
    ));
    commands.spawn((
        Position::new_x(0.0000e+00),
        Velocity::new_y(0.0000e+00),
        Mass::new(1.9890e+30),
        // "sun.gif",
    ));
    commands.spawn((
        Position::new_x(1.0820e+11),
        Velocity::new_y(3.5000e+04),
        Mass::new(4.8690e+24),
        // "venus.gif",
    ));
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

const GRAVITY_CONSTANT: f64 = 6.67e-11;

fn update_acceleration(
    q_objects: Query<(Entity, &Mass, &Position)>,
    mut q_acceleration: Query<(Entity, &mut Acceleration)>,
) {
    // calculate the new acceleration
    let new_acceleration: collections::HashMap<Entity, Acceleration> = q_objects
        .iter_combinations::<2>()
        .into_grouping_map_by(|[origin, _target]| -> Entity { origin.0 })
        .fold(
            Acceleration::zero(),
            |mut acc, &origin_entity, [origin, target]| {
                let target_entity: Entity = target.0;
                if origin_entity == target_entity {
                    return acc;
                }
                let target_mass: &Mass = target.1;
                // let target_position: &Position = target.2;
                let direction = origin.2 .0 - target.2 .0;
                let distance = direction.length().powi(3);
                acc.0 -= GRAVITY_CONSTANT * target_mass.0 * direction / distance;
                acc
            },
        );
    // apply
    q_acceleration.for_each_mut(|(entity, mut acc)| {
        acc.0 = new_acceleration[&entity].0;
    })
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

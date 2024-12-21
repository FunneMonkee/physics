use crate::constants::constants::{coulombs_constant, gravity_force};

pub fn force_between_two_charges(charge_1: f64, charge_2: f64, distance: f64) -> f64 {
    (coulombs_constant() * charge_1 * charge_2 / distance.powi(2)).abs()
}

pub fn gravitational_force(mass: f64, distance: f64) -> f64 {
    (gravity_force() * mass.powi(2) / distance.powi(2)).abs()
}

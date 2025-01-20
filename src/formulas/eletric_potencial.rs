use std::{f64, ops::Neg};

use crate::constants::constants::{coulombs_constant, eletrical_vacuum_permittivity};

pub fn eletric_field(charge: f64, radius: f64) -> f64 {
    coulombs_constant() * charge / radius.powi(2)
}

pub fn eletric_field_from_surfuce_charge(charge: f64) -> f64 {
    (charge / (2. * eletrical_vacuum_permittivity())).abs()
}

pub fn eletric_potencial(charge: f64, radius: f64) -> f64 {
    coulombs_constant() * charge / radius
}

pub fn eletric_potencial_p(charge: f64, eletric_potencial: f64) -> f64 {
    charge * eletric_potencial
}

pub fn work_from_a_to_b(charge: f64, eletric_potencial_a: f64, eletric_potencial_b: f64) -> f64 {
    charge * potencial_difference(eletric_potencial_a, eletric_potencial_b)
}

pub fn work_from_a_to_b_from_eletric_field(charge: f64, eletric_field: f64, distance: f64) -> f64 {
    charge * potencial_difference_from_eletric_field(eletric_field, distance)
}

pub fn potencial_difference(eletric_potencial_a: f64, eletric_potencial_b: f64) -> f64 {
    eletric_potencial_a - eletric_potencial_b
}

pub fn potencial_difference_from_eletric_field(eletric_field: f64, distance: f64) -> f64 {
    eletric_field * distance
}

pub fn eletric_field_force(charge: f64, eletric_field: f64) -> f64 {
    charge * eletric_field
}

pub fn eletric_field_from_potencial_difference(potencial_difference: f64, distance: f64) -> f64 {
    potencial_difference / distance
}

pub fn potencial_energy(charge: f64, potencial_difference: f64) -> f64 {
    charge * potencial_difference
}

pub fn kinetic_energy_from_potencial_energy(potencial_energy: f64) -> f64 {
    potencial_energy.neg()
}

pub fn velocity_from_kinetic_energy_and_mass(kinetic_energy: f64, mass: f64) -> f64 {
    (kinetic_energy * 2. / mass).sqrt()
}

pub fn kinetic_energy_from_capacitor_and_potencial_difference(
    capacitor: f64,
    potencial_difference: f64,
) -> f64 {
    0.5 * capacitor * potencial_difference.powi(2)
}

pub fn permittivity_from_relative_permittivity(relative_permittivity: f64) -> f64 {
    eletrical_vacuum_permittivity() * relative_permittivity
}

pub fn area_from_capacitor_permittivity_distance(
    capacitor: f64,
    permittivity: f64,
    distance: f64,
) -> f64 {
    capacitor * distance / permittivity
}

use std::ops::Neg;

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

pub fn work_from_a_to_b(charge: f64, eletric_potencial_a: f64, eletric_potencial_b: f64) -> f64 {
    charge * potencial_difference(eletric_potencial_a, eletric_potencial_b)
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

pub fn potencial_energy(charge: f64, potencial_difference: f64) -> f64 {
    charge * potencial_difference
}

pub fn kinetic_energy_from_potencial_energy(potencial_energy: f64) -> f64 {
    potencial_energy.neg()
}

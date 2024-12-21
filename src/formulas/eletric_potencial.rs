use crate::constants::constants::coulombs_constant;

pub fn eletric_field(charge: f64, radius: f64) -> f64 {
    coulombs_constant() * charge / radius.powi(2)
}

pub fn eletric_potencial(charge: f64, radius: f64) -> f64 {
    coulombs_constant() * charge / radius
}

pub fn work_from_a_to_b(charge: f64, eletric_potencial_a: f64, eletric_potencial_b: f64) -> f64 {
    charge * (eletric_potencial_a - eletric_potencial_b)
}

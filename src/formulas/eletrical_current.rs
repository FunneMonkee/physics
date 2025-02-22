use std::ops::Neg;

use crate::constants::constants::elemental_charge;

pub fn velocity_from_current_electron_density_and_area(
    electrical_current: f64,
    electron_density: f64,
    area: f64,
) -> f64 {
    electrical_current / (electron_density * elemental_charge() * area)
}

pub fn electromotive_force(transformed_energy: f64, charge: f64) -> f64 {
    transformed_energy / charge
}

pub fn energy_from_time_and_potency(potency: f64, time: f64) -> f64 {
    potency * time
}

pub fn intensity_from_potency_and_current(potency: f64, current: f64) -> f64 {
    potency / current
}

pub fn intensity_from_resistance_and_current(resistance: f64, current: f64) -> f64 {
    current / resistance
}

pub fn resistance_from_current_and_intensity(current: f64, intensity: f64) -> f64 {
    current / intensity
}

pub fn intensity_from_resistance_and_potency(resistance: f64, potency: f64) -> f64 {
    (potency / resistance).sqrt()
}

pub fn potency_from_resistance_and_intensity(resistance: f64, intensity: f64) -> f64 {
    resistance * intensity.powi(2)
}

pub fn resistance_from_resistivity(resistivity: f64, length: f64, area: f64) -> f64 {
    resistivity * length / area
}

pub fn current_between_two_points_from_current_resistance_intensity(
    current: f64,
    resistance: f64,
    intensity: f64,
) -> f64 {
    current + (resistance * intensity)
}

pub fn capacitor_time_constant(resistance: f64, capacitor: f64) -> f64 {
    resistance * capacitor
}

pub fn time_to_charge_capacitor(time_constant: f64, total_charge: f64) -> f64 {
    total_charge.neg().ln_1p() * time_constant.neg()
}

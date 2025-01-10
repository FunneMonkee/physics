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

use std::f64::consts::PI;

pub fn magnetic_force(charge: f64, speed: f64, magnetic_field: f64) -> f64 {
    charge * speed * magnetic_field
}

pub fn radius_from_mass_speed_magnetic_field_and_charge(
    mass: f64,
    speed: f64,
    charge: f64,
    magnetic_field: f64,
) -> f64 {
    (mass * speed) / (charge * magnetic_field)
}

pub fn speed_from_radius_mass_charge_and_magnetic_field(
    mass: f64,
    charge: f64,
    magnetic_field: f64,
    radius: f64,
) -> f64 {
    radius * charge * magnetic_field / mass
}

pub fn time_in_field_circle(speed: f64, radius: f64) -> f64 {
    2. * PI / speed
}

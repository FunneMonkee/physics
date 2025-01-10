use crate::{
    formulas::{
        eletrical_current::{electromotive_force, velocity_from_current_electron_density_and_area},
        geometry::cilinder_area,
    },
    units::unit::Unit,
};

use super::helper::answer;

pub fn solve() {
    println!("Second Theme");
    ex1();
}

fn ex1() {
    let number_of_electrons = Unit {
        value: 5.8,
        prefix: 28,
        is_grams: false,
    }
    .to_base();
    let current = 1.;
    let diameter = Unit {
        value: 1.,
        prefix: -3,
        is_grams: false,
    }
    .to_base();

    answer(
        "Ex1",
        "",
        velocity_from_current_electron_density_and_area(
            current,
            number_of_electrons,
            cilinder_area(diameter / 2.),
        ),
        "m/s",
    );
}

fn ex2() {
    let battery = 6.;
    let charge = 3.;

    answer("Ex2", "part 1", electromotive_force(battery, charge), "V");
}

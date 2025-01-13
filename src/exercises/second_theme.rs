use crate::{
    formulas::{
        eletrical_current::{
            electromotive_force, energy_from_time_and_potency, intensity_from_potency_and_current,
            intensity_from_resistance_and_potency, potency_from_resistance_and_intensity,
            resistance_from_current_and_intensity, resistance_from_resistivity,
            velocity_from_current_electron_density_and_area,
        },
        geometry::cilinder_area,
    },
    units::unit::Unit,
};

use super::helper::answer;

pub fn solve() {
    println!("Second Theme");
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
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

fn ex3() {
    let n_days = 15.;
    let n_lamps = 3.;
    let hours = 5.;
    let potency = 100.;
    let price_per_whatt = Unit {
        value: 0.0647,
        prefix: -3,
        is_grams: false,
    }
    .to_base();

    let time = hours * n_days;
    let potency_sum = potency * n_lamps;

    let total = energy_from_time_and_potency(potency_sum, time) * price_per_whatt;
    answer("Ex3", "", total, "€");
}

fn ex4() {
    let resistance;
    //b
    {
        let potency = 60.;
        let current = 120.;
        let intensity = intensity_from_potency_and_current(potency, current);
        resistance = resistance_from_current_and_intensity(current, intensity);

        answer("Ex4", "b1) intensity", intensity, "A");
        answer("Ex4", "b2) resistance", resistance, "Ω");
    }
    //c
    {
        //resistance is immutable so 240Ω
        answer(
            "Ex4",
            "c)",
            potency_from_resistance_and_intensity(resistance, 0.25),
            "W",
        );
    }
}

fn ex5() {
    let radius = Unit {
        value: 0.362,
        prefix: -2,
        is_grams: false,
    }
    .to_base()
        / 2.;
    let resistivity = Unit {
        value: 1.72,
        prefix: -8,
        is_grams: false,
    }
    .to_base();
    let resistance;
    //a
    {
        resistance = resistance_from_resistivity(resistivity, 1., cilinder_area(radius));
        answer("Ex5", "a)", resistance, "Ω/m");
    }
    //b
    {
        answer(
            "Ex5",
            "b)",
            intensity_from_resistance_and_potency(resistance, 3.),
            "A",
        );
    }
    //c
    {}
}

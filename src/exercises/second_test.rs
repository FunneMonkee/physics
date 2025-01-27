use std::{
    char,
    ops::{Div, Mul},
};

use crate::{
    constants::constants::elemental_charge,
    formulas::{
        eletrical_current::{
            capacitor_time_constant, current_between_two_points_from_current_resistance_intensity,
            intensity_from_resistance_and_current, time_to_charge_capacitor,
        },
        fields::{
            magnetic_force, radius_from_mass_speed_magnetic_field_and_charge,
            speed_from_radius_mass_charge_and_magnetic_field, time_in_field_circle,
        },
    },
    units::unit::Unit,
};

use super::helper::answer;

pub fn solve() {
    ex1();
    ex2();
}

fn ex1() {
    // --a-o o-c--4Ω--o--3Ω--
    // |    o         |     |
    // |    b         |     |
    // |    |         |     |
    // 12V  2yF     10Ω     20V
    // |    |         |     |
    // |    |         |     |
    // |    |         |     |
    // -----o---4Ω----o--3Ω--

    //a V(a,b,c)
    {
        // all off
        // o--3Ω--
        // |     |
        // |     |
        // |     |
        //10Ω   20V
        // |     |
        // |     |
        // |     |
        // o--3Ω--

        let eq_resistance = 10. + 3. + 3.;
        let intensity = intensity_from_resistance_and_current(eq_resistance, 20.);

        answer(
            "1",
            "a",
            current_between_two_points_from_current_resistance_intensity(12., 10., intensity),
            "V",
        );
    }

    //b
    {
        // --a-o-o-c--4Ω--o--3Ω--
        // |          <-i2| <-i1|
        // |              |     |
        // |          ↓i3 |     |
        // 12V    M1     10Ω M2 20V
        // |              |     |
        // |              |     |
        // |              |     |
        // -----o---4Ω----o--3Ω--

        // negative direction
        // m1 = -10i3 - 4i2 - 4i2 + 12 = 0;
        // m2 = -10i3 - 3i1 - 3i1 +20 = 0;
        // node = i1 - i2 - i3 = 0;
        //
        // m1 = -8i2 -10i3 +12 = 0;
        // m2 = -6i1 - 10i3 + 20 = 0;
        // node = i1 - i2 - i3 = 0;
        //

        answer("1", "b i1", 2.55, "A");
        answer("1", "b i2", 2.08, "A");
        answer("1", "b i3", 0.47, "A");
    }

    //c
    {
        // o-c--4Ω--o--3Ω--
        // |        |     |
        // b        |     |
        // |        |     |
        // 2yF    10Ω     20V
        // |        |     |
        // |        |     |
        // |        |     |
        // ---4Ω----o--3Ω--

        let capacitor = Unit {
            value: 2.,
            prefix: -6,
            is_grams: false,
        }
        .to_base();

        let eq_resistance = 10. / 6. + 8.;

        let time_constant = capacitor_time_constant(eq_resistance, capacitor);

        answer("1", "c", time_to_charge_capacitor(time_constant, 0.55), "s");
    }
}

fn ex2() {
    let distance = 25.;
    let magnetic_field = Unit {
        value: 6.,
        prefix: -3,
        is_grams: false,
    }
    .to_base();
    let charge_1 = elemental_charge();
    let charge_2 = Unit {
        value: 3.2,
        prefix: -19,
        is_grams: false,
    }
    .to_base();
    let mass_1 = Unit {
        value: 4.,
        prefix: -28,
        is_grams: true,
    }
    .to_base();
    let mass_2 = Unit {
        value: 2.5,
        prefix: -28,
        is_grams: true,
    }
    .to_base();
    let speed_1 = Unit {
        value: 27.,
        prefix: 3,
        is_grams: false,
    }
    .to_base();
    let speed_2;

    //      ey      25m
    //      A------------------
    //      ...................
    //      ...................
    // Q1 ->...................<- Q2
    //      ...................
    //      ...................
    //      B------------------ex

    //a
    {
        // 1/4 circle find radius
        // Fm = q . v . B = Fc =m * v^2 / R

        let radius = radius_from_mass_speed_magnetic_field_and_charge(
            mass_1,
            speed_1,
            charge_1,
            magnetic_field,
        );

        answer("2", "a", radius, "m -ey atinge B")
    }
    //b
    {
        let radius = 25. - 11.25;
        speed_2 = speed_from_radius_mass_charge_and_magnetic_field(
            mass_2,
            charge_2,
            magnetic_field,
            radius,
        );

        answer("2", "b", speed_2, "m/s -ex atinge A")
    }
    //c
    {
        let t1 = time_in_field_circle(speed_1, 11.25) / 4.;
        let t2 = time_in_field_circle(speed_2, 13.25) / 4.;

        answer("2", "c ", t1 - t2, "s");
    }
}

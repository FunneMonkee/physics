use std::ops::{Div, Mul};

use crate::{
    formulas::eletrical_current::{
        capacitor_time_constant, current_between_two_points_from_current_resistance_intensity,
        intensity_from_resistance_and_current, time_to_charge_capacitor,
    },
    units::unit::Unit,
};

use super::helper::answer;

pub fn solve() {
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

use std::char;

use crate::{
    formulas::{
        eletric_potencial::{eletric_field, eletric_field_force, eletric_potencial},
        geometry::to_radians,
    },
    units::unit::Unit,
};

use super::helper::answer;

pub fn solve() {
    //  y
    // -q ----P---- A
    // --------------
    // ------+2q-----
    // --------------
    // +B------Q----- -q
    //                x

    let square_side = Unit {
        value: 6.,
        prefix: -2,
        is_grams: false,
    }
    .to_base();
    //a
    {
        let charge_q = Unit {
            value: 1.,
            prefix: -19,
            is_grams: false,
        }
        .to_base();
        let charge_2q = Unit {
            value: 2.,
            prefix: -19,
            is_grams: false,
        }
        .to_base();

        let distance = (square_side.powi(2) * 2.).sqrt() / 2.;
        //1ey 2ex attracting
        let field_1 = eletric_field(charge_q, square_side);
        //cos 45 ex sen 45 ey pushing
        let field_3 = eletric_field(charge_2q * 2., distance);

        let resulting_field_ex = field_1 - field_3 * to_radians(45.).cos();
        let resulting_field_ey = field_1 - field_3 * to_radians(45.).sin();

        answer("1", "a ex", resulting_field_ex, "ex N/C");
        answer("1", "a ey", resulting_field_ey, "ey N/C");
        answer(
            "1",
            "a res",
            (resulting_field_ey.powi(2) + resulting_field_ex.powi(2)).sqrt(),
            "N/C",
        );
    }
}

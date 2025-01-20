use crate::{
    formulas::{
        eletric_potencial::{
            eletric_field, eletric_potencial, eletric_potencial_p, work_from_a_to_b,
        },
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
    let charge_2q = Unit {
        value: 2.,
        prefix: -19,
        is_grams: false,
    }
    .to_base();
    let charge_q = Unit {
        value: 1.,
        prefix: -19,
        is_grams: false,
    }
    .to_base();
    let distance = (square_side.powi(2) * 2.).sqrt() / 2.;
    //a
    {
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
    //b
    {
        let charge_a = Unit {
            value: -4.,
            prefix: -9,
            is_grams: false,
        }
        .to_base();

        //a but simetric
        let field_1 = eletric_field(charge_q, square_side);
        let field_3 = eletric_field(charge_2q * 2., distance);

        let resulting_field_ex = (field_1 - field_3 * to_radians(45.).cos()) * charge_a;
        let resulting_field_ey = (field_1 - field_3 * to_radians(45.).sin()) * charge_a;

        answer("1", "b ex", resulting_field_ex, "ex N/C");
        answer("1", "b ey", resulting_field_ey, "ey N/C");
        answer(
            "1",
            "b res",
            (resulting_field_ey.powi(2) + resulting_field_ex.powi(2)).sqrt(),
            "N/C",
        );
    }
    //c
    {
        let half_square = square_side / 2.;
        let charge_p = Unit {
            value: -4.,
            prefix: -9,
            is_grams: false,
        }
        .to_base();
        let distance_q_to_p = (half_square.powi(2) + square_side.powi(2)).sqrt();

        let eletric_potencial_p_2q = eletric_potencial(2. * charge_2q, half_square);
        let eletric_potencial_p_q = eletric_potencial(charge_q, half_square);
        let eletric_potencial_p_lower_q = eletric_potencial(charge_q, distance_q_to_p);

        let eletric_potencial_sum =
            eletric_potencial_p_q + eletric_potencial_p_2q + eletric_potencial_p_lower_q;

        let eletric_potencial = eletric_potencial_p(charge_p, eletric_potencial_sum);

        answer(
            "1",
            "c",
            work_from_a_to_b(charge_p, eletric_potencial, eletric_potencial),
            "J",
        );
    }
}

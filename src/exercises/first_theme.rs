use crate::{constants::constants::{elemental_charge, proton_mass}, formulas::{eletric_potencial::{self, eletric_field, eletric_potencial, work_from_a_to_b}, forces::{force_between_two_charges, gravitational_force}, trignometry::to_radians}, units::unit::Unit};

fn answer(question: &str, sub_question: &str, number: f64, unit: &str) -> () {
    let mut out_number = number.to_string();

    if out_number.len() > 4 {
        out_number = format!("{:e}", number); 
    }

    println!("{} {} Answer: {} {}", question, sub_question, out_number, unit);
}

pub fn solve() {
    ex1();
    ex2();
    ex5();
}

fn ex1() {
    let radius = Unit {
            value: 4.,
            prefix: -15,
            is_grams: false
        };
    // a)
    {
        let charge = elemental_charge();

        answer("Ex1", "a)", force_between_two_charges(charge, charge, radius.to_base()), "N");
    }
    // b)
    {
        let mass = proton_mass();

        answer("Ex1", "a)", gravitational_force(mass, radius.to_base()), "N");
    }
}

fn ex2() {
    let distance = 1.;
    let angle: f64 = to_radians(60.);
    let charge_1 = Unit {
        value: -2.,
        prefix: -6,
        is_grams: false
    };
    let charge_2 = Unit {
        value: 1.,
        prefix: -6,
        is_grams: false
    };
    let force_13 = force_between_two_charges(charge_1.to_base(), charge_2.to_base(), distance);
    let force_12 = force_between_two_charges(charge_1.to_base(), charge_2.to_base(), distance);
    let force_12_x_axis = force_12 * angle.cos();
    let force_12_y_axis = force_12 * angle.sin();

    answer("Ex2", "", force_12_x_axis + force_13, "ex N");
    answer("Ex2", "", force_12_y_axis, "ey N");
}

fn ex5() {
    //ey +q---------A ex
    //    -----------
    //    -----P-----
    //    -----------
    //   2q---------+q 
    let charge_2q = elemental_charge() * 2.;
    let square_side = Unit {
        value: 5.,
        prefix: -3,
        is_grams: false
    };
    let eletric_field_sum: f64;
    let eletric_potencial_sum: f64;
    //a
    {
        //q and q cancel each other
        let square_side_in_meters = square_side.to_base();
        let distance_to_p = (square_side_in_meters.powi(2) * 2.).sqrt() / 2.;
        eletric_field_sum = eletric_field(charge_2q, distance_to_p);
        answer("Ex5", "a)", eletric_field_sum ,"N/C ex");
    }
    //b
    {
        let square_side_in_meters = square_side.to_base();
        let distance_to_p = (square_side_in_meters.powi(2) * 2.).sqrt() / 2.;

        let eletric_potencial_2q = eletric_potencial(charge_2q, distance_to_p);
        let eletric_potencial_q = eletric_potencial(elemental_charge(), distance_to_p);
        eletric_potencial_sum = eletric_potencial_q * 2. + eletric_potencial_2q;

        answer("Ex5", "b)", eletric_potencial_sum,"V");
    }
    //c
    let charge_p = elemental_charge() * 100.;
    {
        let force = eletric_field_sum * charge_p; 

        answer("Ex5", "c)", force, "N");
    }
    //d
    {
        let square_side_in_meters = square_side.to_base();
        let distance_to_p = (square_side_in_meters.powi(2) * 2.).sqrt();

        let eletric_potencial_a_2q = eletric_potencial(charge_2q, distance_to_p);
        let eletric_potencial_a_q = eletric_potencial(elemental_charge(), square_side_in_meters);

        let eletric_potencial_a = eletric_potencial_a_q * 2. + eletric_potencial_a_2q;

        answer("Ex5", "d)", work_from_a_to_b(charge_p, eletric_potencial_a, eletric_potencial_sum), "J");
    }
}

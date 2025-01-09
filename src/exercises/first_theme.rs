use std::ops::Neg;

use crate::{
    constants::constants::{elemental_charge, eletron_mass, proton_mass},
    formulas::{
        eletric_potencial::{
            area_from_capacitor_permittivity_distance, eletric_field, eletric_field_force,
            eletric_field_from_potencial_difference, eletric_field_from_surfuce_charge,
            eletric_potencial, kinetic_energy_from_capacitor_and_potencial_difference,
            kinetic_energy_from_potencial_energy, permittivity_from_relative_permittivity,
            potencial_difference_from_eletric_field, potencial_energy,
            velocity_from_kinetic_energy_and_mass, work_from_a_to_b,
            work_from_a_to_b_from_eletric_field,
        },
        forces::{force_between_two_charges, gravitational_force},
        trignometry::to_radians,
    },
    units::unit::Unit,
};

fn answer(question: &str, sub_question: &str, number: f64, unit: &str) -> () {
    let mut out_number = number.to_string();

    if out_number.len() > 4 {
        out_number = format!("{:e}", number);
    }

    println!(
        "{} {} Answer: {} {}",
        question, sub_question, out_number, unit
    );
}

pub fn solve() {
    ex1();
    ex2();
    ex5();
    ex10();
    ex11();
    ex13();
    ex15();
    ex16();
}

fn ex1() {
    let radius = Unit {
        value: 4.,
        prefix: -15,
        is_grams: false,
    };
    // a)
    {
        let charge = elemental_charge();

        answer(
            "Ex1",
            "a)",
            force_between_two_charges(charge, charge, radius.to_base()),
            "N",
        );
    }
    // b)
    {
        let mass = proton_mass();

        answer(
            "Ex1",
            "a)",
            gravitational_force(mass, radius.to_base()),
            "N",
        );
    }
}

fn ex2() {
    let distance = 1.;
    let angle: f64 = to_radians(60.);
    let charge_1 = Unit {
        value: -2.,
        prefix: -6,
        is_grams: false,
    };
    let charge_2 = Unit {
        value: 1.,
        prefix: -6,
        is_grams: false,
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
        is_grams: false,
    };
    let eletric_field_sum: f64;
    let eletric_potencial_sum: f64;
    //a
    {
        //q and q cancel each other
        let square_side_in_meters = square_side.to_base();
        let distance_to_p = (square_side_in_meters.powi(2) * 2.).sqrt() / 2.;
        eletric_field_sum = eletric_field(charge_2q, distance_to_p);
        answer("Ex5", "a)", eletric_field_sum, "N/C ex");
    }
    //b
    {
        let square_side_in_meters = square_side.to_base();
        let distance_to_p = (square_side_in_meters.powi(2) * 2.).sqrt() / 2.;

        let eletric_potencial_2q = eletric_potencial(charge_2q, distance_to_p);
        let eletric_potencial_q = eletric_potencial(elemental_charge(), distance_to_p);
        eletric_potencial_sum = eletric_potencial_q * 2. + eletric_potencial_2q;

        answer("Ex5", "b)", eletric_potencial_sum, "V");
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
        let distance_to_a = (square_side_in_meters.powi(2) * 2.).sqrt();

        let eletric_potencial_a_2q = eletric_potencial(charge_2q, distance_to_a);
        let eletric_potencial_a_q = eletric_potencial(elemental_charge(), square_side_in_meters);

        let eletric_potencial_a = eletric_potencial_a_q * 2. + eletric_potencial_a_2q;

        answer(
            "Ex5",
            "d)",
            work_from_a_to_b(charge_p, eletric_potencial_a, eletric_potencial_sum),
            "J",
        );
    }
}

fn ex10() {
    let distance = 0.5;
    let eletric_field = Unit {
        value: 8.,
        prefix: 4,
        is_grams: false,
    };
    let charge = elemental_charge();

    //a
    {
        let mass = proton_mass();
        let gravity = 10.;
        let gravity_force = mass * gravity;

        let eletric_field_force = eletric_field_force(charge, eletric_field.to_base());
        answer("Ex10", "a)", gravity_force + eletric_field_force, "N");
    }
    //b
    let potencial_difference =
        potencial_difference_from_eletric_field(eletric_field.to_base(), distance);
    {
        answer("Ex10", "b)", potencial_difference, "V");
    }
    //c
    {
        let potencial_energy = potencial_energy(charge, potencial_difference);

        answer(
            "Ex10",
            "c)",
            kinetic_energy_from_potencial_energy(potencial_energy),
            "V",
        );
    }
}

fn ex11() {
    let surfuce_charge_positive = Unit {
        value: 6.8,
        prefix: -6,
        is_grams: false,
    };
    let surfuce_charge_negative = Unit {
        value: 4.3,
        prefix: -6,
        is_grams: false,
    };
    let eletric_field: f64;
    let potencial_difference: f64;

    //a
    {
        eletric_field = eletric_field_from_surfuce_charge(surfuce_charge_positive.to_base())
            + eletric_field_from_surfuce_charge(surfuce_charge_negative.to_base());

        answer("Ex11", "a)", eletric_field, "N/C");
    }
    //b
    {
        let distance = Unit {
            value: 5.,
            prefix: -3,
            is_grams: false,
        };

        potencial_difference =
            potencial_difference_from_eletric_field(eletric_field, distance.to_base());

        answer("Ex11", "b)", potencial_difference, "V");
    }
    //c
    {
        let charge = Unit {
            value: -3.5,
            prefix: -6,
            is_grams: false,
        };

        answer("Ex11", "c)", charge.to_base() * potencial_difference, "J");
    }
}

fn ex13() {
    //ey  ---------+q ex
    //    -----------
    //    -----A-----
    //    -----------
    //   +q---------+2q
    let charge = Unit {
        value: 3.2,
        prefix: -19,
        is_grams: false,
    };
    let charge_2q = charge.to_base() * 2.;
    let square_side = Unit {
        value: 7.5,
        prefix: -3,
        is_grams: false,
    };
    let eletric_field_sum: f64;
    //a
    {
        let square_side_in_meters = square_side.to_base();
        let distance_to_p = (square_side_in_meters.powi(2) * 2.).sqrt() / 2.;

        let eletric_potencial_2q = eletric_potencial(charge_2q, distance_to_p);
        let eletric_potencial_q = eletric_potencial(charge.to_base(), distance_to_p);
        let eletric_potencial_sum = eletric_potencial_q * 2. + eletric_potencial_2q;

        answer("Ex13", "a)", eletric_potencial_sum, "V");
    }
    //b
    {
        //q and q cancel each other
        let square_side_in_meters = square_side.to_base();
        let distance_to_p = (square_side_in_meters.powi(2) * 2.).sqrt() / 2.;
        eletric_field_sum = eletric_field(charge_2q, distance_to_p);
        answer("Ex13", "b)", eletric_field_sum, "N/C ex");
    }

    //c
    let charge_p = charge.to_base() * 150.;
    {
        let force = eletric_field_sum * charge_p;

        answer("Ex13", "c)", force, "N");
    }
}

fn ex15() {
    let capacitor = Unit {
        value: 5.,
        prefix: -7,
        is_grams: false,
    }
    .to_base();
    let permittivity = 2.3;
    let thickness = Unit {
        value: 2.,
        prefix: -4,
        is_grams: false,
    }
    .to_base();
    let potencial_difference = 80.;
    //a
    {
        answer(
            "Ex15",
            "a)",
            area_from_capacitor_permittivity_distance(
                capacitor,
                permittivity_from_relative_permittivity(permittivity),
                thickness,
            ),
            "m2",
        );
    }
    //b
    {
        answer(
            "Ex15",
            "b)",
            kinetic_energy_from_capacitor_and_potencial_difference(capacitor, potencial_difference),
            "J",
        );
    }
    //c
    {
        answer(
            "Ex15",
            "c)",
            eletric_field_force(
                elemental_charge(),
                eletric_field_from_potencial_difference(potencial_difference, thickness),
            ),
            "N",
        );
    }
    //d
    {
        //conservative system Ec=Ep
        answer(
            "Ex15",
            "d)",
            velocity_from_kinetic_energy_and_mass(
                potencial_energy(elemental_charge(), potencial_difference),
                eletron_mass(),
            ),
            "m/s",
        );
    }
}

fn ex16() {
    let charge = Unit {
        value: 3.,
        prefix: -15,
        is_grams: false,
    }
    .to_base();
    let eletric_field = Unit {
        value: 2.,
        prefix: 3,
        is_grams: false,
    }
    .to_base();
    let radius_a = Unit {
        value: 3.,
        prefix: -2,
        is_grams: false,
    }
    .to_base();
    let radius_b = Unit {
        value: 4.,
        prefix: -2,
        is_grams: false,
    }
    .to_base();
    let distance = (radius_a.powi(2) + radius_b.powi(2)).sqrt();

    //a
    {
        answer(
            "Ex16",
            "a)",
            eletric_field_force(charge, eletric_field),
            "N",
        );
    }
    //b
    {
        answer(
            "Ex16",
            "b)",
            work_from_a_to_b_from_eletric_field(charge, eletric_field, distance).neg(), // -ex
            "J",
        );
    }
}

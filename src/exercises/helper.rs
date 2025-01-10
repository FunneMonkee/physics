pub fn answer(question: &str, sub_question: &str, number: f64, unit: &str) -> () {
    let mut out_number = number.to_string();

    if out_number.len() > 4 {
        out_number = format!("{:e}", number);
    }

    println!(
        "{} {} Answer: {} {}",
        question, sub_question, out_number, unit
    );
}

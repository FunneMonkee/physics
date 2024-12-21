pub struct Unit {
    pub value: f64,
    pub prefix: i32,
    pub is_grams: bool
}

impl Unit {
    pub fn to_base(&self) -> f64 {
        let mut base = self.value * 10f64.powi(self.prefix);

        if self.is_grams {
            base = base * 10f64.powi(3);
        }

        base
    }
}

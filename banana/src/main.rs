use std::collections::HashMap;

#[derive(Debug)]
enum Unit {
    NoUnit,
    KCal,
    Grams,
    Percent,
}

#[derive(Debug)]
struct Amount {
    magnitude: f32,
    unit: Unit,
}

impl Amount {
    fn new(magnitude: f32, unit: Unit) -> Self {
        Self { magnitude, unit }
    }
}

#[derive(Debug)]
struct Banana {
    nutrition: HashMap<String, Amount>,
}

impl Banana {
    fn new() -> Self {
        Self {
            nutrition: HashMap::new(),
        }
    }
}

fn main() {
    let mut banana = Banana::new();
    banana
        .nutrition
        .insert("Energy".to_string(), Amount::new(89.0, Unit::KCal));
    banana
        .nutrition
        .insert("Carbs".to_string(), Amount::new(22.84, Unit::Grams));
    banana
        .nutrition
        .insert("Fat".to_string(), Amount::new(0.33, Unit::Grams));
    banana
        .nutrition
        .insert("Protein".to_string(), Amount::new(1.09, Unit::Grams));
    banana
        .nutrition
        .insert("Water".to_string(), Amount::new(74.91, Unit::Percent));
    
    println!("{:?}", banana.nutrition.get("Water"));
}

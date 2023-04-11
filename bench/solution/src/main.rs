const WEIGHT: i32 = 90;
const REPS: i32 = 6;

fn greets(name: &str) -> String {
    format!("Hello {} 🦀 !", name)
}

fn calculate_total_weight(weight: i32, reps: i32, series: i32) -> i32 {
    weight * reps * series
}

fn main() {
    println!("{}", greets("Joe"));

    let (weight, reps): (i32, i32) = (WEIGHT, REPS);
    println!(
        "Joe can do {} reps of {} kilos on the bench press",
        reps, weight
    );
    let total_weight = calculate_total_weight(weight, reps, 5);
    println!("Joe can do {} kilos on the bench press", total_weight);
}

#[cfg(test)]
mod tests {
    use super::calculate_total_weight;

    #[test]
    fn test_calculate_total_weight() {
        assert_eq!(calculate_total_weight(90, 6, 5), 2700);
    }
}

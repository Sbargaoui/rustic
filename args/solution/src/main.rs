use std::env;

fn greets(name: &str) -> String {
    format!("Hello, {} 🦀 !", name)
}
fn calculate_total_weight(weight: i32, reps: i32, series: i32) -> i32 {
    weight * reps * series
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = args.get(1).expect("Command is required");

    match cmd.as_str() {
        "greets" => {
            let name = args.get(2).expect("Name is required for greetings");
            println!("{}", greets(name));
        }
        "bench" => {
            let weight = args.get(2).expect("Weight is required for bench");
            let reps = args.get(3).expect("Reps is required for bench");
            let series = args.get(4).expect("Series is required for bench");
            println!(
                "Joe can do {} reps of {} kilos on the bench press",
                reps, weight
            );
            let weight: i32 = weight.parse::<i32>().unwrap();
            let reps: i32 = reps.parse::<i32>().unwrap();
            let series: i32 = series.parse::<i32>().unwrap();
            let total_weight = calculate_total_weight(weight, reps, series);
            println!("Joe can do {} kilos on the bench press", total_weight);
        }
        _ => eprintln!("Not supported command"),
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_total_weight;
    use super::greets;

    #[test]
    fn test_greets() {
        assert_eq!(greets("You"), String::from("Hello, You 🦀 !"));
    }

    #[test]
    fn test_calculate_total_weight() {
        assert_eq!(calculate_total_weight(90, 6, 5), 2700);
    }
}

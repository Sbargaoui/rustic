use std::env;

mod bench;
mod greetings;

use bench::calculate_total_weight;
use greetings::greets;

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
                "Sami can do {} reps of {} kilos on the bench press",
                reps, weight
            );
            let weight: i32 = weight.parse::<i32>().unwrap();
            let reps: i32 = reps.parse::<i32>().unwrap();
            let series: i32 = series.parse::<i32>().unwrap();
            let total_weight = calculate_total_weight(weight, reps, series);
            println!("Sami can do {:?} kilos on the bench press", total_weight);
        }
        _ => eprintln!("Not supported command"),
    }
}

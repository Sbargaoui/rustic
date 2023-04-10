mod bench;
mod greetings;

use bench::calculate_total_weight;
use greetings::greets;

use clap::{Parser, Subcommand};

use crate::bench::random_series;

#[derive(Parser)]
#[clap(
    author = "Sami & Aubin",
    version = "1.0.0",
    about = "Rustic cli",
    long_about = None
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greets with name
    Greets {
        /// Name of the person to greet
        #[clap(short, long, value_parser)]
        name: String,
    },
    // Bench with random values
    Bench {
        #[clap(short = 'w', long, value_enum)]
        weight: i32,
        #[clap(short = 'r', long, value_enum)]
        reps: i32,
        // make series optional
        #[clap(short = 's', long, value_enum)]
        series: Option<i32>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Greets { name } => {
            println!("{}", greets(name));
        }
        Commands::Bench {
            weight,
            reps,
            series,
        } => match series {
            // number of series provided
            Some(s) => {
                let total_weight = calculate_total_weight(*weight, *reps, *s);
                println!("Joe can do {:?} kilos on the bench press", total_weight);
            }
            // no series provided
            None => {
                let random = random_series();
                let total_weight = calculate_total_weight(*weight, *reps, random);
                println!("Joe can do {:?} kilos on the bench press", total_weight);
            }
        },
    }
}

pub mod parsing;
pub mod data;

use parsing::args::{Config};
use parsing::parse::{parse_file};

fn main() {
    let config: Config = Config::new();
    match parse_file(&config.dataset) {
        Ok(dataset) => {
            print!("{:5}", "");
            for feature in 0..13 {
                print!("{:>15}", format!("Feature {}", feature));
            }
            println!();
            
            print!("{:5}", "Count");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.counts[feature] as f32);
            }
            println!();

            print!("{:5}", "Mean");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.means[feature]);
            }
            println!();

            print!("{:5}", "Std");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.stds[feature]);
            }
            println!();
            
            print!("{:5}", "Min");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.mins[feature]);
            }
            println!();

            print!("{:5}", "25%");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.quarters[feature]);
            }
            println!();

            print!("{:5}", "50%");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.halfs[feature]);
            }
            println!();

            print!("{:5}", "75%");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.three_quarters[feature]);
            }
            println!();

            print!("{:5}", "Max");
            for feature in 0..13 {
                print!("{:>15.6}", dataset.maxs[feature]);
            }
            println!();
        },
        Err(error) => println!("{}", error)
    }
}

/*
# TODO
- Makefile + rename .rs


*/

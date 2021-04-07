use crate::data::data::{Data, Dataset};

use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::io::BufReader;

// pub const HOGWARTS_HOUSE: usize = 0;
// pub const FIRST_NAME: usize = 1;
// pub const LAST_NAME: usize = 2;
// pub const BIRTHDAY: usize = 3;
// pub const BEST_HAND: usize = 4;

// pub const ARITHMANCY: usize = 0;
// pub const ASTRONOMY: usize = 1;
// pub const HERBOLOGY: usize = 2;
// pub const DEFENSE_AGAINST_THE_DARK_ARTS: usize = 3;
// pub const DIVINATION: usize = 4;
// pub const MUGGLE_STUDIES: usize = 5;
// pub const ANCIENT_RUNES: usize = 6;
// pub const HISTORY_OF_MAGIC: usize = 7;
// pub const TRANSFIGURATION: usize = 8;
// pub const POTIONS: usize = 9;
// pub const CARE_OF_MAGICAL_CREATURES: usize = 10;
// pub const CHARMS: usize = 11;
// pub const FLYING: usize = 12;

pub fn parse_file(dataset_file: &str) -> Result<Dataset, String> {
    if !metadata(dataset_file).expect("error: a problem occured with the file").is_file() {
        return Err(String::from("error: the file should be a file"));
	}
    let file = File::open(dataset_file).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut dataset: Vec<Data> = vec![];
    for line in lines.into_iter().skip(1) {
        if let Ok(content) = line {
            let values: Vec<&str> = content.split(",").collect();
            if values.len() != 19 {
                return Err(String::from("error: bad file format"));
            }
            dataset.push(Data {
                index: values[0].parse::<usize>().expect("error: bad character for index field"),
                infos: values[1..6].iter().map(|field| field.to_string()).collect::<Vec<String>>(),
                grades: values[6..].iter().map(|field| match field.is_empty() {
                    false => Some(field.parse::<f32>().expect("error: bad character for grade field")),
                    _ => None
                }).collect::<Vec<Option<f32>>>()
            });
        }
    }
    return Ok(Dataset::new(dataset));
}
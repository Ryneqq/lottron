use std::{collections::HashMap, fs, path::Path};
use lotto_dto::{Historgram, LottoEntry};
use walkdir::WalkDir;
use std::io;
use itertools::Itertools;

fn load_data(path: impl AsRef<Path>) -> Vec<LottoEntry> {
    read_path(path)
        .flatten()
        .flat_map(|file| file.lines().into_iter().map(ToString::to_string).collect_vec())
        .map(|entry| entry.parse().unwrap())
        .collect()
}

fn read_path(path: impl AsRef<Path>) -> impl Iterator<Item = io::Result<String>> {
   WalkDir::new(path)
        .into_iter()
        .flatten()
        .map(|entry| fs::read_to_string(entry.path()))
}

fn most_common_numbers(entries: Vec<LottoEntry>) -> Vec<(u8, usize)> {
    let mut numbers: HashMap<u8, usize> = (1..50).map(|number| (number, 0)).collect();

    for entry in entries.iter() {
        for num in entry.numbers.iter() {
            numbers.entry(*num).and_modify(|counter| *counter += 1);
        }
    }

    numbers.into_iter().sorted_by(|one, other| other.1.cmp(&one.1)).collect()
}

fn numbers_by_last_occurances(entries: Vec<LottoEntry>) -> Vec<(u8, usize)> {
    let mut numbers: HashMap<u8, usize> = (1..50).map(|number| (number, 0)).collect();

    for entry in entries.iter().sorted_by(|one, other| one.order.cmp(&other.order)) {
        for num in entry.numbers.iter() {
            numbers.entry(*num).and_modify(|order| *order = entry.order);
        }
    }

    numbers.into_iter().sorted_by(|one, other| one.1.cmp(&other.1)).collect()
}

fn main() {
    let data = load_data("data/lotto");

    let last_occurrence = numbers_by_last_occurances(data.clone());

    Historgram::new("Numbers by last occurance", last_occurrence)
        .write_to_file("data/plots/last_occurrence.json")
        .expect("Writing to file");

    let most_common_numbers = most_common_numbers(data);

    Historgram::new("Most commonly apearing numbers in years 2010-2021", most_common_numbers)
        .write_to_file("data/plots/most_common.json")
        .expect("Writing to file");

    // let nums = last_occurrence
    //     .into_iter()
    //     .map(|(num, _)| num)
    //     .take(10)
    //     .chain(
    //         most_common_numbers
    //             .into_iter()
    //             .map(|(num, _)| num)
    //             .take(10)
    //     )
    //     .collect_vec();

    // dbg!(nums);
}

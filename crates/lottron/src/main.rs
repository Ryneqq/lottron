use std::{fs, path::Path};
use lotto_dto::LottoEntry;
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

fn main() {
    let data = load_data("data/");

    let most_common_numbers = data.iter()

    dbg!(data);
}

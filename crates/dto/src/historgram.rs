use std::{fs, path::Path};

use itertools::Itertools;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Historgram {
    pub title: &'static str,
    pub data: Vec<(u8, usize)>
}

impl Historgram {
    pub fn new(title: &'static str, data: impl IntoIterator<Item = (u8, usize)>) -> Self {
        Self {
            title, data: data.into_iter().sorted_by(|a, b| a.0.cmp(&b.0)).collect(),
        }
    }

    pub fn write_to_file(self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;

        fs::write(path, json)
    }
}
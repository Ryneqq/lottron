use std::str::FromStr;

pub type Date = String; // for now its not needed

#[derive(Debug, Clone)]
pub struct LottoEntry {
    pub date: Date,
    pub order: usize,
    pub numbers: Vec<u8>,
}

impl FromStr for LottoEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = s.split(" ");
        let date = items.next().expect("Missing date").to_string();
        let order = items.next()
            .expect("Missing order")
            .split(".")
            .next()
            .expect("Missing order 2")
            .parse::<usize>()
            .expect("Order format is invalid");
        let numbers = items.next()
            .into_iter()
            .flat_map(|nums| nums.split(","))
            .map(|num| num.parse::<u8>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<u8>, _>>()?;

        Ok(LottoEntry { date, order, numbers })
    }
}
use easy_http_request::DefaultHttpRequest;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use rand::Rng;
use scraper::{Html, Selector};
use std::fmt::Debug;
use std::fs;

fn parse_html(document: String) -> Vec<(String, String, Vec<u32>)> {
    let fragment = Html::parse_fragment(&document);
    let selector = Selector::parse("li").unwrap();

    fragment
        .select(&selector)
        .map(|html| html.inner_html().trim().to_string())
        .fold(vec![], |mut acc, num_str| {
            if num_str.contains('.') {
                acc.push((num_str, String::default(), vec![]));
                acc
            } else if num_str.contains('-') {
                let (no, _, numbers) = acc.pop().unwrap();
                acc.push((no, num_str, numbers));
                acc
            } else {
                let (no, date, mut numbers) = acc.pop().unwrap();
                num_str.parse::<u32>().map(|x| numbers.push(x));
                acc.push((no, date, numbers));
                acc
            }
        })
}

fn download_data(year: impl std::fmt::Display) -> Vec<(String, String, Vec<u32>)> {
    let url = format!("http://megalotto.pl/wyniki/lotto/losowania-z-roku-{}", year);
    let response = DefaultHttpRequest::get_from_url_str(&url)
        .unwrap()
        .send()
        .unwrap();
    let html = String::from_utf8(response.body).unwrap();
    let document = Html::parse_document(&html);
    let selector = Selector::parse(".lista_ostatnich_losowan").unwrap();

    document
        .select(&selector)
        .map(|doc| doc.inner_html())
        .flat_map(parse_html)
        .collect()
}

fn save_data(year: impl std::fmt::Display) {
    // let data = download_data()
    //     .chunks(6)
    //     .into_iter()
    //     .map(|chunk| chunk.iter().join(","))
    //     .join("\n");
    let data = download_data(&year)
        .into_iter()
        .map(|(no, date, numbers)| format!("{} {} {}", date, no, numbers.iter().join(",")))
        .join("\n");
    let path = format!("data/lotto_{}.txt", year);

    fs::write(path, data).unwrap();
}

// fn load_data() -> Vec<Vec<u32>> {
//     let parse = |line: &str| -> Vec<u32> {
//         line.split(",")
//             .filter_map(|x| x.parse().ok())
//             .collect_vec()
//     };

//     fs::read_to_string(data_path!())
//         .unwrap()
//         .lines()
//         .map(parse)
//         .collect()
// }

fn distance(data: Vec<u32>) -> Vec<u32> {
    data.into_iter()
        .tuple_windows()
        .filter_map(|(a, b)| if b > a { Some(b - a) } else { None })
        .collect()
}

fn distances(data: Vec<Vec<u32>>) -> Vec<u32> {
    distance(data.concat())
}

fn repetitions(items: Vec<u32>) -> Vec<(u32, u32)> {
    let mut repetitions = items
        .iter()
        .sorted()
        .unique()
        .map(|diff| (*diff, 0))
        .collect_vec();

    for diff in items.iter() {
        repetitions
            .iter_mut()
            .find(|(item, _)| item == diff)
            .map(|(_, rep)| *rep += 1);
    }

    repetitions
}

fn main() {
    // let primes = get_primes(1000000).count();
    // println!("{:?}", primes)
    // let outcomes = (0..100).map(|_| lotto_algorithm().iter().join(","))
    //     .join("\n");

    // println!("{}", outcomes);

    // save_data();
    (2010..=2021).for_each(|year| save_data(year))
    // let data = load_data();
    // let distances = distances(data);
    // let diff_sums = distances.chunks(5)
    //     .into_iter()
    //     .map(|chunk| chunk.iter().sum())
    //     .collect::<Vec<u32>>();

    // println!("{:?}", diff_sums.iter().min());
    // println!("{:?}", diff_sums);

    // let path = format!("charts/distances_{}.png", YEAR);
    // plot!(repetitions(diff_sums), path);

    // let path = format!("charts/lotto_{}.png", YEAR);
    // plot!(repetitions(distances), path);
}

// fn lotto_algorithm() -> Vec<u32> {
//     let data = load_data();
//     let dist = distances(data);
//     let sum = |items: &[u32]| items.into_iter().sum();
//     let diff_sums = dist.chunks(5)
//         .into_iter()
//         .map(sum)
//         .collect::<Vec<u32>>();
//     let precision = 0.1;
//     let skip = diff_sums.len() as f64 * precision;
//     let take = diff_sums.len() as f64 - skip * 2.;
//     let diff_sums = diff_sums.into_iter()
//         .skip(skip as usize)
//         .take(take as usize)
//         .collect_vec();
//     let genrate_weighten_values = |_| generate_outcome(&dist);
//     // let conditions = |nums| outcome_viability(nums, &diff_sums);

//     (0..).map(genrate_weighten_values)
//         .find(|gen| outcome_viability(gen, &diff_sums))
//         .unwrap()
//     // let filter = |dist| dist == distances(nums).into_iter().sum();
//     // let filter = |nums| diff_sums.contains(filter);
//     // let generate = ||

//     // vec![]
// }

fn outcome_viability(generated: &Vec<u32>, diff_sums: &Vec<u32>) -> bool {
    let sum = distance(generated.clone()).into_iter().sum();

    generated.iter().all(|num| *num < 50) && diff_sums.contains(&sum)
}

fn generate_outcome(dist: &Vec<u32>) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let values = dist
        .into_iter()
        .map(|val| *val)
        .choose_multiple(&mut rng, 5);
    let mut number = rng.gen_range(1, 20);

    Some(0)
        .into_iter()
        .chain(values)
        .update(|mut val| {
            *val += number;
            number = *val
        })
        .collect_vec()
}

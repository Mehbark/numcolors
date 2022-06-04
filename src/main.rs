use image::GenericImageView;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Display,
};

fn main() {
    let mut amounts = HashMap::new();

    let paths = env::args().skip(1);
    for path in paths {
        let mut unique_colors = HashSet::new();

        let img = image::open(&path).expect("failed to open path i crie");
        for (_x, _y, color) in img.pixels() {
            unique_colors.insert(color);
        }

        amounts.insert(path, unique_colors.len());
    }

    println!("{}", get_out(amounts));
}

fn get_out(amounts: HashMap<String, usize>) -> impl Display {
    match amounts.len() {
        0 => "".to_string(),
        1 => {
            let (_path, num_colors) = amounts
                .into_iter()
                .next()
                .expect("we just checked that it's not empty");

            num_colors.to_string()
        }
        _ => amounts
            .into_iter()
            .map(|(path, num_colors)| format!("{path}: {num_colors}"))
            .join("\n"),
    }
}

use combination_generator::{Config, CombinationGenerator};

mod combination_generator;

fn main() {
    let config = Config::new(3, 10, 18, 4);
    let mut combination_length_gen = CombinationGenerator::new(config);

    while let Some(combination) = combination_length_gen.next() {
        println!("{:?}", combination);
    }
}

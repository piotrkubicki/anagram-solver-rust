use combination_generator::{Config, CombinationGenerator};

mod combination_generator;

fn main() {
    let config = Config::new(3, 10, 18, 3);
    let mut combination_length_gen = CombinationGenerator::new(config);

    for _ in 1..100 {
        println!("{:?}", combination_length_gen.next());
    }
}

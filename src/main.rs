/// # game_of_life
/// another rust implementation of the Game of Live

use game_of_life::Cells;

fn main() {
    let gens = 10;
    let width = 40;
    let height = 20;

    let cells = Cells::new(width, height);

    for gen in 1..=gens {
        let cells = cells.evolve();
        println!("{}", cells);
        println!("Life - generation {}", gen);
    }
}

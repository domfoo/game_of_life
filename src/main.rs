/// # game_of_life
/// another rust implementation of the Game of Live

use game_of_life::Cells;

fn main() {
    let cells = Cells::new(40, 20);
    println!("{cells}");
}

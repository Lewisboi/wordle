use wordle::wordle_game::game_objects::{Game};

fn main() {
    let mut new_game = Game::default();
    let game_result = new_game.run();
    println!("{:?}", game_result);
}

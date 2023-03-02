use colored::{ColoredString, Colorize};
use std::io::stdin;

pub struct Game<P: Player> {
    word: String,
    number_of_attempts: u8,
    attempts_left: u8,
    board: Board,
    player: P,
}

impl<P: Player> Game<P> {
    pub fn new(word: String, number_of_attempts: u8, player: P) -> Self {
        Self {
            board: Board::new(word.len() as u8, number_of_attempts),
            word: word.to_lowercase(),
            number_of_attempts,
            attempts_left: number_of_attempts,
            player: player,
        }
    }
}

impl<P: Player> Game<P> {
    fn get_diff(&self, player_word: &str) -> Word {
        let mut slots = vec![];
        for i in 0..self.word.len() {
            let player_letter = player_word.chars().nth(i).unwrap();
            if self.word.contains(player_letter) {
                let actual_letter = self.word.chars().nth(i).unwrap();
                if player_letter == actual_letter {
                    slots.push(SlotState::Match(player_letter));
                } else {
                    slots.push(SlotState::PartialMatch(player_letter));
                }
            } else {
                slots.push(SlotState::NonMatch(player_letter));
            }
        }
        Word::Full(slots)
    }

    fn current_word_index(&self) -> usize {
        (self.number_of_attempts - self.attempts_left).into()
    }

    pub fn run(&mut self) -> GameSummary {
        self.board.print();
        while self.attempts_left > 0 {
            let player_word = self.player.get_play(&self.board);
            let diff = self.get_diff(&player_word);
            self.board.add_word(diff, self.current_word_index());
            self.board.print();
            if player_word == self.word {
                break;
            }
            self.attempts_left -= 1;
        }
        GameSummary {
            won: self.attempts_left > 0,
        }
    }
}

impl Default for Game<HumanPlayer> {
    fn default() -> Self {
        Self::new("test".to_owned(), 5, HumanPlayer { word_length: 4 })
    }
}

pub struct Board {
    words: Vec<Word>,
}

impl Board {
    fn new(width: u8, length: u8) -> Self {
        Self {
            words: (0..length).map(|_| Word::Empty(width)).collect(),
        }
    }
    fn add_word(&mut self, word: Word, index: usize) {
        self.words[index] = word;
    }
}

impl Board {
    fn print(&self) {
        for w in &self.words {
            print!("{}", "|".blue());
            w.print();
            print!("{}", "|\n".blue());
        }
    }
}

enum Word {
    Full(Vec<SlotState>),
    Empty(u8),
}

impl Default for Word {
    fn default() -> Self {
        Self::Empty(0)
    }
}

impl Word {
    fn print(&self) {
        match self {
            Self::Empty(n) => print!("{}", "#".repeat(*n as usize).to_owned().blue()),
            Self::Full(v) => {
                for ss in v {
                    print!("{}", ss.to_colored_string());
                }
            }
        }
    }
}

enum SlotState {
    NonMatch(char),
    PartialMatch(char),
    Match(char),
}

impl SlotState {
    fn to_colored_string(&self) -> ColoredString {
        match self {
            Self::Match(n) => n.to_string().green(),
            Self::NonMatch(n) => n.to_string().white(),
            Self::PartialMatch(n) => n.to_string().yellow(),
        }
    }
}

#[derive(Debug)]
pub struct GameSummary {
    won: bool,
}

pub trait Player {
    fn get_play(&self, board: &Board) -> String;
}

pub struct HumanPlayer {
    word_length: u8,
}

impl HumanPlayer {
    fn validate_input(&self, input: &str) -> bool {
        input.len() == self.word_length as usize
    }

    fn get_player_word(&self) -> String {
        loop {
            println!("Insert your guess: ");
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(_) => {
                    if !self.validate_input(buffer.trim()) {
                        println!("{}", "Invalid input, try again".yellow());
                        continue;
                    }
                    break buffer.trim().to_owned();
                }
                Err(_) => {
                    println!(
                        "{}",
                        "There was an error while reading the input, try again".red()
                    );
                    continue;
                }
            }
        }
    }
}

impl Player for HumanPlayer {
    fn get_play(&self, _: &Board) -> String {
        self.get_player_word()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_should_validate_length_ok() {
        let game: Game<HumanPlayer> = Game::default();
        assert!(game.player.validate_input("Test"))
    }

    #[test]
    fn should_not_validate_greater_length() {
        let game: Game<HumanPlayer> = Game::default();
        assert!(!game.player.validate_input("Hello World"))
    }

    #[test]
    fn should_not_validate_less_than() {
        let game: Game<HumanPlayer> = Game::default();
        assert!(!game.player.validate_input("Foo"))
    }
}

use rand::distributions::Standard;
use rand::prelude::Distribution;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Winner {
    Human,
    Computer,
    Draw,
}

impl Display for Winner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Winner::Human => "Human",
                Winner::Computer => "Computer",
                Winner::Draw => "Draw",
            }
        )
    }
}

#[derive(Debug)]
pub struct BestOf(u8);

impl BestOf {
    pub fn new(number: u8) -> Result<Self, &'static str> {
        if (number % 2 != 0) && (number > 2) {
            Ok(Self(number))
        } else {
            Err("Number must be odd and greater than 2")
        }
    }
}

impl Default for BestOf {
    fn default() -> Self {
        Self(5)
    }
}

impl FromStr for BestOf {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>() {
            Ok(value) => BestOf::new(value),
            Err(_) => Err("Could not parse number"),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    human_points: u8,
    computer_points: u8,
    round: u8,
    best_of: BestOf,
}

impl Game {
    pub fn new(best_of: Option<BestOf>) -> Self {
        Self {
            human_points: 0,
            computer_points: 0,
            round: 1,
            best_of: match best_of {
                Some(value) => value,
                None => BestOf::default(),
            },
        }
    }

    pub fn add_point(&mut self, player: &Winner) {
        match player {
            Winner::Human => self.human_points += 1,
            Winner::Computer => self.computer_points += 1,
            Winner::Draw => (),
        }
    }

    pub fn round(&self) -> u8 {
        self.round
    }

    pub fn increase_round(&mut self) {
        self.round += 1
    }

    pub fn human_points(&self) -> u8 {
        self.human_points
    }

    pub fn computer_points(&self) -> u8 {
        self.computer_points
    }

    pub fn best_of(&self) -> u8 {
        self.best_of.0
    }

    pub fn round_winner(&self, human_choice: &Choice, computer_choice: &Choice) -> Winner {
        let result = human_choice.partial_cmp(computer_choice).unwrap();
        match result {
            Ordering::Greater => Winner::Human,
            Ordering::Less => Winner::Computer,
            Ordering::Equal => Winner::Draw,
        }
    }

    pub fn game_winner(&self) -> Winner {
        if self.computer_points > self.human_points {
            Winner::Computer
        } else if self.computer_points < self.human_points {
            Winner::Human
        } else {
            Winner::Draw
        }
    }

    pub fn enough_points_to_end_game(&self) -> bool {
        let minimum_round = (self.best_of() / 2) + 1;
        if (self.human_points == minimum_round) | (self.computer_points == minimum_round) {
            return true
        }
        false
    }
}

#[derive(Debug, PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Distribution<Choice> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        match rng.gen_range(0..=2) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            _ => Choice::Scissors,
        }
    }
}

impl TryFrom<String> for Choice {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "rock\n" | "r\n" => Ok(Self::Rock),
            "paper\n" | "p\n" => Ok(Self::Paper),
            "scissors\n" | "s\n" => Ok(Self::Scissors),
            _ => Err("Unknown choice"),
        }
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Choice::Paper => "Paper",
                Choice::Rock => "Rock",
                Choice::Scissors => "Scissors",
            }
        )
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Rock, &Choice::Paper) => Some(Ordering::Less),
            (Self::Rock, &Choice::Scissors) => Some(Ordering::Greater),
            (Self::Paper, &Choice::Rock) => Some(Ordering::Greater),
            (Self::Paper, &Choice::Scissors) => Some(Ordering::Less),
            (Self::Scissors, &Choice::Paper) => Some(Ordering::Greater),
            (Self::Scissors, &Choice::Rock) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_beats_scissors() {
        let rock = Choice::Rock;
        let scissors = Choice::Scissors;
        assert!(rock > scissors)
    }

    #[test]
    fn paper_beats_rock() {
        let rock = Choice::Rock;
        let paper = Choice::Paper;
        assert!(paper > rock)
    }

    #[test]
    fn scissors_beats_paper() {
        let scissors = Choice::Scissors;
        let paper = Choice::Paper;
        assert!(scissors > paper)
    }

    #[test]
    fn rock_loses_to_paper() {
        let rock = Choice::Rock;
        let paper = Choice::Paper;
        assert!(rock < paper)
    }

    #[test]
    fn paper_loses_to_scissors() {
        let scissors = Choice::Scissors;
        let paper = Choice::Paper;
        assert!(paper < scissors)
    }

    #[test]
    fn scissors_loses_to_rock() {
        let scissors = Choice::Scissors;
        let rock = Choice::Rock;
        assert!(scissors < rock)
    }

    #[test]
    fn same_choice_is_equal() {
        let scissors = Choice::Scissors;
        let paper = Choice::Paper;
        let rock = Choice::Rock;

        assert!(scissors == scissors);
        assert!(paper == paper);
        assert!(rock == rock);
    }

    #[test]
    fn human_gets_point() {
        let mut game = Game::new(None);
        game.add_point(&Winner::Human);
        assert_eq!(game.human_points(), 1);
    }

    #[test]
    fn computer_gets_point() {
        let mut game = Game::new(None);
        game.add_point(&Winner::Computer);
        assert_eq!(game.computer_points(), 1);
    }

    #[test]
    fn should_chose_round_winner() {
        let game = Game::new(None);
        let human_choice = Choice::Paper;
        let computer_choice = Choice::Rock;

        assert_eq!(
            game.round_winner(&human_choice, &computer_choice),
            Winner::Human
        );
    }

    #[test]
    fn if_points_are_equal_game_is_drawn() {
        let mut game = Game::new(None);
        game.add_point(&Winner::Computer);
        game.add_point(&Winner::Computer);
        game.add_point(&Winner::Human);
        game.add_point(&Winner::Human);
        assert_eq!(game.game_winner(), Winner::Draw)
    }

    #[test]
    #[should_panic]
    fn on_even_numbers() {
        BestOf::new(6).unwrap();
    }

    #[test]
    #[should_panic]
    fn on_number_less_than_3() {
        BestOf::new(2).unwrap();
    }

    #[test]
    fn should_stop_when_other_player_cant_win_anymore() {
        let mut game = Game::new(Some(BestOf::default()));
        game.add_point(&Winner::Computer);
        game.add_point(&Winner::Computer);
        game.add_point(&Winner::Computer);
        assert!(game.enough_points_to_end_game());
    }

    #[test]
    fn false_when_not_enough_points_to_end_game_early() {
        let mut game = Game::new(Some(BestOf::default()));
        game.add_point(&Winner::Computer);
        game.add_point(&Winner::Computer);
        game.add_point(&Winner::Human);
        game.add_point(&Winner::Human);
        assert!(!game.enough_points_to_end_game());
    }
}

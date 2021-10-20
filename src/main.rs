use prettytable::{cell, row, Table};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Display;
use std::io;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rock-Paper-Scissors",
    about = "Simple rock-paper-scissors game with nice output formatting"
)]
struct Opt {
    /// Number of rounds to be played. 3, 5 or 7 are available.
    #[structopt(short = "r", long = "rounds")]
    best_of: Option<BestOf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    println!();

    let mut game = Game::new(opt.best_of);

    let mut table = Table::new();

    println!("Welcome to the ROCK - PAPER - SCISSORS game");
    println!("Type 'Scissors(s)', 'Rock(r)' or 'Paper(p)' to select your option");
    println!("Playing total of {} rounds", game.best_of() as u8);
    println!();

    for _ in 0..game.best_of() as u8 {
        let mut human_choice = String::new();
        io::stdin().read_line(&mut human_choice)?;
        let human_choice = Choice::try_from(human_choice)? as Choice;

        let computer_choice: Choice = rand::random();

        println!(
            "{}. Your choice: {}, Computer choice: {}",
            game.round(),
            human_choice,
            computer_choice
        );

        let (winner, round_row) = match game.round_winner(&human_choice, &computer_choice) {
            Winner::Human => (
                Winner::Human,
                row![c -> format!("{}", game.round()), BgFdc -> human_choice, BrFdc -> computer_choice],
            ),
            Winner::Computer => (
                Winner::Computer,
                row![c -> format!("{}", game.round()), BrFdc -> human_choice, BgFdc -> computer_choice],
            ),
            Winner::Draw => (
                Winner::Draw,
                row![c -> format!("{}", game.round()), ByFdc -> human_choice, ByFdc -> computer_choice],
            ),
        };

        game.add_point(&winner);
        table.add_row(round_row);
        game.increase_round();
    }

    println!();
    table.insert_row(0, row![c => "Round", "Player", "Computer"]);
    table.add_row(row![c => "Total", game.human_points(), game.computer_points()]);
    table.add_row(row![H1c -> "Winner", H2c -> format!("{}", game.game_winner())]);
    table.printstd();

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Winner {
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

#[derive(Debug, Clone, Copy)]
enum BestOf {
    Three = 3,
    Five = 5,
    Seven = 7,
}

impl FromStr for BestOf {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "3" => Ok(Self::Three),
            "5" => Ok(Self::Five),
            "7" => Ok(Self::Seven),
            _ => Err("Could not parse given value. You must choose between 3, 5 and 7"),
        }
    }
}

impl Default for BestOf {
    fn default() -> Self {
        Self::Five
    }
}

struct Game {
    human_points: u8,
    computer_points: u8,
    round: u8,
    best_of: BestOf,
}

impl Game {
    fn new(best_of: Option<BestOf>) -> Self {
        Self {
            human_points: 0,
            computer_points: 0,
            round: 1,
            best_of: match best_of {
                Some(value) => value,
                _ => BestOf::default(),
            },
        }
    }

    fn add_point(&mut self, player: &Winner) {
        match player {
            Winner::Human => self.human_points += 1,
            Winner::Computer => self.computer_points += 1,
            Winner::Draw => (),
        }
    }

    fn round(&self) -> u8 {
        self.round
    }

    fn increase_round(&mut self) {
        self.round += 1
    }

    fn human_points(&self) -> u8 {
        self.human_points
    }

    fn computer_points(&self) -> u8 {
        self.computer_points
    }

    fn best_of(&self) -> BestOf {
        self.best_of
    }

    fn round_winner(&self, human_choice: &Choice, computer_choice: &Choice) -> Winner {
        let result = human_choice.partial_cmp(computer_choice).unwrap();
        match result {
            Ordering::Greater => Winner::Human,
            Ordering::Less => Winner::Computer,
            Ordering::Equal => Winner::Draw,
        }
    }

    fn game_winner(&self) -> Winner {
        if self.computer_points > self.human_points {
            Winner::Computer
        } else if self.computer_points < self.human_points {
            Winner::Human
        } else {
            Winner::Draw
        }
    }
}

#[derive(Debug, PartialEq)]
enum Choice {
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
}

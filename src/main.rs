use prettytable::{cell, row, Table};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::io;
use structopt::StructOpt;

extern crate rock_paper_scissors as rps;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rock-Paper-Scissors",
    about = "Simple rock-paper-scissors game with nice output formatting"
)]
struct Opt {
    /// Number of rounds to be played. 3, 5 or 7 are available.
    #[structopt(short = "r", long = "rounds")]
    best_of: Option<rps::BestOf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    println!();

    let mut game = rps::Game::new(opt.best_of);

    let mut table = Table::new();

    println!("Welcome to the ROCK - PAPER - SCISSORS game");
    println!("Type 'Scissors(s)', 'Rock(r)' or 'Paper(p)' to select your option");
    println!("Playing best of {} rounds", game.best_of());
    println!();

    for _ in 0..game.best_of() {
        let mut human_choice = String::new();
        io::stdin().read_line(&mut human_choice)?;
        let human_choice = rps::Choice::try_from(human_choice)? as rps::Choice;

        let computer_choice: rps::Choice = rand::random();

        println!(
            "{}. Your choice: {}, Computer choice: {}",
            game.round(),
            human_choice,
            computer_choice
        );

        let winner = game.round_winner(&human_choice, &computer_choice);
        let round_row = match winner {
            rps::Winner::Human => {
                row![c -> format!("{}", game.round()), BgFdc -> human_choice, BrFdc -> computer_choice]
            }
            rps::Winner::Computer => {
                row![c -> format!("{}", game.round()), BrFdc -> human_choice, BgFdc -> computer_choice]
            }
            rps::Winner::Draw => {
                row![c -> format!("{}", game.round()), ByFdc -> human_choice, ByFdc -> computer_choice]
            }
        };

        game.add_point(&winner);
        table.add_row(round_row);
        game.increase_round();

        if game.enough_points_to_end_game() {
            break
        }
    }

    println!();
    table.insert_row(0, row![c => "Round", "Player", "Computer"]);
    table.add_row(row![c => "Total", game.human_points(), game.computer_points()]);
    table.add_row(row![H1c -> "Winner", H2cb -> format!("{}", game.game_winner())]);
    table.printstd();

    Ok(())
}


extern crate rand;

use std::io;
use rand::Rng;

static mut _wins: i32 = 0;
static mut _losses: i32 = 0;
static mut _ties: i32 = 0;
static mut _rocks: i32 = 0;
static mut _papers: i32 = 0;
static mut _scissors: i32 = 0;

enum input {
	Rock,
	Paper,
	Scissors,
	Quit,
	Error,
}

enum outcome {
	Win,
	Loss,
	Tie,
	Error,
	Quit,
}

struct round {
	player: input,
	cpu: input,
}

impl round {

	fn winner(&self) -> outcome{
		match self.player {
			input::Rock => match self.cpu {
				input::Rock => outcome::Tie,
				input::Scissors => outcome::Win,
				input::Paper => outcome::Loss,
				_ => outcome::Error,
			},
			input::Paper => match self.cpu {
				input::Rock => outcome::Win,
				input::Scissors => outcome::Loss,
				input::Paper => outcome::Tie,
				_ => outcome::Error,
			},
			input::Scissors => match self.cpu {
				input::Rock => outcome::Loss,
				input::Scissors => outcome::Tie,
				input::Paper => outcome::Win,
				_ => outcome::Error,
			},
			input::Quit => outcome::Quit,
			input::Error => outcome::Error,
		}	
	}
}

fn print_round( this_round: &round) {
	print!("Player chose: ");
	let player_move = move_to_text(&this_round.player).to_string();
	println!("{}", player_move);
	
	let cpu_move = move_to_text(&this_round.cpu).to_string();
	println!("Opponent chose: {}", cpu_move);
	
}

fn move_to_text( mv: &input ) -> &str{
	match mv {
		&input::Rock => "Rock",
		&input::Paper => "Paper",
		&input::Scissors => "Scissors",
		_ => "",
	}
}


fn main() {
	println!("Hello, world!");
	
	// main loop
	loop {
		let current_round = round{player: get_move(), cpu: generate_move()};
		match track_stats(&current_round){
			outcome::Win => {print_round(&current_round); println!("You win!")},
			outcome::Loss => {print_round(&current_round); println!("You lose!")},
			outcome::Tie => {print_round(&current_round); println!("It's a tie!")},
			outcome::Error => {println!("Please enter r, p, s, or q!!!!!"),
			outcome::Quit => display_stats(),
		}
	}
}

fn get_move() -> input{
	println!("Enter choice (r,p,s) or q to quit >");
	
	let mut player_move = String::new();
	
	
	io::stdin().read_line(&mut player_move)
		.expect("Failed to read line");
		
	match player_move.trim().as_ref() {
		"r" => input::Rock,
		"p" => input::Paper,
		"s" => input::Scissors,
		"q" => input::Quit,
		_ => input::Error,
	}
		
	//println!("Player chose {}", player_move);

}

fn generate_move() -> input{
	let rand_num = rand::thread_rng().gen_range(0,3);
	match rand_num {
		0 => input::Rock,
		1 => input::Paper,
		_ => input::Scissors,
	}
	
}

fn track_stats(this_round: &round) -> outcome{
	let round_outcome = this_round.winner();
	unsafe{
		match round_outcome {
			outcome::Win => _wins = _wins + 1,
			outcome::Loss => _losses = _losses + 1,
			outcome::Tie => _ties = _ties + 1,
			outcome::Quit => return round_outcome,
			outcome::Error => return outcome::Error,
		}
		match this_round.player {
			input::Rock => _rocks = _rocks + 1,
			input::Scissors => _scissors = _scissors + 1,
			input::Paper => _papers = _papers + 1,
			_ => return outcome::Error,
		}
	}
	round_outcome
	
}

fn display_stats(){

}
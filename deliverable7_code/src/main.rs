extern crate rand;

use std::io;
use rand::Rng;

static mut _wins: f32 = 0.0;
static mut _losses: f32 = 0.0;
static mut _ties: f32 = 0.0;
static mut _rocks: f32 = 0.0;
static mut _papers: f32 = 0.0;
static mut _scissors: f32 = 0.0;

enum Input {
	Rock,
	Paper,
	Scissors,
	Quit,
	Error,
}

enum Outcome {
	Win,
	Loss,
	Tie,
	Error,
	Quit,
}

struct Round {
	player: Input,
	cpu: Input,
}

impl Round {

	fn winner(&self) -> Outcome{
		match self.player {
			Input::Rock => match self.cpu {
				Input::Rock => Outcome::Tie,
				Input::Scissors => Outcome::Win,
				Input::Paper => Outcome::Loss,
				_ => Outcome::Error,
			},
			Input::Paper => match self.cpu {
				Input::Rock => Outcome::Win,
				Input::Scissors => Outcome::Loss,
				Input::Paper => Outcome::Tie,
				_ => Outcome::Error,
			},
			Input::Scissors => match self.cpu {
				Input::Rock => Outcome::Loss,
				Input::Scissors => Outcome::Tie,
				Input::Paper => Outcome::Win,
				_ => Outcome::Error,
			},
			Input::Quit => Outcome::Quit,
			Input::Error => Outcome::Error,
		}	
	}
}

fn print_round( this_round: &Round) {
	print!("Player chose: ");
	let player_move = move_to_text(&this_round.player).to_string();
	println!("{}", player_move);
	
	let cpu_move = move_to_text(&this_round.cpu).to_string();
	println!("Opponent chose: {}", cpu_move);
	
}

fn move_to_text( mv: &Input ) -> &str{
	match mv {
		&Input::Rock => "Rock",
		&Input::Paper => "Paper",
		&Input::Scissors => "Scissors",
		_ => "",
	}
}


fn main() {
	// main loop
	loop {
		let current_round = Round{player: get_move(), cpu: generate_move()};
		match track_stats(&current_round){
			Outcome::Win => {print_round(&current_round); println!("You win!")},
			Outcome::Loss => {print_round(&current_round); println!("You lose!")},
			Outcome::Tie => {print_round(&current_round); println!("It's a tie!")},
			Outcome::Error => {println!("Please enter r, p, s, or q!!!!!")},
			Outcome::Quit => {display_stats(); break},
		}
	}
}

fn get_move() -> Input{
	println!("Enter choice (r,p,s) or q to quit >");
	
	let mut player_move = String::new();
	
	
	io::stdin().read_line(&mut player_move)
		.expect("Failed to read line");
		
	match player_move.trim().as_ref() {
		"r" => Input::Rock,
		"p" => Input::Paper,
		"s" => Input::Scissors,
		"q" => Input::Quit,
		_ => Input::Error,
	}
		
	//println!("Player chose {}", player_move);

}

fn generate_move() -> Input{
	let rand_num = rand::thread_rng().gen_range(0,3);
	match rand_num {
		0 => Input::Rock,
		1 => Input::Paper,
		_ => Input::Scissors,
	}
	
}

fn track_stats(this_round: &Round) -> Outcome{
	let round_outcome = this_round.winner();
	unsafe{
		match round_outcome {
			Outcome::Win => _wins = _wins + 1.0,
			Outcome::Loss => _losses = _losses + 1.0,
			Outcome::Tie => _ties = _ties + 1.0,
			Outcome::Quit => return round_outcome,
			Outcome::Error => return Outcome::Error,
		}
		match this_round.player {
			Input::Rock => _rocks = _rocks + 1.0,
			Input::Scissors => _scissors = _scissors + 1.0,
			Input::Paper => _papers = _papers + 1.0,
			_ => return Outcome::Error,
		}
	}
	round_outcome
	
}

fn display_stats(){
	println!("Player Stats:");
	unsafe{
		let total_games: f32 = _wins + _losses + _ties;
		let mut win_percentage: f32 = (_wins / total_games * 100.0).round() / 100.0;
		
		let mut loss_percentage: f32 = (_losses / total_games * 100.0).round() / 100.0;
		
		let mut tie_percentage: f32 = (_ties / total_games * 100.0).round() / 100.0;
		
		if win_percentage.is_finite() != true {
			win_percentage = 0.0;
		}
		
		if tie_percentage.is_finite() != true {
			tie_percentage = 0.0;
		}
		
		if loss_percentage.is_finite() != true {
			loss_percentage = 0.0;
		}
	
	
		println!("Wins: {} ({}%)", _wins, win_percentage);
		println!("Ties: {} ({}%)", _ties, tie_percentage);
		println!("Losses: {} ({}%)", _losses, loss_percentage);
		println!("Rocks: {}", _rocks);
		println!("Papers: {}", _papers);
		println!("Scissors: {}", _scissors);
	
	}
}
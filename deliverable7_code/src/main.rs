extern crate rand;

use std::io;
use rand::Rng;

enum input {
	Rock,
	Paper,
	Scissors,
	Quit,
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
		}	
	}
}




fn main() {
	println!("Hello, world!");
	
	// main loop
	loop {
		get_move();
		generate_move();
	}
}

fn get_move(){
	println!("Enter choice (r,p,s) or q to quit >");
	
	let mut player_move = String::new();
	
	io::stdin().read_line(&mut player_move)
		.expect("Failed to read line");
		
	//player_move = player_move.trim();
		
	println!("Player chose {}", player_move);

}

fn generate_move() -> input{
	let rand_num = rand::thread_rng().gen_range(0,3);
	match rand_num {
		0 => input::Rock,
		1 => input::Paper,
		_ => input::Scissors,
	}
	
}

fn do_calculations(p_move: String, c_move: String){
	
}

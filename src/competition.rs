use super::neural_network_game::*;

use std::time::Instant;
use rayon::prelude::*;

const NUMBER_OF_ROUNDS: usize = 2;

pub fn run_competition()
{
	//---- Spawn Neural Networks
	// Keep track of the time so we can record how long everything took.
    let start = Instant::now();

	// Work out how many games will be in the first round and create players for each game.
	let first_round_games = (NUMBER_OF_PLAYERS as i32).pow((NUMBER_OF_ROUNDS - 1) as u32);

	// Create a new vector to hold the neural networks.
	let mut nn = create_neural_networks(first_round_games as usize * NUMBER_OF_PLAYERS);

	// Finished creating neural networks.
	println!("Created {0} in {1}", nn.len(), sec_from_time(start));
	let competition_started = Instant::now();

	//---- Start a new round of the tournament
	for i in (0..(NUMBER_OF_ROUNDS)).rev()
	{
		let round_started = Instant::now();
		 
		// Calculate how many games we need to play.
		let number_of_games = (NUMBER_OF_PLAYERS as i32).pow(i as u32);

		let mut games = Vec::new();
		//---- Play out the round and get an array of the winners
		create_and_play_games_parallel(&mut games, number_of_games as usize, &mut nn);

		//---- Round Finished
		println!("Finished round {0} in {1}", NUMBER_OF_ROUNDS - i, sec_from_time(round_started));

		//---- Collect winners
		let winners = games.par_iter().map(|game| game.get_winning_nn())
		.collect();

		// Reset the neural networks we are using to just use the winners.
		nn = winners;
	}
	//---- Complete!
	println!("The winner is {0} total time was {1}", nn[0].nn.get_id(), sec_from_time(competition_started));

	println!("The winner is {0} total time was {1}", nn[0].nn.get_id(), sec_from_time(competition_started));

	let s = format!("winner_of_{0}_competition_{1}", NUMBER_OF_ROUNDS, nn[0].nn.get_id());

	nn[0].nn.save_nn_to_file(s);
}
use super::neural_network_game::*;
use super::neural_network_game::neural_network::*;

use std::time::Instant;
use rand::Rng;

pub fn run_generational(games: usize, generations: usize)
{
	//---- Spawn Neural Networks
	// Keep track of the time so we can record how long everything took.
    let start = Instant::now();

	// Create a new vector to hold the neural networks.
	let mut nn = create_neural_networks(games * NUMBER_OF_PLAYERS);

	// Finished creating neural networks.
	println!("Created {0} in {1}", nn.len(), sec_from_time(start));
	let competition_started = Instant::now();

	//---- Start a new round of the tournament
	for _i in 0..generations
	{
		//let round_started = Instant::now();
		
		//---- Play out the round and get an array of the winners
		let sushi_go_games = create_and_play_games_parallel(games, &mut nn, NUMBER_OF_PLAYERS, NeuralNetworkGameType::Create);

		//---- Round Finished
		//println!("Finished generation {0} in {1}", i, sec_from_time(round_started));

		// Reset the neural networks we are using to just use the winners.
		nn = next_generation(sushi_go_games);
	}
	//---- Complete!
	println!("The winner is {0} total time was {1}", nn[0].get_id(), sec_from_time(competition_started));

	println!("Generations per second: {}", generations as f64 / sec_from_time(competition_started));

	println!("Games per second: {}", (generations as f64 / sec_from_time(competition_started)) * games as f64);

	let s = format!("{0}_games_in_{1}_generations_{2}", games, generations, nn[0].get_id());

	nn[0].save_nn_to_file(s);
}

fn next_generation(games: Vec<NeuralNetworkGame>) -> Vec<NeuralNetwork>
{
	//println!("Generating next generation:");
	let total_score: f32 = games.iter().map(|game| game.get_game().get_winning_score() as f32)
	.sum();

	let scores: Vec<f32> = games.iter().map(|game| game.get_game().get_winning_score() as f32)
	.collect();

	let mut winners: Vec<NeuralNetwork> = games.iter().map(|game| game.get_winning_nn())
		.collect();

	let mut fitness = Vec::new();

	//println!("Previous generation average score:{}", total_score / scores.len() as f32);

	let mut max_score = 0.0;

	for x in scores
	{
		fitness.push(x / total_score);

		max_score = if x > max_score {x} else {max_score};
	}

	//println!("Maximum score was: {}\n", max_score);

	let mut new_nn = Vec::new();

	let chosen = winners.remove(pick_one(&fitness));

	//print_nn_info(&chosen);

	for _i in 0..games.len() * NUMBER_OF_PLAYERS
	{
		let mut mutated_nn = chosen.clone();
		mutated_nn.mutate();
		new_nn.push(mutated_nn);
	}

	return new_nn;
}

fn pick_one(fitness: &Vec<f32>) -> usize
{
	let mut index = 0;
	let mut r: f32 = rand::thread_rng().gen();

	while r > 0.0
	{
		if index < fitness.len()
		{
			r = r - fitness[index];
			index += 1;
		}
		else
		{
			break;
		}
	}

	index -= 1;

	return index;
}

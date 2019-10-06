pub mod neural_network;
pub mod game;

use neural_network::*;

use game::*;
use game::player::card::*;

use rayon::prelude::*;

use std::time::Instant;

use std::io;

mod model_c;
use model_c::*;
use rand::Rng;

//---- Constants ----
pub const NUMBER_OF_PLAYERS: usize = 4;

#[derive(Copy,Clone)]
pub enum NeuralNetworkGameType
{
	Create,
	Play,
	Random
}

impl NeuralNetworkGameType {
	pub fn is_random(&self) -> bool {
		match *self {
			NeuralNetworkGameType::Random => true,
			_ => false,
		}
	}
}

// This is a struct which links a game to a set of neural network players.
pub struct NeuralNetworkGame
{
	game: SushiGoGame,
	nn: Vec<NeuralNetwork>,
	game_type: NeuralNetworkGameType,
}

impl NeuralNetworkGame
{
	pub fn play(&mut self)
	{
		// Setup the game and return the initial game state.
		self.game.setup();

		let mut result = StepResult::NoResult;

		// Keep stepping through the game until we have a winner.
		while !result.is_game_over()
		{
			result = self.game.step(self.take_turn(result));
		}
		
		//---- Game Finished
	}

	fn take_turn(&self, prev_result: StepResult) -> Card
	{
		// If we have enough neural networks have them take a turn.
		if self.game.get_current_player_id() < self.nn.len()
		{
			return pick_cards(&self.game, &self.nn[self.game.get_current_player_id()]);
		}
		else
		{
			if self.game_type.is_random()
			{
				return do_random_turn(&self.game);
			}
			else
			{
				return do_player_turn(&self.game, prev_result);
			}
		}
	}

	pub fn get_winning_nn(&self) -> NeuralNetwork
	{
		self.nn[self.game.get_winner()].clone()
	}

	pub fn get_winning_id(&self) -> usize
	{
		self.game.get_winner()
	}

	pub fn get_nn(&self) -> &Vec<NeuralNetwork>
	{
		&self.nn
	}

	pub fn get_game(&self) -> &SushiGoGame
	{
		&self.game
	}
}

pub fn play_player_game()
{
	// Create a new game and fill it with loaded nn's.
	let new_game = SushiGoGame::new(NUMBER_OF_PLAYERS);
	let mut new_game_nn = Vec::new();

	let mut in_file_name = String::new();

	println!("Enter name of neural network to load:");

	io::stdin().read_line(&mut in_file_name)
		.expect("Failed to read line");

	// Pull the players from the pool of neural networks.
	for _j in 0..(NUMBER_OF_PLAYERS - 1)
	{
		new_game_nn.push(load_nn_from_file(in_file_name.trim()));
	}

	let mut new_game = NeuralNetworkGame{game: new_game, nn: new_game_nn, game_type: NeuralNetworkGameType::Play };

	new_game.play();

	// Game is finished, print some final info.
	new_game.get_game().print_prev_round_results();
	new_game.get_game().print_pudding_scores();
	new_game.get_game().print_final_scores();
}

fn get_usize_from_player_input(message: &str) -> usize
{
	println!("{}", message);

	let mut line_in = String::new();

	io::stdin().read_line(&mut line_in)
		.expect("Failed to read line");

	let mut usize_out = 0;

	match line_in.trim().parse::<usize>()
	{
		Ok(n) => usize_out = n,
		Err(_e) => println!("That was an invalid choice. Dealing with user error is not currently implemented."),
	}

	usize_out
}

pub fn setup_random_game()
{
	let mut random_players = get_usize_from_player_input("Enter how many players will be random (0 - 4):");

	if random_players > NUMBER_OF_PLAYERS
	{
		random_players = NUMBER_OF_PLAYERS;
	}

	let mut in_file_name = String::new();

	if random_players < NUMBER_OF_PLAYERS
	{
		println!("Enter name of neural network to load:");

		io::stdin().read_line(&mut in_file_name)
			.expect("Failed to read line");
	}

	let number_of_games = get_usize_from_player_input("Enter number of games to play:");

	let batch_count = get_usize_from_player_input("Enter number of batches to do, this will reduce memory overhead:");

	play_random_game(batch_count, number_of_games, in_file_name, random_players);
}

// Plays three neural networks against an AI picking random choices.
// Plays number_of_games games per batch. Batches is used to reduce memory usage.
pub fn play_random_game(batches: usize, number_of_games: usize, in_file_name: String, number_of_random_players: usize)
{
	let random_started = Instant::now();

	let mut win_counter: [usize; NUMBER_OF_PLAYERS] = [0; NUMBER_OF_PLAYERS];

	for j in 0..batches
	{
		print!("Starting batch: {}\n", j);
		let batch_started = Instant::now();

		let mut new_game_nn = Vec::new();

		let number_of_neural_networks = NUMBER_OF_PLAYERS - number_of_random_players;

		// Pull the players from the pool of neural networks.
		for _j in 0..(number_of_neural_networks * number_of_games)
		{
			new_game_nn.push(load_nn_from_file(in_file_name.trim()));
		}

		print!("Created {} for batch in {}\n", new_game_nn.len(), sec_from_time(batch_started));

		let games = create_and_play_games_parallel(number_of_games, &mut new_game_nn ,  number_of_neural_networks, NeuralNetworkGameType::Random);

		for x in &games
		{
			//print!("Winner is {}\n", x.get_winning_id());
			win_counter[x.get_winning_id()] += 1;
		}

		print!("Batch finished, total time: {}\n", sec_from_time(batch_started));
	}

	let total_games = number_of_games * batches;

	for i in 0..NUMBER_OF_PLAYERS
	{
		let percentage_wins: f32 = (win_counter[i] as f32 / total_games  as f32) * 100.0;
		print!("Player {} won {}% of games\n", i, percentage_wins);
	}

	println!("\nGames per second: {}\n", total_games as f64 / sec_from_time(random_started));
	println!("Average batch time {}\n", sec_from_time(random_started) / batches as f64);
}

// Creates the number of games required for a round and returns the id's of the winning neural networks.
pub fn create_and_play_games_parallel(number_of_games: usize,
									  nn: &mut Vec<NeuralNetwork>,
									  number_of_neural_networks: usize,
									  in_game_type: NeuralNetworkGameType) -> Vec<NeuralNetworkGame>
{
	let mut games: Vec<NeuralNetworkGame> = Vec::new();

	// Add the number of games we need.
	for _j in 0..number_of_games
	{
		// Create a new game and a vector to hold the neural network players.
		let new_game = SushiGoGame::new(NUMBER_OF_PLAYERS);
		let mut new_game_nn = Vec::new();

		// Pull the players from the pool of neural networks.
		for _j in 0..number_of_neural_networks
		{
			// Check there are nn's in the input array.
			if nn.len() == 0
			{
				assert!(false, "Ran out of nn players!");
			}

			// Add the nn to the list of nn's to be used in this game.
			new_game_nn.push(nn.remove(0));
		}

		// Add the game to the list of games in this round.
		games.push(NeuralNetworkGame{game: new_game, nn: new_game_nn, game_type: in_game_type});
	}

	//---- Play the games in this round and find the winners.
	play_games_parallel(&mut games);

	games
}

fn play_games(games: &mut Vec<NeuralNetworkGame>)
{
	//---- Play the games in this round
    games.iter_mut().for_each(|game| game.play());
}

fn play_games_parallel(games: &mut Vec<NeuralNetworkGame>)
{
	//---- Play the games in this round, but in parallel :O
	games.par_iter_mut().for_each(|game| game.play());
}

pub fn sec_from_time(time: std::time::Instant) -> f64
{
	let elapsed = time.elapsed();
	return (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
}

pub fn create_neural_networks(number: usize) -> Vec<NeuralNetwork>
{
	let mut nn = Vec::new();

	let node_count = get_node_count();

	for i in 0..number	
	{
		nn.push(neural_network::new_neural_network(i as usize, node_count.input_size, node_count.hidden_layer_size, node_count.output_layer_size));
	}

	nn
}

fn do_player_turn(game: &SushiGoGame, prev_result: StepResult) -> Card
{
	print!("The result for the previous round was {}\n", prev_result);

	// Print previous round information.
	if prev_result.is_round_over()
	{
		game.print_prev_round_results();
	}

	println!("\nPrinting Current Game State:");

	for i in 0..game.get_num_players()
	{
		println!("\nChosen Cards for Player {0}", i);

		let mut visible_cards: Vec<Card> = Vec::new();
		for (i, x) in game.get_player_chosen(i).iter().enumerate()
		{
			if i < game.get_player_chosen(game.get_current_player_id()).len()
			{
				visible_cards.push(*x);
			}
		}
		print_cards(&visible_cards);
	}

	let current_hand = game.get_current_player().get_hand();

	println!("\nYour chosen cards:");
	print_cards(&game.get_current_player().get_chosen_cards());

	println!("\nYour hand:");
	print_cards(&current_hand);

	if prev_result.is_chopsticks_available()
	{
		println!("You have the option to pick another card to replace your chopsticks:\n");
	}

	println!("Enter the card you want to pick:");

	let mut chosen_card_slot = String::new();

	io::stdin().read_line(&mut chosen_card_slot)
		.expect("Failed to read line");

	let mut chosen_slot = 0;

	match chosen_card_slot.trim().parse::<usize>()
	{
		Ok(n) => chosen_slot = n,
		Err(_e) => println!("That was an invalid choice. Dealing with user error is not currently implemented."),
	}

	if chosen_slot < current_hand.len()
	{
		return current_hand[chosen_slot];
	}
	else
	{
		println!("Automatically choosing the first slot because of user error.");
		return current_hand[0];
	}
}

fn do_random_turn(game: &SushiGoGame) -> Card
{
	let current_hand = game.get_current_player().get_hand();

	let mut rng = rand::thread_rng();

	let mut rand_index: usize = 0;

	if current_hand.len() - 1 > 0
	{
		rand_index = rng.gen_range(0, current_hand.len() - 1);
	}

	return current_hand[rand_index];
}

pub fn print_nn_info(nn: &NeuralNetwork)
{
	let mut game = SushiGoGame::new(NUMBER_OF_PLAYERS);

	game.setup();

	println!("\nPlayer Hand:");

	for x in game.get_current_player().get_hand()
	{
		println!("{}", x);
	}

	let input = get_nn_input(&game);

	let output = nn.feed_forward(&input);

	println!("\nNode Outputs:");

	let mut to_chose_from: Vec<f32> = Vec::new();

	for (i, x) in output.iter().enumerate()
	{
		println!("card: {0} weight: {1}",Card::from(i), *x);
		if game.get_current_player().get_hand().contains(&(Card::from(i))) {to_chose_from.push(*x)} else {to_chose_from.push(0.0)};
	}

	println!("\nPossible Outputs:");

	for (i, x) in to_chose_from.iter().enumerate()
	{
		println!("card: {0} weight: {1}",Card::from(i), *x);
	}
}
pub mod neural_network;
pub mod game;

use neural_network::*;

use game::*;
use game::player::card::*;

use rayon::prelude::*;

use std::io;

mod model_c;
use model_c::*;

//---- Constants ----
pub const NUMBER_OF_PLAYERS: usize = 4;

// This is a struct which links a game to a set of neural network players.
pub struct NeuralNetworkGame
{
	game: SushiGoGame,
	nn: Vec<NeuralNetwork>
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
			return do_player_turn(&self.game, prev_result);
		}
	}

	pub fn get_winning_nn(&self) -> NeuralNetwork
	{
		self.nn[self.game.get_winner()].clone()
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

	let mut new_game = NeuralNetworkGame{game: new_game, nn: new_game_nn};

	new_game.play();

	// Game is finished, print some final info.
	new_game.get_game().print_prev_round_results();
	new_game.get_game().print_pudding_scores();
	new_game.get_game().print_final_scores();
}

// Creates the number of games required for a round and returns the id's of the winning neural networks.
pub fn create_and_play_games_parallel(number_of_games: usize, nn: &mut Vec<NeuralNetwork>) -> Vec<NeuralNetworkGame>
{
	let mut games: Vec<NeuralNetworkGame> = Vec::new();

	// Add the number of games we need.
	for _j in 0..number_of_games
	{
		// Create a new game and a vector to hold the neural network players.
		let new_game = SushiGoGame::new(NUMBER_OF_PLAYERS);
		let mut new_game_nn = Vec::new();

		// Pull the players from the pool of neural networks.
		for _j in 0..NUMBER_OF_PLAYERS
		{
			if nn.len() == 0
			{
				assert!(false, "Ran out of nn players!");
			}

			new_game_nn.push(nn.remove(0));
		}

		// Add the game to the list of games in this round.
		games.push(NeuralNetworkGame{game: new_game, nn: new_game_nn});
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
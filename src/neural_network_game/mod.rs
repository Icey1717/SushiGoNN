pub mod neural_network;
pub mod game;

use neural_network::*;

use game::*;
use game::player::card::*;

use rayon::prelude::*;

use std::time::Instant;

use radiant_rs::{Display, Renderer, Layer, Sprite, Color};

use enum_map::EnumMap;

use std::io;

//use std::{thread, time};

mod model_c;
use model_c::*;
use rand::Rng;

//---- Constants ----
pub const NUMBER_OF_PLAYERS: usize = 4;

#[derive(Copy,Clone)]
pub enum NeuralNetworkGamePlayerType
{
	NeuralNetwork,
	Human,
	Random
}

impl NeuralNetworkGamePlayerType {
	pub fn is_random(&self) -> bool {
		match *self {
			NeuralNetworkGamePlayerType::Random => true,
			_ => false,
		}
	}

	pub fn is_human(&self) -> bool {
		match *self {
			NeuralNetworkGamePlayerType::Human => true,
			_ => false,
		}
	}
}

pub struct NeuralNetworkGamePlayer
{
	pub player_type: NeuralNetworkGamePlayerType,
	pub nn: NeuralNetwork
}

impl NeuralNetworkGamePlayer
{
	pub fn new(player_type: NeuralNetworkGamePlayerType, nn: NeuralNetwork) -> NeuralNetworkGamePlayer
	{
		NeuralNetworkGamePlayer{player_type, nn}
	}

	pub fn clone(&self) -> NeuralNetworkGamePlayer
	{
		return NeuralNetworkGamePlayer{player_type: self.player_type, nn: self.nn.clone()};
	}
}

// This is a struct which links a game to a set of neural network players.
pub struct NeuralNetworkGame
{
	game: SushiGoGame,
	players: Vec<NeuralNetworkGamePlayer>
}

impl NeuralNetworkGame
{
	pub fn play(&mut self)
	{
		if self.has_human()
		{
			self.play_with_display();
		}
		else
		{
			let mut result = StepResult::NoResult;

			// Keep stepping through the game until we have a winner.
			while !result.is_game_over()
			{
				result = self.step_game(result);
			}

			//---- Game Finished
		}
	}

	pub fn play_with_display(&mut self)
	{
		let display = Display::builder().dimensions((1280, 720)).vsync().title("Window!").build().unwrap();

		let renderer = Renderer::new(&display).unwrap();

		let sprite_map = load_card_sprites(&renderer);

		let layer = Layer::new((1920.0, 1080.0));

		let mut result = StepResult::NoResult;

		let start_draw_y = 200.0;
		let start_draw_x = 160.0;
		let draw_card_size = 64.0;
		let draw_player_gap = 200.0;

		// Keep stepping through the game until we have a winner.
		while !result.is_game_over() && !display.poll_events().was_closed()
		{
			// Print previous round information.
			if result.is_round_over()
			{
				self.game.print_prev_round_results();
			}

			result = self.step_game(result);

			// Clear the layer (layers could also be drawn multiple times, e.g. a static UI might not need to be updated each frame)
			layer.clear();

			for i in 0..self.game.get_num_players()
			{
				for (j, card) in self.game.get_player_hand(i).iter().enumerate()
				{
					sprite_map[*card].draw(&layer, 0, (start_draw_x + (draw_card_size * j as f32), start_draw_y + (draw_player_gap * i as f32)), Color::WHITE);
				}

				for (j, card) in self.game.get_player_chosen(i).iter().enumerate()
				{
					sprite_map[*card].draw(&layer, 0, (start_draw_x + (draw_card_size * j as f32), start_draw_y + draw_card_size + (draw_player_gap * i as f32)), Color::WHITE);
				}
			}


			// draw the layer to the frame after clearing it with solid black.
			display.clear_frame(Color::BLACK);
			renderer.draw_layer(&layer, 0);

			display.swap_frame();

			//let ten_millis = time::Duration::from_millis(10);
			//let now = time::Instant::now();
			//thread::sleep(ten_millis);
		}
		
		//---- Game Finished
		// Game is finished, print some final info.
		self.get_game().print_prev_round_results();
		self.get_game().print_pudding_scores();
		self.get_game().print_final_scores();
	}

	pub fn step_game(&mut self, prev_result: StepResult) -> StepResult
	{
		self.game.step(self.take_turn(prev_result))
	}

	fn take_turn(&self, prev_result: StepResult) -> Card
	{
		// Get the player who should be taking their turn.
		let current_player = &self.players[self.game.get_current_player_id()];

		// Switch behaviour based on if they are human etc.
		match current_player.player_type
		{
			NeuralNetworkGamePlayerType::NeuralNetwork => return pick_cards(&self.game, &current_player.nn),
			NeuralNetworkGamePlayerType::Human => return do_player_turn(&self.game, prev_result),
			_ => return do_random_turn(&self.game),
		};
	}

	pub fn get_winning_nn(&self) -> NeuralNetworkGamePlayer
	{
		self.players[self.game.get_winner()].clone()
	}

	pub fn get_winning_id(&self) -> usize
	{
		self.game.get_winner()
	}

	pub fn get_game(&self) -> &SushiGoGame
	{
		&self.game
	}

	pub fn has_human(&self) -> bool
	{
		for x in &self.players
		{
			if x.player_type.is_human()
			{
				return true;
			}
		}

		false
	}
}

fn load_card_sprites(renderer: &Renderer) -> EnumMap<Card, Sprite>
{
	enum_map!
	{
			Card::Pudding => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::Pudding)).unwrap(),
			Card::Dumpling => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::Dumpling)).unwrap(),
			Card::Sashimi => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::Sashimi)).unwrap(),
			Card::Tempura => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::Tempura)).unwrap(),
			Card::MakiRoll1 => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::MakiRoll1)).unwrap(),
			Card::MakiRoll2 => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::MakiRoll2)).unwrap(),
			Card::MakiRoll3 => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::MakiRoll3)).unwrap(),
			Card::Chopsticks => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::Chopsticks)).unwrap(),
			Card::SalmonNigiri => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::SalmonNigiri)).unwrap(),
			Card::EggNigiri => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::EggNigiri)).unwrap(),
			Card::SquidNigiri => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::SquidNigiri)).unwrap(),
			Card::Wasabi => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::Wasabi)).unwrap(),
			_ => Sprite::from_file(&renderer.context(), get_sprite_filename(&Card::Chopsticks)).unwrap(),
    	}
}

pub fn get_usize_from_player_input(message: &str) -> usize
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

pub fn start_game_setup()
{
	let mut human_players = get_usize_from_player_input("Enter how many players will be human (0 - 4):");

	if human_players > NUMBER_OF_PLAYERS
	{
		human_players = NUMBER_OF_PLAYERS;
	}

	// Filename of the neural network to load.
	let mut in_file_name = String::new();

	let mut random_players = 0;

	if human_players < NUMBER_OF_PLAYERS
	{
		random_players = get_usize_from_player_input("Enter how many players will be random (0 - 4):");

		if random_players > NUMBER_OF_PLAYERS
		{
			random_players = NUMBER_OF_PLAYERS;
		}

		if random_players + human_players < NUMBER_OF_PLAYERS
		{
			println!("Enter name of neural network to load:");

			io::stdin().read_line(&mut in_file_name)
				.expect("Failed to read line");
		}
	}

	let mut number_of_games = 1;
	let mut batch_count = 1;

	if human_players <= 0
	{
		number_of_games = get_usize_from_player_input("Enter number of games to play:");
		batch_count = get_usize_from_player_input("Enter number of batches to do, this will reduce memory overhead:");
	}

	let mut print_nn_weights = false;

	if human_players + random_players < NUMBER_OF_PLAYERS
	{
		print_nn_weights = get_usize_from_player_input("Print neural network weights?") > 0;
	}

	start_game(batch_count, number_of_games, in_file_name, random_players, human_players, print_nn_weights);
}

// Plays three neural networks against an AI picking random choices.
// Plays number_of_games games per batch. Batches is used to reduce memory usage.
pub fn start_game(batches: usize, number_of_games: usize, in_file_name: String, number_of_random_players: usize, number_of_human_players: usize, print_nn_weights: bool)
{
	let random_started = Instant::now();

	let mut win_counter: [usize; NUMBER_OF_PLAYERS] = [0; NUMBER_OF_PLAYERS];

	for j in 0..batches
	{
		print!("Starting batch: {}\n", j);
		let batch_started = Instant::now();
		let mut new_game_nn = Vec::new();

		// Work out how many neural networks we need, and create an empty one for if we
		let number_of_neural_networks = NUMBER_OF_PLAYERS - number_of_random_players - number_of_human_players;
		let empty_nn = new_neural_network(0,0,0,0);

		// Load in the neural network if we have any neural network players.
		let mut loaded_nn = empty_nn.clone();
		if number_of_neural_networks > 0
		{
			loaded_nn = load_nn_from_file(in_file_name.trim());
		}

		// Pull the players from the pool of neural networks.
		for _j in 0..number_of_games
		{
			for _k in 0..number_of_human_players
			{
				new_game_nn.push(NeuralNetworkGamePlayer::new(NeuralNetworkGamePlayerType::Human, empty_nn.clone()))
			}

			for _k in 0..number_of_random_players
			{
				new_game_nn.push(NeuralNetworkGamePlayer::new(NeuralNetworkGamePlayerType::Random, empty_nn.clone()))
			}

			for _k in 0..number_of_neural_networks
			{
				new_game_nn.push(NeuralNetworkGamePlayer::new(NeuralNetworkGamePlayerType::NeuralNetwork, loaded_nn.clone()));
			}
		}

		print!("Created {} for batch in {}\n", new_game_nn.len(), sec_from_time(batch_started));

		let mut games: Vec<NeuralNetworkGame> = Vec::new();

		create_and_play_games_parallel(&mut games, number_of_games, &mut new_game_nn, print_nn_weights);

		for x in &games
		{
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
pub fn create_and_play_games_parallel(	games: &mut Vec<NeuralNetworkGame>,
										number_of_games: usize,
									  	nn: &mut Vec<NeuralNetworkGamePlayer>,
										print_nn_weights: bool)
{
	// Add the number of games we need.
	for _j in 0..number_of_games
	{
		// Create a new game and a vector to hold the neural network players.
		let mut new_game = SushiGoGame::new(NUMBER_OF_PLAYERS);

		// Setup the game.
		new_game.setup();

		// Tell the game whether or not to print nn weights.
		new_game.set_print_nn_weights(print_nn_weights);

		let mut new_game_nn = Vec::new();

		// Pull the players from the pool of neural networks.
		for _j in 0..NUMBER_OF_PLAYERS
		{
			// Check there are nn's in the input array.
			if nn.len() <= 0
			{
				assert!(false, "Ran out of nn players!");
			}

			// Add the nn to the list of nn's to be used in this game.
			new_game_nn.push(nn.remove(nn.len() - 1));
		}

		// Add the game to the list of games in this round.
		games.push(NeuralNetworkGame{game: new_game, players: new_game_nn});
	}

	//print!("Setup {} games. Starting parallel play. \n", number_of_games);

	//---- Play the games in this round.
	play_games_parallel(games);
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

pub fn create_neural_networks(number: usize) -> Vec<NeuralNetworkGamePlayer>
{
	let mut nn = Vec::new();

	let node_count = get_node_count();

	for i in 0..number	
	{
		nn.push(NeuralNetworkGamePlayer::new(NeuralNetworkGamePlayerType::NeuralNetwork, neural_network::new_neural_network(i as usize, node_count.input_node_count, node_count.hidden_node_count, node_count.output_node_count)));
	}

	nn
}

fn do_player_turn(game: &SushiGoGame, prev_result: StepResult) -> Card
{
	print!("The result for the previous round was {}\n", prev_result);

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
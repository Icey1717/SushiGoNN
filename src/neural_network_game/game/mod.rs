use rand::seq::SliceRandom;
use std::fmt;

//----------------------------------- Game Rules ----------------------------------------------
const NUMBER_OF_ROUNDS: u8 = 3;
pub const MAX_HAND_SIZE: usize = 8;

const PUDDING_COUNT: u8 =		10;
const DUMPLING_COUNT: u8 =		14;
const SAHSHIMI_COUNT: u8 =		14;
const TEMPURA_COUNT: u8 =		14;
const MAKI_ROLL_1_COUNT: u8 =	6;
const MAKI_ROLL_2_COUNT: u8 =	12;
const MAKI_ROLL_3_COUNT: u8 =	8;
const CHOPSTICKS_COUNT: u8 =	10;
const SALMON_NIGRI_COUNT: u8 =	10;
const EGG_NIGRI_COUNT: u8 =		5;
const SQUID_NIGRI_COUNT: u8 =	5;
const WASABI_COUNT: u8 =		6;

// For scoring rules see score.rs

//--------------------------------- End Game Rules ----------------------------------------------

pub mod player;
mod score;

use player::*;
use player::card::*;

use score::*;

const PRINT_DATA: bool = false;

//--------------------------------- Start Game Implementation ------------------------------------
#[derive(Copy,Clone)]
pub enum StepResult
{
	Success,
	Error,
	GameOver,
	NoResult,
    ChopsticksAvailable,
	RoundOver,
}

impl fmt::Display for StepResult
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let printable = match *self
		{
			StepResult::Success => "Success!",
			StepResult::Error => "Error!",
			StepResult::GameOver => "Game has been completed!",
			StepResult::NoResult => "Error! No Result!",
			StepResult::ChopsticksAvailable => "Chopsticks are available to use!",
			StepResult::RoundOver => "Round has been completed!",
		};
		write!(f, "{}", printable)
	}
}

impl StepResult {
	pub fn is_game_over(&self) -> bool {
		match *self {
			StepResult::GameOver => true,
			_ => false,
		}
	}

	pub fn is_chopsticks_available(&self) -> bool {
		match *self {
			StepResult::ChopsticksAvailable => true,
			_ => false,
		}
	}

	pub fn is_round_over(&self) -> bool {
		match *self {
			StepResult::RoundOver => true,
			_ => false,
		}
	}
}

pub struct SushiGoGame
{
	deck: Vec<Card>,
	players: Vec<Player>,
	current_player: usize,
	game_over: bool,
	current_round: u8,
	number_of_players: usize,
	winner: usize
}

impl SushiGoGame
{
	pub fn new(number_of_players: usize) -> SushiGoGame
	{
		return SushiGoGame{deck: Vec::new(), players: Vec::new(), current_player: 0, game_over: false, current_round: 0, number_of_players: number_of_players, winner: 0};
	}

	pub fn setup(&mut self)
	{
		//---- Setup
		self.current_player = 0;
		self.game_over = false;
		self.current_round = 0;

		self.deck = setup_deck();
		let mut rng = rand::thread_rng();
		self.deck.shuffle(&mut rng);

		// ---- Print all the cards in the deck.
		if PRINT_DATA
		{
			self.print_deck();
		}

		//---- Create players.
		self.players = setup_players(4);

		//---- Deal players some cards.
		if PRINT_DATA
		{
			print!("Dealing hands.");
		}

		deal_cards(&mut self.players, &mut self.deck);

		if PRINT_DATA
		{
			self.print_hands();
			print!("Starting Round {}.", self.current_round + 1);
		}
	}

	// Tries to play a specific card for the current player.
	pub fn step(&mut self, action: Card) -> StepResult
	{
		let mut result = self.update_game(action);

		if self.game_over
		{
			self.winner = self.calculate_winner();
			result = StepResult::GameOver;
		}

		result
	}

	// Updates the game state using the input card.
	fn update_game(&mut self, action: Card) -> StepResult
	{
		let mut result = StepResult::NoResult;
		let player = &mut self.players[self.current_player];

		if PRINT_DATA
		{
			print!("\nStarting turn for player {}.", player.get_id());

			print!("\nCards in hand:\n");

			print_cards(&player.get_hand());

			print!("\nChosen cards:\n");
			if PRINT_DATA
			{
				print_cards(&player.get_chosen_cards());
			}

			print!("\nChoosing cards...\n");
		}

		// As long as we have not chosen to play no card, adjust the players hand.
		if action != Card::None
		{
			// This will remove a chopsticks from the field and return them to our hand, in the case
			// that we played another card due to the power of chopsticks.
			player.return_chopsticks();

			result = if player.choose_card(action) { StepResult::Success } else { StepResult::Error };
		}

		if PRINT_DATA
		{
			print!("Cards in hand:\n");
			print_cards(&player.get_hand());

			print!("\nChosen cards:\n");
			print_cards(&player.get_chosen_cards());
		}

		// Check to see if the current player has fresh chopsticks and give them another turn
		if player.remove_fresh_chopsticks() && player.get_hand_size() > 0
		{
			if PRINT_DATA
			{
				print!("We have fresh chopsticks in our chosen cards, adding none option and letting us take another go.\n");
			}

            player.add_card_to_hand(Card::None);
			result = StepResult::ChopsticksAvailable;
			return result;
		}

		// Change all the chopsticks back to fresh chopsticks and remove any 'none' card options.
		self.sanitize_hands();

		// Check to see if the round is over now that we have acted.
		if self.check_round_ended()
		{
			self.on_round_end();
			result = StepResult::RoundOver;
			return result;
		}

		// Next players turn
		self.current_player += 1;

		// Round is not over, see if there is another player to act.
		if self.current_player >= self.number_of_players
		{
			// Everyone has acted, switch hands and start again.
			self.current_player = 0;

			self.swap_hands();
		}

		return result;
	}

	fn sanitize_hands(&mut self)
	{
		for x in self.players.iter_mut()
		{
			x.sanitize_cards();
		}
	}

	fn check_round_ended(&self) -> bool
	{
		for x in &self.players
		{
			if x.get_hand_size() > 0
			{
				return false;
			}
		}

		return true;
	}

	fn on_round_end(&mut self)
	{
		if PRINT_DATA
		{
			print!("Completed Round {}.\n", self.current_round + 1);
			self.print_chosen();
		}

		self.current_round += 1;
		self.current_player = 0;

		for x in &mut self.players
		{
			x.add_round_result();
			x.clear_chosen_cards();
		}

		if self.current_round >= NUMBER_OF_ROUNDS
		{
			self.game_over = true;
		}
		else
		{
			//---- Deal players some cards.
			deal_cards(&mut self.players, &mut self.deck);
			self.print_hands();

			if PRINT_DATA
			{
				print!("Starting Round {}:", self.current_round + 1);
			}
		}
	}

	fn swap_hands(&mut self)
	{
		// This hand will store the cards we give to the next player
		let mut replace_hand = self.players[self.players.len() - 1].get_hand();

		for x in &mut self.players
		{
			let clone_hand = x.get_hand();

			x.replace_hand(replace_hand);

			replace_hand = clone_hand;
		}
	}

	// Returns how many players there are in the current game.
	pub fn get_num_players(&self) -> usize
	{
		self.players.len()
	}

	// Returns the hand of the player with the given ID
	pub fn get_player_hand(&self, id: usize) -> Vec<Card>
	{
		let mut hand = Vec::new();
		assert!(id < self.players.len(), "Tried to get player cards with ID outside of bounds!");
		if id < self.players.len()
		{
			hand = self.players[id].get_hand();
		}
		return hand;
	}

	// Returns the chosen cards of the player with the given ID
	pub fn get_player_chosen(&self, id: usize) -> Vec<Card>
	{
		let mut chosen = Vec::new();
		assert!(id < self.players.len(), "Tried to get player cards with ID outside of bounds!");
		if id < self.players.len()
		{
			chosen = self.players[id].get_chosen_cards();
		}
		return chosen;
	}

	// Returns the ID of the current player
	pub fn get_current_player_id(&self) -> usize
	{
		self.current_player
	}

	// Gets the hand of the player who is currently taking their turn.
	pub fn get_current_player_hand(&self) -> Vec<Card>
	{
		self.players[self.current_player].get_hand()
	}

	pub fn get_current_player(&self) -> &Player
	{
		&self.players[self.current_player]
	}

	pub fn get_winning_score(&self) -> i32
	{
		return self.players[self.winner].get_final_score();
	}

	pub fn get_winner(&self) -> usize
	{
		self.winner
	}

	fn print_deck(&self)
	{
		if !PRINT_DATA
		{
			return;
		}

		print!("Cards in deck:");
		for x in &self.deck 
		{
			print!("{}", x);
		}
		print!("\n");
	}

	fn print_hands(&self)
	{
		if !PRINT_DATA
		{
			return;
		}

		//---- Print all the cards each player has.
		for x in self.players.iter()
		{
			print!("Hand for player {}:", x.get_id());
			print_cards(&x.get_hand());
			print!("\n");
		}
	}

	fn print_chosen(&self)
	{
		if !PRINT_DATA
		{
			return;
		}

		//---- Print all the cards each player has.
		for x in self.players.iter()
		{
			print!("Chosen for player {}:", x.get_id());
			print_cards(&x.get_chosen_cards());
			print!("\n");
		}
	}

	// This is a helper function for player games to print the previous rounds results.
	pub fn print_prev_round_results(&self)
	{
		let prev_round = self.current_round - 1;

		print!("\nResults for round: {}\n", prev_round);

		if prev_round < NUMBER_OF_ROUNDS
		{
			// Get the score for those results.
			let round_scores = calc_scores_for_round(&self.players, prev_round);

			// Add the scores together.
			for (i, x) in round_scores.iter().enumerate()
			{
				print!("Score for player {0}: {1}\n", i, *x);
			}
		}
	}

	pub fn print_pudding_scores(&self)
	{
		print!("\nPudding results:\n");
		for x in calc_pudding_counts_for_game(&self.players, NUMBER_OF_ROUNDS).iter()
		{
			print!("Pudding count for player {0}: {1}\n", x.0, x.1);
		}
		// Add on score for puddings.
		for (i, x) in calc_pudding_scores_for_game(&self.players, NUMBER_OF_ROUNDS).iter().enumerate()
		{
			print!("Pudding scores for player {0}: {1}\n", i, *x);
		}
	}

	pub fn print_final_scores(&self)
	{
		print!("\nFinal Results:\n");

		//---- Print all the final scores
		for x in self.players.iter()
		{
			print!("Final score for player {0}: {1}\n", x.get_id(), x.get_final_score());
		}
	}

	fn calculate_winner(&mut self) -> usize
	{
		if PRINT_DATA
		{
			print!("Finished Playing!\n");
		}

		//---- Scoring the game
		let mut final_scores = vec![0; self.players.len()];

		// For each of the rounds.
		for i in 0..NUMBER_OF_ROUNDS
		{
			// Get the score for those results.
			let round_scores = calc_scores_for_round(&self.players, i);

			// Add the scores together.
			for (i, x) in round_scores.iter().enumerate()
			{
				final_scores[i] += *x;
			}
		}

		// Add on score for puddings.
		for (i, x) in calc_pudding_scores_for_game(&self.players, NUMBER_OF_ROUNDS).iter().enumerate()
		{
			if PRINT_DATA
			{
				print!("Pudding scores for player {0}: {1}\n", i, *x);
			}

			final_scores[i] += *x;
		}

		// Work out the highest score.
		let mut highest_score = 0;

		for x in final_scores.iter()
		{
			if *x >= highest_score
			{
				highest_score = *x;
			}
		}

		// Set players final scores and add any winners to the pot.
		let mut winners = Vec::new();

		for (i, x) in self.players.iter_mut().enumerate()
		{
			x.set_final_score(final_scores[i]);

			if x.get_final_score() == highest_score
			{
				winners.push(i);
			}
		}

		if PRINT_DATA
		{
			self.print_final_scores();
		}

		if winners.len() > 0
		{
			// Pick a random winner amongst the players with the highest score.
			let mut rng = rand::thread_rng();
			winners.shuffle(&mut rng);
			return winners[0];
		}

		println!("No winner!");
		return 0;
	}
}

fn setup_deck() -> Vec<Card>
{
	let pudding_vec =			vec![Card::Pudding; PUDDING_COUNT as usize];
	let mut dumpling_vec =		vec![Card::Dumpling; DUMPLING_COUNT as usize];
	let mut sashimi_vec =		vec![Card::Sashimi; SAHSHIMI_COUNT as usize];
	let mut tempura_vec =		vec![Card::Tempura; TEMPURA_COUNT as usize];
	let mut maki_roll1_vec =	vec![Card::MakiRoll1; MAKI_ROLL_1_COUNT as usize];
	let mut maki_roll2_vec =	vec![Card::MakiRoll2; MAKI_ROLL_2_COUNT as usize];
	let mut maki_roll3_vec =	vec![Card::MakiRoll3; MAKI_ROLL_3_COUNT as usize];
	let mut chopsticks_vec =	vec![Card::Chopsticks; CHOPSTICKS_COUNT as usize];
	let mut salmon_nigri_vec =	vec![Card::SalmonNigri; SALMON_NIGRI_COUNT as usize];
	let mut egg_nigri_vec =		vec![Card::EggNigri; EGG_NIGRI_COUNT as usize];
	let mut squid_nigri_vec =	vec![Card::SquidNigri; SQUID_NIGRI_COUNT as usize];
	let mut wasabi_vec =		vec![Card::Wasabi; WASABI_COUNT as usize];

	let mut deck = pudding_vec;
	deck.append(&mut dumpling_vec);
	deck.append(&mut tempura_vec);
	deck.append(&mut sashimi_vec);
	deck.append(&mut maki_roll1_vec);
	deck.append(&mut maki_roll2_vec);
	deck.append(&mut maki_roll3_vec);
	deck.append(&mut chopsticks_vec);
	deck.append(&mut salmon_nigri_vec);
	deck.append(&mut egg_nigri_vec);
	deck.append(&mut squid_nigri_vec);	
	deck.append(&mut wasabi_vec);

	return deck;
}

fn deal_cards<'a>(players: &'a mut Vec<Player>, deck: &'a mut Vec<Card>)
{
	for x in players
	{
		for _i in 0..MAX_HAND_SIZE
		{
			let next_card_option = deck.pop(); //.unwrap();

			match next_card_option
			{
				Some(y) => x.add_card_to_hand(y),
				None => assert!(false, "There was no cards left in the deck, this should be impossible!")
			}
		}
	}
}

fn setup_players(num_players: u8) -> Vec<Player>
{
	let mut players: Vec<Player> = Vec::new();
	for i in 0..num_players
	{
		players.push(new_player(i as usize));
	}
	return players;
}

fn return_chopsticks(player: &mut Player)
{
	player.add_card_to_hand(Card::Chopsticks);
	if player.has_chosen_card(Card::Chopsticks)
	{
		player.remove_chosen_card(Card::Chopsticks);
		return;
	}
	assert!(false, "We assumed we had chopsticks cause we played two cards, but we don't have any chopsticks!");
}

fn debug_print(message: &str)
{
	if !PRINT_DATA
	{
		return;
	}

	print!("{}", message);
}
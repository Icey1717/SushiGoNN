pub mod card;

use card::*;

use crate::neural_network_game::game::PRINT_DATA;

// This struct is used to track results at the end of the round i.e. by tracking the total number of maki rolls, as opposed to the number of each maki roll card (1,2, and 3).
pub struct SushiResult
{
	pub pudding_count: i16,
	pub dumpling_count: i16,
	pub sashimi_count: i16,
	pub tempura_count: i16,
	pub maki_roll_count: i16,
	pub salmon_nigri_count: i16,
	pub egg_nigri_count: i16,
	pub squid_nigri_count: i16,
	pub wasabi_salmon_nigri_count: i16,
	pub wasabi_egg_nigri_count: i16,
	pub wasabi_squid_nigri_count: i16,
}

impl SushiResult
{
	pub fn get_maki_roll_count(&self) -> i16
	{
		self.maki_roll_count
	}

	pub fn add_nigiri(&mut self, nigiri: &Card, has_wasabi: &mut bool)
	{
		if *has_wasabi
		{
			match nigiri
			{
			Card::SalmonNigiri => 	self.wasabi_salmon_nigri_count += 1,
			Card::EggNigiri => 		self.wasabi_egg_nigri_count += 1,
			Card::SquidNigiri => 	self.wasabi_squid_nigri_count += 1,
			_ =>                	{},
			}

			// Reset the wasabi flag reference.
			*has_wasabi = false;
		}
		else
		{
			match nigiri
			{
			Card::SalmonNigiri => 	self.salmon_nigri_count += 1,
			Card::EggNigiri => 		self.egg_nigri_count += 1,
			Card::SquidNigiri => 	self.squid_nigri_count += 1,
			_ =>                	{},
			}
		}
	}
}

// This is the internal representation of the player which is tracked by the game. The state of this player is one part of the input fed into the neural network.
pub struct Player
{
	// A unique ID for this player (in regards to the game)
	id: usize,
    hand: Vec<Card>,
	chosen_cards: Vec<Card>,
	weights: Vec<f64>,
	sushi_go: bool,
	prev_chosen_cards: Vec<Card>,

	// This vector holds the number of cards a player has at the end of a given round, it is used to calculate the score
	round_results: Vec<SushiResult>,
	final_score: i32,
}

impl Player 
{
	pub fn add_card_to_hand(&mut self, new_card: Card)
	{
		self.hand.push(new_card);
	}

	pub fn remove_card_from_hand(&mut self, card_to_remove: Card) -> bool
	{
		for (i, x) in self.hand.iter().enumerate()
		{
			if x == &card_to_remove
			{
				self.hand.remove(i as usize);
				return true;
			}
		}

		assert!(false, "Trying to remove a card we don't have! {0}", card_to_remove);
		return false;
	}

    pub fn has_card_in_hand(&self, card: Card) -> bool
    {
        for x in self.hand.iter()
            {
                if x == &card
                {
                    return true;
                }
            }

        return false;
    }

	pub fn has_chosen_card(&self, card: Card) -> bool
	{
		for x in self.chosen_cards.iter()
		{
			if x == &card
			{
				return true;
			}
		}

		return false;
	}

	pub fn choose_card(&mut self, card: Card) -> bool
	{
        if PRINT_DATA
        {
            print!("The card chosen was {0}.\n", card);
        }

		// Find the card we played and remove it from the hand.
		if !self.remove_card_from_hand(card)
		{
			return false;
		}
		
		self.chosen_cards.push(card);

		return true;
	}

	pub fn get_hand(&self) -> Vec<Card>
	{
		return self.hand.clone();
	}

	pub fn get_hand_size(&self) -> usize
	{
		return self.hand.len();
	}

    // This replaces the players hand and refreshes any chopsticks.
	pub fn replace_hand(&mut self, replacement: Vec<Card>)
    {
		self.hand = replacement;
	}

	// Refreshes the chopsticks in your chosen cards and removes 'none' cards from your hand
    // ready for a new turn.
	pub fn sanitize_cards(&mut self)
	{
        while self.has_card_in_hand(Card::None)
        {
            self.remove_card_from_hand(Card::None);
        }
	}

	// Returns true if we have chopsticks that we didn't pick this round.
	pub fn has_chopsticks(&self) -> bool
	{
		for i in 0..self.chosen_cards.len() - 1
		{
			if self.chosen_cards[i] == Card::Chopsticks
			{
				return true;
			}
		}

		false
	}

	// Removes the chopsticks we used to pick another card and returns them to our hand.
	pub fn return_chopsticks(&mut self)
	{
		if self.remove_chosen_card(Card::Chopsticks)
		{
			if PRINT_DATA
			{
				print!("Removing chopsticks from our chosen cards and adding them back into our hand.\n");
			}

			self.add_card_to_hand(Card::Chopsticks);
		}
	}

	pub fn card_is_at_position_hand(&self, position: usize, in_card: &usize) -> bool
	{
		if position < self.hand.len()
		{
			return self.hand[position] as usize == *in_card;
		}
		return false; // Card cannot be in our hand since we don't have any cards above this point.
	}

	pub fn card_is_at_position_chosen_cards(&self, position: usize, in_card: &usize) -> bool
	{
		if position < self.chosen_cards.len()
		{
			return self.chosen_cards[position] as usize == *in_card;
		}
		return false; // Card cannot be in our hand since we don't have any cards above this point.
	}

	pub fn get_chosen_cards(&self) -> Vec<Card>
	{
		return self.chosen_cards.clone();
	}

	pub fn get_chosen_cards_size(&self) -> usize
	{
		return self.chosen_cards.len();
	}

	pub fn clear_chosen_cards(&mut self)
	{
		self.chosen_cards.clear();
	}

	pub fn remove_chosen_card(&mut self, card_to_remove: Card) -> bool
	{
		for (i, x) in self.chosen_cards.iter().enumerate()
		{
			if x == &card_to_remove
			{
				self.chosen_cards.remove(i as usize);
				return true;
			}
		}

		return false;
	}

	pub fn get_id(&self) -> usize
	{
		return self.id;
	}

    // Adds a struct with our current card count to the vector of results. We should end up with 3
	pub fn add_round_result(&mut self)
	{
		self.round_results.push(self.get_round_results());
	}

	fn get_round_results(&self) -> SushiResult
	{
		// This struct will keep count of each card for points.
		let mut round_results = SushiResult{pudding_count: 0, dumpling_count: 0, sashimi_count: 0, tempura_count: 0, maki_roll_count: 0, salmon_nigri_count: 0, egg_nigri_count: 0, squid_nigri_count: 0, wasabi_salmon_nigri_count: 0, wasabi_egg_nigri_count: 0, wasabi_squid_nigri_count: 0};

		// This flag tracks whether the next nigiri card should be dipped in wasabi.
		let mut has_wasabi = false;

		// Loop through and tally what we have.
		for x in self.chosen_cards.iter()
		{
			match x
			{
				Card::Pudding =>			round_results.pudding_count	+= 1,
				Card::Dumpling =>			round_results.dumpling_count += 1,
				Card::Sashimi =>			round_results.sashimi_count	+= 1,
				Card::Tempura =>			round_results.tempura_count	+= 1,
				Card::MakiRoll1 =>			round_results.maki_roll_count += 1,
				Card::MakiRoll2 =>			round_results.maki_roll_count += 2,
				Card::MakiRoll3 =>			round_results.maki_roll_count += 3,
				Card::Chopsticks =>			{},
				Card::SalmonNigiri =>		round_results.add_nigiri(x, &mut has_wasabi),
				Card::EggNigiri =>			round_results.add_nigiri(x, &mut has_wasabi),
				Card::SquidNigiri =>		round_results.add_nigiri(x, &mut has_wasabi),
				Card::Wasabi =>				has_wasabi = true,
				Card::CardMax =>			{},
				Card::None =>				{},
			}
		}

		return round_results;
	}

	pub fn get_round_result(&self, index: usize) -> &SushiResult
	{
		return &self.round_results[index as usize];
	}

	pub fn set_final_score(&mut self, new_final_score: i32)
	{
		self.final_score = new_final_score;
	}

	pub fn get_final_score(&self) -> i32
	{
		return self.final_score;
	}
}

pub fn new_player(id: usize) -> Player
{
	return Player{id: id as usize, hand: Vec::new(), chosen_cards: Vec::new(), sushi_go: false, weights: vec![1.0; Card::CardMax as usize], round_results: Vec::new(), prev_chosen_cards: Vec::new(), final_score: 0}
}
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
		return self.maki_roll_count;
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

		let mut new_card = card;

		// Mutate nigri cards if we have wasabi.
		for x in self.chosen_cards.iter_mut()
		{
			// For each of our stack, check if we have a wasabi and add this card to it if we do.
			if x == &Card::Wasabi
			{
				// Replace the card we get with a wasabi variant if we have picked that card, or leave it alone otherwise.
				new_card = if card == Card::SalmonNigri {Card::WasabiSalmonNigri} else if card == Card::EggNigri {Card::WasabiEggNigri} else if card == Card::SquidNigri {Card::WasabiSquidNigri} else {card};

				if new_card != card
				{
					if PRINT_DATA
					{
						print!("The card chosen was morphed via the power of wasabi into {0}.\n", new_card);
					}

                    *x = Card::UsedWasabi;
					break;
				}
			}
		}
		
		self.chosen_cards.push(new_card);

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
		for x in self.chosen_cards.iter_mut()
		{
			if x == &Card::Chopsticks
			{
				*x = Card::FreshChopsticks;
			}
		}

        while self.has_card_in_hand(Card::None)
        {
            self.remove_card_from_hand(Card::None);
        }
	}

	// Returns true if the player has fresh chopsticks and can take another go. Converts the found
	// chopsticks back to normal.
	pub fn remove_fresh_chopsticks(&mut self) -> bool
	{
		let mut has_fresh_chopsticks = false;

		for x in self.chosen_cards.iter_mut()
		{
			if x == &Card::FreshChopsticks
			{
				*x = Card::Chopsticks;
				has_fresh_chopsticks = true;
			}
		}

		return has_fresh_chopsticks;
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
		let mut round_results = SushiResult{pudding_count: 0, dumpling_count: 0, sashimi_count: 0, tempura_count: 0, maki_roll_count: 0, salmon_nigri_count: 0, egg_nigri_count: 0, squid_nigri_count: 0, wasabi_salmon_nigri_count: 0, wasabi_egg_nigri_count: 0, wasabi_squid_nigri_count: 0};

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
				Card::SalmonNigri =>		round_results.salmon_nigri_count += 1,
				Card::EggNigri =>			round_results.egg_nigri_count += 1,
				Card::SquidNigri =>			round_results.squid_nigri_count	+= 1,
				Card::Wasabi =>				{}
				Card::CardMax =>			{},
				Card::WasabiSalmonNigri =>	round_results.wasabi_salmon_nigri_count	+= 1,
				Card::WasabiEggNigri =>		round_results.wasabi_egg_nigri_count += 1,
				Card::WasabiSquidNigri =>	round_results.wasabi_squid_nigri_count += 1,
				Card::UsedWasabi =>			{},
				Card::FreshChopsticks =>	{},
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
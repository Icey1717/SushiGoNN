use super::neural_network::*;

use super::game::*;
use super::game::player::card::*;

// Use the feed forward algorithm to pick a card from the players current hand.
pub fn pick_cards(game: &SushiGoGame, nn: &NeuralNetwork) -> Vec<Card>
{
	let mut chosen_cards = Vec::new();

    let player_hand = game.get_player_hand(game.get_current_player_id());
	
	let output = nn.feed_forward(&get_nn_input(game));

	let mut highest_index = 0;
	let mut highest_value = -1.0;

	let mut to_chose_from: Vec<f32> = Vec::new();

	for (i, x) in output.iter().enumerate()
	{
		if player_hand.contains(&(Card::from(i))) {to_chose_from.push(*x)} else {to_chose_from.push(0.0)};
	}

	for (i, x) in to_chose_from.iter().enumerate()
	{
		if *x > highest_value
		{
            highest_index = i;
            highest_value = *x;
		}
	}

	//println!("Chosen Index is {0} and card is {1}", highest_index, Card::from(highest_index));
	
	chosen_cards.push(Card::from(highest_index));

    chosen_cards
}

pub fn get_nn_input(game: &SushiGoGame) -> Vec<f32>
{
	// This will be the vector of inputs fed into the neural network.
	// Size is twice the number 
	let mut player_state: Vec<f32> = Vec::new();

	// For the number of cards we can have in our hand.
	for i in 0..MAX_HAND_SIZE
	{
		// For each type of possible card
		for j in 0..Card::CardMax as usize
		{
			// Enter a 1.0 if that card is in this position, or a 0.0 if it isn't.
			if game.get_current_player().card_is_at_position_hand(i, &j)
			{
				player_state.push(1.0);
			}
			else
			{
				player_state.push(0.0);
			}
		}
	}

	for i in 0..MAX_HAND_SIZE
	{
		// For each type of possible card
		for j in 0..Card::CardMax as usize
		{
			// Enter a 1.0 if that card is in this position, or a 0.0 if it isn't.
			if game.get_current_player().card_is_at_position_chosen_cards(i, &j)
			{
				player_state.push(1.0);
			}
			else
			{
				player_state.push(0.0);
			}
		}
	}

	player_state
}

pub fn get_node_count() -> NodeCount
{
	let input_size: usize = MAX_HAND_SIZE * (Card::CardMax as usize) * 2;
	let hidden_layer_size: usize = 136;
	let output_layer_size: usize = Card::CardMax as usize;

	NodeCount{input_size, hidden_layer_size, output_layer_size}
}
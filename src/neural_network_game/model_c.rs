use super::neural_network::*;

use super::game::*;
use super::game::player::card::*;

// Use the feed forward algorithm to pick a card from the players current hand.
pub fn pick_cards(game: &SushiGoGame, nn: &NeuralNetwork) -> Card
{
	// Get an array of weights based on the cards in our hand.
    let player_hand = game.get_current_player_hand();
	let output = nn.feed_forward(&get_nn_input(game));

	// Work out what the highest weighted card we have is.
	let mut highest_index = 0;
	let mut highest_value = -1.0;

	// This vector holds valid choices.
	let mut to_chose_from: Vec<f32> = Vec::new();

	for (i, x) in output.iter().enumerate()
	{
		if player_hand.contains(&(Card::from(i))) {to_chose_from.push(*x)} else {to_chose_from.push(0.0)};
	}

	// Find the highest value valid choice.
	for (i, x) in to_chose_from.iter().enumerate()
	{
		if *x > highest_value
		{
            highest_index = i;
            highest_value = *x;
		}
	}

	// Print some info about what we chose if the game has that option set.
	if game.should_print_nn_weights()
	{
		print!("Printing weights:\n");
		for (i, x) in to_chose_from.iter().enumerate()
		{
			print!("With the given hand, weight for {} is: {}\n", Card::from(i), x);
		}

		println!("Chosen Index is {0} and card is {1}", highest_index, Card::from(highest_index));
	}
	
	Card::from(highest_index)
}

pub fn get_nn_input(game: &SushiGoGame) -> Vec<f32>
{
	// This will be the vector of inputs fed into the neural network.
	// Size is twice the number 
	let mut player_state: Vec<f32> = Vec::new();

	for i in 0..(Card::CardMax as usize)
	{
		if game.get_current_player().get_hand().contains(&(Card::from(i))) {player_state.push(1.0)} else {player_state.push(0.0)};
	}

	player_state
}

pub fn get_node_count() -> NeuralNetworkProperties
{
	let input_node_count: usize = Card::CardMax as usize;
	let hidden_node_count: usize = 20;
	let output_node_count: usize = Card::CardMax as usize;

	NeuralNetworkProperties {input_node_count, hidden_node_count, output_node_count}
}
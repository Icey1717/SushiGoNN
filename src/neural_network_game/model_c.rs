use super::neural_network::*;

use super::game::*;
use super::game::player::card::*;

// Use the feed forward algorithm to pick a card from the players current hand.
pub fn pick_cards(game: &SushiGoGame, nn: &NeuralNetwork) -> Card
{
    let player_hand = game.get_current_player_hand();
	
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

pub fn get_node_count() -> NodeCount
{
	let input_size: usize = Card::CardMax as usize;
	let hidden_layer_size: usize = 20;
	let output_layer_size: usize = Card::CardMax as usize;

	NodeCount{input_size, hidden_layer_size, output_layer_size}
}
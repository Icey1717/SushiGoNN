use super::player::*;

//---- Scoring Rules ----
const MOST_MAKI_ROLL_POINTS: i8 =		6;
const SECOND_MOST_MAKI_POINTS: i8 = 	3;
const TEMPURA_PAIR_POINTS: i16 =		5;
const SASHIMI_TRIO_POINTS: i16 =		10;
const DUMPLING_POINTS: [i16; 5] =		[1, 3, 6, 10, 15];
const SALMON_NIGRI_POINTS: i16 =		2;
const EGG_NIGRI_POINTS: i16 =			1;
const SQUID_NIGRI_POINTS: i16 =			3;
const WASABI_SALMON_NIGRI_POINTS: i16 =	9;
const WASABI_EGG_NIGRI_POINTS: i16 =	3;
const WASABI_SQUID_NIGRI_POINTS: i16 =	6;

const MOST_PUDDING_SCORE: i16 = 		6;
const LEAST_PUDDING_SCORE: i16 = 		-6;

const TEMPURA_PER_POINT: i16 =          2;
const SASHIMI_PER_POINT: i16 =          3;
const MAX_DUMPLINGS: usize =            4;
//---- End Scoring Rules ----

fn add_score(id: u8, amount: i16, scores: &mut [i32])
{
	if id >= scores.len() as u8
	{
		assert!(id < scores.len() as u8, "Trying to increment score for id which is out of bounds!");
		return;
	}

	scores[id as usize] += amount as i32;
}

pub fn calc_scores_for_round(players: &Vec<Player>, round: u8) -> Vec<i32>
{
	// Gather the results for each player.
	let mut round_results: Vec<&SushiResult> = Vec::new();

	for x in &mut players.iter()
	{
		round_results.push(x.get_round_result(round as usize));
	}

	let mut scores = vec![0; round_results.len()];
	
    //---- Maki Rolls
	add_maki_roll_score(&mut scores, &round_results);

    for (i, x) in round_results.iter().enumerate()
    {
        //---- Tempura
        let tempura_multiplier = x.tempura_count;
        let tempura_score = (tempura_multiplier % TEMPURA_PER_POINT) * TEMPURA_PAIR_POINTS;
        add_score(i as u8, tempura_score , &mut scores);

        //---- Sashimi
        let sashimi_multiplier = x.sashimi_count;
        let sashimi_score = (sashimi_multiplier % SASHIMI_PER_POINT) * SASHIMI_TRIO_POINTS;
        add_score(i as u8, sashimi_score , &mut scores);

        //---- Dumplings
        let mut dumpling_multiplier: usize = x.dumpling_count as usize;

        if dumpling_multiplier > 0
        {
            if dumpling_multiplier > MAX_DUMPLINGS
            {
                dumpling_multiplier = MAX_DUMPLINGS;
            }

            let dumpling_score = DUMPLING_POINTS[dumpling_multiplier - 1];
            add_score(i as u8, dumpling_score, &mut scores);
        }

        // Nigri
        add_score(i as u8, x.salmon_nigri_count * SALMON_NIGRI_POINTS, &mut scores);
        add_score(i as u8, x.egg_nigri_count * EGG_NIGRI_POINTS, &mut scores);
        add_score(i as u8, x.squid_nigri_count * SQUID_NIGRI_POINTS, &mut scores);
//        add_score(i as u8, x.wasabi_salmon_nigri_count * WASABI_SALMON_NIGRI_POINTS, &mut scores);
//        add_score(i as u8, x.wasabi_egg_nigri_count * WASABI_EGG_NIGRI_POINTS, &mut scores);
//        add_score(i as u8, x.wasabi_squid_nigri_count * WASABI_SQUID_NIGRI_POINTS, &mut scores);
    }


	return scores;
}

pub fn calc_pudding_counts_for_game(players: &Vec<Player>, total_rounds: u8) -> Vec<(usize, i16)>
{
	let mut pudding_counts: Vec<(usize, i16)> = Vec::new();

	for x in &mut players.iter()
	{
		let mut current_pudding_count = 0;
		// For each of the rounds.
		for i in 0..total_rounds
			{
				current_pudding_count = x.get_round_result(i as usize).pudding_count;
			}

		let pair = (x.get_id(), current_pudding_count);
		pudding_counts.push(pair);
	}

	pudding_counts.sort_by(|a,b| a.1.cmp(&b.1));
	return pudding_counts;
}
pub fn calc_pudding_scores_for_game(players: &Vec<Player>, total_rounds: u8) -> Vec<i32>
{
	let mut scores = vec![0; players.len()];

	// Gather the results for each player.
	let pudding_counts: Vec<(usize, i16)> = calc_pudding_counts_for_game(players, total_rounds);

	let lowest_value = pudding_counts[0].1;
	let highest_value = pudding_counts[players.len() - 1].1;

	// if everyone had the same number of puds, no points for anyone.
	if lowest_value == highest_value
	{
		return scores;
	}

	let mut lowest_count = 0;
	let mut highest_count = 0;

	for x in &pudding_counts
	{
		if x.1 == lowest_value
		{
			lowest_count += 1;
		}

		if x.1 == highest_value
		{
			highest_count += 1;
		}
	}

	for x in &pudding_counts
	{
		if x.1 == lowest_value
		{
			scores[x.0] = (LEAST_PUDDING_SCORE / lowest_count) as i32;
		}

		if x.1 == highest_value
		{
			scores[x.0] = (MOST_PUDDING_SCORE / highest_count) as i32;
		}
	}

	return scores;
}

fn add_maki_roll_score(scores: &mut [i32], results: &[&SushiResult])
{
	let mut most_maki_rolls = Vec::new();
	let mut second_most_maki_rolls = Vec::new();

	find_most_maki_rolls(results, &mut most_maki_rolls, &mut second_most_maki_rolls);

	let num_most_maki_roll_players = most_maki_rolls.len() as i8;

	if num_most_maki_roll_players > 0
	{
		let maki_roll_points_each = (MOST_MAKI_ROLL_POINTS / num_most_maki_roll_players) as i16;

		for x in most_maki_rolls
		{
			add_score(x as u8, maki_roll_points_each, scores);
		}

		// If there was no tie for first, give some points for second place.
		if num_most_maki_roll_players <= 1
		{
			let num_second_most_maki_players = second_most_maki_rolls.len() as i8;

			if second_most_maki_rolls.len() > 0
			{
				let second_maki_roll_points_each = (SECOND_MOST_MAKI_POINTS / num_second_most_maki_players) as i16;

				for x in second_most_maki_rolls
				{
					add_score(x as u8, second_maki_roll_points_each, scores);
				}
			}
		}
	}
}

// Puts the ID's of the players whe had the most maki rolls in most_maki_rolls and the second most in second_most_maki_rolls.
fn find_most_maki_rolls(results: &[&SushiResult], most_maki_rolls: &mut Vec<u8>, second_most_maki_rolls: &mut Vec<u8>)
{
	let mut highest_maki_roll_count = 0;
	let mut second_highest_maki_roll_count = 0;

	// Work out the highest and second highest counts.
	for x in results.iter()
	{
		let player_maki_roll_count = x.get_maki_roll_count();

		if player_maki_roll_count > highest_maki_roll_count
		{
			highest_maki_roll_count = player_maki_roll_count;
			second_highest_maki_roll_count = highest_maki_roll_count;
		}
	}

	// Add players who had those totals to the appropriate vectors.
	for (i, x) in results.iter().enumerate()
	{
		if x.get_maki_roll_count() == highest_maki_roll_count
		{
			most_maki_rolls.push(i as u8);
		}
		else if x.get_maki_roll_count() == second_highest_maki_roll_count
		{
			second_most_maki_rolls.push(i as u8);
        }
	}
}
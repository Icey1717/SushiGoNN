// An attribute to hide warnings for unused code.
#![allow(dead_code)]
//#![feature(test)]

#[macro_use] extern crate log;

 //extern crate test;
 //use test::Bencher;

extern crate rand;
extern crate random_choice;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rayon;
extern crate rulinalg;

mod neural_network_game;
use neural_network_game::*;

mod competition;

mod gen_algorithm;
use gen_algorithm::*;

use std::io;

fn main() 
{
	println!("Type 'Play' to play against the last generated ai.");

    println!("Type 'Create' to create a new ai.");

    println!("Type 'Random' to play an AI against a random opponent.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

	match guess.trim()
	{
		"Play" => play_player_game(),
		"Create" => picked_generational(),
		"Random" => setup_random_game(),
		_ => println!("You didn't enter 'Play' or 'Create'. These are your only options, don't try and find anything else, there isn't anything to find."),
	}
}

//fn picked_generational()
//{
	//run_generational(64, 1000);
//}

fn picked_generational()
{
	run_generational(64, 1000);
}
// #[bench]
// fn bench_seq(b: &mut Bencher) 
// {
//     b.iter(|| 
// 	{

//     });
// }


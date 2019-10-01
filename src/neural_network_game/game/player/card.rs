use std::fmt;

#[derive(Copy,Clone)]
pub enum Card 
{
    Pudding,
	Dumpling,
	Sashimi,
	Tempura,
	MakiRoll1,
	MakiRoll2,
	MakiRoll3,
	SalmonNigri,
	EggNigri,
	SquidNigri,
	Wasabi,

	// La Combination
	WasabiSalmonNigri,
	WasabiEggNigri,
	WasabiSquidNigri,
	UsedWasabi,

	// Hashi - Chopsticks turn from fresh to normal when used
	// They are refreshed at the start of every turn.
	Chopsticks,
	FreshChopsticks,

	None,
	CardMax,
}

impl fmt::Display for Card 
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		let printable = match *self 
		{
            Card::Pudding => "Pudding",
            Card::Dumpling => "Dumpling",
			Card::Sashimi => "Sashimi",
            Card::Tempura => "Tempura",
			Card::MakiRoll1 => "Maki Roll 1",
			Card::MakiRoll2 => "Maki Roll 2",
			Card::MakiRoll3 => "Maki Roll 3",
            Card::Chopsticks => "Chopsticks",
			Card::SalmonNigri => "Salmon Nigri",
            Card::EggNigri => "Egg Nigri",
			Card::SquidNigri => "Squid Nigri",
            Card::Wasabi => "Wasabi",
			Card::WasabiSalmonNigri => "Salmon Nigri with Wasabi",
			Card::WasabiEggNigri => "Egg Nigri with Wasabi",
			Card::WasabiSquidNigri => "Squid Nigri with Wasabi",
			Card::UsedWasabi => "Used Wasabi",
			Card::FreshChopsticks => "Fresh Chopsticks",
			Card::None => "None",
			Card::CardMax => "Warning",
        };
        write!(f, "{}", printable)
	}
}

impl std::cmp::PartialEq for Card
{
	fn eq (&self, other: &Card) -> bool
	{
		*self as usize == *other as usize
	}
}

impl From<usize> for Card
{
	fn from(item: usize) -> Card
	{
		match item
		{
			0 => Card::Pudding,
			1 => Card::Dumpling,
			2 => Card::Sashimi,
            3 => Card::Tempura,
			4 => Card::MakiRoll1,
			5 => Card::MakiRoll2,
			6 => Card::MakiRoll3,
            7 => Card::Chopsticks,
			8 => Card::SalmonNigri,
            9 => Card::EggNigri,
			10 => Card::SquidNigri,
            11 => Card::Wasabi,
			12 => Card::WasabiSalmonNigri,
			13 => Card::WasabiEggNigri,
			14 => Card::WasabiSquidNigri,
			15 => Card::UsedWasabi,
			16 => Card::FreshChopsticks,
			17 => Card::None,
			_ => Card::CardMax,
		}
	}
}

pub fn print_cards(cards: &[Card])
{
	//---- Print the cards in the input array
	for (i, x) in cards.iter().enumerate()
	{
		println!("{0}: {1}",i,  x);
	}
}
use std::fmt;

#[derive(Copy,Clone,Debug,Enum)]
pub enum Card 
{
    Pudding,
	Dumpling,
	Sashimi,
	Tempura,
	MakiRoll1,
	MakiRoll2,
	MakiRoll3,
	SalmonNigiri,
	EggNigiri,
	SquidNigiri,
	Wasabi,

	// Chopsticks turn from fresh to normal when used
	Chopsticks,
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
			Card::SalmonNigiri => "Salmon Nigri",
            Card::EggNigiri => "Egg Nigri",
			Card::SquidNigiri => "Squid Nigri",
            Card::Wasabi => "Wasabi",
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
			8 => Card::SalmonNigiri,
            9 => Card::EggNigiri,
			10 => Card::SquidNigiri,
            11 => Card::Wasabi,
			12 => Card::None,
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

pub fn get_sprite_filename(card: &Card) -> &str
{
	match card
	{
		Card::Pudding => r"res/images/Pudding_64x64x1.png",
		Card::Dumpling => r"res/images/Dumpling_64x64x1.png",
		Card::Sashimi => r"res/images/Sashimi_64x64x1.png",
		Card::Tempura => r"res/images/Tempura_64x64x1.png",
		Card::MakiRoll1 => r"res/images/Maki_64x64x1.png",
		Card::MakiRoll2 => r"res/images/TwoMaki_64x64x1.png",
		Card::MakiRoll3 => r"res/images/ThreeMaki_64x64x1.png",
		Card::Chopsticks => r"res/images/Chopsticks_64x64x1.png",
		Card::SalmonNigiri => r"res/images/SalmonNigiri_64x64x1.png",
		Card::EggNigiri => r"res/images/EggNigiri_64x64x1.png",
		Card::SquidNigiri => r"res/images/SquidNigiri_64x64x1.png",
		Card::Wasabi => r"res/images/Wasabi_64x64x1.png",
		_ => r"",
	}
}
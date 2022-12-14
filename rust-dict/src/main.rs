// STL
use std::env;
use std::io::Write;
use std::collections::HashSet;
use std::collections::HashMap;

// crates
use serde::Serialize;
use serde::Deserialize;
use serde_json;
use termcolor;
use termcolor::WriteColor;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Meaning
{
	part_of_speech: String,
	definition: String,
	examples: Vec<String>
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Word
{
	name: String,
	meanings: Vec<Meaning>,
	synonyms: Vec<String>,
	antonyms: Vec<String>
}
impl Word
{
	fn print(&self) -> ()
	{
		let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
		let mut red_col = termcolor::ColorSpec::new();
		red_col.set_fg(Some(termcolor::Color::Red));
		let mut green_col = termcolor::ColorSpec::new();
		green_col.set_fg(Some(termcolor::Color::Green));
		let mut cyan_col = termcolor::ColorSpec::new();
		cyan_col.set_fg(Some(termcolor::Color::Cyan));
		let normal_col = termcolor::ColorSpec::new();

		stdout.set_color(&green_col).unwrap();
		write!(&mut stdout, "      ********** {} **********\n\n", self.name).unwrap();

		let mut index = 1;
		let mut part_of_speech = self.meanings[0].part_of_speech.clone();

		stdout.set_color(&red_col).unwrap();
		write!(&mut stdout, "{}:\n", part_of_speech).unwrap();

		for meaning in &self.meanings
		{
			if meaning.part_of_speech != part_of_speech
			{
				part_of_speech = meaning.part_of_speech.clone();
				stdout.set_color(&red_col).unwrap();
				write!(&mut stdout, "\n").unwrap();
				write!(&mut stdout, "{}:\n", part_of_speech).unwrap();
			}

			write!(&mut stdout, "\n").unwrap();
			stdout.set_color(&normal_col).unwrap();
			write!(&mut stdout, " [").unwrap();
			stdout.set_color(&green_col).unwrap();
			write!(&mut stdout, "{}", index).unwrap();
			stdout.set_color(&normal_col).unwrap();
			write!(&mut stdout, "] ").unwrap();

			stdout.set_color(&cyan_col).unwrap();
			write!(&mut stdout, "{}\n", meaning.definition).unwrap();

			index += 1;
		}

		if self.synonyms.len() > 0
		{
			stdout.set_color(&red_col).unwrap();
			write!(&mut stdout, "\n").unwrap();
			write!(&mut stdout, "Synonyms:\n", ).unwrap();
			write!(&mut stdout, "\n").unwrap();

			stdout.set_color(&cyan_col).unwrap();
			write!(&mut stdout, " {}", self.synonyms[0]).unwrap();

			for synonym in &self.synonyms[1..]
			{
				stdout.set_color(&normal_col).unwrap();
				write!(&mut stdout, ", ").unwrap();

				stdout.set_color(&cyan_col).unwrap();
				write!(&mut stdout, "{}", synonym).unwrap();
			}
			write!(&mut stdout, "\n").unwrap();
		}

		if self.antonyms.len() > 0
		{
			stdout.set_color(&red_col).unwrap();
			write!(&mut stdout, "\n").unwrap();
			write!(&mut stdout, "Antonyms:\n", ).unwrap();
			write!(&mut stdout, "\n").unwrap();

			stdout.set_color(&cyan_col).unwrap();
			write!(&mut stdout, " {}", self.antonyms[0]).unwrap();

			for antonym in &self.antonyms[1..]
			{
				stdout.set_color(&normal_col).unwrap();
				write!(&mut stdout, ", ").unwrap();

				stdout.set_color(&cyan_col).unwrap();
				write!(&mut stdout, "{}", antonym).unwrap();
			}
			write!(&mut stdout, "\n").unwrap();
		}

		stdout.reset().unwrap();
	}
}

fn permute(my_str : String) -> HashSet<String>
{
	let mut strings : HashSet<String> = HashSet::new();

	//swap indices
	let chars : Vec<char> = my_str.chars().collect();
	for i in 0..chars.len()-1
	{
		let mut chars_2 = chars.clone();
		chars_2.swap(i, i+1);
		strings.insert(chars_2.into_iter().collect());
	}

	//remove a char
	for i in 0..chars.len()
	{
		let mut chars_2 = chars.clone();
		chars_2.remove(i);
		strings.insert(chars_2.into_iter().collect());
	}

	//dup a char
	for i in 0..chars.len()
	{
		let mut chars_2 = chars.clone();
		let my_char = chars_2[i];
		chars_2.insert(i, my_char);
		strings.insert(chars_2.into_iter().collect());
	}

	//replace a char
	for i in 0..chars.len()
	{
		let mut chars_2 = chars.clone();
		for ch in 'A'..'Z'
		{
			chars_2[i] = ch;
			strings.insert(chars_2.clone().into_iter().collect());
		}
	}

	//insert a char
	for i in 0..chars.len()
	{
		for ch in 'A'..'Z'
		{
			let mut chars_2 = chars.clone();
			chars_2.insert(i, ch);
			strings.insert(chars_2.into_iter().collect());
		}
	}

	strings.remove(&my_str);
	return strings;
}

fn main()
{
	let foodata = include_str!("..\\dict.json");
	let dictionary : HashMap<String, Word> = serde_json::from_str(&foodata).unwrap();

	let args : Vec<String> = env::args().collect();

	let word = dictionary.get(&args[1]);

	let strs = permute(args[1].clone());
	let mut strs_2 : Vec<String> = vec![];
	for str in strs
	{
		let word = dictionary.get(&str);
		if word.is_some()
		{
			strs_2.push(str.clone());
		}
	}

	let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
	let mut red_col = termcolor::ColorSpec::new();
	red_col.set_fg(Some(termcolor::Color::Red));
	let mut green_col = termcolor::ColorSpec::new();
	green_col.set_fg(Some(termcolor::Color::Green));
	let mut cyan_col = termcolor::ColorSpec::new();
	cyan_col.set_fg(Some(termcolor::Color::Cyan));
	let normal_col = termcolor::ColorSpec::new();

	if word.is_some()
	{
		word.unwrap().print();

		if strs_2.len() > 0
		{
			stdout.set_color(&red_col).unwrap();
			write!(&mut stdout, "\n").unwrap();
			write!(&mut stdout, "See also:\n", ).unwrap();
			write!(&mut stdout, "\n").unwrap();

			stdout.set_color(&cyan_col).unwrap();
			write!(&mut stdout, " {}", strs_2[0]).unwrap();

			for related_word in &strs_2[1..]
			{
				stdout.set_color(&normal_col).unwrap();
				write!(&mut stdout, ", ").unwrap();

				stdout.set_color(&cyan_col).unwrap();
				write!(&mut stdout, "{}", related_word).unwrap();
			}
			stdout.set_color(&normal_col).unwrap();
			write!(&mut stdout, "\n").unwrap();
		}
	}
	else
	{
		if strs_2.len() == 0
		{
			println!("Could not find \"{}\"", &args[1]);
		}
		else
		{
			let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
			stdout.set_color(&red_col).unwrap();
			write!(&mut stdout, "Did you mean: ").unwrap();
			stdout.set_color(&cyan_col).unwrap();
			write!(&mut stdout, "{}", strs_2[0]).unwrap();
			for str in &strs_2[1..]
			{
				stdout.reset().unwrap();
				write!(&mut stdout, ", ").unwrap();
				stdout.set_color(&cyan_col).unwrap();
				write!(&mut stdout, "{}", str).unwrap();
			}
			stdout.set_color(&normal_col).unwrap();
			write!(&mut stdout, "\n").unwrap();
		}
	}

	stdout.reset().unwrap();
}

use crate::alloc_prelude::*;

pub const PATH_SEPARATOR: char = '/';

pub fn normalize(path: &str) -> String {
	let mut portions: Vec<String> = Vec::new();
	let mut chars = path.chars().collect::<Vec<char>>();

	// Trim trailing separators
	if chars.first() == Some(&PATH_SEPARATOR) && chars.len() > 1 {
		while chars.last() == Some(&PATH_SEPARATOR) {
			chars.pop();
		}

		// And then add a final separator (which gets stripped out during
		// the rejoin later on, meaning the final segment gets the same
		// checks as the other segments)
		chars.push('/');
	}

	// Trim leading separators
	while chars.first() == Some(&PATH_SEPARATOR) {
		chars.remove(0);
	}

	// Join chars into string parts
	let mut current = String::new();
	for c in chars.iter() {
		if *c == PATH_SEPARATOR {
			match current.as_str() {
				// Ignore single-period and empty segments
				"" | "." => {}

				// Remove previous on double-period
				".." => {
					portions.pop();
				}

				// Keep everything else
				_ => {
					portions.push(current);
				}
			}

			current = String::new();
		} else {
			current.push(*c);
		}
	}

	// And then join string parts into final path
	String::from(PATH_SEPARATOR) + &portions.join(&String::from(PATH_SEPARATOR))
}

pub fn split(path: &str) -> Vec<String> {
	normalize(path)
		.split(PATH_SEPARATOR)
		.filter(|x| x != &"")
		.map(String::from)
		.collect()
}

pub fn join(parts: &[String]) -> String {
	let sep = String::from(PATH_SEPARATOR);
	let npath = parts.join(&sep) + &sep;
	normalize(&npath)
}

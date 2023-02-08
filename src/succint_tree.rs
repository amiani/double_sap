#![allow(dead_code)]

use bitintr::{Tzcnt, Bzhi};

pub trait SuccintTree: Sized {
	fn get_levels(&self) -> &Vec<Vec<u64>>;
	fn get_levels_mut(&mut self) -> &mut Vec<Vec<u64>>;

	fn new(capacity: u32) -> Self;

	fn insert(&mut self, index: usize) {
		for level in 0..self.get_levels().len() {
			let level_index = calc_level_index(level as u32, index);
			set(self, level, level_index);
		}
	}

	fn remove(&mut self, index: usize) {
		for level in 0..self.get_levels().len() {
			let level_index = calc_level_index(level as u32, index);
			let word = unset(self, level, level_index);
			if word == 0 { return; }
		}
	}

	fn range(&self, lower: usize, upper: usize) -> Vec<usize> {
		let mut elements = Vec::<usize>::new();
		let mut x = lower;
		while let Some(successor) = find_successor(self, x) {
			if x >= upper {
				break;
			}
			elements.push(successor);
			x = successor;
		}
		elements
	}

	fn min(&self) -> Option<usize> {
		let final_level = self.get_levels().last().unwrap();
		let word = final_level[0];
		if word & 0b1 == 1 {
			return Some(0);
		}
		find_successor(self, 0)
	}

}

fn calc_level_index(level: u32, index: usize) -> usize {
	index / 64usize.pow(level as u32)
}

fn set<T: SuccintTree>(tree: &mut T, level: usize, index: usize) {
	let word_index = index / 64;
	let bit_index = index % 64;
	let word = &mut tree.get_levels_mut()[level][word_index];
	*word |= 1 << bit_index;
}

fn unset<T: SuccintTree>(tree: &mut T, level: usize, index: usize) -> u64 {
	let word_index = index / 64;
	let bit_index = index % 64;
	let word = &mut tree.get_levels_mut()[level][word_index];
	*word &= !(1 << bit_index);
	*word
}

fn find_successor<T: SuccintTree>(tree: &T, x: usize) -> Option<usize> {
	let (level, index) = find_ancestor_sibling(tree, x)?;
	Some(get_least_descendant(tree, level, index))
}

fn find_ancestor_sibling<T: SuccintTree>(tree: &T, x: usize) -> Option<(usize, usize)> {
	for level in 0..tree.get_levels().len() {
		let level_index = calc_level_index(level as u32, x);
		let word_index = level_index / 64;
		let word = tree.get_levels()[level][word_index];
		if let Some(sibling) = next_sibling(word, (level_index % 64) as u32) {
			let sibling_index = sibling + 64 * word_index;
			return Some((level, sibling_index));
		}
	}
	None
}

fn next_sibling(word: u64, x: u32) -> Option<usize> {
	let clear_low = word
		.reverse_bits()
		.bzhi(63u32 - x)
		.reverse_bits();
	if clear_low == 0 {
		None
	} else {
		Some(clear_low.trailing_zeros() as usize)
	}
}

fn get_least_descendant<T: SuccintTree>(tree: &T, level: usize, level_index: usize) -> usize {
	if level == 0 {
		return level_index;
	}
	let word_index = level_index;
	let word = tree.get_levels()[level-1][word_index];
	let zeros = word.tzcnt() as usize;
	let next_level_index = level_index * 64 + zeros;
	get_least_descendant(tree, level - 1, next_level_index)
}

fn div_up(a: usize, b: usize) -> usize {
    a / b + (a % b != 0) as usize
}


pub struct Set {
	levels: Vec<Vec<u64>>,
}

impl SuccintTree for Set {
	fn new(capacity: u32) -> Self {
		let capacity = f64::from(capacity);
		let num_levels = capacity.log(64.0).ceil() as u32;
		let n = 64usize.pow(num_levels);
		Self {
			levels: (0..num_levels)
				.map(|d| vec![0; div_up(n, 64usize.pow((d + 1) as u32))])
				.collect()
		}
	}

	fn get_levels(&self) -> &Vec<Vec<u64>> {
		&self.levels
	}
	fn get_levels_mut(&mut self) -> &mut Vec<Vec<u64>> {
		&mut self.levels
	}
}

pub struct LinkedSet {
	levels: Vec<Vec<u64>>,
	nexts: Vec<u64>,
}

/*
impl SuccintTree for LinkedSuccintSet {
	fn new(capacity: u32) -> Self {
		let capacity = f64::from(capacity);
		let num_levels = capacity.log(64.0).ceil() as u32;
		let n = 64usize.pow(num_levels);
		Self {
			levels: (0..num_levels)
				.map(|d| vec![0; div_up(n, 64usize.pow((d + 1) as u32))])
				.collect(),
			nexts: 
		}
	}

	fn get_levels(&self) -> &Vec<Vec<u64>> {
		&self.levels
	}
	fn get_levels_mut(&mut self) -> &mut Vec<Vec<u64>> {
		&mut self.levels
	}
}
*/

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn its_last_level_is_one_word() {
		let tree = Set::new(100);
		assert_eq!(tree.levels.last().unwrap().len(), 1);
	}

	#[test]
	fn it_updates_parents_of_4097() {
		let mut tree = Set::new(4097);
		tree.insert(4097);

		let word_index = 4097 / 64;
		assert_eq!(tree.levels[0][word_index], 2);
		let word_index = word_index / 64;
		assert_eq!(tree.levels[1][word_index], 1);
	}

	#[test]
	fn it_finds_the_successor_in_the_same_word() {
		let mut tree = Set::new(100);
		tree.insert(35);

		let successor = find_successor(&tree, 4).unwrap();
		
		assert_eq!(successor, 35);
	}

	#[test]
	fn it_finds_the_successor_in_a_different_word() {
		let mut tree = Set::new(100);
		tree.insert(68);

		let successor = find_successor(&tree, 4).unwrap();
		
		assert_eq!(successor, 68);
	}

	#[test]
	fn successor_returns_none_when_there_is_no_succesor() {
		let tree = Set::new(100);
		let successor = find_successor(&tree, 4);
		assert_eq!(successor, None);
	}

	#[test]
	fn it_finds_the_range_when_all_in_same_word() {
		let mut tree = Set::new(100);
		let range = vec![4, 23, 28, 37, 60];
		for &x in &range {
			tree.insert(x);
		}

		let result = tree.range(3, 62);

		assert_eq!(result, range);
	}

	#[test]
	fn it_finds_the_range_when_in_different_words() {
		let mut tree = Set::new(262_145);
		let range = vec![4, 65, 4097, 262_145];
		for &x in &range {
			tree.insert(x);
		}

		let result = tree.range(3, 262_146);

		assert_eq!(result, range);
	}

	#[test]
	fn it_finds_the_min_zero() {
		let mut set = Set::new(65);
		set.insert(0);
		set.insert(1);
		set.insert(64);
		set.insert(65);
		assert_eq!(set.min().unwrap(), 0);
	}

	#[test]
	fn it_finds_the_min_in_second_word() {
		let mut set = Set::new(65);
		set.insert(65);
		set.insert(79);
		assert_eq!(set.min().unwrap(), 65);
	}
}
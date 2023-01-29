#![allow(dead_code)]

use bitintr::{Tzcnt, Rbit, Bzhi};
pub struct SuccintTree {
	levels: Vec<Vec<u64>>,
}

impl SuccintTree {
	pub fn new(capacity: u32) -> Self {
		let capacity = f64::from(capacity);
		let num_levels = capacity.log(64.0).ceil() as u32;
		let n = 64usize.pow(num_levels);
		Self {
			levels: (0..num_levels)
				.map(|d| vec![0; div_up(n, 64usize.pow((d + 1) as u32))])
				.collect()
		}
	}

	pub fn insert(&mut self, index: usize) {
		for level in 0..self.levels.len() {
			let level_index = Self::calc_level_index(level as u32, index);
			self.set(level, level_index);
		}
	}

	fn set(&mut self, level: usize, index: usize) {
		let word_index = index / 64;
		let bit_index = index % 64;
		let word = &mut self.levels[level][word_index];
		*word |= 1 << bit_index
	}

	pub fn remove(&mut self, index: usize) {
		for level in 0..self.levels.len() {
			let level_index = Self::calc_level_index(level as u32, index);
			let word = self.unset(level, level_index);
			if word == 0 { return; }
		}
	}

	fn calc_level_index(level: u32, index: usize) -> usize {
		index / 64usize.pow(level as u32)
	}

	fn unset(&mut self, level: usize, index: usize) -> u64 {
		let word_index = index / 64;
		let bit_index = index % 64;
		let word = &mut self.levels[level][word_index];
		*word &= !(1 << bit_index);
		*word
	}

	pub fn range(&self, lower: usize, upper: usize) -> Vec<usize> {
		let mut elements = Vec::<usize>::new();
		let x = lower;
		while let Some(x) = self.find_successor(x) {
			if x < upper {
				elements.push(x);
				break;
			}
		}
		elements
	}

	fn find_successor(&self, x: usize) -> Option<usize> {
		let (level, index) = self.find_ancestor_sibling(x)?;
		Some(self.get_least_descendant(level, index))
	}

	fn find_ancestor_sibling(&self, x: usize) -> Option<(usize, usize)> {
		for level in 0..self.levels.len() {
			let level_index = Self::calc_level_index(level as u32, x);
			let word_index = level_index / 64;
			let word = self.levels[level][word_index];
			if let Some(sibling) = Self::next_sibling(word, (level_index % 64) as u32) {
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

	fn get_least_descendant(&self, level: usize, level_index: usize) -> usize {
		4
	}
}

fn div_up(a: usize, b: usize) -> usize {
    a / b + (a % b != 0) as usize
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn its_last_level_is_one_word() {
		let tree = SuccintTree::new(100);
		assert_eq!(tree.levels.last().unwrap().len(), 1);
	}

	#[test]
	fn it_updates_parents_of_4097() {
		let mut tree = SuccintTree::new(4097);
		tree.insert(4097);

		let word_index = 4097 / 64;
		assert_eq!(tree.levels[0][word_index], 2);
		let word_index = word_index / 64;
		assert_eq!(tree.levels[1][word_index], 1);
	}

	#[test]
	fn it_finds_the_successor_in_the_same_word() {
		let mut tree = SuccintTree::new(100);
		tree.insert(35);

		let successor = tree.find_successor(4).unwrap();
		
		assert_eq!(successor, 35);
	}

	#[test]
	fn it_finds_the_successor_in_a_different_word() {
		let mut tree = SuccintTree::new(100);
		tree.insert(68);

		let successor = tree.find_successor(4).unwrap();
		
		assert_eq!(successor, 68);
	}

	#[test]
	fn successor_returns_none_when_there_is_no_succesor() {
		let tree = SuccintTree::new(100);
		let successor = tree.find_successor(4);
		assert_eq!(successor, None);
	}

	#[test]
	fn it_finds_the_range() {
		let mut tree = SuccintTree::new(100);
		let range = vec![4, 23, 28, 37, 60];
		for &x in &range {
			tree.insert(x);
		}

		let result = tree.range(3, 62);

		assert_eq!(range, result);
	}
}
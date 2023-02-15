#![allow(dead_code)]
use crate::{succint_tree::{Set, SuccintTree}, radix_sort::RadixSort};

type Real = f64;

pub trait Aabb {
	fn project_x(&self) -> (Real, Real);
	fn project_y(&self) -> (Real, Real);
	fn project_z(&self) -> (Real, Real);
}

pub fn sweep_and_prune<A: Aabb>(boxes: &[A]) -> Vec<(&A, &A)> {
	let extents_x: Vec<(_, _)> = boxes.iter()
		.map(Aabb::project_x)
		.collect();
	let sorted_indexes_x = unzip_extents(&extents_x).argsort();
	let boundaries_x = find_boundaries(&sorted_indexes_x);

	let extents_y: Vec<(_, _)> = boxes.iter()
		.map(Aabb::project_y)
		.collect();
	let sorted_indexes_y = unzip_extents(&extents_y).argsort();
	find_pairs(&sorted_indexes_y, &boundaries_x, boxes)
}

fn unzip_extents(extents: &[(Real, Real)]) -> Vec<Real> {
	let (left, right): (Vec<_>, Vec<_>) = extents.iter().copied().unzip();
	[left, right].concat()
}

struct CandidateBounds {
	lower: Vec<usize>,
	upper: Vec<usize>,
	ranks: Vec<usize>,
	ranks_inv: Vec<usize>
}

impl CandidateBounds {
	pub fn new(len: usize) -> Self {
		Self {
			lower: vec![0usize; len],
			upper: vec![0usize; len],
			ranks: vec![0usize; len],
			ranks_inv: vec![0usize; len],
		}
	}
}

fn find_boundaries(indexes: &[usize]) -> CandidateBounds {
	let num_boxes = indexes.len() / 2;
	let mut active_boxes = Set::new(num_boxes as u32);
	let mut rank = 0;
	let mut bounds = CandidateBounds::new(num_boxes);
	for &index in indexes {
		if index < num_boxes {
			let box_id = index;
			bounds.ranks[box_id] = rank;
			bounds.ranks_inv[rank] = box_id;
			let box_rank = rank;
			active_boxes.insert(box_rank);
			bounds.lower[box_id] = active_boxes.min().unwrap();	//TODO: remove this unwrap?
			rank += 1;
		} else {
			let box_id = index - num_boxes;
			bounds.upper[box_id] = rank;
			let box_rank = bounds.ranks[box_id];
			active_boxes.remove(box_rank);
		}
	}
	bounds
}

fn find_pairs<'a, A: Aabb>(sorted_indexes: &[usize], bounds: &CandidateBounds, boxes: &'a [A]) -> Vec<(&'a A, &'a A)> {
	let num_boxes = sorted_indexes.len() / 2;
	let mut active_ranks = Set::new(num_boxes as u32);
	let mut pairs = Vec::<(&A, &A)>::new();
	for &index in sorted_indexes {
		if index < num_boxes {
			let object = &boxes[index];
			let are_colliding = |right: &&A| {
				let left_extent = object.project_z();
				let right_extent = right.project_z();
				left_extent.1 > right_extent.0 && left_extent.0 < right_extent.1
			};

			let candidates = active_ranks.range(bounds.lower[index], bounds.upper[index]);
			let colliding_objects = candidates.iter()
				.map(|&c| bounds.ranks_inv[c])
				.map(|index| &boxes[index])
				.filter(are_colliding)
				.map(|colliding_object| (object, colliding_object));
			pairs.extend(colliding_objects);

			let rank = bounds.ranks[index];
			active_ranks.insert(rank);
		} else {
			let rank = bounds.ranks[index - num_boxes];
			active_ranks.remove(rank);
		}
	}
	pairs
}

#[cfg(test)]
mod tests {
	use super::*;
	
	struct BoundingBox {
		position: (f64, f64, f64),
		width: f64,
		length: f64,
		height: f64
	}

	impl Aabb for BoundingBox {
		fn project_x(&self) -> (f64, f64) {
			(self.position.0, self.position.0 + self.width)
		}
		fn project_y(&self) -> (f64, f64) {
			(self.position.1, self.position.1 + self.length)
		}
		fn project_z(&self) -> (f64, f64) {
			(self.position.2, self.position.2 + self.height)
		}
	}

	#[test]
	fn it_finds_two_colliding_boxes() {
		let boxes = [
			BoundingBox {
				position: (1.0, 2.0, 0.0),
				width: 1.0,
				length: 1.0,
				height: 1.0,
			},
			BoundingBox {
				position: (1.5, 2.5, 0.0),
				width: 1.0,
				length: 1.0,
				height: 1.0,
			},
		];

		let colliding_pairs = sweep_and_prune(&boxes);

		assert!(std::ptr::eq(&boxes[0], colliding_pairs[0].1));
		assert!(std::ptr::eq(&boxes[1], colliding_pairs[0].0));
	}
}
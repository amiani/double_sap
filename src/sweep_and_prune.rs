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
	let endpoints_x = unzip_extents(&extents_x);
	let sorted_x = endpoints_x.as_slice().argsort();
	let boundaries_x = find_boundaries(&sorted_x);

	let extents_y: Vec<(_, _)> = boxes.iter()
		.map(Aabb::project_y)
		.collect();
	let endpoints_y = unzip_extents(&extents_y);
	let sorted_y = endpoints_y.as_slice().argsort();
	let colliding_pairs = find_collisions(&sorted_y, &boundaries_x);
	colliding_pairs.iter()
		.map(|pair| (&boxes[pair.0], &boxes[pair.1]))
		.collect()
}

fn unzip_extents(extents: &[(Real, Real)]) -> Vec<Real> {
	let (left, right): (Vec<_>, Vec<_>) = extents.iter().copied().unzip();
	[left, right].concat()
}

struct Boundaries {
	lower: Vec<usize>,
	upper: Vec<usize>,
	rank: Vec<usize>,
	rank_inv: Vec<usize>
}

impl Boundaries {
	pub fn new(len: usize) -> Self {
		Self {
			lower: vec![0usize; len],
			upper: vec![0usize; len],
			rank: vec![0usize; len],
			rank_inv: vec![0usize; len],
		}
	}
}

fn find_boundaries(indexes: &[usize]) -> Boundaries {
	let num_boxes = indexes.len() / 2;
	let mut active_boxes = Set::new(num_boxes as u32);
	let mut rank = 0;
	let mut boundaries = Boundaries::new(num_boxes);
	for &index in indexes {
		if index < num_boxes {
			let box_id = index;
			boundaries.rank[box_id] = rank;
			boundaries.rank_inv[rank] = box_id;
			let box_rank = rank;
			active_boxes.insert(box_rank);
			boundaries.lower[box_id] = active_boxes.min().unwrap();	//TODO: remove this unwrap?
			rank += 1;
		} else {
			let box_id = index - num_boxes;
			boundaries.upper[box_id] = rank;
			let box_rank = boundaries.rank[box_id];
			active_boxes.remove(box_rank);
		}
	}
	boundaries
}

fn find_collisions(sorted_indexes: &[usize], boundaries: &Boundaries) -> Vec<(usize, usize)> {
	unimplemented!()
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

		assert!(std::ptr::eq(&boxes[0], colliding_pairs[0].0));
		assert!(std::ptr::eq(&boxes[1], colliding_pairs[0].1));
	}
}
#![allow(dead_code)]
use crate::{succinct_tree::{Set, SuccinctTree}, radix_sort::RadixSort};

type Real = f64;

pub trait Volume {
	fn project_x(&self) -> (Real, Real);
	fn project_y(&self) -> (Real, Real);
	fn project_z(&self) -> (Real, Real);
}

/// Finds collisions between bounding volumes
/// 
pub fn sweep_and_prune<V: Volume>(volumes: &[V]) -> Vec<(&V, &V)> {
	let extents_x: Vec<(_, _)> = volumes.iter()
		.map(Volume::project_x)
		.collect();
	let sorted_indexes_x = unzip_extents(&extents_x).argsort();
	let candidate_bounds_x = find_candidates(&sorted_indexes_x);

	let extents_y: Vec<(_, _)> = volumes.iter()
		.map(Volume::project_y)
		.collect();
	let sorted_indexes_y = unzip_extents(&extents_y).argsort();
	find_pairs(&sorted_indexes_y, &candidate_bounds_x, volumes)
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

fn find_candidates(indexes: &[usize]) -> CandidateBounds {
	let num_volumes = indexes.len() / 2;
	let mut active_volumes = Set::new(num_volumes as u32);
	let mut rank = 0;
	let mut bounds = CandidateBounds::new(num_volumes);
	for &index in indexes {
		if index < num_volumes {
			let volume_id = index;
			bounds.ranks[volume_id] = rank;
			bounds.ranks_inv[rank] = volume_id;
			let volume_rank = rank;
			active_volumes.insert(volume_rank);
			bounds.lower[volume_id] = active_volumes.min().unwrap();	//TODO: remove this unwrap?
			rank += 1;
		} else {
			let volume_id = index - num_volumes;
			bounds.upper[volume_id] = rank;
			let volume_rank = bounds.ranks[volume_id];
			active_volumes.remove(volume_rank);
		}
	}
	bounds
}

fn find_pairs<'a, V: Volume>(sorted_indexes: &[usize], bounds: &CandidateBounds, volumes: &'a [V]) -> Vec<(&'a V, &'a V)> {
	let num_volumes = sorted_indexes.len() / 2;
	let mut active_ranks = Set::new(num_volumes as u32);
	let mut pairs = Vec::<(&V, &V)>::new();
	for &index in sorted_indexes {
		if index < num_volumes {
			let left = &volumes[index];
			let are_colliding = |right: &&V| {
				let left_extent = left.project_z();
				let right_extent = right.project_z();
				left_extent.1 > right_extent.0 && left_extent.0 < right_extent.1
			};

			let candidates = active_ranks.range(bounds.lower[index], bounds.upper[index]);
			let colliding_objects = candidates.iter()
				.map(|&c| bounds.ranks_inv[c])
				.map(|index| &volumes[index])
				.filter(are_colliding)
				.map(|colliding_volume| (left, colliding_volume));
			pairs.extend(colliding_objects);

			let rank = bounds.ranks[index];
			active_ranks.insert(rank);
		} else {
			let rank = bounds.ranks[index - num_volumes];
			active_ranks.remove(rank);
		}
	}
	pairs
}

#[cfg(test)]
mod tests {
	use super::*;
	
	struct BoundingVolume {
		position: (f64, f64, f64),
		width: f64,
		length: f64,
		height: f64
	}

	impl Volume for BoundingVolume {
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
	fn it_finds_two_colliding_volumes() {
		let volumes = [
			BoundingVolume {
				position: (1.0, 2.0, 0.0),
				width: 1.0,
				length: 1.0,
				height: 1.0,
			},
			BoundingVolume {
				position: (1.5, 2.5, 0.0),
				width: 1.0,
				length: 1.0,
				height: 1.0,
			},
		];

		let colliding_pairs = sweep_and_prune(&volumes);

		assert!(std::ptr::eq(&volumes[0], colliding_pairs[0].1));
		assert!(std::ptr::eq(&volumes[1], colliding_pairs[0].0));
	}
}
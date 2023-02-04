use crate::succint_tree::{Set, SuccintTree};

pub trait Aabb {
	type Coord: Copy;
	fn project_x(&self) -> (&Self::Coord, &Self::Coord);
	fn project_y(&self) -> (&Self::Coord, &Self::Coord);
	fn project_z(&self) -> (&Self::Coord, &Self::Coord);
}

//TODO: remove this type in favour of plain functions?
struct Endpoints<Coord>(Vec<Coord>)
	where
		Coord: Copy;

impl<Coord: Copy> Endpoints<Coord> {
	pub fn new(extents: &[(Coord, Coord)]) -> Self {
		let (left, right): (Vec<_>, Vec<_>) = extents.iter().copied().unzip();
		Self([left, right].concat())
	}

	pub fn sort(&self) -> Vec<usize> {
		unimplemented!()
	}
}

pub fn sweep_and_prune<'a, T: Aabb>(boxes: &[&'a T]) -> Vec<(&'a T, &'a T)> {
	let extents_x: Vec<(_, _)> = boxes.iter()
		.map(|b| b.project_x())
		.collect();
	let endpoints_x = Endpoints::new(&extents_x);
	let sorted_x = endpoints_x.sort();
	let boundaries_x = find_boundaries(&sorted_x);

	let extents_y: Vec<(_, _)> = boxes.iter()
		.map(|b| b.project_y())
		.collect();
	let endpoints_y = Endpoints::new(&extents_y);
	let sorted_y = endpoints_y.sort();
	let colliding_pairs = find_collisions(sorted_x, boundaries_x);
	//turn these pairs of indexes into pairs of &T
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
			boundaries.lower[box_id] = active_boxes.min();
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

#[cfg(test)]
mod tests {
	use super::*;


}
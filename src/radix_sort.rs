pub trait RadixSort {
    fn argsort(&self) -> Vec<usize>;
}

impl RadixSort for &[f64] {
    fn argsort(&self) -> Vec<usize> {
        let flipped: Vec<u64> = self.iter().map(|&f| flip_float(f)).collect();
        argsort(&flipped)
    }
}

impl RadixSort for Vec<f64> {
    fn argsort(&self) -> Vec<usize> {
        let flipped: Vec<u64> = self.iter().map(|&f| flip_float(f)).collect();
        argsort(&flipped)
    }
}

fn sort_in_place(arr: &mut [u64]) {
    for i in 0..8 {
        let radix_of = |x| (x as usize >> (i << 3)) & 0xff;
        // Count digit occurrences
        let mut counters = vec![0; 256];
        for &x in arr.iter() {
            counters[radix_of(x)] += 1;
        }

        // Compute last index of each digit
        for i in 1..256 {
            counters[i] += counters[i - 1];
        }
        // Write elements to their new indices
        for &x in arr.to_owned().iter().rev() {
            counters[radix_of(x)] -= 1;
            arr[counters[radix_of(x)]] = x;
        }
    }
}

fn argsort(arr: &[u64]) -> Vec<usize> {
    let mut indexes: Vec<usize> = (0..arr.len()).collect();
    for place in 0..8 {
        let radix_of = |x| (x as usize >> (place << 3)) & 0xff;
        // Count digit occurrences
        let mut counters = vec![0; 256];
        for &index in &indexes {
            let x = arr[index];
            counters[radix_of(x)] += 1;
        }

        // Compute last index of each digit
        for i in 1..256 {
            counters[i] += counters[i - 1];
        }

        for &index in indexes.clone().iter().rev() {
            let x = arr[index];
            counters[radix_of(x)] -= 1;
            indexes[counters[radix_of(x)]] = index;
        }
    }
    indexes
}

fn flip_float(x: f64) -> u64 {
    let bits = x.to_bits();
    let mask = -((bits >> 63) as i64) as u64 | 0x8000000000000000;
    bits ^ mask
}

fn unflip_float(flipped_bits: u64) -> f64 {
    let mask = ((flipped_bits >> 63) as i64 - 1) as u64 | 0x8000000000000000;
    let bits = flipped_bits ^ mask;
    f64::from_bits(bits)
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn in_place_sort_sorts_3_positive_floats() {
		let floats = [5.0, 3.0, 4.0];
		let mut arr: Vec<u64> = floats.into_iter().map(flip_float).collect();
		sort_in_place(&mut arr);
		let sorted_floats: Vec<f64> = arr.into_iter().map(unflip_float).collect();
		assert_eq!(&sorted_floats, &[3.0, 4.0, 5.0]);
	}

    #[test]
    fn in_place_sorts_3_floats() {
		let floats = [4.0, -5.0, -6.0];
		let mut arr: Vec<u64> = floats.into_iter().map(flip_float).collect();
		sort_in_place(&mut arr);
		let sorted_floats: Vec<f64> = arr.into_iter().map(unflip_float).collect();
		assert_eq!(&sorted_floats, &[-6.0, -5.0, 4.0]);
    }

    /*
    #[test]
    fn argsort_sorts_3_positive_integers() {
        let nums = [5, 3, 4];
        let sorted_indexes = 
        assert_eq!(sorted_indexes, vec![1, 2, 0]);
    }
    */

    #[test]
    fn argsort_sorts_3_floats() {
        let floats = [4.0, -5.0, -6.0];
		let sorted_indexes = floats.as_slice().argsort();
		assert_eq!(&sorted_indexes, &[2, 1, 0]);
    }
}
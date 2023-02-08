pub fn radix_sort(arr: &mut [u64]) {
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

pub fn index_sort(arr: &[u64]) -> Vec<usize> {
	unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::radix_sort;

	#[test]
	fn it_sorts_3_floats() {
		let floats = [5.0, 3.0, 4.0];
		let mut arr: Vec<u64> = floats.into_iter().map(f64::to_bits).collect();
		radix_sort(&mut arr);
		let sorted_floats: Vec<f64> = arr.into_iter().map(f64::from_bits).collect();
		assert_eq!(&sorted_floats, &[3.0, 4.0, 5.0]);
	}
}
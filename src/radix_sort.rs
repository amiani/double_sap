pub trait RadixSort {
    fn argsort(&self) -> Vec<usize>;
}

impl RadixSort for &[f64] {
    fn argsort(&self) -> Vec<usize> {
        let flipped: Vec<u64> = self.iter().map(f64::flip).collect();
        argsort(&flipped, 64)
    }
}

impl RadixSort for Vec<f64> {
    fn argsort(&self) -> Vec<usize> {
        let flipped: Vec<u64> = self.iter().map(f64::flip).collect();
        argsort(&flipped, 64)
    }
}

impl RadixSort for &[f32] {
    fn argsort(&self) -> Vec<usize> {
        let flipped: Vec<u32> = self.iter().map(f32::flip).collect();
        argsort(&flipped, 32)
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

fn argsort<Unsigned>(arr: &[Unsigned], size: usize) -> Vec<usize>
    where
        Unsigned: TryInto<usize> + Copy,
        <Unsigned as TryInto<usize>>::Error: std::fmt::Debug,
{
    let mut indexes: Vec<usize> = (0..arr.len()).collect();
    for place in 0..size/8 {
        let radix_of = |x: Unsigned| {
            let x: usize = x.try_into().unwrap();
            (x >> (place << 3)) & 0xff
        };
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

trait Flip {
    type Flipped;
    fn flip(&self) -> Self::Flipped;
    fn unflip(flipped_bits: &Self::Flipped) -> Self;
}

impl Flip for f64 {
    type Flipped = u64;
    fn flip(&self) -> u64 {
        let bits = self.to_bits();
        let mask = -((bits >> 63) as i64) as u64 | 0x8000_0000_0000_0000;
        bits ^ mask
    }

    fn unflip(flipped_bits: &u64) -> Self {
        let mask = ((flipped_bits >> 63) as i64 - 1) as u64 | 0x8000_0000_0000_0000;
        let bits = flipped_bits ^ mask;
        f64::from_bits(bits)
    }
}

impl Flip for f32 {
    type Flipped = u32;
    fn flip(&self) -> u32 {
        let bits = self.to_bits();
        let mask = -((bits >> 31) as i32) as u32 | 0x8000_0000;
        bits ^ mask
    }

    fn unflip(flipped_bits: &u32) -> Self {
        let mask = ((flipped_bits >> 31) as i32 - 1) as u32 | 0x8000_0000;
        let bits = flipped_bits ^ mask;
        f32::from_bits(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn in_place_sort_sorts_3_positive_f64s() {
		let floats = [5.0, 3.0, 4.0];
		let mut arr: Vec<u64> = floats.iter().map(f64::flip).collect();
		sort_in_place(&mut arr);
		let sorted_floats: Vec<f64> = arr.iter().map(f64::unflip).collect();
		assert_eq!(&sorted_floats, &[3.0, 4.0, 5.0]);
	}

    #[test]
    fn in_place_sorts_3_f64s() {
		let floats = [4.0, -5.0, -6.0];
		let mut arr: Vec<u64> = floats.iter().map(f64::flip).collect();
		sort_in_place(&mut arr);
		let sorted_floats: Vec<f64> = arr.iter().map(f64::unflip).collect();
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
    fn argsort_sorts_3_f64s() {
        let floats = [4.0, -5.0, -6.0];
		let sorted_indexes = floats.as_slice().argsort();
		assert_eq!(&sorted_indexes, &[2, 1, 0]);
    }
    #[test]
    fn argsort_sorts_3_f32s() {
        let floats = [4.0f32, -5.0, -6.0];
		let sorted_indexes = floats.as_slice().argsort();
		assert_eq!(&sorted_indexes, &[2, 1, 0]);
    }
}
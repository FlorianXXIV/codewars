use num::pow;

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    slice
        .iter()
        .rev()
        .enumerate()
        .map(
            |(x,y)|
                y*pow(2, x)
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::binary_slice_to_number;

    #[test]
    fn example_tests() {
        assert_eq!(binary_slice_to_number(&vec![0,0,0,1]), 1);
        assert_eq!(binary_slice_to_number(&vec![0,0,1,0]), 2);
        assert_eq!(binary_slice_to_number(&vec![1,1,1,1]), 15);
        assert_eq!(binary_slice_to_number(&vec![0,1,1,0]), 6);
    }
}
#![feature(option_result_unwrap_unchecked)]

pub fn selection_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    for i in 0..arr.len() {
        let min = arr
            .iter()
            .enumerate()
            .skip(i)
            .min_by(|(_, a), (_, b)| a.cmp(b));
        // SAFETY: arr contains at least 1 element in this for loop so min is never None
        let (min_index, _) = unsafe { min.unwrap_unchecked() };
        arr.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[], &[])]
    #[case(&[0], &[0])]
    #[case(&[0, 1], &[0, 1])]
    #[case(&[1, 0], &[0, 1])]
    fn test(#[case] arr: &[i32], #[case] expected: &[i32]) {
        let mut arr_clone = arr.to_vec();
        selection_sort(&mut arr_clone);
        assert_eq!(arr_clone, expected);
    }
}

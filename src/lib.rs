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

pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[i - j - 1] < arr[i - j] {
                break;
            }

            arr.swap(i - j - 1, i - j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    macro_rules! f {
        ( $function:expr ) => {
            ($function as fn(&mut [i32]), stringify!($function))
        };
    }

    #[rstest]
    #[case(&[], &[])]
    #[case(&[0], &[0])]
    #[case(&[0, 1], &[0, 1])]
    #[case(&[1, 0], &[0, 1])]
    #[case(&[0, 1, 0], &[0, 0, 1])]
    #[case(&[0, 10, 5, 2, 3, 9], &[0, 2, 3, 5, 9, 10])]
    fn test(#[case] arr: &[i32], #[case] expected: &[i32]) {
        let functions = &[f!(selection_sort), f!(insertion_sort)];

        for (sort_function, name) in functions {
            let mut arr_clone = arr.to_vec();
            sort_function(&mut arr_clone);
            assert_eq!(arr_clone, expected, "function: {}()", name);
        }
    }
}

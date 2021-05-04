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

pub fn insertion_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[i - j - 1] < arr[i - j] {
                break;
            }

            arr.swap(i - j - 1, i - j);
        }
    }
}

pub fn shell_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    let mut gap = arr.len() / 2;
    while gap > 0 {
        for block_index in 0..gap {
            for i in 1..arr.len() / gap {
                for j in 0..i {
                    if arr[(i - j - 1) * gap + block_index] < arr[(i - j) * gap + block_index] {
                        break;
                    }

                    arr.swap((i - j - 1) * gap, (i - j) * gap);
                }
            }
        }
        gap /= 2;
    }
}

pub fn merge_sort(arr: &mut [i32]) {
    let mut temp = vec![0; arr.len()];
    let mut length = 1;
    while length < arr.len() {
        let mut from = 0;
        while from < arr.len() {
            let mid = usize::min(from + length, arr.len());
            let upper = usize::min(from + 2 * length, arr.len());
            merge(&arr[from..mid], &arr[mid..upper], &mut temp[from..upper]);
            arr[from..upper].copy_from_slice(&temp[from..upper]);
            from += 2 * length;
        }

        length *= 2;
    }
}

fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut temp_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result[temp_index] = left[left_index];
            left_index += 1;
        } else {
            result[temp_index] = right[right_index];
            right_index += 1;
        }

        temp_index += 1;
    }

    while left_index < left.len() {
        result[temp_index] = left[left_index];
        temp_index += 1;
        left_index += 1;
    }
    while right_index < right.len() {
        result[temp_index] = right[right_index];
        temp_index += 1;
        right_index += 1;
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
        // let functions = &[f!(selection_sort), f!(insertion_sort), f!(shell_sort)];
        //
        // for (sort_function, name) in functions {
        //     let mut arr_clone = arr.to_vec();
        //     sort_function(&mut arr_clone);
        //     assert_eq!(arr_clone, expected, "function: {}()", name);
        // }
        let mut arr_clone = arr.to_vec();
        let length = arr_clone.len();
        merge_sort(&mut arr_clone);
        assert_eq!(arr_clone, expected);
    }
}

pub fn selection_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    for i in 0..arr.len() {
        let mut min_index = i;
        for (index, item) in arr.iter().enumerate().skip(i) {
            if item < &arr[min_index] {
                min_index = index;
            }
        }
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
    // Copied from https://chercher.tech/rust/merge-sort-rust

    let mut temp = vec![0; arr.len()];
    let mut half_length = 1;
    while half_length < arr.len() {
        for i in 0..=arr.len() / (half_length * 2) {
            let from = i * half_length * 2;
            let mid = usize::min(from + half_length, arr.len());
            let upper = usize::min(from + 2 * half_length, arr.len());
            merge(&arr[from..mid], &arr[mid..upper], &mut temp[from..upper]);
            arr[from..upper].copy_from_slice(&temp[from..upper]);
        }

        half_length *= 2;
    }
}

fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut result_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result[result_index] = left[left_index];
            left_index += 1;
        } else {
            result[result_index] = right[right_index];
            right_index += 1;
        }

        result_index += 1;
    }

    if left_index < left.len() {
        result[result_index..].copy_from_slice(&left[left_index..]);
    } else if right_index < right.len() {
        result[result_index..].copy_from_slice(&right[right_index..]);
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
        let functions = &[
            f!(selection_sort),
            f!(insertion_sort),
            f!(shell_sort),
            f!(merge_sort),
        ];

        for (sort_function, name) in functions {
            let mut arr_clone = arr.to_vec();
            sort_function(&mut arr_clone);
            assert_eq!(arr_clone, expected, "function: {}()", name);
        }
    }
}

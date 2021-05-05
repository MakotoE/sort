extern crate test;

use crate::selection_sort;
use rand::distributions::Standard;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use test::Bencher;

fn shuffled_16() -> [i32; 16] {
    SmallRng::seed_from_u64(69).gen()
}

fn sorted_16() -> [i32; 16] {
    let mut array = [0; 16];
    array.copy_from_slice(&(0..16).collect::<Vec<i32>>());
    array
}

fn thousand() -> [i32; 1024] {
    let mut array = [0; 1024];
    for (i, n) in SmallRng::seed_from_u64(69)
        .sample_iter(Standard)
        .take(1024)
        .enumerate()
    {
        array[i] = n
    }
    array
}

#[bench]
fn selection_sort_16_shuffled(b: &mut Bencher) {
    let array = shuffled_16();
    b.iter(|| {
        let mut arr = array;
        selection_sort(&mut arr);
    });
}

#[bench]
fn selection_sort_16_sorted(b: &mut Bencher) {
    let array = sorted_16();
    b.iter(|| {
        let mut arr = array;
        selection_sort(&mut arr);
    });
}

#[bench]
fn selection_sort_1024(b: &mut Bencher) {
    let array = thousand();
    b.iter(|| {
        let mut arr = array;
        selection_sort(&mut arr);
    });
}


use snowflake::{Snowflake, Pcg32};

fn shuffle<T>(v: &mut [T]) {
    let mut rng = Pcg32::default();
    let size = v.len();

    (0..size).for_each(|idx| {
        let i = rng.next_u32() as usize % size;
        v.swap(idx, i);
    });
}

fn main() {
    const N: usize = 10usize;
    let mut snowflake = Snowflake::new();
    let mut arr: [u64; N] = [0u64; N];

    (0..N).for_each(|idx| {
        arr[idx] = snowflake.next();
        println!("{}, {:x}", idx+1, arr[idx]);
    });

    let mut shuffled = arr.clone();
    shuffle(&mut shuffled);
    println!("shuffle");
    (0..N).for_each(|idx| {
        println!("{}, {:x}", idx+1, shuffled[idx]);
    });

    shuffled.sort();
    println!("sort");
    (0..N).for_each(|idx| {
        println!("{}, {:x}, {:x}", idx+1, arr[idx], shuffled[idx]);
    });
}

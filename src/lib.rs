
use std::time::{SystemTime, UNIX_EPOCH};

const MULTIPLIER: u64 = 6364136223846793005;

#[derive(Clone, PartialEq, Eq)]
pub struct Lcg64Xsh32 {
    state: u64,
    increment: u64,
}

pub type Pcg32 = Lcg64Xsh32;

impl Lcg64Xsh32 {
    pub fn new(state: u64, stream: u64) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Lcg64Xsh32::from_state_incr(state, increment)
    }

    pub fn default() -> Self {
        Lcg64Xsh32::new(0xcafef00dd15ea5e5, 1442695040888963407u64 >> 1)
    }

    #[inline]
    fn from_state_incr(state: u64, increment: u64) -> Self {
        let mut pcg = Lcg64Xsh32 { state, increment };
        // Move away from initial value:
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }

    #[inline]
    fn step(&mut self) {
        // prepare the LCG for the next round
        self.state = self
          .state
          .wrapping_mul(MULTIPLIER)
          .wrapping_add(self.increment);
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.step();

        // Output function XSH RR: xorshift high (bits), followed by a random rotate
        // Constants are for 64-bit state, 32-bit output
        const ROTATE: u32 = 59; // 64 - 5
        const XSHIFT: u32 = 18; // (5 + 32) / 2
        const SPARE: u32 = 27; // 64 - 32 - 5

        let rot = (state >> ROTATE) as u32;
        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right(rot)
    }
}

pub struct Snowflake {
    rng: Pcg32,
}

impl Snowflake {
    pub fn new() -> Snowflake {
        Snowflake {
            rng: Pcg32::default()
        }
    }

    #[inline]
    pub fn next(&mut self) -> u64 {
        let timestamp = SystemTime::now()
          .duration_since(UNIX_EPOCH)
          .unwrap()
          .subsec_nanos();
        let rnd = self.rng.next_u32();
        (((timestamp as u64) << 32u64) & 0xFFFFFFFF_00000000u64)
          | ((rnd as u64) & 0x00000000_FFFFFFFFu64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shuffle<T>(v: &mut [T]) {
        let mut rng = Pcg32::default();
        let size = v.len();

        (0..size).for_each(|idx| {
            let i = rng.next_u32() as usize % size;
            v.swap(idx, i);
        });
    }

    #[test]
    fn test_pcg32() {
        let mut rng = Pcg32::default();
        let a = rng.next_u32();
        let b = rng.next_u32();
        println!("{:?}", a);
        println!("{:?}", b);
        assert_ne!(a, b);
    }

    #[test]
    fn test_gen_snowflake_id() {
        let mut snowflake = Snowflake::new();
        let a = snowflake.next();
        let b = snowflake.next();
        println!("{:x}", a);
        println!("{:x}", b);
        assert_ne!(a, b);
    }

    #[test]
    fn test_sort() {
        const N: usize = 10;
        let mut snowflake = Snowflake::new();
        let mut arr: [u64; N] = [0u64; N];

        (0..N).for_each(|idx| {
            arr[idx] = snowflake.next();
            println!("{}, {:x}", idx+1, arr[idx]);
        });

        let mut shuffled = arr.clone();
        println!("shuffle");
        shuffle(&mut shuffled);
        (0..N).for_each(|idx| {
            println!("{}, {:x}", idx+1, shuffled[idx]);
        });

        println!("sort");
        shuffled.sort();
        (0..N).for_each(|idx| {
            println!("{}, {:x}, {:x}", idx+1, arr[idx], shuffled[idx]);
            assert_eq!(arr[idx], shuffled[idx]);
        });
    }
}

//! xoshiro256**. Fine for picking a toy out of the toy box, not for cryptography.

pub struct Rng {
    state: [u64; 4],
}

impl Rng {
    pub fn seeded(seed: u64) -> Self {
        // splitmix64 spreads a weak seed (such as a small integer) across the whole state.
        let mut x = seed;
        let mut next = || {
            x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = x;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        Self {
            state: [next(), next(), next(), next()],
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let s = &mut self.state;
        let result = s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);
        result
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    pub fn range_f64(&mut self, lo: f64, hi: f64) -> f64 {
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        lo + (hi - lo) * self.next_f64()
    }

    /// Inclusive on both ends.
    pub fn range_i64(&mut self, lo: i64, hi: i64) -> i64 {
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        let width = (hi - lo) as u64 + 1;
        lo + (self.next_u64() % width) as i64
    }

    pub fn index(&mut self, len: usize) -> usize {
        (self.next_u64() % len as u64) as usize
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.index(i + 1);
            items.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::seeded(42);
        let mut b = Rng::seeded(42);
        for _ in 0..10 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn floats_stay_in_range() {
        let mut rng = Rng::seeded(7);
        for _ in 0..10_000 {
            let f = rng.next_f64();
            assert!((0.0..1.0).contains(&f));
            let r = rng.range_f64(5.0, 10.0);
            assert!((5.0..10.0).contains(&r));
            let i = rng.range_i64(-3, 3);
            assert!((-3..=3).contains(&i));
        }
    }

    #[test]
    fn shuffle_keeps_every_item() {
        let mut rng = Rng::seeded(1);
        let mut items: Vec<_> = (0..20).collect();
        rng.shuffle(&mut items);
        let mut sorted = items.clone();
        sorted.sort();
        assert_eq!(sorted, (0..20).collect::<Vec<_>>());
    }
}

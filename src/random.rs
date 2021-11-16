#[allow(dead_code)]
pub(crate) enum SeedSource {
    /// Custom user's defined seed.
    Custom(u64),
    /// Uses amount of nanoseconds elapsed from the start of the UNIX_EPOCH.
    Time,
}

/// A simple XOR shift PRNG implementation.
pub(crate) struct Random;

impl Random {
    fn get_seed(seed: SeedSource) -> u64 {
        match seed {
            SeedSource::Time => std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!")
                .as_nanos() as u64,
            SeedSource::Custom(s) => s,
        }
    }

    pub fn random(seed: SeedSource) -> u64 {
        let mut result = Random::get_seed(seed);

        result ^= result << 13;
        result ^= result >> 17;
        result ^= result << 5;

        result
    }

    pub fn random_max(max: u64, seed: SeedSource) -> u64 {
        Random::random(seed) % max
    }
}

#[cfg(test)]
mod tests {
    use crate::random::{Random, SeedSource};
    use std::collections::HashSet;

    #[test]
    fn random_is_really_random() {
        const NUMS: usize = 100;
        let mut numbers = HashSet::with_capacity(NUMS);

        for _ in 0..NUMS {
            numbers.insert(Random::random(SeedSource::Time));
        }

        assert_eq!(numbers.len(), NUMS)
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

// A simple Linear Congruential Generator (LCG)
pub struct Lcg {
    state: u64,
}

impl Lcg {
    // Creates a new LCG seeded with the current system time.
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;
        Self { state: seed }
    }

    // Generates the next pseudo-random u64 number.
    pub fn next_u64(&mut self) -> u64 {
        // These are commonly used parameters for LCGs.
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    /// Generates a random number in the given range [min, max).
    ///
    /// This function works to reduce modulo bias.
    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max, "min must be less than max");
        let range = max - min;
        let mut rand_val = self.next_u64();
        let mut m = rand_val;
        // This loop helps to avoid modulo bias.
        // It rejects values that would cause an uneven distribution.
        if m < range {
            let t = (u64::MAX - range) % range;
            while m < t {
                rand_val = self.next_u64();
                m = rand_val;
            }
        }
        min + (rand_val % range)
    }
}

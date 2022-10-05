pub const BASE: u64 = 256;
pub const PRIME: u64 = 1_000_000_007;

pub struct RollingHash {
    pub base: u64,
    pub prime: u64,
    pub magic: u64,
    pub ibase: Option<u64>,
    pub hash: u64,
}

impl Default for RollingHash {
    fn default() -> Self {
        Self {
            base: BASE,
            prime: PRIME,
            magic: 1,
            ibase: None,
            hash: 0,
        }
    }
}

impl RollingHash {
    /// Check if there is a value for `self.ibase`. In the case of affirmative,
    /// return it. Otherwise, calculate the multiplicative inverse for `self.base`,
    /// update `self.ibase` with that value and return it to the caller.
    /// 
    /// TODO: Take advantage of the extended Euclidean algorithm to improve
    /// this implementation: (Maybe with modinverse crate https://crates.io/crates/modinverse)
    pub fn base_inverse(&mut self) -> u64 {
        match self.ibase {
            Some(i) => i,
            None => {
                for k in 1..self.prime {
                    if (k * self.base) % self.prime == 1 {
                        self.ibase = Some(k);
                        return k;
                    }
                }
                1
            }
        }
    }

    pub fn new(base: u64, prime: u64) -> Self {
        Self {
            base,
            prime,
            ..Default::default()
        }
    }

    pub fn append(&mut self, new: u8) {
        self.hash = (self.hash * self.base + u64::from(new)) % self.prime;
        self.magic = (self.magic * self.base) % self.prime
    }

    pub fn remove(&mut self, old: u8) {
        self.magic = (self.magic * self.base_inverse()) % self.prime;
        self.hash = (self.hash - u64::from(old) * self.magic + self.prime * self.base) % self.prime
    }

    pub fn slide(&mut self, old: u8, new: u8) {
        self.hash =
            (self.hash * self.base - u64::from(old) * self.magic + u64::from(new)) % self.prime;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash_default() {
        let rh = RollingHash::default();

        let asserts = vec![
            (rh.base, BASE, "Default base should be equal to 256"),
            (rh.prime, PRIME, "Default prime should be equal to 1e9 + 7"),
            (rh.magic, 1, "magic should start in 1"),
            (rh.hash, 0, "hash should start in 0"),
        ];

        for pair in asserts {
            assert_eq!(pair.0, pair.1, "{}", pair.2)
        }
    }

    #[test]
    fn test_base_inverse_default() {
        let mut rh = RollingHash::new(2, 31);
        assert_eq!(16, rh.base_inverse())
    }
}

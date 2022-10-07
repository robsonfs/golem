use crate::consts::{BASE, PRIME};

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
                1 // As long as `base` and `prime` are coprime, this will never be returned.
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

pub fn compute_hash(data: &[u8]) -> u64 {
    let mut rh = RollingHash::default();
    for k in data {
        rh.append(*k);
    }

    rh.hash
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
    fn test_rolling_hash_ibase() {
        let rh = RollingHash::default();
        assert!(rh.ibase.is_none(), "ibase should start as a None variant.")
    }

    #[test]
    fn test_rolling_hash_append_empty_hash() {
        let new = 'A' as u8;
        let mut rh = RollingHash::default();

        rh.append(new);
        assert_eq!(65, rh.hash)
    }

    #[test]
    fn test_rolling_hash_append_empty_magic() {
        let new = 'A' as u8;
        let mut rh = RollingHash::default();

        rh.append(new);
        assert_eq!(BASE, rh.magic)
    }

    #[test]
    fn test_compute_hash_empty() {
        let empty_hash = compute_hash(b"");
        assert_eq!(0, empty_hash);
    }

    #[test]
    fn test_compute_hash_bcd() {
        let bcd_hash = compute_hash(b"BCD");
        assert_eq!(4342596, bcd_hash);
    }

    #[test]
    fn test_rolling_hash_slide() {
        let mut rh = RollingHash::default();

        for c in "ABC".as_bytes() {
            rh.append(*c);
        }
        rh.slide('A' as u8, 'D' as u8);

        assert_eq!(4342596, rh.hash);
    }

    #[test]
    fn test_base_inverse() {
        let mut rh = RollingHash::new(2, 31);
        assert_eq!(16, rh.base_inverse())
    }
}

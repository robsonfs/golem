use super::rolling_hash::RollingHash;

pub struct HashStats {
    pub body: String,
    pub k_gram_value: usize,
    pub num_hashes: usize,
    pub hashes: Vec<u64>,
}

impl HashStats {
    /// Creates a new `HashStats` instance
    ///
    /// # Panics
    ///
    /// It will panic either if `k_gram_value` is 0 or `body.len()` < `k_gram_value`
    pub fn new(body: &str, k_gram_value: usize) -> Self {
        assert!(k_gram_value > 0, "K-Gram value should be greater than 0");
        assert!(
            body.len() >= k_gram_value,
            "`body.len()` should be greater than or equal to k_gram_value"
        );

        let num_hashes = if body.len() > k_gram_value {
            body.len() - k_gram_value + 1
        } else {
            1
        };

        Self {
            body: body.to_owned(),
            k_gram_value,
            num_hashes,
            hashes: Vec::with_capacity(num_hashes),
        }
    }

    pub fn get_hashes(&mut self) -> Vec<u64> {
        if self.hashes.is_empty() {
            let bbody = self.body.as_bytes();
            let mut rh = RollingHash::default();
            bbody[..self.k_gram_value]
                .iter()
                .for_each(|k| rh.append(*k));

            self.hashes.push(rh.hash);

            for k in 1..=(self.body.len() - self.k_gram_value) {
                let old_index = k - 1;
                rh.slide(bbody[old_index], bbody[old_index + self.k_gram_value]);
                self.hashes.push(rh.hash);
            }
        }
        self.hashes.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "K-Gram value should be greater than 0")]
    fn test_new_with_invalid_kgram_value() {
        let _ = HashStats::new("Something", 0);
    }

    #[test]
    #[should_panic(expected = "should be greater than or equal to k_gram_value")]
    fn test_new_with_invalid_body_lenth() {
        let _ = HashStats::new("ab", 3);
    }

    #[test]
    fn test_hashes_capacity_one() {
        let stats = HashStats::new("abc", 3);
        assert_eq!(
            stats.hashes.capacity(),
            1,
            "When `body.len()` == `k_gram_value`, `hashes.capacity` should be equal to 1"
        )
    }

    #[test]
    fn test_hashes_capacity() {
        let stats = HashStats::new("o_palmeiras_nao_tem_mundial", 5);
        assert_eq!(
            stats.hashes.capacity(),
            23,
            "capacity should be equal to `body.len() - k_gram_value + 1`"
        )
    }
}

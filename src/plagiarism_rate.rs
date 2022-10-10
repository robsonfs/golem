use crate::{pre_processing::prepare_data, HashStats};
use std::collections::HashSet;

pub fn collect_stats<T: AsRef<str>>(
    first_content: &T,
    second_content: &T,
    lang_code: &str,
    k_gram_value: usize,
) -> (HashStats, HashStats) {
    let content1 = prepare_data(first_content.as_ref(), lang_code);
    let content2 = prepare_data(second_content.as_ref(), lang_code);

    let mut stats1 = HashStats::new(&content1, k_gram_value);
    let mut stats2 = HashStats::new(&content2, k_gram_value);

    let _ = stats1.get_hashes();
    let _ = stats2.get_hashes();

    (stats1, stats2)
}

pub fn calculate_rate(stats1: HashStats, stats2: HashStats) -> f64 {
    let hset_1: HashSet<u64> = HashSet::from_iter(stats1.hashes);
    let hset_2: HashSet<u64> = HashSet::from_iter(stats2.hashes);

    let total_hashes = (stats1.num_hashes + stats2.num_hashes) as f64;
    let num_shared_hashes = hset_1.intersection(&hset_2).count() as f64;

    ((2.0 * num_shared_hashes) / total_hashes) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;

    const FLOAT_TOL: f64 = 0.000_366_210_94;
    const CONTENT_1: &str = r#"
    plagiarism is an act or instance of using or closely imitating the
    language and thoughts of another author without authorization
    "#;
    const CONTENT_2: &str = r#"
    plagiarism is an act of copying the ideas or words of another person
    without giving credit to that person
    "#;

    #[test]
    fn test_calculate_rate_case_1() {
        let (stats1, stats2) = collect_stats(&CONTENT_1, &CONTENT_2, "en", 10);
        let plagi_rate = calculate_rate(stats1, stats2);
        assert!(float_eq!(plagi_rate, 2.41, r2nd <= FLOAT_TOL));
    }

    #[test]
    fn test_calculate_rate_case_2() {
        let (stats1, stats2) = collect_stats(&CONTENT_1, &CONTENT_2, "en", 5);
        let plagi_rate = calculate_rate(stats1, stats2);
        assert!(float_eq!(plagi_rate, 12.90, r2nd <= FLOAT_TOL));
    }
}

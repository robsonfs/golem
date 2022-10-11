use clap::Parser;
use crate::file_ops::read_file_to_string;
use crate::plagiarism_rate::{collect_stats, calculate_rate};

/// Check similarity rate between two files
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// First input file
    #[arg(short, long)]
    pub first_file: String,
    /// Second input file
    #[arg(short, long)]
    pub second_file: String,
    /// Language code (2-letters ISO 639-1 format)
    #[arg(short, long)]
    pub lang_code: String,
    /// K-gram value
    #[arg(short, long)]
    pub kgram_value: usize,
}

impl Args {
    pub fn calculate_similarity(&self) -> Result<f64, std::io::Error> {
        let doc_1 = read_file_to_string(&self.first_file)?;
        let doc_2 = read_file_to_string(&self.second_file)?;

        let (stats1, stats2) = collect_stats(
            &doc_1,
            &doc_2,
            &self.lang_code,
            self.kgram_value,
        );

        Ok(calculate_rate(stats1, stats2))
    }
}

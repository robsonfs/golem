use clap::Parser;
use golem::command_line::Args;

fn main() {
    let args = Args::parse();
    let doc_1 = &args.first_file;
    let doc_2 = &args.second_file;

    println!(
        "The similarity rate between {} and {} is: {}",
        doc_1, doc_2, args.calculate_similarity().unwrap()
    );
}

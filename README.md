
# Golem

**Golem** is a tool written in Rust to help detect plagiarism between two documents. It uses the [Rabbin-Karp algorithm](https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm) to compare two documents and generate a plagiarism rate, as described in [this academic paper](https://www.ijstr.org/final-print/july2017/K-gram-As-A-Determinant-Of-Plagiarism-Level-In-Rabin-karp-Algorithm.pdf).


## Usage

```shell
cargo install --path .
golem --help
Usage: golem --first-file <FIRST_FILE> --second-file <SECOND_FILE> --lang-code <LANG_CODE> --kgram-value <KGRAM_VALUE>

Options:
  -f, --first-file <FIRST_FILE>    First input file
  -s, --second-file <SECOND_FILE>  Second input file
  -l, --lang-code <LANG_CODE>      Language code (2-letters ISO 639-1 format)
  -k, --kgram-value <KGRAM_VALUE>  K-gram value
  -h, --help                       Print help information
  -V, --version                    Print version information

golem -f text_1.txt -s text_2.txt -l en -k 5
The similarity rate between text_1.txt and text_2.txt is: 1.3468013468013467
```

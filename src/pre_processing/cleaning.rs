use super::LANG_CODES;

pub struct CleanedData {
    pub raw_data: String,
    pub language_code: String,
    pub tokenized_data: Vec<String>,
}

impl CleanedData {
    pub fn new<T: AsRef<str> + ?Sized>(raw_data: &T, language_code: &str) -> Self {
        assert!(
            LANG_CODES.binary_search(&language_code).is_ok(),
            "{} is an invalid language code. make sure you are using a 2-letters ISO 639-1 language code",
            &language_code
        );
        Self {
            raw_data: raw_data.as_ref().to_lowercase(),
            language_code: language_code.to_owned(),
            tokenized_data: Vec::new(),
        }
    }

    fn tokenizing(&mut self) {
        self.tokenized_data = self
            .raw_data
            .split_ascii_whitespace()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<String>>();
    }

    fn stop_words_removal(&mut self) {
        let words = stop_words::get(&self.language_code);
        self.tokenized_data
            .retain(|item| !words.contains(&item.to_string()));
    }

    pub fn cleaned_data(&mut self) -> String {
        self.raw_data = self.raw_data.replace('\n', " ");
        self.tokenizing();
        self.stop_words_removal();

        self.tokenized_data.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "make sure you are using a 2-letters ISO 639-1 language code")]
    #[allow(non_snake_case)]
    fn test_create_cleaned_data__language_code() {
        let _ = CleanedData::new("O Rato Roeu a Roupa do Rei de Roma", "portuguese");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_create_cleaned_data__raw_data() {
        let cd = CleanedData::new("A ignorância é a mãe de todas as doenças", "pt");
        assert_eq!(
            cd.raw_data,
            cd.raw_data.to_ascii_lowercase(),
            "`raw_data` should be stored in lowercase"
        )
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_create_cleaned_data__tokenized_data() {
        let cd = CleanedData::new("Ei kysyvä tieltä eksy", "fi");
        assert!(
            cd.tokenized_data.is_empty(),
            "`tokenized_data` should start empty"
        )
    }

    #[test]
    fn test_tokenizing() {
        let mut cd = CleanedData::new("En casa del herrero cuchillo de palo", "es");
        let _ = cd.cleaned_data();
        assert_eq!(
            vec!["casa", "herrero", "cuchillo", "palo"],
            cd.tokenized_data
        );
    }
}

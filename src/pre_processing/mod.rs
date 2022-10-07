// Pre-processing stuffs
mod cleaning;

use cleaning::CleanedData;

pub fn prepare_data(raw_data: &str, language_code: &str) -> String {
    let mut data = CleanedData::new(raw_data, language_code);

    data.cleaned_data()
}

pub const LANG_CODES: [&str; 183] = [
    "aa", "ab", "ae", "af", "ak", "am", "an", "ar", "as", "av", "ay", "az", "ba", "be", "bg", "bi",
    "bm", "bn", "bo", "br", "bs", "ca", "ce", "ch", "co", "cr", "cs", "cu", "cv", "cy", "da", "de",
    "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff", "fi", "fj", "fo", "fr", "fy",
    "ga", "gd", "gl", "gn", "gu", "gv", "ha", "he", "hi", "ho", "hr", "ht", "hu", "hy", "hz", "ia",
    "id", "ie", "ig", "ii", "ik", "io", "is", "it", "iu", "ja", "jv", "ka", "kg", "ki", "kj", "kk",
    "kl", "km", "kn", "ko", "kr", "ks", "ku", "kv", "kw", "ky", "la", "lb", "lg", "li", "ln", "lo",
    "lt", "lu", "lv", "mg", "mh", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "na", "nb", "nd",
    "ne", "ng", "nl", "nn", "no", "nr", "nv", "ny", "oc", "oj", "om", "or", "os", "pa", "pi", "pl",
    "ps", "pt", "qu", "rm", "rn", "ro", "ru", "rw", "sa", "sc", "sd", "se", "sg", "si", "sk", "sl",
    "sm", "sn", "so", "sq", "sr", "ss", "st", "su", "sv", "sw", "ta", "te", "tg", "th", "ti", "tk",
    "tl", "tn", "to", "tr", "ts", "tt", "tw", "ty", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "wa",
    "wo", "xh", "yi", "yo", "za", "zh", "zu",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_codes_is_sorted() {
        let mut lang_codes = LANG_CODES.clone();
        lang_codes.sort_unstable();
        assert_eq!(LANG_CODES, lang_codes, "`LANG_CODES` should be sorted");
    }

    #[test]
    fn test_prepare_data() {
        let cleaned = prepare_data("You have thrown a spanner in the works", "en");
        assert_eq!(
            "thrownspanner",
            cleaned.as_str(),
            "Should return a string without space or stop-words"
        );
    }
}

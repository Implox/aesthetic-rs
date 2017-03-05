extern crate unicode_hfwidth;

use unicode_hfwidth::to_fullwidth;

/// Takes a string and makes it ＡＥＳＴＨＥＴＩＣ in the Vaporwave style.
///
/// Strings passed to this function cannot be used afterwords because once
/// you go aesthetic there is no turning back.
pub fn make_aesthetic(input: String) -> String {
    let output: String = input.chars()
        .map(|x| to_fullwidth(x).unwrap_or(x))
        .collect();
    return output;
}
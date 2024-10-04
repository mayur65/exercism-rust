use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let rev_string = input.graphemes(true).rev().collect();
    return rev_string;
}

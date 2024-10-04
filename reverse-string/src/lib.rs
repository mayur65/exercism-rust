use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    let mut ans_str = input.to_string();
    reverse_grapheme_clusters_in_place(&mut ans_str);
    return ans_str;
}

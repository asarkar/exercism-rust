extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

// We make two passes over the string's contents:
// 1. For each grapheme cluster, reverse the bytes within the grapheme cluster in-place.
// 2. Reverse the bytes of the entire string in-place.
//
// After the second pass, each grapheme cluster has been reversed twice, so,
// its bytes are now back in their original order, but the clusters are now in
// the opposite order within the string.
pub fn reverse(input: &str) -> String {
    let gr_inds =
        UnicodeSegmentation::grapheme_indices(input, true).collect::<Vec<(usize, &str)>>();
    // https://stackoverflow.com/a/71550964/839733
    let mut s_bytes: Vec<u8> = input.as_bytes().to_vec();

    for &(start, _) in &gr_inds {
        let end = gr_inds
            .get(start + 1)
            .map(|(i, _)| i)
            .unwrap_or(&input.len())
            .to_owned();
        s_bytes[start..end].reverse();
    }

    s_bytes.reverse();
    std::str::from_utf8(&s_bytes).unwrap().to_string()
}

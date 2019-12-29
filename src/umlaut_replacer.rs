use regex::Regex;

/// Replace a word within text by another, while ignoring certain ranges of the
/// string
fn replace_ignoring_words(
    text: &String,
    replace_pairs: Vec<(&str, &str)>,
    ignore_ranges: &Vec<std::ops::Range<usize>>,
) -> String {
    let mut edited_text = text.clone(); // create a copy that will be edited
    let mut track_shift = 0; // track if the idx to edit change with respect to the original text
    for (replace_word, replace_with) in replace_pairs {
        if let Ok(re) = Regex::new(replace_word) {
            let replace_shift = replace_with.len() - replace_word.len(); // difference between words that are edited
            
            //walk through text editing every occurence of replace_word
            //while checking that none of the ignore_words are edited
            let mut position = 0;
            'replace_str: loop {
                if let Some(matched_pos) = re.find_at(text, position) {
                    let start_idx = matched_pos.start();
                    let end_idx = matched_pos.end();
                    position = end_idx;
                    // check if bytes that shall be edited are within the ignore range
                    for range in ignore_ranges.iter() {
                        if start_idx >= range.start
                        && end_idx <= range.end {
                            continue 'replace_str;
                        }
                    }
                    // otherwise perform edit
                    edited_text.replace_range(
                        start_idx + track_shift..end_idx + track_shift,
                        replace_with,
                    );
                    track_shift += replace_shift;
                } else {
                    // stop loop once no more match is found
                    break;
                }
            }
        }
    }
    edited_text
}

/// Searches `text` for Umlaute (oe, ae, ue) and replaces with (ö, ä, ü)
/// while ignoring replacements in words given in `ignore_words`
pub fn replace_umlaute(text: &String, ignore_words: &Vec<String>) -> String {
    // extract from ignore_words which ranges are not allowed to edit
    let mut not_allowed_ranges = Vec::new();
    for ign_string in ignore_words.iter() {
        let re_ign = Regex::new(ign_string).unwrap();
        for matched_ign in re_ign.find_iter(text) {
            not_allowed_ranges.push(matched_ign.start()..matched_ign.end());
        }
    }

    // replace umlaute
    replace_ignoring_words(
        text,
        vec![("oe", "ö"), ("ae", "ä"), ("ue", "ü"),
             ("Oe", "Ö"), ("Ae", "Ä"), ("Ue", "Ü")],
        &not_allowed_ranges
    )
}

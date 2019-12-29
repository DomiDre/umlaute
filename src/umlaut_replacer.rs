/// Replace a word within text by another, while ignoring certain ranges of the
/// string
fn replace_ignoring_words(
    text: &String,
    replace_pairs: Vec<(&str, &str)>,
    ignore_ranges: &Vec<(usize, usize)>,
) -> String {
    let mut edited_text = text.clone(); // create a copy that will be edited
    let mut track_shift = 0; // track if the idx to edit change with respect to the original text
    for (replace_word, replace_with) in replace_pairs {
        let replace_shift = replace_with.len() - replace_word.len(); // difference between words that are edited
        
        //walk through text editing every occurence of replace_word
        //while checking that none of the ignore_words are edited
        'replace_str: for &(start_idx, _) in text.match_indices(replace_word).collect::<Vec<_>>().iter() {
            let end_idx = start_idx + replace_word.len();
            // check if bytes that shall be edited are within the ignore range
            for &(start_ignore, end_ignore) in ignore_ranges.iter() {
                if start_idx >= start_ignore
                && end_idx <= end_ignore {
                    continue 'replace_str;
                }
            }
            // otherwise perform edit
            edited_text.replace_range(
                start_idx+track_shift..end_idx+track_shift,
                replace_with,
            );
            track_shift += replace_shift;
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
        for &(match_index, _) in text.match_indices(ign_string).collect::<Vec<_>>().iter() {
            not_allowed_ranges.push((match_index, match_index + ign_string.len()));
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


#[test]
fn replace_text() {
    use crate::umlaut_replacer;
    assert_eq!(
        umlaut_replacer::replace_umlaute(
            &"Schoene Gruesse".to_string(),
            &Vec::new()
        ),
        "Schöne Grüsse".to_string()
    );
}

#[test]
fn ignore_word() {
    use crate::umlaut_replacer;
    assert_eq!(
        umlaut_replacer::replace_umlaute(
            &"Schoene Gruesse Dominique".to_string(),
            &vec!["Dominique".to_string()]
        ),
        "Schöne Grüsse Dominique".to_string()
    );
}